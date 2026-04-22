#include "render.h"
#include "src/util/logger.h"
#include "src/engine/graphics/gpu_pipeline.h"
#include "src/util/config.h"

#include <stdlib.h>

struct {

} gpu_buffer_container = {0};

FrameData frame_data = {0};

// ------ Supporting Functions ------

void render_init(SDL_Window* window) {
	// initialize renderer global state
	frame_data.window = window;

	// selecting GPU Device
	frame_data.device = SDL_CreateGPUDevice(config.shader_format, DEBUG, NULL);
	if (!frame_data.device) {
		log_err("failed to select a gpu device: %s", SDL_GetError());
		return;
	}
	SDL_ClaimWindowForGPUDevice(frame_data.device, window);

}

void render_uninit(void) {
	SDL_WaitForGPUIdle(frame_data.device);

	// releases gpu resources
	// for (u32 i = 0; i < gpu_buffer_container.buffer_count; i++) {
	// 	SDL_ReleaseGPUBuffer(frame_data.device, gpu_buffer_container.vertex_buffer[i]);
	// 	SDL_ReleaseGPUTransferBuffer(frame_data.device, gpu_buffer_container.transfer_buffer[i]);
	// }
	// gpu_pipeline_unload(frame_data.pipeline);

	// free(gpu_buffer_container.vertex_buffer);
	// free(gpu_buffer_container.transfer_buffer);

	// destroy device
	SDL_DestroyGPUDevice(frame_data.device);
}

void render_frame(void) {
	// aquire command buffer
	SDL_GPUCommandBuffer* cmd_buffer = SDL_AcquireGPUCommandBuffer(frame_data.device);
	if (!cmd_buffer) {
		log_err("failed to get command buffer: %s", SDL_GetError());
		return;
	}

	// aquire swapchain 
	SDL_WaitAndAcquireGPUSwapchainTexture(
		cmd_buffer,
		frame_data.window,
		&frame_data.swapchain_texture,
		&frame_data.window_width,
		&frame_data.window_height
	);

	if (!frame_data.swapchain_texture) {
		log_err("swapchain texture failed to be loaded: %s", SDL_GetError());
		SDL_SubmitGPUCommandBuffer(cmd_buffer);
		return;
	}
	
	// begin render pass
	SDL_GPUColorTargetInfo color_target = {
		.texture = frame_data.swapchain_texture,
		.clear_color = {0.1f, 0.1f, 0.15f, 1.0f},
		.load_op = SDL_GPU_LOADOP_CLEAR,
		.store_op = SDL_GPU_STOREOP_STORE
	};

	SDL_GPURenderPass* render_pass = SDL_BeginGPURenderPass(
		cmd_buffer,
		&color_target,
		1,
		NULL
	);

    // bind the pipeline
    // SDL_BindGPUGraphicsPipeline(render_pass, frame_data.pipeline);
    //
    // // bind the vertex buffer
    // SDL_GPUBufferBinding bufferBindings[1];
    // bufferBindings[0].buffer = gpu_buffer_container.vertex_buffer[0]; // index 0 is slot 0 in this example
    // bufferBindings[0].offset = 0; // start from the first byte
    //
    // SDL_BindGPUVertexBuffers(render_pass, 0, bufferBindings, 1); // bind one buffer starting from slot 0
    //
    // SDL_DrawGPUPrimitives(render_pass, 3, 1, 0, 0);

	SDL_EndGPURenderPass(render_pass);
	if(!SDL_SubmitGPUCommandBuffer(cmd_buffer)) {
		log_warn("failed to submit gpu command buffer: %s", SDL_GetError());
	}

}


GPUMesh render_upload_mesh(const cgltf_primitive* prim) {
	const cgltf_accessor* pos = NULL;
	const cgltf_accessor* norm = NULL;
	const cgltf_accessor* uv = NULL;
	(void)norm;(void)uv;

	// parsing primitives
	for (size_t i = 0; i < prim->attributes_count; i++) {
		cgltf_attribute* attr = &prim->attributes[i];

		switch (attr->type) {
			case cgltf_attribute_type_position:
				pos = attr->data;
				break;
			case cgltf_attribute_type_normal:
				norm = attr->data;
				break;
			case cgltf_attribute_type_texcoord:
				uv = attr->data;
				break;
			case cgltf_attribute_type_tangent:
				log_warn("cgltf tangent not implemented yet");
				break;
			case cgltf_attribute_type_color:
				log_warn("cgltf color not implemented yet");
				break;
			case cgltf_attribute_type_joints:
				log_warn("cgltf joints not implemented yet");
				break;
			case cgltf_attribute_type_weights:
				log_warn("cgltf weights not implemented yet");
				break;
			case cgltf_attribute_type_custom:
				log_warn("cgltf custom not implemented yet");
				break;
			case cgltf_attribute_type_invalid:
				log_warn("cgltf invalid");
				break;
			default:
				log_warn("not even in cgltf enum");
				break;
		}
	}

	size_t vertex_count = pos->count;
	ShaderVertex* vertices = (ShaderVertex*)malloc(sizeof(ShaderVertex) * vertex_count);

	//f32* pos = accessor_ptr

	free(vertices);

	GPUMesh mesh = {0};

	return mesh;
}



















