use super::file_buffer::FileBuffer;
use std::collections;

struct BufferMap {
    buffers: collections::HashMap<String, FileBuffer>,
}

impl BufferMap {
    fn add_buffer(&mut self, name: &String, buf: FileBuffer) -> Option<FileBuffer> {
        self.add_buffer_owned(name.clone(), buf)
    }

    fn add_buffer_owned(&mut self, name: String, buf: FileBuffer) -> Option<FileBuffer> {
        self.buffers.insert(name, buf)
    }

    fn rm_buffer(&mut self, name: &String) -> Option<FileBuffer> {
        self.buffers.remove(name)
    }

    fn get_buffer(&self, name: &String) -> Option<&FileBuffer> {
        self.buffers.get(name)
    }

    fn get_mut_buffer(&mut self, name: &String) -> Option<&mut FileBuffer> {
        self.buffers.get_mut(name)
    }
}
