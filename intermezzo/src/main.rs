mod chess;

fn main()
{
    let mut sample_board = chess::Chess_Board { ..Default::default() };
    sample_board.generate_distance();
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
    
    sample_board.move_piece(10, 26);
    sample_board.print_board();
    sample_results = sample_board.pawn_moves(sample_board.board[25], 25);
    println!("{:?}", sample_results);
}
