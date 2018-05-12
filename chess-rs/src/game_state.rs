
use std::fmt;
use std::fmt::Debug;
use std::result::Result;
use std::slice;
use std::str;

use knight;

pub static NO_PIECE:     &'static char = &' '; //0

pub static WHITE_KING:   &'static char = &'♔'; //1
pub static WHITE_QUEEN:  &'static char = &'♕'; //2
pub static WHITE_ROOK:   &'static char = &'♖'; //3
pub static WHITE_BISHOP: &'static char = &'♗'; //4
pub static WHITE_KNIGHT: &'static char = &'♘'; //5
pub static WHITE_PAWN:   &'static char = &'♙'; //6

pub static BLACK_KING:   &'static char = &'♚'; //7
pub static BLACK_QUEEN:  &'static char = &'♛'; //8
pub static BLACK_ROOK:   &'static char = &'♜'; //9
pub static BLACK_BISHOP: &'static char = &'♝'; //10
pub static BLACK_KNIGHT: &'static char = &'♞'; //11
pub static BLACK_PAWN:   &'static char = &'♟'; //12

#[derive(Clone, Debug, PartialEq)]
pub enum PieceType {
    NO_PIECE,
    WHITE_KING,
    WHITE_QUEEN,
    WHITE_ROOK,
    WHITE_BISHOP,
    WHITE_KNIGHT,
    WHITE_PAWN,
    BLACK_KING,
    BLACK_QUEEN,
    BLACK_ROOK,
    BLACK_BISHOP,
    BLACK_KNIGHT,
    BLACK_PAWN
}

pub fn get_piece_integer(piece_type: PieceType ) -> u32 {
    match piece_type {
        PieceType::NO_PIECE => 0u32,
        PieceType::WHITE_KING => 1u32,
        PieceType::WHITE_QUEEN => 2u32,
        PieceType::WHITE_ROOK => 3u32,
        PieceType::WHITE_BISHOP => 4u32,
        PieceType::WHITE_KNIGHT => 5u32,
        PieceType::WHITE_PAWN => 6u32,
        PieceType::BLACK_KING => 7u32,
        PieceType::BLACK_QUEEN => 8u32,
        PieceType::BLACK_ROOK => 9u32,
        PieceType::BLACK_BISHOP => 10u32,
        PieceType::BLACK_KNIGHT => 11u32,
        PieceType::BLACK_PAWN => 12u32
    }
}

fn get_piece_type(piece_int: u32 ) -> PieceType {
    match piece_int {
        0u32  => PieceType::NO_PIECE,
        1u32  => PieceType::WHITE_KING,
        2u32  => PieceType::WHITE_QUEEN,
        3u32  => PieceType::WHITE_ROOK,
        4u32  => PieceType::WHITE_BISHOP,
        5u32  => PieceType::WHITE_KNIGHT,
        6u32  => PieceType::WHITE_PAWN,
        7u32  => PieceType::BLACK_KING,
        8u32  => PieceType::BLACK_QUEEN,
        9u32  => PieceType::BLACK_ROOK,
        10u32 => PieceType::BLACK_BISHOP,
        11u32 => PieceType::BLACK_KNIGHT,
        12u32 => PieceType::BLACK_PAWN,
        _     => PieceType::NO_PIECE
    }
}

pub static TOP_BORDER:  &'static str =    " ┏━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┓";

pub static MIDDLE_BORDER:  &'static str = " ┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫";

pub static BOTTOM_BORDER:  &'static str = " ┗━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┛";


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayerColor {
    BLACK,
    WHITE
}

#[derive(Clone, Copy)]
pub struct Board(pub [u32; 64]);

impl  Board{
    pub fn calculate_id(&self) -> String {
        let mut id = "".to_string();
        for (i, element) in self.0.iter().enumerate() {
            if *element != 0u32 {
                id = id + &format!("{:03}-{:03},", i, element);
            }
        }
        id
    }
}

impl Debug for Board {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "Board {:?}", &self.0 as &[u32])
     }
}

