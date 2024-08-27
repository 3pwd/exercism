use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R> {
    bytes_read: usize,
    inner: R,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: R) -> ReadStats<R> {
        todo!()
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
        todo!("Collect statistics about this call reading {buf:?}")
    }
}

pub struct WriteStats<W> {
    bytes_written: usize,
    inner: W,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: W) -> WriteStats<W> {
        todo!()
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
        todo!("Collect statistics about this call writing {buf:?}")
    }

    fn flush(&mut self) -> Result<()> {
        todo!()
    }
}
