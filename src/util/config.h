#ifndef CONFIG_H_
#define CONFIG_H_

#include "SDL3/SDL_gpu.h"

typedef struct {
	SDL_GPUShaderFormat shader_format;	
} Config;

extern Config config;

void config_init(void);

#endif
