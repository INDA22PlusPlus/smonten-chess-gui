//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

const WIDTH: f32 = 1600.0;
const HEIGHT: f32 = 1600.0;

use ggez::{
    event,
    graphics::{self, Color},
    Context, GameResult,
};
use glam::*;

struct MainState {
    pos_x: f32,
    circle: graphics::Mesh,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            100.0,
            2.0,
            Color::WHITE,
        )?;
        

        Ok(MainState { pos_x: 0.0, circle })
    }
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
                w = !w;

            }
            w = !w;
        }

        

        // canvas.draw(&self.circle, Vec2::new(self.pos_x, 380.0));

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb
    .window_mode(ggez::conf::WindowMode::default().dimensions(WIDTH, HEIGHT))
    .build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}