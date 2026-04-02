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

typedef u32 Audio_ID;

/*
	Stores all playable audio clips:
	- file paths should be stored inside 'resources/config.toml' file
*/

/*
   Each profile coorelates to particular game character or stage of the game/map
   - allows sound queueing and preloading of assets to not overburden system
   */
typedef enum {
	AUDIO_PROFILE_EMPTY,
	AUDIO_PROFILE_MAIN_MENU,
	AUDIO_PROFILE_SELECTION_SCREEN,
	AUDIO_PROFILE_TUX
} AudioProfile;

typedef enum {
	AUDIO_PLAYLIST_BACKGROUND_MUSIC,
	AUDIO_PLAYLIST_TUX_JUMP,
	AUDIO_PLAYLIST_TUX_PUNCH
} AudioPlaylist;

void audio_init(void);
void audio_uninit(void);
void audio_load_profile(AudioProfile profile);
void audio_unload_profile(AudioProfile profile);

// playing sounds
// - all audio clips must come from 'src/util/config.h' config global variable
// - no custom paths as they must all be defined within the configuration file
Audio_ID audio_oneshot(AudioPlaylist clip);
Audio_ID audio_loop(AudioPlaylist clip);
void audio_stop(Audio_ID id); // requires ID returned from above funcs


#endif
