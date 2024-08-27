use std::io::{Read, Result, Write};

pub struct IoStats<T> {
    bytes_through: usize,
    inner: T,
    reads: usize,
    writes: usize,
}

pub type ReadStats<R> = IoStats<R>;
pub type WriteStats<W> = IoStats<W>;

impl<T> IoStats<T> {
    pub fn new(wrapped: T) -> IoStats<T> {
        ReadStats {
            bytes_through: 0,
            inner: wrapped,
            reads: 0,
            writes: 0,
        }
    }

    /// Get a reference to the inner reader.
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Get the number of bytes that have been read from the wrapped reader.
    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }
}

impl<R: Read> IoStats<R> {
    /// Get the number of reads performed on the wrapped reader.
    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes_read = self.inner.read(buf)?;
        self.bytes_through += bytes_read;
        self.reads += 1;
        Ok(bytes_read)
    }
}

impl<W: Write> IoStats<W> {
    /// Get the number of writes performed on the wrapped writer.
    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_written = self.inner.write(buf)?;
        self.bytes_through += bytes_written;
        self.writes += 1;
        Ok(bytes_written)
    }

    /// If writing to a BufWriter, it will effectively write to the buffer only when it is full or when explicity flushed.
    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}
