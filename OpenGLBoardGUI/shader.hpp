#ifndef SHADER_H
#define SHADER_H

#include <cstddef>
#include <glad/glad.h>

#include <string.h>
#include <fstream>
#include <sstream>
#include <iostream>
#include <string>

class Shader {
public:
    unsigned int ID;

    Shader(const char *vertexShaderPath, const char *fragmentShaderPath) {
       changeShader(vertexShaderPath, fragmentShaderPath); 
    }

    void changeShader(const char *vertexShaderPath, const char *fragmentShaderPath) {
        std::string vertexCode;
        std::string fragmentCode;
        std::ifstream vShaderFile;
        std::ifstream fShaderFile;
        vShaderFile.exceptions(std::ifstream::badbit | std::ifstream::failbit);
        fShaderFile.exceptions(std::ifstream::badbit | std::ifstream::failbit);
        try {
            vShaderFile.open(vertexShaderPath);
            fShaderFile.open(fragmentShaderPath);
            std::stringstream vShaderStream, fShaderStream;

            vShaderStream << vShaderFile.rdbuf();
            fShaderStream << fShaderFile.rdbuf();
            vShaderFile.close();
            fShaderFile.close();

            vertexCode = vShaderStream.str();
            fragmentCode = fShaderStream.str();
        } catch (std::ifstream::failure e) {
            std::cout << "ERROR::SHADER::FILE_NOT_READ: " << e.what() << std::endl;
        }
        const char *vShaderCode = vertexCode.c_str();
        const char *fShaderCode = fragmentCode.c_str();

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

    void use() {
        glUseProgram(ID);
    }

private:
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