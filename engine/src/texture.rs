use img;
use uuid::Uuid;
use glium::Display;
use glium::texture::{Texture2dDataSource, Texture2d};
use std::cmp::{PartialEq, Eq};
use std::path::Path;
use resources::Resource;

pub struct Texture {
    pub id: Uuid,
    pub data: Texture2d,
}


impl Texture {
    pub fn new<'a, T>(display: &Display, source: T) -> Texture
            where T: Texture2dDataSource<'a> {
        Texture {
            id: Uuid::new_v4(),
            data: Texture2d::new(display, source).unwrap(),
        }
    }
}


impl PartialEq<Texture> for Texture {
    fn eq(&self, other: &Texture) -> bool {
        self.id == other.id
    }
}


impl Eq for Texture {}


impl Resource for Texture {
    fn load(display: &Display, path: &Path) -> Texture {
        Texture::new(display, img::open(path).unwrap())
    }
}
