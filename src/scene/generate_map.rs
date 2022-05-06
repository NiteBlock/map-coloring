use crate::{cell::Map, utility::file::get_next_file_path};
use egui::{Color32, RichText, Ui};
use std::{collections::HashMap, fs::File, io::Write};

use super::{start::StartingScene, Scene, SceneType};

#[derive(Clone)]
pub struct GenerateMap {
    map: Map,
    next: Box<Option<SceneType>>,
    colorable: bool,
    saved: bool,
    iterations: usize,
}

impl Scene for GenerateMap {
    fn new() -> GenerateMap {
        GenerateMap {
            map: Map::default(),
            next: Box::new(None),
            colorable: true,
            iterations: 0,
            saved: false,
        }
    }
    fn draw(&mut self, ui: &mut Ui) {
        if !self.colorable {
            ui.vertical_centered(|ui| {
                ui.colored_label(Color32::RED, "The current map is not colorable!");
            });
        }
        if self.saved {
            ui.vertical_centered(|ui| {
                ui.colored_label(Color32::RED, "Map saved! Load it in on the home screen!");
            });
        }
        if self.iterations != 0 {
            ui.vertical_centered(|ui| {
                ui.colored_label(Color32::RED, format!("Took {} iterations", self.iterations));
            });
        }

        for chunk in self.map.0.iter().as_slice().chunks(10) {
            ui.columns(3, |cols| {
                cols[1].horizontal(|ui| {
                    for cell in chunk {
                        ui.group(|ui| {
                            ui.colored_label(
                                cell.clone().color(),
                                RichText::new(cell.name.clone()).size(15.),
                            );
                        });
                    }
                });
            })
        }

        ui.add_space(520. - ((self.map.0.len() / 10) as f32) * 33.);
        ui.vertical_centered(|ui| {
            let button = egui::Button::new(
                RichText::new("Color the map")
                    .size(25.)
                    .color(Color32::WHITE),
            )
            .fill(Color32::BLUE);
            let resp = ui.add(button);
            if resp.clicked() {
                // Start the coloring
                let t = self.map.color_map();
                if !t.0 {
                    self.colorable = false;
                } else {
                    self.iterations = t.1
                }
            }
            ui.add_space(10.);
            let button =
                egui::Button::new(RichText::new("Start Over").size(25.).color(Color32::WHITE))
                    .fill(Color32::RED);
            let resp = ui.add(button);
            if resp.clicked() {
                self.next = Box::new(Some(SceneType::Start(StartingScene::new())))
            }
            let button =
                egui::Button::new(RichText::new("Save Map").size(25.).color(Color32::WHITE))
                    .fill(Color32::from_rgb(255, 0, 255));
            let resp = ui.add(button);
            if resp.clicked() {
                let mut file = File::create(get_next_file_path()).unwrap();
                file.write_all(
                    toml::to_string::<HashMap<String, Vec<String>>>(&self.map.clone().into())
                        .expect("Cannot convert")
                        .as_bytes(),
                )
                .unwrap();
                self.saved = true;
            }
        });
    }
    fn next_scene(&self) -> &Option<SceneType> {
        self.next.as_ref()
    }
}

impl From<Map> for GenerateMap {
    fn from(map: Map) -> GenerateMap {
        GenerateMap {
            map,
            next: Box::new(None),
            colorable: true,
            saved: false,
            iterations: 0,
        }
    }
}
