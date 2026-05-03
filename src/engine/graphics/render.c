#include "render.h"
#include "cglm/cam.h"
#include "src/util/logger.h"
#include "src/util/config.h"

#include <stdlib.h>

// struct Mesh {
//     SDL_GPUBuffer* vertexBuffer = nullptr;
//     SDL_GPUBuffer* indexBuffer  = nullptr;
//     uint32_t       indexCount   = 0;
//
//     // transform — rebuilt into MVP on the CPU each frame
//     glm::mat4      modelMatrix  = glm::mat4(1.0f);
// };
//
// void draw(SDL_GPURenderPass* pass, const Mesh& mesh, const glm::mat4& vp) {
//     SDL_GPUBufferBinding vbind = { mesh.vertexBuffer, 0 };
//     SDL_BindGPUVertexBuffers(pass, 0, &vbind, 1);
//
//     SDL_GPUBufferBinding ibind = { mesh.indexBuffer, 0 };
//     SDL_BindGPUIndexBuffer(pass, &ibind, SDL_GPU_INDEXELEMENTSIZE_32BIT);
//
//     glm::mat4 mvp = vp * mesh.modelMatrix;  // vp = P*V, precomputed once per frame
//     SDL_PushGPUVertexUniformData(pass, 0, &mvp, sizeof(mvp));
//
//     SDL_DrawGPUIndexedPrimitives(pass, mesh.indexCount, 1, 0, 0, 0);
// }

struct {
	RenderInstance* instances;
	u32 render_instances;
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

	for (u32 i = 0; i < gpu_buffer_container.render_instances; i++) {
		for (u32 j = 0; j < gpu_buffer_container.instances[i].mesh_count; i++) {
			Mesh* mesh = &gpu_buffer_container.instances[i].meshes[j];
			SDL_ReleaseGPUBuffer(frame_data.device, mesh->index_buffer);
			SDL_ReleaseGPUBuffer(frame_data.device, mesh->vertex_buffer);
			SDL_ReleaseGPUTexture(frame_data.device, mesh->albedo);
			SDL_ReleaseGPUSampler(frame_data.device, mesh->sampler);


		}
		SDL_ReleaseGPUGraphicsPipeline(frame_data.device, gpu_buffer_container.instances[i].pipeline);
		free(gpu_buffer_container.instances[i].meshes);
	}

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

	// running through commands and figuring out what to load and draw
	mat4 vp;
    mat4 view, proj;
    vec3 eye    = { 0.0f, 0.0f, 10.0f };  // camera position
    vec3 center = { 0.0f, 0.0f, 0.0f };   // look-at target
    vec3 up     = { 0.0f, 1.0f, 0.0f };

    // View matrix
    glm_lookat(eye, center, up, view);

    // Projection matrix
    glm_perspective(
        glm_rad(60.0f),   // FOV
        (float)(frame_data.window_width) / frame_data.window_height,           // width / height
        0.1f,             // near plane
        100.0f,           // far plane
        proj
    );

