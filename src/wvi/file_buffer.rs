use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct FileBuffer {
    file: Vec<Vec<char>>,
}

//file_buffer::load_file(&name).unwrap()
pub fn load_file(name: &String) -> std::io::Result<FileBuffer> {
    let f = File::open(name)?;
    let lines = BufReader::new(f)
        .lines()
        .flat_map(|line| line.map(|l| l.chars().collect::<Vec<char>>()))
        .collect();
    Ok(FileBuffer { file: lines })
}

//file_buffer::write_file(&name, &file_buffer).unwrap().unwrap();
pub fn write_file(
    name: &String,
    buf: &FileBuffer,
) -> Result<std::io::Result<()>, std::string::FromUtf8Error> {
    Ok(std::fs::write(
        name,
        String::from_utf8(
            buf.file
                .iter()
                .flat_map(|l| {
                    l.into_iter()
                        .flat_map(|c| {
                            let mut b = [0; 4];
                            let result = c.encode_utf8(&mut b);
                            result.bytes().collect::<Vec<_>>().into_iter()
                        })
                        .chain(std::iter::once(b'\n'))
                })
                .collect::<Vec<_>>(),
        )?,
    ))
}
