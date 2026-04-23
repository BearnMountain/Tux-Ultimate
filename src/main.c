#include "enet/enet.h"
#define SDL_MAIN_USE_CALLBACKS
#include <SDL3/SDL.h>
#include <SDL3/SDL_main.h>

// engine
#include "engine/input.h"
#include "engine/graphics/render.h"
#include "engine/graphics/model.h"

// general
#include "util/logger.h"
#include "src/util/config.h"
#include "src/util/defines.h"

// server
#include "src/net/server/server.h"
#include "src/net/client/client.h"

typedef struct {
	SDL_Window* window;
	u32 width, height;

	// models
	Model* models[10];

	// multiplayer
	b8 enet_initialized;
} AppInfo;

static AppInfo app_info = {0};

SDL_AppResult SDL_AppInit(void** appstate, int argc, char** argv) {
	(void)argc;
	(void)argv;

	config_init();
	enet_initialize();

#if DEBUG
	SDL_SetLogPriorities(SDL_LOG_PRIORITY_VERBOSE);
#endif

	if (!SDL_Init(SDL_INIT_VIDEO)) {
		log_err("failed to initalize SDL: %s", SDL_GetError());
		return SDL_APP_FAILURE;
	}

	app_info.width = 960;
	app_info.height = 540;
	app_info.window = SDL_CreateWindow("Tux-Ultimate", app_info.width, app_info.height, SDL_WINDOW_RESIZABLE);
	if (app_info.window == NULL) {
		log_err("failed to initialize SDL: %s", SDL_GetError());
		return SDL_APP_FAILURE;
	}
	
	// have appstate target app_info
	*appstate = &app_info;

	// intializes renderer
	render_init(app_info.window);


	// loading models
	mat4 proj, view;
	glm_perspective(glm_rad(60.f), (float)frame_data.window_width / frame_data.window_height, 0.1f, 1000.f, proj);

	vec3 eye = {0, 1, 3}, center = {0, 0, 0}, up = {0, 1, 0};
	glm_lookat(eye, center, up, view);

	app_info.models[0] = model_create("res/characters/generic/tux.glb");
	// after model_create
	printf("primitive_count: %d\n", app_info.models[0]->primitive_count);
	printf("vertex_buffer: %p\n", (void*)app_info.models[0]->primitives[0].vertex_buffer);
	printf("index_count: %d\n", app_info.models[0]->primitives[0].index_count);

	return SDL_APP_CONTINUE;
}

SDL_AppResult SDL_AppIterate(void* appstate) {
	(void)appstate;
    // Acquire command buffer + swapchain FIRST
    frame_data.cmd = SDL_AcquireGPUCommandBuffer(frame_data.device);
    if (!frame_data.cmd) return SDL_APP_CONTINUE;

    if (!SDL_AcquireGPUSwapchainTexture(frame_data.cmd, frame_data.window,
            &frame_data.swapchain_texture, NULL, NULL)) {
        SDL_SubmitGPUCommandBuffer(frame_data.cmd);
        return SDL_APP_CONTINUE;
    }
    if (!frame_data.swapchain_texture) { // minimized window
        SDL_SubmitGPUCommandBuffer(frame_data.cmd);
        return SDL_APP_CONTINUE;
    }

	SDL_GPURenderPass* clear_pass = SDL_BeginGPURenderPass(
    frame_data.cmd,
    &(SDL_GPUColorTargetInfo){
        .texture  = frame_data.swapchain_texture,
        .load_op  = SDL_GPU_LOADOP_CLEAR,
        .store_op = SDL_GPU_STOREOP_STORE,
        .clear_color = {0.1f, 0.1f, 0.1f, 1.0f},
    }, 1, NULL);
	SDL_EndGPURenderPass(clear_pass);

	mat4 proj, view;
	glm_perspective(glm_rad(60.f), (float)frame_data.window_width / frame_data.window_height, 0.1f, 1000.f, proj);

	vec3 eye = {0, 1, 10}, center = {0, 0, 0}, up = {0, 1, 0};
	glm_lookat(eye, center, up, view);
	model_render(app_info.models, 1, view, proj);
	
    SDL_SubmitGPUCommandBuffer(frame_data.cmd);  // <-- presents to screen
	return SDL_APP_CONTINUE;
}

SDL_AppResult SDL_AppEvent(void* appstate, SDL_Event* event) {
	(void)appstate;

	switch (event->type) {
		case SDL_EVENT_KEY_DOWN: {
			input_handle(event->key.key);
			if (event->key.key == SDLK_Q || event->key.key == SDLK_ESCAPE) {
				return SDL_APP_SUCCESS;
			}
			
			switch(event->key.key) {
				case SDLK_1:
					server_start((ServerCreateInfo){
						.max_clients = SERVER_MAX_CLIENTS,
						.port = SERVER_PORT
					});
					break;
				case SDLK_2:
					server_stop();
					break;
				case SDLK_3:
					client_connect((ClientCreateInfo){
						.host_name = "localhost", // local for now
						.port = SERVER_PORT, // local for now
						.player_name = "Chap"
					});
					break;
				case SDLK_4:
					client_disconnect();
					break;
				case SDLK_5: {
					NetPacket* packet = packet_create_input((u8*)"hello there", 11);
					client_send_packet(packet, true);
					break;
				}
			}
		}
		default: break;
	}

	return SDL_APP_CONTINUE;
}

void SDL_AppQuit(void* appstate, SDL_AppResult result) {
	(void)appstate;
	(void)result;

	// destroy rendered models
	model_destroy(app_info.models[0]);

	// for networking
	client_disconnect();
	server_stop();

	enet_deinitialize();
	config_save();
	render_uninit();
	SDL_DestroyWindow(app_info.window);
	SDL_Quit();
}
