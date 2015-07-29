#![feature(box_syntax, box_patterns, slice_patterns, rc_weak)]
#[macro_use]
extern crate glium;
extern crate image as img;
extern crate num;
extern crate nalgebra as na;
extern crate time;
extern crate uuid;
extern crate freetype;
extern crate unicode_normalization;


macro_rules! na {
    ($x: expr) =>
        (::na::Vec1::new($x));
    ($x: expr, $y: expr) =>
        (::na::Vec2::new($x, $y));
    ($x: expr, $y: expr, $z: expr) =>
        (::na::Vec3::new($x, $y, $z));
    ($x: expr, $y: expr, $z: expr, $w: expr) =>
        (::na::Vec4::new($x, $y, $z, $w));
}


pub mod canvas;
pub mod color;
pub mod event;
pub mod math;
pub mod resources;
pub mod sprite;
pub mod timer;
pub mod ui;
pub mod animation;
mod transform;
mod mesh;
mod context;
mod image;
mod camera;
mod renderable;
mod texture;
mod engine;


pub use glium::{Frame, Display};
pub use timer::{ProgramTimer, Timer, Ms};
pub use context::Context;
pub use image::Image;
pub use renderable::{Renderable, render};
pub use sprite::Sprite;
pub use event::{Event, Update, update};
pub use ui::{GlyphCache, Glyph, Text, UI, UIBuilder};
pub use resources::Manager;
pub use transform::Transform;
pub use texture::Texture;
pub use engine::Engine;

use glium::glutin::WindowBuilder;
use glium::DisplayBuild;




pub fn build_display(title: String, (width, height): (u32, u32)) -> Display {
    WindowBuilder::new()
        .with_title(title)
        .with_dimensions(width, height)
        .with_vsync()
        .build_glium()
        .unwrap()
}


