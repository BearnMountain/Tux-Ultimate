#ifndef TEXT_ATLAS_H_
#define TEXT_ATLAS_H_

#include "SDL3_ttf/SDL_ttf.h"
#include "SDL3/SDL.h"
#include "util/inc.h"

typedef struct {
    SDL_GPUTexture* texture; // glyph atlas
    SDL_GPUSampler* sampler; // should be linear always
    u32 atlas_width;
    u32 atlas_height;
    u32 font_size;
} FontAtlas;


#endif
