#include "gpu_pipeline.h"
#include "renderer.h"
#include "src/util/logger.h"
#include "src/util/defines.h"

#define SHADER_DIR(path) "shaders/" XSTR(path)

SDL_GPUShader* load_shader(const char* path) {
    // get shader stage
    SDL_GPUShaderStage stage;
    if (strstr(path, "vert") != NULL) stage = SDL_GPU_SHADERSTAGE_VERTEX;
    else if (strstr(path, "frag") != NULL) stage = SDL_GPU_SHADERSTAGE_FRAGMENT;
    else {
        log_err("Could not determine shader type for: %s", path);
		return NULL;
    }

	// get shader format that is compiled on different platforms
	SDL_GPUShaderFormat format = SDL_GetGPUShaderFormats(frame_data.device);
	char* format_name = "spv";
	char* format_entrypoint = "main";
	// if (format & SDL_GPU_SHADERFORMAT_SPIRV) {
	// 	format = SDL_GPU_SHADERFORMAT_SPIRV;
	// 	format_name = "spv";
	// 	format_entrypoint = "main";
	// } else if (format & SDL_GPU_SHADERFORMAT_DXIL) { 
	// 	format = SDL_GPU_SHADERFORMAT_DXIL;
	// 	log_warn("not implemented");
	// 	return NULL;
	// } else if (format & SDL_GPU_SHADERFORMAT_MSL) {
	// 	format = SDL_GPU_SHADERFORMAT_MSL;
	// 	format_name = "msl";
	// 	format_entrypoint = "main0";
	// 	return NULL;
	// }  else {
	// 	log_err("dont support any shader formats supplied");
	// 	return NULL;
	// }
	(void)format_name;
	(void)format; // todo

	size_t shader_size;
	void* code = SDL_LoadFile(SHADER_DIR(path), &shader_size);

	SDL_GPUShader* shader = SDL_CreateGPUShader(frame_data.device, &(SDL_GPUShaderCreateInfo) {
		.code = (Uint8*)code,
		.code_size = shader_size,
		.entrypoint = format_entrypoint,
		.format = SDL_GPU_SHADERFORMAT_SPIRV,
		.stage = stage,
		.num_samplers = 0,
		.num_storage_buffers = 0,
		.num_storage_textures = 0,
		.num_uniform_buffers = 0
	});
	if (!shader) {
		log_err("failed to load gpu shader %s: %s", path, SDL_GetError());
	}

	SDL_free(code);

	return shader;
}

SDL_GPUGraphicsPipeline* gpu_pipeline_load(const char* vertex_path, const char* fragment_path) {
	// grab respective shaders
	SDL_GPUShader* vertex_shader = load_shader(vertex_path);
	SDL_GPUShader* fragment_shader = load_shader(fragment_path);
	if (!vertex_shader || !fragment_shader) {
		return NULL;
	}

	SDL_GPUVertexBufferDescription vert_buffer_description = {
		.slot = 0,
		.input_rate = SDL_GPU_VERTEXINPUTRATE_VERTEX,
		.instance_step_rate = 0,
		.pitch = sizeof(Vertex)
	};

	// describe the vertex attribute
	SDL_GPUVertexAttribute vertexAttributes[2];

	// a_position
	vertexAttributes[0].buffer_slot = 0; // fetch data from the buffer at slot 0
	vertexAttributes[0].location = 0; // layout (location = 0) in shader
	vertexAttributes[0].format = SDL_GPU_VERTEXELEMENTFORMAT_FLOAT3; //vec3
	vertexAttributes[0].offset = 0; // start from the first byte from current buffer position

	// a_color
	vertexAttributes[1].buffer_slot = 0; // use buffer at slot 0
	vertexAttributes[1].location = 1; // layout (location = 1) in shader
	vertexAttributes[1].format = SDL_GPU_VERTEXELEMENTFORMAT_FLOAT4; // vec4
	vertexAttributes[1].offset = sizeof(float) * 3; // 4th float from current buffer position
	
	SDL_GPUColorTargetDescription clr_target_description = {
		.format = SDL_GetGPUSwapchainTextureFormat(frame_data.device, frame_data.window)
	};

	// create pipeline
	SDL_GPUGraphicsPipelineCreateInfo pipeline_info = {
		.vertex_shader = vertex_shader,
		.fragment_shader = fragment_shader,
		.primitive_type = SDL_GPU_PRIMITIVETYPE_TRIANGLELIST,
		.vertex_input_state.num_vertex_buffers = 1,
		.vertex_input_state.vertex_buffer_descriptions = &vert_buffer_description,
		.vertex_input_state.num_vertex_attributes = 2,
		.vertex_input_state.vertex_attributes = vertexAttributes,
		.target_info.num_color_targets = 1,
		.target_info.color_target_descriptions = &clr_target_description
	};

	SDL_GPUGraphicsPipeline* gpu_pipeline = SDL_CreateGPUGraphicsPipeline(frame_data.device, &pipeline_info);
	SDL_ReleaseGPUShader(frame_data.device, vertex_shader);
	SDL_ReleaseGPUShader(frame_data.device, fragment_shader);
	if (!gpu_pipeline) {
		log_err("failed to create gpu_pipeline");
		return NULL;
	}

	return gpu_pipeline;
}

void gpu_pipeline_unload(SDL_GPUGraphicsPipeline* pipeline) {
	SDL_ReleaseGPUGraphicsPipeline(frame_data.device, pipeline);
}
