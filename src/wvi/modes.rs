pub mod command;
pub mod insert;
pub mod mode_utils;
pub mod normal;

use super::{file_buffer::FileBuffer, input::ParseState};
use device_query::Keycode;

pub type BoxMode = Box<dyn Mode + Send + Sync>;

#[derive(Debug)]
pub struct ModeInit {
    pub cursor_pos: V2,
}

pub trait Mode {
    fn new(init: ModeInit) -> Self
    where
        Self: Sized;

    fn accept(
        &mut self,
        buf: &mut FileBuffer,
        key: Keycode,
    ) -> std::io::Result<ParseState<(), Vec<Keycode>>>;
}

#[derive(Eq, PartialEq, Debug)]
pub struct V2 {
    x: usize,
    y: usize,
}

impl V2 {
    pub fn new(x: usize, y: usize) -> V2 {
        V2 { x, y }
    }

    pub fn add(&self, v: &V2) -> V2 {
        V2 {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }

    pub fn sat_sub(&self, v: &V2) -> V2 {
        V2 {
            x: self.x.saturating_sub(v.x),
            y: self.y.saturating_sub(v.y),
        }
    }

    pub fn clamp(&self, v_mx: &V2) -> V2 {
        V2 {
            x: self.x.min(v_mx.x),
            y: self.y.min(v_mx.y),
        }
    }

    fn clamp_x(&self, x_mx: usize) -> V2 {
        self.clamp(&V2::new(x_mx, self.y))
    }

    fn clamp_y(&self, y_mx: usize) -> V2 {
        self.clamp(&V2::new(self.x, y_mx))
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Dir2 {
    Up,
    Down,
    Left,
    Right,
}
