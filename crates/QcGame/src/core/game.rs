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
            skybox::SkyBox, transform::Transform,
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
    component::{
        Button, ButtonMessage, Canvas, Image, ImageLoader, Label, Panel, TextBox, UiNode, Widget,
    },
    core::uiBind::UiBind,
    egui,
    message::UiMessageType,
    panel,
    prelude::FlexDirection,
    Align2, Color32, Margin, RetainedImage,
};
use QcWindowing::{event::WindowEvent, event_loop::ControlFlow, Window};

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
                let mut skybox = SkyBox::new();
                // let image = context.resourceManager.get("texture.dds").unwrap();
                // let texture = Texture::new(image);
                // skybox.material.addTexture(texture);

                let transform = Component::new(Transform::new(Point3::new(0., 0., 0.)));
                let mut obj = GameObject::new("Camera");
                obj.insert(camera);
                obj.insert(Component::new(skybox));
                obj.insert(transform);
                // obj.insert(Component::new(Example));
                currentScene.add_child(obj);

                let mut parent = GameObject::default();
                let transform = Component::new(Transform::new(Point3::new(0., 0., -5.)));
                let tf = parent.addComponent(transform);

                // parent.addComponent(component)

                let obj = GameObject::default();

                let parent = currentScene.add_child(parent);

                let objId = currentScene.add_child_with_parent(obj, Some(parent));

                let obj = &mut currentScene[objId];

                let mut transform = Transform::new(Point3::new(0., 0., -3.));
                transform.set_rotation(Vector3::new(0., 45f32.to_radians(), 0.));

                let mut meshRender = MeshRender::new();
                let mut model = Mesh::new("monkey.mesh");
                model.setMaterialIndex(0);

                meshRender.addModel(model.into());

                let mut materialRender = MaterialRender::new();
                let mut material = Material::default();
                let image = context.resourceManager.get("texture.dds").unwrap();
                let texture = Texture::new(image);
                material.addTexture(texture);
                materialRender.addMaterial(material);

                {
                    let mut canvas = Canvas::new();

                    let mut panel = Panel::new(
                        Widget::default()
                            .with_background(Color32::YELLOW)
                            .with_margin(Margin::symmetric(100., 100.))
                            .with_width(400.)
                            .with_height(400.),
                    )
                    .with_orientation(FlexDirection::Row)
                    .with_spacing(100.);

                    {
                        let mut panel1: Panel = Panel::new(
                            Widget::default()
                                .with_background(Color32::BLACK)
                                .with_padding(Margin::same(10.)),
                        )
                        // .with_orientation(FlexDirection::Column)
                        .with_spacing(20.);
                        let button = Button::default().with_text("OnClick");
                        let index = button.uuid;
                        panel1.addChild(UiNode::new(button));

                        let cube = JsComponent::new("Cube", None);
                        let compId = obj.insert(Component::new(cube));
                        // canvas.addUiBind(
                        //     index,
                        //     UiBind::new(
                        //         objId,
                        //         compId,
                        //         "onClick".to_string(),
                        //         UiMessageType::ButtonMessage(ButtonMessage::Clicked),
                        //     ),
                        // );

                        let button = Button::default().with_text("确定");
                        panel1.addChild(UiNode::new(button));
                        let button = Button::default().with_text("确定");
                        panel1.addChild(UiNode::new(button));
                        let button = Button::default().with_text("确定");
                        panel1.addChild(UiNode::new(button));

                        //     let imgFile = RetainedImage::from_image_bytes(
                        //         "user.jpg",
                        //         include_bytes!("../../assets/user.jpg"),
                        //     )
                        //     .unwrap();
                        //     let mut image = Image::default().with_texture("user.jpg", Some(imgFile));

                        //     panel1.addChild(UiNode::new(image));

                        panel.addChild(UiNode::new(panel1));
                    }
                    {
                        let mut panel1 =
                            Panel::new(Widget::default().with_background(Color32::LIGHT_BLUE))
                                .with_orientation(FlexDirection::Column)
                                .with_spacing(20.);
                        let button = Button::default().with_text("确定");
                        panel1.addChild(UiNode::new(button));
                        let button = Button::default().with_text("确定");
                        panel1.addChild(UiNode::new(button));
                        let button = Button::default().with_text("确定");
                        panel1.addChild(UiNode::new(button));
                        let button = Button::default().with_text("确定");
                        panel1.addChild(UiNode::new(button));
                        panel.addChild(UiNode::new(panel1));
                    }

                    {
                        let mut panel1 =
                            Panel::new(Widget::default().with_background(Color32::LIGHT_RED))
                                .with_orientation(FlexDirection::Column)
                                .with_spacing(20.);

                        let createTextbox = |align: Align2| {
                            TextBox::new(Widget::default().with_height(100.).with_width(100.))
                                .with_text("确定")
                                .with_align(align)
                        };

                        let textbox = createTextbox(Align2::LEFT_TOP);
                        panel1.addChild(UiNode::new(textbox));
                        let textbox = createTextbox(Align2::LEFT_CENTER);
                        panel1.addChild(UiNode::new(textbox));
                        let textbox = createTextbox(Align2::LEFT_BOTTOM);
                        panel1.addChild(UiNode::new(textbox));
                        let textbox = createTextbox(Align2::CENTER_TOP);
                        panel1.addChild(UiNode::new(textbox));
                        panel.addChild(UiNode::new(panel1));
                    }

                    canvas.addChild(UiNode::new(panel));

                    // let imgFile = RetainedImage::from_image_bytes(
                    //     "user.jpg",
                    //     include_bytes!("../../assets/user.jpg"),
                    // )
                    // .unwrap();

                    let image = context.resourceManager.get("user.jpg").unwrap();
                    let img = RetainedImage::load_texture(&image);
                    let image = Image::default().with_texture(img);

                    canvas.addChild(UiNode::new(image));
                    // obj.insert(Component::new(canvas));
                }
                obj.addComponent(Component::new(transform));
                obj.addComponent(Component::new(meshRender));
                obj.addComponent(Component::new(materialRender));

                // obj.addComponent(Component::new(cube));

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
        let window = self.context.window.try_read().unwrap();

        let result = self
            .context
            .uiManager
            .try_write()
            .unwrap()
            .handleEvent(&window, event);

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

            let mut canvasList = vec![&mut self.debugDraw];
            if let Some(index) = currentScene.get_main_canvas() {
                let canvas = currentScene[index].getComponentMut::<Canvas>().unwrap();

                canvasList.push(canvas);
            }

            let mut uiManager = self.context.uiManager.try_write().unwrap();

            uiManager.render(&window, &mut canvasList);

            uiManager.update(canvasList, &mut jsRuntimeManager.handle_scope());
        }
    }

    pub fn postUpdate(&self) {
        self.context.device.swapBuffers();
    }

    pub fn destory(&mut self) {
        let mut ui = self.context.uiManager.try_write().unwrap();
        ui.destory();
    }
}
