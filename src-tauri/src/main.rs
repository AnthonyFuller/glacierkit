// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Specta creates non snake case functions
#![allow(non_snake_case)]
#![feature(try_blocks)]

pub mod game_detection;
pub mod hash_list;
pub mod model;

use std::{collections::HashMap, fmt::Debug, fs, ops::Deref, path::Path, sync::Arc};

use anyhow::{Context, Error, Result};
use arc_swap::{access::Access, ArcSwap, Guard};
use fn_error_context::context;
use game_detection::{detect_installs, GameVersion};
use hash_list::HashList;
use model::{
	AppSettings, AppState, Event, FileBrowserEvent, FileBrowserRequest, GameBrowserEntry, GameBrowserEvent,
	GameBrowserRequest, GlobalEvent, GlobalRequest, Project, ProjectSettings, Request, SettingsEvent, SettingsRequest,
	ToolEvent, ToolRequest
};
use notify::Watcher;
use serde_json::{from_slice, to_vec};
use tauri::{async_runtime, AppHandle, Manager};
use tokio::sync::RwLock;
use tryvial::try_fn;
use uuid::Uuid;
use walkdir::WalkDir;

const HASH_LIST_ENDPOINT: &str =
	"https://github.com/glacier-modding/Hitman-Hashes/releases/latest/download/entity_hash_list.sml";

