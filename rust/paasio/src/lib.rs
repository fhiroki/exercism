use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    operations_account: usize,
    bytes_account: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped: _wrapped,
            operations_account: 0,
            bytes_account: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_account
    }

    pub fn reads(&self) -> usize {
        self.operations_account
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.operations_account += 1;
        let read = self.wrapped.read(buf)?;
        self.bytes_account += read;
        Ok(read)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    operations_account: usize,
    bytes_account: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped: _wrapped,
            operations_account: 0,
            bytes_account: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_account
    }

    pub fn writes(&self) -> usize {
        self.operations_account
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.operations_account += 1;
        let write =self.wrapped.write(buf)?;
        self.bytes_account += write;
        Ok(write)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
