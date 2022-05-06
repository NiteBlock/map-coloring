use egui::{Color32, RichText, Ui};

use crate::cell::Map;

use super::{enter_names::EnterNames, generate_map::GenerateMap, Scene, SceneType};

#[derive(Clone)]
pub struct CreateLinks {
    map: Map,
    next: Box<Option<SceneType>>,
}

impl Scene for CreateLinks {
    fn new() -> CreateLinks {
        CreateLinks {
            map: Map::default(),
            next: Box::new(None),
        }
    }
    fn draw(&mut self, ui: &mut Ui) {
        let mut change: Option<(usize, usize)> = None;
        ui.columns(3, |cols| {
            for (i, cell) in self.map.0.iter().enumerate() {
                cols[1].vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.group(|ui| {
                            ui.label(RichText::new(cell.name.clone()).size(15.));
                            ui.menu_button("Links", |ui| {
                                for (j, cell2) in self.map.0.iter().enumerate() {
                                    if j == i {
                                        continue;
                                    }
                                    let mut is_checked = cell.connections.contains(&j);
                                    let checkbox = ui.checkbox(&mut is_checked, cell2.name.clone());
                                    if checkbox.changed() {
                                        change = Some((i, j))
                                    }
                                }
                            })
                        });
                    });
                });
            }
        });
        if let Some((i, j)) = change {
            self.map.0[i].link_changed(j);
            self.map.0[j].link_changed(i);
        }
        ui.add_space(520. - (self.map.0.len() as f32) * 33.);
        ui.vertical_centered(|ui| {
            let button =
                egui::Button::new(RichText::new("Go back").size(25.).color(Color32::WHITE))
                    .fill(Color32::RED);
            let resp = ui.add(button);
            if resp.clicked() {
                let mut next_scene = EnterNames::new();
                next_scene.set_names(self.map.0.iter().map(|t| t.name.clone()).collect());
                self.next = Box::new(Some(SceneType::EnterNames(next_scene)))
            }
            ui.add_space(10.);
            let button =
                egui::Button::new(RichText::new("Next Step").size(25.).color(Color32::WHITE))
                    .fill(Color32::BLUE);
            let resp = ui.add(button);
            if resp.clicked() {
                let next_scene = GenerateMap::from(self.map.clone());
                self.next = Box::new(Some(SceneType::GenerateMap(next_scene)))
            }
        });
    }
    fn next_scene(&self) -> &Option<SceneType> {
        self.next.as_ref()
    }
}

impl CreateLinks {
    pub fn add_names(&mut self, names: Vec<String>) -> &mut CreateLinks {
        for name in names {
            self.map.add_cell(name);
        }
        self
    }
}
