#include "text_atlas.h"
#include "util/logger.h"
#include "deps/stb/stb_truetype.h"
#include <stdlib.h>

#define FONT_ATLAS_WIDTH 1024
#define FONT_ATLAS_HEIGHT 1024
#define FIRST_CHAR 32
#define LAST_CHAR 126
#define GLYPH_COUNT (LAST_CHAR - FIRST_CHAR + 1)

void text_init() {

}

TTFImage* create_atlas_from_ttf(const char* ttf_path, u32 font_size) {
    // checks
    if (!ttf_path || font_size < 1) {
        log_warn("create_atlas_from_ttf requires valid path and/or valid font_size");
        return NULL;
    }
    if (strlen(ttf_path) < 5) {
        log_warn("create_atlas_from_ttf path is not long enough, must include .ttf extension");
        return NULL;
    }
    const char* ttf_extension = &ttf_path[strlen(ttf_path) - 4];
    if (strcmp(ttf_extension, ".ttf")) {
        log_warn("create_atlas_from_ttf doesnt end with .ttf extnesion");
        return NULL;
    }

    // load font file
    u64 size;
    u8* font_buffer;

    FILE* font_file = fopen(ttf_path, "rb");
    fseek(font_file, 0, SEEK_END);
    size = ftell(font_file); /* how long is the file ? */
    fseek(font_file, 0, SEEK_SET); /* reset */

    font_buffer = malloc(size);

    fread(font_buffer, size, 1, font_file);
    fclose(font_file);

    // creating font bitmap
    u8* font_atlas_bitmap = (u8*)malloc(FONT_ATLAS_HEIGHT * FONT_ATLAS_WIDTH * sizeof(u8));

    // information on how to render the glyphs
    stbtt_packedchar packed_chars[GLYPH_COUNT];
    stbtt_aligned_quad aligned_quad[GLYPH_COUNT];

    // load information to bitmap
    stbtt_pack_context context;
    stbtt_PackBegin(
        &context,
        (unsigned char*)font_atlas_bitmap,
        FONT_ATLAS_WIDTH,
        FONT_ATLAS_HEIGHT,
        0,
        1,
        NULL
    );
    stbtt_PackFontRange(
        &context,                                     // stbtt_pack_context
        font_buffer,                              // Font Atlas texture data
        0,                                        // Font Index
        font_size,                                 // Size of font in pixels. (Use STBTT_POINT_SIZE(fontSize) to use points)
        FIRST_CHAR,                     // Code point of the first character
        GLYPH_COUNT,                // No. of charecters to be included in the font atlas
        packed_chars                               // stbtt_packedchar array, this struct will contain the data to render a glyph
    );
    stbtt_PackEnd(&context);

    for (int i = 0; i < GLYPH_COUNT; i++) {
        float unusedX, unusedY;

        stbtt_GetPackedQuad(
            localState.packedChars,              // Array of stbtt_packedchar
            FONT_ATLAS_WIDTH,                      // Width of the font atlas texture
            FONT_ATLAS_HEIGHT,                     // Height of the font atlas texture
            i,                                   // Index of the glyph
            &unusedX, &unusedY,                  // current position of the glyph in screen pixel coordinates, (not required as we have a different corrdinate system)
            &aligned_quads[i],                    // stbtt_alligned_quad struct. (this struct mainly consists of the texture coordinates)
            0                                    // Allign X and Y position to a integer (doesn't matter because we are not using 'unusedX' and 'unusedY')
        );
    };



    return NULL;
}

FontAtlas* text_create(SDL_GPUDevice* device, const char* ttf_path) {
    FontAtlas* atlas = (FontAtlas*)malloc(sizeof(FontAtlas));


    atlas->texture = SDL_CreateGPUTexture(device, &(SDL_GPUTextureCreateInfo) {
        .type = SDL_GPU_TEXTURETYPE_2D_ARRAY,
		.format = SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM,
		.width = imageData1->w,
		.height = imageData1->h,
		.layer_count_or_depth = 2,
		.num_levels = 1,
		.usage = SDL_GPU_TEXTUREUSAGE_SAMPLER
    });

    atlas->sampler = SDL_CreateGPUSampler(device)
}

void text_destroy(FontAtlas* atlas) {

    free(atlas);
}

void text_render() {
    // SDL_GPUTextureSamplerBinding tex_binding = {
    //     .texture = texture,    // GPU-resident handle
    //     .sampler = sampler
    // };
    //
    // SDL_BindGPUTextureSamplers(
    //     pass,
    //     0,                     // binding slot
    //     &tex_binding,
    //     1
    // );
    // SDL_DrawGPUPrimitives(
    //     pass,
    //     6,    // quad indices
    //     1,    // instance count
    //     0,
    //     0
    // );
}



















































/*
 * Packs font into a 2^n by 2^n atlas
 * stores it all inside a SDL_GPUTexture
 */
