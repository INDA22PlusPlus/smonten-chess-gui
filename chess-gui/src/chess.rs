// LIBRARY STUFF
use test_crate::chess_api::*;
use test_crate::chess_api::Board::*;
use test_crate::chess_api::Move_util::*;
use test_crate::chess_api::Util::*;


fn test_chess() {
    let mut b : Board_state = create_init_board();
    // create_start_config(&b);
    draw(b);
    let m = create_move((6,0), (5,0), &b, b._board_types[1][0]);
    match m {
        Some(mv) => {
            make_move(&mut b, &mv);
                draw(b);
            if is_valid_move(&mut b, &mv) {
                println!("this was a legal move");
            } else {
                println!("you attempted illigal move");
            }

        },
        None => println!("no move to make here")
    }
}
fn draw(b: Board_state) {
    for y in 0..8 {
        for x in 0..8 {
            let y = y as usize;
            let x = x as usize;
            let cell: Cell = b._board_types[y][x];
            let is_black: bool = b._board_color[y][x];
            print!("|{}", get_symbol(cell, is_black));
            
        }
        println!("|");
    }
    println!("");
}
fn get_symbol(cell: Cell, is_black: bool) -> &'static str {
    if is_black {
        match cell {
            Cell::Pawn => "♟",
            Cell::Bishop => "♝",
            Cell::Knight => "♞",
            Cell::Rook => "♜",
            Cell::Queen => "♛",
            Cell::King => "♚",
            Cell::None => "_"
        }
    } else {
        match cell {
            Cell::Pawn => "♙",
            Cell::Bishop => "♗",
            Cell::Knight => "♘",
            Cell::Rook => "♖",
            Cell::Queen => "♕",
            Cell::King => "♔",
            Cell::None => "_"
        }  
    }
}
