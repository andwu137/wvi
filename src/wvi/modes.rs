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

#[derive(Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
