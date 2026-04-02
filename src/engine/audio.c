#include "audio.h"
#include "src/util/vector.h"
#include "src/util/config.h"
#include "src/util/logger.h"

#define MA_NO_DEVICE_IO // disables ma_device API so that sdl handles sound instead of sdl
#define MINIAUDIO_IMPLEMENTATION
#include "external/miniaudio.h"
#include <SDL3/SDL.h>

#define MAX_AUDIO_ENTRIES

// singleton audio engine that plays for entire app:
// - 

typedef struct {
	AudioPlaylist reference;
	ma_sound* instances;
	u32 count;
} AudioEntry;

typedef struct {
	AudioEntry* clips;
	u32 clip_count;
	AudioProfile profile;
} AudioProfileEntry;

struct {
	ma_engine engine;
	ma_resource_manager resource_manager;
	SDL_AudioDeviceID device;
	SDL_AudioSpec spec;

	// storage of audio
	u32 profile_count;
	AudioProfileEntry* profiles;
} audio_playlist = { 0 };

/* TODO:
   

	ma_sound_init_from_file to ma_sound array
	ma_sound_init_copy to copy from sound array, then start and unit after finished and free
	- ma_sound_at_end callback func that can have ma_sound_uninit and free(instace) to get rid of copy

   */

#define CHANNELS 2
#define SAMPLE_RATE 48000

void data_callback(void* user_data, ma_uint8* stream, u32 stream_len) {
	(void)user_data;

	const ma_uint32 frames = (ma_uint32)stream_len 
		/ ma_get_bytes_per_frame(ma_format_f32, audio_playlist.spec.channels);

	ma_engine_read_pcm_frames(
		&audio_playlist.engine, 
		stream, 
		frames, 
		NULL
	);
}

// fills AudioEntry with n clips
void load_n_audio_clips(const char* path, AudioPlaylist playlist_ref, u32 n, AudioEntry* entry) {
	if (!n) {
		log_warn("must load more than 0 instances");
		return;
	}

	entry->instances = (ma_sound*)malloc(n * sizeof(ma_sound));
	if (!entry->instances) {
		log_err("failed to malloc audio instances");
		return;
	}
	entry->count = n;
	entry->reference = playlist_ref;

	// loading audio clips into AudioEntry
	ma_result result;
	for (u32 i = 0; i < n; i++) {
		result = ma_sound_init_from_file(
			&audio_playlist.engine,
			path,
			MA_SOUND_FLAG_DECODE,
			NULL,
			NULL,
			&entry->instances[i]
		);

		if (result != MA_SUCCESS) {
			log_err("failed to load: %s", path);

			// unload everything
			for (u32 j = 0; j < i; j++) {
				ma_sound_uninit(&entry->instances[j]);
			}
			free(entry->instances);
			return;
		}
	}

	// TODO: figure out if init_from_file is 'n' sys calls or if they're cached by the engine
}

void audio_load_profile(AudioProfile profile) {
	// ensure profile isn't allocated multiples times
	for (u32 i = 0; i < audio_playlist.profile_count; i++) {
		if (audio_playlist.profiles[i].profile == profile) {
			log_warn("profile already loaded");
			return;
		}
	}

	// find empty spot or resize
	AudioProfileEntry* entry = NULL;
	for (u32 i = 0; i < audio_playlist.profile_count; i++) {
		if (audio_playlist.profiles[i].profile == AUDIO_PROFILE_EMPTY) {
			entry = &audio_playlist.profiles[i];
		}
	}

	// didn't find empty spot
	if (entry == NULL) {
		AudioProfileEntry* tmp = (AudioProfileEntry*)realloc(audio_playlist.profiles, (audio_playlist.profile_count+1) * sizeof(AudioProfileEntry));
		if (!tmp) {
			log_err("audio playlist's profile realloc failed");
			return;
		}
		audio_playlist.profiles = tmp;

		// resize for new entry
		entry = &audio_playlist.profiles[audio_playlist.profile_count];
	}

	entry->profile = profile;	
	switch (profile) {
		case AUDIO_PROFILE_MAIN_MENU:
			entry->clip_count = 1;
			entry->clips = (AudioEntry*)malloc(entry->clip_count * sizeof(AudioEntry));

			// setting entries
			load_n_audio_clips(config.audio_main_menu, AUDIO_PLAYLIST_BACKGROUND_MUSIC, 1, &entry->clips[0]);
			break;
		case AUDIO_PROFILE_SELECTION_SCREEN:
			log_warn("AUDIO_PROFILE_SELECTION_SCREEN not implemented yet");
			break;
		case AUDIO_PROFILE_TUX: // all custom tux sound clips
			entry->clip_count = 2;
			entry->clips = (AudioEntry*)malloc(entry->clip_count * sizeof(AudioEntry));

			// setting entries
			load_n_audio_clips(config.audio_jump, AUDIO_PLAYLIST_TUX_JUMP, 8, &entry->clips[0]);
			load_n_audio_clips(config.audio_punch, AUDIO_PLAYLIST_TUX_PUNCH, 8, &entry->clips[1]);
			break;
		default: 
			log_warn("profile's not implemented yet");
			break;
	}
	audio_playlist.profile_count++; // increments after so that profile count can be used to access current profile entry
}

