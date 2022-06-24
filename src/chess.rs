use std::collections::HashMap;

// Arrays of ASCII symbols of chess pieces
// Sequence: King, Queen, Rook, Bishop, Knight, Pawn
const BLACK_PIECES: [&str; 6] = ["♔", "♕", "♖", "♗", "♘", "♙"];
const WHITE_PIECES: [&str; 6] = ["♚", "♛", "♜", "♝", "♞", "♟"];

#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[derive(Debug)]
pub struct Coordinate {
	pub row: i8,
	pub col: i8
}

// Macro to expand coord!(x, y) to Coordinate { row: x, col: y }
macro_rules! coord {
	($x:expr, $y:expr) => {
		Coordinate { row: $x, col: $y }
	}
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

#[allow(dead_code)]
impl Coordinate {
	pub fn to_string(&self) -> String {
		// Convert coordinate to string
		let mut result = String::new();
		// Use cols to convert to letters
		const COLS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
		result.push(COLS[self.col as usize]);
		result.push_str(&(1 + self.row).to_string());
		return result;
	}

	pub fn new(row: i8, col: i8) -> Coordinate {
		Coordinate { row, col }
	}
}

#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[derive(Debug)]
pub enum Pieces {
	King,
	Queen,
	Rook,
	Bishop,
	Knight,
	Pawn,
	Empty,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[derive(Debug)]
pub enum Color {
	Black,
	White
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[derive(Debug)]
pub struct Piece {
  pub breed: Pieces,
  pub color: Color
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Board {
	table: [[Piece; 8]; 8],
	pub turn: Color,
	pub white_pieces: Box<HashMap<Coordinate, Piece>>,
	pub black_pieces: Box<HashMap<Coordinate, Piece>>,
	pub castling_permissions: [bool; 4], // White-King, White-Queen, Black-King, Black-Queen
	pub en_passant_target_sq: Option<Coordinate>,
	halfmove_clock: i8,
	fullmove_number: i8,
}

#[allow(dead_code)]
impl Board {
	pub fn new() -> Board {
		// Create new board
		Board {
			table: [[piece!(Empty, Black); 8]; 8],
			turn: Color::White,
			white_pieces: Box::new(HashMap::new()),
			black_pieces: Box::new(HashMap::new()),
			castling_permissions: [false, false, false, false],
			en_passant_target_sq: None,
			halfmove_clock: 0,
			fullmove_number: 1,
		}
	}

	pub fn default() -> Board {
		// Create default board
		let mut board = Board::new();
		// Set pieces
		board.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
		return board;
	}
	
	pub fn clear(&mut self) {
		self.table = [[piece!(Empty, Black); 8]; 8];
		self.white_pieces.clear();
		self.black_pieces.clear();
		self.castling_permissions = [false, false, false, false];
		self.en_passant_target_sq = None;
		self.halfmove_clock = 0;
		self.fullmove_number = 1;
	}

	pub fn place_piece(&mut self, piece: Piece, crd: Coordinate) {
		// Place piece on board
		self.table[crd.row as usize][crd.col as usize] = piece;
		if piece.breed != Pieces::Empty {
			if piece.color == Color::White {
				self.white_pieces.insert(crd, piece);
			} else {
				self.black_pieces.insert(crd, piece);
			}
		}
	}

	pub fn load_fen(&mut self, fen: &str) {
		// Load FEN string into board
		// FEN https://en.wikipedia.org/wiki/Forsyth-Edwards_Notation

		// Split FEN string into parts
		let parts = fen.split(' ').collect::<Vec<&str>>();
		// Check if there are enough parts
		if parts.len() != 6 {
			panic!("Invalid FEN string");
		}
		
		let board = parts[0];
		let turn = parts[1];
		let castling = parts[2];
		let en_passant = parts[3];
		let halfmove_clock = parts[4];
		let fullmove_number = parts[5];

		// Clear board
		self.clear();

		// Load board
		let mut row = 7;
		let mut col = 0;

		for c in board.chars() {
			if c == '/' {
				row -= 1;
				col = 0;
			} else if c.is_digit(10) {
				col += c.to_digit(10).unwrap() as i8;
			} else {
				self.place_piece(match c {
					'K' => piece!(King, White),
					'Q' => piece!(Queen, White),
					'R' => piece!(Rook, White),
					'B' => piece!(Bishop, White),
					'N' => piece!(Knight, White),
					'P' => piece!(Pawn, White),
					'k' => piece!(King, Black),
					'q' => piece!(Queen, Black),
					'r' => piece!(Rook, Black),
					'b' => piece!(Bishop, Black),
					'n' => piece!(Knight, Black),
					'p' => piece!(Pawn, Black),
					 _  => piece!(Empty, White)
				}, coord!(row, col));

				col += 1;
			}
		}
	
		// Load turn
		self.turn = match turn {
			"w" => Color::White,
			"b" => Color::Black,
			_ => panic!("Invalid turn")
		};

		// Load castling permissions
		self.castling_permissions = [false, false, false, false];
		if castling != "-" {
		for c in castling.chars() {
				match c {
					'K' => self.castling_permissions[0] = true,
					'Q' => self.castling_permissions[1] = true,
					'k' => self.castling_permissions[2] = true,
					'q' => self.castling_permissions[3] = true,
					_ => panic!("Invalid castling permissions")
				}
			}
		}

		// Load en passant
		let coord: Option<Coordinate>;

		if en_passant == "-" {
			coord = None;
		} else {
			// Column is in ASCII, so convert to numeric
			let col = en_passant.chars().nth(0).unwrap() as i8 - 'a' as i8;
			// Row is in ASCII, so convert to numeric
			let row = en_passant.chars().nth(1).unwrap() as i8 - '1' as i8;
			coord = Some(coord!(row, col));
		}
		self.en_passant_target_sq = coord;

		// Load halfmove clock
		self.halfmove_clock = halfmove_clock.parse::<i8>().unwrap();

		// Load fullmove number
		self.fullmove_number = fullmove_number.parse::<i8>().unwrap();
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
				// Taking piece but inverting row
				let piece = self.table[7 - row][col];
				
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

	pub fn diagonal_moves(&self, row: i8, col: i8, color: Color) -> Vec<Coordinate> {
		// function to get all diagonal moves
		let mut result: Vec<Coordinate> = Vec::new();
		let mut piece: Piece;
		let (mut new_row, mut new_col): (i8, i8);
		let (mut left_up, mut left_down, mut right_up, mut right_down) = (true, true, true, true);
		
		for delta in 1..8 {
			// left up
			if left_up {
				new_row = row + delta;
				new_col = col - delta;
				
				if (new_row >= 0 && new_col >= 0) && (new_row < 8 && new_col < 8) {
					piece = self.table[new_row as usize][new_col as usize];
					
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
					piece = self.table[new_row as usize][new_col as usize];
					
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
					piece = self.table[new_row as usize][new_col as usize];
					
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
					piece = self.table[new_row as usize][new_col as usize];
					
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
	
	pub fn linear_moves(&self, row: i8, col: i8, color: Color) -> Vec<Coordinate> {
		// function to get all linear moves
		let mut result: Vec<Coordinate> = Vec::new();
		let mut piece: Piece;
		let (mut new_row, mut new_col): (i8, i8);
		let (mut up, mut down, mut left, mut right) = (true, true, true, true);
		
		for delta in 1..8 {
			// up
			if up {
				new_row = row + delta;
				new_col = col;
				
				if (new_row >= 0 && new_col >= 0) && (new_row < 8 && new_col < 8) {
					piece = self.table[new_row as usize][new_col as usize];
					
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
					piece = self.table[new_row as usize][new_col as usize];
					
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
					piece = self.table[new_row as usize][new_col as usize];
					
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
					piece = self.table[new_row as usize][new_col as usize];
					
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

	pub fn get_moves(&self, coord: Coordinate) -> Vec<Coordinate> {
		// function to get all valid moves
		let mut result: Vec<Coordinate> = Vec::new();
		let piece = self.table[coord.row as usize][coord.col as usize];

		use Pieces::*;
		match piece.breed {
			King => {
				/*
					(-1, +1) (0, +1) (+1, +1)
					(-1, 0)     K    (+1, 0)
					(-1, -1) (0, -1) (+1, -1)
				*/

				result.push(coord!(coord.row - 1, coord.col + 1));
				result.push(coord!(coord.row, coord.col + 1));
				result.push(coord!(coord.row + 1, coord.col + 1));
				result.push(coord!(coord.row - 1, coord.col));
				result.push(coord!(coord.row + 1, coord.col));
				result.push(coord!(coord.row - 1, coord.col - 1));
				result.push(coord!(coord.row, coord.col - 1));
				result.push(coord!(coord.row + 1, coord.col - 1));
			},
			Queen => {
				result.append(&mut self.diagonal_moves(coord.row, coord.col, piece.color));
				result.append(&mut self.linear_moves(coord.row, coord.col, piece.color));
			},
			Rook => {
				result.append(&mut self.linear_moves(coord.row, coord.col, piece.color));
			},
			Bishop => {
				result.append(&mut self.diagonal_moves(coord.row, coord.col, piece.color));
			},
			Knight => {
				/*
					. * . * .
					* . . . *
					. . K . .
					* . . . *
					. * . * .
				*/

				result.push(coord!(coord.row - 2, coord.col + 1));
				result.push(coord!(coord.row - 1, coord.col + 2));
				result.push(coord!(coord.row + 1, coord.col + 2));
				result.push(coord!(coord.row + 2, coord.col + 1));
				result.push(coord!(coord.row + 2, coord.col - 1));
				result.push(coord!(coord.row + 1, coord.col - 2));
				result.push(coord!(coord.row - 1, coord.col - 2));
				result.push(coord!(coord.row - 2, coord.col - 1));
			},
			Pawn => {
				let start_row = if piece.color == Color::White { 1 } else { 6 };
				let inc: i8 = if piece.color == Color::White { 1 } else { -1 };

				// Check if the pawn can move forward
				if self.table[(coord.row + inc) as usize][coord.col as usize].breed == Empty {
					result.push(coord!(coord.row + inc, coord.col));
					
					// Check if the pawn can move forward two squares
					if coord.row == start_row {
						if self.table[(coord.row + 2 * inc) as usize][coord.col as usize].breed == Empty {
							result.push(coord!(coord.row + 2 * inc, coord.col));
						}
					}
				}

				// Check if the pawn can move diagonally
				let diag_left_piece = self.table[(coord.row + inc) as usize][(coord.col - 1) as usize];
				let diag_right_piece = self.table[(coord.row + inc) as usize][(coord.col + 1) as usize];

				if diag_left_piece.breed != Empty && diag_left_piece.color != piece.color {
					result.push(coord!(coord.row + inc, coord.col - 1));
				}

				if diag_right_piece.breed != Empty && diag_right_piece.color != piece.color {
					result.push(coord!(coord.row + inc, coord.col + 1));
				}

				// Check if the pawn can take enpassant
				if self.en_passant_target_sq != None {
					let en_passant_target_sq = self.en_passant_target_sq.unwrap();
					if en_passant_target_sq.row == coord.row + inc && en_passant_target_sq.col == coord.col {
						result.push(en_passant_target_sq);
					}
				}
			},
			Empty => {},
		}

		// Filter out out of bounds moves
		result = result.into_iter().filter(|&c| c.row >= 0 && c.row < 8 && c.col >= 0 && c.col < 8).collect();

		// Filter out moves that hit a non-empty piece with opposite color
		result = result.into_iter().filter(|&c| self.table[c.row as usize][c.col as usize].color != piece.color).collect();

		return result;
	}

	pub fn find_king(&self, color: Color) -> Option<Coordinate> {
		for row in 0..8 {
			for col in 0..8 {
				let piece = self.table[row][col];
				if piece.breed == Pieces::King && piece.color == color {
					return Some(coord!(row as i8, col as i8));
				}
			}
		}
		return None;
	}

	pub fn is_check(&self, color: Color) -> bool {
		let king_coord = self.find_king(color);
		if king_coord == None {
			return false;
		}

		let king_coord = king_coord.unwrap();

		// If amount of possible moves is not 0, then the king is not in check
		if self.get_moves(king_coord).len() != 0 {
			return false;
		}

		// Check if some enemy piece can attack the king
		for (coord, _) in match color {
			Color::White => self.black_pieces.iter(),
			Color::Black => self.white_pieces.iter(),
		} {
			if self.get_moves(*coord).contains(&king_coord) {
				return true;
			}
		}	

		return false;
	}
}