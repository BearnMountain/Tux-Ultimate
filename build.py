#!/opt/homebrew/bin/python3

import glob
import subprocess
import sys
import os
import pathlib import Path


# chose build options
# - release: creates release build
# - build: just builds all .c files
# - clean: cleans up everything
# - shaders: builds shaders into platform specific formats
# - vendor: installs all libs that are vendor for static linking
# - 
if __name__ == "__main__"
    cmd = sys.argv[1] if len(sys.argv) > 1 else "build"
