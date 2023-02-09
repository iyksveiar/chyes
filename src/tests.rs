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
  use crate::*;

  #[test]
  fn is_in_check() {
    macro_rules! test {
      ($fen:expr, $color:ident, $expected:expr) => {
        assert_eq!(
          Board::from_fen($fen)
            .expect("Couldn't load FEN")
            .is_in_check(Color::$color),
          $expected
        );
      };
    }

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

    test!("8/8/8/3K4/8/8/8/8 w - - 0 1", White, false);

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

    test!("q7/8/8/3K4/8/8/8/8 w - - 0 1", White, true);

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

    test!("Q7/8/8/3K4/8/8/8/8 w - - 0 1", White, false);

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

    test!("3r4/8/8/3K4/8/8/8/8 w - - 0 1", White, true);

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

    test!("8/8/4n3/8/3K4/8/8/8 w - - 0 1", White, true);

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

    test!("8/8/4p3/3K4/8/8/8/8 w - - 0 1", White, true);

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

    test!("2r1r3/8/7r/3K4/7r/8/8/8 w - - 0 1", White, false);

    // Case 8
    /*
      . . . . .
      . . . . .
      . . K . .
      . . . . .
      . . . . b

      true
    */

    test!("b7/8/8/3K4/8/8/8/8 w - - 0 1", White, true);

    // Case 9
    /*
      . . . . q
      . . . P .
      . . K . .
      . . . . .
      . . . . .

      false
    */

    test!("6b1/8/4P3/3K4/8/8/8/8 w - - 0 1", White, false);

    // Case 10
    /*
      . . . . .
      . . . . .
      . . K . r
      . . . . .
      . . . . .

      true
    */

    test!("8/8/8/3K3r/8/8/8/8 w - - 0 1", White, true);

    // Case 11
    assert_eq!(
      Board::from_fen("4r3/8/8/4K2r/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .is_in_check(Color::White),
      true
    );
  }

  #[test]
  fn coordinates_to_notation() {
    assert_eq!(coord!(1, 1).to_string(), "b7");
    assert_eq!(coord!(3, 4).to_string(), "e5");
    assert_eq!(coord!(5, 7).to_string(), "h3");
    assert_eq!(coord!(7, 7).to_string(), "h1");
  }

  #[test]
  fn coordinates_from_notation() {
    assert_eq!(Coordinate::from_str("b7"), Ok(coord!(1, 1)));
    assert_eq!(Coordinate::from_str("e5"), Ok(coord!(3, 4)));
    assert_eq!(Coordinate::from_str("h3"), Ok(coord!(5, 7)));
    assert_eq!(Coordinate::from_str("h1"), Ok(coord!(7, 7)));
    assert_eq!(Err("Couldn't parse notation"), Coordinate::from_str("a9"));
    assert_eq!(Err("Couldn't parse notation"), Coordinate::from_str("abc"));
  }

  #[test]
  fn king_moves() {
    // Case 1: King in the top left
    /*
      K * .
      * * .
      . . .

      3 valid moves
    */

    assert_eq!(
      Board::from_fen("K7/8/8/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(0, 0))
        .expect("Couldn't generate moves")
        .len(),
      3
    );

    // Case 2: King in the top right
    /*
      . * K
      . * *
      . . .

      3 valid moves
    */

    assert_eq!(
      Board::from_fen("7K/8/8/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(0, 7))
        .expect("Couldn't generate moves")
        .len(),
      3
    );

    // Case 3: King in the bottom left
    /*
      . . .
      * * .
      K * .

      3 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/8/8/8/8/K7 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(7, 0))
        .expect("Couldn't generate moves")
        .len(),
      3
    );

    // Case 4: King in the bottom right
    /*
      . . .
      . * *
      . * K

      3 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/8/8/8/8/7K w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(7, 7))
        .expect("Couldn't generate moves")
        .len(),
      3
    );

    // Case 5: King in the middle
    /*
      * * *
      * K *
      * * *

      8 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/3K4/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(3, 3))
        .expect("Couldn't generate moves")
        .len(),
      8
    );

    // Case 6: King in the middle with a friendly piece in the way
    /*
      * * *
      * K Q
      * * *

      7 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/3KQ3/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(3, 3))
        .expect("Couldn't generate moves")
        .len(),
      7
    );

    // Case 7: King in the middle with an enemy piece in the way
    /*
      * . .
      . K q
      * . .

      3 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/3Kq3/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(3, 3))
        .expect("Couldn't generate moves")
        .len(),
      3
    );

    // Case 8: Rooks and King, stale mate
    /*
      . r . r .
      . . . . r
      . . K . .
      . . . . r
      . . . . .

      0 valid moves
    */

    assert_eq!(
      Board::from_fen("2r1r3/8/7r/3K4/7r/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(3, 3))
        .expect("Couldn't generate moves")
        .len(),
      0
    );

    // Case 9: King in the middle with an enemy piece blocking some way
    /*
      . . . .
      . r . .
      . . K *
      . . * *

      4 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/8/8/5r2/6K1/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(6, 6))
        .expect("Couldn't generate moves")
        .len(),
      4
    );
  }

  #[test]
  fn diagonal_moves() {
    // Case 1: Top left
    /*
      B . .
      . * .
      . . *

      7 valid moves
    */

    assert_eq!(
      Board::from_fen("B7/8/8/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(0, 0))
        .expect("Couldn't generate moves")
        .len(),
      7
    );

    // Case 2: Top right
    /*
      . . B
      . * .
      * . .

      7 valid moves
    */

    assert_eq!(
      Board::from_fen("7B/8/8/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(0, 7))
        .expect("Couldn't generate moves")
        .len(),
      7
    );

    // Case 3: Bottom left
    /*
      . . *
      . * .
      B . .

      7 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/8/8/8/8/B7 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(7, 0))
        .expect("Couldn't generate moves")
        .len(),
      7
    );

    // Case 4: Bottom right
    /*
      * . .
      . * .
      . . B

      7 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/8/8/8/8/7B w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(7, 7))
        .expect("Couldn't generate moves")
        .len(),
      7
    );

    // Case 5: Middle
    /*
      * . *
      . B .
      * . *

      13 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/3B4/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(3, 3))
        .expect("Couldn't generate moves")
        .len(),
      13
    );

    // Case 6: Friendly piece on the way
    /*
      * . *
      . B .
      * . P

      3 valid moves
    */

    assert_eq!(
      Board::from_fen("8/1B6/2P5/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(1, 1))
        .expect("Couldn't generate moves")
        .len(),
      3
    );

    // Case 7: Enemy piece on the way
    /*
      * . * .
      . B . .
      * . p .
      . . . .

      4 valid moves
    */

    assert_eq!(
      Board::from_fen("8/1B6/2p5/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(1, 1))
        .expect("Couldn't generate moves")
        .len(),
      4
    );
  }

  #[test]
  fn linear_moves() {
    // Case 1: Top left
    /*
      R * *
      * . .
      * . .

      14 valid moves
    */

    assert_eq!(
      Board::from_fen("R7/8/8/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(0, 0))
        .expect("Couldn't generate moves")
        .len(),
      14
    );

    // Case 2: Top right
    /*
      * * R
      . . *
      . . *

      14 valid moves
    */

    assert_eq!(
      Board::from_fen("7R/8/8/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(0, 7))
        .expect("Couldn't generate moves")
        .len(),
      14
    );

    // Case 3: Bottom left
    /*
      * . .
      * . .
      R * *

      14 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/8/8/8/8/R7 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(7, 0))
        .expect("Couldn't generate moves")
        .len(),
      14
    );

    // Case 4: Bottom right
    /*
      . . *
      . . *
      * * R

      14 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/8/8/8/8/7R w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(7, 7))
        .expect("Couldn't generate moves")
        .len(),
      14
    );

    // Case 5: Middle
    /*
      . * . .
      * R * *
      . * . .
      . * . .

      14 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/3R4/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(3, 3))
        .expect("Couldn't generate moves")
        .len(),
      14
    );

    // Case 6: Friendly piece on the way
    /*
      . * . .
      * R * *
      . P . .
      . . . .

      10 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/3R4/3P4/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(3, 3))
        .expect("Couldn't generate moves")
        .len(),
      10
    );

    // Case 7: Enemy piece on the way
    /*
      . * . .
      * R * *
      . p . .
      . . . .

      13 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/3R4/3p4/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(3, 3))
        .expect("Couldn't generate moves")
        .len(),
      11
    );
  }

  #[test]
  fn knight_moves() {
    // Case 1: Top left
    /*
      K . . . .
      . . * . .
      . * . . .
      . . . . .
      . . . . .

      2 valid moves
    */

    assert_eq!(
      Board::from_fen("N7/8/8/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(0, 0))
        .expect("Couldn't generate moves")
        .len(),
      2
    );

    // Case 2: Top right
    /*
      . . . . K
      . . * . .
      . . . * .
      . . . . .
      . . . . .

      2 valid moves
    */

    assert_eq!(
      Board::from_fen("7N/8/8/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(0, 7))
        .expect("Couldn't generate moves")
        .len(),
      2
    );

    // Case 3: Bottom left
    /*
      . . . . .
      . . . . .
      . * . . .
      . . * . .
      K . . . .

      2 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/8/8/8/8/N7 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(7, 0))
        .expect("Couldn't generate moves")
        .len(),
      2
    );

    // Case 4: Bottom left
    /*
      . . . . .
      . . . . .
      . . . * .
      . . * . .
      . . . . K

      2 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/8/8/8/8/7N w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(7, 7))
        .expect("Couldn't generate moves")
        .len(),
      2
    );

    // Case 5: Middle
    /*
      . * . * .
      * . . . *
      . . K . .
      * . . . *
      . * . * .

      8 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/3N4/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(3, 3))
        .expect("Couldn't generate moves")
        .len(),
      8
    );

    // Case 6: 1 friendly piece on a way
    /*
      . * . * .
      * . . . *
      . . K . .
      * . . . *
      . P . * .

      7 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/3N4/8/2P5/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(3, 3))
        .expect("Couldn't generate moves")
        .len(),
      7
    );

    // Case 7: 2 friendly pieces on a way
    /*
      . * . * .
      * . . . *
      . . K . .
      * . . . P
      . P . * .

      6 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/3N4/5P2/2P5/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(3, 3))
        .expect("Couldn't generate moves")
        .len(),
      6
    );

    // Case 8: Enemy piece on the way
    /*
      . * . * .
      * . . . *
      . . K . .
      * . . . *
      . * . p .

      8 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/3N4/5p2/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(3, 3))
        .expect("Couldn't generate moves")
        .len(),
      8
    );
  }

  #[test]
  fn pawn_moves() {
    // Case 1: Pawn in the middle
    /*
      . . . . .
      . . * . .
      . . P . .

      1 valid move
    */

    assert_eq!(
      Board::from_fen("8/8/8/8/3P4/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(4, 3))
        .expect("Couldn't generate moves")
        .len(),
      1
    );

    // Case 2: Pawn in the middle with a friendly piece on the way
    /*
      . . . . .
      . . K . .
      . . P . .

      0 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/3K4/3P4/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(4, 3))
        .expect("Couldn't generate moves")
        .len(),
      0
    );

    // Case 3: Pawn in the middle with an enemy piece on the way
    /*
      . . . . .
      . . p . .
      . . P . .

      0 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/3p4/3P4/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(4, 3))
        .expect("Couldn't generate moves")
        .len(),
      0
    );

    // Case 4: Pawn in the middle with a enemy piece that can be captured
    /*
      . . . . .
      . . * p .
      . . P . .

      2 valid move
    */

    assert_eq!(
      Board::from_fen("8/8/8/4p3/3P4/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(4, 3))
        .expect("Couldn't generate moves")
        .len(),
      2
    );

    // Case 5: Pawn in the middle with 2 enemy pieces that can be captured
    /*
      . . . . .
      . p * p .
      . . P . .
      . . . . .

      3 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/2p1p3/3P4/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(4, 3))
        .expect("Couldn't generate moves")
        .len(),
      3
    );

    // Case 6: Pawn in the starting row
    // 2 valid moves

    assert_eq!(
      Board::from_fen("8/8/8/8/8/8/3P4/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(6, 3))
        .expect("Couldn't generate moves")
        .len(),
      2
    );

    // Case 7: Pawn in the starting row with a friendly piece 2 steps away
    // 1 valid move

    assert_eq!(
      Board::from_fen("8/8/8/8/3P4/8/3P4/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(6, 3))
        .expect("Couldn't generate moves")
        .len(),
      1
    );

    // Case 8: Pawn in the starting row with an enemy piece 2 steps away
    // 1 valid moves

    assert_eq!(
      Board::from_fen("8/8/8/8/3p4/8/3P4/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(6, 3))
        .expect("Couldn't generate moves")
        .len(),
      1
    );

    // Case 9: Pawn in the middle with a Pawn that made a 2 step move, en passant possible
    /*
      . . . . .
      . . * * .
      . . P p .
      . . . . .
      . . . . .

      2 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/2Pp4/8/8/8/8 b - d6 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(3, 2))
        .expect("Couldn't generate moves")
        .len(),
      2
    );

    // Case 10: Advanced en passant
    /*
      . . . . .
      . p K * .
      . . P p .
      . . . . .
      . . . . .

      2 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/2pK4/3Pp3/8/8/8/8 w - e6 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(3, 3))
        .expect("Couldn't generate moves")
        .len(),
      2
    );
  }

  #[test]
  fn moves_that_lead_to_check() {
    // Case 1
    /*
       . . .
       K P r
       . . .

       For Pawn it is 0 valid moves
    */

    assert_eq!(
      Board::from_fen("8/8/8/8/1KP3r1/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(4, 2))
        .expect("Couldn't generate moves")
        .len(),
      0
    );

    // Case 2
    /*
      . . .
      K R r
      . . .

      For rook it is 1 valid move
    */

    assert_eq!(
      Board::from_fen("8/8/8/8/1KRr4/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .generate_moves(coord!(4, 2))
        .expect("Couldn't generate moves")
        .len(),
      1
    );
  }

  #[test]
  fn is_checkmate() {
    // Case 1: Double rook mate
    assert_eq!(
      Board::from_fen("K6r/7r/8/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .is_in_checkmate(Color::White),
      true
    );

    // Case 2: Anderssen's mate
    assert_eq!(
      Board::from_fen("6kR/6P1/5K2/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .is_in_checkmate(Color::Black),
      true
    );

    // Case 3: Arabian mate
    assert_eq!(
      Board::from_fen("7k/7R/5N2/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .is_in_checkmate(Color::Black),
      true
    );

    // Case 4: Backrank mate
    assert_eq!(
      Board::from_fen("3R2k1/5ppp/8/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .is_in_checkmate(Color::Black),
      true
    );

    // Case 4: Balestra mate
    assert_eq!(
      Board::from_fen("6k1/8/4B2Q/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .is_in_checkmate(Color::Black),
      true
    );

    // Case 5: Bishop and knight mate
    assert_eq!(
      Board::from_fen("7k/8/5BKN/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .is_in_checkmate(Color::Black),
      true
    );

    // Case 6: Blackburne's mate
    assert_eq!(
      Board::from_fen("5rk1/7B/8/6N1/8/8/1B6/8 w - - 0 1")
        .expect("Failed to load FEN")
        .is_in_checkmate(Color::Black),
      true
    );

    // Case N: Not a mate
    assert_eq!(
      Board::from_fen("K6r/7r/6N1/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .is_in_checkmate(Color::White),
      false
    );
  }

  #[test]
  fn get_king_coord() {
    assert_eq!(
      Board::from_fen("8/1K6/8/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .get_king_coord(Color::White),
      Some(coord!(1, 1))
    );

    assert_eq!(
      Board::from_fen("8/8/6K1/8/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .get_king_coord(Color::White),
      Some(coord!(2, 6))
    );

    assert_eq!(
      Board::from_fen("8/8/8/5K2/8/8/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .get_king_coord(Color::White),
      Some(coord!(3, 5))
    );

    assert_eq!(
      Board::from_fen("8/8/8/8/8/2K5/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .get_king_coord(Color::White),
      Some(coord!(5, 2))
    );

    assert_eq!(
      Board::from_fen("8/8/8/8/8/1K6/8/8 w - - 0 1")
        .expect("Failed to load FEN")
        .get_king_coord(Color::White),
      Some(coord!(5, 1))
    );

    assert_eq!(
      Board::from_fen("8/8/8/8/8/8/6K1/8 w - - 0 1")
        .expect("Failed to load FEN")
        .get_king_coord(Color::White),
      Some(coord!(6, 6))
    );

    assert_eq!(
      Board::from_fen("8/8/8/8/8/8/3K4/8 w - - 0 1")
        .expect("Failed to load FEN")
        .get_king_coord(Color::White),
      Some(coord!(6, 3))
    );
  }
}
