use std::{
	collections::{HashMap, HashSet},
	io::{BufReader, Cursor},
	ops::Deref
};

use anyhow::{anyhow, Context, Result};
use arc_swap::ArcSwap;
use fn_error_context::context;
use itertools::Itertools;
use quickentity_rs::{
	apply_patch, convert_2016_factory_to_modern,
	patch_structs::{Patch, PatchOperation, SubEntityOperation},
	qn_structs::{FullRef, Ref, RefMaybeConstantValue, RefWithConstantValue, SubEntity}
};
use serde::Serialize;
use serde_json::{from_str, from_value, json, to_value, Value};
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};
use tauri::{AppHandle, Manager};
use tryvial::try_fn;
use uuid::Uuid;

use crate::{
	entity::{
		alter_ref_according_to_changelist, calculate_reverse_references, change_reference_to_local,
		check_local_references_exist, get_decorations, get_local_reference, get_recursive_children,
		is_valid_entity_blueprint, is_valid_entity_factory, random_entity_id, CopiedEntityData, ReverseReferenceData
	},
	finish_task,
	game_detection::GameVersion,
	model::{
		AppSettings, AppState, EditorData, EditorRequest, EditorState, EditorType, EditorValidity, EntityEditorRequest,
		EntityMetaPaneRequest, EntityMonacoRequest, EntityTreeRequest, GlobalRequest, Request
	},
	resourcelib::{
		h2016_convert_binary_to_factory, h2016_convert_cppt, h2_convert_binary_to_factory, h2_convert_cppt,
		h3_convert_binary_to_factory, h3_convert_cppt
	},
	rpkg::{
		ensure_entity_in_cache, extract_latest_metadata, extract_latest_overview_info, extract_latest_resource,
		normalise_to_hash
	},
	send_notification, send_request, start_task, Notification, NotificationKind
};

#[try_fn]
#[context("Couldn't handle update content event")]
pub async fn handle_updatecontent(app: &AppHandle, editor_id: Uuid, entity_id: String, content: String) -> Result<()> {
	let app_settings = app.state::<ArcSwap<AppSettings>>();
	let app_state = app.state::<AppState>();

	let mut editor_state = app_state.editor_states.write().await;
	let editor_state = editor_state.get_mut(&editor_id).context("No such editor")?;

	let entity = match editor_state.data {
		EditorData::QNEntity { ref mut entity, .. } => entity,
		EditorData::QNPatch { ref mut current, .. } => current,

		_ => {
			Err(anyhow!("Editor {} is not a QN editor", editor_id))?;
			panic!();
		}
	};

	match from_str(&content) {
		Ok(sub_entity) => match check_local_references_exist(&sub_entity, entity) {
			Ok(EditorValidity::Valid) => {
				if sub_entity != *entity.entities.get(&entity_id).context("No such sub-entity")? {
					if let Some(hash_list) = app_state.hash_list.load().as_ref() {
						if let Some(entry) = hash_list.entries.get(&normalise_to_hash(sub_entity.factory.to_owned())) {
							if !is_valid_entity_factory(&entry.resource_type) {
								send_request(
									&app,
									Request::Editor(EditorRequest::Entity(EntityEditorRequest::Monaco(
										EntityMonacoRequest::UpdateValidity {
											editor_id,
											validity: EditorValidity::Invalid(
												"Invalid factory; unsupported resource type".into()
											)
										}
									)))
								)?;

								return Ok(());
							}
						}

						if let Some(entry) = hash_list
							.entries
							.get(&normalise_to_hash(sub_entity.blueprint.to_owned()))
						{
							if !is_valid_entity_blueprint(&entry.resource_type) {
								send_request(
									&app,
									Request::Editor(EditorRequest::Entity(EntityEditorRequest::Monaco(
										EntityMonacoRequest::UpdateValidity {
											editor_id,
											validity: EditorValidity::Invalid(
												"Invalid blueprint; unsupported resource type".into()
											)
										}
									)))
								)?;

								return Ok(());
							}
						}
					}
					let mut reverse_parent_refs: HashSet<String> = HashSet::new();

					for entity_data in entity.entities.values() {
						match entity_data.parent {
							Ref::Full(ref reference) if reference.external_scene.is_none() => {
								reverse_parent_refs.insert(reference.entity_ref.to_owned());
							}

							Ref::Short(Some(ref reference)) => {
								reverse_parent_refs.insert(reference.to_owned());
							}

							_ => {}
						}
					}

					send_request(
						&app,
						Request::Editor(EditorRequest::Entity(EntityEditorRequest::Tree(
							EntityTreeRequest::NewItems {
								editor_id,
								new_entities: vec![(
									entity_id.to_owned(),
									sub_entity.parent.to_owned(),
									sub_entity.name.to_owned(),
									sub_entity.factory.to_owned(),
									reverse_parent_refs.contains(&entity_id)
								)]
							}
						)))
					)?;

					entity.entities.insert(entity_id.to_owned(), sub_entity);

					send_request(
						&app,
						Request::Editor(EditorRequest::Entity(EntityEditorRequest::Monaco(
							EntityMonacoRequest::UpdateValidity {
								editor_id,
								validity: EditorValidity::Valid
							}
						)))
					)?;

					send_request(
						&app,
						Request::Global(GlobalRequest::SetTabUnsaved {
							id: editor_id,
							unsaved: true
						})
					)?;

					if let Some(resource_packages) = app_state.resource_packages.load().as_ref()
						&& let Some(hash_list) = app_state.hash_list.load().as_ref()
						&& let Some(install) = app_settings.load().game_install.as_ref()
					{
						let game_version = app_state
							.game_installs
							.iter()
							.try_find(|x| anyhow::Ok(x.path == *install))?
							.context("No such game install")?
							.version;

						let task = start_task(&app, "Updating decorations")?;

						let decorations = get_decorations(
							resource_packages,
							&app_state.cached_entities,
							hash_list,
							game_version,
							entity.entities.get(&entity_id).context("No such entity")?,
							entity
						)?;

						send_request(
							&app,
							Request::Editor(EditorRequest::Entity(EntityEditorRequest::Monaco(
								EntityMonacoRequest::UpdateDecorationsAndMonacoInfo {
									editor_id: editor_id.to_owned(),
									entity_id: entity_id.to_owned(),
									local_ref_entity_ids: decorations
										.iter()
										.filter(|(x, _)| entity.entities.contains_key(x))
										.map(|(x, _)| x.to_owned())
										.collect(),
									decorations
								}
							)))
						)?;

						finish_task(&app, task)?;
					}
				} else {
					send_request(
						&app,
						Request::Editor(EditorRequest::Entity(EntityEditorRequest::Monaco(
							EntityMonacoRequest::UpdateValidity {
								editor_id,
								validity: EditorValidity::Valid
							}
						)))
					)?;
				}
			}

			Ok(EditorValidity::Invalid(reason)) => {
				send_request(
					&app,
					Request::Editor(EditorRequest::Entity(EntityEditorRequest::Monaco(
						EntityMonacoRequest::UpdateValidity {
							editor_id,
							validity: EditorValidity::Invalid(reason)
						}
					)))
				)?;
			}

			Err(err) => {
				send_request(
					&app,
					Request::Editor(EditorRequest::Entity(EntityEditorRequest::Monaco(
						EntityMonacoRequest::UpdateValidity {
							editor_id,
							validity: EditorValidity::Invalid(format!("Invalid entity: {}", err))
						}
					)))
				)?;
			}
		},

		Err(err) => {
			send_request(
				&app,
				Request::Editor(EditorRequest::Entity(EntityEditorRequest::Monaco(
					EntityMonacoRequest::UpdateValidity {
						editor_id,
						validity: EditorValidity::Invalid(format!("Invalid entity: {}", err))
					}
				)))
			)?;
		}
	}
}

