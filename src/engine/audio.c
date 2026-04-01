#include "audio.h"
#include "src/util/vector.h"

#define MA_NO_DEVICE_IO // disables ma_device API so that sdl handles sound instead of sdl
#define MINIAUDIO_IMPLEMENTATION
#include "external/miniaudio.h"
#include "src/util/logger.h"
#include <SDL3/SDL.h>

// singleton audio engine that plays for entire app:
// - 
struct {
	ma_engine engine;
	SDL_AudioDeviceID device;
	SDL_AudioSpec spec;
} audio_playlist = { 0 };

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
}

// playing sounds
Audio_ID audio_oneshot(AudioPath path) {
    ma_sound* sound = malloc(sizeof(ma_sound));
    if (!sound) return 0;

    if (ma_sound_init_from_file(
            &audio_playlist.engine,
            path.path,
            MA_SOUND_FLAG_DECODE | MA_SOUND_FLAG_NO_SPATIALIZATION,
            NULL,
            NULL,
            sound
        ) != MA_SUCCESS) {
        free(sound);
        return 0;
    }

    ma_sound_start(sound);
    return (Audio_ID)(uintptr_t)sound;
}

Audio_ID audio_loop() {


	return 0;
}

void audio_stop(Audio_ID id) {
    ma_sound* sound = (ma_sound*)(uintptr_t)id;
    if (!sound) return;

    ma_sound_stop(sound);
    ma_sound_uninit(sound);
    free(sound);
}
