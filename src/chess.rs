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
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Coordinate {
  pub row: u8,
  pub col: u8,
}

impl Coordinate {
  // We assume that the coordinate is valid, when we transform or use it
  // Except when we need to construct a new coordinate

  // (0, 0) is the top left corner of the board

  pub fn to_notation(&self) -> String {
    // (0, 0) -> "a8"
    // (7, 7) -> "h1"

    let mut notation = String::new();

    // Add the column
    notation.push((self.col + 97) as char);

    // Add the row
    notation.push((56 - self.row) as char);

    return notation;
  }

  pub fn from_notation(notation: String) -> Result<Self, ()> {
    // a8 -> (0, 0)
    // h1 -> (7, 7)

    // Check if the notation is valid
    if notation.len() != 2 {
      return Err(());
    }

    // Get the column
    let col = notation.chars().nth(0).unwrap() as u8 - 97;

    // Get the row
    let row = notation.chars().nth(1).unwrap() as u8 - 49;

    // Check if the column and row are valid
    if col > 7 || row > 7 {
      return Err(());
    }

    return Ok(coord!(row, col));
  }

  pub fn as_number(&self) -> u8 {
    // Converts a coordinate to a number
    /*
     * 00 01 02 03 04 05 06 07
     * 08 09 10 11 12 13 14 15
     * 16 17 18 19 20 21 22 23
     * 24 25 26 27 28 29 30 31
     * 32 33 34 35 36 37 38 39
     * 40 41 42 43 44 45 46 47
     * 48 49 50 51 52 53 54 55
     * 56 57 58 59 60 61 62 63
     */

    return self.row * 8 + self.col;
  }

  pub fn from_number(number: u8) -> Result<Self, ()> {
    // Converts a number to a coordinate
    if number > 63 {
      return Err(());
    }

    return Ok(coord!(number / 8, number % 8));
  }

  pub fn is_valid(&self) -> Result<(), ()> {
    // Check if the coordinate is valid
    if self.row > 7 || self.col > 7 {
      return Err(());
    }

    return Ok(());
  }
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
#[derive(Debug)]
pub enum Pieces {
  King,
  Queen,
  Rook,
  Bishop,
  Knight,
  Pawn,
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
#[derive(Debug)]
pub enum Color {
  Black,
  White,
}

#[derive(Hash, Clone, Copy)]
#[derive(Debug)]
pub struct Piece {
  pub breed: Pieces,
  pub color: Color,
}

#[derive(Clone)]
pub struct Board {
  pub turn: Color,
  pub pieces: HashMap<Coordinate, Piece>,
  pub en_passant_target_sq: Option<Coordinate>,
  pub castling: [bool; 4], // [Black King, Black Queen, White King, White Queen]
  pub halfmove_clock: u16,
  pub fullmove_number: u16,
}

#[allow(dead_code)]
impl Board {
  pub fn default() -> Self {
    let mut result = Board::new();
    result
      .load_fen(String::from(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
      ))
      .expect("Could not load FEN");
    return result;
  }

  pub fn new() -> Self {
    Board {
      turn: Color::White,
      pieces: HashMap::new(),
      en_passant_target_sq: None,
      castling: [true, true, true, true],
      halfmove_clock: 0,
      fullmove_number: 1,
    }
  }

  pub fn reset(&mut self) {
    self.turn = Color::White;
    self.pieces.clear();
    self.en_passant_target_sq = None;
    self.castling = [true, true, true, true];
    self.halfmove_clock = 0;
    self.fullmove_number = 1;
  }

  pub fn load_fen(
    &mut self,
    fen: String,
  ) -> Result<(), ()> {
    // Source: https://en.wikipedia.org/wiki/forsyth%e2%80%93edwards_notation

    // Splitting the FEN string into 7 parts
    let mut fen_parts = fen.split(" ");

    let piece_placement = fen_parts.next().ok_or(())?;
    let active_color = fen_parts.next().ok_or(())?;
    let castling_availability = fen_parts.next().ok_or(())?;
    let en_passant_target_square = fen_parts.next().ok_or(())?;
    let halfmove_clock = fen_parts.next().ok_or(())?;
    let fullmove_number = fen_parts.next().ok_or(())?;

    self.reset();

    // Piece placement
    let mut row = 0;
    let mut col = 0;

    for c in piece_placement.chars() {
      if c == '/' {
        row += 1;
        col = 0;
      } else if c.is_digit(10) {
        col += c.to_digit(10).unwrap() as u8;
      } else {
        let color = match c.is_uppercase() {
          true => Color::White,
          false => Color::Black,
        };

        let piece = match c.to_ascii_lowercase() {
          'k' => Pieces::King,
          'q' => Pieces::Queen,
          'r' => Pieces::Rook,
          'b' => Pieces::Bishop,
          'n' => Pieces::Knight,
          'p' => Pieces::Pawn,
          _ => return Err(()),
        };

        self.pieces.insert(
          coord!(row, col),
          Piece {
            breed: piece,
            color,
          },
        );
        col += 1;
      }
    }

    // Active color
    self.turn = match active_color {
      "w" => Color::White,
      "b" => Color::Black,
      _ => return Err(()),
    };

    // Castling availability
    for c in castling_availability.chars() {
      match c {
        'K' => self.castling[2] = true,
        'Q' => self.castling[3] = true,
        'k' => self.castling[0] = true,
        'q' => self.castling[1] = true,
        '-' => (),
        _ => return Err(()),
      }
    }

    // En passant target square
    if en_passant_target_square != "-" {
      self.en_passant_target_sq = Some(Coordinate::from_notation(
        en_passant_target_square.to_string(),
      )?);
    }

    // Halfmove clock
    self.halfmove_clock = halfmove_clock.parse().map_err(|_| ())?;

    // Fullmove number
    self.fullmove_number = fullmove_number.parse().map_err(|_| ())?;

    return Ok(());
  }

