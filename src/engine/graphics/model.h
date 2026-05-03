#ifndef MODEL_H_
#define MODEL_H_

// stores all graphical information regarding all models, including animations

// loads in models with cgltf: 
// - Skeleton: bones, hierarchy, bind pose
// - Skinning: vertex weights, joint indices, inverse bind matrices
// - Animation Clips: keyframes(translation, rotation, scale), time sampling
// - Runtime Evaluation: compute current pose, interpolate keyframes, build final bone matrices
// - GPU Skinning; send bone matrices to shader, deform verties in vertex shader

// animation.h gives abstract way to run its different animation cycles
#include <cglm/cglm.h>
#include "src/util/defines.h"
#include <SDL3/SDL_gpu.h>
#include "src/engine/graphics/render.h"

#define MODEL_PATH "res/characters/" // all models must be in this directory, 

// Reference to model that is stored on gpu
// - pass to render.h to render the model

typedef struct Animation Animation;

typedef struct {
    SDL_GPUGraphicsPipeline* pipeline;
    uint32_t index_count;
	Animation* animation;

	Mesh* meshes;
	u32 mesh_count;
} Model; 

Model* model_create(const char* path, const char* vertex_shader, const char* frag_shader);
void model_destroy(Model* model);


/*
void model_animate_run(Model m)
void model_animate_jump(Model m)
void model_animate_punch(Model m)
*/

#endif
