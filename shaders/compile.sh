#!/usr/bin/env bash

MY_PATH="$(dirname -- "${BASH_SOURCE[0]}")"
echo "Compiling shaders in $MY_PATH"
cd "$MY_PATH"

glslc shader.vert -o vert.spv
glslc shader.frag -o frag.spv
