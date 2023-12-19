mod chess;

// Testing the move generation functions and Chess_Board struct provided by the chess module
fn main()
{
    // Creates a Chess_Board object with default values
    let mut sample_board = chess::Chess_Board { ..Default::default() };
    sample_board.generate_distance();
    // Configures the board with custom values
    sample_board.configure_board(
        [
            4,  0,  3,  5,  6,  3,  2,  4,
            0,  0,  1,  0,  0,  1,  1,  1,
            0,  0,  0,  0,  1,  1, -1,  0,
            0, -1,  0,  0, -2,  0,  0,  0,
            0,  0,  0,  0,  0,  0,  0,  0,
            0,  0,  0,  0,  0,  0,  0,  0,
           -1, -1, -1, -1, -1,  1, -1,  1,
           -4, -2, -3, -5, -6,  0, -2,  0,
        ]
    );
    sample_board.print_board();

    // The following lines of code tests the move generation functions for various pieces
    // and prints the results

    // Bishops, rooks, queens, knights, and pawns at various locations on the configued board
    // are all tested
    
    let mut sample_results = sample_board.bishop_moves(sample_board.board[2], 2);
    println!("{:?}", sample_results);

    sample_results = sample_board.rook_moves(sample_board.board[0], 0);
    println!("{:?}", sample_results);

    sample_results = sample_board.queen_moves(sample_board.board[3], 3);
    println!("{:?}", sample_results);

    sample_results = sample_board.knight_moves(sample_board.board[28], 28);
    println!("{:?}", sample_results);

    sample_results = sample_board.knight_moves(sample_board.board[6], 6);
    println!("{:?}", sample_results);

    sample_results = sample_board.pawn_moves(sample_board.board[13], 13);
    println!("{:?}", sample_results);

    sample_results = sample_board.pawn_moves(sample_board.board[55], 55);
    println!("{:?}", sample_results);

    sample_results = sample_board.pawn_moves(sample_board.board[53], 53);
    println!("{:?}", sample_results);

    // Tests the move_piece function which also adds a Move object to the array of moves within the Chess_Board object
    sample_board.move_piece(10, 26);
    sample_board.print_board();
    sample_results = sample_board.pawn_moves(sample_board.board[25], 25);
    println!("{:?}", sample_results);
}
