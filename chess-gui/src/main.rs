//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps, unused_parens)]




// LIB STUFF
use chess_lib::{create_game, Destination, Color, PieceType};
use chess_lib::Game;
use chess_lib::*;
use prost::Message;
// ! LIB STUFF


// NETWORKING LIB
// use std::net::{TcpStream, TcpListener};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

mod networking;
use networking::*;
// ! NETWORKING LIB



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



#[derive(PartialEq)]
enum State {
    Playing,
    WaitingForOpponent,
}

struct MainState {
    board: Game,
    cur_selected_xy: SelectedXY,
    assets: Assets,

    state: State,
    // Networking
    stream: TcpStream,
    role: Role,
    connected: bool,
    expecting_move_ack: bool
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // SETTING UP ASSETS
        let scale_xy_board = WIDTH/1168.0;
        let mut assets = Assets {
            board_img: graphics::Image::from_path(ctx, "/board.png")?,
            board_drawparam: graphics::DrawParam::new()
                .dest([0.0, 0.0])
                .rotation(0.0)
                .offset([0.0, 0.0])
                .scale([scale_xy_board, scale_xy_board]),
            b_pawn_img: graphics::Image::from_path(ctx, "/b_pawn.png")?,
            b_rook_img: graphics::Image::from_path(ctx, "/b_rook.png")?,
            b_knight_img: graphics::Image::from_path(ctx, "/b_knight.png")?,
            b_bishop_img: graphics::Image::from_path(ctx, "/b_bishop.png")?,
            b_queen_img: graphics::Image::from_path(ctx, "/b_queen.png")?,
            b_king_img: graphics::Image::from_path(ctx, "/b_king.png")?,
            w_pawn_img: graphics::Image::from_path(ctx, "/w_pawn.png")?,
            w_rook_img: graphics::Image::from_path(ctx, "/w_rook.png")?,
            w_knight_img: graphics::Image::from_path(ctx, "/w_knight.png")?,
            w_bishop_img: graphics::Image::from_path(ctx, "/w_bishop.png")?,
            w_queen_img: graphics::Image::from_path(ctx, "/w_queen.png")?,
            w_king_img: graphics::Image::from_path(ctx, "/w_king.png")?,
        };
        // ! SETTING UP ASSETS

        let (stream, client) = {
            let mut args = std::env::args();
            // Skip path to program
            let _ = args.next();

            // Get first argument after path to program
            let host_or_client = args
                .next()
                .expect("Expected arguments: --host or --client 'ip'");

            match host_or_client.as_str() {
                // If the program is running as host we listen on port 8080 until we get a
                // connection then we return the stream.
                "--host" => {
                    let listener = TcpListener::bind("0.0.0.0:1337").unwrap();
                    (listener.incoming().next().unwrap().unwrap(), false)
                }
                // If the program is running as a client we connect to the specified IP address and
                // return the stream.
                "--client" => {
                    let ip = args.next().expect("Expected ip address after --client");
                    let stream = TcpStream::connect(ip).expect("Failed to connect to host");
                    (stream, true)
                }
                // Only --host and --client are valid arguments
                _ => panic!("Unknown command: {}", host_or_client),
            }
        };
        // Set TcpStream to non blocking so that we can do networking in the update thread
        stream
            .set_nonblocking(true)
            .expect("Failed to set stream to non blocking");

