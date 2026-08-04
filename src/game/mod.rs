use std::{path::Path, sync::Arc, time::Duration};

use winit::window::Window;

use crate::{engine::{Engine, assets::server, renderer::{bind_group, material, mesh, pipeline, transform}, scene::camera::CameraAction}, game::io::input::{GameActions, Input}};

mod io;

pub struct Game {
    pub engine: Engine,
    pub input_handler: io::input::Input,
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
        let transform_id_0 = engine.renderer.add_transform(
            transform::Transform::new()
        );
        let transform_id_1 = engine.renderer.add_transform(
            transform::Transform::new()
        );

        // upload to gpu
        engine.renderer.update_transform_snapshots();
        engine.renderer.update_transform(); 
        
        // get stuff renderable each loop
        let material_id = engine.renderer.add_material(material);
        let pipeline_id = engine.renderer.add_pipeline(pipeline);
        {
            let mesh0 = mesh::Mesh::make_quad(
                &device, 
                material_id,
                pipeline_id,
                transform_id_0,
                [100.0, 100.0], // pos
                [100.0, 100.0], // area
            );
            let _mesh_id = engine.renderer.add_mesh(mesh0);

            let mesh1 = mesh::Mesh::make_quad(
                &engine.renderer.get_render_context().device, 
                material_id,
                pipeline_id,
                transform_id_1,
                [100.0, 300.0], 
                [100.0, 100.0],
            );
            let _mesh_id = engine.renderer.add_mesh(mesh1);
        }
 
        
        return Self {
            engine,
            input_handler,
        };
    }

    pub fn frame(&mut self, dt: Duration, tick: u64) {
        let _ = tick;
        let _ = dt;
        let _ = self.engine.renderer.render();
    }

    /// called to update game as much as possible
    pub fn update(&mut self, _frame_time: Duration) {
        self.update_from_input();
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
                zoom_in, zoom_out, 
                rot_up, rot_down, rot_right, rot_left
            ] = inputs {
                if *up { 
                    camera.controller.action(CameraAction::MOVE_Y, 1.0);
				}
                if *down { 
                    camera.controller.action(CameraAction::MOVE_Y, -1.0);
				}
                if *right { 
                    camera.controller.action(CameraAction::MOVE_X, 1.0);
				}
                if *left { 
                    camera.controller.action(CameraAction::MOVE_X, -1.0);
				}
                if *forward { 
                    camera.controller.action(CameraAction::MOVE_Z, 1.0);
				}
                if *backward { 
                    camera.controller.action(CameraAction::MOVE_Z, -1.0);
				}
                if *zoom_in { 
                    camera.controller.action(CameraAction::ZOOM, 1.0);
				}
                if *zoom_out { 
                    camera.controller.action(CameraAction::ZOOM, -1.0);
				}
                if *rot_up { 
                    camera.controller.action(CameraAction::ROTATE_Y, 1.0);
				}
                if *rot_down { 
                    camera.controller.action(CameraAction::ROTATE_Y, -1.0);
				}
                if *rot_right { 
                    camera.controller.action(CameraAction::ROTATE_X, 1.0);
				}
                if *rot_left { 
                    camera.controller.action(CameraAction::ROTATE_X, -1.0);
				}
            }
            camera.controller.update(&mut camera.transform, 1.0);
        }
    }
}
