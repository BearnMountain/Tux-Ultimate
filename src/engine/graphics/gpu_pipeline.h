#ifndef GPU_PIPELINE_H_
#define GPU_PIPELINE_H_

#include "SDL3/SDL_gpu.h"

/*
   Please don't append glsl to the path, just name
   */
SDL_GPUGraphicsPipeline* gpu_pipeline_load(SDL_GPUDevice* device, const char* vertex_path, const char* fragment_path, SDL_GPUTextureFormat color_format, SDL_GPUTextureFormat depth_format);
void gpu_pipeline_unload(SDL_GPUDevice* device, SDL_GPUGraphicsPipeline* pipeline);

#endif
