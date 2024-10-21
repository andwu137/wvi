use super::file_buffer::FileBuffer;
use device_query::Keycode;

type Command = dyn Fn(&mut FileBuffer) -> std::io::Result<()> + Sync + Send;

pub struct Input<'a> {
    input_buf: Vec<Keycode>,
    parsers: Vec<InputParser<'a>>,
    parser_num: usize,
    _private: (),
}

impl<'a> Input<'a> {
    pub fn new(parsers: Vec<InputParser<'a>>) -> Input<'a> {
        Input {
            input_buf: vec![],
            parsers,
            parser_num: 0,
            _private: (),
        }
    }

    pub fn run(&mut self, buf: &mut FileBuffer, key: &Keycode) -> std::io::Result<()> {
        self.input_buf.push(key.clone());
        self.parse(buf, key)
    }

    fn parse(&mut self, buf: &mut FileBuffer, key: &Keycode) -> std::io::Result<()> {
        loop {
            match self.parsers[self.parser_num].parse(key) {
                Result::Command(c) => {
                    c(buf)?;
                    self.reset();
                    return Ok(());
                }
                Result::Continue => return Ok(()),
                Result::Fail => {
                    if !self.next_parser(buf)? {
                        self.reset();
                        return Ok(());
                    }
                }
            }
        }
    }

    // returns true if success
    fn reparse(&mut self, buf: &mut FileBuffer) -> std::io::Result<bool> {
        for k in self
            .input_buf
            .clone()
            .iter()
            .take(self.input_buf.len().saturating_sub(1))
        {
            match self.parsers[self.parser_num].parse(&k) {
                Result::Command(c) => {
                    c(buf)?;
                    // we must have parsed all the input_buf
                    self.reset();
                    return Ok(true);
                }
                Result::Fail => return Ok(false),
                Result::Continue => {}
            }
        }
        Ok(true)
    }

    // returns false if there are no more parsers
    fn next_parser(&mut self, buf: &mut FileBuffer) -> std::io::Result<bool> {
        self.parser_num = self.parser_num + 1;
        if self.parser_num >= self.parsers.len() {
            self.reset();
            Ok(false)
        } else if !self.reparse(buf)? {
            self.next_parser(buf)
        } else {
            Ok(true)
        }
    }

    fn reset(&mut self) {
        self.parser_num = 0;
        self.input_buf.clear();
    }
}

pub struct InputParser<'a> {
    parse_target: Vec<Keycode>,
    pos: usize,
    command: &'a Command,
}

enum Result<'a> {
    Command(&'a Command),
    Continue,
    Fail,
}

impl<'a> InputParser<'a> {
    pub fn new(t: Vec<Keycode>, c: &'a Command) -> InputParser {
        InputParser {
            parse_target: t,
            pos: 0,
            command: c,
        }
    }

    fn parse(&mut self, k: &Keycode) -> Result {
        let (curr_k, done) = self.next();
        if curr_k != *k {
            self.reset();
            return Result::Fail;
        }

        if done {
            self.reset();
            Result::Command(self.command)
        } else {
            Result::Continue
        }
    }

    fn next(&mut self) -> (Keycode, bool) {
        let x = self.parse_target[self.pos];
        self.pos += 1;
        (x, self.pos == self.parse_target.len())
    }

    fn reset(&mut self) {
        self.pos = 0;
    }
}
