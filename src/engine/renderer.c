#include "renderer.h"
#include "src/util/logger.h"
#include "src/engine/gpu_pipeline.h"
#include "src/util/config.h"

#include <stdlib.h>

struct {
	SDL_GPUBuffer* vertex_buffer[1024];
	SDL_GPUTransferBuffer* transfer_buffer[1024];
	u32 buffer_count;
} gpu_buffer_container = {0};

FrameData frame_data = {0};

// ------ Supporting Functions ------

void renderer_init(SDL_Window* window) {
	// initialize renderer global state
	frame_data.window = window;

	// selecting GPU Device
	frame_data.device = SDL_CreateGPUDevice(config.shader_format, DEBUG, NULL);
	if (!frame_data.device) {
		log_err("failed to select a gpu device: %s", SDL_GetError());
		return;
	}
	SDL_ClaimWindowForGPUDevice(frame_data.device, window);

	frame_data.pipeline = gpu_pipeline_load("rect.vert.glsl", "rect.frag.glsl");
	if (!frame_data.pipeline) {
		log_err("failed to load pipeline: %s", SDL_GetError());
		return;
	}

}

void renderer_uninit(void) {
	SDL_WaitForGPUIdle(frame_data.device);

	// releases gpu resources
	for (u32 i = 0; i < gpu_buffer_container.buffer_count; i++) {
		SDL_ReleaseGPUBuffer(frame_data.device, gpu_buffer_container.vertex_buffer[i]);
		SDL_ReleaseGPUTransferBuffer(frame_data.device, gpu_buffer_container.transfer_buffer[i]);
	}
	gpu_pipeline_unload(frame_data.pipeline);

	// free(gpu_buffer_container.vertex_buffer);
	// free(gpu_buffer_container.transfer_buffer);

	// destroy device
	SDL_DestroyGPUDevice(frame_data.device);
}

void renderer_frame(void) {
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
    SDL_BindGPUGraphicsPipeline(render_pass, frame_data.pipeline);

    // bind the vertex buffer
    SDL_GPUBufferBinding bufferBindings[1];
    bufferBindings[0].buffer = gpu_buffer_container.vertex_buffer[0]; // index 0 is slot 0 in this example
    bufferBindings[0].offset = 0; // start from the first byte

    SDL_BindGPUVertexBuffers(render_pass, 0, bufferBindings, 1); // bind one buffer starting from slot 0

    SDL_DrawGPUPrimitives(render_pass, 3, 1, 0, 0);

	SDL_EndGPURenderPass(render_pass);
	if(!SDL_SubmitGPUCommandBuffer(cmd_buffer)) {
		log_warn("failed to submit gpu command buffer: %s", SDL_GetError());
	}

}

void renderer_upload_vertices(Vertex* vertices, u32 count) {
	u32 buffer_size = sizeof(Vertex) * count;
	u32 buffer_index = gpu_buffer_container.buffer_count;

	// setup data to transfer to gpu
	gpu_buffer_container.vertex_buffer[buffer_index] = SDL_CreateGPUBuffer(frame_data.device, &(SDL_GPUBufferCreateInfo) {
		.size = buffer_size,
		.usage = SDL_GPU_BUFFERUSAGE_VERTEX
	});
	gpu_buffer_container.transfer_buffer[buffer_index] = SDL_CreateGPUTransferBuffer(frame_data.device, &(SDL_GPUTransferBufferCreateInfo) {
		.size = buffer_size,
		.usage = SDL_GPU_TRANSFERBUFFERUSAGE_UPLOAD
	});

	Vertex* data = (Vertex*)SDL_MapGPUTransferBuffer(frame_data.device, gpu_buffer_container.transfer_buffer[buffer_index], false);
	SDL_memcpy(data, (void*)vertices, buffer_size);
	SDL_UnmapGPUTransferBuffer(frame_data.device, gpu_buffer_container.transfer_buffer[buffer_index]);

	// start copy pass
	SDL_GPUCommandBuffer* cmd_buffer = SDL_AcquireGPUCommandBuffer(frame_data.device);
	SDL_GPUCopyPass* copy_pass = SDL_BeginGPUCopyPass(cmd_buffer);

	// upload data
	SDL_UploadToGPUBuffer(copy_pass, 
		&(SDL_GPUTransferBufferLocation) {
			.transfer_buffer = gpu_buffer_container.transfer_buffer[buffer_index],
			.offset = 0
		}, 
		&(SDL_GPUBufferRegion) {
			.buffer = gpu_buffer_container.vertex_buffer[buffer_index],
			.size = buffer_size,
			.offset = 0
		}, 
		true
	);

	// for next upload
	gpu_buffer_container.buffer_count++;
	
	// record draw commands
	SDL_EndGPUCopyPass(copy_pass);
	SDL_SubmitGPUCommandBuffer(cmd_buffer);
}

void renderer_submit_triangle(Vertex v[3], SDL_FColor color) {
	// make the colors uniform
	for (int i = 0; i < 3; i++) {
		v[i].r = color.r;
		v[i].g = color.g;
		v[i].b = color.b;
	}
	
	renderer_upload_vertices(v, 3);
}
