#include "model.h"

#ifndef CGLTF_IMPLEMENTATION // just in case
#define CGLTF_IMPLEMENTATION
#endif

#include "src/util/logger.h"
#include "lib/gltf/gltf.h"

typedef struct {
	u32 meshes;
	u32 nodes;
	u32 images;
	u32 materials;
} ModelInfo;

void load_gltf(const char* path) {
	// path 
	char full_path[512];
	strcpy(full_path, path);
	strcat(full_path, MODEL_PATH);

	// loading in animated file
	cgltf_data* data = NULL;

	cgltf_result result = cgltf_parse_file(
		&(cgltf_options){0},
		full_path,
		&data
	);
	if (result != cgltf_result_success) {
		log_err("failed to load model file: %s", full_path);
		cgltf_free(data);
		return;
	}

	cgltf_validate(data);
}



Model* model_create(const char* path) {
	

	return NULL;
}
