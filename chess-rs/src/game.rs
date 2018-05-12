#[macro_use]
use lazy_static;

use std::borrow::BorrowMut;
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::collections::HashMap;

use std::fmt;
use std::fmt::Debug;
use std::marker::Sync;
use std::mem;
use std::slice;
use game_state;
use game_state::{GameState, PlayerColor, PieceType};
use knight;


lazy_static! {
    pub static ref en_passant_map: HashMap<&'static str, Vec<&'static str > > = {
        //maps coord of pawn to enabled last moves to check.
        //if pawn is at A5 and last move was "B7-B5"..then en-passant is enabled.
        let mut m = HashMap::new();
        m.insert("A5", vec!["B7-B5"]);
        m.insert("B5", vec!["A7-A5", "C7-C5"]);
        m.insert("C5", vec!["B7-B5", "D7-D5"]);
        m.insert("D5", vec!["C7-C5", "E7-E5"]);
        m.insert("E5", vec!["D7-D5", "F7-F5"]);
        m.insert("F5", vec!["E7-E5", "G7-G5"]);
        m.insert("G5", vec!["F7-F5", "H7-H5"]);
        m.insert("H5", vec!["G7-G5"]);

        m.insert("A4", vec!["B2-B4"]);
        m.insert("B4", vec!["A2-A4", "C2-C4"]);
        m.insert("C4", vec!["B2-B4", "D2-D4"]);
        m.insert("D4", vec!["C2-C4", "E2-E4"]);
        m.insert("E4", vec!["D2-D4", "F2-F4"]);
        m.insert("F4", vec!["E2-E4", "G2-G4"]);
        m.insert("G4", vec!["F2-F4", "H2-H4"]);
        m.insert("G4", vec!["G2-G4"]);
        m
    };
}


#[derive(Clone, Debug)]
pub struct Piece {
    piece_type: PieceType,
    piece_value: u32,
    current_coord: String,
    allowed_moves: Vec<String>,
}

