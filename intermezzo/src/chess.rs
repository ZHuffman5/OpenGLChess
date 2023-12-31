// the Chess_Board implementation is largely inspired from Sebastian Lague's excellent Coding Adventure video on Chess
// though there are large differences in implementation
// https://www.youtube.com/watch?v=U4ogK0MIzqk
// https://github.com/SebLague/Chess-Coding-Adventure

// Configure Rust tooling settings for excluding certain warnings and errors

#![allow(dead_code)]
#![allow(unused)]
#![allow(non_camel_case_types)]

// The Move struct which stores data related to a move made on the board
pub struct Move
{
    piece:     i8,
    origin:    usize,
    target:    usize,
    capture:   i8,
}

// Board details that contains miscellaneous data that we would may want to use
// rooks_have_moved and kings_have_moved are useful for determining whether castling
// is a legal move
pub struct Board_Details
{
    // rook index order: [0, 7, 56, 63]
    rooks_have_moved: [bool; 4],
    kings_have_moved: [bool; 2],
    // is_check < 0 -> white in check
    // is_check > 0 -> black in check
    is_check: i8,
}

pub struct Chess_Board
{
    // The entire chess board is represented as a single 1d array of 64 integers
    // Colors are represented by the sign of a number (positive or negative)
    // The actual value of the number represents what piece it is
    pub board:       [i8; 64],
    
    // edge_counts is a 2d array useful within move generation functions
    // it is initialized within the generate_distance function
    // The idea originates from the Sebastian Lague video referenced at the beginning of the file
    pub edge_counts: [[usize; 8]; 64],
    
    // A vector (resizable array) of Moves
    pub moves:       Vec<Move>,
    pub details:     Board_Details,
}

// The impl block is used in Rust to implement methods for an object (struct)
// The following functions are all methods belonging to the Chess_Board struct
impl Chess_Board
{
    // Get row based on an index within the board array
    fn get_row(index: usize) -> usize
    {
        return index / 8;
    }

    // Get column based on an index within the board array
    fn get_col(index: usize) -> usize
    {
        return index % 8;
    }

    // Checks if two pieces have matching colors by checking if the two numbers have the same sign
    fn match_color(origin_piece: i8, target_piece: i8) -> bool
    {
        return origin_piece * target_piece > 0;
    }

    // Prints the board into a visual form that can be easily read by a human (ONLY FOR DEBUG PURPOSES)
    // Example of the result of the print_board function:
    // + --- + --- + --- + --- + --- + --- + --- + --- +
    // |  R  |  N  |  B  |  Q  |  K  |  B  |  N  |  R  |
    // + --- + --- + --- + --- + --- + --- + --- + --- +
    // |  P  |  P  |  P  |  P  |  P  |  P  |  P  |  P  |
    // + --- + --- + --- + --- + --- + --- + --- + --- +
    // |     |     |     |     |     |     |     |     |
    // + --- + --- + --- + --- + --- + --- + --- + --- +
    // |     |     |     |     |     |     |     |     |
    // + --- + --- + --- + --- + --- + --- + --- + --- +
    // |     |     |     |     |     |     |     |     |
    // + --- + --- + --- + --- + --- + --- + --- + --- +
    // |     |     |     |     |     |     |     |     |
    // + --- + --- + --- + --- + --- + --- + --- + --- +
    // |  p  |  p  |  p  |  p  |  p  |  p  |  p  |  p  |
    // + --- + --- + --- + --- + --- + --- + --- + --- +
    // |  r  |  n  |  b  |  q  |  k  |  b  |  n  |  r  |
    // + --- + --- + --- + --- + --- + --- + --- + --- +
    pub fn print_board(&self)
    {
        print!("{}{}{}", "+ --- + --- + --- + --- + --- + --- + --- + --- +", "\n", "|  ");

        for (index, square) in self.board.iter().enumerate()
        {
            match square
            {
                -4 => print!("r"),
                -2 => print!("n"),
                -3 => print!("b"),
                -5 => print!("q"),
                -6 => print!("k"),
                -1 => print!("p"),
                 0 => print!(" "),
                 4 => print!("R"),
                 2 => print!("N"),
                 3 => print!("B"),
                 5 => print!("Q"),
                 6 => print!("K"),
                 1 => print!("P"),
                 _ => (),
            }
            
            print!("  |  ");
            
            if (index + 1) % 8 == 0
            {
                print!("{}{}{}", "\n", "+ --- + --- + --- + --- + --- + --- + --- + --- +", "\n");
                
                if (index != 63)
                {
                    print!("|  ");
                }
            }
        } // for (index, square)
    } // fn print_board

