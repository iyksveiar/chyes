mod chess;
mod utils;

/* TUI library */
extern crate ncurses;
use ncurses::*;

// Macro to expand coord!(x, y) to Coordinate { row: x, col: y }
#[allow(unused_macros)]
macro_rules! coord {
  ($x:expr, $y:expr) => {
    Coordinate {
      row: $x, col: $y
    }
  };
}

// Macro to expand piece!(piece, color) to Piece { breed: Pieces::piece, color: Color::color }
#[allow(unused_macros)]
macro_rules! piece {
  ($piece:ident, $color:ident) => {
    Piece {
      breed: Pieces::$piece,
      color: Color::$color
    }
  };
}

fn ncurses_draw_board(board: &chess::Board, row: i32, col: i32) {
    let mut current_row = row;
    let repr: [String; 8] = utils::transform_chess_board_to_strings(board);

    while current_row != row + 8 {
        mv(current_row, col);

        /* Printing row */
        addstr(repr[(current_row - row) as usize].as_str());

        current_row += 1;
    }
}

fn main() {
  let board = Board::default();
  board.draw();
  println!("{}", board.get_fen());
}

#[cfg(test)]
#[path = "./tests.rs"]
mod tests;
