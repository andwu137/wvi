mod wvi;

use device_query::{DeviceEvents, Keycode};
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::Duration;
use wvi::file_buffer::FileBuffer;
use wvi::input::{InputParser, ParseState, Parser};
use wvi::modes::insert::Insert;
use wvi::modes::normal::Normal;
use wvi::modes::{BoxMode, Mode, ModeInit, V2};

const FILE: &str = "test_inputs/random_file.txt";

fn search<M>(_mode: &mut M, buf: &mut FileBuffer) -> std::io::Result<()> {
    let msg = "search";
    println!("{}", msg);
    buf.append(msg.chars().collect());
    Ok(())
}

fn switch<M>(_mode: &mut M, buf: &mut FileBuffer) -> std::io::Result<()> {
    let msg = "switch";
    println!("{}", msg);
    buf.append(msg.chars().collect());
    Ok(())
}

fn write<M>(_mode: &mut M, buf: &mut FileBuffer) -> std::io::Result<()> {
    println!("{}", FILE);
    std::fs::write(
        FILE,
        buf.write_file()
            .expect("unable to convert file buffer to utf8"),
    )
}

fn block_lock<T>(
    m: &Mutex<T>,
    max_tries: usize,
    delay: Duration,
    on_error: fn(PoisonError<MutexGuard<'_, T>>),
) -> Option<MutexGuard<T>> {
    for _ in 1..max_tries {
        let result = m.lock();
        match result {
            Err(e) => on_error(e),
            Ok(guard) => return Some(guard),
        }
        std::thread::sleep(delay);
    }

    None
}

fn default_block_lock<T>(m: &Mutex<T>) -> Option<MutexGuard<T>> {
    block_lock(m, 100_000, Duration::new(0, 100_000), |e| println!("{}", e))
}

fn main() -> std::io::Result<()> {
    let device_state = device_query::DeviceState::new();

    let search_keys = vec![Keycode::Space, Keycode::S, Keycode::F];
    let switch_keys = vec![Keycode::Space, Keycode::H, Keycode::F];
    let write_keys = vec![Keycode::Space, Keycode::F, Keycode::W];
    let override_write_keys = vec![Keycode::Space, Keycode::F];

    let parsers = vec![
        Parser::new(write_keys, Box::new(write::<BoxMode>)).unwrap(),
        Parser::new(search_keys, Box::new(search)).unwrap(),
        Parser::new(override_write_keys.clone(), Box::new(write)).unwrap(),
        Parser::new(switch_keys, Box::new(switch)).unwrap(),
    ]
    .into_iter();

    let main_parser_mutex: Mutex<InputParser<_>> = Mutex::new(InputParser::new(parsers));
    {
        let mut parser = main_parser_mutex.lock().unwrap();
        parser.remove(&override_write_keys);
    }

    let buf_mutex = Mutex::new(FileBuffer::load_file(FILE)?);

    let mode_mutex: Mutex<BoxMode> = Mutex::new(Box::new(Normal::new(ModeInit {
        cursor_pos: V2::new(0, 0),
    })));
    {
        let mut mode = mode_mutex.lock().unwrap();
        *mode = Box::new(Insert::new(ModeInit {
            cursor_pos: V2::new(0, 0),
        }));
    }

    let _guard = device_state.on_key_down(move |key| {
        let mut buf = default_block_lock(&buf_mutex).unwrap();
        let mut mode = default_block_lock(&mode_mutex).unwrap();
        println!("Keyboard key down: {:#?}", key);
        match mode.accept(&mut (*buf), *key) {
            Err(e) => println!("{}", e),
            Ok(ParseState::Success(())) => {}
            Ok(ParseState::Unfinished) => {}
            Ok(ParseState::Failed(failed)) => {
                println!("{:?}", failed);
                let mut main_parser = default_block_lock(&main_parser_mutex).unwrap();
                main_parser.accept_all(failed.into_iter());
                match main_parser.lookup(&mut (*mode), &mut (*buf)) {
                    Err(e) => println!("{}", e),
                    Ok(ParseState::Success(())) => {}
                    Ok(ParseState::Unfinished) => {}
                    Ok(ParseState::Failed(failed)) => {
                        println!("{:?}", failed)
                    }
                }
            }
        }
        //mode.display();
    });

    loop {
        std::thread::sleep(Duration::new(0, 100_000));
    }
}
