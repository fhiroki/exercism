pub struct CircularBuffer<T> {
    capacity: usize,
    buffer: Vec<T>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            buffer: vec![],
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.buffer.len() >= self.capacity {
            return Err(Error::FullBuffer);
        }
        self.buffer.insert(0, _element);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.buffer.len() == 0 {
            return Err(Error::EmptyBuffer);
        }
        Ok(self.buffer.pop().unwrap())
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.buffer.len() == self.capacity {
            let _ = self.read();
        }
        let _ = self.write(_element);
    }
}
