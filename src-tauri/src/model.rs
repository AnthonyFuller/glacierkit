use std::{collections::HashMap, path::PathBuf, sync::Arc};

use arc_swap::{ArcSwap, ArcSwapOption};
use indexmap::IndexMap;
use notify::RecommendedWatcher;
use quickentity_rs::qn_structs::{Entity, Ref, SubEntity, SubType};
use rpkg_rs::runtime::resource::resource_package::ResourcePackage;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use specta::Type;
use structstruck::strike;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
	entity::{CopiedEntityData, ReverseReference},
	game_detection::GameInstall,
	hash_list::HashList,
	intellisense::Intellisense
};

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
	pub extract_modded_files: bool,
	pub game_install: Option<PathBuf>
}

impl Default for AppSettings {
	fn default() -> Self {
		Self {
			extract_modded_files: false,
			game_install: None
		}
	}
}

pub struct AppState {
	pub game_installs: Vec<GameInstall>,
	pub project: ArcSwapOption<Project>,
	pub hash_list: ArcSwapOption<HashList>,
	pub fs_watcher: ArcSwapOption<RecommendedWatcher>,
	pub editor_states: Arc<RwLock<HashMap<Uuid, EditorState>>>,
	pub resource_packages: ArcSwapOption<IndexMap<PathBuf, ResourcePackage>>,
	pub cached_entities: Arc<parking_lot::RwLock<HashMap<String, Entity>>>,
	pub intellisense: ArcSwapOption<Intellisense>
}

#[derive(Debug)]
pub struct EditorState {
	pub file: Option<PathBuf>,
	pub data: EditorData
}

