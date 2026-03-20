#include "config.h"

Config config = {
	.shader_format = SDL_GPU_SHADERFORMAT_MSL // should be forcefully updated by SDL_GetGPUShaderFormat()
};

void config_init(void) {
	// platform dependencies
	// SDL_GPUShaderFormat format = SDL_GetGPUShaderFormat(frame_data.device);
#if defined(__APPLE__) 
	config.shader_format = SDL_GPU_SHADERFORMAT_MSL;
#elif defined(_WIN32)
	config.shader_format = SDL_GPU_SHADERFORMAT_DXIL;
#elif defined(__linux__)
	config.shader_format = SDL_GPU_SHADERFORMAT_SPIRV;
#endif
}
