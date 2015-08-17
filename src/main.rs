#![feature(box_syntax, box_patterns, slice_patterns)]
#[macro_use]
extern crate glium;
extern crate image;
extern crate num;
extern crate nalgebra;
extern crate rand;
extern crate noise;
extern crate uuid;
extern crate time;
extern crate threadpool;
#[macro_use]
extern crate engine;

mod object;
mod game;
mod tile;

pub use nalgebra as na;
use std::path::PathBuf;
use threadpool::ThreadPool;
use na::Vec2;
use glium::{Display, Surface};
use engine::{Texture, Manager, WidgetBuilder, Sprite, Update, Label, Renderer,
             Engine, Camera, Renderable, EventStream, Event, WindowEvent,
             build_display};
use engine::timer::Ms;
use object::Block;
use tile::{Tile, TileGen};
use game::{Game, Input, Output};

const TILE: &'static str = "assets/tile";
const FONT: &'static str = "assets/font.otf";



fn game_path() -> PathBuf {
    PathBuf::new()
}


struct Env<'a> {
    #[allow(dead_code)]
    display: &'a Display,
    font: PathBuf,
    engine: Engine<'a>,
    textures: Manager<'a, Texture>,

}


impl<'a> Env<'a> {
    fn new(display: &'a Display) -> Env<'a> {
        let path = game_path();
        Env {
            display: display,
            engine: Engine::new(display),
            textures: Manager::new(display, path.join(TILE)),
            font: path.join(FONT),
        }
    }

    fn update(&mut self) {
        self.engine.update();
        std::thread::sleep_ms(5);
    }

    #[inline]
    fn now(&self) -> Ms {
        self.engine.timer.now()
    }
}


fn main() {
    let turn_time = 250;
    let display = build_display("sansa".to_string(), (800, 600));
    let renderer = Renderer::new(&display);
    let pool = ThreadPool::new(8);

    let mut env = Env::new(&display);

    let mut game = game::Game::new((100, 100));
    let tile = TileGen::new(&env.textures);
    let mut last_turn = env.now();
    let mut ground = make_tiles(&game, &tile);
    let mut offset: Vec2<i32> = na::zero();
    let mut game_camera = Camera::new(&display);
    let mut ui_camera = Camera::new(&display);

    let test_text = "你们有一个好，全世界跑到什么地方，你们比其他的西方记者啊跑得还快。\n但是呢问来问去的问题啊，too simple，啊，sometimes naïve！";
    let text_style = engine::text::TextStyle::new(env.font.clone());
    let fps_style = engine::text::TextStyle::new(env.font.clone());
    let mut label = env.engine.label(text_style, &test_text)
            .anchor(na![-1.0, -1.0])
            .position(ui_camera.right_top())
            .build(&pool);

    'main: loop {
        let stream = { // update
            let stream = EventStream::new(&display);
            let mut queue: Vec<&mut Update> = Vec::new();
            queue.push(&mut label);
            let delta = env.engine.timer.delta;
            game_camera.update(delta);
            ui_camera.update(delta);
            queue.update(delta, stream)
        };
        // println!("FPS: {:3}", env.engine.timer.fps());
        for e in stream.iter() {
            use glium::glutin::ElementState;

            if let &Event::Window(ref e) = e {
                match e {
                    &WindowEvent::KeyboardInput(ElementState::Released, _, Some(x)) => {
                        use glium::glutin::VirtualKeyCode::*;

                        offset = match x {
                            W => na![ 1,  1],
                            S => na![-1, -1],
                            A => na![-1,  1],
                            D => na![ 1, -1],
                            Q => na![ 0,  1],
                            E => na![ 1,  0],
                            Z => na![-1,  0],
                            X => na![ 0, -1],
                            _ => na![ 0,  0],
                        }
                    }
                    _ => {}
                }
            }
        }
        let now = env.now();
        if now - last_turn >= turn_time {
            ground = make_tiles(&game, &tile);
            let output = game.next(Input::Move ((offset.x, offset.y)));
            last_turn = now;
            match output {
                Output::Move(offset) => {
                    let v = tile.vertical();
                    let h = tile.horizontal();
                    game_camera.reset();
                    game_camera.move_(turn_time, na::cast(match offset {
                        ( 0,  0) => na![ 0,  0],
                        ( 1,  1) => na![ 0,  v],
                        (-1, -1) => na![ 0, -v],
                        (-1,  1) => na![-h,  0],
                        ( 1, -1) => na![ h,  0],
                        ( 0,  1) => na![-h/2,  v/2],
                        ( 1,  0) => na![ h/2,  v/2],
                        (-1,  0) => na![-h/2, -v/2],
                        ( 0, -1) => na![ h/2, -v/2],
                        _ => unreachable!()
                    }));
                }
            }
            offset = na::zero();
        }
        // render
        let mut target = display.draw();
        renderer.render(&mut target, &ground.iter().collect::<Vec<_>>(), game_camera.matrix());
        renderer.render::<Vec<&Renderable>>(&mut target, &vec![&label], ui_camera.matrix());
        target.finish().unwrap();
        env.update();
        for e in stream.iter() {
            match e {
                &Event::Window(WindowEvent::Closed) => break 'main,
                _ => {}
            }
        }
    }
}


fn make_tiles(game: &Game, tile: &TileGen) -> Vec<Sprite> {
    let mut sprites = Vec::new();
    for k in 0..game::LAYER {
        for j in -20..20 {
            for i in -20..20 {
                let unit = game.get([i, j, k]);
                let block = unit.block;
                let role = unit.role;
                let pos = (-i+k, -j+k);
                if let Block::Nil = block {} else {
                    sprites.push(tile.sprite(block.offset(), pos));
                }
                if let Some(role) = role {
                    sprites.push(tile.sprite(role.offset(), pos));
                }
            }
        }
    }
    return sprites;
}
