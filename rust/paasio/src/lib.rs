use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    reads: usize,
    bytes_read: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,
            reads: 0,
            bytes_read: 0,
        }
    }

    pub fn get_ref(&self) -> &R { &self.wrapped }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize { self.reads }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes_read = self.wrapped.read(buf)?;
        self.bytes_read += bytes_read;
        self.reads += 1;
        Ok(bytes_read)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    writes: usize,
    bytes_written: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,
            writes: 0,
            bytes_written: 0,
        }
    }

    pub fn get_ref(&self) -> &W { &self.wrapped }

    pub fn bytes_through(&self) -> usize { self.bytes_written }

    pub fn writes(&self) -> usize { self.writes }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_written = self.wrapped.write(buf)?;
        self.writes += 1;
        self.bytes_written += bytes_written;
        Ok(bytes_written)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
