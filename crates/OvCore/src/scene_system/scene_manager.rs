use std::sync::Arc;

use OvTools::utils::r#ref::Ref;

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
    pub fn loadScene(scene: Scene) -> Ref<Self> {
        Ref::new(Self {
            currentScene: Some(Scene::new()),
        })
    }
    pub fn loadSceneFromStr(&mut self, scene: &str, res: Arc<ResourceManager>) {
        self.currentScene = Some(Scene::load(scene, res));
    }
    pub fn getCurrentScene(&self) -> &Option<Scene> {
        &self.currentScene
    }
    pub fn getCurrentSceneMut(&mut self) -> &mut Option<Scene> {
        &mut self.currentScene
    }
}
