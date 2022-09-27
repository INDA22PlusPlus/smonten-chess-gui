fn main() {
    let mut b : Board_state = create_init_board();
    create_start_config(&b);
    for row in b._board_types {
        for cell in row: {
            print!("|{}", get_symbol(cell));
        }
        println!("|");
    }
}

fn get_symbol(cell: Cell) -> &str {
    match cell {
        Cell::King => "k",
        Cell::Queen => "q",
        Cell::Bishop => "b",
        Cell::Knight => "h",
        Cell::Rook => "r",
        Cell::Pawn => "p"
        Cell::None => "_"
    }
}
