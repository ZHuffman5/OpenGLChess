#[path ="chess_core.rs"] pub mod chess_core;

// An enum containing all piece types for squares
#[derive(Clone, Copy)]
pub enum Piece_Type
{
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
    Empty,
}

// The piece object
// derive(Clone, Copy) automatically implements the clone and copy functions for this struct (a unique Rust implementation detail for objects)
#[derive(Clone, Copy)]
pub struct Piece
{
    pub piece_type: Piece_Type,
    pub color: chess_core::Player,
    pub has_moved: bool,
}

// The methods for the Piece object
impl Piece
{
    // The constructor
    pub fn new(piece_type: Piece_Type, color: chess_core::Player) -> Self
    {
        return Piece { piece_type, color, has_moved: false };
    }

    // Generating moves based on the piece type
    pub fn generate_moves()
    {
        todo!();
    }
}

fn pawn_moves()
{
    todo!();
}

fn bishop_moves()
{
    todo!();
}

fn knight_moves()
{
    todo!();
}

fn rook_moves()
{
    todo!();
}

fn queen_moves()
{
    todo!();
}

fn king_moves()
{
    todo!();
}

