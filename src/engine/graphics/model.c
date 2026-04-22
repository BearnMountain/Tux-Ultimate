#include "model.h"

#include "src/util/logger.h"
#include <string.h>

// helper funcs defined at end of file
cgltf_data* load_gltf(const char* path);
void* gltf_accessor_ptr(const cgltf_accessor* a);

Model* model_create(const char* path) {
	if (!path) { return NULL; }
	if (strcmp(path + strlen(path) - 4, ".glb") != 0) { // checks stem
		log_warn("model creation requires .glb file: %s invalid", path);
		return NULL;
	}

	Model* model = (Model*)malloc(sizeof(Model));

	cgltf_data* data = load_gltf(path);
	cgltf_free(data);

	return model;
}



// helpers
cgltf_data* load_gltf(const char* path) {
	cgltf_options opts = {0};
	cgltf_data* data = NULL;
	cgltf_result result = cgltf_parse_file(&opts, path, &data);
	if (result != cgltf_result_success) {
		log_err("failed to parse .glb file: %s", path);
		return NULL;
	}

	result = cgltf_load_buffers(&opts, data, path);
	if (result != cgltf_result_success) {
		cgltf_free(data);
		log_err("failed to load .glb buffers: %s", path);
		return NULL;
	}

	cgltf_validate(data);
	return data;
}

void* gltf_accessor_ptr(const cgltf_accessor* a) {
	return (i8*)a->buffer_view->buffer->data 
		+ a->buffer_view->offset 
		+ a->offset;
}

