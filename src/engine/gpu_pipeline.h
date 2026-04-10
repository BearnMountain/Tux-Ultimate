#ifndef GPU_PIPELINE_H_
#define GPU_PIPELINE_H_

#include "SDL3/SDL_gpu.h"

#define SHADER_DIR "build/shaders/"

SDL_GPUGraphicsPipeline* gpu_pipeline_load(const char* vertex_path, const char* fragment_path);
void gpu_pipeline_unload(SDL_GPUGraphicsPipeline* pipeline);

#endif
