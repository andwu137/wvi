mod wvi;
use std::sync::Mutex;
use std::time::Duration;

use device_query::DeviceEvents;
use device_query::{self, Keycode};
use wvi::file_buffer::FileBuffer;
use wvi::input::{Input, InputParser};

const FILE: &str = "test_file";

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

fn main() -> std::io::Result<()> {
    let device_state = device_query::DeviceState::new();

    let search_keys = vec![Keycode::Space, Keycode::S, Keycode::F];
    let switch_keys = vec![Keycode::Space, Keycode::H, Keycode::F];
    let write_keys = vec![Keycode::Space, Keycode::F, Keycode::W];

    let write = InputParser::new(write_keys, &write);
    let s = InputParser::new(search_keys, &search);
    let h = InputParser::new(switch_keys, &switch);

    let input_mutex = Mutex::new(Input::new(vec![], vec![write, s, h]));

    let buf_mutex = Mutex::new(FileBuffer::load_file(FILE)?);

    let _guard = device_state.on_key_down(move |key| {
        let mut data = input_mutex.lock().unwrap();
        let mut buf = buf_mutex.lock().unwrap();
        println!("Keyboard key down: {:#?}", key);
        data.run(&mut (*buf), key);
    });

    loop {
        std::thread::sleep(Duration::new(0, 100_000));
    }
}
