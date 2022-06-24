use crate::chess::*;

// Macro to expand coord!(x, y) to Coordinate { row: x, col: y }
macro_rules! coord {
    ($x:expr, $y:expr) => {
        Coordinate { row: $x, col: $y }
    };
}

// Macro to expand piece!(x, y) to Piece { breed: Pieces::x, color: Color::y }
macro_rules! piece {
    ($x:ident, $y:ident) => {
        Piece {
            breed: Pieces::$x,
            color: Color::$y,
        }
    };
}

#[test]
fn new_board() {
  let board = Board::new();
  assert_eq!(board.white_pieces.len(), 0);
  assert_eq!(board.black_pieces.len(), 0);
}

#[test]
fn default_board() {
  let board = Board::default();
  assert_eq!(board.white_pieces.len(), 16);
  assert_eq!(board.black_pieces.len(), 16);
  assert_eq!(board.turn, Color::White);
  assert_eq!(board.castling_permissions, [true, true, true, true]);
}

#[test]
fn king_moves() {
  let mut board = Board::new();
  let king = piece!(King, White);
  let mut moves: Vec<Coordinate>;
  
  // First case: King in the top left
  // K * .
  // * * .
  // . . .
  // 3 valid moves
  
  board.place_piece(king, coord!(0, 0));
  moves = board.get_moves(coord!(0, 0));
  assert_eq!(moves.len(), 3);
  board.clear();
  
  // Second case: King in the top right
  // . * K
  // . * *
  // . . .
  // 3 valid moves
  
  board.place_piece(king, coord!(0, 7));
  moves = board.get_moves(coord!(0, 7));
  assert_eq!(moves.len(), 3);
  board.clear();
  
  // Third case: King in the bottom left
  // . . .
  // * * .
  // K * .
  // 3 valid moves
  
  board.place_piece(king, coord!(7, 0));
  moves = board.get_moves(coord!(7, 0));
  assert_eq!(moves.len(), 3);
  board.clear();
  
  // Fourth case: King in the bottom right
  // . . .
  // . * *
  // . * K
  // 3 valid moves
  
  board.place_piece(king, coord!(7, 7));
  moves = board.get_moves(coord!(7, 7));
  assert_eq!(moves.len(), 3);
  board.clear();
  
  // Fifth case: King in the middle
  // * * *
  // * K *
  // * * *
  // 8 valid moves
  
  board.place_piece(king, coord!(4, 4));
  moves = board.get_moves(coord!(4, 4));
  assert_eq!(moves.len(), 8);
  board.clear();
  
  // Sixth case: King in the middle, with a friendly Piece in the way
  // * * *
  // * K Q
  // * * *
  // 7 valid moves
  
  board.place_piece(king, coord!(4, 4));
  board.place_piece(piece!(Queen, White), coord!(3, 4));
  moves = board.get_moves(coord!(7-4, 4));
  assert_eq!(moves.len(), 7);
  board.clear();
  
  // Seventh case: King in the middle, with an enemy Piece in the way
  // * . .
  // . K q
  // * . .
  // 3 valid moves
  
  board.place_piece(king, coord!(4, 4));
  board.place_piece(piece!(Queen, Black), coord!(4, 5));
  moves = board.get_moves(coord!(7-4, 4));
  assert_eq!(moves.len(), 3);
  board.clear();
  
  // Eighth case: Rooks and King, stale mate
  /*
  . r . r .
  . . . . r
  . . K . .
  . . . . r
  . . . . .
  
  0 valid moves
  */
  
  board.load_fen("2r1r3/8/7r/3K4/7r/8/8/8 w - - 0 1");
  moves = board.get_moves(coord!(7-3, 3));
  assert_eq!(moves.len(), 0);
  board.clear();
  
  // Ninth case: King in the middle, with an enemy Piece blocKing Some space
  /*
  . . . .
  . r . .
  . . K *
  . . * *
  
  4 valid moves
  */
  
  board.place_piece(king, coord!(4, 4));
  board.place_piece(piece!(Rook, Black), coord!(3, 3));
  moves = board.get_moves(coord!(7-4, 4));
  assert_eq!(moves.len(), 4);
  board.clear();
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
  
  moves = board.diagonal_moves(0, 0, Color::White);
  assert_eq!(moves.len(), 7);
  
  // Case 2: Top right
  /*
  . . B
  . * .
  * . .
  
  7 valid moves
  */
  
  moves = board.diagonal_moves(0, 7, Color::White);
  assert_eq!(moves.len(), 7);
  
  // Case 3: Bottom left
  /*
  . . *
  . * .
  B . .
  
  7 valid moves
  */
  
  moves = board.diagonal_moves(7, 0, Color::White);
  assert_eq!(moves.len(), 7);
  
  // Case 4: Bottom right
  /*
  * . .
  . * .
  . . B
  
  7 valid moves
  */
  
  moves = board.diagonal_moves(7, 7, Color::White);
  assert_eq!(moves.len(), 7);
  
  // Case 5: Middle
  /*
  * . *
  . B .
  * . *
  
  13 valid moves
  */
  
  moves = board.diagonal_moves(3, 3, Color::White);
  assert_eq!(moves.len(), 13);
  
  // Case 6: Friendly Piece on the way
  /*
  * . * .
  . B . .
  * . p .
  . . . .
  
  10 valid moves
  */
  
  board.place_piece(piece!(Pawn, White), coord!(6, 6));
  moves = board.diagonal_moves(5, 5, Color::White);
  assert_eq!(moves.len(), 9);
  
  // Case 7: Enemy Piece on the way
  board.place_piece(piece!(Pawn, Black), coord!(6, 6));
  moves = board.diagonal_moves(5, 5, Color::White);
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
  
  moves = board.linear_moves(0, 0, Color::White);
  assert_eq!(moves.len(), 14);
  
  // Case 2: Top right
  /*
  * * R
  . . *
  . . *
  
  14 valid moves
  */
  
  moves = board.linear_moves(0, 7, Color::White);
  assert_eq!(moves.len(), 14);
  
  // Case 3: Bottom left
  /*
  * . .
  * . .
  R * *
  
  14 valid moves
  */
  
  moves = board.linear_moves(7, 0, Color::White);
  assert_eq!(moves.len(), 14);
  
  // Case 4: Bottom right
  /*
  . . *
  . . *
  * * R
  
  14 valid moves
  */
  
  moves = board.linear_moves(7, 7, Color::White);
  assert_eq!(moves.len(), 14);
  
  // Case 5: Middle
  /*
  . * . .
  * R * *
  . * . .
  . * . .
  
  14 valid moves
  */
  
  moves = board.linear_moves(3, 3, Color::White);
  assert_eq!(moves.len(), 14);
  
  // Case 6: Friendly Piece on the way
  /*
  . * . .
  * R * *
  . P . .
  . . . .
  
  12 valid moves
  */
  
  board.place_piece(piece!(Pawn, White), coord!(6, 6));
  moves = board.linear_moves(6, 5, Color::White);
  assert_eq!(moves.len(), 12);
  
  // Case 7: Enemy Piece on the way
  /*
  . * . .
  * R * *
  . p . .
  . . . .
  
  13 valid moves
  */
  
  board.place_piece(piece!(Pawn, Black), coord!(6, 6));
  moves = board.linear_moves(6, 5, Color::White);
  assert_eq!(moves.len(), 13);
}

