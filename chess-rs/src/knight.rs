use game_state;

pub fn get_knight_moves(state: &game_state::GameState, piece_coord: Vec<u8>) -> Vec<String> {
    println!("get_knight_moves player_turn {:?} ", state.player_turn);
    let mut can_move_here = true;
    let mut allowed_knight_moves: Vec<String> = Vec::new();
    let front_left_short0 = piece_coord[0] + 2;
    let front_left_short1 = piece_coord[1] + 1;
    if  front_left_short0  < 'I' as u8 {
        if  front_left_short1  < '9' as u8 {
            let mut coord:Vec<u8> = Vec::new();
            coord.push(front_left_short0 );
            coord.push(front_left_short1);
            if let Ok(playerat_opt) = state.get_player_color_at(coord.as_slice()) {
                if let Some(playerAt) = playerat_opt {
                    if  playerAt == state.player_turn {
                        can_move_here = false;
                    }
                }
                if can_move_here {
                    let mut knight_move_str = String::new();

                    knight_move_str.push(piece_coord[0] as char);
                    knight_move_str.push(piece_coord[1] as char);
                    knight_move_str.push('-');
                    knight_move_str.push(front_left_short0 as char);
                    knight_move_str.push(front_left_short1 as char);

                    allowed_knight_moves.push(knight_move_str);
                }
            }
        }
    }
    can_move_here = true;
    let front_right_short0 = piece_coord[0] - 2;
    let front_right_short1 = piece_coord[1] + 1;
    if  front_right_short0  >= 'A' as u8 {
        if  front_right_short1  < '9' as u8 {
            let mut coord:Vec<u8> = Vec::new();
            coord.push(front_right_short0 );
            coord.push(front_right_short1);
            if let Ok(playerat_opt) = state.get_player_color_at(&coord) {
                if let Some(playerAt) = playerat_opt {
                    if  playerAt == state.player_turn {
                        can_move_here = false;
                    }
                }
                if can_move_here {
                    let mut knight_move_str = String::new();

                    knight_move_str.push(piece_coord[0] as char);
                    knight_move_str.push(piece_coord[1] as char);
                    knight_move_str.push('-');
                    knight_move_str.push(front_right_short0 as char);
                    knight_move_str.push(front_right_short1 as char);

                    allowed_knight_moves.push(knight_move_str);
                }
            }
        }
    }
    can_move_here = true;
    let front_left_long0 = piece_coord[0] + 1;
    let front_left_long1 = piece_coord[1] + 2;
    if  front_left_long0  < 'I' as u8 {
        if  front_left_long1  < '9' as u8 {
            let mut coord:Vec<u8> = Vec::new();
            coord.push(front_left_long0 );
            coord.push(front_left_long1);
            if let Ok(playerat_opt) = state.get_player_color_at(&coord) {
                if let Some(playerAt) = playerat_opt {
                    if  playerAt == state.player_turn {
                        can_move_here = false;
                    }
                }
                if can_move_here {
                    let mut knight_move_str = String::new();

                    knight_move_str.push(piece_coord[0] as char);
                    knight_move_str.push(piece_coord[1] as char);
                    knight_move_str.push('-');
                    knight_move_str.push(front_left_long0 as char);
                    knight_move_str.push(front_left_long1 as char);

                    allowed_knight_moves.push(knight_move_str);
                }
            }
        }
    }
    can_move_here = true;
    let front_right_long0 = piece_coord[0] - 1;
    let front_right_long1 = piece_coord[1] + 2;
    if  front_right_long0  >= 'A' as u8 {
        if  front_right_long1  < '9' as u8 {
            let mut coord:Vec<u8> = Vec::new();
            coord.push(front_left_long0 );
            coord.push(front_left_long1);
            if let Ok(playerat_opt) = state.get_player_color_at(&coord) {
                if let Some(playerAt) = playerat_opt {
                    if  playerAt == state.player_turn {
                        can_move_here = false;
                    }
                }
                if can_move_here {
                    let mut knight_move_str = String::new();

                    knight_move_str.push(piece_coord[0] as char);
                    knight_move_str.push(piece_coord[1] as char);
                    knight_move_str.push('-');
                    knight_move_str.push(front_right_long0 as char);
                    knight_move_str.push(front_right_long1 as char);

                    allowed_knight_moves.push(knight_move_str);
                }
            }
        }
    }
    can_move_here = true;
    let back_right_long0 = piece_coord[0] - 1 ;
    let back_right_long1 = piece_coord[1] - 2;
    if  back_right_long0  >= 'A' as u8 {
        if  back_right_long1  > '0' as u8 {
            let mut coord:Vec<u8> = Vec::new();
            coord.push(back_right_long0 );
            coord.push(back_right_long1);
            if let Ok(playerat_opt) = state.get_player_color_at(&coord) {
                if let Some(playerAt) = playerat_opt {
                    if  playerAt == state.player_turn {
                        can_move_here = false;
                    }
                }
                if can_move_here {
                    let mut knight_move_str = String::new();

                    knight_move_str.push(piece_coord[0] as char);
                    knight_move_str.push(piece_coord[1] as char);
                    knight_move_str.push('-');
                    knight_move_str.push(back_right_long0 as char);
                    knight_move_str.push(back_right_long1 as char);

                    allowed_knight_moves.push(knight_move_str);
                }
            }
        }
    }
    can_move_here = true;
    let back_right_short0 = piece_coord[0] - 2;
    let back_right_short1 = piece_coord[1] - 1;
    if  back_right_short0  >= 'A' as u8 {
        if  back_right_short1  > '0' as u8 {
            let mut coord:Vec<u8> = Vec::new();
            coord.push(back_right_short0 );
            coord.push(back_right_short1);
            if let Ok(playerat_opt) = state.get_player_color_at(&coord) {
                if let Some(playerAt) = playerat_opt {
                    if  playerAt == state.player_turn {
                        can_move_here = false;
                    }
                }
                if can_move_here {
                    let mut knight_move_str = String::new();

                    knight_move_str.push(piece_coord[0] as char);
                    knight_move_str.push(piece_coord[1] as char);
                    knight_move_str.push('-');
                    knight_move_str.push(back_right_short0 as char);
                    knight_move_str.push(back_right_short1 as char);

                    allowed_knight_moves.push(knight_move_str);
                }
            }
        }
    }
    can_move_here = true;
    let back_left_long0 = piece_coord[0] + 1 ;
    let back_left_long1 = piece_coord[1] - 2;
    if  back_left_long0  < 'I' as u8 {
        if  back_left_long1  > '0' as u8 {
            let mut coord:Vec<u8> = Vec::new();
            coord.push(back_left_long0 );
            coord.push(back_left_long1);
            if let Ok(playerat_opt) = state.get_player_color_at(&coord) {
                if let Some(playerAt) = playerat_opt {
                    if  playerAt == state.player_turn {
                        can_move_here = false;
                    }
                }
                if can_move_here {
                    let mut knight_move_str = String::new();

                    knight_move_str.push(piece_coord[0] as char);
                    knight_move_str.push(piece_coord[1] as char);
                    knight_move_str.push('-');
                    knight_move_str.push(back_left_long0 as char);
                    knight_move_str.push(back_left_long1 as char);

                    allowed_knight_moves.push(knight_move_str);
                }
            }
        }
    }
    can_move_here = true;
    let back_left_short0 = piece_coord[0] + 2 ;
    let back_left_short1 = piece_coord[1] - 1;
    if  back_left_short0  < 'I' as u8 {
        if  back_left_short1  > '0' as u8 {
            let mut coord:Vec<u8> = Vec::new();
            coord.push(back_left_short0 );
            coord.push(back_left_short1);
            if let Ok(playerat_opt) = state.get_player_color_at(&coord) {
                if let Some(playerAt) = playerat_opt {
                    if  playerAt == state.player_turn {
                        can_move_here = false;
                    }
                }
                if can_move_here {
                    let mut knight_move_str = String::new();

                    knight_move_str.push(piece_coord[0] as char);
                    knight_move_str.push(piece_coord[1] as char);
                    knight_move_str.push('-');
                    knight_move_str.push(back_left_short0 as char);
                    knight_move_str.push(back_left_short1 as char);

                    allowed_knight_moves.push(knight_move_str);
                }
            }
        }
    }
    allowed_knight_moves
}
