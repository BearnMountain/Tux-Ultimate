#ifndef CAMERA_H_
#define CAMERA_H_

#include "util/defines.h"
#include <cglm/cglm.h>

void camera_update();
void camera_process_event(SDL_Event* e);

#endif
