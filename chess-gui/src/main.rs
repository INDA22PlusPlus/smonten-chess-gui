//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]



// LIB STUFF
use test_crate::chess_api::*;
use test_crate::chess_api::Board::*;
use test_crate::chess_api::Move_util::*;
use test_crate::chess_api::Util::*;
// ! LIB STUFF

use std::{env, path};
const WIDTH: f32 = 1600.0;
const HEIGHT: f32 = 1600.0;

use ggez::{
    event,
    graphics::{self, Color},
    Context, GameResult,
};
use glam::*;

struct MainState {
    board: Board_state, 
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState { board: create_init_board() })
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
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::CanvasLoadOp::Clear([0.1, 0.2, 0.3, 1.0].into()),
        );

        let s = WIDTH*0.125;
        let mut w = true;
        for _y in 0..8 {
            for _x in 0..8 {
                let x = _x as f32;
                let y = _y as f32;

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

                
                let cell: Cell = self.board._board_types[_y][_x];
                let is_black = self.board._board_color[_y][_x];
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
                w = !w;

            }
            w = !w;
        }
        // let mouse_in_window = ggez::input::mouse::get_grabbed(_ctx);
        // let im = graphics::Image::from_path(ctx, Path::new("./img/pawn.png"), true);
        // canvas.draw(&im, param)
    

        // canvas.draw(&self.circle, Vec2::new(self.pos_x, 380.0));

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

    let cb = ggez::ContextBuilder::new("super_simple", "ggez").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb
    .window_mode(ggez::conf::WindowMode::default().dimensions(WIDTH, HEIGHT))
    .build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}