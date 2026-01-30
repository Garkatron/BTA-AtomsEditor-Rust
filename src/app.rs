use std::{
    fs,
    path::{Path, PathBuf},
};

use egui::Ui;
use egui_ltreeview::{TreeView, TreeViewBuilder, TreeViewState};
use rust_embed::Embed;

use crate::{
    components::{form_view::Form, tabs::Tabs},
    files::project::{File, Project},
    form::form::Document,
};

#[derive(Embed)]
#[folder = "assets"]
struct Assets;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    // Files
    tab_index: usize,
    #[serde(skip)]
    tabs: Tabs,

    #[serde(skip)]
    project: Project,

    #[serde(skip)]
    tree_state: TreeViewState<i32>,

    #[serde(skip)]
    documents: Vec<Form>,

    #[serde(skip)]
    current_selected: i32,
}
impl Default for TemplateApp {
    fn default() -> Self {
        let schema_file = Assets::get("block.schema.toml")
            .ok_or_else(|| format!("Asset not found: {}", "path"))
            .expect("ERROR AAAAAAAAAAAAAAA");
        let schema_str = str::from_utf8(&schema_file.data)
            .map_err(|_| format!("Invalid UTF-8 in asset: {}", "path"))
            .expect("A");

        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            tab_index: 0,
            tabs: Tabs::default(),

            project: Project::new(
                "Test",
                Path::new("/home/bazzite/IdeaProjects/BTA-Atoms/run/btd/data/jade.project"),
            )
            .load(),
            tree_state: TreeViewState::default(),
            documents: vec![Form::new(
                Document::from_toml(schema_str).expect("EEEEEEEEEEERROOR"),
            )],
            current_selected: 0,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    pub fn project_tree(&mut self, ui: &mut egui::Ui) {
        let id = ui.make_persistent_id(self.project.name.clone());
        let (response, actions) =
            TreeView::new(id).show_state(ui, &mut self.tree_state, |builder| {
                builder.dir(0, "Root");
                Self::build_project_tree_static(builder, &self.project.files.children);
                builder.close_dir();
            });

        if let Some(selected) = self.tree_state.selected().first() {
            println!("ID seleccionado del TreeView: {}", selected);
            if let Some(file) = self.project.get_file(*selected) {
                if file.id != self.current_selected && !file.is_folder {
                    self.current_selected = file.id;
                    // self.content = fs::read_to_string(file.path.clone()).expect("ERROR")
                }
            } else {
                println!("No se encontró archivo con ID: {}", selected);
            }
        }
    }

    fn build_project_tree_static(builder: &mut TreeViewBuilder<'_, i32>, files: &[File]) {
        for file in files {
            if file.is_folder {
                builder.dir(file.id, &file.name);
                Self::build_project_tree_static(builder, &file.children);
                builder.close_dir();
            } else {
                builder.leaf(file.id, &file.name);
            }
        }
    }

    pub fn central_panel_content(&mut self, ui: &mut egui::Ui) {
        let available_size = ui.available_size();
        ui.horizontal(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(available_size.x * 0.22, available_size.y),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        ui.add_space(8.0);
                        ui.heading("📝 Forms");
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(8.0);

                    let scroll_height = available_size.y - 80.0;
                    egui::ScrollArea::vertical()
                        .id_salt("schema_list_scroll")
                        .auto_shrink([false, false])
                        .max_height(scroll_height)
                        .show(ui, |ui| {
                            ui.add_space(4.0);
                            self.project_tree(ui);
                        });
                },
            );

            ui.separator();

            ui.allocate_ui_with_layout(
                egui::vec2(available_size.x * 0.75, available_size.y),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    ui.add_space(8.0);

                    let content_height = available_size.y - 80.0;
                    egui::ScrollArea::vertical()
                        .id_salt("tab_content_scroll")
                        .auto_shrink([false, false])
                        .max_height(content_height)
                        .show(ui, |ui| {
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                ui.vertical(|ui| {
                                    /*
                                    egui::TextEdit::multiline(&mut self.content)
                                        .desired_width(f32::INFINITY)
                                        .font(egui::TextStyle::Monospace),
                                     */
                                    self.documents[0].show_state(ui);
                                });
                            });
                        });
                },
            );
        });
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // ! FILE TABS

            self.central_panel_content(ui);
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
/*


ui.horizontal(|ui| {
    ui.label("Write something: ");
    ui.text_edit_singleline(&mut self.label);
});

ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
if ui.button("Increment").clicked() {
    self.value += 1.0;
}

ui.separator();

ui.add(egui::github_link_file!(
    "https://github.com/emilk/eframe_template/blob/main/",
    "Source code."
));

ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
    powered_by_egui_and_eframe(ui);
    egui::warn_if_debug_build(ui);
});
*/
