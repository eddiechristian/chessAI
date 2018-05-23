use game_state;
use std::str;

pub fn get_rook_moves(state: &game_state::GameState, piece_coord: Vec<u8>) -> Vec<String> {
    let mut can_move_here = true;
    let mut allowed_rook_moves: Vec<String> = Vec::new();

    //down
    let down = (0i8,1i8);

    //up
    let up = (0i8,-1i8);

    //left
    let left = (-1i8,0i8);

    //right
    let right = (1i8,0i8);

    let mut move_piece_coord0 = piece_coord[0] as i8;
    let mut move_piece_coord1 = piece_coord[1] as i8;
    //down
    while true {
        move_piece_coord0 += down.0;
        move_piece_coord1 += down.1;
        if  (move_piece_coord1  as u8)  < 'G' as u8 {
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
                    let mut rook_move_str = String::new();

                    rook_move_str.push(piece_coord[0] as char);
                    rook_move_str.push(piece_coord[1] as char);
                    rook_move_str.push('-');
                    rook_move_str.push(move_piece_coord0 as u8 as char);
                    rook_move_str.push(move_piece_coord1 as u8 as char);

                    allowed_rook_moves.push(rook_move_str);
                } else {
                    break;
                }
            }

        } else {
            break;
        }
    }
    //up
    move_piece_coord0 = piece_coord[0] as i8;
    move_piece_coord1 = piece_coord[1] as i8;
    can_move_here = true;
    while true {
        move_piece_coord0 += up.0;
        move_piece_coord1 += up.1;
        can_move_here = true;
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
                    let mut rook_move_str = String::new();

                    rook_move_str.push(piece_coord[0] as char);
                    rook_move_str.push(piece_coord[1] as char);
                    rook_move_str.push('-');
                    rook_move_str.push(move_piece_coord0 as u8 as char);
                    rook_move_str.push(move_piece_coord1 as u8 as char);

                    allowed_rook_moves.push(rook_move_str);
                } else {
                    break;
                }
            }

        } else {
            break;
        }
    }
    //left
    move_piece_coord0 = piece_coord[0] as i8;
    move_piece_coord1 = piece_coord[1] as i8;
    can_move_here = true;
    while true {
        move_piece_coord0 += left.0;
        move_piece_coord1 += left.1;
        can_move_here = true;
        if  (move_piece_coord0 as u8)  > 'A' as u8 {
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
                    let mut rook_move_str = String::new();

                    rook_move_str.push(piece_coord[0] as char);
                    rook_move_str.push(piece_coord[1] as char);
                    rook_move_str.push('-');
                    rook_move_str.push(move_piece_coord0 as u8 as char);
                    rook_move_str.push(move_piece_coord1 as u8 as char);

                    allowed_rook_moves.push(rook_move_str);
                } else {
                    break;
                }
            }

        } else {
            break;
        }
    }
    //right
    move_piece_coord0 = piece_coord[0] as i8;
    move_piece_coord1 = piece_coord[1] as i8;
    can_move_here = true;
    while true {
        move_piece_coord0 += right.0;
        move_piece_coord1 += right.1;
        can_move_here = true;
        if  (move_piece_coord0 as u8)  < 'I' as u8 {
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
                    let mut rook_move_str = String::new();

                    rook_move_str.push(piece_coord[0] as char);
                    rook_move_str.push(piece_coord[1] as char);
                    rook_move_str.push('-');
                    rook_move_str.push(move_piece_coord0 as u8 as char);
                    rook_move_str.push(move_piece_coord1 as u8 as char);

                    allowed_rook_moves.push(rook_move_str);
                } else {
                    break;
                }
            }

        } else {
            break;
        }
    }
    allowed_rook_moves
}
