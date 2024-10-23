use std::sync::{LazyLock, Mutex};

use super::mode_utils;
use super::Mode;
use crate::wvi::file_buffer::FileBuffer;
use crate::wvi::input::{Command, InputParser, ParseState, Parser};
use crate::wvi::modes::{Dir2, V2};
use device_query::Keycode;

static PARSER: LazyLock<Mutex<InputParser<Normal>>> = LazyLock::new(|| {
    Mutex::new(InputParser::new(
        vec![
            Parser::new(vec![Keycode::Q], Box::new(|_mode, _buf| Ok(()))).unwrap(),
            Parser::new(vec![Keycode::H], Normal::move_cursor_curry(Dir2::Left)).unwrap(),
            Parser::new(vec![Keycode::J], Normal::move_cursor_curry(Dir2::Down)).unwrap(),
            Parser::new(vec![Keycode::K], Normal::move_cursor_curry(Dir2::Up)).unwrap(),
            Parser::new(vec![Keycode::L], Normal::move_cursor_curry(Dir2::Right)).unwrap(),
        ]
        .into_iter(),
    ))
});

pub struct Normal {
    cursor: V2,
}

impl Mode for Normal {
    fn accept(
        &mut self,
        buf: &mut FileBuffer,
        key: Keycode,
    ) -> std::io::Result<ParseState<(), Vec<Keycode>>> {
        let mut p = PARSER.lock().unwrap(); // WARN: unwrap here
        p.accept(key);
        p.lookup(self, buf)
    }
}

impl Normal {
    pub fn new(cursor_pos: V2) -> Normal {
        Normal { cursor: cursor_pos }
    }

    fn move_cursor_curry(dir: Dir2) -> Command<Normal> {
        Box::new(move |mode, buf| {
            println!("{:?}", dir);
            mode.move_cursor(buf, dir);
            Ok(())
        })
    }

    fn move_cursor(&mut self, buf: &FileBuffer, dir: Dir2) {
        self.cursor = mode_utils::move_cursor(&self.cursor, buf, dir);
    }

    fn display_cursor(&self, buf: &FileBuffer) -> V2 {
        mode_utils::display_cursor(&self.cursor, buf)
    }
}
