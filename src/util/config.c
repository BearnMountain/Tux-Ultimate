#include "config.h"
#include "src/util/logger.h"
#include <stdlib.h>
#include <stdio.h>

#define CONFIG_FILE_PATH "res/config.toml"

Config config = {
	.shader_format = SDL_GPU_SHADERFORMAT_MSL, // should be forcefully updated by SDL_GetGPUShaderFormat()
	.fullscreen = false,
	.height = 800,
	.width = 600,
	.volume = 50,
	.audio_shot = "res/audio/shot.mp4",
	.game_tick = 0
};

// simplifying settings 
typedef enum {
	CFG_INT,
	CFG_FLOAT,
	CFG_BOOL,
	CFG_STRING
} ConfigType;

typedef struct {
	const char* key;
	ConfigType type;
	void* dst;
} ConfigKey;

// all settings go here
ConfigKey cfg_keys[] = {
    { "fullscreen",     CFG_BOOL,  &((Config*)0)->fullscreen },
    { "width",          CFG_INT,   &((Config*)0)->width      },
    { "height",         CFG_INT,   &((Config*)0)->height     },
    { "volume",         CFG_FLOAT, &((Config*)0)->volume     },
    { "audio_shot",     CFG_FLOAT, &((Config*)0)->audio_shot },
};

#define CFG_KEY_COUNT (sizeof(cfg_keys)/sizeof(cfg_keys[0]))

// helper functions
static char* trim(char* s) { // null terminated strings
	while (*s == ' ') s++; // trims start
	if (*s == '\0') return s; // cuts out lines that end
	
	// trims end of line
	char* end = s + strlen(s) - 1;
	while (end > s && *end == ' ') end--;
	end[1] = '\0';
	return s;
}

void config_init(void) {
	Config tmp_config;

	// platform dependencies
#if defined(__APPLE__) 
	tmp_config.shader_format = SDL_GPU_SHADERFORMAT_MSL;
#elif defined(_WIN32)
	tmp_config.shader_format = SDL_GPU_SHADERFORMAT_DXIL;
#elif defined(__linux__)
	tmp_config.shader_format = SDL_GPU_SHADERFORMAT_SPIRV;
#endif

	log_warn("not implemented");
	// FILE* file = fopen(CONFIG_FILE_PATH, "r");
	// if (!file) {
	// 	log_err("failed to load custom config: %s", CONFIG_FILE_PATH);
	// 	return;
	// }
	//
	// fseek(file, 0, SEEK_END);
	// size_t size = ftell(file);
	// rewind(file);
	//
	// char* buffer = (char*)malloc(size);
	// fread(buffer, sizeof(char), size, file);
	//
	// size_t cfg_index = 0;
	// size_t line_start = 0;
	// for (size_t i = line_start; i < size; i++) {
	// 	if (buffer[i] == '\n') {
	// 		char str[i - line_start];
	// 		memcpy(str, buffer + line_start, i - line_start);
	// 		str[i-line_start] = '\0';
	//
	// 		// cleaning up buffered line
	// 		char* comment = strstr(str, "#");
	// 		if (comment) *comment = '\0';
	// 		trim(str);
	//
	// 		// removes useless lines
	// 		if (i - line_start == 0 || str[0] == '[') {
	// 			line_start = i + 1;
	// 			continue;
	// 		}
	//
	// 		// seperates both the key and value to be compared and stored in tmp config
	// 		char* delimiter = strstr(str, "=");
	// 		if (delimiter) *delimiter = '\0';
	// 		trim(str);
	// 		trim(delimiter + 1);
	//
	// 		if (!strcmp(cfg_keys[cfg_index].key, str)) {
	// 			log_info("%s", delimiter + 1);
	// 			cfg_index++;
	// 		} else {
	// 			log_warn("config incorrect order, please visit \"https://github.com/BearnMountain/Tux-Ultimate\" for correct format.");
	// 			log_warn("failed at: %s", delimiter + 1);
	// 			return;
	// 		}
	//
	// 		line_start = i + 1;
	// 	}
	// }

	memcpy(&config, &tmp_config, sizeof(Config));
}

void config_save(void) {

}
