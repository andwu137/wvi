use super::file_buffer::FileBuffer;
use device_query::Keycode;
use std::collections::{HashMap, VecDeque};

pub type Command<M> = Box<dyn Fn(&mut M, &mut FileBuffer) -> std::io::Result<()> + Send + Sync>;

pub struct Parser<M> {
    keys: Vec<Keycode>,
    command: Command<M>,
}

impl<M> Parser<M> {
    pub fn new(ks: Vec<Keycode>, c: Command<M>) -> Option<Parser<M>> {
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

pub struct InputParser<M> {
    input_buf: Vec<Keycode>,
    parser: ParserTree<M>,
}

impl<M> InputParser<M> {
    pub fn new(parsers: impl Iterator<Item = Parser<M>>) -> InputParser<M> {
        let mut parser_tree = ParserTree::new();
        parser_tree.add_all(parsers);

        InputParser {
            input_buf: Vec::new(),
            parser: parser_tree,
        }
    }

    pub fn add_all(&mut self, parsers: impl Iterator<Item = Parser<M>>) {
        self.parser.add_all(parsers)
    }

    pub fn accept(&mut self, key: Keycode) {
        self.input_buf.push(key);
    }

    pub fn accept_all(&mut self, keys: impl Iterator<Item = Keycode>) {
        for k in keys {
            self.accept(k);
        }
    }

    pub fn lookup_with(
        &mut self,
        mode: &mut M,
        buf: &mut FileBuffer,
        keys: impl Iterator<Item = Keycode>,
    ) -> std::io::Result<ParseState<(), Vec<Keycode>>> {
        let result = self.parser.get_commands_at(keys);
        match result {
            ParseState::Failed(()) => Ok(ParseState::Failed(self.reset())),
            ParseState::Unfinished => Ok(ParseState::Unfinished),
            ParseState::Success(commands) => {
                for c in commands {
                    c(mode, buf)?;
                }
                self.reset();
                Ok(ParseState::Success(()))
            }
        }
    }

    pub fn lookup(
        &mut self,
        mode: &mut M,
        buf: &mut FileBuffer,
    ) -> std::io::Result<ParseState<(), Vec<Keycode>>> {
        // NOTE: naive
        self.lookup_with(mode, buf, self.input_buf.clone().into_iter())
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

struct ParserTree<M> {
    commands: VecDeque<Command<M>>,
    children: HashMap<Keycode, ParserTree<M>>,
}

pub enum ParseState<S, F> {
    Failed(F),
    Unfinished,
    Success(S),
}

impl<M> ParserTree<M> {
    fn new() -> ParserTree<M> {
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

    fn add_command(&mut self, c: Command<M>) {
        self.commands.push_back(c)
    }

    fn add_all(&mut self, parsers: impl Iterator<Item = Parser<M>>) {
        for p in parsers {
            self.add(p);
        }
    }

    pub fn add(&mut self, parser: Parser<M>) {
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
        &self,
        keys: impl Iterator<Item = Keycode>,
    ) -> ParseState<&VecDeque<Command<M>>, ()> {
        let mut parser_tree = self;
        for k in keys {
            dbg!(k);
            match parser_tree.children.get(&k) {
                None => return ParseState::Failed(()),
                Some(t) => parser_tree = t,
            }
        }

        if parser_tree.is_empty() {
            ParseState::Failed(())
        } else if parser_tree.commands.len() == 0 {
            ParseState::Unfinished
        } else {
            ParseState::Success(&parser_tree.commands)
        }
    }
}
