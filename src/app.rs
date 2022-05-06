use egui::{Color32, RichText, Vec2};
use epi::App;

use crate::scene::{Scene, SceneType};

pub struct MapColoringApp {
    scene: SceneType, // the current scene that contains the scene and does the scene.
}

impl Default for MapColoringApp {
    fn default() -> MapColoringApp {
        MapColoringApp {
            scene: SceneType::new(), // create a default scene
        }
    }
}

impl App for MapColoringApp {
    fn name(&self) -> &str {
        "Map Coloring" // the name of the application
    }
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        frame.set_window_size(Vec2::new(1000., 1000.));
        if let Some(scene) = (self).scene.next_scene() {
            // if there is a new scene to be created or changed or whatever
            self.scene = scene.clone(); // update it
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            // show the default panel
            ui.vertical_centered(|ui| {
                ui.heading(
                    RichText::new("Map Coloring")
                        .size(70.)
                        .color(Color32::DARK_GRAY),
                ); // create the main heading
            });
            ui.add_space(20.);
            self.scene.draw(ui); // draw the current scene
        });
    }
}