fn main() {
	#[cfg(debug_assertions)]
	{
		tauri_specta::ts::export(specta::collect_types![event], "../src/lib/bindings.ts").unwrap();

		specta::export::ts("../src/lib/bindings-types.ts").unwrap();
	}

	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![event])
		.setup(|app| {
			let app_data_path = app.path_resolver().app_data_dir().unwrap();

			let mut invalid = true;
			if let Ok(read) = fs::read(app_data_path.join("settings.json")) {
				if let Ok(settings) = from_slice::<AppSettings>(&read) {
					invalid = false;
					app.manage(ArcSwap::new(settings.into()));
				}
			}

			if invalid {
				let settings = AppSettings::default();
				fs::create_dir_all(&app_data_path).unwrap();
				fs::write(app_data_path.join("settings.json"), to_vec(&settings).unwrap()).unwrap();
				app.manage(ArcSwap::new(settings.into()));
			}

			app.manage(AppState {
				game_installs: detect_installs().unwrap(),
				project: None.into(),
				hash_list: fs::read(app_data_path.join("hash_list.sml"))
					.ok()
					.and_then(|x| serde_smile::from_slice(&x).ok())
					.into(),
				fs_watcher: None.into(),
				editor_states: RwLock::new(HashMap::new()).into()
			});

			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

#[tauri::command]
#[specta::specta]
fn event(app: AppHandle, event: Event) {
	async_runtime::spawn(async move {
		let app_settings = app.state::<ArcSwap<AppSettings>>();
		let app_state = app.state::<AppState>();

		if let Err::<_, Error>(e) = try {
			match event {
				Event::Tool(event) => match event {
					ToolEvent::FileBrowser(event) => match event {
						FileBrowserEvent::Select(path) => {}

						FileBrowserEvent::Create { path, is_folder } => {
							let task = start_task(
								&app,
								format!(
									"Creating {} {}",
									if is_folder { "folder" } else { "file" },
									path.file_name().unwrap().to_string_lossy()
								)
							)?;

							if is_folder {
								fs::create_dir(path)?;
							} else {
								fs::write(path, "")?;
							}

							finish_task(&app, task)?;
						}

						FileBrowserEvent::Delete(path) => {
							let task = start_task(
								&app,
								format!("Moving {} to bin", path.file_name().unwrap().to_string_lossy())
							)?;

							trash::delete(path)?;

							finish_task(&app, task)?;
						}

						FileBrowserEvent::Rename { old_path, new_path } => {
							let task = start_task(
								&app,
								format!(
									"Renaming {} to {}",
									old_path.file_name().unwrap().to_string_lossy(),
									new_path.file_name().unwrap().to_string_lossy()
								)
							)?;

							fs::rename(old_path, new_path)?;

							finish_task(&app, task)?;
						}
					},

					ToolEvent::GameBrowser(event) => match event {
						GameBrowserEvent::Select(path) => {
							// TODO
						}

						GameBrowserEvent::Search(query) => {
							let task = start_task(&app, format!("Searching game files for {}", query))?;

							if let Some(install) = app_state
								.project
								.load()
								.as_ref()
								.unwrap()
								.settings
								.load()
								.game_install
								.as_ref()
							{
								let install = app_state
									.game_installs
									.iter()
									.find(|x| x.path == *install)
									.context("No such game install as specified in project.json")?;

								let game_flag = match install.version {
									GameVersion::H1 => 0b000010,
									GameVersion::H2 => 0b000100,
									GameVersion::H3 => 0b001000
								};

								if let Some(x) = app_state.hash_list.load().deref() {
									send_request(
										&app,
										Request::Tool(ToolRequest::GameBrowser(GameBrowserRequest::NewTree {
											game_description: format!(
												"{} ({})",
												match install.version {
													GameVersion::H1 => "HITMAN™",
													GameVersion::H2 => "HITMAN 2",
													GameVersion::H3 => "HITMAN 3"
												},
												install.platform
											),
											entries: x
												.entries
												.iter()
												.filter(|x| x.game_flags & game_flag == game_flag)
												.filter(|x| query.split(' ').all(|y| x.path.contains(y)))
												.map(|x| GameBrowserEntry {
													hash: x.hash.to_owned(),
													path: x.path.to_owned(),
													hint: x.hint.to_owned()
												})
												.collect()
										}))
									)?;
								}
							}

							finish_task(&app, task)?;
						}
					},

					ToolEvent::Settings(event) => match event {
						SettingsEvent::Initialise => {
							send_request(
								&app,
								Request::Tool(ToolRequest::Settings(SettingsRequest::Initialise {
									game_installs: app_state.game_installs.to_owned(),
									settings: (*app_settings.inner().load_full()).to_owned()
								}))
							)?;
						}

						SettingsEvent::ChangeGameInstall(path) => {
							send_request(
								&app,
								Request::Tool(ToolRequest::GameBrowser(GameBrowserRequest::SetEnabled(path.is_some())))
							)?;

							if let Some(project) = app_state.project.load().deref() {
								let mut settings = (*project.settings.load_full()).to_owned();
								settings.game_install = path;
								fs::write(project.path.join("project.json"), to_vec(&settings).unwrap()).unwrap();
								project.settings.store(settings.into());
							}
						}

						SettingsEvent::ChangeExtractModdedFiles(value) => {
							let mut settings = (*app_settings.load_full()).to_owned();
							settings.extract_modded_files = value;
							fs::write(
								app.path_resolver()
									.app_data_dir()
									.context("Couldn't get app data dir")?
									.join("settings.json"),
								to_vec(&settings).unwrap()
							)
							.unwrap();
							app_settings.store(settings.into());
						}

						SettingsEvent::ChangeGFEPath(value) => {
							let mut settings = (*app_settings.load_full()).to_owned();
							settings.game_file_extensions_path = value;
							fs::write(
								app.path_resolver()
									.app_data_dir()
									.context("Couldn't get app data dir")?
									.join("settings.json"),
								to_vec(&settings).unwrap()
							)
							.unwrap();
							app_settings.store(settings.into());
						}
					}
				},

				// Event::Editor(event) => match event {},
				Event::Global(event) => match event {
					GlobalEvent::LoadWorkspace(path) => {
						let task = start_task(&app, format!("Loading project {}", path.display()))?;

						let mut files = vec![];

						for entry in WalkDir::new(&path)
							.sort_by_file_name()
							.into_iter()
							.filter_map(|x| x.ok())
						{
							files.push((
								entry.path().into(),
								entry.metadata().context("Couldn't get file metadata")?.is_dir()
							));
						}

						let settings;
						if let Ok(read) = fs::read(path.join("project.json")) {
							if let Ok(read_settings) = from_slice::<ProjectSettings>(&read) {
								settings = read_settings;
							} else {
								settings = ProjectSettings::default();
								fs::create_dir_all(&path).unwrap();
								fs::write(path.join("project.json"), to_vec(&settings).unwrap()).unwrap();
							}
						} else {
							settings = ProjectSettings::default();
							fs::create_dir_all(&path).unwrap();
							fs::write(path.join("project.json"), to_vec(&settings).unwrap()).unwrap();
						}

						app_state.project.store(Some(
							Project {
								path: path.to_owned(),
								settings: Arc::new(settings.to_owned()).into()
							}
							.into()
						));

						send_request(
							&app,
							Request::Global(GlobalRequest::SetWindowTitle(
								path.file_name().unwrap().to_string_lossy().into()
							))
						)?;

						send_request(
							&app,
							Request::Tool(ToolRequest::Settings(SettingsRequest::ChangeProjectSettings(
								settings.to_owned()
							)))
						)?;

						send_request(
							&app,
							Request::Tool(ToolRequest::FileBrowser(FileBrowserRequest::NewTree {
								base_path: path.to_owned(),
								files
							}))
						)?;

						let notify_path = path.to_owned();
						let notify_app = app.to_owned();

						app_state.fs_watcher.store(Some(
							{
								let mut watcher =
									notify::recommended_watcher(move |evt: Result<notify::Event, notify::Error>| {
										if let Err::<_, Error>(e) = try {
											if let Ok(evt) = evt {
												if evt.need_rescan() {
													// Refresh the whole tree

													let mut files = vec![];

													for entry in WalkDir::new(&notify_path)
														.sort_by_file_name()
														.into_iter()
														.filter_map(|x| x.ok())
													{
														files.push((
															entry.path().into(),
															entry
																.metadata()
																.context("Couldn't get file metadata")?
																.is_dir()
														));
													}

													send_request(
														&notify_app,
														Request::Tool(ToolRequest::FileBrowser(
															FileBrowserRequest::NewTree {
																base_path: notify_path.to_owned(),
																files
															}
														))
													)?;

													return;
												}

												match evt.kind {
													notify::EventKind::Create(kind) => match kind {
														notify::event::CreateKind::File => {
															send_request(
																&notify_app,
																Request::Tool(ToolRequest::FileBrowser(
																	FileBrowserRequest::Create {
																		path: evt
																			.paths
																			.first()
																			.context("Create event had no paths")?
																			.to_owned(),
																		is_folder: false
																	}
																))
															)?;
														}

														notify::event::CreateKind::Folder => {
															send_request(
																&notify_app,
																Request::Tool(ToolRequest::FileBrowser(
																	FileBrowserRequest::Create {
																		path: evt
																			.paths
																			.first()
																			.context("Create event had no path")?
																			.to_owned(),
																		is_folder: true
																	}
																))
															)?;
														}

														notify::event::CreateKind::Any
														| notify::event::CreateKind::Other => {
															if let Ok(metadata) = fs::metadata(
																evt.paths
																	.first()
																	.context("Create event had no paths")?
															) {
																send_request(
																	&notify_app,
																	Request::Tool(ToolRequest::FileBrowser(
																		FileBrowserRequest::Create {
																			path: evt
																				.paths
																				.first()
																				.context("Create event had no paths")?
																				.to_owned(),
																			is_folder: metadata.is_dir()
																		}
																	))
																)?;
															}
														}
													},

													notify::EventKind::Modify(notify::event::ModifyKind::Name(
														notify::event::RenameMode::Both
													)) => {
														send_request(
															&notify_app,
															Request::Tool(ToolRequest::FileBrowser(
																FileBrowserRequest::Rename {
																	old_path: evt
																		.paths
																		.first()
																		.context("Rename-both event had no first path")?
																		.to_owned(),
																	new_path: evt
																		.paths
																		.get(1)
																		.context(
																			"Rename-both event had no second path"
																		)?
																		.to_owned()
																}
															))
														)?;
													}

													notify::EventKind::Modify(notify::event::ModifyKind::Name(
														notify::event::RenameMode::From
													)) => {
														send_request(
															&notify_app,
															Request::Tool(ToolRequest::FileBrowser(
																FileBrowserRequest::Delete(
																	evt.paths
																		.first()
																		.context("Rename-from event had no path")?
																		.to_owned()
																)
															))
														)?;
													}

													notify::EventKind::Modify(notify::event::ModifyKind::Name(
														notify::event::RenameMode::To
													)) => {
														if let Ok(metadata) = fs::metadata(
															evt.paths
																.first()
																.context("Rename-to event had no paths")?
														) {
															send_request(
																&notify_app,
																Request::Tool(ToolRequest::FileBrowser(
																	FileBrowserRequest::Create {
																		path: evt
																			.paths
																			.first()
																			.context("Rename-to event had no paths")?
																			.to_owned(),
																		is_folder: metadata.is_dir()
																	}
																))
															)?;
														}
													}

													notify::EventKind::Remove(_) => {
														send_request(
															&notify_app,
															Request::Tool(ToolRequest::FileBrowser(
																FileBrowserRequest::Delete(
																	evt.paths
																		.first()
																		.context("Remove event had no path")?
																		.to_owned()
																)
															))
														)?;
													}

													_ => {}
												}
											}
										} {
											send_request(
												&notify_app,
												Request::Global(GlobalRequest::ErrorReport {
													error: format!("{:?}", e.context("Notifier error"))
												})
											)
											.expect("Couldn't send error report to frontend");
										}
									})?;

								watcher.watch(&path, notify::RecursiveMode::Recursive)?;

								watcher
							}
							.into()
						));

						finish_task(&app, task)?;

						let task = start_task(&app, "Acquiring latest hash list")?;

						if let Ok(data) = reqwest::get(HASH_LIST_ENDPOINT).await {
							if let Ok(data) = data.bytes().await {
								let hash_list = HashList::from_slice(&data)?;

								fs::write(
									app.path_resolver()
										.app_data_dir()
										.context("Couldn't get app data dir")?
										.join("hash_list.sml"),
									serde_smile::to_vec(&hash_list).unwrap()
								)
								.unwrap();

								app_state.hash_list.store(Some(hash_list.into()));
							}
						}

						finish_task(&app, task)?;

						send_request(
							&app,
							Request::Tool(ToolRequest::GameBrowser(GameBrowserRequest::SetEnabled(
								settings.game_install.is_some() && app_state.hash_list.load().is_some()
							)))
						)?;
					}
				}
			}
		} {
			send_request(
				&app,
				Request::Global(GlobalRequest::ErrorReport {
					error: format!("{:?}", e)
				})
			)
			.expect("Couldn't send error report to frontend");
		}
	});
}

#[try_fn]
#[context("Couldn't send task start event for {:?} to frontend", name.as_ref())]
pub fn start_task(app: &AppHandle, name: impl AsRef<str>) -> Result<Uuid> {
	let task_id = Uuid::new_v4();
	app.emit_all("start-task", (&task_id, name.as_ref()))?;
	task_id
}

#[try_fn]
#[context("Couldn't send task finish event for {:?} to frontend", task)]
pub fn finish_task(app: &AppHandle, task: Uuid) -> Result<()> {
	app.emit_all("finish-task", &task)?;
}

#[try_fn]
#[context("Couldn't send request {:?} to frontend", request)]
pub fn send_request(app: &AppHandle, request: Request) -> Result<()> {
	app.emit_all("request", &request)?;
}
