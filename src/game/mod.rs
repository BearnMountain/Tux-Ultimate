use std::{sync::Arc};
use winit::{window::{Window}};

pub mod client;
pub mod server;
pub mod ui;
pub mod input;

use crate::{engine::Engine};

// max time that a frame isnt updated

/// Game is the gui interface that encapsulates all
/// rendered items + user inputs
///
/// Notes:
/// - Client: game client is for rendering video game state and such
/// - Server: game server is for the server interface, doesnt need
///   heavy wgpu rendering, so under wgpu is live loaded from menu
pub struct Game {
    tick: u64,

    // game specific
    ui: ui::UI,

    // general stuff
    pub engine: Engine,
    pub input_handler: input::GameInput,
}

impl Game {
    pub fn init(
        window: Arc<Window>,
    ) -> Self {
        // creates everything needed to run a game
        // graphics and ui created
        let engine = Engine::new(window.clone());
        let input_handler = input::GameInput::new();

        return Self {
            ui: ui::UI::init(),
            tick: 0,
            engine,
            input_handler,
        };
    }

    /// called at monitor refresh rate(just for graphics)
    pub fn frame(&mut self) -> anyhow::Result<()> {
        self.engine.begin_ui();
        self.ui.frame(self.engine.ui_context());
        self.engine.end_ui();

        self.engine.renderer.render()?;

        return Ok(());
    }

    /// called every tick(game updates, server, etc)
    pub fn update(&mut self) {
        self.tick += 1;
    }

}

// pub struct GameClient {
//     tick: u64,
//     frames: u64,
//
//     pub engine: Engine,
//     pub input_handler: io::input::Input,
//
//     pub upload_list: RequireUpload,
// }
//
// impl GameClient {
//     pub fn init(window: Arc<Window>) -> Self {
//         let mut engine = Engine::new(window.clone());
//         let input_handler = Input::new();
//
//         // testing engine
//         let device = engine.renderer.get_render_context().device.clone();
//         let queue = engine.renderer.get_render_context().queue.clone();
//
//         // gather test data
//         let (shader_text, texture_raw, gltf_json, gltf_bin) = pollster::block_on(async {
//             tokio::try_join!(
//                 Server::preload_text(Path::new("shaders/shader.wgsl")),
//                 Server::preload_raw(Path::new("textures/brick-texture-54.png")),
//                 Server::preload_text(Path::new("characters/test/tux/scene.gltf")),
//                 Server::preload_raw(Path::new("characters/test/tux/scene.bin")),
//             )
//         }).expect("rip");
//
//         let shader_handle = engine
//             .asset_server
//             .load_shader(shader_text, None, None)
//             .expect("failed to load shader source");
//         let texture_handle = engine
//             .asset_server
//             .load_texture(texture_raw)
//             .expect("failed to load texture source");
//
//         let material_layout = LayoutBuilder::new(&device)
//             .add_texture_view(
//                 wgpu::ShaderStages::FRAGMENT, 
//                 wgpu::TextureSampleType::Float { filterable: true }, 
//                 wgpu::TextureViewDimension::D2
//             )
//             .add_texture_sampler(
//                 wgpu::ShaderStages::FRAGMENT, 
//                 wgpu::SamplerBindingType::Filtering
//             )
//             .build("material bind group");
//
//         let pipeline = {
//             let contex = engine.renderer.get_render_context();
//             pipeline::Builder::new(&contex.device)
//                 .set_shader(engine.asset_server.get_shader(shader_handle).unwrap())
//                 .set_pixel_format(contex.config.format)
//                 .add_buffer_layout(Some(mesh::Vertex::get_layout()))
//                 .add_bind_group_layout(&material_layout.layout.clone()) // idx: 0
//                 .add_bind_group_layout(&engine.renderer.get_transform_layout().layout.clone()) // idx: 1
//                 .add_bind_group_layout(&engine.renderer.camera.uploader.layout.layout.clone()) // idx: 2
//                 .set_depth(true, true)
//                 .set_blend(None)
//                 .set_depth_format(wgpu::TextureFormat::Depth32Float)
//                 .build_pipeline("pipeline test")
//         };
//
//         let material = Material::new(
//             "test material", 
//             engine.asset_server.get_texture(texture_handle).unwrap(), 
//             &device, 
//             &material_layout
//         );
//
//         // get stuff renderable each loop
//         let _material_id = engine.renderer.add_material(material);
//         let _pipeline_id = engine.renderer.add_pipeline(pipeline);
//
//         // load models
//         {
//             let _model = Model::create_from_gltf(
//                 &gltf_json,
//                 &device,
//                 &queue,
//             );
//         }
//
//         return Self {
//             tick: 0,
//             frames: 0,
//             engine,
//             input_handler,
//             upload_list: RequireUpload { 
//                 camera: false, 
//             },
//         };
//     }
//
//     pub fn setup_game(&mut self) {
//         let cube1 = self.create_cube(
//             Handle::new(0), 
//             Handle::new(0), 
//             Vec3::new(0.0, 10.0, -8.0), 
//             [1.0, 1.0, 1.0], 
//             false
//         );
//
//         let cube2 = self.create_cube(
//             Handle::new(0), 
//             Handle::new(0), 
//             Vec3::new(-5.0, -5.0, -13.0), 
//             [10.0, 1.0, 10.0], 
//             true
//         );
//
//         let renderer = &mut self.engine.renderer;
//         renderer.add_render_resource(cube1);
//         renderer.add_render_resource(cube2);
//
//         self.engine.renderer.update_transforms();
//     }
//
//     /// called at monitor refresh rate
//     pub fn frame(&mut self, _dt: Duration, _tick: u64) -> anyhow::Result<()> {
//         // upload data to shaders 
//         {
//             let upload_list = &mut self.upload_list;
//             if upload_list.camera {
//                 self.engine.renderer.camera.uploader.upload(&self.engine.renderer.camera.transform);
//                 upload_list.camera = false;
//             }
//         }
//
//         // update screen
//         self.engine.renderer.render()?;
//
//         // ---- RESET ----
//
//         return Ok(());
//     }
//
//     /// called every tick
//     pub fn update(&mut self, _dt: Duration) {
//         self.tick += 1;
//
//         self.update_from_input();
//         self.engine.physics_world.update(
//             1.0/60.0, 
//             self.engine.renderer.get_transform_cache()
//         );
//
//         // reseting inputs
//         self.input_handler.mouse_delta = Vec2::ZERO;
//         self.input_handler.mouse_scroll_delta = Vec2::ZERO;
//     }
//

