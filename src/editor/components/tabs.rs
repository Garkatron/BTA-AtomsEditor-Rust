use egui::{ScrollArea, Ui};

pub struct Tab {
    pub title: String,
    pub ui: Box<dyn FnMut(&mut Ui)>,
}

pub struct Tabs {
    index: usize,
    pub tabs: Vec<Tab>,
}

impl Tabs {
    pub fn new(tabs: Vec<Tab>) -> Self {
        Self { index: 0, tabs }
    }

    pub fn tab_bar(&mut self, ui: &mut Ui) {
        ScrollArea::horizontal()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (i, tab) in self.tabs.iter().enumerate() {
                        let selected = self.index == i;
                        if ui.selectable_label(selected, &tab.title).clicked() {
                            self.index = i;
                        }
                    }
                });
            });
    }

    pub fn show_content(&mut self, ui: &mut Ui) {
        if let Some(tab) = self.tabs.get_mut(self.index) {
            (tab.ui)(ui);
        }
    }

    pub fn add_tab(
        &mut self,
        title: impl Into<String>,
        ui_fn: impl FnMut(&mut Ui) + 'static,
    ) -> &mut Self {
        self.tabs.push(Tab {
            title: title.into(),
            ui: Box::new(ui_fn),
        });
        self
    }

    pub fn selected(&self) -> usize {
        self.index
    }
}

impl Default for Tabs {
    fn default() -> Self {
        Self {
            index: 0,
            tabs: Vec::new(),
        }
    }
}
