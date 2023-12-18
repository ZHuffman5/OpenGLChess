#version 330 core
in vec4 gl_FragCoord;
out vec4 FragColor;

// x,y still need to be normalized when passed as input
uniform float selc_x; // User Selected Piece x-coord
uniform float selc_y; // User Selected Piece y-coord (0 starts top left)
uniform float piece_type; // Type of piece user selected (represented as an int -6 -> 6)

// Predefined color matrix
mat4 green = mat4(
    0.0, 0.5, 0.0, 0.0,
    0.5, 0.0, 0.5, 0.0,
    0.0, 0.5, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0
);
mat4 red = mat4(
    0.75, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0
);
mat4 normal = mat4( // Wont change the multiplied vector
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 0.0
);

// Move highlighting logic
bool isPiece(vec2 coords, vec2 sel_square) {
    vec2 diff = vec2(abs(coords-sel_square));
    if (abs(piece_type) == 3) // Bishop
       return diff.x == diff.y; 
    else if (abs(piece_type) == 2) // Knight
        return (diff.x == 1.0 && diff.y == 2.0) || (diff.x == 2.0 && diff.y == 1.0);
    else if (abs(piece_type) == 4) // Castle
        return coords.x == sel_square.x || coords.y == sel_square.y;
    else if (abs(piece_type) == 5) // Queen
        return (coords.x == sel_square.x || coords.y == sel_square.y) || (diff.x == diff.y);
    else if (abs(piece_type) == 6) // King
        return diff.x <= 1.0 && diff.y <= 1.0;
    else if (piece_type == -1) { // White Pawn
        diff = (coords-sel_square);
        if (sel_square.y == 1.0)
            return diff.y <= 2.0 && diff.y > 0.0 && diff.x == 0.0;
        else
            return diff.y == 1.0 && diff.x == 0.0;
    } // end of white pawn 
    else if (piece_type == 1) { // Black Pawn
        diff = vec2(coords-sel_square);
        if (sel_square.y == 6.0)
            return diff.y >= -2.0 && diff.y < 0.0 && diff.x == 0.0;
        else
            return diff.y == -1.0 && diff.x == 0.0;
    } // end of black pawn
    return false;
}

void main()
{
    mat4 color_mat;
    vec2 sel_square = vec2(floor(selc_x / 100.0), abs(floor((selc_y / 100) - 7.0))); // Normalize selected coords
    vec2 coords = vec2(floor(gl_FragCoord.x / 100.0), floor(gl_FragCoord.y / 100.0)); // Normalize screen coords
    float transform = 1.0;

    color_mat = normal;
    transform = mod(coords.x+ mod(coords.y, 2.0), 2.0); // Transform = 0 or 1 if black or white square
    if (isPiece(coords, sel_square)) {
        color_mat = green;
        transform = transform == 0.0 ? 0.8 : transform; // Instead of black square make color darker
    } 
    if (coords.x == sel_square.x && coords.y == sel_square.y) { // Highlight clicked square as seperate color
        color_mat = red; 
        transform = 1.0;
    }
    color_mat = transform * color_mat;
    
    FragColor = color_mat * vec4(1.0, 1.0, 1.0, 1.0);
}