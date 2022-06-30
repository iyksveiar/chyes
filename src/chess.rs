use std::collections::HashMap;

// Sequence: King, Queen, Rook, Bishop, Knight, Pawn
const BLACK_PIECES: [&str; 6] = ["♔", "♕", "♖", "♗", "♘", "♙"];
const WHITE_PIECES: [&str; 6] = ["♚", "♛", "♜", "♝", "♞", "♟"];

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

// Coordinate struct
#[derive(Eq, Hash, Clone, Copy, PartialEq, Debug)]
pub struct Coordinate {
	pub row: i8,
	pub col: i8,
}

#[allow(dead_code)]
impl Coordinate {
	/*
	56 57 58 59 60 61 62 63
	48 49 50 51 52 53 54 55
	40 41 42 43 44 45 46 47
	32 33 34 35 36 37 38 39
	24 25 26 27 28 29 30 31
	16 17 18 19 20 21 22 23
	08 09 10 11 12 13 14 15
	00 01 02 03 04 05 06 07
	*/
	
	pub fn to_string(&self) -> String {
		// Convert coordinate to string
		let mut result = String::new();
		// Use cols to convert to letters
		const COLS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
		result.push(COLS[self.col as usize]);
		result.push_str(&(8 - self.row).to_string());
		return result;
	}
	
	pub fn as_number(&self) -> i8 {
		// Convert coordinate to number
		// NOTE: This starts from left bottom corner, so we need to reverse the row
		return (7 - self.row) * 8 + self.col;
	}
	
	pub fn from_number(num: i8) -> Coordinate {
		// Convert number to coordinate
		// NOTE: This starts from left bottom corner, so we need to reverse the row
		Coordinate {
			row: 7 - (num / 8),
			col: num % 8,
		}
	}
}

// Chess Pieces
#[derive(Eq, Hash, Clone, Copy, PartialEq, Debug)]
pub enum Pieces {
	King,
	Queen,
	Rook,
	Bishop,
	Knight,
	Pawn,
	Empty,
}

#[derive(Hash, Eq, Clone, Copy, PartialEq, Debug)]
pub enum Color {
	White,
	Black,
}

#[derive(Hash, Eq, Clone, Copy, PartialEq, Debug)]
pub struct Piece {
	pub breed: Pieces,
	pub color: Color,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Board {
	pub board: [[Piece; 8]; 8], // 2D array of Pieces
	pub turn: Color,
	pub castling_rights: [bool; 4], // 0: white king side, 1: white queen side, 2: black king side, 3: black queen side
	pub white_pieces: Box<HashMap<i8, Piece>>,
	pub black_pieces: Box<HashMap<i8, Piece>>,
	pub en_passant_target_sq: Option<i8>,
	halfmove_clock: i8,
	fullmove_number: i8,
}

#[allow(dead_code)]
impl Board {
	pub fn new() -> Self {
		Board {
			board: [[piece!(Empty, White); 8]; 8],
			turn: Color::White,
			castling_rights: [false, false, false, false],
			white_pieces: Box::new(HashMap::new()),
			black_pieces: Box::new(HashMap::new()),
			en_passant_target_sq: None,
			halfmove_clock: 0,
			fullmove_number: 1,
		}
	}
	
	pub fn clear(&mut self) {
		for i in 0..8 {
			for j in 0..8 {
				self.board[i][j] = piece!(Empty, White);
			}
		}
		self.turn = Color::White;
		self.castling_rights = [false, false, false, false];
		self.white_pieces = Box::new(HashMap::new());
		self.black_pieces = Box::new(HashMap::new());
		self.en_passant_target_sq = None;
		self.halfmove_clock = 0;
		self.fullmove_number = 1;
	}
	
