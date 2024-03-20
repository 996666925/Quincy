use std::sync::Arc;

use QcTools::utils::r#ref::Ref;

use crate::resources::ResourceManager;

use super::scene::Scene;


#[derive(Debug)]
pub struct SceneManager {
    currentScene: Option<Scene>,
}

impl SceneManager {
    pub fn new() -> Ref<Self> {
        Ref::new(Self {
            currentScene: Some(Scene::new()),
        })
    }
    pub fn load_scene(scene: Scene) -> Ref<Self> {
        Ref::new(Self {
            currentScene: Some(Scene::new()),
        })
    }
    pub fn load_scene_from_str(&mut self, scene: &str, res: Arc<ResourceManager>) {
        self.currentScene = Some(Scene::load(scene, res));
    }
    pub fn get_current_scene(&self) -> &Option<Scene> {
        &self.currentScene
    }
    pub fn get_current_scene_mut(&mut self) -> &mut Option<Scene> {
        &mut self.currentScene
    }
}
