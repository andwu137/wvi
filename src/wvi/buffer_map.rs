use crate::wvi::file_buffer;
use std::collections;

struct BufferMap {
    buffers: collections::HashMap<String, file_buffer::FileBuffer>,
}

impl BufferMap {
    fn add_buffer(
        &mut self,
        name: &String,
        buf: file_buffer::FileBuffer,
    ) -> Option<file_buffer::FileBuffer> {
        self.add_buffer_owned(name.clone(), buf)
    }

    fn add_buffer_owned(
        &mut self,
        name: String,
        buf: file_buffer::FileBuffer,
    ) -> Option<file_buffer::FileBuffer> {
        self.buffers.insert(name, buf)
    }

    fn rm_buffer(&mut self, name: &String) -> Option<file_buffer::FileBuffer> {
        self.buffers.remove(name)
    }

    fn get_buffer(&self, name: &String) -> Option<&file_buffer::FileBuffer> {
        self.buffers.get(name)
    }

    fn get_mut_buffer(&mut self, name: &String) -> Option<&mut file_buffer::FileBuffer> {
        self.buffers.get_mut(name)
    }
}
