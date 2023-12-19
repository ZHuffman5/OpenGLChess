// Constants for the 8 directions on a chess board
pub const north: Direction = Direction { d_row: -1, d_col:  0 };
pub const south: Direction = Direction { d_row:  1, d_col:  0 };
pub const east:  Direction = Direction { d_row:  0, d_col:  1 };
pub const west:  Direction = Direction { d_row:  0, d_col: -1 };
pub const northeast: Direction = Direction { d_row: -1, d_col:  1 };
pub const northwest: Direction = Direction { d_row: -1, d_col: -1 };
pub const southeast: Direction = Direction { d_row:  1, d_col:  1 };
pub const southwest: Direction = Direction { d_row:  1, d_col: -1 };

// The Player enum representing a Player
// None is useful for empty squares or for when a draw happens
#[derive(Clone, Copy)]
pub enum Player
{
    None,
    White,
    Black,
}

// Methods for the Player enum containing a method that returns the corresponding opponent of a Player
impl Player
{
    pub fn opponent(&self) -> Self
    {
        return match self
        {
            Self::None => Self::None,
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

// Square object representing a square on the chess board
pub struct Square
{
    pub row: usize,
    pub col: usize,
}

// constructor for the square
impl Square
{
    pub fn new(row: usize, col: usize) -> Self
    {
        return Square
        {
            row,
            col,
        }
    }
}

// Operator overloading for ==
// Allows us to directly compare two Square objects by simply using ==
impl PartialEq for Square
{
    fn eq(&self, other: &Self) -> bool {
        return    (self.row == other.row)
               && (self.col == other.col);
    }
    
    fn ne(&self, other: &Self) -> bool {
        return !(self == other);
    }
}

// Operator overloading for +
// Allows us to add a Direction to a Square directly with +
impl std::ops::Add<Direction> for Square
{
    type Output = Square;

    fn add(self, rhs: Direction) -> Self::Output {
        return Square
        {
            row: ((self.row as i8) + rhs.d_row) as usize,
            col: ((self.col as i8) + rhs.d_col) as usize,
        }
    }
}

// Direction object
pub struct Direction
{
    d_row: i8,
    d_col: i8,
}

// Constructor
impl Direction
{
    pub fn new(d_row: i8, d_col: i8) -> Self
    {
        return Direction { d_row, d_col };
    }
}

// Operator overloading for + for direction
// Allows us to add two direction objects together directly with +
impl std::ops::Add for Direction
{
    type Output = Direction;

    fn add(self, rhs: Self) -> Self::Output {
        return Direction
        {
            d_row: self.d_row + rhs.d_row,
            d_col: self.d_col + rhs.d_col,
        };
    }
}

// Multiply operator overload
// Allows us to multiply a Direction by a number quantity with *
// direction_object * 3
impl std::ops::Mul<i8> for Direction
{
    type Output = Direction;

    fn mul(self, rhs: i8) -> Self::Output {
        return Direction
        {
            d_row: self.d_row * rhs,
            d_col: self.d_col * rhs,
        }
    }
    
}

// Operator overloading in Rust is not bi-directional, so I need to implement it twice
impl std::ops::Mul<Direction> for i8
{
    type Output = Direction;

    fn mul(self, rhs: Direction) -> Self::Output {
        return Direction
        {
            d_row: rhs.d_row * self,
            d_col: rhs.d_col * self,
        }
    }
}

// Operator overloading in Rust is not bi-directional, so I need to implement it twice
impl std::ops::Add<Square> for Direction
{
    type Output = Square;

    fn add(self, rhs: Square) -> Self::Output {
        return Square
        {
            row: ((rhs.row as i8) + self.d_row) as usize,
            col: ((rhs.col as i8) + self.d_col) as usize,
        }
    }
}


