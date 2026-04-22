#ifndef RESOURCE_LOADER_H_
#define RESOURCE_LOADER_H_

#include "src/util/defines.h"
#include <SDL3/SDL.h>

#define SHADER_DIR "build/shaders/"

char* resource_load_file(const char* path, u32* size); 
SDL_GPUShader* resource_load_shader(SDL_GPUDevice* device, const char* path); // pass in shaders name: rect.frag

#endif
