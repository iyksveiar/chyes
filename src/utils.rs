use crate::chess;

#[allow(unused_macros)]
macro_rules! piece {
  ($piece:ident, $color:ident) => {
    chess::Piece {
      breed: Pieces::$piece,
      color: Color::$color
    }
  };
}

macro_rules! coord {
  ($x:expr, $y:expr) => {
    chess::Coordinate {
      row: $x, col: $y
    }
  };
}

pub fn transform_chess_board_to_strings(chess_board: &chess::Board) -> [String; 8] {
    /* Transform the chess board into a 2D array of chars */
    /* This is used to print the board in ncurses */

    const EMPTY_STRING: String = String::new();
    let mut board: [String; 8] = [EMPTY_STRING; 8];

    for row in 0..8 {
        for col in 0..8 {
            let piece = chess_board.get_piece(&coord!(row, col));

            board[row as usize].push(match piece {
                Some(piece) => {
                    piece.breed.to_unicode(piece.color)
                },
                None => '.'
            });
        }
    }

    return board
}
