#![allow(dead_code)]
#![allow(unused)]
pub mod pipeline;
pub mod mesh;
pub mod context;
pub mod bind_group;
pub mod material;
// pub mod model_loader;

use winit::dpi::PhysicalSize;


pub struct Renderer {
    graphics: context::RenderContext,

    pipeline_cache: pipeline::PipelineStorage,
    materials_cache: material::MaterialStorage,

    // renderables
    meshes_cache: mesh::MeshStorage,
}

impl Renderer {
    pub fn new(graphics: context::RenderContext) -> Self {
        return Self {
            graphics: graphics,
            pipeline_cache: pipeline::PipelineStorage::new(),
            materials_cache: material::MaterialStorage::new(),
            meshes_cache: mesh::MeshStorage::new(),
        };
    }

    fn render_pass(&mut self, pass: &mut wgpu::RenderPass) {
        { // self.meshes_cache run
            // sorting meshes pipeline -> material
            let meshes = self.meshes_cache.get_all_sorted(); 
            if meshes.len() == 0 {
                return;
            }

            let mut pipeline_id: pipeline::PipelineID = meshes[0].pipeline_id;
            let mut material_id: material::MaterialID = meshes[0].material_id;
            let mut material_bind_index = 0;
            pass.set_pipeline(&self.pipeline_cache.get(pipeline_id).unwrap());
            pass.set_bind_group(material_bind_index, &self.materials_cache.get(material_id).unwrap().bind_group, &[]);
            pass.set_vertex_buffer(0, meshes[0].vertex_buffer.slice(..));
            pass.set_index_buffer(meshes[0].index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            pass.draw_indexed(0..6, 0, 0..1); // TODO: need to not hardcode this

            for mesh in meshes {
                // updates pipeline/material for new sorted batch
                if mesh.pipeline_id != pipeline_id {
                    pipeline_id = mesh.pipeline_id;
                    pass.set_pipeline(&self.pipeline_cache.get(pipeline_id).unwrap());
                }
                if mesh.material_id != material_id {
                    material_bind_index += 1;
                    material_id = mesh.material_id;
                    pass.set_bind_group(material_bind_index, &self.materials_cache.get(material_id).unwrap().bind_group, &[]);
                }

                pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                pass.draw_indexed(0..6, 0, 0..1); // TODO: need to not hardcode this


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
                    g: 0.7,
                    b: 0.3,
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
                    depth_stencil_attachment: None,
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

    pub fn resize(&mut self, physical_size: Option<PhysicalSize<u32>>) {
        if let Some(size) = physical_size {
            self.graphics.resize(size.width, size.height);
        } else {
            self.graphics.resize(self.graphics.size.width, self.graphics.size.height);
        }
    }

    pub fn update_surface(&mut self) {
        self.graphics.update_surface();
    }

    // adding items to cache
    pub fn add_material(&mut self, material: material::Material) -> material::MaterialID {
        return self.materials_cache.add(material);
    }
    pub fn add_pipeline(&mut self, pipeline: wgpu::RenderPipeline) -> pipeline::PipelineID {
        return self.pipeline_cache.add(pipeline);
    }
    pub fn add_mesh(&mut self, mesh: mesh::Mesh) -> mesh::MeshID {
        return self.meshes_cache.add(mesh);
    }

    // get items from internal struct
    pub fn get_render_context(&self) -> &context::RenderContext {
        return &self.graphics;
    }
}
