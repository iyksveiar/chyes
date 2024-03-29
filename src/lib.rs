use core::fmt;
use std::convert::{TryFrom, TryInto};
use std::{collections::HashMap, str::FromStr};

use tui::style::Style;
use tui::widgets::Widget;

// Sequence: King, Queen, Rook, Bishop, Knight, Pawn
// NOTE: Might be changable in the future, via a command line argument
pub(crate) const BLACK_PIECES: [char; 6] = ['♔', '♕', '♖', '♗', '♘', '♙'];
pub(crate) const WHITE_PIECES: [char; 6] = ['♚', '♛', '♜', '♝', '♞', '♟'];

macro_rules! coord {
  ($x:expr, $y:expr) => {
    Coordinate {
      row: $x, col: $y
    }
  };
}

// Coordinate struct
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Coordinate {
  row: u8,
  col: u8
}

impl TryInto<u8> for Coordinate {
  type Error = &'static str;

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

  fn try_into(self) -> Result<u8, Self::Error> {
    if self.row > 7 || self.col > 7 {
      Err("Coordinate out of bounds")
    } else {
      Ok(self.row * 8 + self.col)
    }
  }
}

impl TryFrom<u8> for Coordinate {
  type Error = &'static str;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    if value > 63 {
      Err("Coordinate out of bounds")
    } else {
      Ok(Coordinate {
        row: value / 8,
        col: value % 8
      })
    }
  }
}

impl FromStr for Coordinate {
  type Err = &'static str;

  fn from_str(notation: &str) -> Result<Self, Self::Err> {
    // a8 -> (0, 0)
    // h1 -> (7, 7)

    // Check if the notation is valid
    if notation.len() != 2 {
      return Err("Couldn't parse notation")
    }

    // Get row and column
    let (row, col) = match (
      (56 - notation.chars().nth(1).unwrap() as isize).try_into(),
      (notation.chars().next().unwrap() as isize - 97).try_into()
    ) {
      (Ok(row), Ok(col)) => (row, col),
      _ => return Err("Couldn't parse notation")
    };

    // Check if the column and row are valid
    if col > 7 || row > 7 {
      return Err("Provided notation is invalid")
    }

    Ok(coord!(row, col))
  }
}

impl fmt::Display for Coordinate {
  fn fmt(
    &self,
    f: &mut fmt::Formatter<'_>
  ) -> fmt::Result {
    // (0, 0) -> "a8"
    // (7, 7) -> "h1"
    write!(f, "{}{}", (self.col + 97) as char, (56 - self.row) as char)
  }
}

impl Coordinate {
  // We assume that the coordinate is valid, when we transform or use it
  // Except when we need to construct a new coordinate

  // (0, 0) is the top left corner of the board

  #[inline]
  pub fn is_valid(&self) -> bool { self.row <= 7 && self.col <= 7 }

