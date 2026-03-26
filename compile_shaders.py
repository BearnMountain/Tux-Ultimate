#!/opt/homebrew/bin/python3
import os
import subprocess
import platform
import sys

SHADERS_SRC = "src/shaders"
SHADERS_OUT = "build/debug/shaders"
SYSTEM = platform.system()
SHADER_FORMAT = ".glsl"  # Fix: was .glsl

if not os.path.exists(SHADERS_SRC):
    sys.exit(1)

if not os.path.exists(SHADERS_OUT):
    os.makedirs(SHADERS_OUT)

def compile_shaders():
    for file in os.listdir(SHADERS_SRC):
        if not file.endswith(".vert" + SHADER_FORMAT) and not file.endswith(".frag" + SHADER_FORMAT):
            continue

        src_path = os.path.join(SHADERS_SRC, file)
        out_path = os.path.join(SHADERS_OUT, file)

        subprocess.run([
            "glslc",
            src_path,
            "-o", out_path + ".spv"
        ])

        src_path = out_path + ".spv"
        print("COMPILING SHADER: " + src_path)

        # finds proper cross compilation file extension
        if SYSTEM == "Windows":
            out_path = os.path.join(out_path, ".dxil")
        elif SYSTEM == "Linux":
            sys.exit(1)
        elif SYSTEM == "Darwin":
            out_path = os.path.join(out_path, ".msl")

        # compiles shaders
        subprocess.run([
            "shadercross", 
            src_path,
            "-o", out_path
        ])

if __name__ == "__main__":
    compile_shaders()
