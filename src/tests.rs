use crate::*;

macro_rules! coord {
  ($x:expr, $y:expr) => {
    Coordinate {
      row: $x, col: $y
    }
  };
}

macro_rules! piece {
  ($piece:ident, $color:ident) => {
    Piece {
      breed: Pieces::$piece,
      color: Color::$color
    }
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_in_check() {
    let mut board: Board = Board::new();

    // Case 1: King in the middle
    /*
      . . . . . . .
      . . . . . . .
      . . . . . . .
      . . . K . . .
      . . . . . . .
      . . . . . . .
      . . . . . . .

      false
    */

    board.place_piece(piece!(King, White), coord!(3, 3));
    assert_eq!(board.is_in_check(Color::White), false);
    board.reset();

    // Case 2
    /*
      q . . . . . .
      . . . . . . .
      . . . . . . .
      . . . K . . .
      . . . . . . .
      . . . . . . .
      . . . . . . .

      true
    */

    board.place_piece(piece!(Queen, Black), coord!(0, 0));
    board.place_piece(piece!(King, White), coord!(3, 3));
    assert_eq!(board.is_in_check(Color::White), true);
    board.reset();

    // Case 3
    /*
      Q . . . . . .
      . . . . . . .
      . . . . . . .
      . . . K . . .
      . . . . . . .
      . . . . . . .
      . . . . . . .

      false
    */

    board.place_piece(piece!(Queen, White), coord!(0, 0));
    board.place_piece(piece!(King, White), coord!(3, 3));
    assert_eq!(board.is_in_check(Color::White), false);
    board.reset();

    // Case 4
    /*
      . . . r . . .
      . . . . . . .
      . . . . . . .
      . . . K . . .
      . . . . . . .
      . . . . . . .
      . . . . . . .

      true
    */

    board.place_piece(piece!(King, White), coord!(3, 3));
    board.place_piece(piece!(Rook, Black), coord!(0, 3));
    assert_eq!(board.is_in_check(Color::White), true);
    board.reset();

    // Case 5
    /*
      . . . . . . .
      . . . . . . .
      . . . . . . .
      . . . K . . .
      . . . . . . .
      . . . . k . .
      . . . . . . .

      true
    */

    board.place_piece(piece!(King, White), coord!(3, 3));
    board.place_piece(piece!(Knight, Black), coord!(5, 4));
    assert_eq!(board.is_in_check(Color::White), true);
    board.reset();

    // Case 6
    /*
      . . . . . . .
      . . . . . . .
      . . . . p . .
      . . . K . . .
      . . . . . . .
      . . . . . . .
      . . . . . . .

      true
    */

    board.place_piece(piece!(King, White), coord!(3, 3));
    board.place_piece(piece!(Pawn, Black), coord!(2, 4));
    assert_eq!(board.is_in_check(Color::White), true);
    board.reset();

    // Case 7
    /*
      . . r . r . .
      . . . . . . .
      . . . . . . r
      . . . K . . .
      . . . . . . r
      . . . . . . .
      . . . . . . .

      false
    */

    board.place_piece(piece!(King, White), coord!(3, 3));
    board.place_piece(piece!(Rook, Black), coord!(0, 2));
    board.place_piece(piece!(Rook, Black), coord!(0, 4));
    board.place_piece(piece!(Rook, Black), coord!(2, 6));
    board.place_piece(piece!(Rook, Black), coord!(4, 6));
    assert_eq!(board.is_in_check(Color::White), false);
    board.reset();

    // Case 8
    /*
      . . . . .
      . . . . .
      . . K . .
      . . . . .
      . . . . b

      true
    */

    board.place_piece(piece!(King, White), coord!(2, 2));
    board.place_piece(piece!(Bishop, Black), coord!(4, 4));
    assert_eq!(board.is_in_check(Color::White), true);
    board.reset();

    // Case 9
    /*
      . . . . q
      . . . P .
      . . K . .
      . . . . .
      . . . . .

      false
    */

    board.place_piece(piece!(King, White), coord!(3, 3));
    board.place_piece(piece!(Pawn, White), coord!(2, 4));
    board.place_piece(piece!(Queen, Black), coord!(1, 5));
    assert_eq!(board.is_in_check(Color::White), false);
    board.reset();

    // Case 10
    /*
      . . . . .
      . . . . .
      . . K . r
      . . . . .
      . . . . .

      true
    */

    board.place_piece(piece!(King, White), coord!(2, 2));
    board.place_piece(piece!(Rook, Black), coord!(2, 4));
    assert_eq!(board.is_in_check(Color::White), true);
    board.reset();

    // Case 11
    let board = Board::from_fen("4r3/8/8/4K2r/8/8/8/8 w - - 0 1").expect("Failed to load FEN");
    assert_eq!(board.is_in_check(Color::White), true);
  }

  #[test]
  fn coordinates_to_notation() {
    assert_eq!(coord!(1, 1).to_notation(), "b7");
    assert_eq!(coord!(3, 4).to_notation(), "e5");
    assert_eq!(coord!(5, 7).to_notation(), "h3");
    assert_eq!(coord!(7, 7).to_notation(), "h1");
  }

  #[test]
  fn coordinates_from_str() {
    assert_eq!(coord!(1, 1), Coordinate::from_str("b7").unwrap());
    assert_eq!(coord!(3, 4), Coordinate::from_str("e5").unwrap());
    assert_eq!(coord!(5, 7), Coordinate::from_str("h3").unwrap());
    assert_eq!(coord!(7, 7), Coordinate::from_str("h1").unwrap());
  }

  #[test]
  fn king_moves() {
    let mut board = Board::new();

    // Case 1: King in the top left
    /*
      K * .
      * * .
      . . .

      3 valid moves
    */

    board.place_piece(piece!(King, White), coord!(0, 0));
    assert_eq!(board.generate_moves(coord!(0, 0)).len(), 3);
    board.reset();

    // Case 2: King in the top right
    /*
      . * K
      . * *
      . . .

      3 valid moves
    */

    board.place_piece(piece!(King, White), coord!(0, 7));
    assert_eq!(board.generate_moves(coord!(0, 7)).len(), 3);
    board.reset();

    // Case 3: King in the bottom left
    /*
      . . .
      * * .
      K * .

      3 valid moves
    */

    board.place_piece(piece!(King, White), coord!(7, 0));
    assert_eq!(board.generate_moves(coord!(7, 0)).len(), 3);
    board.reset();

    // Case 4: King in the bottom right
    /*
      . . .
      . * *
      . * K

      3 valid moves
    */

    board.place_piece(piece!(King, White), coord!(7, 7));
    assert_eq!(board.generate_moves(coord!(7, 7)).len(), 3);
    board.reset();

    // Case 5: King in the middle
    /*
      * * *
      * K *
      * * *

      8 valid moves
    */

    board.place_piece(piece!(King, White), coord!(4, 4));
    assert_eq!(board.generate_moves(coord!(4, 4)).len(), 8);
    board.reset();

    // Case 6: King in the middle with a friendly piece in the way
    /*
      * * *
      * K Q
      * * *

      7 valid moves
    */

    board.place_piece(piece!(King, White), coord!(4, 4));
    board.place_piece(piece!(Queen, White), coord!(4, 5));
    assert_eq!(board.generate_moves(coord!(4, 4)).len(), 7);
    board.reset();

    // Case 7: King in the middle with an enemy piece in the way
    /*
      * . .
      . K q
      * . .

      3 valid moves
    */

    board.place_piece(piece!(King, White), coord!(4, 4));
    board.place_piece(piece!(Queen, Black), coord!(4, 5));
    assert_eq!(board.generate_moves(coord!(4, 4)).len(), 3);
    board.reset();

    // Case 8: Rooks and King, stale mate
    /*
      . r . r .
      . . . . r
      . . K . .
      . . . . r
      . . . . .

      0 valid moves
    */

    board
      .load_fen("2r1r3/8/7r/3K4/7r/8/8/8 w - - 0 1")
      .expect("Failed to load FEN");
    assert_eq!(board.generate_moves(coord!(3, 3)).len(), 0);
    board.reset();

    // Case 9: King in the middle with an enemy piece blocking some way
    /*
      . . . .
      . r . .
      . . K *
      . . * *

      4 valid moves
    */

    board.place_piece(piece!(King, White), coord!(4, 4));
    board.place_piece(piece!(Rook, Black), coord!(3, 3));
    assert_eq!(board.generate_moves(coord!(4, 4)).len(), 4);
    board.reset();
  }

  #[test]
  fn diagonal_moves() {
    let mut board = Board::new();

    // Case 1: Top left
    /*
      B . .
      . * .
      . . *

      7 valid moves
    */

    board.place_piece(piece!(Bishop, White), coord!(0, 0));
    assert_eq!(board.generate_moves(coord!(0, 0)).len(), 7);

    // Case 2: Top right
    /*
      . . B
      . * .
      * . .

      7 valid moves
    */

    board.reset();
    board.place_piece(piece!(Bishop, White), coord!(0, 7));
    assert_eq!(board.generate_moves(coord!(0, 7)).len(), 7);

    // Case 3: Bottom left
    /*
      . . *
      . * .
      B . .

      7 valid moves
    */

    board.reset();
    board.place_piece(piece!(Bishop, White), coord!(7, 0));
    assert_eq!(board.generate_moves(coord!(7, 0)).len(), 7);

    // Case 4: Bottom right
    /*
      * . .
      . * .
      . . B

      7 valid moves
    */

    board.reset();
    board.place_piece(piece!(Bishop, White), coord!(7, 7));
    assert_eq!(board.generate_moves(coord!(7, 7)).len(), 7);

    // Case 5: Middle
    /*
      * . *
      . B .
      * . *

      13 valid moves
    */

    board.reset();
    board.place_piece(piece!(Bishop, White), coord!(3, 3));
    assert_eq!(board.generate_moves(coord!(3, 3)).len(), 13);

    // Case 6: Friendly piece on the way
    /*
      * . *
      . B .
      * . P

      3 valid moves
    */

    board.reset();
    board.place_piece(piece!(Pawn, White), coord!(2, 2));
    board.place_piece(piece!(Bishop, White), coord!(1, 1));
    assert_eq!(board.generate_moves(coord!(1, 1)).len(), 3);

    // Case 7: Enemy piece on the way
    /*
      * . * .
      . B . .
      * . p .
      . . . .

      10 valid moves
    */

    board.reset();
    board.place_piece(piece!(Pawn, Black), coord!(2, 2));
    board.place_piece(piece!(Bishop, White), coord!(1, 1));
    assert_eq!(board.generate_moves(coord!(1, 1)).len(), 4);
  }

  #[test]
  fn linear_moves() {
    let mut board = Board::new();

    // Case 1: Top left
    /*
      R * *
      * . .
      * . .

      14 valid moves
    */

    board.place_piece(piece!(Rook, White), coord!(0, 0));
    assert_eq!(board.generate_moves(coord!(0, 0)).len(), 14);
    board.reset();

    // Case 2: Top right
    /*
      * * R
      . . *
      . . *

      14 valid moves
    */

    board.place_piece(piece!(Rook, White), coord!(0, 7));
    assert_eq!(board.generate_moves(coord!(0, 7)).len(), 14);
    board.reset();

    // Case 3: Bottom left
    /*
      * . .
      * . .
      R * *

      14 valid moves
    */

    board.place_piece(piece!(Rook, White), coord!(7, 0));
    assert_eq!(board.generate_moves(coord!(7, 0)).len(), 14);
    board.reset();

    // Case 4: Bottom right
    /*
      . . *
      . . *
      * * R

      14 valid moves
    */

    board.place_piece(piece!(Rook, White), coord!(7, 7));
    assert_eq!(board.generate_moves(coord!(7, 7)).len(), 14);
    board.reset();

    // Case 5: Middle
    /*
      . * . .
      * R * *
      . * . .
      . * . .

      14 valid moves
    */

    board.place_piece(piece!(Rook, White), coord!(3, 3));
    assert_eq!(board.generate_moves(coord!(3, 3)).len(), 14);
    board.reset();

    // Case 6: Friendly piece on the way
    /*
      . * . .
      * R * *
      . P . .
      . . . .

      12 valid moves
    */

    board.place_piece(piece!(Rook, White), coord!(6, 5));
    board.place_piece(piece!(Pawn, White), coord!(6, 6));
    assert_eq!(board.generate_moves(coord!(6, 5)).len(), 12);

    // Case 7: Enemy piece on the way
    /*
      . * . .
      * R * *
      . p . .
      . . . .

      13 valid moves
    */

    board.place_piece(piece!(Rook, White), coord!(6, 5));
    board.place_piece(piece!(Pawn, Black), coord!(6, 6));
    assert_eq!(board.generate_moves(coord!(6, 5)).len(), 13);
  }

  #[test]
  fn knight_moves() {
    let mut board: Board = Board::new();

    // Case 1: Top left
    /*
      K . . . .
      . . * . .
      . * . . .
      . . . . .
      . . . . .

      2 valid moves
    */

    board.place_piece(piece!(Knight, White), coord!(0, 0));
    assert_eq!(board.generate_moves(coord!(0, 0)).len(), 2);
    board.reset();

    // Case 2: Top right
    /*
      . . . . K
      . . * . .
      . . . * .
      . . . . .
      . . . . .

      2 valid moves
    */

    board.place_piece(piece!(Knight, White), coord!(0, 7));
    assert_eq!(board.generate_moves(coord!(0, 7)).len(), 2);
    board.reset();

    // Case 3: Bottom left
    /*
      . . . . .
      . . . . .
      . * . . .
      . . * . .
      K . . . .

      2 valid moves
    */

    board.place_piece(piece!(Knight, White), coord!(7, 0));
    assert_eq!(board.generate_moves(coord!(7, 0)).len(), 2);
    board.reset();

    // Case 4: Bottom left
    /*
      . . . . .
      . . . . .
      . . . * .
      . . * . .
      . . . . K

      2 valid moves
    */

    board.place_piece(piece!(Knight, White), coord!(7, 7));
    assert_eq!(board.generate_moves(coord!(7, 7)).len(), 2);
    board.reset();

    // Case 5: Middle
    /*
      . * . * .
      * . . . *
      . . K . .
      * . . . *
      . * . * .

      8 valid moves
    */

    board.place_piece(piece!(Knight, White), coord!(2, 2));
    assert_eq!(board.generate_moves(coord!(2, 2)).len(), 8);
    board.reset();

    // Case 6: 1 friendly piece on a way
    /*
      . * . * .
      * . . . *
      . . K . .
      * . . . *
      . P . * .

      7 valid moves
    */

    board.place_piece(piece!(Knight, White), coord!(2, 2));
    board.place_piece(piece!(Pawn, White), coord!(4, 1));
    assert_eq!(board.generate_moves(coord!(2, 2)).len(), 7);
    board.reset();

    // Case 7: 2 friendly pieces on a way
    /*
      . * . * .
      * . . . *
      . . K . .
      * . . . P
      . P . * .

      6 valid moves
    */

    board.place_piece(piece!(Knight, White), coord!(2, 2));
    board.place_piece(piece!(Pawn, White), coord!(4, 1));
    board.place_piece(piece!(Pawn, White), coord!(3, 4));
    assert_eq!(board.generate_moves(coord!(2, 2)).len(), 6);
    board.reset();

    // Case 8: Enemy piece on the way
    /*
      . * . * .
      * . . . *
      . . K . .
      * . . . *
      . * . p .

      8 valid moves
    */

    board.place_piece(piece!(Knight, White), coord!(2, 2));
    board.place_piece(piece!(Pawn, Black), coord!(4, 3));
    assert_eq!(board.generate_moves(coord!(2, 2)).len(), 8);
    board.reset();
  }

  #[test]
  fn pawn_moves() {
    let mut board: Board = Board::new();

    // Case 1: Pawn in the middle
    /*
      . . . . .
      . . * . .
      . . P . .

      1 valid move
    */

    board.place_piece(piece!(Pawn, White), coord!(4, 3));
    assert_eq!(board.generate_moves(coord!(4, 3)).len(), 1);
    board.reset();

    // Case 2: Pawn in the middle with a friendly piece on the way
    /*
      . . . . .
      . . K . .
      . . P . .

      0 valid moves
    */

    board.place_piece(piece!(Pawn, White), coord!(4, 3));
    board.place_piece(piece!(King, White), coord!(3, 3));
    assert_eq!(board.generate_moves(coord!(4, 3)).len(), 0);
    board.reset();

    // Case 3: Pawn in the middle with an enemy piece on the way
    /*
      . . . . .
      . . p . .
      . . P . .

      0 valid moves
    */

    board.place_piece(piece!(Pawn, White), coord!(4, 3));
    board.place_piece(piece!(Pawn, Black), coord!(3, 3));
    assert_eq!(board.generate_moves(coord!(4, 3)).len(), 0);
    board.reset();

    // Case 4: Pawn in the middle with a enemy piece that can be captured
    /*
      . . . . .
      . . * p .
      . . P . .

      2 valid move
    */

    board.place_piece(piece!(Pawn, White), coord!(4, 3));
    board.place_piece(piece!(Pawn, Black), coord!(3, 4));
    assert_eq!(board.generate_moves(coord!(4, 3)).len(), 2);
    board.reset();

    // Case 5: Pawn in the middle with 2 enemy pieces that can be captured
    /*
      . . . . .
      . p * p .
      . . P . .
      . . . . .

      3 valid moves
    */

    board.place_piece(piece!(Pawn, White), coord!(4, 3));
    board.place_piece(piece!(Pawn, Black), coord!(3, 4));
    board.place_piece(piece!(Pawn, Black), coord!(3, 2));
    assert_eq!(board.generate_moves(coord!(4, 3)).len(), 3);
    board.reset();

    // Case 6: Pawn in the starting row
    // 2 valid moves

    board.place_piece(piece!(Pawn, White), coord!(6, 1));
    assert_eq!(board.generate_moves(coord!(6, 1)).len(), 2);
    board.reset();

    // Case 7: Pawn in the starting row with a friendly piece 2 steps away
    // 1 valid move

    board.place_piece(piece!(Pawn, White), coord!(6, 1));
    board.place_piece(piece!(Pawn, White), coord!(4, 1));
    assert_eq!(board.generate_moves(coord!(6, 1)).len(), 1);
    board.reset();

    // Case 8: Pawn in the starting row with an enemy piece 2 steps away
    // 1 valid moves

    board.place_piece(piece!(Pawn, White), coord!(6, 1));
    board.place_piece(piece!(Pawn, Black), coord!(4, 1));
    assert_eq!(board.generate_moves(coord!(6, 1)).len(), 1);
    board.reset();

    // Case 9: Pawn in the middle with a Pawn that made a 2 step move, en passant possible
    /*
      . . . . .
      . . * * .
      . . P p .
      . . . . .
      . . . . .

      2 valid moves
    */

    board
      .load_fen("8/8/8/2Pp4/8/8/8/8 b - d6 0 1")
      .expect("Could not load FEN");
    assert_eq!(board.generate_moves(coord!(3, 2)).len(), 2);
    board.reset();

    // Case 10: Advanced en passant
    /*
      . . . . .
      . p K * .
      . . P p .
      . . . . .
      . . . . .

      2 valid moves
    */

    board
      .load_fen("8/8/2pK4/3Pp3/8/8/8/8 w - e6 0 1")
      .expect("Failed to load FEN");
    assert_eq!(board.generate_moves(coord!(3, 3)).len(), 2);
    board.reset();
  }

  #[test]
  fn moves_that_lead_to_check() {
    let mut board = Board::new();

    // Case 1
    /*
       . . .
       K P r
       . . .

       For piece!(Pawn, White) it is 0 valid moves
    */

    board
      .load_fen("8/8/8/8/1KP3r1/8/8/8 w - - 0 1")
      .expect("Failed to load FEN");
    assert_eq!(board.generate_moves(coord!(4, 2)).len(), 0);

    // Case 2
    /*
      . . .
      K R r
      . . .

      For rook it is 1 valid move
    */

    board
      .load_fen("8/8/8/8/1KRr4/8/8/8 w - - 0 1")
      .expect("Failed to load FEN");
    assert_eq!(board.generate_moves(coord!(4, 2)).len(), 1);
  }

  #[test]
  fn is_checkmate() {
    let mut board = Board::new();

    // Case 1: Double rook mate
    board
      .load_fen("K6r/7r/8/8/8/8/8/8 w - - 0 1")
      .expect("Failed to load FEN");
    assert_eq!(board.is_in_checkmate(Color::White), true);
    board.reset();

    // Case 2: Anderssen's mate
    board
      .load_fen("6kR/6P1/5K2/8/8/8/8/8 w - - 0 1")
      .expect("Failed to load FEN");
    assert_eq!(board.is_in_checkmate(Color::Black), true);
    board.reset();

    // Case 3: Arabian mate
    board
      .load_fen("7k/7R/5N2/8/8/8/8/8 w - - 0 1")
      .expect("Failed to load FEN");
    assert_eq!(board.is_in_checkmate(Color::Black), true);
    board.reset();

    // Case 4: Backrank mate
    board
      .load_fen("3R2k1/5ppp/8/8/8/8/8/8 w - - 0 1")
      .expect("Failed to load FEN");
    assert_eq!(board.is_in_checkmate(Color::Black), true);
    board.reset();

    // Case 4: Balestra mate
    board
      .load_fen("6k1/8/4B2Q/8/8/8/8/8 w - - 0 1")
      .expect("Failed to load FEN");
    assert_eq!(board.is_in_checkmate(Color::Black), true);
    board.reset();

    // Case 5: Bishop and knight mate
    board
      .load_fen("7k/8/5BKN/8/8/8/8/8 w - - 0 1")
      .expect("Failed to load FEN");
    assert_eq!(board.is_in_checkmate(Color::Black), true);
    board.reset();

    // Case 6: Blackburne's mate
    board
      .load_fen("5rk1/7B/8/6N1/8/8/1B6/8 w - - 0 1")
      .expect("Failed to load FEN");
    assert_eq!(board.is_in_checkmate(Color::Black), true);

    // Case N: Not a mate
    board
      .load_fen("K6r/7r/6N1/8/8/8/8/8 w - - 0 1")
      .expect("Failed to load FEN");
    assert_eq!(board.is_in_checkmate(Color::White), false);
  }

  #[test]
  fn get_king_coord() {
    let mut board = Board::new();

    board.place_piece(piece!(King, White), coord!(1, 1));
    assert_eq!(board.get_king_coord(Color::White), Some(coord!(1, 1)));
    board.reset();

    board.place_piece(piece!(King, White), coord!(2, 7));
    assert_eq!(board.get_king_coord(Color::White), Some(coord!(2, 7)));
    board.reset();

    board.place_piece(piece!(King, White), coord!(3, 5));
    assert_eq!(board.get_king_coord(Color::White), Some(coord!(3, 5)));
    board.reset();

    board.place_piece(piece!(King, White), coord!(5, 2));
    assert_eq!(board.get_king_coord(Color::White), Some(coord!(5, 2)));
    board.reset();

    board.place_piece(piece!(King, White), coord!(5, 1));
    assert_eq!(board.get_king_coord(Color::White), Some(coord!(5, 1)));
    board.reset();

    board.place_piece(piece!(King, White), coord!(6, 6));
    assert_eq!(board.get_king_coord(Color::White), Some(coord!(6, 6)));
    board.reset();

    board.place_piece(piece!(King, White), coord!(6, 3));
    assert_eq!(board.get_king_coord(Color::White), Some(coord!(6, 3)));
    board.reset();
  }
}