  #[inline]
  pub fn from_notation(notation: &'static str) -> Result<Self, &'static str> {
    Self::from_str(notation)
  }

  #[inline]
  pub fn to_notation(&self) -> String { self.to_string() }
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub enum Color {
  Black = 0,
  White = 1
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub enum Pieces {
  King,
  Queen,
  Rook,
  Bishop,
  Knight,
  Pawn
}

impl std::ops::Index<Pieces> for [char; 6] {
  type Output = char;

  fn index(
    &self,
    index: Pieces
  ) -> &Self::Output {
    match index {
      Pieces::King => &self[0],
      Pieces::Queen => &self[1],
      Pieces::Rook => &self[2],
      Pieces::Bishop => &self[3],
      Pieces::Knight => &self[4],
      Pieces::Pawn => &self[5]
    }
  }
}

impl Pieces {
  pub fn to_unicode(
    &self,
    color: Color
  ) -> char {
    // Convert a piece to a unicode character
    match color {
      Color::Black => BLACK_PIECES[*self],
      Color::White => WHITE_PIECES[*self]
    }
  }
}

#[derive(Hash, Clone, Copy)]
pub struct Piece {
  breed: Pieces,
  color: Color
}

impl fmt::Display for Piece {
  fn fmt(
    &self,
    f: &mut fmt::Formatter<'_>
  ) -> fmt::Result {
    write!(f, "{}", self.breed.to_unicode(self.color))
  }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum CastlingSides {
  KingSide = 0,
  QueenSide = 1
}

#[derive(Clone)]
pub struct Board {
  turn:                 Color,
  pieces:               HashMap<Coordinate, Piece>,
  en_passant_target_sq: Option<Coordinate>,
  castling:             [[bool; 2]; 2], // [color][side]
  halfmove_clock:       u16,
  fullmove_number:      u16
}

impl Default for Board {
  fn default() -> Self { Self::default() }
}

impl Widget for Board {
  fn render(
    self,
    area: tui::layout::Rect,
    buf: &mut tui::buffer::Buffer
  ) {
    for row in 0..8 {
      for col in 0..8 {
        // Determine what is background color for given coordinate
        let color: tui::style::Color = if (row + col) % 2 == 0 {
          tui::style::Color::Rgb(161, 189, 203) // Light
        } else {
          tui::style::Color::Rgb(254, 255, 255) // Dark
        };

        // Draw piece if there is one
        let piece_as_str = match self.pieces.get(&coord!(row as u8, col as u8)) {
          Some(piece) => piece.to_string(),
          None => " ".to_string()
        };
        buf.set_string(
          area.x + col,
          area.y + row,
          piece_as_str,
          Style::default().fg(tui::style::Color::Black).bg(color)
        );
      }
    }
  }
}

impl fmt::Display for Board {
  fn fmt(
    &self,
    f: &mut fmt::Formatter<'_>
  ) -> fmt::Result {
    for row in 0..8 {
      for col in 0..8 {
        let coord = coord!(row, col);
        if let Some(piece) = self.pieces.get(&coord) {
          write!(f, "{}", piece)?;
        } else {
          write!(f, ".")?;
        }
      }
      writeln!(f)?;
    }

    Ok(())
  }
}

impl Board {
  pub fn default() -> Self {
    return Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
      .expect("Couldn't create default board")
  }

  pub fn load_fen(
    &mut self,
    fen: &str
  ) -> Result<(), &str> {
    // Source: https://en.wikipedia.org/wiki/forsyth%e2%80%93edwards_notation

    // Splitting the FEN string into 7 parts
    let (
      piece_placement,
      active_color,
      castling_availability,
      en_passant_target_square,
      halfmove_clock,
      fullmove_number
    ) = match fen.split(' ').collect::<Vec<_>>().as_slice() {
      [pp, ac, ca, ep, hc, fulln] => (*pp, *ac, *ca, *ep, *hc, *fulln),
      _ => panic!("The FEN string doesn't match the pattern")
    };

    self.reset();

    // Piece placement
    let mut row: u8 = 0;
    let mut col: u8 = 0;

    for c in piece_placement.chars() {
      if c == '/' {
        row += 1;
        col = 0;
      } else if c.is_ascii_digit() {
        col += c
          .to_digit(10)
          .expect("Couldn't parse digit in FEN string notation") as u8;
      } else {
        let color = if c.is_uppercase() {
          Color::White
        } else {
          Color::Black
        };

        use Pieces::*;
        let piece = match c.to_ascii_lowercase() {
          'k' => King,
          'q' => Queen,
          'r' => Rook,
          'b' => Bishop,
          'n' => Knight,
          'p' => Pawn,
          _ => return Err("Invalid piece while parsing FEN notation")
        };

        // Add the piece to the board and increment the column
        self.pieces.insert(
          coord!(row, col),
          Piece {
            breed: piece,
            color
          }
        );
        col += 1;
      }
    }

    // Active color
    self.turn = match active_color {
      "w" => Color::White,
      "b" => Color::Black,
      _ => return Err("Invalid active color while parsing FEN notation (only w/b)")
    };

    macro_rules! set_castling {
      ($color:ident, $side:ident) => {
        self.castling[Color::$color as usize][CastlingSides::$side as usize]
      };
    }

    // Castling availability
    for c in castling_availability.chars() {
      match c {
        'K' => set_castling!(White, KingSide) = true,
        'Q' => set_castling!(White, QueenSide) = true,
        'k' => set_castling!(Black, KingSide) = true,
        'q' => set_castling!(Black, QueenSide) = true,
        '-' => break,
        _ => {
          return Err(
            "Invalid castling availability notation while parsing FEN notation (K/Q/k/q/- are \
             avaivable)"
          )
        },
      }
    }

    // En passant target square
    if en_passant_target_square != "-" {
      self.en_passant_target_sq = Some(Coordinate::from_str(en_passant_target_square)?);
    }

    // Halfmove clock
    self.halfmove_clock = halfmove_clock
      .parse()
      .map_err(|_| ())
      .expect("Couldn't parse halfmove clock in FEN notation");

    // Fullmove number
    self.fullmove_number = fullmove_number
      .parse()
      .map_err(|_| ())
      .expect("Couldn't parse fullmove number in FEN notation");

    Ok(())
  }

  pub fn new() -> Self {
    Board {
      turn:                 Color::White,
      pieces:               HashMap::new(),
      en_passant_target_sq: None,

      // Set castling to true for both sides
      castling:        [[false; 2]; 2],
      halfmove_clock:  0,
      fullmove_number: 1
    }
  }

  pub fn reset(&mut self) {
    self.turn = Color::White;
    self.pieces.clear();
    self.en_passant_target_sq = None;
    self.castling = [[false; 2]; 2];
    self.halfmove_clock = 0;
    self.fullmove_number = 1;
  }

  pub fn from_fen(fen: &str) -> Result<Self, &str> {
    let mut board = Board::new();
    board.load_fen(fen).expect("Couldn't load FEN string");
    Ok(board)
  }

  pub fn get_fen(&self) -> String {
    // Source: https://en.wikipedia.org/wiki/forsyth%e2%80%93edwards_notation

    let mut fen = String::new();

    // Piece placement
    for row in 0..8 {
      let mut empty_squares_count = 0;

      for col in 0..8 {
        if let Some(piece) = self.pieces.get(&coord!(row, col)) {
          if empty_squares_count > 0 {
            fen.push_str(&empty_squares_count.to_string());
            empty_squares_count = 0;
          }

          use Pieces::*;
          let c = match piece.breed {
            King => 'K',
            Queen => 'Q',
            Rook => 'R',
            Bishop => 'B',
            Knight => 'N',
            Pawn => 'P'
          };

          // It is K/Q for white, and k/q for black
          fen.push(match piece.color {
            Color::White => c,
            Color::Black => c.to_ascii_lowercase()
          });
        } else {
          empty_squares_count += 1;
        }
      }

      if empty_squares_count > 0 {
        fen.push_str(&empty_squares_count.to_string());
      }

      if row < 7 {
        fen.push('/'); // New row
      }
    }

    fen.push(' ');

    // Active color
    fen.push(match self.turn {
      Color::White => 'w',
      Color::Black => 'b'
    });

    fen.push(' ');

    macro_rules! can_castle {
      ($color:ident, $side:ident) => {
        self.castling[Color::$color as usize][CastlingSides::$side as usize]
      };
    }

    // Castling availability
    if self.castling[0] == [false, false] && self.castling[1] == [false, false] {
      fen.push('-');
    } else {
      if can_castle!(White, KingSide) {
        fen.push('K');
      }

      if can_castle!(White, QueenSide) {
        fen.push('Q');
      }

      if can_castle!(Black, KingSide) {
        fen.push('k');
      }

      if can_castle!(Black, QueenSide) {
        fen.push('q');
      }
    }

    fen.push(' ');

    // En passant target square
    match self.en_passant_target_sq {
      Some(coord) => fen.push_str(&coord.to_string()),
      None => fen.push('-')
    };

    fen.push_str(format!(" {} {}", self.halfmove_clock, self.fullmove_number).as_str());

    fen
  }

  pub fn place_piece(
    &mut self,
    piece: Piece,
    coord: Coordinate
  ) -> Option<Piece> {
    let old_piece = coord.is_valid().then_some(self.get_piece(&coord)).unwrap();

    self.pieces.insert(coord, piece);

    old_piece
  }

  pub fn get_piece(
    &self,
    coordinate: &Coordinate
  ) -> Option<Piece> {
    let result = self.pieces.get(coordinate);

    result.copied()
  }

  pub fn draw(&self) {
    for row in 0..8 {
      for col in 0..8 {
        let piece = self.get_piece(&coord!(row, col));

        match piece {
          Some(piece) => {
            print!("{} ", piece.breed.to_unicode(piece.color));
          },
          None => print!(". ")
        }
      }
      println!();
    }
  }

  fn generate_pseudo_legal_moves(
    &self,
    coordinate: Coordinate
  ) -> Result<Vec<Coordinate>, &'static str> {
    let piece = self.get_piece(&coordinate);

    if piece.is_none() {
      return Ok(Vec::new())
    }

    let piece = piece.unwrap();
    let mut moves: Vec<Coordinate> = Vec::new();
    let numeric: u8 = coordinate.try_into()?;

    use Pieces::*;
    match piece.breed {
      King => {
        /*
         * -7 -8 -9
         * -1  0 +1
         * +7 +8 +9
         */

        for delta in [-9, -8, -7, -1, 1, 7, 8, 9].iter() {
          let new_numeric = numeric as i8 + delta;

          if !(0..=63).contains(&new_numeric) {
            continue
          }

          let new_coord = Coordinate::try_from(new_numeric as u8).expect("Invalid coordinate");

          if new_coord.is_valid()
            && (new_coord.row as i8 - coordinate.row as i8).abs() <= 1
            && (new_coord.col as i8 - coordinate.col as i8).abs() <= 1
          {
            let on_way_piece = self.get_piece(&new_coord);

            if on_way_piece.is_none() || on_way_piece.unwrap().color != piece.color {
              moves.push(new_coord);
            }
          }
        }
      },
      Queen => {
        /*
         * -7 -8 -9
         * -1  0 +1
         * +7 +8 +9
         */

        let mut deltas = vec![-9, -8, -7, -1, 1, 7, 8, 9];

        for depth in 1..8 {
          for delta in deltas.clone().iter() {
            let new_numeric = numeric as i8 + delta * depth;

            if !(0..=63).contains(&new_numeric) {
              continue
            }

            let new_coord = Coordinate::try_from(new_numeric as u8).expect("Invalid coordinate");

            if new_coord.is_valid() {
              if [-7, -9, 7, 9].contains(delta)
                && (new_coord.row as i8 - coordinate.row as i8).abs()
                  != (new_coord.col as i8 - coordinate.col as i8).abs()
              {
                continue
              }

              if [-8, -1, 1, 8].contains(delta)
                && (new_coord.row as i8 - coordinate.row as i8).abs() != 0
                && (new_coord.col as i8 - coordinate.col as i8).abs() != 0
              {
                continue
              }

              if let Some(on_way_piece) = self.get_piece(&new_coord) {
                if on_way_piece.color != piece.color {
                  moves.push(new_coord);
                }

                deltas.retain(|&x| x != *delta);
              } else {
                moves.push(new_coord);
              }
            }
          }
        }
      },
      Rook => {
        /*
         * -7 -8 -9
         * -1  0 +1
         * +7 +8 +9
         */

        let mut deltas = vec![-8, -1, 1, 8];

        for depth in 1..8 {
          for delta in deltas.clone().iter() {
            let new_numeric = numeric as i8 + delta * depth;

            if !(0..=63).contains(&new_numeric) {
              continue
            }

            let new_coord = Coordinate::try_from(new_numeric as u8).expect("Invalid coordinate");

            if new_coord.is_valid()
              && (new_coord.row == coordinate.row || new_coord.col == coordinate.col)
            {
              if let Some(on_way_piece) = self.get_piece(&new_coord) {
                if on_way_piece.color != piece.color {
                  moves.push(new_coord);
                }

                deltas.retain(|&x| x != *delta);
              } else {
                moves.push(new_coord);
              }
            }
          }
        }
      },
      Bishop => {
        /*
         * -7 -8 -9
         * -1  0 +1
         * +7 +8 +9
         */

        let mut deltas = vec![-9, -7, 7, 9];

        for depth in 1..8 {
          for delta in deltas.clone().iter() {
            let new_numeric = numeric as i8 + delta * depth;

            if !(0..=63).contains(&new_numeric) {
              continue
            }

            let new_coord = Coordinate::try_from(new_numeric as u8).expect("Invalid coordinate");

            if new_coord.is_valid()
              && (new_coord.row as i8 - coordinate.row as i8).abs()
                == (new_coord.col as i8 - coordinate.col as i8).abs()
            {
              if let Some(on_way_piece) = self.get_piece(&new_coord) {
                if on_way_piece.color != piece.color {
                  moves.push(new_coord);
                }

                deltas.retain(|&x| x != *delta);
              } else {
                moves.push(new_coord);
              }
            }
          }
        }
      },
      Knight => {
        let deltas = vec![-17, -15, -10, -6, 6, 10, 15, 17];

        for delta in deltas.iter() {
          let new_numeric = numeric as i8 + delta;

          if !(0..64).contains(&new_numeric) {
            continue
          }

          let new_coord = Coordinate::try_from(new_numeric as u8).expect("Invalid coordinate");

          if new_coord.is_valid()
            && (new_coord.row as i8 - coordinate.row as i8).abs() <= 2
            && (new_coord.col as i8 - coordinate.col as i8).abs() <= 2
          {
            if let Some(on_way_piece) = self.get_piece(&new_coord) {
              if on_way_piece.color != piece.color {
                moves.push(new_coord);
              }
            } else {
              moves.push(new_coord);
            }
          }
        }
      },
      Pawn => {
        let increment = if piece.color == Color::White { -1 } else { 1 };
        let starting_row = if piece.color == Color::White { 6 } else { 1 };

        let new_coord = coord!((coordinate.row as i8 + increment) as u8, coordinate.col);

        // Just straight moves
        if new_coord.is_valid() {
          let on_way_piece = self.get_piece(&new_coord);

          if on_way_piece.is_none() {
            moves.push(new_coord);

            if coordinate.row == starting_row {
              let new_coord = coord!((new_coord.row as i8 + increment) as u8, new_coord.col);

              if new_coord.is_valid() {
                let on_way_piece = self.get_piece(&new_coord);

                if on_way_piece.is_none() {
                  moves.push(new_coord);
                }
              }
            }
          }
        }

        // Attacking moves
        let move1_numeric = coordinate.row as i8 + increment;
        let move2_numeric = coordinate.row as i8 + increment;

        if (0..=63).contains(&move1_numeric) && (0..=7).contains(&(coordinate.col as i8 - 1)) {
          let move1 = coord!(move1_numeric as u8, coordinate.col - 1);

          if move1.is_valid() {
            let on_way_piece = self.get_piece(&move1);

            if on_way_piece.is_some() && on_way_piece.unwrap().color != piece.color {
              moves.push(move1);
            }

            if self.en_passant_target_sq.is_some() && self.en_passant_target_sq.unwrap() == move1 {
              moves.push(move1);
            }
          }
        }

        if (0..=63).contains(&move2_numeric) && (0..=7).contains(&(coordinate.col as i8 + 1)) {
          let move2 = coord!(move2_numeric as u8, coordinate.col + 1);

          if move2.is_valid() {
            let on_way_piece = self.get_piece(&move2);

            if on_way_piece.is_some() && on_way_piece.unwrap().color != piece.color {
              moves.push(move2);
            }

            if self.en_passant_target_sq.is_some() && self.en_passant_target_sq.unwrap() == move2 {
              moves.push(move2);
            }
          }
        }
      }
    }

    Ok(moves)
  }

  pub fn move_piece(
    &mut self,
    start: Coordinate,
    target: Coordinate
  ) -> Result<Option<Piece>, String> {
    let start_piece = self
      .get_piece(&start)
      .expect("No piece at start coordinate");

    let old_piece = self.get_piece(&target);

    if old_piece.is_some() && old_piece.unwrap().color == start_piece.color {
      return Err(String::from(
        "Trying to move to square with same color piece"
      ))
    }

    self.place_piece(start_piece, target);

    // Remove start piece from hashmap
    self.pieces.remove(&start);

    // Update en passant target square
    if
    /* piece is a pawn */
    start_piece.breed == Pieces::Pawn {
      if (start.row as i8 - target.row as i8).abs() == 2 {
        let increment = if start_piece.color == Color::White {
          -1
        } else {
          1
        };
        self.en_passant_target_sq = Some(coord!((target.row as i8 + increment) as u8, target.col));
      } else {
        self.en_passant_target_sq = None;
      }
    } else {
      self.en_passant_target_sq = None;
    }

    macro_rules! set_castling {
      ($color:ident, $side:ident) => {
        self.castling[Color::$color as usize][CastlingSides::$side as usize]
      };
    }

    // Update castling rights
    // If the king has moved
    if start_piece.breed == Pieces::King {
      if start_piece.color == Color::White {
        set_castling!(White, KingSide) = false;
        set_castling!(White, QueenSide) = false;
      } else {
        set_castling!(Black, KingSide) = false;
        set_castling!(Black, QueenSide) = false;
      }
    }

    // Checking if rook has moved
    if start_piece.breed == Pieces::Rook {
      if start_piece.color == Color::White {
        if start == coord!(7, 0) {
          self.castling[Color::White as usize][CastlingSides::QueenSide as usize] = false;
        } else if start == coord!(7, 7) {
          self.castling[Color::White as usize][CastlingSides::KingSide as usize] = false;
        }
      } else if start == coord!(0, 0) {
        self.castling[Color::Black as usize][CastlingSides::QueenSide as usize] = false;
      } else if start == coord!(0, 7) {
        self.castling[Color::Black as usize][CastlingSides::KingSide as usize] = false;
      }
    }

    // Promotion
    if start_piece.breed == Pieces::Pawn {
      let promotion_row = if start_piece.color == Color::White {
        0
      } else {
        7
      };

      if target.row == promotion_row {
        let queen = Piece {
          color: start_piece.color,
          breed: Pieces::Queen
        };
        self.place_piece(queen, target);
      }
    }

    // Incrementing clocks
    self.halfmove_clock += 1;
    if self.turn == Color::Black {
      self.fullmove_number += 1;
    }

    // Switching turn
    self.turn = if self.turn == Color::White {
      Color::Black
    } else {
      Color::White
    };

    Ok(old_piece)
  }

  pub fn generate_moves(
    &self,
    coord: Coordinate
  ) -> Result<Vec<Coordinate>, &'static str> {
    // Get pseudo-legal moves and filter out moves that would put the king in check
    let mut moves = self.generate_pseudo_legal_moves(coord)?;
    let color = self
      .get_piece(&coord)
      .expect("Trying to generate moves for empty square")
      .color;

    moves.retain(|&x| {
      let mut new_board = self.clone();
      new_board.move_piece(coord, x).unwrap();

      !new_board.is_in_check(color)
    });

    Ok(moves)
  }

  pub fn get_king_coord(
    &self,
    color: Color
  ) -> Option<Coordinate> {
    for (coord, piece) in self.pieces.iter() {
      if piece.breed == Pieces::King && piece.color == color {
        return Some(*coord)
      }
    }

    None
  }

  pub fn is_in_check(
    &self,
    color: Color
  ) -> bool {
    let king_coord = self.get_king_coord(color);

    if king_coord.is_none() {
      // No king - no check
      return false
    }

    for (coord, piece) in self.pieces.iter() {
      if piece.color != color && piece.breed != Pieces::King {
        let moves = self
          .generate_pseudo_legal_moves(*coord)
          .expect("Couldn't generate moves to check if the King is in check");

        if moves.contains(&king_coord.unwrap()) {
          return true
        }
      }
    }

    false
  }

  pub fn is_in_checkmate(
    &self,
    color: Color
  ) -> bool {
    if self.get_king_coord(color).is_none() {
      return false
    }

    for (coord, piece) in self.pieces.iter() {
      if piece.color == color && piece.breed != Pieces::King {
        let moves = self
          .generate_moves(*coord)
          .expect("Couldn't geenrate moves");

        if !moves.is_empty() {
          return false
        }
      }
    }

    true
  }
}

#[cfg(test)]
mod tests;
