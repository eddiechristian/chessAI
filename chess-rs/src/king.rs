use game_state;
use std::str;

pub fn get_king_moves(state: &game_state::GameState, piece_coord: Vec<u8>) -> Vec<String> {
    let mut can_move_here = true;
    let mut allowed_king_moves: Vec<String> = Vec::new();

    //down
    let down = (0i8,1i8);

    //up
    let up = (0i8,-1i8);

    //left
    let left = (-1i8,0i8);

    //right
    let right = (1i8,0i8);

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
    //down
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
                let mut king_move_str = String::new();

                king_move_str.push(piece_coord[0] as char);
                king_move_str.push(piece_coord[1] as char);
                king_move_str.push('-');
                king_move_str.push(move_piece_coord0 as u8 as char);
                king_move_str.push(move_piece_coord1 as u8 as char);

                allowed_king_moves.push(king_move_str);
            }
        }

    }
    //up
    move_piece_coord0 = piece_coord[0] as i8;
    move_piece_coord1 = piece_coord[1] as i8;

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
                let mut king_move_str = String::new();

                king_move_str.push(piece_coord[0] as char);
                king_move_str.push(piece_coord[1] as char);
                king_move_str.push('-');
                king_move_str.push(move_piece_coord0 as u8 as char);
                king_move_str.push(move_piece_coord1 as u8 as char);

                allowed_king_moves.push(king_move_str);
            }
        }

    }
    //left
    move_piece_coord0 = piece_coord[0] as i8;
    move_piece_coord1 = piece_coord[1] as i8;

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
                let mut king_move_str = String::new();

                king_move_str.push(piece_coord[0] as char);
                king_move_str.push(piece_coord[1] as char);
                king_move_str.push('-');
                king_move_str.push(move_piece_coord0 as u8 as char);
                king_move_str.push(move_piece_coord1 as u8 as char);

                allowed_king_moves.push(king_move_str);
            }
        }

    }
    //right
    move_piece_coord0 = piece_coord[0] as i8;
    move_piece_coord1 = piece_coord[1] as i8;

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
                let mut king_move_str = String::new();

                king_move_str.push(piece_coord[0] as char);
                king_move_str.push(piece_coord[1] as char);
                king_move_str.push('-');
                king_move_str.push(move_piece_coord0 as u8 as char);
                king_move_str.push(move_piece_coord1 as u8 as char);

                allowed_king_moves.push(king_move_str);
            }
        }
    }
    //down_right
    let mut move_piece_coord0 = piece_coord[0] as i8;
    let mut move_piece_coord1 = piece_coord[1] as i8;

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
                    let mut king_move_str = String::new();

                    king_move_str.push(piece_coord[0] as char);
                    king_move_str.push(piece_coord[1] as char);
                    king_move_str.push('-');
                    king_move_str.push(move_piece_coord0 as u8 as char);
                    king_move_str.push(move_piece_coord1 as u8 as char);

                    allowed_king_moves.push(king_move_str);
                }
            }
        }
    }
    //up_right
    move_piece_coord0 = piece_coord[0] as i8;
    move_piece_coord1 = piece_coord[1] as i8;

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
                    let mut king_move_str = String::new();

                    king_move_str.push(piece_coord[0] as char);
                    king_move_str.push(piece_coord[1] as char);
                    king_move_str.push('-');
                    king_move_str.push(move_piece_coord0 as u8 as char);
                    king_move_str.push(move_piece_coord1 as u8 as char);

                    allowed_king_moves.push(king_move_str);
                }
            }
        }
    }
    //down_left
    move_piece_coord0 = piece_coord[0] as i8;
    move_piece_coord1 = piece_coord[1] as i8;
    can_move_here = true;
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
                    let mut king_move_str = String::new();

                    king_move_str.push(piece_coord[0] as char);
                    king_move_str.push(piece_coord[1] as char);
                    king_move_str.push('-');
                    king_move_str.push(move_piece_coord0 as u8 as char);
                    king_move_str.push(move_piece_coord1 as u8 as char);

                    allowed_king_moves.push(king_move_str);
                }
            }
        }
    }
    //up_left
    move_piece_coord0 = piece_coord[0] as i8;
    move_piece_coord1 = piece_coord[1] as i8;
    can_move_here = true;
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
                    let mut king_move_str = String::new();

                    king_move_str.push(piece_coord[0] as char);
                    king_move_str.push(piece_coord[1] as char);
                    king_move_str.push('-');
                    king_move_str.push(move_piece_coord0 as u8 as char);
                    king_move_str.push(move_piece_coord1 as u8 as char);

                    allowed_king_moves.push(king_move_str);
                }
            }
        }
    }

    allowed_king_moves
}
