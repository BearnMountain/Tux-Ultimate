#ifndef RENDER_H_
#define RENDER_H_

#include "SDL3/SDL_gpu.h"
#include "cglm/types.h"
#include "src/util/defines.h"
#include "lib/gltf/gltf.h"

// all data for gpu and rendering is stored here
typedef struct {
	SDL_Window* window;
	SDL_GPUDevice* device;
	SDL_GPUGraphicsPipeline* pipeline;
	SDL_GPUCommandBuffer* cmd;
	SDL_GPUTexture* swapchain_texture;
	u32 window_width, window_height;
} FrameData;

extern FrameData frame_data;

// rendering 
typedef struct {
	SDL_GPUBuffer* vertex_buffer;
	SDL_GPUBuffer* index_buffer;
	u32 index_count;
} GPUMesh;

typedef struct {
	vec3 pos;
	vec3 norm;
	vec2 uv;
} ShaderVertex;

void render_init(SDL_Window* window);
void render_uninit(void);

void render_frame(void);

// submitting vertices to render to the screen
GPUMesh render_upload_mesh(const cgltf_primitive* prim);

// void render_submit(
// SDL_GPUGraphicsPipeline* pipeline,
// SDL_GPUBuffer* vertex_buffer.
// SDL_GPUBuffer* indice_buffer,
// u32 index_count,
// u32 first_index,
// u32 vertex_offset
// )

#endif

// vulkan pipeline, shaders, draw calls