    // Moves the piece to the indicated index of the board and appends a Move to the vector of Moves
    pub fn move_piece(&mut self, origin: usize, target: usize)
    {
        let origin_piece = self.board[origin];
        self.board[target] = self.board[origin];
        self.board[origin] = 0;
        
        self.moves.push(
            Move
            {
                piece: self.board[target],
                origin,
                target,
                capture: origin_piece,
            }
        );
    }

    // The generate_distance function only needs to be run once on initialization
    // It calculates the number of squares to the edge of the board for every
    // square on the board in all eight direction and stores it in the edge_counts
    // array (a 2d array of 8x64, 8 directions for 64 squares)
    //
    // These distance values are incredibly useful for move generation functions because
    // they provide a value that can be used in for loops and ensure that index values will
    // not be out of bounds
    pub fn generate_distance(&mut self)
    {
        for col in 0..8
        {
            for row in 0..8
            {
                let count_top    = row;
                let count_bottom = 7 - row;
                let count_left   = col;
                let count_right  = 7 - col;
                
                let square_index = col + (row * 8);
                
                self.edge_counts[square_index] = [
                    count_top,
                    count_bottom,
                    count_left,
                    count_right,
                    std::cmp::min(count_top,    count_left),  // top-left     diagonal
                    std::cmp::min(count_top,    count_right), // top-right    diagonal
                    std::cmp::min(count_bottom, count_right), // bottom-right diagonal
                    std::cmp::min(count_bottom, count_left),  // bottom-left  diagonal
                ];
            }
        }
    }

    // Generates all legal moves for pawns
    pub fn pawn_moves(&self, color: i8, square: usize) -> Vec<usize>
    {
        let start_row:     usize = if color < 0 {  6 } else { 1 };
        let offset:        i8    = if color < 0 { -8 } else { 8 };
        let promotion_row: usize = if color < 0 {  0 } else { 7 };
        let mut can_promote = false;

        let directions: [i8; 2] = if color < 0 { [ -9, -7 ] } else { [ 9, 7 ] };
        let edge_count_index    = if color < 0 { [  4,  5 ] } else { [ 6, 7 ] };

        // The index offsets for en passant (values depends on the color)
        let ep_directions: [i8; 2] = if color < 0 { [ -9, -7 ] } else { [ 7,  9 ] };
        let ep_neighbors:  [i8; 2] = if color < 0 { [ -1,  1 ] } else { [ 1, -1 ] };
        let ep_edge_count_index    = [2, 3];

        let mut results: Vec<usize> = vec![];

        // The index of the square in front of the pawn piece passed into the function
        let square_in_front = (square as i8 + offset) as usize;

        // Moving forward one square
        if self.board[square_in_front] == 0
        {
            // Pawn promotions
            if (Self::get_row(square_in_front) == promotion_row)
            {
                results.push(200 + square_in_front);
                results.push(300 + square_in_front);
                results.push(400 + square_in_front);
                results.push(500 + square_in_front);
            } else
            {
                results.push(square_in_front);

                // Pawn pushing forward 2 squares
                if    Self::get_row(square) == start_row
                   && self.board[(square_in_front as i8 + offset) as usize] == 0
                {
                    results.push((square_in_front as i8 + offset) as usize);
                }
            }
        }

        // Generating moves for diagonal pawn captures
        // NOTE: in rust for i_idx in 0..2 is the same as for (int i = 0; i < 2; i++)
        for i_idx in 0..2
        {
            if self.edge_counts[square][edge_count_index[i_idx]] > 0
            {
                let capture_square = (square as i8 + directions[i_idx]) as usize;

                if (Self::get_row(capture_square) == promotion_row)
                {
                    results.push(200 + capture_square);
                    results.push(300 + capture_square);
                    results.push(400 + capture_square);
                    results.push(500 + capture_square);
                } else
                {
                    if    self.board[capture_square] != 0
                       && !Self::match_color(color, self.board[capture_square])
                    {
                        results.push(capture_square);
                    }
                }
            }
        }

        // En Passant in chess
        if self.moves.len() >= 1
        {
            let last_move: &Move = &self.moves[self.moves.len() - 1];

            for i_idx in 0..2
            {
                if self.edge_counts[square][ep_edge_count_index[i_idx]] > 0
                {
                    let ep_square = (square as i8 + ep_directions[i_idx]) as usize;
                    let neighbor_square = (square as i8 + ep_neighbors[i_idx]) as usize;

                    if    last_move.piece.abs() == 1
                       && !Self::match_color(last_move.piece, color)
                       && last_move.target == neighbor_square
                       && i8::abs(last_move.origin as i8 - last_move.target as i8) == 16
                    {
                        results.push(ep_square);
                    }
                }
            }
        }

        return results;
    }