#[try_fn]
#[context("Couldn't handle open factory event")]
pub async fn handle_openfactory(app: &AppHandle, factory: String) -> Result<()> {
	let app_settings = app.state::<ArcSwap<AppSettings>>();
	let app_state = app.state::<AppState>();

	if let Some(install) = app_settings.load().game_install.as_ref()
		&& let Some(hash_list) = app_state.hash_list.load().as_ref()
		&& let Some(resource_packages) = app_state.resource_packages.load().as_deref()
	{
		let factory = normalise_to_hash(factory);

		if let Ok((filetype, _, _)) = extract_latest_overview_info(resource_packages, &factory) {
			if filetype == "TEMP" {
				let task = start_task(&app, format!("Loading entity {}", factory))?;

				let game_install_data = app_state
					.game_installs
					.iter()
					.try_find(|x| anyhow::Ok(x.path == *install))?
					.context("No such game install")?;

				ensure_entity_in_cache(
					resource_packages,
					&app_state.cached_entities,
					game_install_data.version,
					hash_list,
					&factory
				)?;

				let entity = app_state.cached_entities.read().get(&factory).unwrap().to_owned();

				let default_tab_name = format!(
					"{} ({})",
					entity
						.entities
						.get(&entity.root_entity)
						.context("Root entity doesn't exist")?
						.name,
					factory
				);

				let tab_name = if let Some(entry) = hash_list.entries.get(&factory) {
					if let Some(path) = entry.path.as_ref() {
						path.replace("].pc_entitytype", "")
							.replace("].pc_entitytemplate", "")
							.split('/')
							.last()
							.map(|x| x.to_owned())
							.unwrap_or(default_tab_name)
					} else if let Some(hint) = entry.hint.as_ref() {
						format!("{} ({})", hint, factory)
					} else {
						default_tab_name
					}
				} else {
					default_tab_name
				};

				let id = Uuid::new_v4();

				app_state.editor_states.write().await.insert(
					id.to_owned(),
					EditorState {
						file: None,
						data: EditorData::QNPatch {
							base: Box::new(entity.to_owned()),
							current: Box::new(entity.to_owned()),
							settings: Default::default()
						}
					}
				);

				send_request(
					&app,
					Request::Global(GlobalRequest::CreateTab {
						id,
						name: tab_name,
						editor_type: EditorType::QNPatch
					})
				)?;

				finish_task(&app, task)?;
			} else {
				let id = Uuid::new_v4();

				app_state.editor_states.write().await.insert(
					id.to_owned(),
					EditorState {
						file: None,
						data: EditorData::ResourceOverview {
							hash: factory.to_owned()
						}
					}
				);

				send_request(
					&app,
					Request::Global(GlobalRequest::CreateTab {
						id,
						name: format!("Resource overview ({factory})"),
						editor_type: EditorType::ResourceOverview
					})
				)?;
			}
		} else {
			send_notification(
				&app,
				Notification {
					kind: NotificationKind::Error,
					title: "Not a vanilla resource".into(),
					subtitle: "This factory doesn't exist in the base game files.".into()
				}
			)?;
		}
	} else {
		send_notification(
			&app,
			Notification {
				kind: NotificationKind::Error,
				title: "No game selected".into(),
				subtitle: "You can't open game files without a copy of the game selected.".into()
			}
		)?;
	}
}