  pub fn get_fen(&self) -> String {
    // Source: https://en.wikipedia.org/wiki/forsyth%e2%80%93edwards_notation

    let mut fen = String::new();

    // Piece placement
    for row in 0..8 {
      let mut empty_squares = 0;

      for col in 0..8 {
        let coord = coord!(row, col);

        if self.pieces.contains_key(&coord) {
          if empty_squares > 0 {
            fen.push_str(&empty_squares.to_string());
            empty_squares = 0;
          }

          let piece = self.pieces.get(&coord).unwrap();

          let c = match piece.breed {
            Pieces::King => 'K',
            Pieces::Queen => 'Q',
            Pieces::Rook => 'R',
            Pieces::Bishop => 'B',
            Pieces::Knight => 'N',
            Pieces::Pawn => 'P',
          };

          fen.push(match piece.color {
            Color::White => c,
            Color::Black => c.to_ascii_lowercase(),
          });
        } else {
          empty_squares += 1;
        }
      }

      if empty_squares > 0 {
        fen.push_str(&empty_squares.to_string());
      }

      if row < 7 {
        fen.push('/');
      }
    }

    fen.push(' ');

    // Active color
    fen.push(match self.turn {
      Color::White => 'w',
      Color::Black => 'b',
    });

    fen.push(' ');

    // Castling availability
    if self.castling.iter().all(|&x| !x) {
      fen.push('-');
    } else {
      if self.castling[2] {
        fen.push('K');
      }

      if self.castling[3] {
        fen.push('Q');
      }

      if self.castling[0] {
        fen.push('k');
      }

      if self.castling[1] {
        fen.push('q');
      }
    }

    fen.push(' ');

    // En passant target square
    match self.en_passant_target_sq {
      Some(coord) => fen.push_str(&coord.to_notation()),
      None => fen.push('-'),
    };

    fen.push(' ');

    // Halfmove clock
    fen.push_str(&self.halfmove_clock.to_string());

    fen.push(' ');

    // Fullmove number
    fen.push_str(&self.fullmove_number.to_string());

    return fen;
  }

  pub fn place_piece(
    &mut self,
    piece: Piece,
    coord: Coordinate,
  ) -> Option<Piece> {
    // Checking if the coordinate is valid
    match coord.is_valid() {
      Ok(_) => (),
      Err(()) => panic!("Invalid coordinate"),
    }

    let old_piece = self.get_piece(&coord);

    self.pieces.insert(coord, piece);

    return old_piece;
  }

  pub fn get_piece(
    &self,
    coordinate: &Coordinate,
  ) -> Option<Piece> {
    let result = self.pieces.get(coordinate);

    return match result {
      Some(piece_ptr) => Some(piece_ptr.clone()),
      None => None,
    };
  }

  pub fn draw(&self) {
    for row in 0..8 {
      for col in 0..8 {
        let piece = self.get_piece(&coord!(row, col));

        match piece {
          Some(piece) => {
            let pieces_str: &[&str; 6] = match piece.color {
              Color::White => &WHITE_PIECES,
              Color::Black => &BLACK_PIECES,
            };

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
              }
            );
          },
          None => print!(". "),
        }
      }
      println!();
    }
  }

