mod wvi;

use device_query::DeviceEvents;
use device_query::Keycode;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::PoisonError;
use std::time::Duration;
use wvi::file_buffer::FileBuffer;
use wvi::input::InputParser;
use wvi::input::Parser;

const FILE: &str = "test_inputs/random_file.txt";

fn search(buf: &mut FileBuffer) -> std::io::Result<()> {
    let msg = "search";
    println!("{}", msg);
    buf.append(msg.chars().collect());
    Ok(())
}

fn switch(buf: &mut FileBuffer) -> std::io::Result<()> {
    let msg = "switch";
    println!("{}", msg);
    buf.append(msg.chars().collect());
    Ok(())
}

fn write(buf: &mut FileBuffer) -> std::io::Result<()> {
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
        Parser::new(write_keys, write).unwrap(),
        Parser::new(search_keys, search).unwrap(),
        Parser::new(override_write_keys.clone(), write).unwrap(),
        Parser::new(switch_keys, switch).unwrap(),
    ];

    let parser_mutex = Mutex::new(InputParser::new(parsers));
    {
        let mut parser = parser_mutex.lock().unwrap();
        parser.remove(&override_write_keys);
    }

    let buf_mutex = Mutex::new(FileBuffer::load_file(FILE)?);

    let _guard = device_state.on_key_down(move |key| {
        let mut parser = default_block_lock(&parser_mutex).unwrap();
        let mut buf = default_block_lock(&buf_mutex).unwrap();
        println!("Keyboard key down: {:#?}", key);
        match parser.accept(*key, &mut (*buf)) {
            Err(e) => println!("{}", e),
            Ok(None) => {}
            Ok(Some(failed)) => println!("{:?}", failed),
        }
    });

    loop {
        std::thread::sleep(Duration::new(0, 100_000));
    }
}
