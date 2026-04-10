#ifndef RENDER_H_
#define RENDER_H_

#include "SDL3/SDL_gpu.h"
#include "src/util/defines.h"

typedef struct {
	SDL_Window* window;
	SDL_GPUDevice* device;
	SDL_GPUGraphicsPipeline* pipeline;
	SDL_GPUCommandBuffer* cmd;
	SDL_GPUTexture* swapchain_texture;
	u32 window_width, window_height;
} FrameData;

typedef struct {
	f32 x, y, z;
	f32 r, g, b, a;
} Vertex;

extern FrameData frame_data;

void renderer_init(SDL_Window* window);
void renderer_uninit(void);

void renderer_frame(void);

// submitting vertices to be rendered to the screen
void renderer_submit_triangle(Vertex v[3], SDL_FColor color);

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
