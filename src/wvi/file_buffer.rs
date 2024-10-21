use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct FileBuffer {
    file: Vec<Vec<char>>,
}

impl FileBuffer {
    pub fn len(&self) -> usize {
        self.file.len()
    }

    pub fn line_len(&self, y: usize) -> Option<usize> {
        self.file.get(y).map(|l| l.len())
    }

    pub fn get_line(&self, y: usize) -> Option<&Vec<char>> {
        self.file.get(y)
    }

    pub fn get_line_mut(&mut self, y: usize) -> Option<&mut Vec<char>> {
        self.file.get_mut(y)
    }

    pub fn get(&self, y: usize, x: usize) -> Option<&char> {
        self.get_line(y)?.get(x)
    }

    pub fn get_mut(&mut self, y: usize, x: usize) -> Option<&mut char> {
        self.get_line_mut(y)?.get_mut(x)
    }

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
