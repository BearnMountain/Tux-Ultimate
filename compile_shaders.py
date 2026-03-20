#!/opt/homebrew/bin/python3

import os
import subprocess

SHADERS_SRC = "src/shaders"
SHADERS_OUT = "build/debug/shaders"

SHADER_FORMAT = ".glsl"

def compile_shaders():
    if not os.path.exists(SHADERS_OUT):
        os.makedirs(SHADERS_OUT)

    for file in os.listdir(SHADERS_SRC):
        if file.endswith(".vert" + SHADER_FORMAT) or file.endswith(".frag" + SHADER_FORMAT):
            input_path = os.path.join(SHADERS_SRC, file)
            output_path = os.path.join(SHADERS_OUT, file + ".spv")
            
            print(f"Compiling {file}...")
            # Using glslangValidator or dxc
            subprocess.run(["glslangValidator", "-V", input_path, "-o", output_path])

if __name__ == "__main__":
    compile_shaders()
