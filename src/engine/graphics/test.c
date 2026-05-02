// // model.c
//
// #include "model.h"
// #include <cgltf.h>
// #include <string.h>
// #include <stdlib.h>
// #include <stdio.h>
//
// // reads the first image in the glTF and uploads it as an RGBA8 texture
// // returns NULL if there are no images (model will render without a texture)
// static SDL_GPUTexture* upload_texture(SDL_GPUDevice* device, cgltf_data* data)
// {
//     if (data->images_count == 0) return NULL;
//
//     cgltf_image*  img     = &data->images[0];
//     cgltf_buffer_view* bv = img->buffer_view;
//     if (!bv) return NULL;
//
//     // raw bytes of the embedded PNG/JPEG
//     const uint8_t* bytes = (const uint8_t*)bv->buffer->data + bv->offset;
//     uint32_t       nbytes = (uint32_t)bv->size;
//
//     // decode with SDL_image (already a dependency of SDL3 in most setups)
//     SDL_IOStream* io  = SDL_IOFromConstMem(bytes, nbytes);
//     SDL_Surface*  surf = IMG_Load_IO(io, true); // closes io
//     if (!surf) return NULL;
//
//     SDL_Surface* rgba = SDL_ConvertSurface(surf, SDL_PIXELFORMAT_RGBA32);
//     SDL_DestroySurface(surf);
//     if (!rgba) return NULL;
//
//     uint32_t w = (uint32_t)rgba->w;
//     uint32_t h = (uint32_t)rgba->h;
//     uint32_t tex_size = w * h * 4;
//
//     SDL_GPUTextureCreateInfo tex_info = {
//         .type                 = SDL_GPU_TEXTURETYPE_2D,
//         .format               = SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM,
//         .usage                = SDL_GPU_TEXTUREUSAGE_SAMPLER,
//         .width                = w,
//         .height               = h,
//         .layer_count_or_depth = 1,
//         .num_levels           = 1,
//     };
//     SDL_GPUTexture* tex = SDL_CreateGPUTexture(device, &tex_info);
//     if (!tex) { SDL_DestroySurface(rgba); return NULL; }
//
//     SDL_GPUTransferBufferCreateInfo tb_info = {
//         .usage = SDL_GPU_TRANSFERBUFFERUSAGE_UPLOAD,
//         .size  = tex_size,
//     };
//     SDL_GPUTransferBuffer* tb = SDL_CreateGPUTransferBuffer(device, &tb_info);
//     void* mapped = SDL_MapGPUTransferBuffer(device, tb, false);
//     memcpy(mapped, rgba->pixels, tex_size);
//     SDL_UnmapGPUTransferBuffer(device, tb);
//     SDL_DestroySurface(rgba);
//
//     SDL_GPUCommandBuffer* cmd  = SDL_AcquireGPUCommandBuffer(device);
//     SDL_GPUCopyPass*      copy = SDL_BeginGPUCopyPass(cmd);
//
//     SDL_GPUTextureTransferInfo src = {
//         .transfer_buffer = tb,
//         .offset          = 0,
//         .pixels_per_row  = w,
//         .rows_per_layer  = h,
//     };
//     SDL_GPUTextureRegion dst = {
//         .texture   = tex,
//         .mip_level = 0,
//         .layer     = 0,
//         .x = 0, .y = 0, .z = 0,
//         .w = w, .h = h, .d = 1,
//     };
//     SDL_UploadToGPUTexture(copy, &src, &dst, false);
//
//     SDL_EndGPUCopyPass(copy);
//     SDL_SubmitGPUCommandBuffer(cmd);
//     SDL_ReleaseGPUTransferBuffer(device, tb);
//
//     return tex;
// }
//
// // ---------------------------------------------------------------------------
// // model_create
// // ---------------------------------------------------------------------------
//
// Model* model_create(const char* path, const char* vertex_shader, const char* frag_shader)
// {
//     if (!path) return NULL;
//     if (strcmp(path + strlen(path) - 4, ".glb") != 0) {
//         log_warn("model creation requires '.glb' file: %s invalid", path);
//         return NULL;
//     }
//
//     Model* model = calloc(1, sizeof(Model));
//
//     model->pipeline = gpu_pipeline_load(
//         vertex_shader,
//         frag_shader,
//         SDL_GPU_TEXTUREFORMAT_INVALID,
//         SDL_GPU_TEXTUREFORMAT_INVALID
//     );
//     if (!model->pipeline) {
//         log_err("failed to create model pipeline");
//         free(model);
//         return NULL;
//     }
//
//     // ---- parse .glb --------------------------------------------------------
//     cgltf_options options = {0};
//     cgltf_data*   data    = NULL;
//
//     if (cgltf_parse_file(&options, path, &data) != cgltf_result_success) {
//         log_err("failed to parse .glb file: %s", path);
//         gpu_pipeline_unload(model->pipeline);
//         free(model);
//         return NULL;
//     }
//     if (cgltf_load_buffers(&options, data, path) != cgltf_result_success) {
//         log_err("failed to load .glb buffers: %s", path);
//         cgltf_free(data);
//         gpu_pipeline_unload(model->pipeline);
//         free(model);
//         return NULL;
//     }
//     cgltf_validate(data);
//
//     // ---- extract geometry from first primitive of first mesh ---------------
//     if (data->meshes_count == 0) {
//         log_err("no meshes in %s", path);
//         cgltf_free(data);
//         gpu_pipeline_unload(model->pipeline);
//         free(model);
//         return NULL;
//     }
//
//     cgltf_primitive* prim = &data->meshes[0].primitives[0];
//
//     // find accessors for POSITION, NORMAL, TEXCOORD_0
//     cgltf_accessor* acc_pos   = NULL;
//     cgltf_accessor* acc_norm  = NULL;
//     cgltf_accessor* acc_uv    = NULL;
//
//     for (cgltf_size i = 0; i < prim->attributes_count; i++) {
//         cgltf_attribute* attr = &prim->attributes[i];
//         if (attr->type == cgltf_attribute_type_position)  acc_pos  = attr->data;
//         if (attr->type == cgltf_attribute_type_normal)    acc_norm = attr->data;
//         if (attr->type == cgltf_attribute_type_texcoord && attr->index == 0) acc_uv = attr->data;
//     }
//
//     if (!acc_pos) {
//         log_err("mesh has no POSITION attribute: %s", path);
//         cgltf_free(data);
//         gpu_pipeline_unload(model->pipeline);
//         free(model);
//         return NULL;
//     }
//
//     uint32_t vert_count = (uint32_t)acc_pos->count;
//     Vertex*  verts      = calloc(vert_count, sizeof(Vertex));
//
//     for (uint32_t i = 0; i < vert_count; i++) {
//         cgltf_accessor_read_float(acc_pos,  i, verts[i].position, 3);
//         if (acc_norm) cgltf_accessor_read_float(acc_norm, i, verts[i].normal,   3);
//         if (acc_uv)   cgltf_accessor_read_float(acc_uv,   i, verts[i].uv,       2);
//     }
//
//     // indices
//     cgltf_accessor* acc_idx = prim->indices;
//     uint32_t  idx_count = acc_idx ? (uint32_t)acc_idx->count : 0;
//     uint32_t* indices   = NULL;
//
//     if (acc_idx) {
//         indices = malloc(idx_count * sizeof(uint32_t));
//         for (uint32_t i = 0; i < idx_count; i++)
//             indices[i] = (uint32_t)cgltf_accessor_read_index(acc_idx, i);
//     }
//
//     model->index_count = idx_count;
//
//     // ---- upload to GPU -----------------------------------------------------
//     SDL_GPUDevice* device = gpu_get_device(); // your app's accessor
//
//     model->vertex_buffer = upload_buffer(
//         device,
//         SDL_GPU_BUFFERUSAGE_VERTEX,
//         verts,
//         vert_count * sizeof(Vertex)
//     );
//
//     if (indices) {
//         model->index_buffer = upload_buffer(
//             device,
//             SDL_GPU_BUFFERUSAGE_INDEX,
//             indices,
//             idx_count * sizeof(uint32_t)
//         );
//     }
//
//     free(verts);
//     free(indices);
//
//     // ---- texture + sampler -------------------------------------------------
//     model->albedo_texture = upload_texture(device, data);
//
//     SDL_GPUSamplerCreateInfo samp_info = {
//         .min_filter        = SDL_GPU_FILTER_LINEAR,
//         .mag_filter        = SDL_GPU_FILTER_LINEAR,
//         .mipmap_mode       = SDL_GPU_SAMPLERMIPMAPMODE_LINEAR,
//         .address_mode_u    = SDL_GPU_SAMPLERADDRESSMODE_REPEAT,
//         .address_mode_v    = SDL_GPU_SAMPLERADDRESSMODE_REPEAT,
//         .address_mode_w    = SDL_GPU_SAMPLERADDRESSMODE_REPEAT,
//     };
//     model->sampler = SDL_CreateGPUSampler(device, &samp_info);
//
//     cgltf_free(data);
//     return model;
// }
//
// // ---------------------------------------------------------------------------
// // model_draw
// // ---------------------------------------------------------------------------
//
// void model_draw(Model* model, SDL_GPURenderPass* pass, SDL_GPUDevice* device, ModelUniforms* uniforms)
// {
//     if (!model || !pass) return;
//
//     SDL_BindGPUGraphicsPipeline(pass, model->pipeline);
//
//     // bind vertex buffer at slot 0
//     SDL_GPUBufferBinding vb = { .buffer = model->vertex_buffer, .offset = 0 };
//     SDL_BindGPUVertexBuffers(pass, 0, &vb, 1);
//
//     // bind index buffer (32-bit indices)
//     SDL_GPUBufferBinding ib = { .buffer = model->index_buffer, .offset = 0 };
//     SDL_BindGPUIndexBuffer(pass, &ib, SDL_GPU_INDEXELEMENTSIZE_32BIT);
//
//     // bind albedo texture + sampler to fragment slot 0
//     if (model->albedo_texture) {
//         SDL_GPUTextureSamplerBinding tsb = {
//             .texture = model->albedo_texture,
//             .sampler = model->sampler,
//         };
//         SDL_BindGPUFragmentSamplers(pass, 0, &tsb, 1);
//     }
//
//     // push model matrix as vertex uniform (slot 0)
//     SDL_PushGPUVertexUniformData(pass, 0, uniforms, sizeof(ModelUniforms));
//
//     SDL_DrawGPUIndexedPrimitives(pass, model->index_count, 1, 0, 0, 0);
// }
//
// // ---------------------------------------------------------------------------
// // model_destroy
// // ---------------------------------------------------------------------------
//
// void model_destroy(Model* model, SDL_GPUDevice* device)
// {
//     if (!model) return;
//     if (model->vertex_buffer)  SDL_ReleaseGPUBuffer(device, model->vertex_buffer);
//     if (model->index_buffer)   SDL_ReleaseGPUBuffer(device, model->index_buffer);
//     if (model->albedo_texture) SDL_ReleaseGPUTexture(device, model->albedo_texture);
//     if (model->sampler)        SDL_ReleaseGPUSampler(device, model->sampler);
//     gpu_pipeline_unload(model->pipeline);
//     free(model);
// }
