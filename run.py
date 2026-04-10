#!/opt/homebrew/bin/python3

# chose build options
# - release: creates release build
# - debug: builds and runs with debug flag
# - build: just builds all .c files
# - clean: cleans up everything
# - shaders: builds shaders into platform specific formats
# - vendor: installs all libs that are vendor for static linking

import subprocess
import shutil
import sys
import os
from pathlib import Path

# configuration setup
BINARY     = "Tux-Ultimate"
BUILD_DIR  = Path("build")
OBJ_OUT    = BUILD_DIR / "obj"
SHADER_OUT = BUILD_DIR / "shaders"

SRC_DIR    = Path("src")
LIB_DIR    = Path("lib")
SHADER_DIR = Path("res/shaders")

LIBS       = "sdl3 cglm"

CC         = os.getenv("CC", "clang")
PLATFORM   = "linux" if sys.platform.startswith("linux") else sys.platform # linux, win32, darwin

CFLAGS = [
    "-std=c17",
    "-Iinclude",
    "-Ilib/stb",
    "-Ilib/clay",
]
LDFLAGS = [
]

# TODO: remove for vendored libs later on
CFLAGS  += subprocess.run(['pkg-config', '--cflags', LIBS], capture_output=True, text=True).stdout.split()
LDFLAGS += subprocess.run(['pkg-config',   '--libs', LIBS], capture_output=True, text=True).stdout.split()

# platform specific constants
match PLATFORM:
    case "win32":
        SHADER_FMT = "dxil";
    case "darwin":
        SHADER_FMT = "msl";
    case "linux":
        SHADER_FMT = "spv";
    case _:
        print("Unsupported platform")
        sys.exit(1)

def write_ninja():
    # finds all compiled and to be compiled files for ninja
    src_files = list(SRC_DIR.rglob("*.c"))
    shader_files = list(SHADER_DIR.rglob("*.glsl")) # always .glsl file format

    with open("build.ninja", "w") as ninja:
        # ninja rules
        ninja.write(
            f"ninja_required_version = 1.10\n"
            f"\n"
            f"cc = {CC}\n"
            f"cflags = {" ".join(CFLAGS)}\n"
            f"ldflags = {" ".join(LDFLAGS)}\n"
            f"builddir = {BUILD_DIR}\n"
            f"\n"
            f"rule cc\n" # compilation rule
            f"  command = $cc $cflags -c $in -o $out\n"
            f"  description = CC $in\n"
            f"\n"
            f"rule link\n" # linking objs
            f"  command = $cc $in -o $out $ldflags\n"
            f"  description = LINK $out\n"
            f"\n"
            f"rule glslc_vert\n" # compiling shaders of all types
            f"  command = glslc -fshader-stage=vert $in -o $out\n"
            f"  description = GLSLC $in\n"
            f"\n"
            f"rule glslc_frag\n" 
            f"  command = glslc -fshader-stage=frag $in -o $out\n"
            f"  description = GLSLC $in\n"
            f"\n"
            f"rule glslc_geom\n" 
            f"  command = glslc -fshader-stage=geom $in -o $out\n"
            f"  description = GLSLC $in\n"
            f"\n"
            f"rule glslc_comp\n" 
            f"  command = glslc -fshader-stage=comp $in -o $out\n"
            f"  description = GLSLC $in\n"
            f"\n"
            f"rule shader\n" # reflection or platform shader
            f"  command = shadercross $in -o $out\n"
            f"  description = SHADER $in\n"
            f"\n"
        )

        # c build
        obj_out = []
        for src in src_files:
            object_file = OBJ_OUT / (src.stem + ".o")
            ninja.write(
                    f"build {object_file}: cc {src}\n"
            )
            obj_out.append(object_file)

        # shader build
        shader_out = []
        for shader in shader_files:
            spv = SHADER_OUT / (shader.stem + ".spv")
            refl = SHADER_OUT / (shader.stem + ".json")
            stage = [ sub for sub in ["geom", "frag", "vert", "comp"] if sub in shader.name ]
            if not stage:
                print(shader / " is incorrect filename")
                continue

            ninja.write(f"build {spv}: glslc_{stage[0]} {shader}\n")
            ninja.write(f"build {refl}: shader {spv}\n")
            shader_out.append(spv)
            shader_out.append(refl)

            if SHADER_FMT == "dxil":
                dxil_shader = SHADER_OUT / (shader.stem + ".dxil")
                ninja.write(f"build {dxil_shader}: shader {spv}\n")
                shader_out.append(dxil_shader)
            if SHADER_FMT == "msl":
                msl_shader = SHADER_OUT / (spv.stem + ".msl")
                ninja.write(f"build {msl_shader}: shader {spv}\n")
                shader_out.append(msl_shader)

        # linking everything together
        ninja.write(
            f"\n"
            f"build {BUILD_DIR}/{BINARY}: link {' '.join(str(o) for o in obj_out)}\n"
            f"\n"
            f"default {BUILD_DIR}/{BINARY} {' '.join(map(str, shader_out))}\n"
        )
    
def write_compile_flags():
    flags = CFLAGS if isinstance(CFLAGS, list) else CFLAGS.split()
    with open("compile_flags.txt", "w") as f:
        f.write("\n".join(flags))


if __name__ == "__main__":
    cmd = sys.argv[1] if len(sys.argv) > 1 else "build"

    if not os.path.exists(BUILD_DIR):
        os.makedirs(BUILD_DIR)
    if not os.path.exists(OBJ_OUT):
        os.makedirs(OBJ_OUT)
    if not os.path.exists(SHADER_OUT):
        os.makedirs(SHADER_OUT)

    match cmd:
        case "release":
            print("not implemented")
        case "debug":
            CFLAGS += [ "-g", "-O0", "-Wall", "-Wextra", "-DDEBUG" ] 
            write_ninja()
            write_compile_flags()
            subprocess.run(["ninja", "-j4"])
            subprocess.run(["./build/Tux-Ultimate"])
        case "build":
            CFLAGS += [ "-g", "-O0", "-Wall", "-Wextra", "-DDEBUG" ] 
            write_ninja()
            write_compile_flags()
        case "clean":
            if BUILD_DIR.exists():
                shutil.rmtree(BUILD_DIR)
            if Path("compile_flags.txt").exists():
                Path("compile_flags.txt").unlink()
            if Path("build.ninja").exists():
                Path("build.ninja").unlink()
        case "vendor":
            print("not implemented")