  fn get_moves_in_direction(
    &self,
    mut deltas: Vec<i8>,
    max_depth: u8,
    coord: Coordinate,
    coord_delta: u8, // TODO: Write explanation
    dynamic: bool, // TODO: Write explanation
  ) -> Vec<Coordinate> {
    /*
     * -9 -8 -7
     * -1  00 +1
     * +7 +8 +9
     */

    let color = self.get_piece(&coord).expect("Couldn't get piece").color;

    let mut moves = Vec::new();
    let numeric: u8 = coord.as_number(); // Numeric representation of the coordinate

    /*
     * Loop trough depth
     * Loop trough deltas, using while loop and index variable, so it is simple to remove elements
     * Generate the new coordinate
     * Validate the new coordinate, not in numerical form, so that we can check if the piece is on the board
     * Check what piece is standing on the new coordinate
     * If there is no piece, add the move
     * If there is a friendly piece, remove the delta from the deltas vector
     * If there is an enemy piece, add the move and remove the delta from the deltas vector
     */

    for depth in 1..=max_depth {
      let mut i = 0;

      while i < deltas.len() {
        let delta = deltas[i];
        let new_numeric = numeric as i8 + delta as i8 * depth as i8;

        if new_numeric < 0 || new_numeric > 63 {
          deltas.remove(i);
          continue;
        }

        let new_coord = Coordinate::from_number(new_numeric as u8).expect("Invalid coordinate");

        match new_coord.is_valid() {
          Ok(_) => (),
          Err(()) => {
            deltas.remove(i);
            continue;
          },
        }

        let check_coord_delta = if dynamic {
          depth
        } else {
          coord_delta
        };

        if (new_coord.row as i8 - coord.row as i8).abs() != check_coord_delta as i8 ||
           (new_coord.col as i8 - coord.col as i8).abs() != check_coord_delta as i8 {
          deltas.remove(i);
          continue;
        }

        let piece = self.get_piece(&new_coord);

        match piece {
          Some(piece) => {
            if piece.color == color {
              deltas.remove(i);
            } else {
              moves.push(new_coord);
              deltas.remove(i);
            }
          },
          None => moves.push(new_coord),
        }

        i += 1;
      }
    }

    return moves;
  }

  pub fn get_moves(
    &self,
    coordinate: Coordinate,
  ) -> Vec<Coordinate> {
    return match self.get_piece(&coordinate) {
      Some(piece) => {
        let linear_moves_deltas: Vec<i8> = vec![-1, 1, -8, 8];
        let diagonal_moves_deltas: Vec<i8> = vec![-9, -7, 7, 9];
        let all_moves_deltas: Vec<i8> = vec![-1, 1, -8, 8, -9, -7, 7, 9];

        let mut moves: Vec<Coordinate> = Vec::new();

        /*
         * Loop trough depth
         * Loop trough deltas, using while loop and index variable, so it is simple to remove elements
         * Generate the new coordinate
         * Validate the new coordinate, not in numerical form, so that we can check if the piece is on the board
         * Check what piece is standing on the new coordinate
         * If there is no piece, add the move
         * If there is a friendly piece, remove the delta from the deltas vector
         * If there is an enemy piece, add the move and remove the delta from the deltas vector
         */

        use Pieces::*;
        match piece.breed {
          King => {
            moves.append(&mut self.get_moves_in_direction(all_moves_deltas, 1, coordinate, 0, true));
          },
          Queen => {
            moves.append(&mut self.get_moves_in_direction(all_moves_deltas, 8, coordinate, 0, true));
          },
          Rook => {
            moves.append(&mut self.get_moves_in_direction(linear_moves_deltas, 8, coordinate, 0, true));
          },
          Bishop => {
            moves.append(&mut self.get_moves_in_direction(diagonal_moves_deltas, 8, coordinate, 0, true));
          },
          Knight => {
            moves.append(&mut self.get_moves_in_direction(vec![-17, -15, -10, -6, 6, 10, 15, 17], 1, coordinate, 3, false));
          },
          Pawn => {
            let mut deltas: Vec<i8> = Vec::new();

            if piece.color == Color::White {
              deltas.push(-8);
              if coordinate.row == 6 {
                deltas.push(-16);
              }
            } else {
              deltas.push(8);
              if coordinate.row == 1 {
                deltas.push(16);
              }
            }

            moves.append(&mut self.get_moves_in_direction(deltas, 1, coordinate, 0, true));
          },
        }

        moves
      },
      None => Vec::new(),
    };
  }

  pub fn get_king_coord(&self, color: Color) -> Option<Coordinate> {
    for (coord, piece) in &self.pieces {
      if piece.breed == Pieces::King && piece.color == color {
        return Some(coord.clone());
      }
    }

    return None;
  }

  #[allow(unused)]
  pub fn is_in_check(&self, color: Color) -> bool {
      // NOT IMPLEMENTED
      return false;
  }

  #[allow(unused)]
  pub fn is_in_checkmate(&self, color: Color) -> bool {
      // NOT IMPLEMENTED
      return false;
  }
}
