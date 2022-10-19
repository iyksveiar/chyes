use crate::chess::*;

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_in_check() {
    let mut board: Board = Board::new();
    let king = piece!(King, White);

    // Case 1: King in the middle
    /*
    . . . . .
    . . . . .
    . . K . .
    . . . . .
    . . . . .

    false
    */

    board.place_piece(king, coord!(3, 3));
    assert_eq!(board.is_in_check(Color::White), false);
    board.reset();

    // Case 2
    /*
    q . . . .
    . . . . .
    . . K . .
    . . . . .
    . . . . .

    true
    */

    board.place_piece(piece!(Queen, Black), coord!(0, 0));
    board.place_piece(king, coord!(3, 3));
    assert_eq!(board.is_in_check(Color::White), true);
    board.reset();

    // Case 3
    /*
    Q . . . .
    . . . . .
    . . K . .
    . . . . .
    . . . . .

    false
    */

    board.place_piece(piece!(Queen, White), coord!(0, 0));
    board.place_piece(king, coord!(3, 3));
    assert_eq!(board.is_in_check(Color::White), false);
    board.reset();

    // Case 4
    /*
    . . r . .
    . . . . .
    . . K . .
    . . . . .
    . . . . .

    true
    */

    board.place_piece(king, coord!(3, 3));
    board.place_piece(piece!(Rook, Black), coord!(0, 3));
    assert_eq!(board.is_in_check(Color::White), true);
    board.reset();

    // Case 5
    /*
    . . . . .
    . . . . .
    . . K . .
    . . . . .
    . . . k .

    true
    */

    board.place_piece(king, coord!(3, 3));
    board.place_piece(piece!(Knight, Black), coord!(5, 4));
    assert_eq!(board.is_in_check(Color::White), true);
    board.reset();

    // Case 6
    /*
    . . . . .
    . . . p .
    . . K . .
    . . . . .
    . . . . .

    true
    */

    board.place_piece(king, coord!(3, 3));
    board.place_piece(piece!(Pawn, Black), coord!(2, 4));
    assert_eq!(board.is_in_check(Color::White), true);
    board.reset();

    // Case 7
    /*
    . r . r .
    . . . . r
    . . K . .
    . . . . r
    . . . . .

    false
    */

    board.place_piece(king, coord!(3, 3));
    board.place_piece(piece!(Rook, Black), coord!(0, 2));
    board.place_piece(piece!(Rook, Black), coord!(0, 4));
    board.place_piece(piece!(Rook, Black), coord!(2, 7));
    board.place_piece(piece!(Rook, Black), coord!(4, 4));
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

    board.place_piece(king, coord!(3, 3));
    board.place_piece(piece!(Bishop, Black), coord!(5, 5));
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

    board.place_piece(king, coord!(4, 3));
    board.place_piece(piece!(Pawn, White), coord!(3, 4));
    board.place_piece(piece!(Queen, Black), coord!(0, 7));
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

    board.place_piece(king, coord!(3, 3));
    board.place_piece(piece!(Rook, Black), coord!(3, 7));
    assert_eq!(board.is_in_check(Color::White), true);
    board.reset();

    // Case 11
    board.load_fen(String::from("4r3/8/8/4K2r/8/8/8/8 w - - 0 1"));
    assert_eq!(board.is_in_check(Color::White), true);
    board.reset();
  }

  #[test]
  fn coordinates_to_notation() {
    assert_eq!(coord!(1, 1).to_notation(), "b7".to_string());
    assert_eq!(coord!(3, 4).to_notation(), "e5".to_string());
    assert_eq!(coord!(5, 7).to_notation(), "h3".to_string());
    assert_eq!(coord!(7, 7).to_notation(), "h1".to_string());
  }

  #[test]
  fn king_moves() {
    let mut board = Board::new();
    let king = piece!(King, White);
    let mut moves: Vec<Coordinate>;

    // Case 1: King in the top left
    /*
      K *
      * * .
      . . .
    */
    board.place_piece(king, coord!(0, 0));
    moves = board.get_moves(coord!(0, 0));
    assert_eq!(moves.len(), 3);
    board.reset();

    // Second case: King in the top right
    // . * K
    // . * *
    // . . .
    // 3 valid moves

    board.place_piece(king, coord!(0, 7));
    moves = board.get_moves(coord!(0, 7));
    assert_eq!(moves.len(), 3);
    board.reset();

    // Third case: King in the bottom left
    // . . .
    // * * .
    // K * .
    // 3 valid moves

    board.place_piece(king, coord!(7, 0));
    moves = board.get_moves(coord!(7, 0));
    assert_eq!(moves.len(), 3);
    board.reset();

    // Fourth case: King in the bottom right
    // . . .
    // . * *
    // . * K
    // 3 valid moves

    board.place_piece(king, coord!(7, 7));
    moves = board.get_moves(coord!(7, 7));
    assert_eq!(moves.len(), 3);
    board.reset();

    // Fifth case: King in the middle
    // * * *
    // * K *
    // * * *
    // 8 valid moves

    board.place_piece(king, coord!(4, 4));
    moves = board.get_moves(coord!(4, 4));
    assert_eq!(moves.len(), 8);
    board.reset();

    // Sixth case: King in the middle, with a friendly piece in the way
    // * * *
    // * K Q
    // * * *
    // 7 valid moves

    board.place_piece(king, coord!(4, 4));
    board.place_piece(piece!(Queen, White), coord!(3, 4));
    moves = board.get_moves(coord!(4, 4));
    assert_eq!(moves.len(), 7);
    board.reset();

    // Seventh case: King in the middle, with an enemy piece in the way
    // * . .
    // . K q
    // * . .
    // 3 valid moves

    board.place_piece(king, coord!(4, 4));
    board.place_piece(piece!(Queen, Black), coord!(4, 5));
    moves = board.get_moves(coord!(4, 4));
    assert_eq!(moves.len(), 3);
    board.reset();

    // Eighth case: Rooks and King, stale mate
    /*
    . r . r .
    . . . . r
    . . K . .
    . . . . r
    . . . . .

    0 valid moves
    */

    board.load_fen(String::from("2r1r3/8/7r/3K4/7r/8/8/8 w - - 0 1"));
    moves = board.get_moves(coord!(3, 3));
    assert_eq!(moves.len(), 0);
    board.reset();

    // Ninth case: King in the middle, with an enemy piece blocKing Some space
    /*
    . . . .
    . r . .
    . . K *
    . . * *

    4 valid moves
    */

    board.place_piece(king, coord!(4, 4));
    board.place_piece(piece!(Rook, Black), coord!(3, 3));
    moves = board.get_moves(coord!(4, 4));
    assert_eq!(moves.len(), 4);
    board.reset();
  }

  #[test]
  fn diagonal_moves() {
    let mut board = Board::new();
    let mut moves: Vec<Coordinate>;

    // Case 1: Top left
    /*
    B . .
    . * .
    . . *

    7 valid moves
    */

    board.place_piece(piece!(Bishop, White), coord!(0, 0));
    moves = board.get_moves(coord!(0, 0));
    assert_eq!(moves.len(), 7);

    // Case 2: Top right
    /*
    . . B
    . * .
    * . .

    7 valid moves
    */

    board.reset();
    board.place_piece(piece!(Bishop, White), coord!(0, 7));
    moves = board.get_moves(coord!(0, 7));
    assert_eq!(moves.len(), 7);

    // Case 3: Bottom left
    /*
    . . *
    . * .
    B . .

    7 valid moves
    */

    board.reset();
    board.place_piece(piece!(Bishop, White), coord!(7, 0));
    moves = board.get_moves(coord!(7, 0));
    assert_eq!(moves.len(), 7);

    // Case 4: Bottom right
    /*
    * . .
    . * .
    . . B

    7 valid moves
    */

    board.reset();
    board.place_piece(piece!(Bishop, White), coord!(7, 7));
    moves = board.get_moves(coord!(7, 7));
    assert_eq!(moves.len(), 7);

    // Case 5: Middle
    /*
    * . *
    . B .
    * . *

    13 valid moves
    */

    board.reset();
    board.place_piece(piece!(Bishop, White), coord!(3, 3));
    moves = board.get_moves(coord!(3, 3));
    assert_eq!(moves.len(), 13);

    // Case 6: Friendly piece on the way
    /*
    * . * .
    . B . .
    * . p .
    . . . .

    10 valid moves
    */

    board.reset();
    board.place_piece(piece!(Pawn, White), coord!(6, 6));
    board.place_piece(piece!(Bishop, White), coord!(5, 5));
    moves = board.get_moves(coord!(5, 5));
    assert_eq!(moves.len(), 9);

    // Case 7: Enemy piece on the way
    board.reset();
    board.place_piece(piece!(Pawn, Black), coord!(6, 6));
    board.place_piece(piece!(Bishop, White), coord!(5, 5));
    moves = board.get_moves(coord!(5, 5));
    assert_eq!(moves.len(), 10);
  }

  #[test]
  fn linear_moves() {
    let mut board = Board::new();
    let mut moves: Vec<Coordinate>;

    // Case 1: Top left
    /*
    R * *
    * . .
    * . .

    14 valid moves
    */

    moves = board.get_moves(coord!(0, 0));
    assert_eq!(moves.len(), 14);

    // Case 2: Top right
    /*
    * * R
    . . *
    . . *

    14 valid moves
    */

    moves = board.get_moves(coord!(0, 7));
    assert_eq!(moves.len(), 14);

    // Case 3: Bottom left
    /*
    * . .
    * . .
    R * *

    14 valid moves
    */

    moves = board.get_moves(coord!(7, 0));
    assert_eq!(moves.len(), 14);

    // Case 4: Bottom right
    /*
    . . *
    . . *
    * * R

    14 valid moves
    */

    moves = board.get_moves(coord!(7, 7));
    assert_eq!(moves.len(), 14);

    // Case 5: Middle
    /*
    . * . .
    * R * *
    . * . .
    . * . .

    14 valid moves
    */

    moves = board.get_moves(coord!(3, 3));
    assert_eq!(moves.len(), 14);

    // Case 6: Friendly piece on the way
    /*
    . * . .
    * R * *
    . P . .
    . . . .

    12 valid moves
    */

    board.place_piece(piece!(Pawn, White), coord!(6, 6));
    moves = board.get_moves(coord!(6, 5));
    assert_eq!(moves.len(), 12);

    // Case 7: Enemy piece on the way
    /*
    . * . .
    * R * *
    . p . .
    . . . .

    13 valid moves
    */

    board.place_piece(piece!(Pawn, Black), coord!(6, 6));
    moves = board.get_moves(coord!(6, 5));
    assert_eq!(moves.len(), 13);
  }

  #[test]
  fn knight_moves() {
    let mut board: Board = Board::new();
    let knight = piece!(Knight, White);
    let mut moves: Vec<Coordinate>;

    // Case 1: Top left
    /*
    K . . . .
    . . * . .
    . * . . .
    . . . . .
    . . . . .

    2 valid moves
    */

    board.place_piece(knight, coord!(0, 0));
    moves = board.get_moves(coord!(0, 0));
    assert_eq!(moves.len(), 2);
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

    board.place_piece(knight, coord!(0, 7));
    moves = board.get_moves(coord!(0, 7));
    assert_eq!(moves.len(), 2);
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

    board.place_piece(knight, coord!(7, 0));
    moves = board.get_moves(coord!(7, 0));
    assert_eq!(moves.len(), 2);
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

    board.place_piece(knight, coord!(7, 7));
    moves = board.get_moves(coord!(7, 7));
    assert_eq!(moves.len(), 2);
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

    board.place_piece(knight, coord!(2, 2));
    moves = board.get_moves(coord!(2, 2));
    assert_eq!(moves.len(), 8);
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

    board.place_piece(knight, coord!(2, 2));
    board.place_piece(piece!(Pawn, White), coord!(4, 1));
    moves = board.get_moves(coord!(2, 2));
    assert_eq!(moves.len(), 7);
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

    board.place_piece(knight, coord!(2, 2));
    board.place_piece(piece!(Pawn, White), coord!(4, 1));
    board.place_piece(piece!(Pawn, White), coord!(3, 4));
    moves = board.get_moves(coord!(2, 2));
    assert_eq!(moves.len(), 6);
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

    board.place_piece(knight, coord!(2, 2));
    board.place_piece(piece!(Pawn, Black), coord!(4, 3));
    moves = board.get_moves(coord!(2, 2));
    assert_eq!(moves.len(), 8);
    board.reset();
  }

  #[test]
  fn pawn_moves() {
    let mut board: Board = Board::new();
    let pawn = piece!(Pawn, White);
    let mut moves: Vec<Coordinate>;

    // Case 1: Pawn in the middle
    /*
    . . . . .
    . . * . .
    . . P . .

    1 valid move
    */

    board.place_piece(pawn, coord!(4, 3));
    moves = board.get_moves(coord!(4, 3));
    assert_eq!(moves.len(), 1);
    board.reset();

    // Case 2: Pawn in the middle with a friendly piece on the way
    /*
    . . . . .
    . . K . .
    . . P . .

    0 valid moves
    */

    board.place_piece(pawn, coord!(4, 3));
    board.place_piece(piece!(King, White), coord!(3, 3));
    moves = board.get_moves(coord!(4, 3));
    assert_eq!(moves.len(), 0);
    board.reset();

    // Case 3: Pawn in the middle with an enemy piece on the way
    /*
    . . . . .
    . . p . .
    . . P . .

    0 valid moves
    */

    board.place_piece(pawn, coord!(4, 3));
    board.place_piece(piece!(Pawn, Black), coord!(3, 3));
    moves = board.get_moves(coord!(4, 3));
    assert_eq!(moves.len(), 0);
    board.reset();

    // Case 4: Pawn in the middle with a enemy piece that can be captured
    /*
    . . . . .
    . . * p .
    . . P . .

    2 valid move
    */

    board.place_piece(pawn, coord!(4, 3));
    board.place_piece(piece!(Pawn, Black), coord!(3, 4));
    moves = board.get_moves(coord!(4, 3));
    assert_eq!(moves.len(), 2);
    board.reset();

    // Case 5: Pawn in the middle with 2 enemy pieces that can be captured
    /*
    . . . . .
    . p * p .
    . . P . .
    . . . . .

    3 valid moves
    */

    board.place_piece(pawn, coord!(4, 3));
    board.place_piece(piece!(Pawn, Black), coord!(3, 4));
    board.place_piece(piece!(Pawn, Black), coord!(3, 2));
    moves = board.get_moves(coord!(4, 3));
    assert_eq!(moves.len(), 3);
    board.reset();

    // Case 6: Pawn in the starting row
    // 2 valid moves

    board.place_piece(pawn, coord!(6, 1));
    moves = board.get_moves(coord!(6, 1));
    assert_eq!(moves.len(), 2);
    board.reset();

    // Case 7: Pawn in the starting row with a friendly piece 2 steps away
    // 1 valid move

    board.place_piece(pawn, coord!(6, 1));
    board.place_piece(piece!(Pawn, White), coord!(4, 1));
    moves = board.get_moves(coord!(6, 1));
    assert_eq!(moves.len(), 1);
    board.reset();

    // Case 8: Pawn in the starting row with an enemy piece 2 steps away
    // 1 valid moves

    board.place_piece(pawn, coord!(6, 1));
    board.place_piece(piece!(Pawn, Black), coord!(4, 1));
    moves = board.get_moves(coord!(6, 1));
    assert_eq!(moves.len(), 1);
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

    board.place_piece(pawn, coord!(3, 3));
    board.place_piece(piece!(Pawn, Black), coord!(3, 4));
    board.en_passant_target_sq = Some(coord!(2, 4));
    moves = board.get_moves(coord!(3, 3));
    assert_eq!(moves.len(), 2);
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

    board.load_fen(String::from("8/8/2pK4/3Pp3/8/8/8/8 w - e6 0 1"));
    assert_eq!(moves.len(), 2);
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

       For pawn it is 0 valid moves
    */

    board.load_fen(String::from("8/8/8/8/1KP3r1/8/8/8 w - - 0 1"));
    assert_eq!(board.get_moves(coord!(4, 2)).len(), 0);

    // Case 2
    /*
      . . .
      K R r
      . . .

      For rook it is 1 valid move
    */

    board.load_fen(String::from("8/8/8/8/1KRr4/8/8/8 w - - 0 1"));
    assert_eq!(board.get_moves(coord!(4, 2)).len(), 1);
  }

  // NOTE: Not implemented filtering of moves that lead to check, no sense to run it =D
  #[test]
  fn is_checkmate() {
    let mut board = Board::new();

    // Case 1: Double rook mate
    /*
       K . r
       . . r
       . . .

       true
    */

    println!("Double rook mate");
    board.load_fen(String::from("K6r/7r/8/8/8/8/8/8 w - - 0 1"));
    assert_eq!(board.is_in_checkmate(Color::White), true);
    board.reset();

    // Case 2: Anderssen's mate
    /*
     * . k R
     * . P .
     * K . .
     */

    println!("Anderssen's mate");
    // board.load_fen(String::from("6kR/6P1/5K2/8/8/8/8/8 w - - 0 1"));
    board.place_piece(piece!(King, Black), coord!(0, 6));
    board.place_piece(piece!(Rook, White), coord!(0, 7));
    board.place_piece(piece!(Pawn, White), coord!(1, 6));
    board.place_piece(piece!(King, White), coord!(2, 5));
    assert_eq!(board.is_in_checkmate(Color::Black), true);
    board.reset();

    // Case 3: Arabian mate

    println!("Arabian mate");
    board.load_fen(String::from("7k/7R/5N2/8/8/8/8/8 w - - 0 1"));
    board.reset();

    // Case N: Not a mate
    println!("Not a mate");
    board.load_fen(String::from("K6r/7r/6N1/8/8/8/8/8 w - - 0 1"));
    assert_eq!(board.is_in_checkmate(Color::White), false);
  }

  #[test]
  fn get_king_coord() {
    let mut board = Board::new();
    let king = piece!(King, White);

    macro_rules! test {
      ($x: expr, $y: expr) => {
        board.place_piece(king, coord!($x, $y));
        assert_eq!(board.get_king_coord(Color::White), Some(coord!($x, $y)));
        board.reset();
      };
    }

    test!(1, 1);
    test!(2, 7);
    test!(3, 5);
    test!(5, 2);
    test!(5, 1);
    test!(6, 6);
    test!(6, 3);
  }
}