    // VP = Projection * View
    glm_mat4_mul(proj, view, vp);

/*
bind pipeline/shader
bind vertex buffer
bind index buffer
bind textures + samplers   ← must be done before draw
bind uniform buffers       ← must be done before draw
DrawIndexedPrimitives(...)
*/

	
	for (u32 i = 0; i < gpu_buffer_container.mesh_count; i++) {
		Mesh* m = &gpu_buffer_container.meshes[i];

		SDL_BindGPUGraphicsPipeline(render_pass, m->pipeline);

		SDL_GPUBufferBinding vbind = { m->vertex_buffer, 0 };
		SDL_BindGPUVertexBuffers(render_pass, 0, &vbind, 1);

		SDL_GPUBufferBinding ibind = { m->index_buffer, 0 };
		SDL_BindGPUIndexBuffer(render_pass, &ibind, SDL_GPU_INDEXELEMENTSIZE_32BIT);

		mat4 mvp;
		glm_mat4_mul(vp, m->model, mvp);
		SDL_PushGPUVertexUniformData(cmd_buffer, 0, &mvp, sizeof(mvp));

		SDL_DrawGPUIndexedPrimitives(render_pass, gpu_buffer_container.meshes[i].index_count, 1, 0, 0, 0);
	}

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

void render_submit_mesh(Mesh* mesh, u32 mesh_count) {
	gpu_buffer_container.meshes = realloc(gpu_buffer_container.meshes, sizeof(Mesh) * (gpu_buffer_container.mesh_count + mesh_count));
	for (u32 i = 0; i < mesh_count; i++) {
		memcpy(&gpu_buffer_container.meshes[gpu_buffer_container.mesh_count++], &mesh[i], sizeof(Mesh));
	}
}

SDL_GPUBuffer* render_upload_buffer(const void* data, u32 size, SDL_GPUBufferUsageFlags flags) {
	SDL_GPUBufferCreateInfo buf_info = { .usage = flags, .size = size };
	SDL_GPUBuffer* buffer = SDL_CreateGPUBuffer(frame_data.device, &buf_info);
	if (!buffer) {
		log_err("failed to create upload buffer");
		return NULL;
	}

	// transfer buffer
	SDL_GPUTransferBufferCreateInfo transfer_info = {
		.usage = SDL_GPU_TRANSFERBUFFERUSAGE_UPLOAD,
		.size = size
	};
	SDL_GPUTransferBuffer* transfer = SDL_CreateGPUTransferBuffer(frame_data.device, &transfer_info);
	if (!transfer) {
		log_err("failed to create transfer buffer");
		SDL_ReleaseGPUBuffer(frame_data.device, buffer);
		return NULL;
	}

	// staging: data -> transfer buffer
	void* mapped = SDL_MapGPUTransferBuffer(frame_data.device, transfer, false);
	memcpy(mapped, data, size);
	SDL_UnmapGPUTransferBuffer(frame_data.device, transfer);
	
	// upload: transfer buffer -> gpu
	SDL_GPUCommandBuffer* cmd = SDL_AcquireGPUCommandBuffer(frame_data.device);
	SDL_GPUCopyPass* pass = SDL_BeginGPUCopyPass(cmd);

	SDL_GPUTransferBufferLocation src = { .transfer_buffer = transfer, .offset = 0 };
	SDL_GPUBufferRegion dst = { .buffer = buffer, .offset = 0, .size = size };
	SDL_UploadToGPUBuffer(pass, &src, &dst, false);

	SDL_EndGPUCopyPass(pass);
	SDL_SubmitGPUCommandBuffer(cmd);
	SDL_ReleaseGPUTransferBuffer(frame_data.device, transfer);

	return buffer;
}



SDL_GPUTexture* render_upload_texture(const u8* data, u32 texture_width, u32 texture_height) {
	u32 size = texture_height * texture_width * 4;
    SDL_GPUTextureCreateInfo tinfo = {
        .type                 = SDL_GPU_TEXTURETYPE_2D,
        .format               = SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM,
        .usage                = SDL_GPU_TEXTUREUSAGE_SAMPLER,
        .width                = texture_width,
        .height               = texture_height,
        .layer_count_or_depth = 1,
        .num_levels           = 1,
    };
    SDL_GPUTexture* texture = SDL_CreateGPUTexture(frame_data.device, &tinfo);

    // transfer buffer upload
    SDL_GPUTransferBufferCreateInfo tbinfo = {
        .usage = SDL_GPU_TRANSFERBUFFERUSAGE_UPLOAD,
        .size  = size,
    };
    SDL_GPUTransferBuffer* tb = SDL_CreateGPUTransferBuffer(frame_data.device, &tbinfo);

    void* mapped = SDL_MapGPUTransferBuffer(frame_data.device, tb, false);
    memcpy(mapped, data, size);
    SDL_UnmapGPUTransferBuffer(frame_data.device, tb);

	// upload to gpu
    SDL_GPUCommandBuffer* cmd = SDL_AcquireGPUCommandBuffer(frame_data.device);
    SDL_GPUCopyPass* pass = SDL_BeginGPUCopyPass(cmd);

    SDL_GPUTextureTransferInfo src = {
        .transfer_buffer = tb,
        .offset = 0,
    };
    SDL_GPUTextureRegion dst = {
        .texture = texture,
        .mip_level = 0,
        .layer = 0,
        .x = 0, .y = 0, .z = 0,
        .w = texture_width, 
		.h = texture_height, 
		.d = 1,
    };
    SDL_UploadToGPUTexture(pass, &src, &dst, false);

    SDL_EndGPUCopyPass(pass);
    SDL_SubmitGPUCommandBuffer(cmd);
    SDL_ReleaseGPUTransferBuffer(frame_data.device, tb);

	return texture;
}





















