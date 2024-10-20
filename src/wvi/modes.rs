mod command;
mod insert;
mod normal;

struct V2 {
    x: usize,
    y: usize,
}

impl V2 {
    fn new(x: usize, y: usize) -> V2 {
        V2 { x, y }
    }

    fn add(&self, v: &V2) -> V2 {
        V2 {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }

    fn sat_sub(&self, v: &V2) -> V2 {
        V2 {
            x: self.x.saturating_sub(v.x),
            y: self.y.saturating_sub(v.y),
        }
    }

    fn clamp(&self, v_mx: &V2) -> V2 {
        V2 {
            x: std::cmp::min(self.x, v_mx.x),
            y: std::cmp::min(self.y, v_mx.y),
        }
    }
}

#[derive(Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
