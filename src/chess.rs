use std::collections::HashMap;

// Sequence: King, Queen, Rook, Bishop, Knight, Pawn
// NOTE: Might be changable in the future, via a command line argument
const BLACK_PIECES: [&str; 6] = ["♔", "♕", "♖", "♗", "♘", "♙"];
const WHITE_PIECES: [&str; 6] = ["♚", "♛", "♜", "♝", "♞", "♟"];

// Macro to expand coord!(x, y) to Coordinate { row: x, col: y }
macro_rules! coord {
  ($x:expr, $y:expr) => {
    Coordinate { row: $x, col: $y }
  };
}

// piece!(King, Black) -> Piece { breed: Pieces::King, color: Color::Black }
macro_rules! piece {
  ($piece:ident, $color:ident) => {
    Piece {
      breed: Pieces::$piece,
      color: Color::$color,
    }
  };
}

// Coordinate struct
// NOTE: I tnink that using separate struct for such a simple thing is a bit overkill
// And on top of that it slows down the program a bit (I think)
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
    /* Convert coordinate to string */

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
  pub white_pieces: Box<HashMap<Coordinate, Piece>>,
  pub black_pieces: Box<HashMap<Coordinate, Piece>>,
  pub en_passant_target_sq: Option<Coordinate>,
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

  pub fn reset(&mut self) {
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

  pub fn load_fen(
    &mut self,
    fen: &str,
  ) {
    // function to parse fen string
    // source: https://en.wikipedia.org/wiki/forsyth%e2%80%93edwards_notation

    let mut fen_array = fen.split(' ');
    let fen_board = fen_array.next().unwrap();
    let fen_turn = fen_array.next().unwrap();
    let fen_castling = fen_array.next().unwrap();
    let _fen_en_passant = fen_array.next().unwrap();
    let _fen_half_move = fen_array.next().unwrap(); // todo
    let _fen_full_move = fen_array.next().unwrap(); // todo

    self.reset(); // Reset the board before loading the fen

    // Change the turn
    self.turn = match fen_turn {
      "w" => Color::White,
      "b" => Color::Black,
      _ => panic!("invalid turn"),
    };

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
    self.castling_rights = [
      fen_castling.contains('K'),
      fen_castling.contains('Q'),
      fen_castling.contains('k'),
      fen_castling.contains('q'),
    ];
  }

  pub fn get_fen(&self) -> String {
    // function to convert the Board to FEN
    // source: https://en.wikipedia.org/wiki/Forsyth%e2%80%93Edwards_notation

    let mut fen_board = String::new();
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

          let chr = match piece.breed {
            King => 'k',
            Queen => 'q',
            Rook => 'r',
            Bishop => 'b',
            Knight => 'n',
            Pawn => 'p',
            _ => panic!("Invalid piece"),
          };

          fen_board.push(
            // If the piece is white, then uppercase the character
            match piece.color {
              White => chr.to_ascii_uppercase(),
              Black => chr,
            },
          );
        }
      }

      if empty_count > 0 {
        fen_board.push_str(&empty_count.to_string());
        empty_count = 0;
      }

      if i != 7 {
        fen_board.push('/');
      }
    }

    // Separated by spaces add info about the turn, castling, en passant, and halfmove clock
    fen_board.push(' ');

    fen_board.push(match self.turn {
      Color::White => 'w',
      Color::Black => 'b',
    });

    // TODO: check if castling is possible
    fen_board.push(' ');

    const CASTLING: [char; 4] = ['K', 'Q', 'k', 'q'];

    for i in 0..4 {
      if self.castling_rights[i] {
        fen_board.push(CASTLING[i]);
      }
    }

    fen_board.push(' ');

    if self.en_passant_target_sq == None {
      fen_board.push('-');
    } else {
      fen_board.push_str(&self.en_passant_target_sq.unwrap().to_string());
    }

    fen_board.push(' ');
    fen_board.push_str(&self.halfmove_clock.to_string());

    fen_board.push(' ');
    fen_board.push_str(&self.fullmove_number.to_string());

    return fen_board;
  }

  pub fn default() -> Self {
    let mut result = Board::new();
    result.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    return result;
  }

  pub fn place_piece(
    &mut self,
    piece: Piece,
    coord: Coordinate,
  ) {
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
    .insert(coord, piece);
  }

  pub fn draw(&self) {
    /* Function to draw the board */

    for row in 0..8 {
      for col in 0..8 {
        let piece: Piece = self.board[row][col];

        let pieces_str: &[&str; 6] = match piece.color {
          Color::White => &WHITE_PIECES,
          Color::Black => &BLACK_PIECES,
        };

        // match the Piece to the correct character
        use Pieces::*;
        print!(
          "{} ",
          match piece.breed {
            King => pieces_str[0],
            Queen => pieces_str[1],
            Rook => pieces_str[2],
            Bishop => pieces_str[3],
            Knight => pieces_str[4],
            Pawn => pieces_str[5],
            Empty => "\x1b[39;49m.\x1b[0m",
          }
        );
      }
      println!();
    }
  }

  pub fn get_diagonal_moves(
    &self,
    coord: Coordinate,
    color: Color,
  ) -> Vec<Coordinate> {
    /* Function to get all diagonal moves from given coordinate */

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

  pub fn get_linear_moves(
    &self,
    coord: Coordinate,
    color: Color,
  ) -> Vec<Coordinate> {
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

  pub fn modify_sq(
    &mut self,
    coord: Coordinate,
    new_piece: Piece,
  ) -> Option<Piece> {
    let old_piece = self.get_piece(coord);

    // Checking if the coordinate is valid
    if coord.row < 0 || coord.row > 7 || coord.col < 0 || coord.col > 7 {
      return None;
    }

    self.board[coord.row as usize][coord.col as usize] = new_piece;

    // Returns piece from square if there was a piece
    return match old_piece {
      Some(piece) => Some(piece),
      None => None,
    };
  }

  pub fn apply_move(
    &mut self,
    starting: Coordinate,
    ending: Coordinate,
  ) -> Option<Piece> {
    // Returns the piece that was captured
    let piece = self.get_piece(starting);

    if piece.is_none() {
      return None;
    }

    let piece = piece.unwrap();
    let captured_piece = self.modify_sq(ending, piece);
    self.modify_sq(
      starting,
      Piece {
        breed: Pieces::Empty,
        color: piece.color,
      },
    );

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

      self.en_passant_target_sq = Some(coord!(ending.row + inc, ending.col));
    }

    // Modify the map of pieces
    match piece.color {
      Color::White => &mut self.white_pieces,
      Color::Black => &mut self.black_pieces,
    }
    .insert(ending, piece);

    match piece.color {
      Color::White => &mut self.white_pieces,
      Color::Black => &mut self.black_pieces,
    }
    .remove(&starting);

    match captured_piece {
      Some(piece) => {
        match piece.color {
          Color::White => &mut self.black_pieces,
          Color::Black => &mut self.white_pieces,
        }
        .remove(&ending);
      },
      None => {},
    };

    // Invert a turn
    use Color::*;
    self.turn = match self.turn {
      White => Black,
      Black => White,
    };

    return captured_piece;
  }

  pub fn get_king_coord(
    &self,
    color: Color,
  ) -> Option<Coordinate> {
    for (coord, piece) in match color {
      Color::White => &self.white_pieces,
      Color::Black => &self.black_pieces,
    }
    .iter()
    {
      if piece.breed == Pieces::King {
        return Some(*coord);
      }
    }

    return None;
  }

  pub fn get_piece(
    &self,
    coord: Coordinate,
  ) -> Option<Piece> {
    // It returns None if the piece you are trying to get is empty, so there is no need to
    // check for a piece breed all the time if you can just check for None =D

    // Checking if the coordinate is valid
    if (coord.row >= 0 && coord.col >= 0) && (coord.row < 8 && coord.col < 8) {
      let piece = self.board[coord.row as usize][coord.col as usize];

      return match piece.breed {
        Pieces::Empty => None,
        _ => Some(piece),
      };
    }

    return None;
  }

  pub fn filter_check_moves(
    &self,
    moves: &mut Vec<Coordinate>,
    coord: Coordinate,
  ) {
    let piece = self.get_piece(coord).unwrap();
    let mut board_copy = Box::new(self.clone());

    // If the piece is king, you have to check all the moves
    // If the piece is not a king, it is enough to check only one move

    match piece.breed {
      Pieces::King => {
        let mut index = 0;

        while index < moves.len() {
          let move_coord = moves[index];

          let captured = board_copy.apply_move(coord, move_coord);

          if board_copy.is_in_check(piece.color) {
            moves.remove(index);
          } else {
            index += 1;
          }

          // Undo the move
          board_copy.apply_move(move_coord, coord);

          if captured.is_some() {
            board_copy.place_piece(captured.unwrap(), move_coord);
          }
        }
      },
      _ => {
        let move_coord = moves[0];

        board_copy.apply_move(coord, move_coord);

        if board_copy.is_in_check(piece.color) {
          moves.clear();
        }
      },
    }
  }

  pub fn get_moves(
    &self,
    coord: Coordinate,
  ) -> Vec<Coordinate> {
    let piece = self.get_piece(coord);

    return match piece {
      Some(piece) => {
        // Get default moves for the piece
        // Filter coordinates that hit friendly pieces
        // Create a copy of the board
        // Apply the move to the copy
        // Check if friendly king is in check
        // If it is, remove the move from the list

        let mut moves = Vec::new();

        use Pieces::*;
        match piece.breed {
          King => {
            let mut check_coord = |diff: i8| {
              let new_coord = Coordinate::from_number(coord.as_number() + diff);

              // Checking if new coorddinate is in the board
              if (new_coord.row >= 0 && new_coord.row <= 7)
                && (new_coord.col >= 0 && new_coord.col <= 7)
                && (new_coord.row - coord.row).abs() < 2
                && (new_coord.col - coord.col).abs() < 2
              {
                let on_way_piece = self.get_piece(new_coord);

                if on_way_piece.is_none() || on_way_piece.unwrap().color != piece.color
                // You hit an enemy piece
                {
                  moves.push(new_coord);
                }
              }
            };

            check_coord(1);
            check_coord(-1);
            check_coord(8);
            check_coord(-8);
            check_coord(9);
            check_coord(-9);
            check_coord(7);
            check_coord(-7);

            // Iterate over moves, apply them to the copy of the board
            // and check if the king is in check

            self.filter_check_moves(&mut moves, coord);

            // TODO: Castling
          },

          Queen => {
            moves.append(&mut self.get_diagonal_moves(coord, piece.color));
            moves.append(&mut self.get_linear_moves(coord, piece.color));

            self.filter_check_moves(&mut moves, coord);
          },

          Rook => {
            moves.append(&mut self.get_linear_moves(coord, piece.color));

            self.filter_check_moves(&mut moves, coord);
          },

          Bishop => {
            moves.append(&mut self.get_diagonal_moves(coord, piece.color));

            self.filter_check_moves(&mut moves, coord);
          },

          Knight => {
            let mut check_coord = |diff: i8| {
              let new_coord = Coordinate::from_number(coord.as_number() + diff);

              // Checking if new coorddinate is in the board
              if (new_coord.row >= 0 && new_coord.row <= 7)
                && (new_coord.col >= 0 && new_coord.col <= 7)
                && (new_coord.row - coord.row).abs() < 3
                && (new_coord.col - coord.col).abs() < 3
              {
                let on_way_piece = self.get_piece(new_coord);

                if on_way_piece.is_none() || on_way_piece.unwrap().color != piece.color
                // You hit an enemy piece
                {
                  moves.push(new_coord);
                }
              }
            };

            check_coord(10);
            check_coord(-6);
            check_coord(17);
            check_coord(-15);
            check_coord(15);
            check_coord(-17);
            check_coord(6);
            check_coord(-10);

            self.filter_check_moves(&mut moves, coord);
          },

          Pawn => {
            let starting_row = if piece.color == Color::White { 6 } else { 1 };
            let inc = if piece.color == Color::White { 1 } else { -1 };

            // Checking the sqaure in front of the pawn
            if self
              .get_piece(Coordinate::from_number(coord.as_number() + inc * 8))
              .is_none()
            {
              moves.push(Coordinate::from_number(coord.as_number() + inc * 8));

              // Checking if the pawn is in the starting row
              if coord.row == starting_row {
                // If the second square in front of the pawn is empty, add a move
                if self
                  .get_piece(Coordinate::from_number(coord.as_number() + inc * 16))
                  .is_none()
                {
                  moves.push(Coordinate::from_number(coord.as_number() + inc * 16));
                }
              }
            }

            // Attacking moves
            match self.get_piece(Coordinate::from_number(coord.as_number() + inc * 7)) {
              Some(under_attack_piece) => {
                if under_attack_piece.color != piece.color {
                  moves.push(Coordinate::from_number(coord.as_number() + inc * 7));
                }
              },
              None => {},
            }

            match self.get_piece(Coordinate::from_number(coord.as_number() + inc * 9)) {
              Some(under_attack_piece) => {
                if under_attack_piece.color != piece.color {
                  moves.push(Coordinate::from_number(coord.as_number() + inc * 9));
                }
              },
              None => {},
            }

            // En passant
            match self.en_passant_target_sq {
              Some(en_passant_target_sq) => {
                if Coordinate::from_number(coord.as_number() + inc * 7) == en_passant_target_sq {
                  moves.push(Coordinate::from_number(coord.as_number() + inc * 7));
                }

                if Coordinate::from_number(coord.as_number() + inc * 9) == en_passant_target_sq {
                  moves.push(Coordinate::from_number(coord.as_number() + inc * 9));
                }
              },
              None => {},
            }
          },

          Empty => {},
        };

        moves
      },
      None => Vec::new(),
    };
  }

  pub fn is_in_check(
    &self,
    color: Color,
  ) -> bool {
    let king_coord: Option<Coordinate> = self.get_king_coord(color);

    if king_coord.is_none() {
      // No king found on the board
      return false;
    }

    for (coord, _) in match color {
      Color::White => self.black_pieces.iter(),
      Color::Black => self.white_pieces.iter(),
    } {
      if self.get_moves(*coord).contains(&king_coord.unwrap()) {
        return true;
      }
    }

    return false;
  }

  pub fn is_in_checkmate(
    &mut self,
    color: Color,
  ) -> bool {
    // Get king coordinate
    let king_coord: Option<Coordinate> = self.get_king_coord(color);

    if king_coord.is_none() {
      // No king found on the board
      return false;
    }

    // If king not in the check, return false
    if !self.is_in_check(color) {
      return false;
    }

    // If king can move, return false
    if self.get_moves(king_coord.unwrap()).len() > 0 {
      return false;
    }

    // If any piece can block the check, return false
    for (coord, _) in match color {
      Color::White => self.white_pieces.iter(),
      Color::Black => self.black_pieces.iter(),
    } {
      // Get moves for each coordinate, if length > 0, return false
      if self.get_moves(*coord).len() > 0 {
        return false;
      }
    }

    return true;
  }
}