impl Piece {
    pub fn calculate_new_moves(&mut self,
                        state: &game_state::GameState,
                        previous_player_move: &Vec<&str>) {
        {
            self.allowed_moves.clear();
            let piece_coord  = self.current_coord.clone();
            let piece_coord_bytes  = self.current_coord.clone().into_bytes();
            match self.piece_type {
                PieceType::WHITE_PAWN |
                PieceType::BLACK_PAWN => {
                    if let Some(moves_that_enable_en_passant_vec) = en_passant_map.get(&piece_coord[..]) {
                        let last_move = format!("{}-{}", previous_player_move[0] ,
                                previous_player_move[1] );
                        if moves_that_enable_en_passant_vec.contains(&&last_move[..]) {
                            let coord0 = &previous_player_move[0].to_string().into_bytes();
                            let coord1 = &previous_player_move[1].to_string().into_bytes();

                            let en_passant_move = format!("en-passant{}-{}{}",
                                    previous_player_move[1],
                                    match  self.piece_type {
                                        PieceType::WHITE_PAWN => '6',
                                        PieceType::BLACK_PAWN => '3',
                                        _ => ' '
                                    },
                                    ((coord0[0] + coord1[0]) / 2 ) as char,
                                    );
                            self.allowed_moves.push(en_passant_move.to_string());
                        }
                    }
                    if self.piece_type == PieceType::WHITE_PAWN {
                        let chess_moves = vec![
                            format!("{}{}" ,  piece_coord_bytes[0] as char, (piece_coord_bytes[1] as u8 + 2) as char ),
                            format!("{}{}" ,  piece_coord_bytes[0] as char, (piece_coord_bytes[1] as u8 + 1) as char ),
                            format!("{}{}" ,  (piece_coord_bytes[0] as u8 - 1) as char, (piece_coord_bytes[1] as u8 + 1) as char ),
                            format!("{}{}" ,  (piece_coord_bytes[0] as u8 + 1) as char, (piece_coord_bytes[1] as u8 + 1) as char ),
                        ];
                        for (index, chess_move) in chess_moves.iter().enumerate() {
                            if ((index == 0) && (piece_coord_bytes[1] == '2' as u8)) || (index > 0){
                                if let Ok(coloropt) = state.get_player_color_at(&chess_move.as_bytes()) {
                                    if let Some(player_color) = coloropt  {
                                        if (index > 1) && (player_color == PlayerColor::BLACK) {
                                            self.allowed_moves.push(chess_move.to_string());
                                        }
                                    } else if index < 2{
                                        self.allowed_moves.push(chess_move.to_string());
                                    }
                                }
                            }
                        }
                    } else {
                        let chess_moves = vec![
                            format!("{}{}" ,  piece_coord_bytes[0] as char, (piece_coord_bytes[1] as u8 - 2) as char ),
                            format!("{}{}" ,  piece_coord_bytes[0] as char, (piece_coord_bytes[1] as u8 - 1) as char ),
                            format!("{}{}" ,  (piece_coord_bytes[0] as u8 - 1) as char, (piece_coord_bytes[1] as u8 - 1) as char ),
                            format!("{}{}" ,  (piece_coord_bytes[0] as u8 + 1) as char, (piece_coord_bytes[1] as u8 - 1) as char ),
                        ];
                        for (index, chess_move) in chess_moves.iter().enumerate() {
                            if ((index == 0) && (piece_coord_bytes[1] == '7' as u8)) ||  (index > 0) {
                                if let Ok(coloropt) = state.get_player_color_at(&chess_move.as_bytes()) {
                                    if let Some(player_color) = coloropt  {
                                        if (index > 1) && (player_color == PlayerColor::WHITE) {
                                            self.allowed_moves.push(chess_move.to_string());
                                        }
                                    } else if index < 2{
                                        self.allowed_moves.push(chess_move.to_string());
                                    }
                                }
                            }
                        }

                    }//end black pawns
                },
                PieceType::WHITE_KING |
                PieceType::BLACK_KING=> {

                },
                PieceType::WHITE_QUEEN |
                PieceType::BLACK_QUEEN => {

                },
                PieceType::WHITE_ROOK |
                PieceType::BLACK_ROOK => {

                },
                PieceType::WHITE_BISHOP |
                PieceType::BLACK_BISHOP => {

                },
                PieceType::WHITE_KNIGHT |
                PieceType::BLACK_KNIGHT  => {
                    self.allowed_moves = knight::get_knight_moves(&state, piece_coord.into_bytes());
                },
                PieceType::NO_PIECE=> {}
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    pub pieces: BTreeMap<String, Box<Piece>>,
}

#[derive(Clone, Debug)]
pub struct Game {
    pub player_turn: PlayerColor,
    pub players: Vec<Player>,
    pub state: GameState,
    pub state_map: BTreeMap<String, String> //maps "H1" -> "WHITE_ROOK2"
}

pub fn get_mut_piece_at<'a>( game: &'a mut Game, coord: &str) -> Option<&'a mut  Piece> {

    if let Some(piece_str) = game.state_map.get(coord) {
        if &piece_str[0..1] == "W"{
            if let Some(mut piece_opt_box) = game.players[1].pieces.get_mut(piece_str) {
                Some(piece_opt_box.borrow_mut())
            } else {
                None
            }
        } else {
            if let Some(mut piece_opt_box) = game.players[0].pieces.get_mut(piece_str) {
                Some(piece_opt_box.borrow_mut())
            } else {
                None
            }
        }
    } else {
        None
    }

}

pub fn get_piece_at<'a>( game: &'a  Game, coord: &str) -> Option<&'a Piece> {

    if let Some(piece_str) = game.state_map.get(coord) {
        if &piece_str[0..1] == "W"{
            if let Some(piece_opt_box) = game.players[1].pieces.get(piece_str) {
                Some(piece_opt_box.borrow())
            } else {
                None
            }
        } else {
            if let Some(piece_opt_box) = game.players[0].pieces.get(piece_str) {
                Some(piece_opt_box.borrow())
            } else {
                None
            }
        }
    } else {
        None
    }

}

