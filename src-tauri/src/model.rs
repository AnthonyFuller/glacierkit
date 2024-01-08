use std::{collections::HashMap, path::PathBuf, sync::Arc};

use arc_swap::{ArcSwap, ArcSwapOption};
use notify::RecommendedWatcher;
use quickentity_rs::qn_structs::{Entity, Ref, SubEntity};
use serde::{Deserialize, Serialize};
use specta::Type;
use structstruck::strike;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{game_detection::GameInstall, hash_list::HashList};

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
	pub extract_modded_files: bool,
	pub game_file_extensions_path: Option<PathBuf>
}

impl Default for AppSettings {
	fn default() -> Self {
		Self {
			extract_modded_files: false,
			game_file_extensions_path: None
		}
	}
}

#[derive(Debug)]
pub struct AppState {
	pub game_installs: Vec<GameInstall>,
	pub project: ArcSwapOption<Project>,
	pub hash_list: ArcSwapOption<HashList>,
	pub fs_watcher: ArcSwapOption<RecommendedWatcher>,
	pub editor_states: Arc<RwLock<HashMap<Uuid, EditorState>>>
}

#[derive(Debug)]
pub struct EditorState {
	pub file: Option<PathBuf>,
	pub data: EditorData
}

#[derive(Debug, Clone)]
pub enum EditorData {
	Nil,
	Text { content: String, file_type: TextFileType },
	QNEntity(Box<Entity>),
	QNPatch { base: Box<Entity>, current: Box<Entity> }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
	pub path: PathBuf,
	pub settings: ArcSwap<ProjectSettings>
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSettings {
	pub game_install: Option<PathBuf>
}

impl Default for ProjectSettings {
	fn default() -> Self {
		Self { game_install: None }
	}
}

#[derive(Type, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct GameBrowserEntry {
	pub hash: String,
	pub path: String,
	pub hint: String
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
				}
			}),

			GameBrowser(pub enum GameBrowserEvent {
				Select(Option<String>),
				Search(String)
			}),

			Settings(pub enum SettingsEvent {
				Initialise,
				ChangeGameInstall(Option<PathBuf>),
				ChangeExtractModdedFiles(bool),
				ChangeGFEPath(Option<PathBuf>)
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
					}
				}),

				Monaco(pub enum EntityMonacoEvent {
					UpdateContent {
						editor_id: Uuid,
						entity_id: String,
						content: String
					}
				})
			})
		}),

		Global(pub enum GlobalEvent {
			LoadWorkspace(PathBuf),
			SelectTab(Uuid),
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
					}
				}),

				Monaco(pub enum EntityMonacoRequest {
					ReplaceContent {
						editor_id: Uuid,
						entity_id: String,
						content: String
					},

					UpdateIntellisense {

					},

					UpdateValidity {
						editor_id: Uuid,
						validity: EditorValidity
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
				editor_type: EditorType,
				file: Option<PathBuf>
			},
			SelectTab(Uuid),
			SetTabUnsaved {
				id: Uuid,
				unsaved: bool
			}
		})
	}
}
