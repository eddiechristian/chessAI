use game_state;
use std::str;

pub fn get_bishop_moves(state: &game_state::GameState, piece_coord: Vec<u8>) -> Vec<String> {
    let mut can_move_here = true;
    let mut allowed_bishop_moves: Vec<String> = Vec::new();

    //down right
    let down_right = (1i8,1i8);

    //up right
    let up_right = (1i8,-1i8);

    //down left
    let down_left = (-1i8,1i8);

    //up left
    let up_left = (-1i8,-1i8);

    let mut move_piece_coord0 = piece_coord[0] as i8;
    let mut move_piece_coord1 = piece_coord[1] as i8;
    //down_right
    while true {
        move_piece_coord0 += down_right.0;
        move_piece_coord1 += down_right.1;
        if  (move_piece_coord0 as u8) < 'I' as u8  {
            if  (move_piece_coord1  as u8)  < '9' as u8 {
                let mut coord:Vec<u8> = Vec::new();
                coord.push(move_piece_coord0 as u8);
                coord.push(move_piece_coord1 as u8);
                if let Ok(playerat_opt) = state.get_player_color_at(coord.as_slice()) {
                    if let Some(playerAt) = playerat_opt {
                        if  playerAt == state.player_turn {
                            can_move_here = false;
                        }
                    }
                    if can_move_here {
                        let mut bishop_move_str = String::new();

                        bishop_move_str.push(piece_coord[0] as char);
                        bishop_move_str.push(piece_coord[1] as char);
                        bishop_move_str.push('-');
                        bishop_move_str.push(move_piece_coord0 as u8 as char);
                        bishop_move_str.push(move_piece_coord1 as u8 as char);

                        allowed_bishop_moves.push(bishop_move_str);
                    } else {
                        break;
                    }
                }

            } else {
                break;
            }
        } else {
            break;
        }
    }
    //up_right
    move_piece_coord0 = piece_coord[0] as i8;
    move_piece_coord1 = piece_coord[1] as i8;
    can_move_here = true;
    while true {
        move_piece_coord0 += up_right.0;
        move_piece_coord1 += up_right.1;
        can_move_here = true;
        if  (move_piece_coord0 as u8)  < 'I' as u8 {
            if  (move_piece_coord1 as u8)  > '0' as u8 {
                let mut coord:Vec<u8> = Vec::new();
                coord.push(move_piece_coord0 as u8);
                coord.push(move_piece_coord1 as u8);
                if let Ok(playerat_opt) = state.get_player_color_at(coord.as_slice()) {
                    if let Some(playerAt) = playerat_opt {
                        if  playerAt == state.player_turn {
                            can_move_here = false;
                        }
                    }
                    if can_move_here {
                        let mut bishop_move_str = String::new();

                        bishop_move_str.push(piece_coord[0] as char);
                        bishop_move_str.push(piece_coord[1] as char);
                        bishop_move_str.push('-');
                        bishop_move_str.push(move_piece_coord0 as u8 as char);
                        bishop_move_str.push(move_piece_coord1 as u8 as char);

                        allowed_bishop_moves.push(bishop_move_str);
                    } else {
                        break;
                    }
                }

            } else {
                break;
            }
        } else {
            break;
        }
    }
    //down_left
    move_piece_coord0 = piece_coord[0] as i8;
    move_piece_coord1 = piece_coord[1] as i8;
    can_move_here = true;
    while true {
        move_piece_coord0 += down_left.0;
        move_piece_coord1 += down_left.1;
        if  (move_piece_coord0 as u8)  >= 'A' as u8 {
            if  (move_piece_coord1 as u8)  < '9' as u8 {
                let mut coord:Vec<u8> = Vec::new();
                coord.push(move_piece_coord0 as u8);
                coord.push(move_piece_coord1 as u8);
                if let Ok(playerat_opt) = state.get_player_color_at(coord.as_slice()) {
                    if let Some(playerAt) = playerat_opt {
                        if  playerAt == state.player_turn {
                            can_move_here = false;
                        }
                    }
                    if can_move_here {
                        let mut bishop_move_str = String::new();

                        bishop_move_str.push(piece_coord[0] as char);
                        bishop_move_str.push(piece_coord[1] as char);
                        bishop_move_str.push('-');
                        bishop_move_str.push(move_piece_coord0 as u8 as char);
                        bishop_move_str.push(move_piece_coord1 as u8 as char);

                        allowed_bishop_moves.push(bishop_move_str);
                    } else {
                        break;
                    }
                }

            } else {
                break;
            }
        } else {
            break;
        }
    }
    //up_left
    move_piece_coord0 = piece_coord[0] as i8;
    move_piece_coord1 = piece_coord[1] as i8;
    can_move_here = true;
    while true {
        move_piece_coord0 += up_left.0;
        move_piece_coord1 += up_left.1;
        if  (move_piece_coord0 as u8)  >= 'A' as u8 {
            if  (move_piece_coord1 as u8)  > '0' as u8 {
                let mut coord:Vec<u8> = Vec::new();
                coord.push(move_piece_coord0 as u8);
                coord.push(move_piece_coord1 as u8);
                if let Ok(playerat_opt) = state.get_player_color_at(coord.as_slice()) {
                    if let Some(playerAt) = playerat_opt {
                        if  playerAt == state.player_turn {
                            can_move_here = false;
                        }
                    }
                    if can_move_here {
                        let mut bishop_move_str = String::new();

                        bishop_move_str.push(piece_coord[0] as char);
                        bishop_move_str.push(piece_coord[1] as char);
                        bishop_move_str.push('-');
                        bishop_move_str.push(move_piece_coord0 as u8 as char);
                        bishop_move_str.push(move_piece_coord1 as u8 as char);

                        allowed_bishop_moves.push(bishop_move_str);
                    } else {
                        break;
                    }
                } else {
                    panic!("could not find piece");
                }

            } else {
                break;
            }
        } else {
            break;
        }
    }
    allowed_bishop_moves
}
