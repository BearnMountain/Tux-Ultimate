#include "resource_loader.h"
#include "src/util/config.h"
#include "src/util/logger.h"
#include <stdio.h>
#include <stdlib.h>

char* resource_load_file(const char* path, u32* size) {
	char* buffer;

	FILE* file = fopen(path, "r");
	if (!file) {
		log_err("failed to open file: %s\n", path);
		return NULL;
	}

	fseek(file, 0, SEEK_END);
	u32 tmp_size = ftell(file);
	rewind(file);

	buffer = (char*)malloc(tmp_size + 1);

	fread(buffer, sizeof(char), tmp_size, file);
	buffer[tmp_size] = '\0';
	if (size) *size = tmp_size;

	fclose(file);

	return buffer;
}

SDL_GPUShader* resource_load_shader(SDL_GPUDevice* device, const char* path) {
	if (strcmp(path + strlen(path) - 5, ".glsl") == 0) { // for easier parsing
		log_warn("please dont give full shader path, ignore glsl extension");
		return NULL;
	}

    // get shader stage
    SDL_GPUShaderStage stage;
    if (strstr(path, "vert") != NULL) stage = SDL_GPU_SHADERSTAGE_VERTEX;
    else if (strstr(path, "frag") != NULL) stage = SDL_GPU_SHADERSTAGE_FRAGMENT;
    else {
        log_err("Could not determine shader type for: %s", path);
		return NULL;
    }

	// get shader format that is compiled on different platforms
	SDL_GPUShaderFormat format = config.shader_format;
	char* format_name;
	char* format_entrypoint;
	if (format & SDL_GPU_SHADERFORMAT_SPIRV) {
		format_name = "spv";
		format_entrypoint = "main";
	} else if (format & SDL_GPU_SHADERFORMAT_DXIL) { 
		format_name = "dxil";
		format_entrypoint = "main";
	} else if (format & SDL_GPU_SHADERFORMAT_MSL) {
		format_name = "msl";
		format_entrypoint = "main0";
	}  else {
		log_err("doesnt support any shader formats supplied");
		return NULL;
	}

	// loading file from compiled, platform specific, shader: dxil, msl, spv
	u32 shader_size;
    char format_shader_file[512];
    snprintf(format_shader_file, sizeof(format_shader_file), SHADER_DIR"%s.%s", path, format_name);
	u8* code = (u8*)resource_load_file(format_shader_file, &shader_size);

	// getting buffers
	u32 json_size;
	char json_shader_file[512];
	snprintf(json_shader_file, sizeof(json_shader_file), SHADER_DIR"%s.json", path);
	char* json_info = resource_load_file(json_shader_file, &json_size);
	u32 samplers, storage_textures, storage_buffers, uniform_buffers;
	sscanf(json_info, "{ \"samplers\": %d, \"storage_textures\": %d, \"storage_buffers\": %d, \"uniform_buffers\": %d",
		&samplers, 
		&storage_textures, 
		&storage_buffers, 
		&uniform_buffers
	);


	SDL_GPUShader* shader = SDL_CreateGPUShader(device, &(SDL_GPUShaderCreateInfo) {
		.code = code,
		.code_size = shader_size,
		.entrypoint = format_entrypoint,
		.format = format,
		.stage = stage,
		.num_samplers = samplers,
		.num_storage_buffers = storage_textures,
		.num_storage_textures = storage_buffers,
		.num_uniform_buffers = uniform_buffers
	});

	if (!shader) {
		log_err("failed to load gpu shader %s: %s", path, SDL_GetError());
	}

	free(code);
	free(json_info);

	return shader;
}