impl Default for Board {
    fn default() -> Board {
        Board([get_piece_integer(PieceType::WHITE_ROOK),
               get_piece_integer(PieceType::WHITE_KNIGHT),
               get_piece_integer(PieceType::WHITE_BISHOP),
               get_piece_integer(PieceType::WHITE_QUEEN),
               get_piece_integer(PieceType::WHITE_KING),
               get_piece_integer(PieceType::WHITE_BISHOP),
               get_piece_integer(PieceType::WHITE_KNIGHT),
               get_piece_integer(PieceType::WHITE_ROOK),
               get_piece_integer(PieceType::WHITE_PAWN),
               get_piece_integer(PieceType::WHITE_PAWN),
               get_piece_integer(PieceType::WHITE_PAWN),
               get_piece_integer(PieceType::WHITE_PAWN),
               get_piece_integer(PieceType::WHITE_PAWN),
               get_piece_integer(PieceType::WHITE_PAWN),
               get_piece_integer(PieceType::WHITE_PAWN),
               get_piece_integer(PieceType::WHITE_PAWN),
               0,   0,  0,  0,  0,  0,  0,  0,
               0,   0,  0,  0,  0,  0,  0,  0,
               0,   0,  0,  0,  0,  0,  0,  0,
               0,   0,  0,  0,  0,  0,  0,  0,
               get_piece_integer(PieceType::BLACK_PAWN),
               get_piece_integer(PieceType::BLACK_PAWN),
               get_piece_integer(PieceType::BLACK_PAWN),
               get_piece_integer(PieceType::BLACK_PAWN),
               get_piece_integer(PieceType::BLACK_PAWN),
               get_piece_integer(PieceType::BLACK_PAWN),
               get_piece_integer(PieceType::BLACK_PAWN),
               get_piece_integer(PieceType::BLACK_PAWN),
               get_piece_integer(PieceType::BLACK_ROOK),
               get_piece_integer(PieceType::BLACK_KNIGHT),
               get_piece_integer(PieceType::BLACK_BISHOP),
               get_piece_integer(PieceType::BLACK_QUEEN),
               get_piece_integer(PieceType::BLACK_KING),
               get_piece_integer(PieceType::BLACK_BISHOP),
               get_piece_integer(PieceType::BLACK_KNIGHT),
               get_piece_integer(PieceType::BLACK_ROOK)
            ])
    }
}

impl AsMut<[u32]> for Board {
    fn as_mut(&mut self) -> &mut [u32] {
        &mut self.0
    }
}

impl AsRef<[u32]> for Board {
    fn as_ref(&self) -> &[u32] {
        &self.0
    }
}

impl<'a> Into< &'a [u8]> for Board {
    fn into(self) -> &'a [u8] {
        return unsafe {
                    slice::from_raw_parts(self.0.as_ptr() as *const u8, 256)
                };
    }
}


#[derive(Clone, Debug)]
pub struct GameState {
    pub board: Board,
    pub id:  Vec<u8>,
    pub player_turn: PlayerColor,
}

impl PartialEq for GameState {
    fn eq(&self, other: &GameState) -> bool {
        self.id == other.id
    }
}
impl Eq for GameState {}

impl GameState {
    pub fn new()->GameState  {
        let board = Board::default();
        GameState {
            board: board,
            id: board.calculate_id().into_bytes(),
            player_turn: PlayerColor::BLACK,
        }

    }
    //
    // fn in_check(&self, piece: usize) -> Vec<String> {
    //     let mut allowed_check_moves: Vec<String> = Vec::new();
    //     allowed_check_moves
    // }

    pub fn get_player_color_at(&self, coord: &[u8]) -> Result< Option<PlayerColor>, String > {
        let piece_result = self.piece_type_at(coord);
        if let Ok(piece) = piece_result {
            match piece {
                PieceType::NO_PIECE   => Ok(None),
                PieceType::WHITE_KING |
                PieceType::WHITE_QUEEN |
                PieceType::WHITE_ROOK |
                PieceType::WHITE_BISHOP |
                PieceType::WHITE_KNIGHT |
                PieceType::WHITE_PAWN => Ok(Some(PlayerColor::WHITE)),
                _ => Ok(Some(PlayerColor::BLACK))
            }
        } else {
            Err(format!("fix this"))
        }
    }

    pub fn get_coord(&self, index: usize) ->  Result<Vec<u8>, String > {
        let mut coord_vec: Vec<u8> = Vec::new();
        if index > 63 {
            return Err(format!("index to high {}", index));
        }
        let coord0 = match index % 8 {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            4 => 'E',
            5 => 'F',
            6 => 'G',
            7 => 'H',
            _ => 'Z'
        };
        let coord1 = match index / 8 {
            0 => '1',
            1 => '2',
            2 => '3',
            3 => '4',
            4 => '5',
            5 => '6',
            6 => '7',
            7 => '8',
            _ => 'Z'
        };

        coord_vec.push(coord0 as u8);
        coord_vec.push(coord1 as u8);
        Ok(coord_vec)
    }