	pub fn load_fen(&mut self, fen: &str) {
		// function to parse fen string
		// source: https://en.wikipedia.org/wiki/forsyth%e2%80%93edwards_notation
		
		let mut fen_array = fen.split(' ');
		let fen_board = fen_array.next().unwrap();
		let fen_turn = fen_array.next().unwrap();
		let fen_castling = fen_array.next().unwrap();
		let _fen_en_passant = fen_array.next().unwrap();
		let _fen_half_move = fen_array.next().unwrap(); // todo
		let _fen_full_move = fen_array.next().unwrap(); // todo
		
		self.clear();
		
		// change the turn
		if fen_turn == "w" {
			self.turn = Color::White;
		} else if fen_turn == "b" {
			self.turn = Color::Black;
		} else {
			panic!("invalid turn");
		}
		
		// parse the Board
		let mut row: i8 = 0;
		let mut col: i8 = 0;
		let mut piece: Option<Piece> = None;
		
		for c in fen_board.chars() {
			if c == '/' {
				row += 1;
				col = 0; // because at the end of the loop, col will be incremented
			} else if c.is_digit(10) {
				col += c.to_digit(10).unwrap() as i8;
			} else {
				piece = match c {
					'K' => Some(piece!(King, White)),
					'Q' => Some(piece!(Queen, White)),
					'R' => Some(piece!(Rook, White)),
					'B' => Some(piece!(Bishop, White)),
					'N' => Some(piece!(Knight, White)),
					'P' => Some(piece!(Pawn, White)),
					'k' => Some(piece!(King, Black)),
					'q' => Some(piece!(Queen, Black)),
					'r' => Some(piece!(Rook, Black)),
					'b' => Some(piece!(Bishop, Black)),
					'n' => Some(piece!(Knight, Black)),
					'p' => Some(piece!(Pawn, Black)),
					_ => Some(piece!(Empty, White)),
				};
			};
			
			// if Piece is not None, then insert the Piece
			if piece != None {
				self.place_piece(piece.unwrap(), coord!(row, col));
				piece = None;
				col += 1;
			}
		}
		
		// Castling
		let mut castling_rights = [false, false, false, false];
		for c in fen_castling.chars() {
			match c {
				'K' => castling_rights[0] = true,
				'Q' => castling_rights[1] = true,
				'k' => castling_rights[2] = true,
				'q' => castling_rights[3] = true,
				_ => (),
			};
		}
		self.castling_rights = castling_rights;
	}
	
	pub fn get_fen(&self) -> String {
		// function to convert the Board to FEN
		// source: https://en.wikipedia.org/wiki/Forsyth%e2%80%93Edwards_notation
		
		let mut fen_board: String = String::new();
		let mut empty_count: u8 = 0;
		
		for i in 0..8 {
			for j in 0..8 {
				let piece = self.board[i][j];
				
				if piece.breed == Pieces::Empty {
					empty_count += 1;
				} else {
					if empty_count > 0 {
						fen_board.push_str(&empty_count.to_string());
						empty_count = 0;
					}
					
					use Color::*;
					use Pieces::*;
					
					let mut chr = match piece.breed {
						King => 'k',
						Queen => 'q',
						Rook => 'r',
						Bishop => 'b',
						Knight => 'n',
						Pawn => 'p',
						_ => panic!("Invalid piece"),
					};
					
					// If piece is white, transform to upper case
					if piece.color == White {
						chr = chr.to_ascii_uppercase();
					}
					
					fen_board.push_str(chr.to_string().as_str());
				}
			}
			
			if empty_count > 0 {
				fen_board.push_str(&empty_count.to_string());
				empty_count = 0;
			}
			
			if i != 7 {
				fen_board.push_str("/");
			}
		}
		
		// Separated by spaces add info about the turn, castling, en passant, and halfmove clock
		fen_board.push_str(" ");
		if self.turn == Color::White {
			fen_board.push_str("w");
		} else {
			fen_board.push_str("b");
		}
		
		// TODO: check if castling is possible
		fen_board.push_str(" ");
		if self.castling_rights[0] {
			fen_board.push_str("K");
		}
		if self.castling_rights[1] {
			fen_board.push_str("Q");
		}
		if self.castling_rights[2] {
			fen_board.push_str("k");
		}
		if self.castling_rights[3] {
			fen_board.push_str("q");
		}
		
		fen_board.push_str(" ");
		if self.en_passant_target_sq == None {
			fen_board.push_str("-");
		} else {
			fen_board.push_str(&self.en_passant_target_sq.unwrap().to_string());
		}
		
		fen_board.push_str(" ");
		fen_board.push_str(&self.halfmove_clock.to_string());
		
		fen_board.push_str(" ");
		fen_board.push_str(&self.fullmove_number.to_string());
		
		return fen_board;
	}
	