void audio_unload_profile(AudioProfile profile) {
	for (u32 i = 0; i < audio_playlist.profile_count; i++) {
		if (audio_playlist.profiles[i].profile == profile) {
			// remove all clips
			for (u32 j = 0; j < audio_playlist.profiles[j].clip_count; j++) {
				AudioEntry* entry = &audio_playlist.profiles[i].clips[j];

				// removing all instances of duplicated sound clips
				if (entry->instances != NULL) {
					for (u32 k = 0; k < entry->count; k++) {
						ma_sound_uninit(&entry->instances[k]);
					}
					free(entry->instances);
				}
			}

			free(audio_playlist.profiles[i].clips);
			audio_playlist.profiles[i].clip_count = 0;
			audio_playlist.profiles[i].profile = AUDIO_PROFILE_EMPTY; // allows for loading to take this spot
				
			return;
		}
	}
}

void audio_init() {
	// init mini audio
	ma_engine_config engine_cfg = ma_engine_config_init();
	engine_cfg.noDevice = MA_TRUE;
	engine_cfg.channels = CHANNELS;
	engine_cfg.sampleRate = SAMPLE_RATE;

	if (ma_engine_init(&engine_cfg, &audio_playlist.engine) != MA_SUCCESS) {
		log_err("failed to initialize miniaudio engine");
		return;
	}

	// init sdl audio
	// SDL_InitSubSystem(SDL_INIT_AUDIO);
	SDL_zero(audio_playlist.spec);
	audio_playlist.spec.freq = SAMPLE_RATE;
	audio_playlist.spec.format = SDL_AUDIO_F32;
	audio_playlist.spec.channels = CHANNELS;

	audio_playlist.device = SDL_OpenAudioDevice(
		audio_playlist.device,
		&audio_playlist.spec
	);

	if (!audio_playlist.device) {
		log_err("failed to open audio device: %s", SDL_GetError());
		return;
	}

	SDL_PauseAudioDevice(audio_playlist.device);
}

void audio_uninit() {
	if (audio_playlist.device) {
		SDL_CloseAudioDevice(audio_playlist.device);
	}
	ma_engine_uninit(&audio_playlist.engine);
	SDL_QuitSubSystem(SDL_INIT_AUDIO);
}

// playing sounds
Audio_ID audio_oneshot(AudioPlaylist clip) {
    // ma_sound* sound = malloc(sizeof(ma_sound));
    // if (!sound) return 0;
    //
    // if (ma_sound_init_from_file(
    //         &audio_playlist.engine,
    //         path,
    //         MA_SOUND_FLAG_DECODE | MA_SOUND_FLAG_NO_SPATIALIZATION,
    //         NULL,
    //         NULL,
    //         sound
    //     ) != MA_SUCCESS) {
    //     free(sound);
    //     return 0;
    // }
    //
    // ma_sound_start(sound);
    // return (Audio_ID)(uintptr_t)sound;
}

Audio_ID audio_loop(AudioPlaylist clip) {


	return 0;
}

void audio_stop(Audio_ID id) {
    // ma_sound* sound = (ma_sound*)(uintptr_t)id;
    // if (!sound) return;
    //
    // ma_sound_stop(sound);
    // ma_sound_uninit(sound);
    // free(sound);
}
