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
        let new_pos = match dir {
            // TODO: ASSERT
            Direction::Up => self.cursor.add(&V2::new(0, 1)),
            Direction::Down => self.cursor.sat_sub(&V2::new(0, 1)),
            Direction::Left => self.cursor.sat_sub(&V2::new(1, 0)),
            Direction::Right => self.cursor.add(&V2::new(1, 0)),
        };
    }

    fn display_cursor(&self, buf: &FileBuffer) -> V2 {
        assert!(self.cursor.y >= buf.len());
        V2::clamp(
            &self.cursor,
            &V2::new(buf.line_len(self.cursor.y).unwrap(), self.cursor.y),
        )
    }
}
