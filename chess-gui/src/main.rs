//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps, unused_parens)]



// LIB STUFF ANTMAG
// use test_crate::chess_api::*;
// use test_crate::chess_api::Board::*;
// use test_crate::chess_api::Move_util::*;
// use test_crate::chess_api::Util::*;
// ! LIB STUFF ANTMAG


// TMP LIB STUFF LOCAL
mod lib;
use lib::chess_api::*;
use lib::chess_api::Board::*;
use lib::chess_api::Move_util::*;
use lib::chess_api::Util::*;
// ! TMP LIB STUFF LOCAL

use std::{env, path};
const WIDTH: f32 = 1600.0;
const HEIGHT: f32 = 1600.0;
const SEL_COL: Color = Color::CYAN;

use ggez::{
    event,
    graphics::{self, Color},
    Context, GameResult,
};
use glam::*;

struct MainState {
    board: Board_state,
    selected_xy: SelectedXY,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState { board: create_init_board(), selected_xy: SelectedXY::None})
    }
    // fn new(ctx: &mut Context) -> GameResult<MainState> {
    //     let circle = graphics::Mesh::new_circle(
    //         ctx,
    //         graphics::DrawMode::fill(),
    //         vec2(0., 0.),
    //         100.0,
    //         2.0,
    //         Color::WHITE,
    //     )?;
        

    //     Ok(MainState { pos_x: 0.0, circle })
    // }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // self.pos_x = self.pos_x % 800.0 + 1.0;
        let mpos = _ctx.mouse.position();
        let mx = mpos.x;
        let my = mpos.y;

        
        if _ctx.mouse.button_pressed(ggez::input::mouse::MouseButton::Left) {
            let selected_x = (8.0 * mx / WIDTH).floor() as usize;
            let selected_y = (8.0 * my / HEIGHT).floor() as usize;


            match self.selected_xy {
                SelectedXY::None => {
                    if self.board._board_color[selected_y][selected_x] == self.board.turn {
                        self.selected_xy = SelectedXY::Selected((selected_x, selected_y));
                    }
                },
                SelectedXY::Selected(sel_xy) => {
                    if self.board._board_color[selected_y][selected_x] == self.board.turn {
                        // MAKE MOVE from selxy to selected_x, selected_y to

                        
                        // OBS MADE 7 - to flip

                        let from = (7-sel_xy.1 as i32, sel_xy.0 as i32);
                        let to = (7-selected_y as i32, selected_x as i32);
                        let o_mv = create_move(from, to, &self.board, self.board._board_types[selected_y][selected_x]);
                        match o_mv {
                            Some(mv) => {
                                make_move(&mut self.board, &mv);
                                self.selected_xy = SelectedXY::None;
                                // self.board.turn = !self.board.turn;
                            },
                            None => (),
                        }

                    }
                },
            }
            
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::CanvasLoadOp::Clear([0.1, 0.2, 0.3, 1.0].into()),
        );

        // GET MOUSE 
        // if ggez::input::mouse::cursor_grabbed(ctx) {
            let mpos = ctx.mouse.position();
            let mx = mpos.x;
            let my = mpos.y;
        // }
        // ! GET MOUSE

        // MAIN LOOP THROUGH ROWS AND SQUARES
        let s = WIDTH*0.125;
        let mut w = true;
        for _y in 0..8 {
            for _x in 0..8 {
                let x = _x as f32;
                let y = _y as f32;

                // DRAWING SQUARE
                let rect = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect {x: x*s, y: y*s, h: s, w: s},
                    match w {
                        true => Color::WHITE,
                        false => Color::BLACK,
                    },
                )?;
                canvas.draw(&rect, Vec2::new(0.0, 0.0));
                // ! DRAWING SQUARE

                // DRAWING FRAME
                if mx > x*s && mx < (x+1.0)*s && my > y*s && my < (y+1.0)*s {
                    let frame = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::stroke(20.0),
                        graphics::Rect {x: x*s, y: y*s, h: s, w: s},
                        match self.selected_xy {
                            SelectedXY::None => {
                                Color::GREEN
                            },
                            SelectedXY::Selected(sel_xy) => {
                                if sel_xy == (_x, _y) {
                                    SEL_COL
                                } else {
                                    Color::GREEN
                                }
                            }
                        },
                    )?;
                    canvas.draw(&frame, Vec2::new(0.0, 0.0));
                    

                } else {
                    match self.selected_xy {
                        SelectedXY::Selected(xy) => {
                            if xy == (_x, _y) {
                                let frame = graphics::Mesh::new_rectangle(
                                    ctx,
                                    graphics::DrawMode::stroke(20.0),
                                    graphics::Rect {x: x*s, y: y*s, h: s, w: s},
                                    SEL_COL
                                )?;
                                canvas.draw(&frame, Vec2::new(0.0, 0.0));
                            }

                        },
                        SelectedXY::None => (),
                    }                    
                }
                // ! DRAWING FRAME



                // DRAWING PIECE
                
                let cell: Cell = self.board._board_types[7-_y][_x];
                let is_black = self.board._board_color[7-_y][_x];
                let path = get_path(cell, is_black);
                match cell {
                    Cell::None => (),
                    _ => {
                        let scale_xy = (WIDTH*0.125)/75.0;
                        let image1 = graphics::Image::from_path(ctx, path, true)?;
                        let drawparams = graphics::DrawParam::new()
                        .dest([x*s, y*s])
                        .rotation(0.0)
                        .offset([0.0, 0.0])
                        .scale([scale_xy, scale_xy]);
                        canvas.draw(&image1, drawparams);
                    },
                }
                // ! DRAWING SQUARE


                w = !w;

            }
            w = !w;
        }
        // ! MAIN LOOP THROUGH ROWS AND SQUARES

        canvas.finish(ctx)?;

        Ok(())
    }

}

fn get_path(cell: Cell, is_black: bool) -> String {
    let bw = match is_black {
        true => "b",
        false => "w",
    };
    let piece_name = match cell {
        Cell::None => "",
        Cell::Bishop => "bishop",
        Cell::Pawn => "pawn",
        Cell::Rook => "rook",
        Cell::Knight => "knight",
        Cell::King => "king",
        Cell::Queen => "queen",
    };
    format!("/{bw}_{piece_name}.png")
}

pub fn main() -> GameResult {

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("hoho", "ggez").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb
    .window_mode(ggez::conf::WindowMode::default().dimensions(WIDTH, HEIGHT))
    .build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

enum SelectedXY {
    None,
    Selected((usize, usize)),
}