	pub fn default() -> Self {
		let mut result = Board::new();
		result.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
		return result;
	}
	
	pub fn place_piece(&mut self, piece: Piece, coord: Coordinate) {
		// Checking bounds
		if coord.row > 7 || coord.col > 7 {
			panic!("invalid Coordinates {} {}", coord.row, coord.col);
		}
		
		self.board[coord.row as usize][coord.col as usize] = piece;
		
		// Add to the piece map
		match piece.color {
			Color::White => &mut self.white_pieces,
			Color::Black => &mut self.black_pieces,
		}
		.insert(coord.as_number(), piece);
	}
	
	pub fn draw(&self) {
		// function to draw the Board
		/*
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		*/
		
		let mut array_of_chars_for_pieces: &[&str; 6];
		
		for row in 0..8 {
			for col in 0..8 {
				let piece: Piece = self.board[row][col];
				
				if piece.color == Color::White {
					array_of_chars_for_pieces = &WHITE_PIECES;
				} else {
					array_of_chars_for_pieces = &BLACK_PIECES;
				}
				
				// match the Piece to the correct character
				use Pieces::*;
				match piece.breed {
					King => {
						print!("{} ", array_of_chars_for_pieces[0]);
					}
					
					Queen => {
						print!("{} ", array_of_chars_for_pieces[1]);
					}
					
					Rook => {
						print!("{} ", array_of_chars_for_pieces[2]);
					}
					
					Bishop => {
						print!("{} ", array_of_chars_for_pieces[3]);
					}
					
					Knight => {
						print!("{} ", array_of_chars_for_pieces[4]);
					}
					
					Pawn => {
						print!("{} ", array_of_chars_for_pieces[5]);
					}
					
					Empty => {
						print!("\x1b[39;49m.\x1b[0m ");
					}
				}
			}
			println!();
		}
	}
	
	pub fn diagonal_moves(&self, coord: Coordinate, color: Color) -> Vec<Coordinate> {
		// function to get all diagonal moves
		let mut result: Vec<Coordinate> = Vec::new();
		let mut piece: Piece;
		let (row, col) = (coord.row, coord.col);
		let (mut new_row, mut new_col): (i8, i8);
		let (mut left_up, mut left_down, mut right_up, mut right_down) = (true, true, true, true);
		
		for delta in 1..8 {
			// left up
			if left_up {
				new_row = row + delta;
				new_col = col - delta;
				
				if (new_row >= 0 && new_col >= 0) && (new_row < 8 && new_col < 8) {
					piece = self.board[new_row as usize][new_col as usize];
					
					if piece.breed != Pieces::Empty {
						if piece.color != color {
							result.push(coord!(new_row, new_col));
							left_up = false;
						} else if piece.color == color {
							left_up = false;
						}
					} else {
						result.push(coord!(new_row, new_col));
					}
				} else {
					left_up = false;
				}
			}
			
			// left down
			if left_down {
				new_row = row - delta;
				new_col = col - delta;
				
				if (new_row >= 0 && new_col >= 0) && (new_row < 8 && new_col < 8) {
					piece = self.board[new_row as usize][new_col as usize];
					
					if piece.breed != Pieces::Empty {
						if piece.color != color {
							result.push(coord!(new_row, new_col));
							left_down = false;
						} else if piece.color == color {
							left_down = false;
						}
					} else {
						result.push(coord!(new_row, new_col));
					}
				} else {
					left_down = false;
				}
			}
			
			// right up
			if right_up {
				new_row = row + delta;
				new_col = col + delta;
				
				if (new_row >= 0 && new_col >= 0) && (new_row < 8 && new_col < 8) {
					piece = self.board[new_row as usize][new_col as usize];
					
					if piece.breed != Pieces::Empty {
						if piece.color != color {
							result.push(coord!(new_row, new_col));
							right_up = false;
						} else if piece.color == color {
							right_up = false;
						}
					} else {
						result.push(coord!(new_row, new_col));
					}
				} else {
					right_up = false;
				}
			}
			
			// right down
			if right_down {
				new_row = row - delta;
				new_col = col + delta;
				
				if (new_row >= 0 && new_col >= 0) && (new_row < 8 && new_col < 8) {
					piece = self.board[new_row as usize][new_col as usize];
					
					if piece.breed != Pieces::Empty {
						if piece.color != color {
							result.push(coord!(new_row, new_col));
							right_down = false;
						} else if piece.color == color {
							right_down = false;
						}
					} else {
						result.push(coord!(new_row, new_col));
					}
				} else {
					right_down = false;
				}
			}
		}
		
		return result;
	}
	
