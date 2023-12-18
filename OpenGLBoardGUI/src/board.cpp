#include <cstddef>
#include <iostream>
#include <glad/glad.h>
#include <GLFW/glfw3.h>
#include "../shader.hpp"

#define INIT_SIZE 800 // Init window width + height
#define TILE_SIZE 100 // INIT_SIZE / 8

// Background color if board doesnt render
#define RGB_MAX 255.0f
#define RED ( 51 / RGB_MAX)
#define GREEN ( 77 / RGB_MAX)
#define BLUE ( 77 / RGB_MAX)

int pieces[64] = { // Default Chess Board Representation
     4,  2,  3,  5,  6,  3,  2,  4,     // Pawn   = 1
     1,  1,  1,  1,  1,  1,  1,  1,     // Rook   = 4
     0,  0,  0,  0,  0,  0,  0,  0,     // Knight = 2
     0,  0,  0,  0,  0,  0,  0,  0,     // Bishop = 3
     0,  0,  0,  0, -1,  0,  0,  0,     // King   = 5
     0,  0,  0,  0,  0,  0,  0,  0,     // Queen  = 6
    -1, -1, -1, -1, -1, -1, -1, -1,     //              Black > 0
    -4, -2, -3, -5, -6, -3, -2, -4      //              White < 0
};
double board_x;         // Clicked square x-coord
double board_y;         // Clicked square y-coord
double selc_piece_type; // Int between [-6,6]
int    isPieceSelected; // True or False

// Callback function for when mouse clicks square
void mouseButtonCallback(GLFWwindow *window, int button, int action, int mods) {
    if (button == GLFW_MOUSE_BUTTON_LEFT && action == GLFW_PRESS) {
        glfwGetCursorPos(window, &board_x, &board_y);
        int x = board_x / TILE_SIZE; // Make coords to 0-7 
        int y = (board_y / TILE_SIZE); // Make coords 0-7 and 0 starts in top left
        int i = (8*y)+x; // Convert x,y to 0-63 => '8*|y-7| + x'
        selc_piece_type = pieces[i];

        if (selc_piece_type != 0 && isPieceSelected != true) { // Start piece moving
           isPieceSelected = true;
        } else if (isPieceSelected == true) { // After Selecting piece

            // ------------ Piece Validation and Move piece ------------
            // (Needs to be added with chess logic program from Daniel)

        } else if (selc_piece_type == 0) { // Unselect piece
            isPieceSelected = false;
        }
    }
}

// Close window with "esc" key
void checkWindowClose(GLFWwindow *window) {
    if (glfwGetKey(window, GLFW_KEY_ESCAPE) == GLFW_PRESS) {
        glfwSetWindowShouldClose(window, true);
    }
}

int main() {
    // Init Window
    // -----------
    glfwInit();
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE);

    GLFWwindow *window = glfwCreateWindow(INIT_SIZE, INIT_SIZE, "OpenGL Window", NULL, NULL); 
    if (window == NULL) {
        std::cout << "error creating window..." << std::endl;
        glfwTerminate();
        return -1;
    }
    glfwMakeContextCurrent(window);
    glfwSetMouseButtonCallback(window, mouseButtonCallback);

    if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)) {
        std::cout << "failed to init glad..." << std::endl;
        return -1;
    }

    // Load shader files, see "shader.hpp" constructor 
    Shader shader("src/board.vs", "src/board.fs"); 

    // Vertex and Buffer Data
    // ----------------------
    float vertices[] = {
         1.0f,  1.0f, 0.0f,  // top right
         1.0f, -1.0f, 0.0f,  // bottom right
        -1.0f, -1.0f, 0.0f,  // bottom left
        -1.0f,  1.0f, 0.0f   // top left 
    };
    unsigned int indices[] = { 
        0, 1, 3,  // first Triangle
        1, 2, 3   // second Triangle
    };
    
    // Buffer Objects: VertexBufferObject, VertexArrayObject, ElementBufferObject
    unsigned int VBO, VAO, EBO;
    glGenVertexArrays(1, &VAO);
    glGenBuffers(1, &VBO);
    glGenBuffers(1, &EBO);

    glBindVertexArray(VAO);

    glBindBuffer(GL_ARRAY_BUFFER, VBO);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW); // Load Vertex Data
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, EBO);
    glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(indices), indices, GL_STATIC_DRAW); 

    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), (void *)0);
    glEnableVertexAttribArray(0);

    // unbinding / cleaning up
    glBindVertexArray(0);
    glBindBuffer(GL_ARRAY_BUFFER, 0);
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);

    // Main Program Loop
    // -----------------
    while (!glfwWindowShouldClose(window)) {
        
        checkWindowClose(window);

        shader.use();
        glBindVertexArray(VAO);

        glUniform1f(glGetUniformLocation(shader.ID, "selc_x"), (float)board_x); // x,y and type get defined in mouseClickCallback
        glUniform1f(glGetUniformLocation(shader.ID, "selc_y"), (float)board_y); 
        glUniform1f(glGetUniformLocation(shader.ID, "piece_type"), (float)selc_piece_type);

        glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0); // Draw Board Data Across Screen

        glfwSwapBuffers(window); // GPU thing, too technical to explain in comment
        glfwPollEvents();
    }

    glfwTerminate();
}
