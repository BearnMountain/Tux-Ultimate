#ifndef RENDER_H_
#define RENDER_H_

#include "SDL3/SDL_gpu.h"
#include "cglm/types.h"
#include "src/util/defines.h"

// all data for gpu and rendering is stored here
// global for use of all files inside graphics/*
typedef struct {
	SDL_Window* window;
	SDL_GPUDevice* device;
	SDL_GPUCommandBuffer* cmd;
	SDL_GPUTexture* swapchain_texture;
	u32 window_width, window_height;
} FrameData;

extern FrameData frame_data;

typedef struct {
	vec3 position;
	vec3 normal;
	vec2 uv;
} Vertex;

typedef struct {
    SDL_GPUBuffer* vertex_buffer;
    SDL_GPUBuffer* index_buffer;
    u32 index_count;
    u32 index_offset;

    // add when you have textures:
    SDL_GPUTexture* albedo;
    SDL_GPUSampler* sampler;
} Mesh;

typedef struct {
	SDL_GPUGraphicsPipeline* pipeline;
	Mesh* meshes;
	u32 mesh_count;
	mat4 model;
} RenderInstance;

void render_init(SDL_Window* window);
void render_uninit(void);

void render_frame(void);

// not copied
RenderInstance render_create_render_instance();
void render_submit_render_instance(Mesh* mesh, u32 mesh_count);

// uploads arbitrary data to gpu and returns buffer to use for binding for draw calls
SDL_GPUBuffer* render_upload_buffer(const void* data, u32 size, SDL_GPUBufferUsageFlags flags);
SDL_GPUTexture* render_upload_texture(const u8* data, u32 texture_width, u32 texture_height);
SDL_GPUSampler* render_upload_sampler(void);

// submitting vertices to render to the screen
// GPUMesh render_upload_mesh(const cgltf_primitive* prim);

// void render_submit(
// SDL_GPUGraphicsPipeline* pipeline,
// SDL_GPUBuffer* vertex_buffer.
// SDL_GPUBuffer* indice_buffer,
// u32 index_count,
// u32 first_index,
// u32 vertex_offset
// )

#endif
