use std::fs;

use eframe::egui;
use serde::{Deserialize, Serialize};

use crate::ProjectInfo;

#[derive(Deserialize, Serialize)]
pub struct SelectorApp {
    projects: Vec<ProjectInfo>,
    projects_folder: Option<String>,
    #[serde(skip)]
    error_message: Option<String>,
}

impl SelectorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configurar estilo, fonts, etc. como en tu TemplateApp::new
        // cc.egui_ctx.set_visuals(egui::Visuals::dark());

        if let Some(storage) = cc.storage {
            let mut app: Self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();

            if app.projects_folder.is_some() {
                let _ = app.load_projects();
            }

            app
        } else {
            Default::default()
        }
    }
    fn load_projects(&mut self) -> Result<(), std::io::Error> {
        self.projects.clear();

        if let Some(folder) = &self.projects_folder {
            for entry in fs::read_dir(folder)? {
                let entry = entry?;
                let fpath = entry.path();

                if !fpath.is_dir() {
                    continue;
                }

                let project_entry = ProjectInfo {
                    name: fpath
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned(),
                    path: fpath.to_string_lossy().to_string(),
                };

                self.projects.push(project_entry);
            }
        }
        Ok(())
    }
}
impl eframe::App for SelectorApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Select project");

            ui.horizontal(|ui| {
                ui.label("Projects Folder:");
                ui.label(
                    self.projects_folder
                        .as_ref()
                        .map(|s| s.as_str())
                        .unwrap_or("Not selected"),
                );
            });

            if ui.button("📁 Choose Projects Folder").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.projects_folder = Some(path.to_string_lossy().to_string());
                    self.error_message = None;

                    match self.load_projects() {
                        Ok(_) => {}
                        Err(err) => {
                            self.error_message = Some(format!("Error loading projects: {}", err));
                        }
                    }
                }
            }

            ui.separator();

            if let Some(error) = &self.error_message {
                ui.colored_label(egui::Color32::RED, error);
                ui.separator();
            }

            if self.projects_folder.is_some() {
                ui.label(format!("Projects found: {}", self.projects.len()));
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for project in &self.projects {
                        if ui.button(&project.name).clicked() {
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                use std::process::Command;
                                match Command::new("./target/release/editor")
                                    .arg(&project.path)
                                    .spawn()
                                {
                                    Ok(_) => {
                                        self.error_message =
                                            Some(format!("Opening: {}", project.name));
                                    }
                                    Err(e) => {
                                        self.error_message =
                                            Some(format!("Failed to open editor: {}", e));
                                    }
                                }
                            }

                            #[cfg(target_arch = "wasm32")]
                            {
                                let url = format!("editor.html?project={}", project.path);
                                if let Some(window) = web_sys::window() {
                                    let _ = window.open_with_url_and_target(&url, "_blank");
                                }
                            }
                        }
                    }
                });
            } else {
                ui.label("Please select a projects folder to begin");
            }
        });
    }
}

impl Default for SelectorApp {
    fn default() -> Self {
        Self {
            projects: vec![],
            projects_folder: None,
            error_message: None,
        }
    }
}
