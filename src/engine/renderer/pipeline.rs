use crate::engine::assets::{types::shader::Shader};

pub type PipelineID = usize;

pub struct Builder<'a> {
    shader: Option<&'a Shader>,

    // pipeline options
    pixel_format: wgpu::TextureFormat,
    vertex_buffer_layout: Vec<Option<wgpu::VertexBufferLayout<'static>>>,
    bind_group_layouts: Vec<Option<&'a wgpu::BindGroupLayout>>,
    depth_enabled: bool,
    depth_write: bool,
    blend_state: Option<wgpu::BlendState>,
    depth_format: wgpu::TextureFormat,

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
            depth_enabled: true,
            depth_write: true,
            blend_state: None,
            depth_format: wgpu::TextureFormat::Depth32Float,
        };
    }

    pub fn get_layout_size(&self) -> usize {
        return self.bind_group_layouts.len();
    }

    pub fn reset(&mut self) {
        self.vertex_buffer_layout.clear();
        self.bind_group_layouts.clear();
        self.shader = None;
    }

    pub fn add_buffer_layout(&mut self, layout: Option<wgpu::VertexBufferLayout<'static>>) -> &mut Self {
        self.vertex_buffer_layout.push(layout);
        return self;
    }

    pub fn add_bind_group_layout(&mut self, layout: &'a wgpu::BindGroupLayout) -> &mut Self {
        self.bind_group_layouts.push(Some(layout));
        return self;
    }

    pub fn set_shader(&mut self, shader: &'a Shader) -> &mut Self {
        self.shader = Some(shader);
        return self;
    }
    
    pub fn set_pixel_format(&mut self, pixel_format: wgpu::TextureFormat) -> &mut Self {
        self.pixel_format = pixel_format;
        return self;
    }

    pub fn set_depth(&mut self, enabled: bool, write: bool) -> &mut Self {
        self.depth_enabled = enabled;
        self.depth_write = write;
        return self;
    }

    pub fn set_blend(&mut self, blend: Option<wgpu::BlendState>) -> &mut Self {
        self.blend_state = blend;
        return self;
    }

    pub fn set_depth_format(&mut self, format: wgpu::TextureFormat) -> &mut Self {
        self.depth_format = format;
        return self;
    }

    pub fn build_pipeline(&mut self, label: &str) -> wgpu::RenderPipeline {
        // describes resources available to shaders
        let pipeline_layout = self.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some(label),
                bind_group_layouts: &self.bind_group_layouts,
                immediate_size: 4,
            }
        );

        // draw order
        let depth_stencil = if self.depth_enabled {
            Some(wgpu::DepthStencilState {
                format: self.depth_format,
                depth_write_enabled: Some(self.depth_write),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            })
        } else {
            None
        };

        // describes frag shaders output location
        let render_targets = [Some(wgpu::ColorTargetState {
            format: self.pixel_format,
            blend: self.blend_state,
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
                depth_stencil,
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

// pub struct PipelineStorage {
//     pipelines: Vec<wgpu::RenderPipeline>,
// }
//
// impl PipelineStorage {
//     pub fn new() -> Self {
//         return Self {
//             pipelines: Vec::new(),
//         };
//     }
//
//     pub fn get(&self, id: PipelineID) -> Option<&wgpu::RenderPipeline> {
//         return self.pipelines.get(id);
//     }
//
//     pub fn add(&mut self, pipeline: wgpu::RenderPipeline) -> PipelineID {
//         self.pipelines.push(pipeline);
//         return self.pipelines.len() - 1;
//     }
// }