    // A special function that is almost exactly the same as pawn_moves except it only
    // returns the capture squares a pawn can make
    //
    // This is only useful for determining what squares a pawn is attacking and for implementing
    // check and checkmate
    pub fn pawn_captures(&self, color: i8, square: usize) -> Vec<usize>
    {
        let mut results: Vec<usize> = vec![];
        
        let directions: [i8; 2] = if color < 0 { [ -9, -7 ] } else { [ 9, 7 ] };
        let edge_count_index    = if color < 0 { [  4,  5 ] } else { [ 6, 7 ] };

        let ep_directions: [i8; 2] = if color < 0 { [ -9, -7 ] } else { [ 7,  9 ] };
        let ep_neighbors:  [i8; 2] = if color < 0 { [ -1,  1 ] } else { [ 1, -1 ] };
        let ep_edge_count_index    = [2, 3];

        let promotion_row: usize = if color < 0 {  0 } else { 7 };
        let offset:        i8    = if color < 0 { -8 } else { 8 };
        let square_in_front = (square as i8 + offset) as usize;

        for i_idx in 0..2
        {
            if self.edge_counts[square][edge_count_index[i_idx]] > 0
            {
                let capture_square = (square as i8 + directions[i_idx]) as usize;

                if (Self::get_row(capture_square) == promotion_row)
                {
                    results.push(200 + capture_square);
                    results.push(300 + capture_square);
                    results.push(400 + capture_square);
                    results.push(500 + capture_square);
                } else
                {
                    if    self.board[capture_square] != 0
                       && !Self::match_color(color, self.board[capture_square])
                    {
                        results.push(capture_square);
                    }
                }
            }
        }
        
        if self.moves.len() >= 1
        {
            let last_move: &Move = &self.moves[self.moves.len() - 1];

            for i_idx in 0..2
            {
                if self.edge_counts[square][ep_edge_count_index[i_idx]] > 0
                {
                    let ep_square = (square as i8 + ep_directions[i_idx]) as usize;
                    let neighbor_square = (square as i8 + ep_neighbors[i_idx]) as usize;

                    if    last_move.piece.abs() == 1
                       && !Self::match_color(last_move.piece, color)
                       && last_move.target == neighbor_square
                       && i8::abs(last_move.origin as i8 - last_move.target as i8) == 16
                    {
                        results.push(ep_square);
                    }
                }
            }
        }

        if self.board[square_in_front] == 0
        {
            if (Self::get_row(square_in_front) == promotion_row)
            {
                results.push(200 + square_in_front);
                results.push(300 + square_in_front);
                results.push(400 + square_in_front);
                results.push(500 + square_in_front);
            }
        }
        
        return results;
    }

    // Generating legal knight moves
    pub fn knight_moves(&self, color: i8, square: usize) -> Vec<usize>
    {
        let mut results: Vec<usize> = vec![];
        let directions = [-17, -15, -10, -6, 6, 10, 15, 17];
        let bound_checks = [
            // The knight's L shaped move pattern always an offset of 2 squares
            // in one direction and an offset of 1 in another direction
            // [2 squares direction, 1 square direction]
            // uses the indexes of edge_counts[square]
            [0, 2],
            [0, 3],
            [2, 0],
            [3, 0],
            [2, 1],
            [3, 1],
            [1, 2],
            [1, 3],
        ];
        
        for d_idx in 0..directions.len()
        {
            if     self.edge_counts[square][bound_checks[d_idx][0]] < 2
                || self.edge_counts[square][bound_checks[d_idx][1]] < 1
            {
                continue;
            }

            let destination = (square as i8 + directions[d_idx]) as usize;
            
            if !Self::match_color(color, self.board[destination])
            {
                results.push(destination);
            }
        }

        return results;
    }

    // Generating legal bishop moves
    pub fn bishop_moves(&self, color: i8, square: usize) -> Vec<usize>
    {
        let mut results: Vec<usize> = vec![];
        let directions = [-7, -9, 9, 7];

        for d_idx in 0..directions.len()
        {
            for s_idx in 0..self.edge_counts[square][d_idx + 4]
            {
                let destination = (square as i8 + directions[d_idx] * (s_idx as i8 + 1)) as usize;
                let target_piece = self.board[destination];

                // If the square is empty, add index to the legal moves, continue the loop
                if target_piece == 0
                {
                    results.push(destination);
                    continue;
                }

                // If the piece encountered is the same color as the bishop, don't add to moves and break from loop
                if Self::match_color(color, target_piece)
                {
                    break;
                }

                results.push(destination);

                if !Self::match_color(color, target_piece)
                {
                    break;
                }
            }
        }

        return results;
    }

