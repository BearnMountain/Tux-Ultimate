#![allow(dead_code)]
#![allow(unused)]
pub mod pipeline;
pub mod mesh;
pub mod context;
pub mod bind_group;
pub mod material;
pub mod coordinate;
pub mod math;
pub mod transform;
pub mod model_loader;
pub mod render_resource;

use glam::Vec3;
use winit::dpi::PhysicalSize;

use crate::{engine::{renderer::{material::Material, mesh::Mesh, render_resource::{RenderResources, RenderStorage}, transform::{Transform, TransformStorage}}, scene::camera}, util::handle::Handle};

pub struct Renderer {
    graphics: context::RenderContext,
    pub camera: camera::Camera,

    renderables: Vec<RenderResources>,

    pipeline_cache: RenderStorage<wgpu::RenderPipeline>,
    materials_cache: RenderStorage<Material>,
    transform_cache: TransformStorage,
    meshes_cache: RenderStorage<Mesh>,
}

impl Renderer {
    pub fn new(graphics: context::RenderContext, width: f32, height: f32) -> Self {
        let transform_cache = transform::TransformStorage::new(
            &graphics.device,
            &graphics.queue,
        );
        let camera = camera::Camera::new(
            &graphics.device, 
            &graphics.queue, 
            Vec3::new(0.0, 0.0, 0.0), 
            width / height,
        );

        return Self {
            graphics,
            camera,
            renderables: Vec::new(),
            pipeline_cache: RenderStorage::new(),
            materials_cache: RenderStorage::new(),
            transform_cache,
            meshes_cache: RenderStorage::new(),
        };
    }

    fn render_pass(&mut self, pass: &mut wgpu::RenderPass) {
        { // self.meshes_cache run
            // sorting meshes pipeline -> material
            let meshes = self.meshes_cache.get_all_sorted(); 

            if meshes.len() == 0 {
                return;
            }

            pass.set_bind_group(1, &self.transform_cache.bind_group, &[]);
            pass.set_bind_group(2, &self.camera.uploader.bind_group, &[]);

            // binds new pipeline/material(bind groups) whenever needed
            let mut current_pipeline: Option<pipeline::PipelineID> = None;
            let mut current_material: Option<pipeline::PipelineID> = None;

            for mesh in meshes {
                if current_pipeline != Some(mesh.pipeline_id) {
                    current_pipeline = Some(mesh.pipeline_id);
                    pass.set_pipeline(
                        &self.pipeline_cache.get(mesh.pipeline_id).unwrap()
                    );
                }
            
                if current_material != Some(mesh.material_id) {
                    current_material = Some(mesh.material_id);
                    pass.set_bind_group(
                        0,
                        &self.materials_cache.get(mesh.material_id).unwrap().bind_group,
                        &[]
                    );
                }
            
                // renders basic mesh to screen
                pass.set_immediates(0, &(mesh.transform_id as u32).to_ne_bytes());
                pass.set_vertex_buffer(0, mesh.buffer.slice(0..mesh.offset));
                pass.set_index_buffer(mesh.buffer.slice(mesh.offset..), wgpu::IndexFormat::Uint16);
                pass.draw_indexed(0..mesh.index_count, 0, 0..1);
            }
        }
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        self.graphics.window.request_redraw();

        // surface texture can be used with imgui::image to render image under ui
        let surface_texture = match self.graphics.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Timeout |
            wgpu::CurrentSurfaceTexture::Occluded => return Ok(()),
            wgpu::CurrentSurfaceTexture::Outdated |
            wgpu::CurrentSurfaceTexture::Lost => {
                self.graphics.resize(self.graphics.size.width, self.graphics.size.height);
                return Ok(());
            },
            wgpu::CurrentSurfaceTexture::Validation => {
                panic!("Surface validation failed");
            },
        };

        let image_view = surface_texture.texture.create_view(
            &wgpu::TextureViewDescriptor::default(),
        );

        // queue for all draw calls
        let mut command_encoder = self.graphics.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Renderer Encoder"),
            }
        );

        // screen's clear color/reset
        let screen_reset = wgpu::RenderPassColorAttachment {
            view: &image_view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.5,
                }),
                store: wgpu::StoreOp::Store,
            },
        };

        // submit render pass commands
        {
            let mut pass = command_encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("Renderpass"),
                    color_attachments: &[Some(screen_reset)],
                    depth_stencil_attachment: Some(
                        wgpu::RenderPassDepthStencilAttachment {
                            view: ,
                            depth_ops: todo!(),
                            stencil_ops: todo!(),
                        }
                    ),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                }
            );

            // pass through each pipeline and render
            self.render_pass(&mut pass);
        }

        self.graphics.queue.submit(std::iter::once(command_encoder.finish()));
        self.graphics.queue.present(surface_texture);

        return Ok(());
    }

    pub fn resize(&mut self, physical_size: PhysicalSize<u32>) {
        self.graphics.resize(physical_size.width, physical_size.height);
    }

    // updaters to renderer
    pub fn update_surface(&mut self) {
        self.graphics.update_surface();
    }

    pub fn update_transforms(&mut self) {
        self.transform_cache.upload();
    }

    /// adding items to cache
    /// all take ownership
    pub fn add_material(
        &mut self, 
        material: material::Material
    ) -> Handle<Material> {
        return self.materials_cache.add(material);
    }
    pub fn add_pipeline(
        &mut self, 
        pipeline: wgpu::RenderPipeline
    ) -> Handle<wgpu::RenderPipeline> {
        return self.pipeline_cache.add(pipeline);
    }
    pub fn add_transform(
        &mut self, 
        transform: transform::Transform
    ) -> Handle<Transform> {
        return self.transform_cache.add(transform);
    }
    pub fn add_mesh(
        &mut self, 
        mesh: mesh::Mesh
    ) -> Handle<Mesh> {
        return self.meshes_cache.add(mesh);
    }

    // get items from internal struct
    pub fn get_render_context(&self) -> &context::RenderContext {
        return &self.graphics;
    }
    pub fn get_transform_layout(&self) -> &bind_group::LayoutInfo {
        return &self.transform_cache.layout;
    }
    pub fn get_transform(
        &mut self, 
        i: Handle<Transform>,
    ) -> Option<&mut transform::Transform> {
        return self.transform_cache.get(i);
    }

    pub fn get_transform_cache(&mut self) -> &mut TransformStorage {
        return &mut self.transform_cache;
    }
}
