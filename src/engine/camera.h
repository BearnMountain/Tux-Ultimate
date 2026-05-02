#ifndef CAMERA_H_
#define CAMERA_H_

#include "src/util/defines.h"
#include <cglm/cglm.h>
#include <SDL3/SDL.h>

typedef struct Camera Camera;

// updates by absolute value
void camera_process_event(Camera* camera, SDL_Event* e);
mat4* get_view_matrix(Camera* camera);
mat4* get_projection_matrix(Camera* camera);

#endif
