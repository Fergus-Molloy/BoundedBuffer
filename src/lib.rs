//! A circular queue within a fixed size buffer
//!
//! A `BoundedBuffer` is initialised with a fixed size. Users can then push and pop items in and
//! out of the buffer in a FIFO circular queue
mod tests;

/// Implements a circular queue as a fixed size vector
pub struct BoundedBuffer<T>
where
    T: Clone,
{
    //needs to be an option because the list can be `empty` but len != 0
    list: Vec<Option<T>>,
    size: usize,
    front: usize,
    back: usize,
}

impl<T> BoundedBuffer<T>
where
    T: Clone, //needs to be clone since remove doesn't work for some reason
{
    /// Create a new instance of a BoundedBuffer
    ///
    /// # Panics
    ///
    /// If `size` is less than or equal to 0
    ///
    /// # Exmples
    /// ```
    ///  # use bounded_buffer::BoundedBuffer;
    /// let mut buffer = BoundedBuffer::new(10);
    /// # buffer.push("hello world"); // allows rust to infer the type
    ///
    /// // with an explicit type annotation
    /// let buffer: BoundedBuffer<i32> = BoundedBuffer::new(10);
    /// ```
    pub fn new(size: usize) -> BoundedBuffer<T> {
        if size <= 0 {
            panic!("Size of buffer cannot be initialised to <= 0");
        } else {
            let list = Vec::with_capacity(size);
            BoundedBuffer {
                list,
                size,
                front: 0,
                back: 0,
            }
        }
    }

    /// Push a new item into the buffer
    ///
    /// # Errors
    ///
    /// If the buffer is full this function will return an error
    /// otherwise it will return `()`
    ///
    /// # Example
    ///
    /// ```
    ///  # use bounded_buffer::BoundedBuffer;
    /// let mut buf = BoundedBuffer::new(10);
    /// assert!(buf.push("hello").is_ok());
    ///
    /// // returns an error
    /// let mut buf = BoundedBuffer::new(1);
    /// buf.push("hello");
    /// assert!(buf.push("world").is_err());
    /// ```
    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if (self.list.len() > 0) && ((self.front == self.back) && (self.list[self.front].is_some()))
        {
            Err("list is full")
        } else if self.list.len() < self.size {
            self.list.push(Some(item));
            self.back = (self.back + 1) % self.size;
            Ok(())
        } else {
            self.list[self.back] = Some(item);
            self.back = (self.back + 1) % self.size;
            Ok(())
        }
    }

    /// Pop the oldest item in the buffer
    ///
    /// # Errors
    ///
    /// Returns an error if buffer is empty
    ///
    /// # Example
    ///
    /// ```
    ///  # use bounded_buffer::BoundedBuffer;
    /// let mut buf = BoundedBuffer::new(10);
    /// buf.push("Hello");
    /// buf.push("World");
    /// assert_eq!("Hello",buf.pop().unwrap());
    ///
    /// // returns an error
    /// let mut buf: BoundedBuffer<i32> = BoundedBuffer::new(1);
    /// assert!(buf.pop().is_err());
    /// ```
    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.list.len() == 0 {
            Err("list is empty")
        } else {
            // want to use list.remove(self.front) but it panics (i think due to the shift left)
            // let item = self.list.remove(self.front);
            let item = self.list.get(self.front).unwrap().clone();
            self.list[self.front] = None;
            self.front = (self.front + 1) % self.size;
            match item {
                Some(v) => Ok(v),
                None => Err("List is empty"),
            }
        }
    }
}