// Atlas* text_atlas_create(SDL_GPUDevice* device, const char* font_file_path, u32 font_size) {
//     u32 image_width, image_height;
//     image_height = 0;
//     image_width = 0;
//
//     SDL_Color* buffer = NULL;
//
//     // fill in buffer
//
//     Atlas* atlas = (Atlas*)malloc(sizeof(Atlas));
//
//     // creates packet texture
//     atlas->texture = SDL_CreateGPUTexture(device, &(SDL_GPUTextureCreateInfo){
// 		.type = SDL_GPU_TEXTURETYPE_2D_ARRAY,
// 		.format = SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM, // for rgba color struct
// 		.width = image_width,
// 		.height = image_height,
// 		.layer_count_or_depth = 2,
// 		.num_levels = 1,
// 		.usage = SDL_GPU_TEXTUREUSAGE_SAMPLER // can use it in shaders
//     });
//     if (!atlas->texture) {
//         log_err("text_atlas_create failed to load texture: %s with error %s", font_file_path, SDL_GetError());
//         return NULL;
//     }
//
//     // creates transfer buffer to hold pixel data
//     u32 buffer_size = image_height * image_width * sizeof(SDL_Color);
//     SDL_GPUTransferBuffer* transfer_buffer = SDL_CreateGPUTransferBuffer(device, &(SDL_GPUTransferBufferCreateInfo){
//         .usage = SDL_GPU_TRANSFERBUFFERUSAGE_UPLOAD,
//         .size = buffer_size
//     });
//     if (!transfer_buffer) {
//         log_err("text_atlas_create failed to create transfer buffer: %s", SDL_GetError());
//         return NULL;
//     }
//
//     // map transfer buffer and copy color data into it
//     void* transfer_data = SDL_MapGPUTransferBuffer(device, transfer_buffer, false);
//     if (transfer_buffer) {
//         memcpy(transfer_data, buffer, buffer_size);
//         SDL_UnmapGPUTransferBuffer(device, transfer_buffer);
//     } else {
//         log_err("text_atlas_create failed to map transfer buffer");
//         SDL_ReleaseGPUTransferBuffer(device, transfer_buffer);
//         SDL_ReleaseGPUTexture(device, atlas->texture);
//         return NULL;
//     }
//
//     // aquire command buffer to upload/cache to gpu
//     SDL_GPUCommandBuffer* cmd = SDL_AcquireGPUCommandBuffer(device);
//     if (!cmd) {
//         SDL_ReleaseGPUTransferBuffer(device, transfer_buffer);
//         SDL_ReleaseGPUTexture(device, atlas->texture);
//         return NULL;
//     }
//
//     SDL_GPUCopyPass* copy_pass = SDL_BeginGPUCopyPass(cmd);
//
//     // upload to command buffer
//     SDL_UploadToGPUTexture(copy_pass,
//         &(SDL_GPUTextureTransferInfo) {
//             .transfer_buffer = transfer_buffer,
//             .offset = 0,
//             .pixels_per_row = image_width,
//             .rows_per_layer = image_height
//         },
//         &(SDL_GPUTextureRegion) {
//             .texture = atlas->texture,
//             .w = image_width,
//             .h = image_height,
//             .d = 1 // 2d texture
//         }
//         , false
//     );
//
//     SDL_EndGPUCopyPass(copy_pass);
//     SDL_SubmitGPUCommandBuffer(cmd);
//     SDL_ReleaseGPUTransferBuffer(device, transfer_buffer);
//
//     return atlas;
// }

// #define MAX_VERTEX_COUNT 4000
// #define MAX_INDEX_COUNT  6000
//
// typedef struct Vec3 {
//     f32 x, y, z;
// } Vec3;
//
// typedef struct Vertex {
//     Vec3 pos;
//     SDL_FColor colour;
//     SDL_FPoint uv;
// } Vertex;
//
// typedef struct GeometryData {
//     Vertex *vertices;
//     u32 vertex_count;
//     u32 *indices;
//     u32 index_count;
// } GeometryData;
//
// static TTF_Font* font = NULL;
// static TTF_TextEngine* engine = NULL;
// static GeometryData geometry = {0};
//
// // creates single instance of text engine for rendering under sdl_gpu
// b8 text_init(SDL_GPUDevice* device, const char* font_file_path, u32 font_size) {
//     font = TTF_OpenFont(font_file_path, font_size);
//     if (!font) {
//         log_err("text_init failed to load text file: %s", font_file_path);
//         return 0;
//     }
//
//     engine = TTF_CreateGPUTextEngine(device);
//     if (!engine) {
//         log_err("text_init failed to create GPU text engine");
//         return 0;
//     }
//
//     geometry.vertices = SDL_calloc(MAX_VERTEX_COUNT, sizeof(Vertex));
//     geometry.indices = SDL_calloc(MAX_INDEX_COUNT, sizeof(u32));
//
//     SDL_
//
//
//     return 1;
// }
//
// // frees ttf
// void text_uninit(void) {
//     TTF_CloseFont(font);
//     TTF_DestroyGPUTextEngine(engine);
//     TTF_Quit();
// }
//
// // creates single instance of some
// TTF_Text* text_create(const char* sentence) {
//     if (!sentence) {
//         log_err("text_create please provide a valid sentence");
//         return NULL;
//     }
//     if (!engine || !font) {
//         log_err("text_create please called text_init to setup text engine and font");
//         return NULL;
//     }
//
//     TTF_Text* text = TTF_CreateText(engine, font, sentence, 0);
//     if (!text) {
//         log_err("text_create failed to generate text: %s", sentence);
//         return NULL;
//     }
//
//     return text;
// }
//
// void text_destroy(TTF_Text* text) {
//     TTF_DestroyText(text);
//     text = NULL;
// }
//
// void text_render(SDL_GPURenderPass* pass, const TTF_Text* text, int x, int y) {
// }
