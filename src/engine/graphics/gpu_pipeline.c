#include "gpu_pipeline.h"
#include "src/util/logger.h"
#include "src/util/defines.h"
#include "src/util/resource_loader.h"
#include <stdio.h>
#include <stdlib.h>

SDL_GPUGraphicsPipeline* gpu_pipeline_load(SDL_GPUDevice* device, const char* vertex_path, const char* fragment_path, SDL_GPUTextureFormat color_format, SDL_GPUTextureFormat depth_format) {
	// grab respective shaders
	SDL_GPUShader* vertex_shader = resourse_load_shader(device, vertex_path);
	SDL_GPUShader* fragment_shader = resourse_load_shader(device, fragment_path);
	if (!vertex_shader || !fragment_shader) {
		log_err("failed to load shaders: %s & %s", vertex_path, fragment_path);
		return NULL;
	}

	// getting vertex attributes from json
	char json_file[512];
	snprintf(json_file, sizeof(json_file), SHADER_DIR"%s.json", vertex_path);
	char* json_attr = resourse_load_file(json_file);
/*
    Uint32 location;                    
    Uint32 buffer_slot;                 
    SDL_GPUVertexElementFormat format;  
    Uint32 offset;                      
*/

	u32 attribute_count = 0;
	char* start_of_attr = strstr(json_attr, "inputs") + 6; // start of array
	if (!start_of_attr) { // should always have inputs even without them
		log_warn("json file incorrect: %s", json_attr);
		return NULL;
	}
	for (; start_of_attr != '\0'; start_of_attr++) {
		if (start_of_attr == '{') {
			attribute_count++;
		} else if (start_of_attr == ']') {
			break; // end of inputs array
		}
	}

	// fills out attributes
	SDL_GPUVertexAttribute attributes[attribute_count];
	u32 attr_offset[attribute_count]; 

	char* attr_set = json_attr;
	for (u32 i = 0; i < attribute_count; i++) {
		char* type = NULL;
		u32 location = 0;
		attr_set = strstr(attr_set, "type") + 5; // goes to next set

		// grabs type
		for (char* k = attr_set; *k != '\0'; k++) {
			if (*k == '"') { // finds opening quote
				for (char* j = k + 1; *j != '\0'; j++) {
					if (*j == '"') { // finds closing quote
						*j = '\0';
						type = k+1; // sets type as a string with null terminator
						attr_set = j+1;
						break;
					}
				}
			}
		}

		// grabs location
		attr_set = strstr(attr_set, "location") + 8;
		for (char* k = attr_set; *k != '\0'; k++) {
			if (*k == ':') { // finds surround area
				for (char* j = k + 1; *j != '\0'; j++) {
					if (*j == '}') {
						*j = '\0';
						location = atoi(k + 1);
						attr_set = j + 1;
						break;
					}
				}
			}
		}

		attributes[i].buffer_slot = 0; // only every one, only one struct per shader
		attributes[i].location = location;

		// gets format
		if (!strncmp(type, "int", 3)) {
			if (*(type+3) == '4') { attributes[i].format = SDL_GPU_VERTEXELEMENTFORMAT_INT4; attr_offset[location] = 4 * sizeof(int); }
			else if (*(type+3) == '3') { attributes[i].format = SDL_GPU_VERTEXELEMENTFORMAT_INT3; attr_offset[location] = 3 * sizeof(int); } 
			else if (*(type+3) == '2') { attributes[i].format = SDL_GPU_VERTEXELEMENTFORMAT_INT2; attr_offset[location] = 2 * sizeof(int); }
			else { attributes[i].format = SDL_GPU_VERTEXELEMENTFORMAT_INT; attr_offset[location] = sizeof(int); }
		} else if (!strncmp(type, "float", 5)) {
			if (*(type+5) == '4') { attributes[i].format = SDL_GPU_VERTEXELEMENTFORMAT_FLOAT4; attr_offset[location] = 4 * sizeof(float); }
			else if (*(type+5) == '3') { attributes[i].format = SDL_GPU_VERTEXELEMENTFORMAT_FLOAT3; attr_offset[location] = 3 * sizeof(float); }
			else if (*(type+5) == '2') { attributes[i].format = SDL_GPU_VERTEXELEMENTFORMAT_FLOAT2; attr_offset[location] = 2 * sizeof(float); }
			else { attributes[i].format = SDL_GPU_VERTEXELEMENTFORMAT_FLOAT; attr_offset[location] = sizeof(float); }		
		} else {
			log_warn("not implemented yet, change for primitive support: %s", type);
			return NULL;
		}

	}

	// setting attribute offset
	for (u32 i = 0; i < attribute_count; i++) {
		u32 sum = 0;
		for (u32 j = 0; j < attributes[i].location; j++) {
			sum += attr_offset[j];
		}
		attributes[i].offset = sum;
	}



	SDL_GPUGraphicsPipeline* pipeline = SDL_CreateGPUGraphicsPipeline(device, &(SDL_GPUGraphicsPipelineCreateInfo) {
		.vertex_shader = vertex_shader,
		.fragment_shader = fragment_shader,
	});


	SDL_ReleaseGPUShader(device, vertex_shader);
	SDL_ReleaseGPUShader(device, fragment_shader);
	if (!pipeline) {
		log_err("failed to create gpu_pipeline");
		return NULL;
	}

	return pipeline;
}

void gpu_pipeline_unload(SDL_GPUDevice* device, SDL_GPUGraphicsPipeline* pipeline) {
	SDL_ReleaseGPUGraphicsPipeline(device, pipeline);
}


/*

   { "samplers": 0, "storage_textures": 0, "storage_buffers": 0, "uniform_buffers": 2, 
   "inputs": [], "outputs": [{ "name": "out_uv", "type": "float2", "location": 0 }] }

   */
