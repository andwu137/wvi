use super::file_buffer::FileBuffer;
use device_query::{self, Keycode};

type Command = dyn Fn(&mut FileBuffer) -> std::io::Result<()> + Sync + Send;

pub struct Input<'a> {
    input_buf: Vec<Keycode>,
    parsers: Vec<InputParser<'a>>,
    parser_num: usize,
    _private: (),
}

impl<'a> Input<'a> {
    pub fn new(ib: Vec<Keycode>, p: Vec<InputParser<'a>>) -> Input<'a> {
        Input {
            input_buf: ib,
            parsers: p,
            parser_num: 0,
            _private: (),
        }
    }

    pub fn run(&mut self, buf: &mut FileBuffer, key: &Keycode) {
        self.input_buf.push(key.clone());
        dbg!(key);
        self.parse(buf, key);
        dbg!(
            &self.input_buf,
            self.parser_num,
            self.parsers[self.parser_num].pos
        );
        println!("\n\n");
    }

    fn parse(&mut self, buf: &mut FileBuffer, key: &Keycode) {
        loop {
            println!("hi");
            match self.parsers[self.parser_num].parse(key) {
                Result::Command(c) => {
                    c(buf);
                    self.reset();
                    return;
                }
                Result::Continue => return,
                Result::Fail => {
                    if !self.next_parser(buf) {
                        self.reset();
                        return;
                    }
                }
            }
        }
    }

    // returns true if success
    fn reparse(&mut self, buf: &mut FileBuffer) -> bool {
        for k in self
            .input_buf
            .clone()
            .iter()
            .take(self.input_buf.len().saturating_sub(1))
        {
            match self.parsers[self.parser_num].parse(&k) {
                Result::Command(c) => {
                    c(buf);
                    // we must have parsed all the input_buf
                    self.reset();
                    return true;
                }
                Result::Fail => return false,
                Result::Continue => {}
            }
        }
        true
    }

    // returns false if there are no more parsers
    fn next_parser(&mut self, buf: &mut FileBuffer) -> bool {
        self.parser_num = self.parser_num + 1;
        if self.parser_num >= self.parsers.len() {
            self.reset();
            false
        } else if !self.reparse(buf) {
            self.next_parser(buf)
        } else {
            true
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
        dbg!(&self.parse_target, self.pos);
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
