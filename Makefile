# Project Settings
EXE_NAME  := Tux-Ultimate
SRC_DIR   := src
BUILD_DIR := build
EXT_DIR   := external
SDL_DIR   := $(EXT_DIR)/SDL

# Compiler & Flags
CC      := gcc
CFLAGS  := -Wall -Wextra -MMD -MP -I. -Iinclude -I$(EXT_DIR) -I$(EXT_DIR)/stb -I$(SDL_DIR)/include
LDFLAGS := -L$(BUILD_DIR)/sdl_lib -lSDL3 -lpthread -lm

# Detect OS
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S), Darwin)
    NPROC   := $(shell sysctl -n hw.ncpu)
    LIB_EXT := dylib
else
    NPROC   := $(shell nproc)
    LIB_EXT := so
endif

# Mode Logic
MODE ?= debug
ifeq ($(MODE), release)
    CFLAGS     += -O3 -DNDEBUG
    TARGET_DIR := $(BUILD_DIR)/release
else
    CFLAGS     += -g -O0 -DDEBUG
    TARGET_DIR := $(BUILD_DIR)/debug
endif

# Files
SRCS := $(shell find $(SRC_DIR) -name '*.c') $(EXT_DIR)/stb/stb_truetype.c
OBJS := $(patsubst %.c,$(TARGET_DIR)/%.o,$(SRCS))
DEPS := $(OBJS:.o=.d)
TARGET := $(TARGET_DIR)/$(EXE_NAME)

.PHONY: all clean run shaders sdl_vendor

all: sdl_vendor shaders $(TARGET)

sdl_vendor:
	@if [ ! -d $(SDL_DIR) ]; then \
		echo "Cloning SDL3 repository..."; \
		git clone https://github.com/libsdl-org/SDL.git $(SDL_DIR); \
	else \
		echo "SDL3 already cloned."; \
	fi
	@if [ ! -f $(BUILD_DIR)/sdl_lib/libSDL3.$(LIB_EXT) ]; then \
		cmake -S $(SDL_DIR) -B $(BUILD_DIR)/sdl_build -DSDL_TESTS=OFF -DSDL_EXAMPLES=OFF; \
		cmake --build $(BUILD_DIR)/sdl_build -j$(NPROC); \
		mkdir -p $(BUILD_DIR)/sdl_lib; \
		cp $(BUILD_DIR)/sdl_build/libSDL3* $(BUILD_DIR)/sdl_lib/; \
	fi

shaders:
	@python3 compile_shaders.py

$(TARGET): $(OBJS)
	@mkdir -p $(dir $@)
	$(CC) $(OBJS) -o $@ $(LDFLAGS) -Wl,-rpath,'@loader_path/../sdl_lib'

$(TARGET_DIR)/%.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

# This MUST be on its own line at the bottom
-include $(DEPS)

clean:
	rm -rf $(BUILD_DIR)

run: all
	./$(TARGET)