	pub fn linear_moves(&self, coord: Coordinate, color: Color) -> Vec<Coordinate> {
		// function to get all linear moves
		let mut result: Vec<Coordinate> = Vec::new();
		let mut piece: Piece;
		let row = coord.row;
		let col = coord.col;
		let (mut new_row, mut new_col): (i8, i8);
		let (mut up, mut down, mut left, mut right) = (true, true, true, true);
		
		for delta in 1..8 {
			// up
			if up {
				new_row = row + delta;
				new_col = col;
				
				if (new_row >= 0 && new_col >= 0) && (new_row < 8 && new_col < 8) {
					piece = self.board[new_row as usize][new_col as usize];
					
					if piece.breed != Pieces::Empty {
						if piece.color != color {
							result.push(coord!(new_row, new_col));
							up = false;
						} else if piece.color == color {
							up = false;
						}
					} else {
						result.push(coord!(new_row, new_col));
					}
				} else {
					up = false;
				}
			}
			
			// down
			if down {
				new_row = row - delta;
				new_col = col;
				
				if (new_row >= 0 && new_col >= 0) && (new_row < 8 && new_col < 8) {
					piece = self.board[new_row as usize][new_col as usize];
					
					if piece.breed != Pieces::Empty {
						if piece.color != color {
							result.push(coord!(new_row, new_col));
							down = false;
						} else if piece.color == color {
							down = false;
						}
					} else {
						result.push(coord!(new_row, new_col));
					}
				} else {
					down = false;
				}
			}
			
			// left
			if left {
				new_row = row;
				new_col = col - delta;
				
				if (new_row >= 0 && new_col >= 0) && (new_row < 8 && new_col < 8) {
					piece = self.board[new_row as usize][new_col as usize];
					
					if piece.breed != Pieces::Empty {
						if piece.color != color {
							result.push(coord!(new_row, new_col));
							left = false;
						} else if piece.color == color {
							left = false;
						}
					} else {
						result.push(coord!(new_row, new_col));
					}
				} else {
					left = false;
				}
			}
			
			// right
			if right {
				new_row = row;
				new_col = col + delta;
				
				if (new_row >= 0 && new_col >= 0) && (new_row < 8 && new_col < 8) {
					piece = self.board[new_row as usize][new_col as usize];
					
					if piece.breed != Pieces::Empty {
						if piece.color != color {
							result.push(coord!(new_row, new_col));
							right = false;
						} else if piece.color == color {
							right = false;
						}
					} else {
						result.push(coord!(new_row, new_col));
					}
				} else {
					right = false;
				}
			}
		}
		
		return result;
	}
	
	pub fn modify_sq(&mut self, coord: Coordinate, new_piece: Piece) -> Option<Piece> {
		// Returns piece from square if there was a piece
		match self.get_piece(coord) {
			Some(piece) => {
				self.board[coord.row as usize][coord.col as usize] = new_piece;
				return Some(piece);
			}
			None => {
				self.board[coord.row as usize][coord.col as usize] = new_piece;
				return None;
			}
		}
	}
	
