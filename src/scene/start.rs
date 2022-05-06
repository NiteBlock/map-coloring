use std::process;

use egui::{Color32, RichText, Ui};
use glob::glob;

use crate::cell::Map;

use super::{enter_names::EnterNames, generate_map::GenerateMap, Scene, SceneType};

#[derive(Clone)]
pub struct StartingScene(pub Box<Option<SceneType>>);

impl Scene for StartingScene {
    fn new() -> Self
    where
        Self: Sized,
    {
        StartingScene(Box::new(None))
    }
    fn draw(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            let button =
                egui::Button::new(RichText::new("Get Started").size(50.).color(Color32::WHITE))
                    .fill(Color32::BLUE);
            let resp = ui.add(button);
            if resp.clicked() {
                self.0 = Box::new(Some(SceneType::EnterNames(EnterNames::new())))
            }
        });
        ui.vertical_centered(|ui| {
            ui.menu_button(
                RichText::new("Load from file")
                    .size(50.)
                    .color(Color32::WHITE),
                |ui| {
                    for p in glob("./maps/*.toml").unwrap().filter_map(|t| t.ok()) {
                        if ui
                            .button(p.file_name().unwrap().to_str().unwrap())
                            .clicked()
                        {
                            self.0 = Box::new(Some(SceneType::GenerateMap(GenerateMap::from(
                                Map::from_file(&p),
                            ))))
                        }
                    }
                },
            );
        });
        // QUIT BUTTON
        ui.vertical_centered(|ui| {
            let button = egui::Button::new(RichText::new("Quit").size(50.).color(Color32::WHITE))
                .fill(Color32::RED);
            let resp = ui.add(button);
            if resp.clicked() {
                process::exit(0);
            }
        });
    }
    fn next_scene(&self) -> &Option<SceneType> {
        self.0.as_ref()
    }
}
