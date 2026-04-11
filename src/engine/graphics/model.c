#include "model.h"

#include "src/util/logger.h"
#include "lib/gltf/gltf.h"
#include <string.h>

Model* model_create(const char* path) {
	Model* model = (Model*)malloc(sizeof(Model));
	
	// creates path from define to resource dir
	char full_path[512];
	strcpy(full_path, path);
	strcat(full_path, MODEL_PATH);

	cgltf_options opt = {
		0
	};
	cgltf_data* data = NULL;
	cgltf_result result = cgltf_parse_file(&opt, full_path, &data);
	if (result != cgltf_result_success) {
		log_err("failed to parse model file: %s", full_path);
		cgltf_free(data);
		return 0;
	}
	cgltf_load_buffers(&opt, data, full_path);
	if (result != cgltf_result_success) {
		log_err("failed to load buffer from model file: %s", full_path);
		cgltf_free(data);
		return 0;
	}

	// extracting skeleton
	// load inverse bind matrices from skin
	// set up animation reference

	return model;
}

void model_animate(Model* model, f32 time) {
	f32 dt = fmodf(time, model->duration);

	for (u32 i = 0; i < model->bone_count; i++) {
		// find keyframe for model->bones[i]
		// interpolate translation, rotation, and scale
		// build local matrix
		mat4 local = glm_translate(NULL, NULL);
	}
}
