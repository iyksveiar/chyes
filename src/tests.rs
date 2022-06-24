use crate::chess::*;

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn new_board() {
		let board = Board::new();
		assert_eq!(board.board.len(), 8);
		for row in 0..8 {
			assert_eq!(board.board[row].len(), 8);
		}
	}
	
	#[test]
	fn is_in_check() {
		let mut board: Board = Board::new();
		let king = Piece {
			breed: Pieces::King,
			color: Color::White,
		};
		
		// Case 1: King in the middle
		/*
		. . . . .
		. . . . .
		. . K . .
		. . . . .
		. . . . .
		
		false
		*/
		
		board.place_piece(king, 3, 3);
		assert_eq!(board.is_in_check(Color::White), false);
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
		
		board.place_piece(
			Piece {
				breed: Pieces::Queen,
				color: Color::Black,
			},
			0,
			0,
		);
		board.place_piece(king, 3, 3);
		assert_eq!(board.is_in_check(Color::White), true);
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
		
		board.place_piece(
			Piece {
				breed: Pieces::Queen,
				color: Color::White,
			},
			0,
			0,
		);
		board.place_piece(king, 3, 3);
		assert_eq!(board.is_in_check(Color::White), false);
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
		
		board.place_piece(king, 3, 3);
		board.place_piece(
			Piece {
				breed: Pieces::Rook,
				color: Color::Black,
			},
			0,
			3,
		);
		assert_eq!(board.is_in_check(Color::White), true);
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
		
		board.place_piece(king, 3, 3);
		board.place_piece(
			Piece {
				breed: Pieces::Knight,
				color: Color::Black,
			},
			5,
			4,
		);
		assert_eq!(board.is_in_check(Color::White), true);
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
		
		board.place_piece(king, 3, 3);
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::Black,
			},
			2,
			4,
		);
		assert_eq!(board.is_in_check(Color::White), true);
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
		
		board.place_piece(king, 3, 3);
		board.place_piece(
			Piece {
				breed: Pieces::Rook,
				color: Color::Black,
			},
			0,
			2,
		);
		board.place_piece(
			Piece {
				breed: Pieces::Rook,
				color: Color::Black,
			},
			0,
			4,
		);
		board.place_piece(
			Piece {
				breed: Pieces::Rook,
				color: Color::Black,
			},
			2,
			7,
		);
		board.place_piece(
			Piece {
				breed: Pieces::Rook,
				color: Color::Black,
			},
			4,
			4,
		);
		assert_eq!(board.is_in_check(Color::White), false);
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
		
		board.place_piece(king, 3, 3);
		board.place_piece(
			Piece {
				breed: Pieces::Bishop,
				color: Color::Black,
			},
			5,
			5,
		);
		assert_eq!(board.is_in_check(Color::White), true);
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
		
		board.place_piece(king, 4, 3);
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::White,
			},
			3,
			4,
		);
		board.place_piece(
			Piece {
				breed: Pieces::Queen,
				color: Color::Black,
			},
			0,
			7,
		);
		assert_eq!(board.is_in_check(Color::White), false);
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
		
		board.place_piece(king, 3, 3);
		board.place_piece(
			Piece {
				breed: Pieces::Rook,
				color: Color::Black,
			},
			3,
			7,
		);
		assert_eq!(board.is_in_check(Color::White), true);
		board.clear();
		
		// Case 11
		board.load_fen("4r3/8/8/4K2r/8/8/8/8 w - - 0 1");
		assert_eq!(board.is_in_check(Color::White), true);
		board.clear();
	}
	
	#[test]
	fn coordinates_to_notation() {
		assert_eq!(Coordinate { row: 1, col: 1 }.to_string(), "b7".to_string());
		assert_eq!(Coordinate { row: 3, col: 4 }.to_string(), "e5".to_string());
		assert_eq!(Coordinate { row: 5, col: 7 }.to_string(), "h3".to_string());
		assert_eq!(Coordinate { row: 7, col: 7 }.to_string(), "h1".to_string());
	}
	
	#[test]
	fn king_moves() {
		let mut board = Board::new();
		let king = Piece {
			color: Color::White,
			breed: Pieces::King,
		};
		let mut moves: Vec<Coordinate>;
		
		// First case: King in the top left
		// K * .
		// * * .
		// . . .
		// 3 valid moves
		
		board.place_piece(king, 0, 0);
		moves = board.get_moves(0, 0);
		assert_eq!(moves.len(), 3);
		board.clear();
		
		// Second case: King in the top right
		// . * K
		// . * *
		// . . .
		// 3 valid moves
		
		board.place_piece(king, 0, 7);
		moves = board.get_moves(0, 7);
		assert_eq!(moves.len(), 3);
		board.clear();
		
		// Third case: King in the bottom left
		// . . .
		// * * .
		// K * .
		// 3 valid moves
		
		board.place_piece(king, 7, 0);
		moves = board.get_moves(7, 0);
		assert_eq!(moves.len(), 3);
		board.clear();
		
		// Fourth case: King in the bottom right
		// . . .
		// . * *
		// . * K
		// 3 valid moves
		
		board.place_piece(king, 7, 7);
		moves = board.get_moves(7, 7);
		assert_eq!(moves.len(), 3);
		board.clear();
		
		// Fifth case: King in the middle
		// * * *
		// * K *
		// * * *
		// 8 valid moves
		
		board.place_piece(king, 4, 4);
		moves = board.get_moves(4, 4);
		assert_eq!(moves.len(), 8);
		board.clear();
		
		// Sixth case: King in the middle, with a friendly Piece in the way
		// * * *
		// * K Q
		// * * *
		// 7 valid moves
		
		board.place_piece(king, 4, 4);
		board.place_piece(
			Piece {
				color: Color::White,
				breed: Pieces::Queen,
			},
			3,
			4,
		);
		moves = board.get_moves(4, 4);
		assert_eq!(moves.len(), 7);
		board.clear();
		
		// Seventh case: King in the middle, with an enemy Piece in the way
		// * . .
		// . K q
		// * . .
		// 3 valid moves
		
		board.place_piece(king, 4, 4);
		board.place_piece(
			Piece {
				color: Color::Black,
				breed: Pieces::Queen,
			},
			4,
			5,
		);
		moves = board.get_moves(4, 4);
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
		moves = board.get_moves(3, 3);
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
		
		board.place_piece(king, 4, 4);
		board.place_piece(
			Piece {
				color: Color::Black,
				breed: Pieces::Rook,
			},
			3,
			3,
		);
		moves = board.get_moves(4, 4);
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
		
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::White,
			},
			6,
			6,
		);
		moves = board.diagonal_moves(5, 5, Color::White);
		assert_eq!(moves.len(), 9);
		
		// Case 7: Enemy Piece on the way
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::Black,
			},
			6,
			6,
		);
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
		
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::White,
			},
			6,
			6,
		);
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
		
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::Black,
			},
			6,
			6,
		);
		moves = board.linear_moves(6, 5, Color::White);
		assert_eq!(moves.len(), 13);
	}
	
	#[test]
	fn knight_moves() {
		let mut board: Board = Board::new();
		let knight: Piece = Piece {
			breed: Pieces::Knight,
			color: Color::White,
		};
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
		
		board.place_piece(knight, 0, 0);
		moves = board.get_moves(0, 0);
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
		
		board.place_piece(knight, 0, 7);
		moves = board.get_moves(0, 7);
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
		
		board.place_piece(knight, 7, 0);
		moves = board.get_moves(7, 0);
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
		
		board.place_piece(knight, 7, 7);
		moves = board.get_moves(7, 7);
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
		
		board.place_piece(knight, 2, 2);
		moves = board.get_moves(2, 2);
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
		
		board.place_piece(knight, 2, 2);
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::White,
			},
			4,
			1,
		);
		moves = board.get_moves(2, 2);
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
		
		board.place_piece(knight, 2, 2);
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::White,
			},
			4,
			1,
		);
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::White,
			},
			3,
			4,
		);
		moves = board.get_moves(2, 2);
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
		
		board.place_piece(knight, 2, 2);
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::Black,
			},
			4,
			3,
		);
		moves = board.get_moves(2, 2);
		assert_eq!(moves.len(), 8);
		board.clear();
	}
	
	#[test]
	fn pawn_moves() {
		let mut board: Board = Board::new();
		let pawn: Piece = Piece {
			breed: Pieces::Pawn,
			color: Color::White,
		};
		let mut moves: Vec<Coordinate>;
		
		// Case 1: Pawn in the middle
		/*
		. . . . .
		. . * . .
		. . P . .
		
		1 valid move
		*/
		
		board.place_piece(pawn, 4, 3);
		moves = board.get_moves(4, 3);
		assert_eq!(moves.len(), 1);
		board.clear();
		
		// Case 2: Pawn in the middle with a friendly Piece on the way
		/*
		. . . . .
		. . K . .
		. . P . .
		
		0 valid moves
		*/
		
		board.place_piece(pawn, 4, 3);
		board.place_piece(
			Piece {
				breed: Pieces::King,
				color: Color::White,
			},
			3,
			3,
		);
		moves = board.get_moves(4, 3);
		assert_eq!(moves.len(), 0);
		board.clear();
		
		// Case 3: Pawn in the middle with an enemy Piece on the way
		/*
		. . . . .
		. . p . .
		. . P . .
		
		0 valid moves
		*/
		
		board.place_piece(pawn, 4, 3);
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::Black,
			},
			3,
			3,
		);
		moves = board.get_moves(4, 3);
		assert_eq!(moves.len(), 0);
		board.clear();
		
		// Case 4: Pawn in the middle with a enemy Piece that can be captured
		/*
		. . . . .
		. . * p .
		. . P . .
		
		2 valid move
		*/
		
		board.place_piece(pawn, 4, 3);
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::Black,
			},
			3,
			4,
		);
		moves = board.get_moves(4, 3);
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
		
		board.place_piece(pawn, 4, 3);
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::Black,
			},
			3,
			4,
		);
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::Black,
			},
			3,
			2,
		);
		moves = board.get_moves(4, 3);
		assert_eq!(moves.len(), 3);
		board.clear();
		
		// Case 6: Pawn in the starting row
		// 2 valid moves
		
		board.place_piece(pawn, 6, 1);
		moves = board.get_moves(6, 1);
		assert_eq!(moves.len(), 2);
		board.clear();
		
		// Case 7: Pawn in the starting row with a friendly Piece 2 steps away
		// 1 valid move
		
		board.place_piece(pawn, 6, 1);
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::White,
			},
			4,
			1,
		);
		moves = board.get_moves(6, 1);
		assert_eq!(moves.len(), 1);
		board.clear();
		
		// Case 8: Pawn in the starting row with an enemy Piece 2 steps away
		// 1 valid moves
		
		board.place_piece(pawn, 6, 1);
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::Black,
			},
			4,
			1,
		);
		moves = board.get_moves(6, 1);
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
		
		board.place_piece(pawn, 4, 3);
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::Black,
			},
			4,
			4,
		);
		board.last_2_moves_pawn = Some(Coordinate { row: 4, col: 4 });
		board.get_moves(4, 3);
		moves = board.get_moves(4, 3);
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
		
		board.place_piece(pawn, 4, 3);
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::Black,
			},
			4,
			4,
		);
		board.place_piece(
			Piece {
				breed: Pieces::King,
				color: Color::White,
			},
			3,
			3,
		);
		board.place_piece(
			Piece {
				breed: Pieces::Pawn,
				color: Color::Black,
			},
			3,
			2,
		);
		board.last_2_moves_pawn = Some(Coordinate { row: 4, col: 4 });
		board.get_moves(4, 3);
		moves = board.get_moves(4, 3);
		assert_eq!(moves.len(), 2);
		board.clear();
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
    assert_eq!(board.get_moves(4, 2).len(), 0);

    // Case 2
    /*
       . . .
       K R r
       . . .

       For rook it is 1 valid move
     */

    board.load_fen("8/8/8/8/1KRr4/8/8/8 w - - 0 1");
    assert_eq!(board.get_moves(4, 2).len(), 1);
  }

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

    board.load_fen("K6r/7r/8/8/8/8/8/8 w - - 0 1");
    assert_eq!(board.is_in_checkmate(Color::White), true);
    board.clear();

    // Case 2: Anderssen's mate
    board.load_fen("6kR/6P1/5K2/8/8/8/8/8 w - - 0 1");
    assert_eq!(board.is_in_checkmate(Color::Black), true);
    board.clear();

    // Case 3: Arabian mate
    board.load_fen("7k/7R/5N2/8/8/8/8/8 w - - 0 1");
    board.clear();

    // Case N: Not a mate
    board.load_fen("K6r/7r/6N1/8/8/8/8/8 w - - 0 1");
    assert_eq!(board.is_in_checkmate(Color::White), false);
  }
}