impl Game {
    pub fn calculate_new_moves(&mut self, previous_player_move: &Vec<&str>) {

        let playter_turn_index = {
            match self.player_turn {
                PlayerColor::WHITE => 1,
                PlayerColor::BLACK => 0
            }
        };

        let mut piece_to_remove = "".to_string();
        {
            let pieces: &mut BTreeMap<String, Box<Piece>> = &mut self.players[playter_turn_index].pieces;
            for (coord_string, piece) in pieces.iter_mut() {
                if piece.current_coord == previous_player_move[previous_player_move.len()-1] {
                    piece_to_remove = coord_string.clone();
                } else {
                    piece.calculate_new_moves(&self.state, previous_player_move);
                    println!("{:?} {:?} piece:{:?}",
                        match playter_turn_index {
                            1 => "WHITE",
                            _ => "BLACK"
                        },
                        coord_string,
                        piece);
                }
            }
        }
        self.players[playter_turn_index].pieces.remove(&piece_to_remove);
    }

    pub fn take_action(&mut self, game_move: &str) {
// 012345678901234567
//"D5-en-passantC6-C5"
        let (len_squares, squares, en_passant) = {
            if game_move.len() > 5{
                //castling or en-passant
                let ff = &game_move[3..13];
                if &game_move[3..13] == "en-passant" {
                    let mut v = Vec::new();
                    v.push(&game_move[0..2]);
                    v.push(&game_move[13..15]);
                    v.push(&game_move[16..]);
                    (3,v,true)
                } else {
                    let v = Vec::new();
                    (0,v,false)
                }
            } else {
                (2, game_move.split('-').collect(), false)
            }
        };

        println!("{:?}", squares);
        let from_val =  { get_mut_piece_at(self, squares[0]).unwrap().piece_value };
        let from_index =  self.state.get_index(squares[0].as_bytes()).unwrap();

        if let Ok (to_index) = self.state.get_index(squares[1].as_bytes()) {
            self.state.board.0[to_index] = from_val;
            self.state.board.0[from_index] = 0u32;
            if en_passant {
                if let Ok (en_passant_index) = self.state.get_index(squares[2].as_bytes()) {
                    self.state.board.0[en_passant_index] = 0u32;
                }
            }
        } else {
            panic!(format!("could not get index for {:?}",squares[1].as_bytes()));
        }
        {
            let to_piece = &mut get_mut_piece_at( self, squares[0]).unwrap();
            to_piece.current_coord = squares[1].to_string();
        }
        if let Some(state_map_val) = self.state_map.remove(squares[0]) {
            println!("state_map_val {:?}", state_map_val);
            self.state_map.insert(squares[len_squares-1].to_string(), state_map_val);
        } else {
            panic!(format!("could not remove state_map entry {:?}",squares[0]));
        }
        //println!("state_map_new_key {:?} val: {:?}", squares[1], self.state_map.get(squares[1]));
        self.state.id = self.state.board.calculate_id().into_bytes();
        self.player_turn = {
            match self.player_turn {
                PlayerColor::WHITE => PlayerColor::BLACK,
                PlayerColor::BLACK => PlayerColor::WHITE
            }
        };
        self.calculate_new_moves(&squares);
    }

