use std::fmt::Debug;

pub struct CircularBuffer<T: Clone + Debug> {
    buffer: Vec<Option<T>>,
    first: usize,
    last: usize,
    is_empty: bool,
    is_full: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone + Debug> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        (0..capacity).for_each(|_| { buffer.push(None) });
        CircularBuffer {
            buffer,
            first: 0,
            last: 0,
            is_empty: true,
            is_full: false,
        }
    }

    pub fn write(&mut self, value: T) -> Result<(), Error> {
        if !self.is_empty {
            self.last = self.advance(self.last);
            if self.last == self.first {
                self.is_full = true;
                self.first = self.advance(self.first)
            }
        }

        let result = match &self.buffer.get(self.last) {
            Some(Some(_)) => { Err(Error::FullBuffer) },
            _ => {
                self.buffer[self.last] = Some(value);
                self.is_empty = false;
                self.is_full = self.advance(self.last) == self.first;
                Ok(())
            }
        };
        result
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty { return Err(Error::EmptyBuffer) }
        let result = match &self.buffer[self.first] {
            Some(val) => {
                let result = val.clone();
                self.buffer[self.first] = None;
                self.first = self.advance(self.first);
                self.is_full = false;
                Ok(result)
            },
            None => {
                Err(Error::EmptyBuffer)
            }
        };
        result
    }

    pub fn clear(&mut self) {
        self.is_empty = true;
        self.is_full = false;
        self.buffer.fill(None);
    }

    pub fn overwrite(&mut self, val: T) {
        if !self.is_full {
            self.write(val);
            return
        }
        self.buffer[self.first] = Some(val);
        self.first = self.advance(self.first);
        self.last = self.advance(self.last);
    }

    fn advance(&mut self, i: usize) -> usize {
        if i + 1 >= self.buffer.capacity() { 0 } else { i + 1 }
    }
}