    pub fn get_index(&self, coord: &[u8]) ->  Result<usize, String > {
        let col:u8  = match coord[0] as char {
            'a'|'A' => 0,
            'b'|'B' => 1,
            'c'|'C' => 2,
            'd'|'D' => 3,
            'e'|'E' => 4,
            'f'|'F' => 5,
            'g'|'G' => 6,
            'h'|'H' => 7,
            _       => return Err(format!("Invalid col {}", coord[0]))
        };

        let row:u8  = match coord[1] as char {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _   => return Err(format!("Invalid row {}", coord[1]))
        };
        let index:usize = ((row * 8) + col) as usize;
        Ok(index)
    }

     fn generate_line(&self, line: u8) -> String {
        let mut  coord0:u8 = b'A';
        let mut line = format!("{}", line);
        for x in 1..9 {
            let coord = format!("{}{}",coord0 as char, line );

            line = match x {
                1 => line + "┃ " + self.char_piece_at(coord.as_bytes()).ok().unwrap().to_string().as_ref(),
                _ => line + " ┃ " + self.char_piece_at(coord.as_bytes()).ok().unwrap().to_string().as_ref(),
            };
            coord0 = coord0 + 1;
        }
        line = line + " ┃\n";
        line
    }

    fn char_piece_at(&self, coord: &[u8]) -> Result<&char, String > {

       let index = self.get_index(coord)?;

       match self.board.0[index] {
           0 => Ok(NO_PIECE),
           1 => Ok(WHITE_KING),
           2 => Ok(WHITE_QUEEN),
           3 => Ok(WHITE_ROOK),
           4 => Ok(WHITE_BISHOP),
           5 => Ok(WHITE_KNIGHT),
           6 => Ok(WHITE_PAWN),
           7 => Ok(BLACK_KING),
           8 => Ok(BLACK_QUEEN),
           9 => Ok(BLACK_ROOK),
           10 => Ok(BLACK_BISHOP),
           11 => Ok(BLACK_KNIGHT),
           12 => Ok(BLACK_PAWN),
           _ => Ok(NO_PIECE)
       }
    }

    fn piece_type_at(&self, coord: &[u8]) -> Result<PieceType, String > {

       let index = self.get_index(coord)?;

       match self.board.0[index] {
           0 => Ok(PieceType::NO_PIECE),
           1 => Ok(PieceType::WHITE_KING),
           2 => Ok(PieceType::WHITE_QUEEN),
           3 => Ok(PieceType::WHITE_ROOK),
           4 => Ok(PieceType::WHITE_BISHOP),
           5 => Ok(PieceType::WHITE_KNIGHT),
           6 => Ok(PieceType::WHITE_PAWN),
           7 => Ok(PieceType::BLACK_KING),
           8 => Ok(PieceType::BLACK_QUEEN),
           9 => Ok(PieceType::BLACK_ROOK),
           10 => Ok(PieceType::BLACK_BISHOP),
           11 => Ok(PieceType::BLACK_KNIGHT),
           12 => Ok(PieceType::BLACK_PAWN),
           _ => Ok(PieceType::NO_PIECE)
       }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let grid_line = "    a   b   c   d   e   f   g   h\n";
        let top_border = format!("{}\n", TOP_BORDER);
        let middle_border = format!("{}\n", MIDDLE_BORDER);
        let bottom_border = format!("{}\n", BOTTOM_BORDER);
        let line1 = self.generate_line(1);
        let line2 = self.generate_line(2);
        let line3 = self.generate_line(3);
        let line4 = self.generate_line(4);
        let line5 = self.generate_line(5);
        let line6 = self.generate_line(6);
        let line7 = self.generate_line(7);
        let line8 = self.generate_line(8);

        write!(f, "id:{}\n\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            str::from_utf8(&self.id).unwrap() , grid_line, top_border, line1, middle_border, line2, middle_border, line3, middle_border,
            line4, middle_border, line5, middle_border, line6, middle_border, line7,
            middle_border, line8, bottom_border)
    }
}
