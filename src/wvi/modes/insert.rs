use std::sync::{LazyLock, Mutex};

use super::{mode_utils, Mode, ModeInit};
use crate::wvi::file_buffer::FileBuffer;
use crate::wvi::input::{Command, InputParser, ParseState, Parser};
use crate::wvi::modes::{Dir2, V2};
use device_query::Keycode;

static PARSER: LazyLock<Mutex<InputParser<Insert>>> = LazyLock::new(|| {
    Mutex::new(InputParser::new(
        vec![
            Parser::new(vec![Keycode::T], Box::new(|_mode, _buf| Ok(()))).unwrap(),
            Parser::new(vec![Keycode::H], Insert::move_cursor_curry(Dir2::Left)).unwrap(),
            Parser::new(vec![Keycode::J], Insert::move_cursor_curry(Dir2::Down)).unwrap(),
            Parser::new(vec![Keycode::K], Insert::move_cursor_curry(Dir2::Up)).unwrap(),
            Parser::new(vec![Keycode::L], Insert::move_cursor_curry(Dir2::Right)).unwrap(),
        ]
        .into_iter(),
    ))
});

pub struct Insert {
    cursor: V2,
}

impl Mode for Insert {
    fn new(init: ModeInit) -> Insert {
        Insert {
            cursor: init.cursor_pos,
        }
    }

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

impl Insert {
    fn move_cursor_curry(dir: Dir2) -> Command<Insert> {
        Box::new(move |mode, buf| {
            //println!("");
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
