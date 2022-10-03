//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps, unused_parens)]



// LIB STUFF
use chess_lib::{create_game, Destination, Color, PieceType};
use chess_lib::Game;
use chess_lib::*;
// ! LIB STUFF



use ggez::{
    event,
    graphics,
    Context, GameResult,
};
use glam::*;

use std::{env, path};
const WIDTH: f32 = 800.0;
const HEIGHT: f32 = WIDTH;
const SEL_COL: graphics::Color = graphics::Color::CYAN;

struct MainState {
    board: Game,
    cur_selected_xy: SelectedXY,
    assets: Assets
}

impl MainState {
    fn new(ctx: &mut Context, assets: Assets) -> GameResult<MainState> {
        Ok(MainState { board: create_game(), cur_selected_xy: SelectedXY::None, assets: assets})
    }
}

struct Assets {
    board_img: graphics::Image,
    board_drawparam: graphics::DrawParam,
    b_pawn_img: graphics::Image,
    b_rook_img: graphics::Image,
    b_knight_img: graphics::Image,
    b_bishop_img: graphics::Image,
    b_queen_img: graphics::Image,
    b_king_img: graphics::Image,
    w_pawn_img: graphics::Image,
    w_rook_img: graphics::Image,
    w_knight_img: graphics::Image,
    w_bishop_img: graphics::Image,
    w_queen_img: graphics::Image,
    w_king_img: graphics::Image,
}
impl Assets {
    fn get_image(&self, p: &Piece) -> &graphics::Image {
        match p.color {
            Color::Black => {
                match p.piece_type {
                    PieceType::Pawn => &self.b_pawn_img,
                    PieceType::Rook => &self.b_rook_img,
                    PieceType::Knight => &self.b_knight_img,
                    PieceType::Bishop => &self.b_bishop_img,
                    PieceType::Queen => &self.b_queen_img,
                    PieceType::King => &self.b_king_img,
                }
            },
            Color::White => {
                match p.piece_type {
                    PieceType::Pawn => &self.w_pawn_img,
                    PieceType::Rook => &self.w_rook_img,
                    PieceType::Knight => &self.w_knight_img,
                    PieceType::Bishop => &self.w_bishop_img,
                    PieceType::Queen => &self.w_queen_img,
                    PieceType::King => &self.w_king_img,
                }
            }
        }
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // self.pos_x = self.pos_x % 800.0 + 1.0;
        let mpos = _ctx.mouse.position();
        let mx = mpos.x;
        let my = mpos.y;


        if _ctx.mouse.button_pressed(ggez::input::mouse::MouseButton::Left) {
            let new_sel_x = (8.0 * mx / WIDTH).floor() as usize;
            let new_sel_y = (8.0 * my / HEIGHT).floor() as usize;
            let new_sel_xy = (new_sel_x, new_sel_y);

            
            // let new_selected_is_black = self.board._board_color[new_sel_y][new_sel_x];
            
            let new_sel_square = self.board.get_square_xy(new_sel_xy);
            match new_sel_square {
                // PRESSED SQUARE IS EMPTY
                Content::Empty => {
                    
                    match self.cur_selected_xy {
                        // A SQUARE IS CURRENTLY SELECTED
                        SelectedXY::Selected(cur_sel_xy) => {
                            match self.board.get_destinations(cur_sel_xy) {
                                // CAN WE EVEN MOVE THE PIECE HERE?
                                Destinations::Exists(d) => {
                                    if d.contains(&new_sel_xy) {
                                        println!("to empty from playable piece, legal move. Shuld move!");
                                        // THEN WE CAN MAKE OUR MOVE
                                        self.board.move_from_to(cur_sel_xy, new_sel_xy);
                                        // OBS HAS TO RESET SELCT
                                        self.cur_selected_xy = SelectedXY::None;
                                    }
                                },
                                // CANT MOVE
                                Destinations::None => (),
                            }

                        },
                        // CANT SELECT EMPTY IF HAVN'T SELECTED PIECE
                        SelectedXY::None => (),
                    }
                },
                // PRESSED SQUARE IS OCCUPIED
                Content::Occupied(new_sel_p) => {
                    match self.cur_selected_xy {
                        // A PIECE IS CURRENTLY SELECTED
                        SelectedXY::Selected(cur_sel_xy) => {
                            if self.board.get_turn() == new_sel_p.color {
                                // SELECTED PIECE OF OWN COLOR -> RESELECT
                                self.cur_selected_xy = SelectedXY::Selected(new_sel_xy);
                            } else {
                                match self.board.get_destinations(cur_sel_xy) {
                                    Destinations::Exists(dests) => {
                                        if dests.contains(&new_sel_xy) {
                                            // SELECTED PIECE OF DIFFERENT COLOR -> KILL!
                                            println!("kill!");
                                            self.board.move_from_to(cur_sel_xy, new_sel_xy);
                                            // OBS RESET SELECT
                                            self.cur_selected_xy = SelectedXY::None;
                                        } else {
                                            println!("attempted illigal move")
                                        }
                                    },
                                    Destinations::None => (),
                                }
                            }
                        },
                        // CURRENTLY NO SELECTED SQUARE, THIS IS FIRST SELECT
                        SelectedXY::None => {
                            if self.board.coordinates_playable(new_sel_xy) {
                                self.cur_selected_xy = SelectedXY::Selected(new_sel_xy);
                            }
                        }
                    }
                }
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


        // DRAWING BOARD AS IMAGE
        canvas.draw(&self.assets.board_img, self.assets.board_drawparam);
        // ! DRAWING BOARD AS IMAGE


        // MAIN LOOP THROUGH ROWS AND SQUARES
        let s = WIDTH*0.125;
        let mut w = true;
        for _y in 0..8 {
            for _x in 0..8 {
                let x = _x as f32;
                let y = _y as f32;

                // DRAWING SQUARE
                match self.cur_selected_xy {
                    SelectedXY::Selected(cur_sel_xy) => {
                        match self.board.get_destinations(cur_sel_xy) {
                            Destinations::Exists(dests) => {
                                if dests.contains(&(_x, _y)) {
                                    let rect = graphics::Mesh::new_rectangle(
                                        ctx,
                                        graphics::DrawMode::fill(),
                                        graphics::Rect {x: x*s, y: y*s, h: s, w: s},
                                        graphics::Color::RED,
                                    )?;
                                    canvas.draw(&rect, Vec2::new(0.0, 0.0));
                                }
                            },
                            Destinations::None => (),
                        }
                    },
                    SelectedXY::None => (),
                }
                // ! DRAWING SQUARE



                // DRAWING FRAME
                if mx > x*s && mx < (x+1.0)*s && my > y*s && my < (y+1.0)*s {
                    let frame = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::stroke(20.0),
                        graphics::Rect {x: x*s, y: y*s, h: s, w: s},
                        match self.cur_selected_xy {
                            SelectedXY::None => {
                                graphics::Color::GREEN
                            },
                            SelectedXY::Selected(sel_xy) => {
                                if sel_xy == (_x, _y) {
                                    SEL_COL
                                } else {
                                    graphics::Color::GREEN
                                }
                            }
                        },
                    )?;
                    canvas.draw(&frame, Vec2::new(0.0, 0.0));
                    

                } else {
                    match self.cur_selected_xy {
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

                let square = self.board.get_square_xy((_x, _y));
                let scale_xy = (WIDTH*0.125)/75.0;
                let drawparams = graphics::DrawParam::new()
                .dest([x*s, y*s])
                .rotation(0.0)
                .offset([0.0, 0.0])
                .scale([scale_xy, scale_xy]);
                match square {
                    Content::Empty => (),
                    Content::Occupied(p) => {
                        let piece_img = self.assets.get_image(p);
                        canvas.draw(piece_img, drawparams);
                    },
                }                
                // ! DRAWING PIECE

                w = !w;
            }
            w = !w;
        }
        // ! MAIN LOOP THROUGH ROWS AND SQUARES

        canvas.finish(ctx)?;

        Ok(())
    }

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

    // SETTING UP ASSETS
    let scale_xy_board = WIDTH/1168.0;
    let mut assets = Assets {
        board_img: graphics::Image::from_path(&ctx, "/board.png", true)?,
        board_drawparam: graphics::DrawParam::new()
        .dest([0.0, 0.0])
        .rotation(0.0)
        .offset([0.0, 0.0])
        .scale([scale_xy_board, scale_xy_board]),
        b_pawn_img: graphics::Image::from_path(&ctx, "/b_pawn.png", true)?,
        b_rook_img: graphics::Image::from_path(&ctx, "/b_rook.png", true)?,
        b_knight_img: graphics::Image::from_path(&ctx, "/b_knight.png", true)?,
        b_bishop_img: graphics::Image::from_path(&ctx, "/b_bishop.png", true)?,
        b_queen_img: graphics::Image::from_path(&ctx, "/b_queen.png", true)?,
        b_king_img: graphics::Image::from_path(&ctx, "/b_king.png", true)?,
        w_pawn_img: graphics::Image::from_path(&ctx, "/w_pawn.png", true)?,
        w_rook_img: graphics::Image::from_path(&ctx, "/w_rook.png", true)?,
        w_knight_img: graphics::Image::from_path(&ctx, "/w_knight.png", true)?,
        w_bishop_img: graphics::Image::from_path(&ctx, "/w_bishop.png", true)?,
        w_queen_img: graphics::Image::from_path(&ctx, "/w_queen.png", true)?,
        w_king_img: graphics::Image::from_path(&ctx, "/w_king.png", true)?,
        
    };
    // ! SETTING UP ASSETS

    let state = MainState::new(&mut ctx, assets)?;
    event::run(ctx, event_loop, state)
}

enum SelectedXY {
    None,
    Selected((usize, usize)),
}