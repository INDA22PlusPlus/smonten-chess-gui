
/*




board structure: Use array, [[]]
enum to represent an arbitrary piece or empty; 
Utilize a simple threat_buffer that is recomputed every move

todo: Move generation
    - Representing moves
todo: Scheck mate


Implament a generate_moves_simple for each piece type
implament a move_check function that determines if the move is valid(simulate move and recompute threat buffer)

*/

#[allow(unused_parens)]
#[allow(dead_code)]
pub mod chess_api{
    use std::{slice, mem::swap};

    use self::{Util::construct_move_buffer, Move_util::get_move_list};
    
    pub const WHITE : bool = false;
    pub const BLACK : bool = true;
    pub const MAX_MOVES : usize = 32;

    #[derive(Clone, Copy)]
    pub struct Board_state{
        //state defining parameeters
        pub _board_types : [[Cell; 8]; 8],
        pub _board_hasmoved : [[bool; 8]; 8],
        pub _board_color : [[bool; 8]; 8],

        pub threat_buff : [[[bool; 8]; 8]; 2], //is king in scheck, only public for testing
        pub turn : bool, //0 is white, 1 is black
        castling: [bool; 2],
        
        pub king_pos : [(usize, usize); 2],
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Cell{
        King,
        Queen,
        Bishop,
        Rook,
        Knight,
        Pawn,
        None
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Move_type{
        Peassant,
        Castling,
        Capture,
        Promotion,
        Move
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Move{
        pub from : (i32, i32),
        pub to : (i32, i32),
        pub typ : Move_type,
        pub color : bool,
        pub promo_type : Cell, //Only used in the case of a promotion typ
    }

    pub mod Board{
        use std::{mem::swap, option};
        use super::*;
        
        //Essentially a board constructor
        pub fn create_init_board() -> Board_state{
            let mut B = Board_state{
                _board_types : [[Cell::None; 8]; 8],
                _board_hasmoved : [[false; 8]; 8],
                _board_color : [[false; 8]; 8],

                threat_buff : [[[false; 8]; 8]; 2],
                turn : false,
                castling : [false; 2],
                king_pos : [(0,0); 2]
            };

            B.king_pos[0] = (7,4);
            B.king_pos[1] = (0,4);
            // Why can't i do swap(&mut B.king_pos[0], &mut B.king_pos[1]) ? I guess B.king_pos becomes mutable in both?

            create_start_config(&mut B);
            update_threat_buffer(&mut B);

            return B;
        }

        fn create_start_config(board : &mut Board_state){
            create_start_config_color(board, false);
            create_start_config_color(board, true);
        }

        fn create_start_config_color(board : &mut Board_state, color : bool){
            let mut row:i32 = if(color as i32 == 0) {7} else {0};
            let row_dir = if(color as i32 == 0) {-1} else {1};

            let row1_layout = [
                Cell::Rook,
                Cell::Knight,
                Cell::Bishop,
                Cell::Queen,
                Cell::King,
                Cell::Bishop,
                Cell::Knight,
                Cell::Rook
            ];
    
            for i in 0..8{
                board._board_types[row as usize][i] = row1_layout[i];
                board._board_color[row as usize][i] = color;
            }
            row += row_dir;
            for i in 0..8{
                board._board_types[row as usize][i] = Cell::Pawn;
                board._board_color[row as usize][i] = color;
            }
        }

        pub fn update_threat_buffer(board : &mut Board_state){
            for row in 0..8 {
                for col in 0..8 {
                    let at = &board._board_types[row][col]; //Having implamented the copy trait
                    let at_color = board._board_color[row][col];
                    
                    match at{
                        Cell::Pawn => Pawn::generate_threat(row as i32, col as i32, board, at_color),
                        Cell::Knight => Knight::generate_threat(row as i32, col as i32, board, at_color),
                        Cell::Bishop => Bishop::generate_threat(row as i32, col as i32, board, at_color),
                        Cell::Rook => Rook::generate_threat(row as i32, col as i32, board, at_color),
                        Cell::Queen => Queen::generate_threat(row as i32, col as i32, board, at_color),
                        Cell::King => King::generate_threat(row as i32, col as i32, board, at_color),
                        Cell::None => continue
                    };
                }
            }
        }
    }

    pub fn scheck(board : &Board_state, color : bool) -> bool { //is king of color(color) in scheck?
        let (x,y) = board.king_pos[color as usize];
        return board.threat_buff[(!color) as usize][x][y];
    }

    pub fn is_scheck_mate(board : &mut Board_state, color : bool) -> bool{
        if(!scheck(board, color)) {return false;}
        
        for i in 0..8{
            for j in 0..8{
                if let Cell::None = board._board_types[i as usize][j as usize] {continue;}
                let at_color = board._board_color[i as usize][j as usize];
                if(at_color == color){
                    let mut move_buffer = construct_move_buffer();
                    if (get_move_list(board, (i,j), &mut move_buffer) > 0){
                        return false;
                    }
                }
            }
        }
        
        return true;
    }

    pub fn is_stale_mate(board : &mut Board_state) -> bool{
        for i in 0..8{
            for j in 0..8{
                if let Cell::None = board._board_types[i as usize][j as usize] {continue;}
                let mut move_buffer = construct_move_buffer();
                if (get_move_list(board, (i,j), &mut move_buffer) > 0){
                    return false;
                }
            }
        }
        return true;
    }

    pub fn end_turn(board : &mut Board_state){
        board.turn = !board.turn;
    }

    pub mod Move_util{
        use super::{*, Board::update_threat_buffer, Util::{move_check, construct_move_buffer}};
        use std::io::*;

        
        /*Moves can be created if the following conditions are met: 
        (1) there is a piece on from 
        (2) If there is a piece at to, it is not of the same color*/
        pub fn create_move(from : (i32, i32), to : (i32,i32), board : &Board_state, promo_type : Cell) -> Option<Move> {
            let typ = get_move_type(from, to, board);
            
            match typ{
                Some(p) => {
                    return Some(Move{
                        from : from,
                        to : to,
                        typ : p,
                        color : board._board_color[from.0 as usize][from.1 as usize],
                        promo_type : promo_type
                    });
                },
                None => {return None;}
            };
        }

        pub fn get_move_type(from : (i32, i32), to : (i32,i32), board : &Board_state) -> Option<Move_type>{
            let to_type = board._board_types[to.0 as usize][to.1 as usize];
            let from_type = board._board_types[from.0 as usize][from.1 as usize];
            if let Cell::None = from_type {return None;}

            let from_color = board._board_color[from.0 as usize][from.1 as usize]; //The color making a move
            if let Cell::None = to_type{ //Promotion or Move
                if let Cell::Pawn = from_type{
                    if((to.0 == 0 && from_color == WHITE) || (to.0 == 7 && from_color == BLACK)){
                        return Some(Move_type::Promotion);
                    }else{
                        return Some(Move_type::Move);
                    }
                }else{
                    return Some(Move_type::Move);
                }
            }else{
                if board._board_color[to.0 as usize][to.1 as usize] == from_color{
                    //Check for castling here
                    return None;
                }else{ //Capture
                    return Some(Move_type::Capture);
                }
            }; 
        }

        
        pub fn get_move_list(board : &mut Board_state, _cell : (i32, i32), move_buffer : &mut [Move; MAX_MOVES]) -> usize{
            let mut intermediate_buffer = construct_move_buffer();
            let mv_cnt = match board._board_types[_cell.0 as usize][_cell.1 as usize] {
                Cell::Queen => Queen::generate_moves_simple(_cell.0, _cell.1, board, &mut intermediate_buffer),
                Cell::King => King::generate_moves_simple(_cell.0, _cell.1, board, &mut intermediate_buffer),
                Cell::Bishop => Bishop::generate_moves_simple(_cell.0, _cell.1, board, &mut intermediate_buffer),
                Cell::Knight => Knight::generate_moves_simple(_cell.0, _cell.1, board, &mut intermediate_buffer),
                Cell::Rook => Rook::generate_moves_simple(_cell.0, _cell.1, board, &mut intermediate_buffer),
                Cell::Pawn => Pawn::generate_moves_simple(_cell.0, _cell.1, board, &mut intermediate_buffer),
                Cell::None => 0
            };

            let mut buffer_idx = 0;
            for i in 0..mv_cnt {
                if(move_check(board, &intermediate_buffer[i])){
                    move_buffer[buffer_idx] = intermediate_buffer[i];
                    buffer_idx += 1;
                }
            }
            return buffer_idx;
        }

        pub fn query_promo_type() -> Cell{
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).expect("Did not enter a correct string! Crashing!!!1 AAHHH! :(");

            return match buffer.as_str() {
                "queen" => Cell::Queen,
                "bishop" => Cell::Bishop,
                "rook" => Cell::Rook,
                "knight" => Cell::Knight,
                _ => Cell::None
            };
        }

        pub fn make_move(board : &mut Board_state, mv : &Move){
            match mv.typ {
                Move_type::Move => move_update_move(board, mv),
                Move_type::Capture => move_update_capture(board, mv),
                Move_type::Promotion => {
                    move_update_promotion(board, mv, mv.promo_type);
                },
                _ => panic!("Inavlid move_type ??")
            };
            update_threat_buffer(board);
            end_turn(board);
        }

        pub fn move_update_promotion(board : &mut Board_state, mv : &Move, promo_type : Cell){
        
            //A promotion move could also be a capture move
            if let Cell::None = board._board_types[mv.to.0 as usize][mv.to.1 as usize] {
                move_update_move(board, mv);
            } else {
                move_update_capture(board, mv);
            }
            board._board_types[mv.to.0 as usize][mv.to.1 as usize] = promo_type;
        }
    
        pub fn move_update_capture(board : &mut Board_state, mv : &Move){
            move_update_move(board, mv);
        }
    
        pub fn move_update_move(board : &mut Board_state, mv : &Move){
            let from_usize = (mv.from.0 as usize, mv.from.1 as usize);
            let to_usize = (mv.to.0 as usize, mv.to.1 as usize);
            
            //Updating the king position in the case it was moved
            if let Cell::King = board._board_types[from_usize.0][from_usize.1] {
                board.king_pos[mv.color as usize] = to_usize;
            }

            board._board_types[to_usize.0][to_usize.1] = board._board_types[from_usize.0][from_usize.1];
            board._board_types[from_usize.0][from_usize.1] = Cell::None;

            board._board_color[to_usize.0][to_usize.1] = board._board_color[from_usize.0][from_usize.1];
            board._board_color[from_usize.0][from_usize.1] = WHITE;

            board._board_hasmoved[to_usize.0][to_usize.1] = true;
            board._board_hasmoved[from_usize.0][from_usize.1] = false;

        }
    }

    pub mod Util{
        use std::i32;
        use super::*;
        use super::Move_util::*;

        pub fn is_valid_move(board : &mut Board_state, mv : &Move) -> bool{
            let mut move_buffer = construct_move_buffer();
            let mv_cnt = get_move_list(board, mv.from, &mut move_buffer);

            for i in 0..mv_cnt {
                let curr_mv = move_buffer[i];
                if(curr_mv == *mv){
                    return true;
                }
            }
            return false;
        }

        //Simple moves are checked here, true if passed
        pub fn move_check(board : &Board_state, mv : &Move) -> bool{

            //If we are moving a king
            if let Cell::King = board._board_types[mv.from.0 as usize][mv.from.1 as usize] {
                //Check if the color not doing the move is threatening the cell that we want to move the king to
                if(board.threat_buff[(!mv.color) as usize][mv.to.0 as usize][mv.to.1 as usize] == true) {return false;}
            }
            //We should not be able to make a move that will capture a king
            if let Cell::King = board._board_types[mv.to.0 as usize][mv.to.1 as usize]{return false;}

            //Make a copy of the board and simulate a move, update_threat_buffer, check if check.
            let mut board_copy = *board;
            make_move(&mut board_copy, mv);
            return !scheck(&board_copy, mv.color);
        }

        pub fn generate_threat_static(li : &[(i32, i32)], board : &mut Board_state, color : bool){
            for (row, col) in li.iter() {
                if(row < &0 || row > &7 || col < &0 || col > &7){ continue;}
                
                let at : &Cell = &board._board_types[*row as usize][*col as usize];
                board.threat_buff[color as usize][*row as usize][*col as usize] = true;
            }
        }

        pub fn generate_threat_dir(mut row : i32, mut col : i32, dir : (i32, i32), board : &mut Board_state, color : bool){
            row += dir.0;
            col += dir.1;

            while(row >= 0 && row < 8 && col >= 0 && col < 8){
                let at : &Cell = &board._board_types[row as usize][col as usize];
                board.threat_buff[color as usize][row as usize][col as usize] = true;
                if let Cell::None = at{}else{
                    break;
                }
                
                row += dir.0;
                col += dir.1;
            }
        }

        pub fn construct_move_buffer() -> [Move; MAX_MOVES]{
            return [Move {
                from : (0,0),
                to : (0,0),
                typ : Move_type::Move,
                color : false,
                promo_type : Cell::None
            }; MAX_MOVES];
        }

        //We generate the moves for a piece of color(color) given the positions listed in li
        pub fn generate_moves_simple_static(from : (i32,i32), li : &[(i32, i32)], board : &mut Board_state, move_buffer : &mut [Move; MAX_MOVES]) -> usize{
            let mut buffer_idx: usize= 0;
            assert!(buffer_idx < MAX_MOVES);

            for (row, col) in li{
                if(*row < 0 || *row > 7 || *col < 0 || *col > 7){ continue;}

                //If this errors then the move we tried to create was not remotely valid
                let res = create_move(from, (*row, *col), board, Cell::None);
                if let Option::Some(p) = res {
                    move_buffer[buffer_idx] = p;
                    buffer_idx += 1;
                }
            }

            return buffer_idx;
        }

        pub fn generate_moves_simple_dir(from : (i32,i32), dir : (i32, i32), board : &mut Board_state, move_buffer : &mut [Move; MAX_MOVES]) -> usize{
            let mut buffer_idx: usize= 0;
            assert!(buffer_idx < MAX_MOVES);

            let from_color = board._board_color[from.0 as usize][from.1 as usize];
            let mut to = (from.0 + dir.0, from.1 + dir.1);
            let mut break_flag : bool = false;
            
            while(to.0 >= 0 && to.0 < 8 && to.1 >= 0 && to.1 < 8 && !break_flag){
                let at = board._board_types[to.0 as usize][to.1 as usize];
                let at_color = board._board_color[to.0 as usize][to.1 as usize];
                if let Cell::None = at {} else{
                    break_flag = true;
                    if(from_color == at_color) {continue;}
                }

                move_buffer[buffer_idx] = create_move(from, to, board, Cell::None).unwrap(); //This is expected to work
                buffer_idx += 1;
                to.0 += dir.0;
                to.1 += dir.1;
            }

            return buffer_idx;
        }


    }

    pub mod Knight{
        use super::{Board_state, MAX_MOVES};
        use super::Move;
        use super::Util::*;

        pub fn generate_threat(row : i32, col : i32, board : &mut Board_state, color : bool){
            let li : [(i32, i32); 8] = [
                (row+1, col-2),
                (row+2, col-1),
    
                (row+2, col+1),
                (row+1, col+2),
                
                (row-1, col+2),
                (row-2, col+1),
                
                (row-2, col-1),
                (row-1, col-2)
            ];
            generate_threat_static(&li, board, color);
        }

        pub fn generate_moves_simple(row : i32, col : i32, board: &mut Board_state, move_buffer : &mut [Move; MAX_MOVES]) -> usize{
            let li : [(i32, i32); 8] = [
                (row+1, col-2),
                (row+2, col-1),
    
                (row+2, col+1),
                (row+1, col+2),
                
                (row-1, col+2),
                (row-2, col+1),
                
                (row-2, col-1),
                (row-1, col-2)
            ];
            return generate_moves_simple_static((row, col), &li, board, move_buffer);
        }

        /* 
        pub fn generate() -> u64{
            
        }
        */
    }

    pub mod Bishop{
        use super::MAX_MOVES;
        use super::Board_state;
        use super::Move;
        use super::Util::*;

        pub fn generate_threat(row : i32, col : i32, board : &mut Board_state, color : bool){
            let dir_li : [(i32,i32); 4] = [
                (-1,-1),
                (-1,1),
                (1,-1),
                (1,1)
            ];
            for dir in dir_li{
                generate_threat_dir(row, col, dir, board, color);
            }
        }

        pub fn generate_moves_simple(row : i32, col : i32, board: &mut Board_state, move_buffer : &mut [Move; MAX_MOVES]) -> usize{
            let dir_li : [(i32,i32); 4] = [
                (-1,-1),
                (-1,1),
                (1,-1),
                (1,1)
            ];
            let mut curr_cnt = 0;

            for dir in dir_li{
                curr_cnt += generate_moves_simple_dir((row,col), dir, board, move_buffer);
            }
            return curr_cnt;
        }

    }

    pub mod Rook{
        use super::Board_state;
        use super::Util::*;
        use super::Move;
        use super::MAX_MOVES;

        pub fn generate_threat(row : i32, col : i32, board : &mut Board_state, color : bool){
            let dir_li : [(i32, i32); 4] = [
                (1,0),
                (-1,0),
                (0,1),
                (0,-1)
            ];

            for dir in dir_li{
                generate_threat_dir(row, col, dir, board, color);
            }
        }

        pub fn generate_moves_simple(row : i32, col : i32, board: &mut Board_state, move_buffer : &mut [Move; MAX_MOVES]) -> usize{
            let dir_li : [(i32, i32); 4] = [
                (1,0),
                (-1,0),
                (0,1),
                (0,-1)
            ];
            let mut curr_cnt = 0;
            for dir in dir_li{
                curr_cnt += generate_moves_simple_dir((row, col), dir, board, move_buffer);
            }
            return curr_cnt;
        }

    }

    pub mod Queen{
        use super::Board_state;
        use super::Util::*;
        use super::MAX_MOVES;
        use super::Move;

        pub fn generate_threat(row : i32, col : i32, board : &mut Board_state, color : bool){
            let dir_li : [(i32, i32); 8] = [
                (1, 0),
                (-1,0),
                (0,1),
                (0,-1),
                (-1, -1),
                (-1,1),
                (1,-1),
                (1,1)
            ];

            for dir in dir_li{
                generate_threat_dir(row, col, dir, board, color);
            }
        }

        pub fn generate_moves_simple(row : i32, col : i32, board: &mut Board_state, move_buffer : &mut [Move; MAX_MOVES]) -> usize{
            let dir_li : [(i32, i32); 8] = [
                (1, 0),
                (-1,0),
                (0,1),
                (0,-1),
                (-1, -1),
                (-1,1),
                (1,-1),
                (1,1)
            ];
            
            let mut curr_cnt = 0;
            for dir in dir_li{
                curr_cnt += generate_moves_simple_dir((row, col), dir, board, move_buffer);
            }
            return curr_cnt;
        }

    }

    pub mod King{
        use super::Board_state;
        use super::Util::*;
        use super::MAX_MOVES;
        use super::Move;

        pub fn generate_threat(row : i32, col : i32, board : &mut Board_state, color : bool){
            let li : [(i32, i32); 8] = [
                (row+1, col+1),
                (row-1, col+1),
                (row+1, col-1),
                (row-1, col-1),
                
                (row-1, col),
                (row, col-1),
                (row+1, col),
                (row, col+1)
            ];

            generate_threat_static(&li, board, color);
        }

        pub fn generate_moves_simple(row : i32, col : i32, board: &mut Board_state, move_buffer : &mut [Move; MAX_MOVES]) -> usize{
            let li : [(i32, i32); 8] = [
                (row+1, col+1),
                (row-1, col+1),
                (row+1, col-1),
                (row-1, col-1),
                
                (row-1, col),
                (row, col-1),
                (row+1, col),
                (row, col+1)
            ];

            return generate_moves_simple_static((row,col), &li, board, move_buffer);
        }
    }

    pub mod Pawn{
        use super::Util::*;
        use super::*;
        
        pub fn generate_threat(row : i32, col : i32, board : &mut Board_state, color : bool){
            let row_increment = if color == false {-1} else {1}; //if color is white
            
            let li : [(i32,i32); 2] = [
                (row + row_increment, col + 1),
                (row + row_increment, col - 1)
            ];
            generate_threat_static(&li, board, color);
        }

        pub fn generate_moves_simple(row : i32, col : i32, board: &mut Board_state, move_buffer : &mut [Move; MAX_MOVES]) -> usize{
            let color = board._board_color[row as usize][col as usize];
            let row_increment = if color == false {-1} else {1}; //if color is white
            let mut li : [(i32,i32); 4] = [
                (-1, -1),
                (-1, -1), //Terrible workaround, these will be ignored by the static generator
                (-1, -1),
                (-1, -1),
            ];
            let new_row_1 = row + row_increment;
            let new_row_2 = row + 2*row_increment;
            let has_moved : bool = board._board_hasmoved[row as usize][col as usize];

            if(new_row_1 >= 0 && new_row_1 < 8){
                //One up
                if let Cell::None = board._board_types[new_row_1 as usize][col as usize] {
                    li[0] = (new_row_1, col);
                }
                //Two up
                if(new_row_2 >= 0 && new_row_2 < 8 && has_moved == false){
                    if let Cell::None = board._board_types[new_row_2 as usize][col as usize]{
                        li[1] = (new_row_2, col);
                    }
                }
                //One up, left
                if(col-1 >= 0 && col-1 < 8){
                    if let Cell::None = board._board_types[new_row_1 as usize][(col-1) as usize] {} else {
                        li[2] = (new_row_1, col-1);
                    }
                }
                //One up, right
                if(col+1 >= 0 && col+1 < 8){
                    if let Cell::None = board._board_types[new_row_1 as usize][(col+1) as usize] {} else {
                        li[3] = (new_row_1, col+1);
                    }
                }
            }
            
            return generate_moves_simple_static((row, col), &li, board, move_buffer); //This will handle the boundary checks
        }

    }

    pub mod Testing_interface{
        use super::*;

        pub fn create_blank_board() -> Board_state {
            let mut B = Board_state{
                _board_types : [[Cell::None; 8]; 8],
                _board_hasmoved : [[false; 8]; 8],
                _board_color : [[false; 8]; 8],

                threat_buff : [[[false; 8]; 8]; 2],
                turn : false,
                castling : [false; 2],
                king_pos : [(0,0); 2]
            };

            return B;
        }
    }


}


/*
pub struct Board_state{
        //state defining parameeters
        _board : [[Cell; 8]; 8],
        threat_buff : [u64; 2], //is king in scheck
        turn : bool, //0 is white, 1 is black
        castling: [bool; 2],
        
        king_pos : [(u8, u8); 2],
    }

*/