	pub fn apply_move(&mut self, starting: Coordinate, ending: Coordinate) -> Option<Piece> {
		// Returns the piece that was captured
		let piece = self.get_piece(starting);
		
		if piece.is_none() {
			return None;
		}
		
		let piece = piece.unwrap();
		let captured_piece = self.modify_sq(ending, piece);
		
		if piece.breed == Pieces::Pawn && (ending.row - starting.row).abs() == 2 {
			let inc = match piece.color {
				Color::White => -1,
				Color::Black => 1,
			};
			// Setting a target square as a square behind the pawn
			/*
			* . . .
			* . # .
			* . P .
			* P - Pawn that made a double move
			* # - Target square for en passant
			*/
			self.en_passant_target_sq = Some(coord!(ending.row + inc, ending.col).as_number());
		}
		
		// Modify the map of pieces
		match piece.color {
			Color::White => &mut self.white_pieces,
			Color::Black => &mut self.black_pieces,
		}
		.insert(ending.as_number(), piece);
		
		match piece.color {
			Color::White => &mut self.white_pieces,
			Color::Black => &mut self.black_pieces,
		}
		.remove(&(starting.as_number()));
		
		match captured_piece {
			Some(piece) => {
				match piece.color {
					Color::White => &mut self.black_pieces,
					Color::Black => &mut self.white_pieces,
				}
				.remove(&(ending.as_number()));
			},
			None => {}
		};
		
		// Invert a turn
        use Color::*;
        self.turn = match self.turn {
            White => Black,
            Black => White
        };
		
		return captured_piece;
	}
	
	pub fn get_king_coord(&self, color: Color) -> Option<Coordinate> {
		for (num, piece) in match color {
			Color::White => &self.white_pieces,
			Color::Black => &self.black_pieces,
		}.iter() {
			if piece.breed == Pieces::King {
				return Some(Coordinate::from_number(*num));
			}
		}
		
		return None;
	}
	
	fn filter_check_moves(
		&self,
		piece_coord: Coordinate,
		moves: Vec<Coordinate>,
	) -> Vec<Coordinate> {
		let mut clone_board = Board::new();
		clone_board.load_fen(&self.get_fen()[..]);
		let piece = self.board[piece_coord.row as usize][piece_coord.col as usize];
		let mut result: Vec<Coordinate> = Vec::new();
		
		for move_coord in moves {
			let captured = clone_board.apply_move(piece_coord, move_coord);
			if !clone_board.is_in_check(piece.color) {
				result.push(move_coord);
			}
			// Undo the move by swapping the pieces back
			clone_board.apply_move(move_coord, piece_coord);
			if captured != None {
				clone_board.place_piece(captured.unwrap(), move_coord);
			}
		}
		
		return result;
	}
	
	pub fn get_piece(&self, coord: Coordinate) -> Option<Piece> {
        // It returns None if the piece you are trying to get is empty, so there is no need to
        // check for a piece breed all the time if you can just check for None =D
		let piece = self.board[coord.row as usize][coord.col as usize];
		return match piece.breed {
			Pieces::Empty => None,
			_ => Some(piece),
		};
	}
	
