#include "enet/enet.h"
#define SDL_MAIN_USE_CALLBACKS
#include <SDL3/SDL.h>
#include <SDL3/SDL_main.h>

#include "engine/input.h"
#include "engine/graphics/render.h"
#include "util/logger.h"
#include "src/util/config.h"
#include "src/util/defines.h"

typedef struct {
	SDL_Window* window;
	u32 width, height;

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


	return SDL_APP_CONTINUE;
}

SDL_AppResult SDL_AppIterate(void* appstate) {
	(void)appstate;

	render_frame();
	
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
		}
		default: break;
	}

	return SDL_APP_CONTINUE;
}

void SDL_AppQuit(void* appstate, SDL_AppResult result) {
	(void)appstate;
	(void)result;

	enet_deinitialize();
	config_save();
	render_uninit();
	SDL_DestroyWindow(app_info.window);
	SDL_Quit();
}
