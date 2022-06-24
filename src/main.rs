mod chess;

/*
fn main() {
  let mut board = chess::Board::new();
  let queen = chess::Piece {
    breed: chess::Pieces::Queen,
    color: chess::Color::White,
  };
  let black_pawn = chess::Piece {
    breed: chess::Pieces::Pawn,
    color: chess::Color::Black,
  };

  board.place_piece(queen, chess::Coordinate::new(3, 3));
  let moves = board.get_moves(chess::Coordinate::new(3, 3)); 

  for move_ in moves {
    // Place a black pawn on each move coordinate
    board.place_piece(black_pawn, move_);
  }

  board.draw();
}
*/

fn main() {
  let mut board: chess::Board = chess::Board::new();
  // board.place_piece(piece!(Queen, Black), coord!(0, 0));
  board.place_piece(chess::Piece {
    breed: chess::Pieces::Queen,
    color: chess::Color::Black,
  }, chess::Coordinate { row: 0, col: 0 });
  /*
  board.place_piece(chess::Piece {
    breed: chess::Pieces::King,
    color: chess::Color::White,
  }, chess::Coordinate { row: 3, col: 3 });
  */

  let black_pawn = chess::Piece {
      breed: chess::Pieces::Pawn,
      color: chess::Color::Black,
  };

  let moves = board.get_moves(chess::Coordinate { row: 0, col: 0 });
  /*
  for move_ in moves {
    // Place a black pawn on each move coordinate
    board.place_piece(black_pawn, move_);
  }
  */

  board.draw();
  println!("{}", moves.len());
} 

#[cfg(test)]
#[path = "./tests.rs"]
mod tests;