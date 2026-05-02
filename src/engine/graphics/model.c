#define CGLTF_IMPLEMENTATION
#include "lib/gltf/gltf.h"

#include "model.h"

#include "src/engine/graphics/gpu_pipeline.h"
#include "src/engine/graphics/render.h"
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

	// getting vertice and attributes of model
	// for (u32 i = 0; i < data->materials_count; i++) {
	// 	cgltf_pbr_metallic_roughness clr = data->materials[i].pbr_metallic_roughness;
	// 	log_info("(%f, %f, %f, %f)", 
	// 		clr.base_color_factor[0],
	// 		clr.base_color_factor[1],
	// 		clr.base_color_factor[2],
	// 		clr.base_color_factor[3]
	// 	);
	// }

	// grabbing model data for loading characters
	// node and mesh loading

	// material and texture 

	// uploading to gpu


	// animation:
	// - pos, tex coord, norm, joint id, weights


	// loop through all meshes
	for (u32 i = 0; i < data->meshes_count; i++) {
		// loop through all primitives
		for (u32 j = 0; j < data->meshes[i].primitives_count; j++) {
			
			// position
			cgltf_primitive* primitive = &data->meshes[i].primitives[j];
			for (u32 k = 0; k < primitive->attributes_count; k++) {
				printf("\n%s\n", primitive->attributes[i].name);
				switch (primitive->attributes[i].type) {
					case cgltf_attribute_type_invalid:
						printf("invalid");
						break;
					case cgltf_attribute_type_position:
						printf("pos");
						break;
					case cgltf_attribute_type_normal:
						printf("norm");
						break;
					case cgltf_attribute_type_tangent:
						printf("tangent");
						break;
					case cgltf_attribute_type_texcoord:
						printf("texcoord");
						break;
					case cgltf_attribute_type_color:
						printf("clr");
						break;
					case cgltf_attribute_type_joints:
						printf("joint");
						break;
					case cgltf_attribute_type_weights:
						printf("weights");
						break;
					case cgltf_attribute_type_custom:
						printf("custom");
						break;
					case cgltf_attribute_type_max_enum:
						printf("enum");
						break;
				}
			}
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

// void* gltf_accessor_ptr(const cgltf_accessor* a) {
// 	return (i8*)a->buffer_view->buffer->data 
// 		+ a->buffer_view->offset 
// 		+ a->offset;
// }



// // helper funcs defined at end of file
// cgltf_data* load_gltf(const char* path);
// void* gltf_accessor_ptr(const cgltf_accessor* a);
// static void load_model_pipeline(void);
// static void unpack_f3(const cgltf_accessor* a, float* out);
// static void unpack_f2(const cgltf_accessor* a, float* out);
// static Vertex* build_vertices(const cgltf_primitive* prim, uint32_t* out_count);
// static uint32_t* build_indices(const cgltf_accessor* acc);
// static SDL_GPUBuffer* upload_buffer(SDL_GPUBufferUsageFlags usage, const void* data, size_t size);
//
// Model* model_create(const char* path) {
// 	if (!path) { return NULL; }
// 	if (strcmp(path + strlen(path) - 4, ".glb") != 0) { // checks stem
// 		log_warn("model creation requires '.glb' file: %s invalid", path);
// 		return NULL;
// 	}
//
// 	load_model_pipeline();
// 	cgltf_data* data = load_gltf(path);
//
// 	u32 total = 0;
// 	for (cgltf_size m = 0; m < data->meshes_count; m++) {
// 		total += (u32)data->meshes[m].primitives_count;
// 	}
//
// 	Model* model = (Model*)calloc(1, sizeof(Model));
// 	model->primitives = (Primitive*)calloc(total, sizeof(Primitive));
//
// 	for (cgltf_size m = 0; m < data->meshes_count; m++) {
// 		cgltf_mesh* mesh = &data->meshes[m];
// 		for (cgltf_size p = 0; p < mesh->primitives_count; p++) {
// 			cgltf_primitive* prim = &mesh->primitives[p];
// 			if (!prim->indices) {
// 				log_warn("skipping non-indexed primitives");
// 				continue;
// 			}
//
// 			u32 vert_count = 0;
// 			Vertex* verts = build_vertices(prim, &vert_count);
// 			if (!verts) continue;
// 			u32 ic = (u32)prim->indices->count;
// 			u32* idx = build_indices(prim->indices);
//
// 			Primitive* pr = &model->primitives[model->primitive_count++];
//             pr->vertex_buffer = upload_buffer(SDL_GPU_BUFFERUSAGE_VERTEX, verts, vert_count * sizeof(Vertex));
//             pr->indice_buffer = upload_buffer(SDL_GPU_BUFFERUSAGE_INDEX, idx, ic * sizeof(uint32_t));
//             pr->index_count = ic;
//             glm_mat4_identity(pr->transform);
//
//             free(verts);
//             free(idx);
// 		}
// 	}
//
// 	cgltf_free(data);
// 	log_info("model loaded '%s' (%d primtives)", path, model->primitive_count);
//
// 	return model;
// }
//
// void model_render(Model** list, int count, mat4 view, mat4 proj) {
//     if (!count || !g_pipeline) return;
//
//     SDL_GPURenderPass* pass = SDL_BeginGPURenderPass(
//         frame_data.cmd,
//         &(SDL_GPUColorTargetInfo){
//             .texture  = frame_data.swapchain_texture,
//             .load_op  = SDL_GPU_LOADOP_LOAD,
//             .store_op = SDL_GPU_STOREOP_STORE,
//         }, 1, NULL);   /* NULL = no depth, as requested */
//
//     SDL_BindGPUGraphicsPipeline(pass, g_pipeline);
//
//     for (int mi = 0; mi < count; mi++) {
//         Model* m = list[mi];
//         for (int pi = 0; pi < m->primitive_count; pi++) {
//             Primitive* pr = &m->primitives[pi];
//
//             /* MVP = proj * view * model */
//             ModelUniforms u;
//             glm_mat4_mul(proj, view,         u.mvp);
//             glm_mat4_mul(u.mvp, pr->transform, u.mvp);
//
//             SDL_PushGPUVertexUniformData(frame_data.cmd, 0, &u, sizeof(u));
//
//             SDL_BindGPUVertexBuffers(pass, 0,
//                 &(SDL_GPUBufferBinding){ .buffer = pr->vertex_buffer }, 1);
//             SDL_BindGPUIndexBuffer(pass,
//                 &(SDL_GPUBufferBinding){ .buffer = pr->indice_buffer },
//                 SDL_GPU_INDEXELEMENTSIZE_32BIT);
//
//             SDL_DrawGPUIndexedPrimitives(pass, pr->index_count, 1, 0, 0, 0);
//         }
//     }
//
//     SDL_EndGPURenderPass(pass);
// }
//
// void model_destroy(Model* m) {
//     if (!m) return;
//     SDL_GPUDevice* dev = frame_data.device;
//     for (int i = 0; i < m->primitive_count; i++) {
//         SDL_ReleaseGPUBuffer(dev, m->primitives[i].vertex_buffer);
//         SDL_ReleaseGPUBuffer(dev, m->primitives[i].indice_buffer);
//     }
//     free(m->primitives);
//     free(m);
// }
//
// // helpers
// cgltf_data* load_gltf(const char* path) {
// 	cgltf_options opts = {0};
// 	cgltf_data* data = null;
// 	cgltf_result result = cgltf_parse_file(&opts, path, &data);
// 	if (result != cgltf_result_success) {
// 		log_err("failed to parse .glb file: %s", path);
// 		return null;
// 	}
//
// 	result = cgltf_load_buffers(&opts, data, path);
// 	if (result != cgltf_result_success) {
// 		cgltf_free(data);
// 		log_err("failed to load .glb buffers: %s", path);
// 		return null;
// 	}
//
// 	cgltf_validate(data);
// 	return data;
// }
//
// void* gltf_accessor_ptr(const cgltf_accessor* a) {
// 	return (i8*)a->buffer_view->buffer->data 
// 		+ a->buffer_view->offset 
// 		+ a->offset;
// }
//
// static void load_model_pipeline(void) {
// 	if (g_pipeline) return;
// 	g_pipeline = gpu_pipeline_load(
// 		"model.vert",
// 		"model.frag",
// 		sdl_gpu_textureformat_invalid,
// 		sdl_gpu_textureformat_invalid
// 	);
// 	if(!g_pipeline) {
// 		log_err("failed to create model pipeline");
// 	}
// }
//
// static void unpack_f3(const cgltf_accessor* a, float* out) {
//     for (cgltf_size i = 0; i < a->count; i++)
//         cgltf_accessor_read_float(a, i, out + i * 3, 3);
// }
// static void unpack_f2(const cgltf_accessor* a, float* out) {
//     for (cgltf_size i = 0; i < a->count; i++)
//         cgltf_accessor_read_float(a, i, out + i * 2, 2);
// }
//
// static Vertex* build_vertices(const cgltf_primitive* prim, uint32_t* out_count) {
//     const cgltf_accessor *pos = NULL, *norm = NULL, *uv = NULL;
//     for (cgltf_size i = 0; i < prim->attributes_count; i++) {
//         switch (prim->attributes[i].type) {
//             case cgltf_attribute_type_position: pos  = prim->attributes[i].data; break;
//             case cgltf_attribute_type_normal:   norm = prim->attributes[i].data; break;
//             case cgltf_attribute_type_texcoord: uv   = prim->attributes[i].data; break;
//             default: break;
//         }
//     }
//     if (!pos) return NULL;
//
//     uint32_t n = (uint32_t)pos->count;
//     *out_count = n;
//
//     float* positions = malloc(n * 3 * sizeof(float));
//     float* normals   = calloc(n * 3, sizeof(float));
//     float* uvs       = calloc(n * 2, sizeof(float));
//
//     unpack_f3(pos, positions);
//     if (norm) unpack_f3(norm, normals);
//     if (uv)   unpack_f2(uv,   uvs);
//
//     Vertex* verts = malloc(n * sizeof(Vertex));
//     for (uint32_t i = 0; i < n; i++) {
//         verts[i].pos[0]    = positions[i*3+0];
//         verts[i].pos[1]    = positions[i*3+1];
//         verts[i].pos[2]    = positions[i*3+2];
//         verts[i].normal[0] = normals[i*3+0];
//         verts[i].normal[1] = normals[i*3+1];
//         verts[i].normal[2] = normals[i*3+2];
//         verts[i].uv[0]     = uvs[i*2+0];
//         verts[i].uv[1]     = uvs[i*2+1];
//     }
//     free(positions); free(normals); free(uvs);
//     return verts;
// }
//
// static uint32_t* build_indices(const cgltf_accessor* acc) {
//     uint32_t* idx = malloc(acc->count * sizeof(uint32_t));
//     for (cgltf_size i = 0; i < acc->count; i++)
//         cgltf_accessor_read_uint(acc, i, &idx[i], 1);
//     return idx;
// }
//
// static SDL_GPUBuffer* upload_buffer(SDL_GPUBufferUsageFlags usage, const void* data, size_t size) {
//     SDL_GPUDevice* dev = frame_data.device;
//
//     SDL_GPUBuffer* buf = SDL_CreateGPUBuffer(dev, &(SDL_GPUBufferCreateInfo){
//         .usage = usage,
//         .size  = (uint32_t)size,
//     });
//
//     SDL_GPUTransferBuffer* staging = SDL_CreateGPUTransferBuffer(dev,
//         &(SDL_GPUTransferBufferCreateInfo){
//             .usage = SDL_GPU_TRANSFERBUFFERUSAGE_UPLOAD,
//             .size  = (uint32_t)size,
//         });
//
//     void* mapped = SDL_MapGPUTransferBuffer(dev, staging, false);
//     memcpy(mapped, data, size);
//     SDL_UnmapGPUTransferBuffer(dev, staging);
//
//     SDL_GPUCommandBuffer* cmd  = SDL_AcquireGPUCommandBuffer(dev);
//     SDL_GPUCopyPass* pass = SDL_BeginGPUCopyPass(cmd);
//     SDL_UploadToGPUBuffer(pass,
//         &(SDL_GPUTransferBufferLocation){ .transfer_buffer = staging, .offset = 0 },
//         &(SDL_GPUBufferRegion){ .buffer = buf, .offset = 0, .size = (uint32_t)size },
//         false);
//     SDL_EndGPUCopyPass(pass);
//     SDL_SubmitGPUCommandBuffer(cmd);
//     SDL_ReleaseGPUTransferBuffer(dev, staging);
//
//     return buf;
// }
