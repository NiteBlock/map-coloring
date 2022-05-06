use egui::Ui;

use self::{
    create_links::CreateLinks, enter_names::EnterNames, generate_map::GenerateMap,
    start::StartingScene,
};

pub mod create_links;
pub mod enter_names;
pub mod generate_map;
pub mod start;

pub trait Scene: Clone {
    fn draw(&mut self, _ui: &mut Ui) {}
    fn new() -> Self
    where
        Self: Sized;
    fn next_scene(&self) -> &Option<SceneType> {
        &None
    }
}

#[derive(Clone)]
pub enum SceneType {
    Start(StartingScene),
    EnterNames(EnterNames),
    CreateLinks(CreateLinks),
    GenerateMap(GenerateMap),
}

impl Scene for SceneType {
    fn draw(&mut self, ui: &mut Ui) {
        match self {
            SceneType::Start(t) => t.draw(ui),
            SceneType::EnterNames(t) => t.draw(ui),
            SceneType::CreateLinks(t) => t.draw(ui),
            SceneType::GenerateMap(t) => t.draw(ui),
        }
    }
    fn new() -> SceneType {
        SceneType::Start(StartingScene::new())
    }
    fn next_scene(&self) -> &Option<SceneType> {
        match self {
            SceneType::Start(t) => t.next_scene(),
            SceneType::EnterNames(t) => t.next_scene(),
            SceneType::CreateLinks(t) => t.next_scene(),
            SceneType::GenerateMap(t) => t.next_scene(),
        }
    }
}
