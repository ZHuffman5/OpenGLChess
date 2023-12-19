#ifndef SHADER_H
#define SHADER_H

#include <cstddef>
#include <glad/glad.h>

#include <string.h>
#include <fstream>
#include <sstream>
#include <iostream>
#include <string>

// Note:
// This file includes a custom "Shader" class which is used for untility/abstraction purposes to reduce the 
// amount of backend OpenGL code written directly into the main source file. The "backend" code happening is code
// for taking shader (glsl) source code from a file and converting it to a character array pointer to
// pass into a created OpenGL shader program. Said shader program can be refrenced with the Shader object.

class Shader {
public:
    unsigned int ID;

    Shader(const char *vertexShaderPath, const char *fragmentShaderPath) {
       changeShader(vertexShaderPath, fragmentShaderPath); // Code not directly in constructor so shader source files can be changed later in execution
    }

    void changeShader(const char *vertexShaderPath, const char *fragmentShaderPath) {
        // File Reading
        // ------------
        std::string vertexCode;
        std::string fragmentCode;
        std::ifstream vShaderFile;
        std::ifstream fShaderFile;
        vShaderFile.exceptions(std::ifstream::badbit | std::ifstream::failbit); // Set input stream flags for error grabbing
        fShaderFile.exceptions(std::ifstream::badbit | std::ifstream::failbit);
        try {
            vShaderFile.open(vertexShaderPath); // Open file
            fShaderFile.open(fragmentShaderPath);
            std::stringstream vShaderStream, fShaderStream;

            vShaderStream << vShaderFile.rdbuf(); // Read file contents into input/buffer stream
            fShaderStream << fShaderFile.rdbuf();
            vShaderFile.close(); // Clean up :)
            fShaderFile.close();

            vertexCode = vShaderStream.str(); // Convert to c++ String
            fragmentCode = fShaderStream.str();
        } catch (std::ifstream::failure e) {
            std::cout << "ERROR::SHADER::FILE_NOT_READ: " << e.what() << std::endl;
        }
        const char *vShaderCode = vertexCode.c_str(); // Convert c++ string to needed *char[] (pointer to char array)
        const char *fShaderCode = fragmentCode.c_str();

        // OpenGL Shader Loading/Creation and Compiling
        // --------------------------------------------
        unsigned int vertex, fragment;
        vertex = glCreateShader(GL_VERTEX_SHADER);
        glShaderSource(vertex, 1, &vShaderCode, NULL);
        glCompileShader(vertex); 
        checkCompile(vertex, false);

        fragment = glCreateShader(GL_FRAGMENT_SHADER);
        glShaderSource(fragment, 1, &fShaderCode, NULL);
        glCompileShader(fragment);
        checkCompile(fragment, false);

        ID = glCreateProgram();
        glAttachShader(ID, vertex);
        glAttachShader(ID, fragment);
        glLinkProgram(ID);
        checkCompile(ID, true);

        glDeleteShader(vertex);
        glDeleteShader(fragment);
    }

    // Activate Shader Program
    void use() {
        glUseProgram(ID);
    }

private:
    // Error Checking :)
    void checkCompile(unsigned int shader, bool isProgram) {
        int sucess;
        char infoLog[512];
        if (isProgram) {
            glGetProgramiv(shader, GL_LINK_STATUS, &sucess);
            if (!sucess) {
                glGetProgramInfoLog(shader, 512, NULL, infoLog);
                std::cout << "ERROR:PROGRAM_LINKING_ERROR " << infoLog << std::endl;
            }
        } else {
            glGetShaderiv(shader, GL_COMPILE_STATUS, &sucess);
            if (!sucess) {
                glGetShaderInfoLog(shader, 512, NULL, infoLog);
                std::cout << "ERROR::SHADER::COMPILATION_ERROR " << infoLog << std::endl;
            }
        }
    }
};

#endif
