#ifndef RENDER_H_
#define RENDER_H_

#include "SDL3/SDL_gpu.h"
#include "cglm/types.h"
#include "src/util/defines.h"

// all data for gpu and rendering is stored here
// global for use of all files inside graphics/*
typedef struct {
	SDL_Window* window;
	SDL_GPUDevice* device;
	SDL_GPUCommandBuffer* cmd;
	SDL_GPUTexture* swapchain_texture;
	u32 window_width, window_height;
} FrameData;

extern FrameData frame_data;

// rendering 
// typedef struct {
// 	SDL_GPUBuffer* vertex_buffer;
// 	SDL_GPUBuffer* index_buffer;
// 	u32 index_count;
// } GPUMesh;
//
// typedef struct {
// 	vec3 pos;
// 	vec3 norm;
// 	vec2 uv;
// } ShaderVertex;
//
// typedef enum {
//     RENDER_CMD_MESH,
//     RENDER_CMD_SKINNED_MESH,   // animated
//     RENDER_CMD_UI_QUAD,
//     RENDER_CMD_UI_TEXT,
//     RENDER_CMD_SPRITE,
// } RenderCommandType;
//
// typedef struct {
// 	RenderCommandType type;
//
// 	SDL_GPUBuffer* vertex_buffer;
// 	SDL_GPUBuffer* index_buffer;
// } RenderCommand;


// typedef struct {
//     RenderCommandType type;
//
//     // sort key — render.c uses this to batch/sort everything
//     // encode: layer | shader_id | depth | texture_id
//     u64 sort_key;
//
//     union {
//         struct {
//             u32      mesh_id;
//             u32      material_id;
//             mat4     transform;
//         } mesh;
//
//         struct {
//             u32      mesh_id;
//             u32      material_id;
//             mat4     transform;
//             u32      skeleton_id;       // index into bone buffer on GPU
//             u32      anim_frame;
//         } skinned_mesh;
//
//         struct {
//             vec2     pos;
//             vec2     size;
//             vec4     color;
//             vec4     uv_rect;           // into atlas
//             u32      texture_id;
//         } ui_quad;
//
//         struct {
//             vec2     pos;
//             vec4     color;
//             u32      font_id;
//             u32      glyph_start;       // index into pre-built glyph buffer
//             u32      glyph_count;
//         } ui_text;
//     };
// } RenderCommand;

typedef struct {
    SDL_GPUBuffer* vertex_buffer;
    SDL_GPUBuffer* index_buffer;
    uint32_t index_count;
    mat4 model;

    // add when you have textures:
    SDL_GPUTexture* albedo;
    SDL_GPUSampler* sampler;

    // add when one model has multiple materials (e.g. a .obj with submeshes):
    // store a list of these instead of a single Mesh
    uint32_t index_offset;   // where in the index buffer this submesh starts
} Mesh;

void render_init(SDL_Window* window);
void render_uninit(void);

void render_frame(void);

// not copied
void render_submit_mesh(Mesh* mesh);

// uploads arbitrary data to gpu and returns buffer to use for binding for draw calls
SDL_GPUBuffer* render_upload_buffer(const void* data, u32 size, SDL_GPUBufferUsageFlags flags);

// submitting vertices to render to the screen
// GPUMesh render_upload_mesh(const cgltf_primitive* prim);

// void render_submit(
// SDL_GPUGraphicsPipeline* pipeline,
// SDL_GPUBuffer* vertex_buffer.
// SDL_GPUBuffer* indice_buffer,
// u32 index_count,
// u32 first_index,
// u32 vertex_offset
// )

#endif

// vulkan pipeline, shaders, draw calls
