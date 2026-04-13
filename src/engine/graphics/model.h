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
#include "lib/gltf/gltf.h"

#define MODEL_PATH "res/characters/" // all models must be in this directory, 

// typedef struct {
// 	vec3 pos;
// 	vec3 norm;
// 	vec2 tex_coord;
// 	vec4 joint_indices;
// 	vec4 joint_weights;
// } Model_Vertex;
//
// typedef struct {
// 	mat4 transform;
// 	u32 parent_idx;
// 	char name[64];
// } Model_Bone;
//
// typedef struct {
// 	u32 bone_count;
// 	Model_Bone* bones;
// 	mat4* inverse_bind_matrices;
// 	mat4* current_pos_matrices;
//
// 	// storage
// 	cgltf_animation* anim;
// 	f32 duration;
// } Model;


// creates model:
// - presumes path is prefixed by MODEL_PATH
// Model* model_create(const char* path);
// void model_destroy(Model* model);
//
// // placeholder for now
// void model_animate(Model* model, f32 time);


// Reference to model that is stored on gpu
// - pass to render.h to render the model
typedef struct {

} Model; 

Model* model_create(const char* path);

/*
void model_animate_run(Model m)
void model_animate_jump(Model m)
void model_animate_punch(Model m)
   */

#endif
