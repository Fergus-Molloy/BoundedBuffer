//! A circular queue within a fixed size buffer
//!
//! A `BoundedBuffer<T>` is initialised with a fixed size. Users can then push and pop items in and
//! out of the buffer in a FIFO circular queue
/// Implements a circular queue in a fixed size vector
pub struct BoundedBuffer<T>
where
    T: Copy,
{
    list: Vec<Option<T>>,
    size: usize,
    front: usize,
    back: usize,
}

impl<T> BoundedBuffer<T>
where
    T: Copy,
{
    /// Create a new instance of a BoundedBuffer
    /// # Arguments
    ///
    /// * `size` - The size of the buffer to be created
    ///
    /// # Example
    ///
    /// ```
    /// let mut buf: BoundedBuffer<i32> = BoundedBuffer::new(10);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if given a size of 0
    pub fn new(size: usize) -> BoundedBuffer<T> {
        if size == 0 {
            panic!("cannot create a BoundedBuffer of size 0");
        }
        let mut list = Vec::with_capacity(size);
        let front = 0;
        let back = 0;
        BoundedBuffer {
            list,
            size,
            front,
            back,
        }
    }

    /// Push a new item into the buffer
    ///
    /// Push returns a result because if the buffer is full the operation will fail
    ///
    /// # Arguments
    ///
    /// * `item` - The item to be added to the buffer
    ///
    /// # Example
    ///
    /// ```
    /// let mut buf = BoundedBuffer::new(10);
    /// buf.push("hello").unwrap();
    /// ```
    ///
    pub fn push(&mut self, item: T) -> Result<T, &'static str> {
        if (self.list.len() > 0) && ((self.front == self.back) && (self.list[self.front].is_some()))
        {
            Err("list is full")
        } else if self.list.len() < self.size {
            self.list.push(Some(item));
            self.back = (self.back + 1) % self.size;
            Ok(item)
        } else {
            self.list[self.back] = Some(item);
            self.back = (self.back + 1) % self.size;
            Ok(item)
        }
    }
    /// Pop the oldest item out of the buffer
    /// Returns an option as buffer could be empty
    ///
    /// # Example
    ///
    /// ```
    /// let mut buf = BoundedBuffer::new(10);
    /// buf.push("Hello");
    /// buf.push("World");
    /// println!("{}",buf.pop().unwrap());
    /// ```
    ///
    pub fn pop(&mut self) -> Option<T> {
        if self.list.len() == 0 {
            None
        } else {
            let item = self.list[self.front];
            self.list[self.front] = None;
            self.front = (self.front + 1) % self.size;
            item
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pop_some() {
        let mut buf = BoundedBuffer::new(1);
        buf.push(1).unwrap();
        let value: i32 = 1;
        assert_eq!(value, buf.pop().unwrap());
    }

    #[test]
    fn pop_none() {
        let mut buf: BoundedBuffer<i32> = BoundedBuffer::new(1);
        assert!(buf.pop().is_none());
    }

    #[test]
    fn push_ok() {
        let mut buf = BoundedBuffer::new(1);
        assert!(buf.push(2).is_ok());
    }

    #[test]
    fn push_err() {
        let mut buf = BoundedBuffer::new(1);
        buf.push(2).unwrap();
        assert!(buf.push(3).is_err());
    }

    #[test]
    #[should_panic]
    fn init_zero() {
        let _buf: BoundedBuffer<i32> = BoundedBuffer::new(0);
    }

    #[test]
    fn overwrite() {
        let mut buf = BoundedBuffer::new(1);
        buf.push(1).unwrap();
        assert!(buf.push(2).is_err())
    }

    #[test]
    fn empty() {
        let mut buf: BoundedBuffer<i32> = BoundedBuffer::new(1);
        assert_eq!(None, buf.pop());
    }

    #[test]
    fn circular() {
        let mut buf = BoundedBuffer::new(3);
        buf.push("hello").unwrap();
        buf.push("to the").unwrap();
        buf.push("world").unwrap();
        buf.pop().unwrap();
        buf.push("again").unwrap();
        let actual = vec!["to the", "world", "again"];
        let mut test = Vec::new();
        for _ in 0..3 {
            test.push(buf.pop().unwrap());
        }
        println!("actaul:",);
        for x in &actual {
            println!("{}", x);
        }
        println!("\nresult:",);
        for x in &test {
            println!("{}", x);
        }
        assert_eq!(actual, test);
    }
}
