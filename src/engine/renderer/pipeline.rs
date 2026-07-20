use std::env::current_dir;
use std::fs;

use crate::engine::assets::{types::shader::Shader};

pub struct Builder<'a> {
    shader: Option<&'a Shader>,
    pixel_format: wgpu::TextureFormat,
    vertex_buffer_layout: Vec<Option<wgpu::VertexBufferLayout<'static>>>,
    bind_group_layouts: Vec<&'a wgpu::BindGroupLayout>,
    device: &'a wgpu::Device,
}

impl<'a> Builder<'a> {
    pub fn new(device: &'a wgpu::Device) -> Self {
        return Self {
            shader: None,
            pixel_format: wgpu::TextureFormat::Rgba8Unorm,
            vertex_buffer_layout: Vec::new(),
            bind_group_layouts: Vec::new(),
            device: device,
        };
    }

    pub fn reset(&mut self) {
        self.vertex_buffer_layout.clear();
        self.bind_group_layouts.clear();
        self.shader = None;
    }

    pub fn add_buffer_layout(&mut self, layout: Option<wgpu::VertexBufferLayout<'static>>) {
        self.vertex_buffer_layout.push(layout);
    }

    pub fn add_bind_group_layout(&mut self, layout: &'a wgpu::BindGroupLayout) {
        self.bind_group_layouts.push(layout);
    }

    pub fn load_shader(&mut self, shader: &'a Shader) {
        self.shader = Some(shader);
    }
    
    pub fn set_pixel_format(&mut self, pixel_format: wgpu::TextureFormat) {
        self.pixel_format = pixel_format;
    }

    pub fn build_pipeline(&mut self, label: &str) -> wgpu::RenderPipeline {
        // describes resources available to shaders
        let bind_group_layouts: Vec<Option<&wgpu::BindGroupLayout>> =
            self.bind_group_layouts.iter().map(|i| Some(*i)).collect();
        let pipeline_layout = self.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some(label),
                bind_group_layouts: &bind_group_layouts,
                immediate_size: 0,
            }
        );

        // describes frag shaders output location
        let render_targets = [Some(wgpu::ColorTargetState {
            format: self.pixel_format,
            blend: Some(
                wgpu::BlendState::ALPHA_BLENDING
            ),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        // creates gpu pipeline
        let render_pipeline = self.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(&pipeline_layout),

                vertex: wgpu::VertexState {
                    module: &self.shader.unwrap().shader_module,
                    entry_point: Some(&self.shader.unwrap().vertex_entry),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers: &self.vertex_buffer_layout,
                },

                // how triangles are assembled
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &self.shader.unwrap().shader_module,
                    entry_point: Some(&self.shader.unwrap().fragment_entry),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    targets: &render_targets,
                }),
                multiview_mask: None,
                cache: None,
            }
        );

        self.reset();

        return render_pipeline;
    }
}