        Ok(MainState {
            board: create_game(),
            cur_selected_xy: SelectedXY::None,
            assets: assets,
            state: if client {
                    State::WaitingForOpponent
                } else {
                    State::Playing
                },
            stream: stream,
            role: match client {
                true => Role::Client,
                false => Role::Server,
            },
            connected: false,
            expecting_move_ack: false,

        })
    }

    fn network_connect(&mut self) {
        match self.role {
            Role::Server => {

            },
            Role::Client => {
                // CLIENT IS TRYING TO CONNECT
                // let cr = networking::C2sConnectRequest {

                // };
            }
        }
    }

    fn send_connect_request_c2s(&mut self) {
        println!("sending connect request to server");
        let cr = networking::C2sConnectRequest {
            game_id: 1,
            spectate: false,
        };
        let msg = networking::C2sMessage {
            msg: Some(networking::c2s_message::Msg::ConnectRequest(cr)),
        };
        let packet = msg.encode_to_vec();
        self.stream.write(&packet);
    }

    fn expect_connect_ack_c2s(&mut self) {
        println!("expecting connect ack from server");
        let mut buf: [u8; 512] = [0_u8; 512];
        let buf_len = match self.stream.read(&mut buf) {
            Ok(l) => l,
            Err(e) => 0,
        };
        let raw_msg = networking::S2cMessage::decode(&buf[..buf_len]).expect("read went wrong");
        match raw_msg.msg {
            None => (),
            Some(msg) => {
                match msg {
                    networking::s2c_message::Msg::ConnectAck(msg_ca) => {
                        let success = msg_ca.success;
                        if success {
                            // CONNECTION SECURE!
                            println!("connected to server! :)");
                            self.connected = true;
                            // ! CONNECTION SECURE!
                        }
                    },
                    _ => (),
                }
            }
        }
    }

    fn expect_connect_request_s2c(&mut self) {
        println!("expecting connect request from client");
        let mut buf: [u8; 512] = [0_u8; 512];
        let buf_len = match self.stream.read(&mut buf) {
            Ok(l) => l,
            Err(e) => 0,
        };
        let raw_msg = networking::C2sMessage::decode(&buf[..buf_len]).expect("read went wrong");
        match raw_msg.msg {
            None => (),
            Some(msg) => {
                match msg {
                    networking::c2s_message::Msg::ConnectRequest(msg_cr) => {
                        let id = msg_cr.game_id;
                        let spectate = msg_cr.spectate;

                        // CONNECTION SECURE!
                        println!("connected to client! :)");
                        self.connected = true;
                        let ca = networking::S2cConnectAck {
                            success: true,
                            game_id: Some(1),
                            starting_position: Some(networking::BoardState {fen_string: self.board.get_fen()}),
                            client_is_white: Some(false),
                        };
                        let msg = networking::S2cMessage {
                            msg: Some(networking::s2c_message::Msg::ConnectAck(ca)),
                        };
                        let packet = msg.encode_to_vec();
                        self.stream.write(&packet);
                        // ! CONNECTION SECURE!
                    },
                    _ => (),
                }
            }
        }

    }

    fn expect_move_ack_c2s(&mut self) {
        println!("expecting move ack from server");
        let mut buf: [u8; 512] = [0_u8; 512];
        let buf_len = match self.stream.read(&mut buf) {
            Ok(l) => l,
            Err(e) => 0,
        };
        let raw_msg = networking::S2cMessage::decode(&buf[..buf_len]).expect("read went wrong");
        match raw_msg.msg {
            None => (),
            Some(msg) => {
                match msg {
                    networking::s2c_message::Msg::MoveAck(msg_ma) => {
                        let legal = msg_ma.legal;
                        let board_result = msg_ma.board_result;

                        if legal {
                            println!("move ok :)");
                            // update board
                            println!("must read fen_string!");
                            self.state = State::WaitingForOpponent;
                        } else {
                            println!("illigal move, try again");
                        }
                    },
                    _ => (),
                }
            }
        }
    }

    fn send_move_packet_c2s(&mut self, mut from: (usize, usize), mut to: (usize, usize), promotion: Option<networking::Piece>) {
        // FLIP Y, FOLLOW THE PROTOCOL
        from.1 = 7-from.1;
        to.1 = 7-to.1;
        // ! FLIP Y, FOLLOW THE PROTOCOL

        let prom_to_send = match promotion {
            None => {
                None
            },
            Some(p) => {
                Some(p as i32)
            }
        };
        let m = networking::Move {
            from_square: self.xy_to_square(from),
            to_square: self.xy_to_square(to),
            promotion: prom_to_send,
        };

        let msg = networking::C2sMessage {
            msg: Some(c2s_message::Msg::Move(m)),
        };
        

        // let mut buf: [u8; 512] = [0_u8; 512];
        let packet = msg.encode_to_vec();
        self.stream.write(&packet);
    }

    fn send_move_packet_s2c(&mut self, mut from: (usize, usize), mut to: (usize, usize), promotion: Option<networking::Piece>) {
        // FLIP Y, FOLLOW THE PROTOCOL
        from.1 = 7-from.1;
        to.1 = 7-to.1;
        // ! FLIP Y, FOLLOW THE PROTOCOL

        let prom_to_send = match promotion {
            None => {
                None
            },
            Some(p) => {
                Some(p as i32)
            }
        };
        let m = networking::Move {
            from_square: self.xy_to_square(from),
            to_square: self.xy_to_square(to),
            promotion: prom_to_send,
        };

        let msg = networking::S2cMessage {
            msg: Some(s2c_message::Msg::Move(m)),
        };
        

        // let mut buf: [u8; 512] = [0_u8; 512];
        let packet = msg.encode_to_vec();
        self.stream.write(&packet);
    }

    // server receving packet
    fn recieve_packet_c2s(&mut self) {
        let mut buf: [u8; 512] = [0_u8; 512];
        let buf_len = match self.stream.read(&mut buf) {
            Ok(l) => l,
            Err(e) => 0,
        };

        let raw_msg = networking::C2sMessage::decode(&buf[..buf_len]).expect("read went wrong");
        match raw_msg.msg {
            None => (),
            Some(msg) => {
                match msg {
                    networking::c2s_message::Msg::Move(msg_m) => {
                        let mut from = self.square_to_xy(msg_m.from_square);
                        let mut to = self.square_to_xy(msg_m.to_square);
                        let promotion = msg_m.promotion;

                        // FLIP Y, FOLLOW THE PROTOCOL
                        from.1 = 7-from.1;
                        to.1 = 7-to.1;
                        // ! FLIP Y, FOLLOW THE PROTOCOL

                        match self.board.get_destinations(from) {
                            Destinations::None => {
                                self.bad_move_s2c();
                            },
                            Destinations::Exists(d) => {
                                if d.contains(&to) {
                                    println!("server performing clients move");
                                    self.board.move_from_to(from, to);
                                    self.ok_move_s2c();

                                    // UPDATING STATE
                                    self.state = State::Playing;
                                } else {
                                    self.bad_move_s2c();
                                }
                            }
                        }

                    },
                    networking::c2s_message::Msg::ConnectRequest(msg_cr) => {
                        let id = msg_cr.game_id;
                        let spectate = msg_cr.spectate;
                    }
                }
            }
        }
    }

    // client recieving packet
    fn recieve_packet_s2c(&mut self) {
        let mut buf: [u8; 512] = [0_u8; 512];
        let buf_len = match self.stream.read(&mut buf) {
            Ok(l) => l,
            Err(e) => 0,
        };

        let raw_msg = networking::S2cMessage::decode(&buf[..buf_len]).expect("read went wrong");
        match raw_msg.msg {
            None => (),
            Some(msg) => {
                match msg {
                    s2c_message::Msg::Move(msg_m) => {
                        let mut from = self.square_to_xy(msg_m.from_square);
                        let mut to = self.square_to_xy(msg_m.to_square);
                        let promotion = msg_m.promotion;

                        // FLIP Y, FOLLOW THE PROTOCOL
                        from.1 = 7-from.1;
                        to.1 = 7-to.1;
                        // ! FLIP Y, FOLLOW THE PROTOCOL


                        self.board.move_from_to(from, to);

                        // UPDATING STATE
                        self.state = State::Playing;
                        
                    },
                    s2c_message::Msg::ConnectAck(msg_ca) => {
                        let success = msg_ca.success;
                        let id = msg_ca.game_id;
                        let starting_pos = msg_ca.starting_position;
                        let I_am_white = msg_ca.client_is_white;
                    },
                    s2c_message::Msg::MoveAck(msg_ma) => {
                        let legal = msg_ma.legal;
                        let board_state = msg_ma.board_result;
                    },
                    
                }
            }
        }
        
    }

    // server sending to client, client made bad move!
    fn bad_move_s2c(&self) {
        println!("client tried bad move");
        let ack = networking::S2cMoveAck {
            legal: false,
            board_result: Some(networking::BoardState {
                fen_string: self.board.get_fen(),
            }),
        };
    }

    fn ok_move_s2c(&self) {
        println!("client tried bad move");
        let ack = networking::S2cMoveAck {
            legal: true,
            board_result: Some(networking::BoardState {
                fen_string: self.board.get_fen(),
            }),
        };
    }

    pub fn square_to_xy(&self, s: u32) -> (usize, usize) {
        let y = (s as f32 / 8.0).floor() as usize;
        let x = (s % 8) as usize;
        (x, y)
    }
    pub fn xy_to_square(&self, xy: (usize, usize)) -> u32 {
        (xy.0+xy.1*8) as u32
    }
    // fn send_move_packet_C2s() {
    //     let mut buf: [u8; 512] = [0_u8; 512];

    // }
    // fn recieve_packet_s2C() {
    //     let mut buf: [u8; 512] = [0_u8; 512];

    // }
    // fn recieve_packet_C2s() {
    //     let mut buf: [u8; 512] = [0_u8; 512];
        
    // }

    // fn recieve_packet() {
    //     let mut buf: [u8; 512] = [0_u8; 512];
    //     let n = self.stream.read(&mut buf).expect("could not read stream.");
    //     let packet =
        
        
    // }

    // fn connect(&self) {
    //     let mut buf: [u8; 512] = [0_u8; 512];
    //     let n = self.stream.read(&mut buf).expect("could not read stream.");

    //     let packet = networking:
    // }


    // /// Sends a move packet of the current position and sets the state to waiting
    // fn send_move_packet(&mut self, move_to_send: networking::Move) {
    //     let mut buf: [u8; 512] = [0_u8; 512];
    //     prost::Message::encode(&self, buf)
    //     self.stream
    //         .write(&mut buf)
    //         .expect("Failed to send move packet");
    //     self.state = State::WaitingForOpponent;
    // }


    fn update_mouse_select(&mut self, _ctx: &mut Context) {
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
                                        println!("moving to empty square");
                                        // THEN WE CAN MAKE OUR MOVE
                                        self.make_move(cur_sel_xy, new_sel_xy);
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
                                            self.make_move(cur_sel_xy, new_sel_xy);
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
    
    }

    fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) {
        // THE MOVE
        match self.role {
            Role::Server => {
                self.board.move_from_to(from, to);
                self.send_move_packet_s2c(from, to, None);
            },
            Role::Client => {
                self.board.move_from_to(from, to);
                // SEND MOVE NETWORKING
                self.send_move_packet_c2s(from, to, None);
                // self.expecting_move_ack = true;
            }
        }
        // ! THE MOVE

        // OBS RESET SELECT
        self.cur_selected_xy = SelectedXY::None;
        // UPDATE STATE
        self.state = State::WaitingForOpponent;

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
    fn get_image(&self, p: &chess_lib::Piece) -> &graphics::Image {
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

        // NETWORKING
        if !self.connected {
            match self.role {
                Role::Client => {
                    self.send_connect_request_c2s();
                    self.expect_connect_ack_c2s();
                    // match self.state {
                    //     State::Playing => {
                    //         self.send_connect_request_c2s();
                    //         self.expect_connect_ack_c2s();
                    //         self.state = State::WaitingForOpponent;
                    //     },
                    //     _ => (),
                    // }
                    // self.connected = true;
                }, 
                Role::Server => {
                    self.expect_connect_request_s2c();
                    // match self.state {
                    //     State::Playing => {
                    //         self.expect_connect_request_s2c();
                    //     },
                    //     _ => (),
                    // }
                    // self.connected = true;
                },
            }
        } else if self.expecting_move_ack {
            match self.role {
                Role::Client => {
                    self.expect_move_ack_c2s();
                },
                Role::Server => {
                    panic!("Server should not expect move ack");
                }
            }
        } else {
            if self.expecting_move_ack {
                self.expect_move_ack_c2s();
            }
            match self.state {
                State::Playing => {
                    self.update_mouse_select(_ctx);
                },
                State::WaitingForOpponent => {
                    // If we recieved at move packet we first set the enemy pos to the recieved
                    // position and then set the state to playing
                    
                    match self.role {
                        Role::Client => {
                            self.recieve_packet_s2c();
                        },
                        Role::Server => {
                            self.recieve_packet_c2s();
                        }
                    }
                }
            }
        }
        // ! NETWORKING

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

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

enum SelectedXY {
    None,
    Selected((usize, usize)),
}

enum Role {
    Server,
    Client
}