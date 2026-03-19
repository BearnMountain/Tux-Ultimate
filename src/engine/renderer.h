#ifndef RENDERER_H_
#define RENDERER_H_

#include "SDL3/SDL_gpu.h"

typedef struct {
	SDL_GPUCommandBuffer* cmd;
	SDL_GPUTexture* swapchain_texture;
} FrameData;

void renderer_init(void);
void renderer_uninit(void);

void renderer_wait_for_frame(void);
void renderer_submit_frame(void);

#endif

// vulkan pipeline, shaders, draw calls
