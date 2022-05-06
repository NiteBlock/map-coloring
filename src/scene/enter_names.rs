use egui::{Color32, RichText, Ui};

use super::{create_links::CreateLinks, Scene, SceneType};

#[derive(Clone)]
pub struct EnterNames {
    names: Vec<String>,
    next: Box<Option<SceneType>>,
}

impl Scene for EnterNames {
    fn new() -> Self
    where
        Self: Sized,
    {
        EnterNames {
            names: vec!["".to_string()],
            next: Box::new(None),
        }
    }
    fn draw(&mut self, ui: &mut Ui) {
        let mut new = Vec::new();
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new("Enter Country/State Names")
                    .size(50.)
                    .color(Color32::BLACK),
            );
        });
        ui.columns(3, |cols| {
            for name in self.names.iter() {
                cols[1].vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.group(|ui| {
                            let mut t = name.clone();
                            ui.add(egui::TextEdit::singleline(&mut t));
                            let button = egui::Button::new(
                                RichText::new("Delete").size(15.).color(Color32::WHITE),
                            )
                            .fill(Color32::RED);
                            let resp = ui.add(button);
                            if !resp.clicked() {
                                new.push(t);
                            }
                        });
                    });
                });
            }
        });
        ui.add_space(520. - (new.len() as f32) * 33.);
        ui.vertical_centered(|ui| {
            if new.len() < 17 {
                let button =
                    egui::Button::new(RichText::new("Create").size(25.).color(Color32::WHITE))
                        .fill(Color32::GREEN);
                let resp = ui.add(button);
                if resp.clicked() {
                    new.push("".to_string())
                }
                ui.add_space(10.)
            } else {
                ui.add_space(45.)
            }
            let button =
                egui::Button::new(RichText::new("Next Step").size(25.).color(Color32::WHITE))
                    .fill(Color32::BLUE);
            let resp = ui.add(button);
            if resp.clicked() {
                let mut next_scene = CreateLinks::new();
                next_scene.add_names(new.clone());
                self.next = Box::new(Some(SceneType::CreateLinks(next_scene)))
            }
        });
        self.names = new;
    }
    fn next_scene(&self) -> &Option<SceneType> {
        self.next.as_ref()
    }
}

impl EnterNames {
    pub fn set_names(&mut self, names: Vec<String>) {
        self.names = names;
    }
}