#[derive(Debug, Clone)]
pub enum EditorData {
	Nil,
	Text {
		content: String,
		file_type: TextFileType
	},
	QNEntity {
		settings: EphemeralQNSettings,
		entity: Box<Entity>
	},
	QNPatch {
		settings: EphemeralQNSettings,
		base: Box<Entity>,
		current: Box<Entity>
	}
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EphemeralQNSettings {
	pub show_reverse_parent_refs: bool
}

impl Default for EphemeralQNSettings {
	fn default() -> Self {
		Self {
			show_reverse_parent_refs: false
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
	pub path: PathBuf,
	pub settings: ArcSwap<ProjectSettings>
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSettings {}

impl Default for ProjectSettings {
	fn default() -> Self {
		Self {}
	}
}

#[derive(Type, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct GameBrowserEntry {
	pub hash: String,
	pub path: Option<String>,
	pub hint: Option<String>
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "data")]
pub enum TextFileType {
	Json,
	ManifestJson,
	PlainText,
	Markdown
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "data")]
pub enum EditorType {
	Nil,
	Text { file_type: TextFileType },
	QNEntity,
	QNPatch
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "data")]
pub enum EditorValidity {
	Valid,
	Invalid(String)
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PastableTemplate {
	pub name: String,
	pub icon: String,
	pub paste_data: CopiedEntityData
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PastableTemplateCategory {
	pub name: String,
	pub icon: String,
	pub templates: Vec<PastableTemplate>
}

strike! {
	#[strikethrough[derive(Type, Serialize, Deserialize, Clone, Debug)]]
	#[strikethrough[serde(rename_all = "camelCase", tag = "type", content = "data")]]
	pub enum Event {
		Tool(pub enum ToolEvent {
			FileBrowser(pub enum FileBrowserEvent {
				Select(Option<PathBuf>),

				Create {
					path: PathBuf,
					is_folder: bool
				},

				Delete(PathBuf),

				Rename {
					old_path: PathBuf,
					new_path: PathBuf
				},

				NormaliseQNFile {
					path: PathBuf
				},

				ConvertEntityToPatch {
					path: PathBuf
				},

				ConvertPatchToEntity {
					path: PathBuf
				}
			}),

			GameBrowser(pub enum GameBrowserEvent {
				Select(String),
				Search(String)
			}),

			Settings(pub enum SettingsEvent {
				Initialise,
				ChangeGameInstall(Option<PathBuf>),
				ChangeExtractModdedFiles(bool)
			})
		}),

		Editor(pub enum EditorEvent {
			Text(pub enum TextEditorEvent {
				Initialise {
					id: Uuid
				},

				UpdateContent {
					id: Uuid,
					content: String
				}
			}),

			Entity(pub enum EntityEditorEvent {
				General(pub enum EntityGeneralEvent {
					SetShowReverseParentRefs {
						editor_id: Uuid,
						show_reverse_parent_refs: bool
					}
				}),

				Tree(pub enum EntityTreeEvent {
					Initialise {
						editor_id: Uuid
					},

					Select {
						editor_id: Uuid,
						id: String
					},

					Create {
						editor_id: Uuid,
						id: String,
						content: SubEntity
					},

					Delete {
						editor_id: Uuid,
						id: String
					},

					Rename {
						editor_id: Uuid,
						id: String,
						new_name: String
					},

					Reparent {
						editor_id: Uuid,
						id: String,
						new_parent: Ref
					},

					Copy {
						editor_id: Uuid,
						id: String
					},

					Paste {
						editor_id: Uuid,
						parent_id: String
					},

					Search {
						editor_id: Uuid,
						query: String
					},

					ShowHelpMenu {
						editor_id: Uuid,
						entity_id: String
					},

					UseTemplate {
						editor_id: Uuid,
						parent_id: String,
						template: CopiedEntityData
					}
				}),

				Monaco(pub enum EntityMonacoEvent {
					UpdateContent {
						editor_id: Uuid,
						entity_id: String,
						content: String
					},

					FollowReference {
						editor_id: Uuid,
						reference: String
					},

					OpenFactory {
						editor_id: Uuid,
						factory: String
					}
				}),

				MetaPane(pub enum EntityMetaPaneEvent {
					JumpToReference {
						editor_id: Uuid,
						reference: String
					},

					SetNotes {
						editor_id: Uuid,
						entity_id: String,
						notes: String
					}
				}),

				Metadata(pub enum EntityMetadataEvent {
					Initialise {
						editor_id: Uuid
					},

					SetFactoryHash {
						editor_id: Uuid,
						factory_hash: String
					},

					SetBlueprintHash {
						editor_id: Uuid,
						blueprint_hash: String
					},

					SetRootEntity {
						editor_id: Uuid,
						root_entity: String
					},

					SetSubType {
						editor_id: Uuid,
						sub_type: SubType
					},

					SetExternalScenes {
						editor_id: Uuid,
						external_scenes: Vec<String>
					}
				}),

				Overrides(pub enum EntityOverridesEvent {
					Initialise {
						editor_id: Uuid
					},

					UpdatePropertyOverrides {
						editor_id: Uuid,
						content: String
					},

					UpdateOverrideDeletes {
						editor_id: Uuid,
						content: String
					},

					UpdatePinConnectionOverrides {
						editor_id: Uuid,
						content: String
					},

					UpdatePinConnectionOverrideDeletes {
						editor_id: Uuid,
						content: String
					}
				})
			})
		}),

		Global(pub enum GlobalEvent {
			LoadWorkspace(PathBuf),
			SelectTab(Option<Uuid>),
			RemoveTab(Uuid),
			SaveTab(Uuid)
		})
	}
}

strike! {
	#[strikethrough[derive(Type, Serialize, Deserialize, Clone, Debug)]]
	#[strikethrough[serde(rename_all = "camelCase", tag = "type", content = "data")]]
	pub enum Request {
		Tool(pub enum ToolRequest {
			FileBrowser(pub enum FileBrowserRequest {
				Create {
					path: PathBuf,
					is_folder: bool
				},

				Delete(PathBuf),

				Rename {
					old_path: PathBuf,
					new_path: PathBuf
				},

				Select(Option<PathBuf>),

				NewTree {
					base_path: PathBuf,

					/// Relative path, is folder
					files: Vec<(PathBuf, bool)>
				}
			}),

			GameBrowser(pub enum GameBrowserRequest {
				SetEnabled(bool),

				NewTree {
					game_description: String,
					entries: Vec<GameBrowserEntry>
				}
			}),

			Settings(pub enum SettingsRequest {
				Initialise {
					game_installs: Vec<GameInstall>,
					settings: AppSettings
				},
				ChangeProjectSettings(ProjectSettings)
			})
		}),

		Editor(pub enum EditorRequest {
			Text(pub enum TextEditorRequest {
				ReplaceContent {
					id: Uuid,
					content: String
				},

				SetFileType {
					id: Uuid,
					file_type: TextFileType
				},
			}),

			Entity(pub enum EntityEditorRequest {
				Tree(pub enum EntityTreeRequest {
					/// Will trigger a Select event from the tree - ensure this doesn't end up in a loop
					Select {
						editor_id: Uuid,
						id: Option<String>
					},

					NewTree {
						editor_id: Uuid,

						/// ID, parent, name, factory, has reverse parent refs
						entities: Vec<(String, Ref, String, String, bool)>
					},

					/// Instructs the frontend to take the list of new entities, add any new ones and update any ones that already exist (by ID) with the new information.
					/// This is used for pasting, and for ensuring that icons/parent status/name are updated when a sub-entity is updated.
					NewItems {
						editor_id: Uuid,

						/// ID, parent, name, factory, has reverse parent refs
						new_entities: Vec<(String, Ref, String, String, bool)>
					},

					SearchResults {
						editor_id: Uuid,

						/// The IDs of the entities matching the query
						results: Vec<String>
					},

					ShowHelpMenu {
						editor_id: Uuid,
						factory: String,
						input_pins: Vec<String>,
						output_pins: Vec<String>,
						default_properties_html: String
					},

					SetTemplates {
						editor_id: Uuid,
						templates: Vec<PastableTemplateCategory>
					}
				}),

				Monaco(pub enum EntityMonacoRequest {
					ReplaceContent {
						editor_id: Uuid,
						entity_id: String,
						content: String
					},

					UpdateIntellisense {
						editor_id: Uuid,
						entity_id: String,
						properties: Vec<(String, String, Value, bool)>,
						pins: (Vec<String>, Vec<String>)
					},

					UpdateDecorationsAndMonacoInfo {
						editor_id: Uuid,
						entity_id: String,
						decorations: Vec<(String, String)>,
						local_ref_entity_ids: Vec<String>
					},

					UpdateValidity {
						editor_id: Uuid,
						validity: EditorValidity
					}
				}),

				MetaPane(pub enum EntityMetaPaneRequest {
					SetReverseRefs {
						editor_id: Uuid,
						entity_names: HashMap<String, String>,
						reverse_refs: Vec<ReverseReference>
					},

					SetNotes {
						editor_id: Uuid,
						entity_id: String,
						notes: String
					}
				}),

				Metadata(pub enum EntityMetadataRequest {
					Initialise {
						editor_id: Uuid,
						factory_hash: String,
						blueprint_hash: String,
						root_entity: String,
						sub_type: SubType,
						external_scenes: Vec<String>
					}
				}),

				Overrides(pub enum EntityOverridesRequest {
					Initialise {
						editor_id: Uuid,
						property_overrides: String,
						override_deletes: String,
						pin_connection_overrides: String,
						pin_connection_override_deletes: String
					},

					UpdateDecorations {
						editor_id: Uuid,
						decorations: Vec<(String, String)>,
					}
				})
			})
		}),

		Global(pub enum GlobalRequest {
			ErrorReport { error: String },
			SetWindowTitle(String),
			CreateTab {
				id: Uuid,
				name: String,
				editor_type: EditorType
			},
			SelectTab(Uuid),
			SetTabUnsaved {
				id: Uuid,
				unsaved: bool
			},
			RemoveTab(Uuid)
		})
	}
}
