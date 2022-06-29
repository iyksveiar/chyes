mod chess;
use chess::*;

// Macro to expand coord!(x, y) to Coordinate { row: x, col: y }
macro_rules! coord {
    ($x:expr, $y:expr) => {
        Coordinate { row: $x, col: $y }
    };
}

// Macro to expand piece!(piece, color) to Piece { breed: Pieces::piece, color: Color::color }
macro_rules! piece {
    ($piece:ident, $color:ident) => {
        Piece {
            breed: Pieces::$piece,
            color: Color::$color,
        }
    };
}

fn main() {}

#[cfg(test)]
#[path = "./tests.rs"]
mod tests;