	pub fn get_moves(&self, coord: Coordinate) -> Vec<i8> {
		/*
		56 57 58 59 60 61 62 63
		48 49 50 51 52 53 54 55
		40 41 42 43 44 45 46 47
		32 33 34 35 36 37 38 39
		24 25 26 27 28 29 30 31
		16 17 18 19 20 21 22 23
		08 09 10 11 12 13 14 15
		00 01 02 03 04 05 06 07
		*/
		
		let piece = self.get_piece(coord);
		let position = coord.as_number();
		let mut moves: Vec<i8> = Vec::new();
		
		match piece {
			Some(piece) => {
				use Pieces::*;
				match piece.breed {
					King => {
						/*
						+7 +8 +9
						-1  0 +1
						-9 -8 -7
						*/
						
						let mut check = |num: i8, coord: Coordinate, row_diff: i8, col_diff: i8| {
							let new_row = coord.row + row_diff;
							let new_col = coord.col + col_diff;
							
							if (new_row >= 0 && new_row <= 7) && (new_col >= 0 && new_col <= 7) {
								let on_way_piece = self.get_piece(coord!(new_row, new_col));
								if on_way_piece.is_none() || on_way_piece.unwrap().color != piece.color {
                                    moves.push(num);
								}
							}
						};
						
						check(position + 7, coord, -1, -1);
						check(position + 9, coord, -1, 1);
						check(position + 8, coord, -1, 0);
						check(position - 1, coord, 0, -1);
						check(position + 1, coord, 0, 1);
						check(position - 9, coord, 1, -1);
						check(position - 8, coord, 1, 0);
						check(position - 7, coord, 1, 1);
					}
					Queen => {
						moves.append(
							&mut self
							.linear_moves(coord, piece.color)
							.iter()
							.map(|x| x.as_number())
							.collect(),
						);
						moves.append(
							&mut self
							.diagonal_moves(coord, piece.color)
							.iter()
							.map(|x| x.as_number())
							.collect(),
						);
					}
					
					Rook => {
						moves = self
						.linear_moves(coord, piece.color)
						.iter()
						.map(|x| x.as_number())
						.collect();
					}
					
					Bishop => {
						moves = self
						.diagonal_moves(coord, piece.color)
						.iter()
						.map(|x| x.as_number())
						.collect();
					}
					
					Knight => {
						let mut check = |num: i8, coord: Coordinate, row_diff: i8, col_diff: i8| {
							let new_row = coord.row + row_diff;
							let new_col = coord.col + col_diff;
							
							if (new_row >= 0 && new_row <= 7) && (new_col >= 0 && new_col <= 7) {
								let on_way_piece = self.get_piece(coord!(new_row, new_col));
								if on_way_piece.is_none() || on_way_piece.unwrap().color != piece.color {
                                    moves.push(num);
								}
							}
						};
						
						check(position + 10, coord, -1, 2);
						check(position - 6, coord, 1, 2);
						check(position + 17, coord, -2, 1);
						check(position - 15, coord, 2, 1);
						check(position + 15, coord, -2, -1);
						check(position - 17, coord, 2, -1);
						check(position + 6, coord, -1, -2);
						check(position - 10, coord, 1, -2);
					}
					Pawn => {
						let starting_row = if piece.color == Color::White { 6 } else { 1 };
						let inc = if piece.color == Color::White { 1 } else { -1 };
						
						if coord.row == starting_row {
							// If the square in front of the pawn is empty, add a move
							if self
							.get_piece(Coordinate::from_number(position + inc * 8))
							.is_none()
							{
								moves.push(position + inc * 8);
								// If the second square in front of the pawn is empty, add a move
								if self.get_piece(Coordinate::from_number(position + inc * 16)).is_none() {
									moves.push(position + inc * 16);
								}
							}
						} else {
							// If the square in front of the pawn is empty, add a move
							if self
							.get_piece(Coordinate::from_number(position + inc * 8))
							.is_none()
							{
								moves.push(position + inc * 8);
							}
						}
						
						// Attacking moves
						match self.get_piece(Coordinate::from_number(position + inc * 7)) {
							Some(under_attack_piece) => {
								if under_attack_piece.color != piece.color {
									moves.push(position + inc * 7);
								}
							}
							None => {}
						}
						
						match self.get_piece(Coordinate::from_number(position + inc * 9)) {
							Some(under_attack_piece) => {
								if under_attack_piece.color != piece.color {
									moves.push(position + inc * 9);
								}
							}
							None => {}
						}
						
						// En passant
						if self.en_passant_target_sq != None {
							if position + inc * 7 == self.en_passant_target_sq.unwrap() {
								moves.push(position + inc * 7);
							}
							
							if position + inc * 9 == self.en_passant_target_sq.unwrap() {
								moves.push(position + inc * 9);
							}
						}
					}
					
					Empty => {}
				}
			}
			None => {}
		}
		
		return moves;
	}
	
	pub fn is_in_check(&self, color: Color) -> bool {
		let king_coord: Option<Coordinate> = self.get_king_coord(color);
		if king_coord.is_none() {
			return false;
		}
		
		for (coord, _) in match color {
			Color::White => self.black_pieces.iter(),
			Color::Black => self.white_pieces.iter(),
		} {
			if self
			.get_moves(Coordinate::from_number(*coord))
			.contains(&king_coord.unwrap().as_number())
			{
				return true;
			}
		}
		
		return false;
	}
	
	pub fn is_in_checkmate(&mut self, color: Color) -> bool {
		if self.get_king_coord(color) == None {
			return false;
		}
		
		for (num, _) in match color {
			Color::White => self.white_pieces.iter(),
			Color::Black => self.black_pieces.iter(),
		} {
			if self.get_moves(Coordinate::from_number(*num)).len() != 0 {
				return false;
			}
		}
		
		return true;
	}
}
