use std::{path::Path, sync::Arc, time::Duration};

use glam::{Vec2, Vec3};
use winit::window::Window;

use crate::{engine::{Engine, assets::server, renderer::{bind_group, material, mesh, pipeline, transform}, scene::camera::CameraAction}, game::io::input::{GameActions, Input}};

mod io;

struct RequireUpload {
    camera: bool,
}

pub struct Game {
    tick: u64,

    pub engine: Engine,
    pub input_handler: io::input::Input,

    pub upload_list: RequireUpload,
}

impl Game {
    pub fn init(window: Arc<Window>) -> Self {
        let mut engine = Engine::new(window.clone());
        let input_handler = Input::new();

        // testing engine
        let device = engine.renderer.get_render_context().device.clone();

        // gather test data
        let (shader_text, texture_raw) = pollster::block_on(async {
            tokio::try_join!(
                server::Server::preload_text(Path::new("shaders/shader.wgsl")),
                server::Server::preload_raw(Path::new("textures/brick-texture-54.png")),
            )
        }).expect("rip");

        let shader_handle = engine
            .asset_server
            .load_shader(shader_text, None, None)
            .expect("failed to load shader source");
        let texture_handle = engine
            .asset_server
            .load_texture(texture_raw)
            .expect("failed to load texture source");
        
        let material_layout = bind_group::LayoutBuilder::new(&device)
            .add_texture_view(
                wgpu::ShaderStages::FRAGMENT, 
                wgpu::TextureSampleType::Float { filterable: true }, 
                wgpu::TextureViewDimension::D2
            )
            .add_texture_sampler(
                wgpu::ShaderStages::FRAGMENT, 
                wgpu::SamplerBindingType::Filtering
            )
            .build("material bind group");
        
        let pipeline = {
            let contex = engine.renderer.get_render_context();
            pipeline::Builder::new(&contex.device)
                .set_shader(engine.asset_server.get_shader(shader_handle).unwrap())
                .set_pixel_format(contex.config.format)
                .add_buffer_layout(Some(mesh::Vertex::get_layout()))
                .add_bind_group_layout(&material_layout.layout.clone()) // idx: 0
                .add_bind_group_layout(&engine.renderer.get_transform_layout().layout.clone()) // idx: 1
                .add_bind_group_layout(&engine.renderer.camera.uploader.layout.layout.clone()) // idx: 2
                .build_pipeline("pipeline test")
        };
        
        let material = material::Material::new(
            "test material", 
            engine.asset_server.get_texture(texture_handle).unwrap(), 
            &device, 
            &material_layout
        );
       
        // get stuff renderable each loop
        let material_id = engine.renderer.add_material(material);
        let pipeline_id = engine.renderer.add_pipeline(pipeline);
        {
            // let camera = &engine.renderer.camera.transform;
            // let spawn_pos = camera.position + camera.forward_direction() * 5.0;
            // let size = Vec3::splat(1.0);
            // let pos = spawn_pos - size * 0.5;

            let mut transform0 = transform::Transform::default();
            transform0.position = Vec3::new(0.0, 2.0, -8.0);

            let transform_id = engine.renderer.add_transform(transform0);


            let mesh0 = mesh::Mesh::make_cube(
                &device, 
                material_id,
                pipeline_id,
                transform_id,
                [1.0, 1.0, 1.0], // volume
            );

            let _mesh_id = engine.renderer.add_mesh(mesh0);
            engine.renderer.update_transforms();

            // println!("{:?}", transform0.matrix());

        }

        
        return Self {
            tick: 0,
            engine,
            input_handler,
            upload_list: RequireUpload { 
                camera: false, 
            },
        };
    }

    /// called at monitor refresh rate
    pub fn frame(&mut self, _dt: Duration, _tick: u64) -> anyhow::Result<()> {
        // upload data to shaders 
        {
            let upload_list = &mut self.upload_list;
            if upload_list.camera {
                // self.engine.renderer.camera.uploader.upload(&self.engine.renderer.camera.transform);
                upload_list.camera = false;
            }
        }

        // update screen
        self.engine.renderer.render()?;

        // ---- RESET ----

        return Ok(());
    }

    /// called every tick
    pub fn update(&mut self, _dt: Duration, tick: u64) {
        self.update_from_input();

        // reseting inputs
        self.input_handler.mouse_delta = Vec2::ZERO;
        self.input_handler.mouse_scroll_delta = Vec2::ZERO;

        self.tick = tick;
    }

    /// all winit input events update game here
    fn update_from_input(&mut self) {
        { // camera
            let camera = &mut self.engine.renderer.camera;
            let inputs = &self.input_handler.action_state[
                (GameActions::CAMERA_UP as usize)..=(GameActions::CAMERA_ROTATE_LEFT as usize)
            ];
            // matches inputs to what type of change should happen to the camera
            if let [
                up, down, right, left, forward, backward,
                rot_up, rot_down, rot_right, rot_left
            ] = inputs {
                if *up { 
                    camera.controller.action(CameraAction::MOVE_Y, 0.2);
				}
                if *down { 
                    camera.controller.action(CameraAction::MOVE_Y, -0.2);
				}
                if *right { 
                    camera.controller.action(CameraAction::MOVE_X, 0.2);
				}
                if *left { 
                    camera.controller.action(CameraAction::MOVE_X, -0.2);
				}
                if *forward { 
                    camera.controller.action(CameraAction::MOVE_Z, 0.2);
				}
                if *backward { 
                    camera.controller.action(CameraAction::MOVE_Z, -0.2);
				}
                if *rot_up { 
                    camera.controller.action(CameraAction::ROTATE_Y, 0.2);
				}
                if *rot_down { 
                    camera.controller.action(CameraAction::ROTATE_Y, -0.2);
				}
                if *rot_right { 
                    camera.controller.action(CameraAction::ROTATE_X, 0.2);
				}
                if *rot_left { 
                    camera.controller.action(CameraAction::ROTATE_X, -0.2);
				}
            }

            camera.controller.action(CameraAction::ROTATE_X, self.input_handler.mouse_delta.x as f32 * 0.02);
            camera.controller.action(CameraAction::ROTATE_Y, self.input_handler.mouse_delta.y as f32 * -0.02);
            camera.controller.action(CameraAction::ZOOM, self.input_handler.mouse_scroll_delta.y);

            if camera.controller.update(&mut camera.transform, 0.1) {
                self.upload_list.camera = true;
                self.engine.renderer.camera.uploader.upload(&self.engine.renderer.camera.transform);
            }
        }
    }
}