    pub fn new()-> Game  {
        let mut white_pieces = BTreeMap::new();
        let mut black_pieces = BTreeMap::new();

        let mut state_map = BTreeMap::new();

        // let white_rook1 = Piece {
        //     piece_type: PieceType::WHITE_ROOK,
        //     piece_value: game_state::get_piece_integer(PieceType::WHITE_ROOK),
        //     current_coord: "A1".to_string(),
        //     allowed_moves: Vec::new(),
        //     pieces_affected: move_vecs::affected_pieces_map.get("A1").unwrap() as *const PieceAffected,
        // };
        // state_map.insert("A1".to_string(),"WHITE_ROOK1".to_string());
        //
        // let white_rook2 = Piece {
        //     piece_type: PieceType::WHITE_ROOK,
        //     piece_value: game_state::get_piece_integer(PieceType::WHITE_ROOK),
        //     current_coord: "H1".to_string(),
        //     allowed_moves: Vec::new(),
        //     pieces_affected: move_vecs::affected_pieces_map.get("H1").unwrap() as *const PieceAffected,
        // };
        // state_map.insert("H1".to_string(),"WHITE_ROOK2".to_string());
        //
        // let white_knight1 = Piece {
        //     piece_type: PieceType::WHITE_KNIGHT,
        //     piece_value: game_state::get_piece_integer(PieceType::WHITE_KNIGHT),
        //     current_coord: "B1".to_string(),
        //     allowed_moves: vec!["C3".to_string(), "A3".to_string()],
        //     pieces_affected: move_vecs::affected_pieces_map.get("B1").unwrap() as *const PieceAffected,
        // };
        // state_map.insert("B1".to_string(),"WHITE_KNIGHT1".to_string());
        //
        // let white_knight2 = Piece {
        //     piece_type: PieceType::WHITE_KNIGHT,
        //     piece_value: game_state::get_piece_integer(PieceType::WHITE_KNIGHT),
        //     current_coord: "G1".to_string(),
        //     allowed_moves: vec!["H3".to_string(), "F3".to_string()],
        //     pieces_affected: move_vecs::affected_pieces_map.get("G1").unwrap() as *const PieceAffected,
        // };
        // state_map.insert("G1".to_string(),"WHITE_KNIGHT2".to_string());
        //
        // let white_bishop1 = Piece {
        //     piece_type: PieceType::WHITE_BISHOP,
        //     piece_value: game_state::get_piece_integer(PieceType::WHITE_BISHOP),
        //     current_coord: "C1".to_string(),
        //     allowed_moves: Vec::new(),
        //     pieces_affected: move_vecs::affected_pieces_map.get("C1").unwrap() as *const PieceAffected,
        // };
        // state_map.insert("C1".to_string(),"WHITE_BISHOP1".to_string());
        //
        // let white_bishop2 = Piece {
        //     piece_type: PieceType::WHITE_BISHOP,
        //     piece_value: game_state::get_piece_integer(PieceType::WHITE_BISHOP),
        //     current_coord: "F1".to_string(),
        //     allowed_moves: Vec::new(),
        //     pieces_affected: move_vecs::affected_pieces_map.get("F1").unwrap() as *const PieceAffected,
        // };
        // state_map.insert("F1".to_string(),"WHITE_BISHOP2".to_string());
        //
        let white_queen = Piece {
            piece_type: PieceType::WHITE_QUEEN,
            piece_value: game_state::get_piece_integer(PieceType::WHITE_QUEEN),
            current_coord: "D1".to_string(),
            allowed_moves: Vec::new(),
        };
        state_map.insert("D1".to_string(),"WHITE_QUEEN".to_string());
        //
        // let white_king = Piece {
        //     piece_type: PieceType::WHITE_KING,
        //     piece_value: game_state::get_piece_integer(PieceType::WHITE_KING),
        //     current_coord: "E1".to_string(),
        //     allowed_moves: Vec::new(),
        //     pieces_affected: move_vecs::affected_pieces_map.get("E1").unwrap() as *const PieceAffected,
        // };
        // state_map.insert("E1".to_string(),"WHITE_KING".to_string());


        let white_pawn1 = Piece {
            piece_type: PieceType::WHITE_PAWN,
            piece_value: game_state::get_piece_integer(PieceType::WHITE_PAWN),
            current_coord: "A2".to_string(),
            allowed_moves: vec!["A3".to_string(), "A4".to_string()],
        };
        state_map.insert("A2".to_string(),"WHITE_PAWN1".to_string());
        //
        let white_pawn2 = Piece {
            piece_type: PieceType::WHITE_PAWN,
            piece_value: game_state::get_piece_integer(PieceType::WHITE_PAWN),
            current_coord: "B2".to_string(),
            allowed_moves: vec!["B3".to_string(), "B4".to_string()],
        };
        state_map.insert("B2".to_string(),"WHITE_PAWN2".to_string());

        let white_pawn3 = Piece {
            piece_type: PieceType::WHITE_PAWN,
            piece_value: game_state::get_piece_integer(PieceType::WHITE_PAWN),
            current_coord: "C2".to_string(),
            allowed_moves: vec!["C3".to_string(), "C4".to_string()],
        };
        state_map.insert("C2".to_string(),"WHITE_PAWN3".to_string());

        let white_pawn4 = Piece {
            piece_type: PieceType::WHITE_PAWN,
            piece_value: game_state::get_piece_integer(PieceType::WHITE_PAWN),
            current_coord: "D2".to_string(),
            allowed_moves: vec!["D3".to_string(), "D4".to_string()],
        };
        state_map.insert("D2".to_string(),"WHITE_PAWN4".to_string());

        let white_pawn5 = Piece {
            piece_type: PieceType::WHITE_PAWN,
            piece_value: game_state::get_piece_integer(PieceType::WHITE_PAWN),
            current_coord: "E2".to_string(),
            allowed_moves: vec!["E3".to_string(), "E4".to_string()],
        };
        state_map.insert("E2".to_string(),"WHITE_PAWN5".to_string());

        let white_pawn6 = Piece {
            piece_type: PieceType::WHITE_PAWN,
            piece_value: game_state::get_piece_integer(PieceType::WHITE_PAWN),
            current_coord: "F2".to_string(),
            allowed_moves: vec!["F3".to_string(), "F4".to_string()],
        };
        state_map.insert("F2".to_string(),"WHITE_PAWN6".to_string());

        let white_pawn7 = Piece {
            piece_type: PieceType::WHITE_PAWN,
            piece_value: game_state::get_piece_integer(PieceType::WHITE_PAWN),
            current_coord: "G2".to_string(),
            allowed_moves: vec!["G3".to_string(), "G4".to_string()],
        };
        state_map.insert("G2".to_string(),"WHITE_PAWN7".to_string());

        let white_pawn8 = Piece {
            piece_type: PieceType::WHITE_PAWN,
            piece_value: game_state::get_piece_integer(PieceType::WHITE_PAWN),
            current_coord: "H2".to_string(),
            allowed_moves: vec!["H3".to_string(), "H4".to_string()],
        };
        state_map.insert("H2".to_string(),"WHITE_PAWN8".to_string());

        // white_pieces.insert("WHITE_ROOK1".to_string(), Box::new(white_rook1));
        // white_pieces.insert("WHITE_ROOK2".to_string(), Box::new(white_rook2));
        // white_pieces.insert("WHITE_KNIGHT1".to_string(), Box::new(white_knight1));
        // white_pieces.insert("WHITE_KNIGHT2".to_string(), Box::new(white_knight2));
        // white_pieces.insert("WHITE_BISHOP1".to_string(), Box::new(white_bishop1));
        // white_pieces.insert("WHITE_BISHOP2".to_string(), Box::new(white_bishop2));
        white_pieces.insert("WHITE_QUEEN".to_string(), Box::new(white_queen));
        // white_pieces.insert("WHITE_KING".to_string(), Box::new(white_king));
        white_pieces.insert("WHITE_PAWN1".to_string(), Box::new(white_pawn1));
        white_pieces.insert("WHITE_PAWN2".to_string(), Box::new(white_pawn2));
        white_pieces.insert("WHITE_PAWN3".to_string(), Box::new(white_pawn3));
        white_pieces.insert("WHITE_PAWN4".to_string(), Box::new(white_pawn4));
        white_pieces.insert("WHITE_PAWN5".to_string(), Box::new(white_pawn5));
        white_pieces.insert("WHITE_PAWN6".to_string(), Box::new(white_pawn6));
        white_pieces.insert("WHITE_PAWN7".to_string(), Box::new(white_pawn7));
        white_pieces.insert("WHITE_PAWN8".to_string(), Box::new(white_pawn8));

        let black_pawn1 = Piece {
            piece_type: PieceType::BLACK_PAWN,
            piece_value: game_state::get_piece_integer(PieceType::BLACK_PAWN),
            current_coord: "A7".to_string(),
            allowed_moves: vec!["A6".to_string(), "A5".to_string()],
        };
        state_map.insert("B7".to_string(),"BLACK_PAWN1".to_string());

        let black_pawn2 = Piece {
            piece_type: PieceType::BLACK_PAWN,
            piece_value: game_state::get_piece_integer(PieceType::BLACK_PAWN),
            current_coord: "B7".to_string(),
            allowed_moves: vec!["B6".to_string(), "B5".to_string()],
        };
        state_map.insert("B7".to_string(),"BLACK_PAWN2".to_string());

        let black_pawn3 = Piece {
            piece_type: PieceType::BLACK_PAWN,
            piece_value: game_state::get_piece_integer(PieceType::BLACK_PAWN),
            current_coord: "C7".to_string(),
            allowed_moves: vec!["C6".to_string(), "C5".to_string()],
        };
        state_map.insert("C7".to_string(),"BLACK_PAWN3".to_string());

        let black_pawn4 = Piece {
            piece_type: PieceType::BLACK_PAWN,
            piece_value: game_state::get_piece_integer(PieceType::BLACK_PAWN),
            current_coord: "D7".to_string(),
            allowed_moves: vec!["D6".to_string(), "D5".to_string()],
        };
        state_map.insert("D7".to_string(),"BLACK_PAWN4".to_string());

        let black_pawn5 = Piece {
            piece_type: PieceType::BLACK_PAWN,
            piece_value: game_state::get_piece_integer(PieceType::BLACK_PAWN),
            current_coord: "E7".to_string(),
            allowed_moves: vec!["E6".to_string(), "E5".to_string()],
        };
        state_map.insert("E7".to_string(),"BLACK_PAWN5".to_string());

        let black_pawn6 = Piece {
            piece_type: PieceType::BLACK_PAWN,
            piece_value: game_state::get_piece_integer(PieceType::BLACK_PAWN),
            current_coord: "F7".to_string(),
            allowed_moves: vec!["F6".to_string(), "F5".to_string()],
        };
        state_map.insert("F7".to_string(),"BLACK_PAWN6".to_string());

        let black_pawn7 = Piece {
            piece_type: PieceType::BLACK_PAWN,
            piece_value: game_state::get_piece_integer(PieceType::BLACK_PAWN),
            current_coord: "G7".to_string(),
            allowed_moves: vec!["G6".to_string(), "G5".to_string()],
        };
        state_map.insert("G7".to_string(),"BLACK_PAWN7".to_string());

        let black_pawn8 = Piece {
            piece_type: PieceType::BLACK_PAWN,
            piece_value: game_state::get_piece_integer(PieceType::BLACK_PAWN),
            current_coord: "H7".to_string(),
            allowed_moves: vec!["H6".to_string(), "H5".to_string()],
        };
        state_map.insert("H7".to_string(),"BLACK_PAWN8".to_string());

        black_pieces.insert("BLACK_PAWN1".to_string(), Box::new(black_pawn1));
        black_pieces.insert("BLACK_PAWN2".to_string(), Box::new(black_pawn2));
        black_pieces.insert("BLACK_PAWN3".to_string(), Box::new(black_pawn3));
        black_pieces.insert("BLACK_PAWN4".to_string(), Box::new(black_pawn4));
        black_pieces.insert("BLACK_PAWN5".to_string(), Box::new(black_pawn5));
        black_pieces.insert("BLACK_PAWN6".to_string(), Box::new(black_pawn6));
        black_pieces.insert("BLACK_PAWN7".to_string(), Box::new(black_pawn7));
        black_pieces.insert("BLACK_PAWN8".to_string(), Box::new(black_pawn8));

        Game {
            player_turn: PlayerColor::WHITE,
            players: vec![
                Player {
                    pieces: black_pieces
                },
                Player {
                    pieces: white_pieces
                }
            ],
            state: GameState::new(),
            state_map:  state_map
        }

    }

}
