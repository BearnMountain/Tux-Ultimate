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
#include "src/util/config.h"

typedef u32 Audio_ID;

/*
	Stores all playable audio clips:
	- file paths should be stored inside 'resources/config.toml' file
*/

typedef enum {
	AUDIO_PROFILE_MAIN_MENU,
	AUDIO_PROFILE_SELECTION_SCREEN,
	AUDIO_PROFILE_TUX
} AudioProfile;

typedef enum {
	AUDIO_PLAYLIST_BACKGROUND_MUSIC
} AudioPlaylist;

void audio_init(void);
void audio_uninit(void);

// playing sounds
// - all audio clips must come from 'src/util/config.h' config global variable
// - no custom paths as they must all be defined within the configuration file
Audio_ID audio_oneshot(AudioPlaylist clip);
Audio_ID audio_loop(AudioPlaylist clip);
void audio_stop(Audio_ID id); // requires ID returned from above funcs


#endif
