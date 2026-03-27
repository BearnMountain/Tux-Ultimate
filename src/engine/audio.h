#ifndef AUDIO_H_
#define AUDIO_H_

/*
 * SFX: oneshot steps, impact, ui sounds
 * Ambience: looping env sounds, location trigger sound sfx, 
 * Music: state driven
 *
 * Mixing Buses: volume, eq, compression, reverb, send snapshots swap mix presets instantly
 *
 */

#include "src/util/defines.h"

// DEFINED AUDIO CLIPS
#define AUDIO_BASIC_ "res/audio/basic.mp3"

typedef u32 Audio_ID;

void audio_init();
void audio_uninit();

// playing sounds
Audio_ID audio_oneshot();
Audio_ID auido_loop();

#endif
