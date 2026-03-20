#ifndef UI_H_
#define UI_H_

#include "SDL3/SDL_events.h"
#include "external/clay.h"

void ui_init(void);
void ui_uninit(void);
void ui_event_handler(SDL_Event e);

#endif
