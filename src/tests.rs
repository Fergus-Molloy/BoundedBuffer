#[cfg(test)]
use crate::BoundedBuffer;
#[test]
fn pop_ok() {
    let mut buf: BoundedBuffer<i32> = BoundedBuffer::new(1);
    buf.push(1).unwrap();
    let value: i32 = 1;
    assert_eq!(value, buf.pop().expect("test"));
}

#[test]
fn pop_none() {
    let mut buf: BoundedBuffer<i32> = BoundedBuffer::new(1);
    assert!(buf.pop().is_err());
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
    assert!(buf.pop().is_err());
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
    assert_eq!(actual, test);
}
