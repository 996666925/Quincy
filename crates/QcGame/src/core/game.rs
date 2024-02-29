use std::{
    any::TypeId,
    fmt::format,
    ops::Deref,
    ptr::null,
    sync::{Arc, RwLock},
};

use env_logger::fmt::Color;
use log::debug;
use nalgebra::{Point3, Vector3};
use thunderdome::Index;
use QcCore::{
    ecs::{
        component::{Component, Named, V8},
        components::{
            camera::Camera, material_render::MaterialRender, mesh_render::MeshRender,
            transform::Transform,
        },
        game_object::GameObject,
    },
    resources::material::Material,
    scene_system::scene::Scene,
};
use QcMacros::Comp;
use QcRender::resources::Mesh;
use QcRender::{
    gl,
    resources::{Model, Texture},
};
use QcScript::{core::JsComponent, utils::GoExt, v8};
use QcTools::time::clock::Clock;
use QcUI::{
    component::{Button, ButtonMessage, Canvas, Image, Label, Panel, TextBox, UiNode, Widget},
    core::uiBind::UiBind,
    message::UiMessageType,
    panel,
    prelude::FlexDirection,
    Color32, Margin, RetainedImage,
};
use QcWindowing::{event::WindowEvent, event_loop::ControlFlow};

use super::{context::Context, game_render::GameRender};

pub struct Game {
    gameRender: Arc<GameRender>,
    context: Arc<Context>,
    elapsed: f32,
    debugDraw: Canvas,
    fps: Index,
}

impl Game {
    ///初始化场景(暂时)
    pub fn createScene(context: Arc<Context>) {
        let mut sceneManagerRef = context.sceneManager.try_write().unwrap();
        // sceneManagerRef.loadSceneFromStr(
        //     include_str!("../../assets/main.scene"),
        //     context.resourceManager.clone(),
        // );

        {
            let currentScene = sceneManagerRef.getCurrentSceneMut();
            if let Some(currentScene) = currentScene {
                let camera = Component::new(Camera::new());
                let transform = Component::new(Transform::new(Point3::new(0., 0., 0.)));
                let mut obj = GameObject::new("Camera");
                obj.insert(camera);
                obj.insert(transform);
                // obj.insert(Component::new(Example));
                currentScene.insert(obj);

                let mut transform = Transform::new(Point3::new(0., 0., -3.));
                transform.setRotation(Vector3::new(0., 45f32.to_radians(), 0.));

                let obj = GameObject::default();

                let objId = currentScene.insert(obj);

                let obj = &mut currentScene[objId];

                let mut meshRender = MeshRender::new();
                let mut model = Mesh::new("monkey.mesh");
                model.setMaterialIndex(0);

                meshRender.addModel(model.into());

                let mut materialRender = MaterialRender::new();
                let mut material = Material::new("standard");
                let image = context.resourceManager.get("texture.dds").unwrap();
                let texture = Texture::new(image);
                material.addTexture(texture);
                materialRender.addMaterial(material);

            
                let mut canvas = Canvas::new();

                let mut panel = Panel::new()
                    .orientation(FlexDirection::Column)
                    .background(Color32::YELLOW)
                    .margin(Margin::symmetric(100., 100.))
                    .width(400.)
                    .height(400.)
                    .spacing(100.);

                {
                    let mut panel1: Panel = Panel::new().spacing(20.);
                    let button = Button::new("确定");
                    let index = panel1.addChild(UiNode::new(button));

                    // canvas.addUiBind(
                    //     index,
                    //     UiBind::new(
                    //         objId,
                    //         compId,
                    //         "onClick".to_string(),
                    //         UiMessageType::ButtonMessage(ButtonMessage::Clicked),
                    //     ),
                    // );

                    let button = Button::new("确定");
                    panel1.addChild(UiNode::new(button));
                    let button = Button::new("确定");
                    panel1.addChild(UiNode::new(button));
                    let button = Button::new("确定");
                    panel1.addChild(UiNode::new(button));

                    let mut image = Image::new();

                    let imgFile = RetainedImage::from_image_bytes(
                        "user.jpg",
                        include_bytes!("../../assets/user.jpg"),
                    )
                    .unwrap();
                    image.setTexture("user.jpg", Some(imgFile));

                    panel1.addChild(UiNode::new(image));

                    panel.addChild(UiNode::new(panel1));
                }
                {
                    let mut panel1 = Panel::new()
                        .orientation(FlexDirection::Column)
                        // .margin(Margin::same(50.))
                        .background(Color32::LIGHT_BLUE)
                        .spacing(20.);
                    let button = Button::new("确定");
                    panel1.addChild(UiNode::new(button));
                    let button = Button::new("确定");
                    panel1.addChild(UiNode::new(button));
                    let button = Button::new("确定");
                    panel1.addChild(UiNode::new(button));
                    let button = Button::new("确定");
                    // panel1.addChild(UiNode::new(button));
                    let textbox = TextBox::new("确定");
                    panel1.addChild(UiNode::new(textbox));
                    panel.addChild(UiNode::new(panel1));
                }

                canvas.addChild(UiNode::new(panel));
                let mut image = Image::new();

                let imgFile = RetainedImage::from_image_bytes(
                    "user.jpg",
                    include_bytes!("../../assets/user.jpg"),
                )
                .unwrap();
                image.setTexture("user.jpg", Some(imgFile));
                canvas.addChild(UiNode::new(image));
                obj.insert(Component::new(transform));
                obj.insert(Component::new(meshRender));
                obj.insert(Component::new(materialRender));

                let cube = JsComponent::new("Cube", None);
                obj.insert(Component::new(cube));
                // obj.insert(Component::new(cube));
                // obj.insert(Component::new(canvas));

                // println!("{}", currentScene.save());
            }
        }

        let mut jsManager = context.jsRuntimeManager.try_write().unwrap();

        let scope = &mut jsManager.handle_scope();

        let context = scope.get_current_context();

        let global = context.global(scope);

        let currentScene = sceneManagerRef.getCurrentSceneMut().as_mut().unwrap();

        for (_, go) in currentScene.iter_mut() {
            for (_, comp) in go.iter_mut() {
                if comp.type_id() == TypeId::of::<JsComponent>() {
                    let jsComp = comp.castMut::<JsComponent>().unwrap();

                    let jsValue = {
                        let objName =
                            v8::String::new(scope, &format!("##{}##", jsComp.getName())).unwrap();
                        let obj = global.get(scope, objName.into()).unwrap();

                        let obj = v8::Local::<v8::Function>::try_from(obj).unwrap();

                        let undefined = v8::undefined(scope);
                        let obj = obj.call(scope, undefined.into(), &[]).unwrap();

                        obj
                    };

                    jsComp.setValue(Some(v8::Global::new(scope, jsValue).into()))
                }
            }
        }

        for (_, go) in currentScene.iter_mut() {
            let name = go.getName().to_string();
            for (_, comp) in go.iter_mut() {
                if comp.type_id() == TypeId::of::<JsComponent>() {
                    let jsComp = comp.castMut::<JsComponent>().unwrap();

                    let comp = v8::Local::<v8::Value>::new(scope, jsComp.getV8Value());
                    GoExt::setParentName(comp, scope, &name);
                    GoExt::onStart(comp, scope);
                }
            }
        }
    }

