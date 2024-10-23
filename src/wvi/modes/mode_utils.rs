use crate::wvi::file_buffer::FileBuffer;

use super::{Dir2, V2};

// cursor must remain in bounds,
// -> not go beyond rightmost character on that line
// -> not go beyond lowest line in buf
pub fn move_cursor(cursor: &V2, buf: &FileBuffer, dir: Dir2) -> V2 {
    let cur = match dir {
        Dir2::Up => cursor.sat_sub(&V2::new(0, 1)),
        Dir2::Down => cursor.add(&V2::new(0, 1)).clamp_y(buf.len() - 1),
        Dir2::Left => display_cursor(cursor, buf).sat_sub(&V2::new(1, 0)),
        Dir2::Right => display_cursor(cursor, buf)
            .add(&V2::new(1, 0))
            .clamp_x(buf.line_len(cursor.y).unwrap() - 1),
    };

    assert!(cur.y < buf.len());
    cur
}

pub fn display_cursor(cursor: &V2, buf: &FileBuffer) -> V2 {
    assert!(cursor.y < buf.len());
    let cur = cursor.clamp(&V2::new(buf.line_len(cursor.y).unwrap() - 1, cursor.y));
    assert!(cur.y < buf.len() && cur.x < buf.line_len(cur.y).unwrap());
    cur
}
