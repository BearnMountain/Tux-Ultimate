mod io;

use std::{path::Path, sync::Arc, time::Duration};

use glam::{Vec2, Vec3};
use winit::window::Window;

use crate::{
    engine::{
        Engine, assets::server::Server, physics::{
            body::RigidBody, collider::Collider
        }, renderer::{RenderResources, bind_group::{self, LayoutBuilder}, material::{Material, MaterialID}, mesh::{self, Mesh}, model_loader::Model, pipeline::{self, PipelineID}, transform::Transform}, scene::camera::CameraAction}, game::io::input::{GameActions, Input}
};

struct RequireUpload {
    camera: bool,
}

pub struct Game {
    tick: u64,
    frames: u64,

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
        let queue = engine.renderer.get_render_context().queue.clone();

        // gather test data
        let (shader_text, texture_raw, gltf_json, gltf_bin) = pollster::block_on(async {
            tokio::try_join!(
                Server::preload_text(Path::new("shaders/shader.wgsl")),
                Server::preload_raw(Path::new("textures/brick-texture-54.png")),
                Server::preload_text(Path::new("characters/test/tux/scene.gltf")),
                Server::preload_raw(Path::new("characters/test/tux/scene.bin")),
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
        
        let material_layout = LayoutBuilder::new(&device)
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
        
        let material = Material::new(
            "test material", 
            engine.asset_server.get_texture(texture_handle).unwrap(), 
            &device, 
            &material_layout
        );
       
        // get stuff renderable each loop
        let _material_id = engine.renderer.add_material(material);
        let _pipeline_id = engine.renderer.add_pipeline(pipeline);

        // load models
        {
            let _model = Model::create_from_gltf(
                &gltf_json,
                &device,
                &queue,
            );
        }
        
        return Self {
            tick: 0,
            frames: 0,
            engine,
            input_handler,
            upload_list: RequireUpload { 
                camera: false, 
            },
        };
    }

    pub fn setup_game(&mut self) {
        self.create_cube(
            0, 
            0, 
            Vec3::new(0.0, 10.0, -8.0), 
            [1.0, 1.0, 1.0], 
            false
        );

        self.create_cube(
            0, 
            0, 
            Vec3::new(-5.0, -5.0, -13.0), 
            [10.0, 1.0, 10.0], 
            true
        );

        self.engine.renderer.update_transforms();
    }

    /// called at monitor refresh rate
    pub fn frame(&mut self, _dt: Duration, _tick: u64) -> anyhow::Result<()> {
        // upload data to shaders 
        {
            let upload_list = &mut self.upload_list;
            if upload_list.camera {
                self.engine.renderer.camera.uploader.upload(&self.engine.renderer.camera.transform);
                upload_list.camera = false;
            }
        }

        // update screen
        self.engine.renderer.render()?;

        // ---- RESET ----

        return Ok(());
    }

    /// called every tick
    pub fn update(&mut self, _dt: Duration) {
        self.tick += 1;

        self.update_from_input();
        self.engine.physics_world.update(
            1.0/60.0, 
            self.engine.renderer.get_transform_cache()
        );

        // reseting inputs
        self.input_handler.mouse_delta = Vec2::ZERO;
        self.input_handler.mouse_scroll_delta = Vec2::ZERO;
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
                // self.engine.renderer.camera.uploader.upload(&self.engine.renderer.camera.transform);
            }
        }

        { // player
            let player = &mut self.engine.physics_world.bodies[0];
            player.desired_direction.x = 0.0;
            player.desired_direction.y = 0.0;

            if self.input_handler.action_state[GameActions::PLAYER_LEFT as usize] {
                player.desired_direction.x -= 0.2;
            }

            if self.input_handler.action_state[GameActions::PLAYER_RIGHT as usize] {
                player.desired_direction.x += 0.2;
            }

            if self.input_handler.action_state[GameActions::PLAYER_UP as usize] {
                player.desired_direction.y += 30.0;
            }

            if self.input_handler.action_state[GameActions::PLAYER_DOWN as usize] {
                player.desired_direction.y -= 0.2;
            }

            if self.input_handler.action_state[GameActions::PLAYER_ROTATE as usize] {
                player.apply_torque(30.0);
            }

            self.engine.renderer.update_transforms();
        }
    }

    fn create_cube(
        &mut self,
        pipeline_id: PipelineID,
        material_id: MaterialID,
        position: Vec3,
        volume: [f32; 3], // width, height, depth
        is_static: bool,
    ) -> RenderResources {
        let transform_id = self.engine.renderer
            .add_transform(Transform {
                position,
                ..Default::default()
            });
        let polygon: Vec<Vec2> = vec![
            Vec2::new(-volume[0]/2.0, -volume[1]/2.0),
            Vec2::new(-volume[0]/2.0, volume[1]/2.0),
            Vec2::new(volume[0]/2.0, volume[1]/2.0),
            Vec2::new(volume[0]/2.0, -volume[1]/2.0),
        ];

        self.engine.physics_world.add(
            if is_static { RigidBody::new_static(transform_id) } 
            else { RigidBody::new(transform_id, 1.0) }, 
            Collider::new(polygon),
        );

        let mesh = Mesh::make_cube(
            &self.engine.renderer.get_render_context().device, 
            material_id,
            pipeline_id,
            transform_id,
            volume, // volume
        );

        let mesh_id = self.engine.renderer.add_mesh(mesh);

        return RenderResources {
            pipeline: pipeline_id,
            material: material_id,
            transform: transform_id,
            mesh: mesh_id,
        };
    }

}
