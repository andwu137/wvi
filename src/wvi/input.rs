use super::file_buffer::FileBuffer;
use device_query::Keycode;
use std::collections;

type Command = dyn Fn(&mut FileBuffer) -> std::io::Result<()> + Sync + Send;

pub struct Parser<'a> {
    keys: Vec<Keycode>,
    command: &'a Command,
}

impl<'a> Parser<'a> {
    pub fn new(ks: Vec<Keycode>, c: &'a Command) -> Parser<'a> {
        Parser {
            keys: ks,
            command: c,
        }
    }
}

pub struct InputParser<'a> {
    input_buf: Vec<Keycode>,
    parser: ParserTree<'a>,
}

impl<'a> InputParser<'a> {
    pub fn new(parsers: Vec<Parser<'a>>) -> InputParser<'a> {
        let mut parser_tree = ParserTree::new();
        for p in parsers {
            parser_tree.add(p);
        }

        InputParser {
            input_buf: Vec::new(),
            parser: parser_tree,
        }
    }

    pub fn accept(
        &mut self,
        key: Keycode,
        buf: &mut FileBuffer,
    ) -> std::io::Result<Option<Vec<Keycode>>> {
        self.input_buf.push(key);
        // NOTE: naive
        let result = self
            .parser
            .run_command(self.input_buf.clone().into_iter(), buf);
        match result {
            ParseState::Failed => Ok(Some(self.reset())),
            ParseState::Unfinished => Ok(None),
            ParseState::Success(r) => {
                self.reset();
                r.map(|_| None)
            }
        }
    }

    pub fn reset(&mut self) -> Vec<Keycode> {
        let r = self.input_buf.clone();
        self.input_buf.clear();
        r
    }
}

enum ParserTree<'a> {
    Finished(&'a Command),
    Branch(collections::HashMap<Keycode, ParserTree<'a>>),
}

enum ParseState<T> {
    Failed,
    Unfinished,
    Success(T),
}

impl<'a> ParserTree<'a> {
    fn new() -> ParserTree<'a> {
        ParserTree::Branch(collections::HashMap::new())
    }

    pub fn add(&mut self, parser: Parser<'a>) -> bool {
        let mut p = self;
        for k in parser.keys {
            match p {
                ParserTree::Finished(_) => return false,
                ParserTree::Branch(h) => {
                    p = h.entry(k).or_insert_with(ParserTree::new);
                }
            }
        }

        match p {
            ParserTree::Finished(_) => false,
            ParserTree::Branch(_) => {
                *p = ParserTree::Finished(parser.command);
                true
            }
        }
    }

    pub fn run_command(
        &mut self,
        keys: impl Iterator<Item = Keycode>,
        buf: &mut FileBuffer,
    ) -> ParseState<std::io::Result<()>> {
        let mut p = self;
        for k in keys {
            dbg!(k);
            if let ParserTree::Branch(h) = p {
                match h.get_mut(&k) {
                    None => return ParseState::Failed,
                    Some(t) => p = t,
                }
            }
        }

        match p {
            ParserTree::Finished(c) => ParseState::Success(c(buf)),
            ParserTree::Branch(_) => ParseState::Unfinished,
        }
    }
}
