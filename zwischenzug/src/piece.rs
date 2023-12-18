#[path ="chess_core.rs"] pub mod chess_core;

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

#[derive(Clone, Copy)]
pub struct Piece
{
    pub piece_type: Piece_Type,
    pub color: chess_core::Player,
    pub has_moved: bool,
}

impl Piece
{
    pub fn new(piece_type: Piece_Type, color: chess_core::Player) -> Self
    {
        return Piece { piece_type, color, has_moved: false };
    }
    
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

