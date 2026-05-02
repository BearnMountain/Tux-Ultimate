#ifndef CONFIG_H_
#define CONFIG_H_

#include <SDL3/SDL_gpu.h>
#include <SDL3/SDL.h>
#include "src/util/defines.h"

typedef struct {
	// platform dependent constants
	SDL_GPUShaderFormat shader_format;	

	// player info
	char* player_name;

	// window options
	u32 fullscreen;
	u32 height;
	u32 width;

	// graphics

	// character loading

	// audio options
	f32 volume;
	char* audio_shot; // all paths are defined in config for general sounds

	// global information
	u32 game_tick;

	// keybinds 
	SDL_Keycode move_forward;
} Config;

extern Config config;

void config_init(void);
void config_save(void);

#endif

