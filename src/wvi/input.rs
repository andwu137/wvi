use super::file_buffer::FileBuffer;
use device_query::Keycode;
use std::collections::{HashMap, VecDeque};

type Command = fn(&mut FileBuffer) -> std::io::Result<()>;

pub struct Parser {
    keys: Vec<Keycode>,
    command: Command,
}

impl Parser {
    pub fn new(ks: Vec<Keycode>, c: Command) -> Option<Parser> {
        if 0 < ks.len() {
            Some(Parser {
                keys: ks,
                command: c,
            })
        } else {
            None
        }
    }
}

pub struct InputParser {
    input_buf: Vec<Keycode>,
    parser: ParserTree,
}

impl InputParser {
    pub fn new(parsers: Vec<Parser>) -> InputParser {
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
            .get_commands_at(self.input_buf.clone().into_iter());
        match result {
            ParseState::Failed => Ok(Some(self.reset())),
            ParseState::Unfinished => Ok(None),
            ParseState::Success(commands) => {
                for c in commands {
                    c(buf)?;
                }
                self.reset();
                Ok(None)
            }
        }
    }

    pub fn remove(&mut self, keys: &Vec<Keycode>) {
        self.parser.remove(keys)
    }

    pub fn reset(&mut self) -> Vec<Keycode> {
        let r = self.input_buf.clone();
        self.input_buf.clear();
        r
    }
}

struct ParserTree {
    commands: VecDeque<Command>,
    children: HashMap<Keycode, ParserTree>,
}

enum ParseState<T> {
    Failed,
    Unfinished,
    Success(T),
}

impl ParserTree {
    fn new() -> ParserTree {
        ParserTree {
            commands: VecDeque::new(),
            children: HashMap::new(),
        }
    }

    pub fn print_children(&self, depth: usize) {
        println!(": {}", self.commands.len());
        for (k, p) in &self.children {
            for _ in 0..depth {
                print!("  ");
            }
            print!("{}", k,);
            p.print_children(depth + 1);
        }
    }

    fn is_finished(&self) -> bool {
        self.commands.len() != 0
    }

    fn is_empty(&self) -> bool {
        self.commands.len() == 0 && self.children.len() == 0
    }

    fn add_command(&mut self, c: Command) {
        self.commands.push_back(c)
    }

    pub fn add(&mut self, parser: Parser) {
        let mut parser_tree = self;
        for k in parser.keys {
            println!("{}", k);
            parser_tree = parser_tree
                .children
                .entry(k)
                .or_insert_with(ParserTree::new);
        }

        parser_tree.add_command(parser.command);
    }

    pub fn remove(&mut self, keys: &Vec<Keycode>) {
        let mut parser_tree = self;
        for (i, key) in keys.iter().enumerate() {
            match parser_tree.children.get_mut(key) {
                None => return,
                Some(p) => {
                    if p.commands.len() == 0 as usize && p.children.len() == 0 as usize {
                        p.children.remove(key);
                        return;
                    } else if keys.len() - 1 == i {
                        p.commands.pop_front();
                        return;
                    } else {
                        parser_tree = p;
                    }
                }
            }
        }
    }

    pub fn get_commands_at(
        &mut self,
        keys: impl Iterator<Item = Keycode>,
    ) -> ParseState<&VecDeque<Command>> {
        let mut parser_tree = self;
        for k in keys {
            dbg!(k);
            match parser_tree.children.get_mut(&k) {
                None => return ParseState::Failed,
                Some(t) => parser_tree = t,
            }
        }

        if parser_tree.is_empty() {
            ParseState::Failed
        } else if parser_tree.commands.len() == 0 {
            ParseState::Unfinished
        } else {
            ParseState::Success(&parser_tree.commands)
        }
    }
}