    pub fn new(context: Arc<Context>) -> Self {
        //加载js文件和scene文件
        {
            let mut jsRuntimeManager = context.jsRuntimeManager.try_write().unwrap();
            jsRuntimeManager
                .op_state()
                .borrow_mut()
                .put(context.sceneManager.clone());
            let sceneManager = context.sceneManager.clone();
            let mut sceneManager = sceneManager.try_write().unwrap();
            let currentScene = sceneManager.getCurrentSceneMut().as_mut().unwrap();
            jsRuntimeManager
                .op_state()
                .borrow_mut()
                .put(currentScene as *mut Scene);
            jsRuntimeManager
                .execute_script_static("ov", include_str!("../../../QcJs/dist/quincy.umd.cjs"))
                .unwrap();
        }

        Self::createScene(context.clone());
        // {
        //     if let Some(scene) = context
        //         .sceneManager
        //         .try_write()
        //         .unwrap()
        //         .getCurrentSceneMut()
        //     {
        //         if let Some(index) = scene.getMainCanvas() {
        //             if let Some(canvas) = scene[index].getComponentMut::<Canvas>() {

        //             }
        //         }
        //     }
        // }

        let fps = Label::new(Widget::default().with_foreground(Color32::RED)).with_text("fps");

        let mut canvas = Canvas::new();
        let index = canvas.addChild(UiNode::new(fps));

        let gameRender = GameRender::new(context.clone());
        Self {
            gameRender,
            context,
            elapsed: 0.,
            fps: index,
            debugDraw: canvas,
        }
    }
    pub fn preUpdate(&self, event: &WindowEvent) {
        let result = self
            .context
            .uiManager
            .try_write()
            .unwrap()
            .handleEvent(event);

        match event {
            WindowEvent::MouseInput { state, .. } => {}
            _ => {
                if result.consumed {
                    return;
                }
            }
        }

        let jsManager = self.context.jsRuntimeManager.clone();
        self.context
            .inputManager
            .try_write()
            .unwrap()
            .handleEvent(event, &mut jsManager.try_write().unwrap().handle_scope());
    }

    pub fn update(&mut self, clock: &Clock) {
        self.elapsed += clock.getDeltaTime();
        if self.elapsed > 1. {
            let label = self.debugDraw[self.fps].castMut::<Label>().unwrap();
            label.setText(&format!("fps: {}", clock.getFrameRate()));

            self.elapsed = 0.;
        }

        let window = self.context.window.try_read().unwrap();
        {
            let mut jsRuntimeManager = self.context.jsRuntimeManager.try_write().unwrap();
            let jsRuntime = jsRuntimeManager.main_realm();

            let mut sceneManager = self.context.sceneManager.try_write().unwrap();
            let currentScene = sceneManager.getCurrentSceneMut().as_mut().unwrap();
            jsRuntimeManager
                .op_state()
                .borrow_mut()
                .put::<*mut Scene>(currentScene);
            currentScene.update(
                clock.getDeltaTime(),
                jsRuntime,
                jsRuntimeManager.v8_isolate(),
            );
        }

        {}

        //渲染游戏场景
        self.gameRender.renderScene();

        //渲染游戏ui
        {
            let mut jsRuntimeManager = self.context.jsRuntimeManager.try_write().unwrap();

            let mut sceneManager = self.context.sceneManager.try_write().unwrap();
            let currentScene = sceneManager.getCurrentSceneMut().as_mut().unwrap();

            if let Some(index) = currentScene.getMainCanvas() {
                let canvas = currentScene[index].getComponentMut::<Canvas>().unwrap();
                let mut uiManager = self.context.uiManager.try_write().unwrap();

                uiManager.render(&window, canvas, &mut self.debugDraw);

                uiManager.update(canvas, &mut jsRuntimeManager.handle_scope());
            }
        }
    }

    pub fn postUpdate(&self) {
        self.context.device.swapBuffers();
    }
}
