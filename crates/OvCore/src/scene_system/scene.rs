use deno_core::{v8, JsRealm};
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use thunderdome::Index;
use OvRender::resources::{Mesh, Texture, UniformInfo};

use crate::ecs::components::material_render::MaterialRender;
use crate::ecs::components::mesh_render::MeshRender;
use crate::ecs::{components::camera::Camera, game_object::GameObject, graph::Graph};
use crate::resources::ResourceManager;

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    graph: Graph,
    camera: Cell<Option<Index>>,
    canvas: Cell<Option<Index>>,
    html: Cell<Option<Index>>,
}

impl Deref for Scene {
    type Target = Graph;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

impl DerefMut for Scene {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graph
    }
}

impl Scene {
    pub fn new() -> Self {
        Self {
            graph: Graph::default(),
            camera: Cell::new(None),
            canvas: Cell::new(None),
            html: Cell::new(None),
        }
    }

    pub fn getMainHtmlRender(&self) -> Option<Index> {
        if self.html.get().is_none() {
            let html = self
                .graph
                .iter()
                .find(|obj| obj.1.getComponentBoxByName("HtmlRender").is_some())
                .map(|handle| handle.0);
            self.html.set(html);
        }
        self.html.get()
    }

    pub fn getMainCanvas(&self) -> Option<Index> {
        if self.canvas.get().is_none() {
            let canvas = self
                .graph
                .iter()
                .find(|obj| obj.1.getComponentBoxByName("Canvas").is_some())
                .map(|handle| handle.0);
            self.canvas.set(canvas);
        }
        self.canvas.get()
    }

    pub fn getMainCamera(&self) -> Option<Index> {
        if self.camera.get().is_none() {
            let camera = self
                .graph
                .iter()
                .find(|obj| obj.1.getComponent::<Camera>().is_some())
                .map(|handle| handle.0);
            self.camera.set(camera);
        }
        self.camera.get()
    }

    pub fn update(&mut self, dt: f32, js: JsRealm, isolate: &mut v8::OwnedIsolate) {
        for (_, go) in self.graph.iter_mut() {
            go.update(dt, js.clone(), isolate);

            for (_, children) in go.children.iter_mut() {
                children.update(dt, js.clone(), isolate)
            }
        }
    }

    pub fn save(&self) -> String {
        ron::to_string(&self).unwrap()
    }

    pub fn load(config: &str, res: Arc<ResourceManager>) -> Self {
        let mut scene: Self = ron::from_str(config).unwrap();
        for (_, go) in scene.iter_mut() {
            if let Some(meshRender) = go.getComponentMut::<MeshRender>() {
                let mut models = meshRender.getModels().clone();

                for model in models.iter_mut() {
                    for mesh in model.iter_mut() {
                        let mut result = Mesh::new(mesh.getName());
                        if let Some(index) = mesh.getMaterialIndex() {
                            result.setMaterialIndex(index);
                        }
                        *mesh = result;
                    }
                }
                meshRender.setModels(models);
            }
            if let Some(materialRender) = go.getComponentMut::<MaterialRender>() {
                let mut materials = materialRender.getMaterialList().clone();
                for material in materials.iter_mut() {
                    let mut uniformList = material.getUniformList().clone();
                    for uniform in uniformList.iter_mut() {
                        match uniform {
                            UniformInfo::Texture(texture) => {
                                let image = res
                                    .get(texture.getName())
                                    .expect(&format!("无法加载图片：{}", texture.getName()));
                                *texture = Texture::new(image);
                            }
                        }
                    }

                    material.setUniformList(uniformList);
                }
                materialRender.setMaterialList(materials);
            }
        }

        scene
    }

    pub fn addChild(&mut self, go: GameObject) -> Index {
        let index = self.insert(go);
        self[index].setRoot(index);
        index
    }
    pub fn getGameObject(&self, name: &str) -> Option<Index> {
        self.iter()
            .find(|(_, go)| go.getName() == name)
            .map(|go| go.0)
    }
}

#[cfg(test)]
mod test {
    use std::cell::Cell;

    use crate::{
        ecs::{component::Component, components::camera::Camera, game_object::GameObject},
        scene_system::scene::Scene,
    };

    #[test]
    fn cell() {
        let cell = Cell::new(456);
        println!("{}", cell.get());
        println!("{}", cell.get());
    }

    #[test]
    fn serde() {
        let mut scene = Scene::new();
        let mut obj = GameObject::default();
        obj.addComponent(Component::new(Camera::new()));
        scene.insert(obj);
        let str = ron::to_string(&scene).unwrap();
        println!("{:#?}", str);
        let ex: Scene = ron::from_str(&str).unwrap();
        println!("{:#?}", ex);
    }
}
