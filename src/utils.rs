#[path="./chess.rs"]
mod chess;

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

pub fn transform_chess_board_to_strings(chess_board: &chess::Board) -> [[char; 8]; 8] {
    /* Transform the chess board into a 2D array of chars */
    /* This is used to print the board in ncurses */
    let mut board = [['.'; 8]; 8]; // '.' is the empty square

    for row in 0..8 {
      for col in 0..8 {
        let piece = chess_board.get_piece(&coord!(row, col));

        match piece {
          Some(piece) => {
            let pieces_str: &[&str; 6] = match piece.color {
              chess::Color::White => &chess::WHITE_PIECES,
              chess::Color::Black => &chess::BLACK_PIECES
            };

            use chess::Pieces::*;
            board[row as usize][col as usize] =
              match piece.breed {
                King => pieces_str[0],
                Queen => pieces_str[1],
                Rook => pieces_str[2],
                Bishop => pieces_str[3],
                Knight => pieces_str[4],
                Pawn => pieces_str[5]
              }.chars().next().unwrap(); // Get the first char of the string
          },
          None => {/* Do nothing since the board is initialized with '.' by default */}
        }
      }
    }

    return board
}