#[test]
fn knight_moves() {
  let mut board: Board = Board::new();
  let knight: Piece = piece!(Knight, White);
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
  board.clear();
  
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
  board.clear();
  
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
  board.clear();
  
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
  board.clear();
  
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
  board.clear();
  
  // Case 6: 1 friendly Piece on a way
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
  board.clear();
  
  // Case 7: 2 friendly Pieces on a way
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
  board.clear();
  
  // Case 8: Enemy Piece on the way
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
  board.clear();
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
  board.clear();
  
  // Case 2: Pawn in the middle with a friendly Piece on the way
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
  board.clear();
  
  // Case 3: Pawn in the middle with an enemy Piece on the way
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
  board.clear();
  
  // Case 4: Pawn in the middle with a enemy Piece that can be captured
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
  board.clear();
  
  // Case 5: Pawn in the middle with 2 enemy Pieces that can be captured
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
  board.clear();
  
  // Case 6: Pawn in the starting row
  // 2 valid moves
  
  board.place_piece(pawn, coord!(6, 1));
  moves = board.get_moves(coord!(6, 1));
  assert_eq!(moves.len(), 2);
  board.clear();
  
  // Case 7: Pawn in the starting row with a friendly Piece 2 steps away
  // 1 valid move
  
  board.place_piece(pawn, coord!(6, 1));
  board.place_piece(piece!(Pawn, White), coord!(4, 1));
  moves = board.get_moves(coord!(6, 1));
  assert_eq!(moves.len(), 1);
  board.clear();
  
  // Case 8: Pawn in the starting row with an enemy Piece 2 steps away
  // 1 valid moves
  
  board.place_piece(pawn, coord!(6, 1));
  board.place_piece(piece!(Pawn, Black), coord!(4, 1));
  moves = board.get_moves(coord!(6, 1));
  assert_eq!(moves.len(), 1);
  board.clear();
  
  // Case 9: Pawn in the middle with a Pawn that made a 2 step move, en passant possible
  /*
  . . . . .
  . . * * .
  . . P p .
  . . . . .
  . . . . .
  
  2 valid moves
  */
  
  board.load_fen("8/8/8/3Pp3/8/8/8/8 w - - 0 1");
  board.en_passant_target_sq = Some(coord!(2, 4));
  moves = board.get_moves(coord!(3, 3));
  assert_eq!(moves.len(), 2);
  board.clear();
  
  // Case 10: Advanced en passant
  /*
  . . . . .
  . p K * .
  . . P p .
  . . . . .
  . . . . .
  
  2 valid moves
  */
  
  board.load_fen("8/8/1pK5/2Pp4/8/8/8/8 w - - 0 1");
  board.en_passant_target_sq = Some(coord!(2, 3));
  moves = board.get_moves(coord!(3, 2)); 
  assert_eq!(moves.len(), 2);
  board.clear();
}

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
  assert_eq!(board.is_check(Color::White), false);
  board.clear();
  
  // Case 2
  /*
  q . . . .
  . . . . .
  . . K . .
  . . . . .
  . . . . .
  
  true
  */
  
  board.place_piece(piece!(Queen, Black), coord!(7, 7));
  board.place_piece(king, coord!(3, 3));
  assert_eq!(board.is_check(Color::White), true);
  board.clear();
  
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
  assert_eq!(board.is_check(Color::White), false);
  board.clear();
  
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
  assert_eq!(board.is_check(Color::White), true);
  board.clear();
  
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
  assert_eq!(board.is_check(Color::White), true);
  board.clear();
  
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
  assert_eq!(board.is_check(Color::White), true);
  board.clear();
  
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
  assert_eq!(board.is_check(Color::White), false);
  board.clear();
  
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
  assert_eq!(board.is_check(Color::White), true);
  board.clear();
  
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
  assert_eq!(board.is_check(Color::White), false);
  board.clear();
  
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
  assert_eq!(board.is_check(Color::White), true);
  board.clear();
  
  // Case 11
  board.load_fen("4r3/8/8/4K2r/8/8/8/8 w - - 0 1");
  assert_eq!(board.is_check(Color::White), true);
  board.clear();
}

#[test]
fn coordinate_to_notation() {
  assert_eq!(coord!(7 - 1, 1).to_string(), "b7".to_string());
  assert_eq!(coord!(7 - 3, 4).to_string(), "e5".to_string());
  assert_eq!(coord!(7 - 5, 7).to_string(), "h3".to_string());
  assert_eq!(coord!(7 - 7, 7).to_string(), "h1".to_string());
}

#[test]
fn moves_that_lead_to_check() {
  let mut board = Board::new();
  // Case 1
  /*
      . . .
      K P r
      . . .

      For Pawn it is 0 valid moves
  */

  board.load_fen("8/8/8/8/1KP3r1/8/8/8 w - - 0 1");
  assert_eq!(board.get_moves(coord!(7-4, 2)).len(), 0);

  // Case 2
  /*
      . . .
      K R r
      . . .

      For rook it is 1 valid move
    */

  board.load_fen("8/8/8/8/1KRr4/8/8/8 w - - 0 1");
  assert_eq!(board.get_moves(coord!(7-4, 2)).len(), 1);
}