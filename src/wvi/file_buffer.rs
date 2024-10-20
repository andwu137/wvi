use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct FileBuffer {
    pub file: Vec<Vec<char>>,
}

impl FileBuffer {
    //file_buffer::load_file(&name).unwrap()
    pub fn load_file<R>(name: R) -> std::io::Result<FileBuffer>
    where
        R: AsRef<std::path::Path>,
    {
        let f = File::open(name)?;
        let lines = BufReader::new(f)
            .lines()
            .flat_map(|line| line.map(|l| l.chars().collect::<Vec<char>>()))
            .collect();
        Ok(FileBuffer { file: lines })
    }

    pub fn write_file(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(
            self.file
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
        )
    }

    pub fn append(&mut self, newline: Vec<char>) {
        self.file.append(&mut vec![newline]);
    }
}
