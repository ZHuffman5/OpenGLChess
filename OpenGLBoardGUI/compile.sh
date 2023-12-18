#!/bin/zsh
gcc src/board.cpp src/glad.c -lglfw -lGL -lX11 -lpthread -lXrandr -lXi -ldl -lstdc++ -lm -o out
./out
rm out
