#include "src/util/defines.h"
#define SDL_MAIN_USE_CALLBACKS
#include <SDL3/SDL.h>
#include <SDL3/SDL_main.h>

#include "engine/input.h"
#include "engine/renderer.h"
#include "util/logger.h"

typedef struct {
	SDL_Window* window;
	SDL_GPUDevice* gpu;
	u32 width, height;
} AppInfo;

static AppInfo app_info = {0};

SDL_AppResult SDL_AppInit(void** appstate, int argc, char** argv) {
	(void)appstate;
	(void)argc;
	(void)argv;

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

	// intializes renderer
	renderer_init();

	return SDL_APP_CONTINUE;
}

SDL_AppResult SDL_AppIterate(void* appstate) {
	(void)appstate;
	return SDL_APP_CONTINUE;
}

SDL_AppResult SDL_AppEvent(void* appstate, SDL_Event* event) {
	(void)appstate;

	switch (event->type) {
		case SDL_EVENT_KEY_DOWN: {
			input_handle(event->key.key);
		}
		default: break;
	}

	return SDL_APP_CONTINUE;
}

void SDL_AppQuit(void* appstate, SDL_AppResult result) {
	(void)appstate;
	(void)result;

	SDL_DestroyWindow(app_info.window);
	renderer_uninit();
}
