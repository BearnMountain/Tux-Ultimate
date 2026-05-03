#define CGLTF_IMPLEMENTATION
#include "lib/gltf/gltf.h"

#include "stb_image.h"
#include "model.h"
#include "src/engine/graphics/gpu_pipeline.h"
#include "src/util/logger.h"

#include <string.h>

// opaque
typedef struct {
	vec3 position;
	vec3 rotation;
} JointTransform;

typedef struct {
	JointTransform* transforms;
	f32 time_stamp;
} KeyFrames;

typedef struct Animation {
	KeyFrames* frames;
} Animation;


// helpers


// models creation
Model* model_create(const char* path, const char* vertex_shader, const char* frag_shader) {
	if (!path) return NULL;
	if (strcmp(path + strlen(path) - 4, ".glb") != 0) { // checks stem
		log_warn("model creation requires '.glb' file: %s invalid", path);
		return NULL;
	}

	Model* model = malloc(sizeof(Model));
	model->pipeline = gpu_pipeline_load(
		vertex_shader,
		frag_shader,
		SDL_GPU_TEXTUREFORMAT_INVALID,
		SDL_GPU_TEXTUREFORMAT_INVALID
	);
	if (!model->pipeline) {
		log_err("failed to create model pipeline");
		return NULL;
	}

	// grabbing .glb data and loading into static model
	cgltf_data* data = NULL;
	cgltf_options options = {0};
	cgltf_result result = cgltf_parse_file(&options, path, &data);
	if (result != cgltf_result_success) {
		log_err("failed to parse .glb file: %s", path);
		gpu_pipeline_unload(model->pipeline);
		free(model);
		return NULL;
	}

	result = cgltf_load_buffers(&options, data, path);
	if (result != cgltf_result_success) {
		log_err("failed to loaded .glb buffers: %s", path);
		cgltf_free(data);
		gpu_pipeline_unload(model->pipeline);
		free(model);
		return NULL;
	}
	cgltf_validate(data);

	// counts total meshes
	u32 total_meshes = 0;
	for (u32 i = 0; i < data->meshes_count; i++) {
		total_meshes += data->meshes[i].primitives_count;
	}

	model->meshes = calloc(total_meshes, sizeof(Mesh));
	model->mesh_count = 0;

	// loop through all meshes and their primitives
	for (u32 i = 0; i < data->meshes_count; i++) {
		for (u32 j = 0; j < data->meshes[i].primitives_count; j++) {
			// position
			cgltf_primitive* primitive = &data->meshes[i].primitives[j];
			Mesh* mesh = &model->meshes[model->mesh_count++];

			// VERTEX
			cgltf_accessor* position = NULL;
			cgltf_accessor* normal = NULL;
			cgltf_accessor* uv = NULL;

			for (u32 k = 0; k < primitive->attributes_count; k++) {
				switch (primitive->attributes[k].type) {
					case cgltf_attribute_type_position: position = primitive->attributes[k].data; break;
					case cgltf_attribute_type_normal: 	normal = primitive->attributes[k].data; break;
					case cgltf_attribute_type_texcoord: uv = primitive->attributes[k].data; break;
					default: break;
				}
			}

			if (!position) {
				log_warn("model loading file error, primitive without positional mesh data");
				model->mesh_count--;
				continue;
			}

			u32 vertex_count = (u32)position->count;
			Vertex* vertices = malloc(vertex_count * sizeof(Vertex));

			for (u32 v = 0; v < vertex_count; v++) {
				cgltf_accessor_read_float(position, v, vertices[v].position, 3);
				if (normal) cgltf_accessor_read_float(normal, v, vertices[v].normal, 3);
				if (uv) cgltf_accessor_read_float(uv, v, vertices[v].uv, 2);
			}

			// INDEX
			u32 index_count = (u32)primitive->indices->count;
			u32* indices = malloc(index_count * sizeof(u32));

			for (u32 idx = 0; idx < index_count; idx++) {
				indices[idx] = cgltf_accessor_read_index(primitive->indices, idx);
			}

			mesh->index_count = index_count;

			// TEXTURE
			if (primitive->material && primitive->material->has_pbr_metallic_roughness && primitive->material->pbr_metallic_roughness.base_color_texture.texture) {
				cgltf_image* img = primitive->material->pbr_metallic_roughness.base_color_texture.texture->image;
			
				// texture
				u32 texture_width, texture_height, channels;
				u8* pixel = NULL;
				if (img->buffer_view) {
					void* raw = (char*)img->buffer_view->buffer->data + img->buffer_view->offset;
					size_t len = img->buffer_view->size;
					pixel = stbi_load_from_memory(raw, (int)len, (int*)&texture_width, (int*)&texture_height, (int*)&channels, 4);
				} else if (img->uri) {
					log_warn("external file reference for texture loading not allowed: %s", img->uri);
				}

				if (!pixel) {
					log_warn("failed to decode texture: %s", img->name ? img->name : "unnamed");
					free(vertices);
					free(indices);
					continue;
				}

				mesh->albedo = render_upload_texture(pixel, texture_width, texture_height);

				// sensible defaults
				SDL_GPUSamplerCreateInfo info = {
					.min_filter    = SDL_GPU_FILTER_LINEAR,
					.mag_filter    = SDL_GPU_FILTER_LINEAR,
					.mipmap_mode   = SDL_GPU_SAMPLERMIPMAPMODE_LINEAR,
					.address_mode_u = SDL_GPU_SAMPLERADDRESSMODE_REPEAT,
					.address_mode_v = SDL_GPU_SAMPLERADDRESSMODE_REPEAT,
					.address_mode_w = SDL_GPU_SAMPLERADDRESSMODE_REPEAT,
				};

				cgltf_sampler* sampler = primitive->material->pbr_metallic_roughness.base_color_texture.texture->sampler;
				if (sampler) {
					// mag filter
					switch (sampler->mag_filter) {
						case 9728: info.mag_filter = SDL_GPU_FILTER_NEAREST; break; // GL_NEAREST
						case 9729: info.mag_filter = SDL_GPU_FILTER_LINEAR;  break; // GL_LINEAR
						default:   break;
					}

					// min filter & mipmap
					switch (sampler->min_filter) {
						case 9728: // GL_NEAREST
							info.min_filter  = SDL_GPU_FILTER_NEAREST;
							info.mipmap_mode = SDL_GPU_SAMPLERMIPMAPMODE_NEAREST;
							break;
						case 9729: // GL_LINEAR
							info.min_filter  = SDL_GPU_FILTER_LINEAR;
							info.mipmap_mode = SDL_GPU_SAMPLERMIPMAPMODE_LINEAR;
							break;
						case 9984: // GL_NEAREST_MIPMAP_NEAREST
							info.min_filter  = SDL_GPU_FILTER_NEAREST;
							info.mipmap_mode = SDL_GPU_SAMPLERMIPMAPMODE_NEAREST;
							break;
						case 9985: // GL_LINEAR_MIPMAP_NEAREST
							info.min_filter  = SDL_GPU_FILTER_LINEAR;
							info.mipmap_mode = SDL_GPU_SAMPLERMIPMAPMODE_NEAREST;
							break;
						case 9986: // GL_NEAREST_MIPMAP_LINEAR
							info.min_filter  = SDL_GPU_FILTER_NEAREST;
							info.mipmap_mode = SDL_GPU_SAMPLERMIPMAPMODE_LINEAR;
							break;
						case 9987: // GL_LINEAR_MIPMAP_LINEAR
							info.min_filter  = SDL_GPU_FILTER_LINEAR;
							info.mipmap_mode = SDL_GPU_SAMPLERMIPMAPMODE_LINEAR;
							break;
						default: break;
					}

					// wrap modes
					switch (sampler->wrap_s) {
						case 33071: info.address_mode_u = SDL_GPU_SAMPLERADDRESSMODE_CLAMP_TO_EDGE; break;
						case 33648: info.address_mode_u = SDL_GPU_SAMPLERADDRESSMODE_MIRRORED_REPEAT; break;
						case 10497: info.address_mode_u = SDL_GPU_SAMPLERADDRESSMODE_REPEAT; break;
						default:    info.address_mode_u = SDL_GPU_SAMPLERADDRESSMODE_REPEAT; break;
					}
					switch (sampler->wrap_t) {
						case 33071: info.address_mode_v = SDL_GPU_SAMPLERADDRESSMODE_CLAMP_TO_EDGE; break;
						case 33648: info.address_mode_v = SDL_GPU_SAMPLERADDRESSMODE_MIRRORED_REPEAT; break;
						case 10497: info.address_mode_v = SDL_GPU_SAMPLERADDRESSMODE_REPEAT; break;
						default:    info.address_mode_v = SDL_GPU_SAMPLERADDRESSMODE_REPEAT; break;
					}

					info.address_mode_w = SDL_GPU_SAMPLERADDRESSMODE_REPEAT; // no W in gltf
				}
				mesh->sampler = SDL_CreateGPUSampler(frame_data.device, &info);

				stbi_image_free(pixel);
			}

			// upload to gpu
			mesh->vertex_buffer = render_upload_buffer(vertices, vertex_count * sizeof(Vertex), SDL_GPU_BUFFERUSAGE_VERTEX);
			mesh->index_buffer = render_upload_buffer(indices, index_count * sizeof(u32), SDL_GPU_BUFFERUSAGE_INDEX);
		}
	}

	cgltf_free(data);

	return model;
}

void model_destroy(Model* m) {
     if (!m) return;
     // SDL_GPUDevice* dev = frame_data.device;
     // for (int i = 0; i < m->primitive_count; i++) {
     //     SDL_ReleaseGPUBuffer(dev, m->primitives[i].vertex_buffer);
     //     SDL_ReleaseGPUBuffer(dev, m->primitives[i].indice_buffer);
     // }
	 SDL_ReleaseGPUGraphicsPipeline(frame_data.device, m->pipeline);
     free(m);
}
