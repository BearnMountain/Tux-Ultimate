#include "camera.h"

#include <SDL3/SDL.h>

typedef struct Camera {
	vec3 velocity;
	vec3 position;

	versor rotation;

	mat4 model;
	mat4 projection;
	mat4 view;
} Camera;

void camera_init(Camera* camera) {
	glm_vec3_zero(camera->velocity);
	glm_vec3_zero(camera->position);
	glm_quat_identity(camera->rotation);
	glm_mat4_identity(camera->view);
}

void camera_process_event(Camera* camera, SDL_Event* e) {

	switch (e->key.key) {
        case SDLK_UP:
			camera->velocity[2] = -1.0f;
        	break;
        case SDLK_DOWN:
			camera->velocity[2] = 1.0f;
        	break;
        case SDLK_LEFT:
			camera->velocity[0] = -1.0f;
        	break;
        case SDLK_RIGHT:
			camera->velocity[0] = 1.0f;
        	break;
	}

	if (e->motion.type == SDL_EVENT_MOUSE_WHEEL) {
		if (e->motion.y > 0) { // scroll up
			
		} else if (e->motion.y < 0) { // scroll down

		}
	}

	// glm_quat_mat4(camera->rotation, )
}

mat4* get_view_matrix(Camera* camera) {

	return NULL;
}

mat4* get_projection_matrix(Camera* camera) {

	return NULL;
}

mat4* get_mvp(Camera* camera) {

	// 1. Create quaternion (angle in radians, axis)
	// glm::quat myQuaternion = glm::angleAxis(glm::radians(angle), glm::vec3(0, 1, 0));
	//
	// // 2. Convert to mat4
	// glm::mat4 rotationMatrix = glm::toMat4(myQuaternion);
	//
	// // 3. Update Model Matrix
	// glm::mat4 Model = TranslationMatrix * rotationMatrix * ScaleMatrix;
	//
	// // 4. Update MVP
	// glm::mat4 MVP = Projection * View * Model;
	mat4* mvp = malloc(sizeof(mat4));
	glm_mat4_mul(camera->projection, camera->view, mvp);
	glm_mat4_mul(mvp, camera->model, mvp);

	return mvp;
}
