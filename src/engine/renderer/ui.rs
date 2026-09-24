use egui::Context;
use egui_wgpu::{Renderer, RendererOptions, ScreenDescriptor};
use egui_winit::State;
use wgpu::{CommandEncoder, TextureView};
use winit::{event::WindowEvent, window::Window};



pub struct Egui {
    // fonts, styles, ui
    pub context: egui::Context, 
    // converts winit events to egui inputs
    state: egui_winit::State,
    // converts egui render commands into wgpu 
    renderer: egui_wgpu::Renderer,

    // queued draws
    texture: Vec<egui::ClippedPrimitive>,
    output: Option<egui::FullOutput>,
}

impl Egui {
    pub fn new (
        device: &wgpu::Device,
        window: &Window,
        surface_format: wgpu::TextureFormat,
    ) -> Self {
        let context = egui::Context::default();

        let state = State::new(
            context.clone(),
            egui::ViewportId::ROOT,
            window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );

        let renderer = Renderer::new(
            device, 
            surface_format, 
            RendererOptions::default(),
        );

        return Self {
            context,
            state,
            renderer,
            texture: Vec::new(),
            output: None,
        };
    }

    pub fn handle_event(
        &mut self,
        window: &Window,
        event: &WindowEvent,
    ) -> bool {
        return self.state
            .on_window_event(window, event)
            .consumed;
    }

    /// ----- Drawing Frame -----
    /// begin_frame
    /// ui
    /// end frame
    /// ** everything rendered inside renderer
    
    pub fn begin_frame(&mut self, window: &Window) {
        let input = self.state.take_egui_input(window);

        self.context.begin_pass(input);
    }

    pub fn ui<F>(&mut self, f: F) 
    where
        F: FnOnce(&Context),
    {
        f(&self.context);
    }

    pub fn end_frame(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        window: &Window,
    ) {
        let output = self.context.end_pass();

        self.state.handle_platform_output(
            window, 
            output.platform_output.clone()
        );

        let ppp = self.context.pixels_per_point();
        let texture = self.context.tessellate(
            output.shapes.clone(),
            ppp,
        );

        // uploading to gpu
        for (id, delta) in &output.textures_delta.set {
            self.renderer.update_texture(
                device,
                queue,
                *id,
                &delta[0],
            );
        }

        self.texture = texture;
        self.output = Some(output);
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut CommandEncoder,
        target: &TextureView,
        screen_size: [u32; 2],
    ) {
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: screen_size,
            pixels_per_point: self.context.pixels_per_point(),
        };

        let mut pass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("egui Render Pass"),
                color_attachments: &[Some(
                    wgpu::RenderPassColorAttachment {
                        view: target,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    },
                )],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            },
        );

        // egui-wgpu requires a 'static render pass.
        let pass = pass.forget_lifetime();

        self.renderer.update_buffers(
            device,
            queue,
            pass,
            &self.texture,
            &screen_descriptor,
        );

        // `update_buffers` may need to operate on the render pass,
        // so begin another pass for the actual draw is not appropriate.
        //
        // See the implementation below instead.
    }

    // ----- Configure -----
    pub fn context(&self) -> &Context {
        return &self.context;
    }

    pub fn context_mut(&mut self) -> &mut Context {
        return &mut self.context;
    }

    pub fn set_fonts(&self, fonts: egui::FontDefinitions) {
        self.context.set_fonts(fonts);
    }

    pub fn set_style(&self, style: egui::Style) {
        self.context.set_style_of(egui::Theme::Dark, style);
    }
}











