use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    bytes_read: usize,
    inner: R,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            bytes_read: 0,
            inner: wrapped,
            reads: 0,
        }
    }

    /// Get a reference to the inner reader.
    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    /// Get the number of bytes that have been read from the wrapped reader.
    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    /// Get the number of reads performed on the wrapped reader.
    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes_read = self.inner.read(buf)?;
        self.bytes_read += bytes_read;
        self.reads += 1;
        Ok(bytes_read)
    }
}

pub struct WriteStats<W> {
    bytes_written: usize,
    inner: W,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            bytes_written: 0,
            inner: wrapped,
            writes: 0,
        }
    }

    /// Get a reference to the inner writer.
    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    /// Get the number of bytes that have been written to the wrapped writer.
    pub fn bytes_through(&self) -> usize {
        self.bytes_written
    }

    /// Get the number of writes performed on the wrapped writer.
    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_written = self.inner.write(buf)?;
        self.bytes_written += bytes_written;
        self.writes += 1;
        Ok(bytes_written)
    }

    /// If writing to a BufWriter, it will effectively write to the buffer only when it is full or when explicity flushed.
    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}
