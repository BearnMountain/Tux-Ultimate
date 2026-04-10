#ifndef CONFIG_H_
#define CONFIG_H_

#include "SDL3/SDL_gpu.h"
#include "src/util/defines.h"

typedef struct {
	// platform dependent constants
	SDL_GPUShaderFormat shader_format;	

	// window options
	u32 fullscreen;
	u32 height;
	u32 width;

	// graphics

	// audio options
	f32 volume;
	char* audio_shot; // all paths are defined in config for general sounds
} Config;

extern Config config;

void config_init(void);
void config_save(void);

#endif

