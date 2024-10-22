use crate::wvi::file_buffer::FileBuffer;
use crate::wvi::modes::{Direction, V2};

struct Normal {
    cursor: V2,
}

impl Normal {
    fn new(cursor_pos: V2) -> Normal {
        Normal { cursor: cursor_pos }
    }

    // cursor must remain in bounds,
    // -> not go beyond rightmost character on that line
    // -> not go beyond lowest line in buf
    fn move_cursor(&mut self, buf: &FileBuffer, dir: Direction) {
        self.cursor = match dir {
            Direction::Up => self.cursor.sat_sub(&V2::new(0, 1)),
            Direction::Down => self.cursor.add(&V2::new(0, 1)).clamp_y(buf.len()),
            Direction::Left => self.display_cursor(buf).sat_sub(&V2::new(1, 0)),
            Direction::Right => self
                .display_cursor(buf)
                .add(&V2::new(1, 0))
                .clamp_x(buf.line_len(self.cursor.y).unwrap()),
        };

        assert!(self.cursor.y < buf.len());
    }

    fn display_cursor(&self, buf: &FileBuffer) -> V2 {
        assert!(self.cursor.y < buf.len());
        let cur = self.cursor.clamp(&V2::new(
            buf.line_len(self.cursor.y).unwrap(),
            self.cursor.y,
        ));
        assert!(cur.y < buf.len());
        cur
    }
}