    // Generate rook moves
    // Identical to the bishop_moves function but with different directions
    pub fn rook_moves(&self, color: i8, square: usize) -> Vec<usize>
    {
        let mut results: Vec<usize> = vec![];
        let directions = [-8, 8, -1, 1];

        for d_idx in 0..directions.len()
        {
            for s_idx in 0..self.edge_counts[square][d_idx]
            {
                let destination = (square as i8 + directions[d_idx] * (s_idx as i8 + 1)) as usize;
                let target_piece = self.board[destination];

                if target_piece == 0
                {
                    results.push(destination);
                    continue;
                }

                if Self::match_color(color, target_piece)
                {
                    break;
                }

                results.push(destination);

                if !Self::match_color(color, target_piece)
                {
                    break;
                }
            }
        }

        return results;
    }

    // Generating queen_moves can be done by combining the results for a rook and bishop on that square
    pub fn queen_moves(&self, color: i8, square: usize) -> Vec<usize>
    {
        let mut results: Vec<usize> = vec![];
        
        let diag = self.bishop_moves(color, square);
        let orthogonal = self.rook_moves(color, square);

        results.extend(diag);
        results.extend(orthogonal);

        return results;
    }

    // Set a custom position on the board
    pub fn configure_board(&mut self, board: [i8; 64])
    {
        self.board = board;
    }

    // Generate king moves
    pub fn king_moves(&self, color: i8, square: usize) -> Vec<usize>
    {
        let mut results: Vec<usize> = vec![];
        
        let king_detail_index = if color < 0 { 0 } else { 1 };
        let rook_detail_index = if color < 0 { [ 2, 3 ] } else { [ 0, 1 ] };
        let rook_squares = if color < 0 { [ 56, 63 ] } else { [ 0, 7 ] };
        let king_squares = if color < 0 { 60 } else { 4 };

        // amounts are in the order of the directions stored in edge_counts
        let directions = [8, -8, -1, 1, -9, -7, 9, 7];
        
        for i_idx in 0..8
        {
            if self.edge_counts[square][i_idx] > 0
            {
                let new_square = (square as i8 + directions[i_idx]) as usize;
                
                if !Self::match_color(self.board[square], self.board[new_square])
                {
                    results.push(new_square);
                }
            }
        }

        // Castling
        'castle: for i_idx in 0..2
        {
            if    self.details.rooks_have_moved[rook_detail_index[i_idx]] == false
               && self.details.kings_have_moved[king_detail_index] == false
            {
                if    self.board[rook_squares[i_idx]].abs() == 4
                   && self.board[king_squares].abs() == 6
                {
                    if rook_squares[i_idx] < king_squares
                    {
                        for s_idx in (rook_squares[i_idx] + 1)..=(king_squares - 1)
                        {
                            if self.board[s_idx] != 0
                            {
                                continue 'castle;
                            }
                        }
                    } else
                    {
                        for s_idx in (king_squares + 1)..=(rook_squares[i_idx] - 1)
                        {
                            if self.board[s_idx] != 0
                            {
                                continue 'castle;
                            }
                        }
                    }
                }

                if i_idx == 0
                {
                    // Short castle
                    results.push(101);
                } else
                {
                    // Long castle
                    results.push(1001);
                }
            }
        }
        
        return results;
    }

    // A function that returns every square that the opponent pieces are attacking
    // Very useful for when check will be implemented
    fn check_opponent_attacks(&mut self, color: i8) -> Vec<usize>
    {
        let mut results: Vec<usize> = vec![];

        for i_idx in 0..64
        {
            let current_piece = self.board[i_idx];
            
            if    current_piece != 0
               && !Self::match_color(color, current_piece)
            {
                match current_piece {
                    4 | -4 => results.extend(self.rook_moves    (color, i_idx)),
                    2 | -2 => results.extend(self.knight_moves  (color, i_idx)),
                    3 | -3 => results.extend(self.bishop_moves  (color, i_idx)),
                    5 | -5 => results.extend(self.queen_moves   (color, i_idx)),
                    1 | -1 => results.extend(self.pawn_captures (color, i_idx)),
                    _ => (),
                }
            }
        }
        
        return results;
    }
} // impl Chess_Board

// Setting up the default values for the Chess_Board struct
impl Default for Chess_Board
{
    fn default() -> Self
    {
        Chess_Board
        { 
            board:
            [
                4,  2,  3,  5,  6,  3,  2,  4,
                1,  1,  0,  1,  0,  1,  1,  1,
                0,  0,  0,  0,  0,  0,  0,  0,
                0,  0,  0,  0,  0,  0,  0,  0,
                0,  0,  0,  0,  0,  0,  0,  0,
                0,  0,  0,  0,  0,  0,  0,  0,
               -1, -1, -1, -1, -1, -1, -1, -1,
               -4, -2, -3, -5, -6, -3, -2, -4,
            ],
            edge_counts: [[0; 8]; 64],
            moves: vec![],
            details: Board_Details { 
                rooks_have_moved: [false, false, false, false],
                kings_have_moved: [false, false],
                is_check: 0,
            },
        }
    }
}
