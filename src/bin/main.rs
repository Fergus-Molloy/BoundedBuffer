use bounded_buffer::BoundedBuffer;
fn main() {
    let mut buf = BoundedBuffer::new(3);
    buf.push(1).unwrap();
    println!("{}", buf.pop().unwrap());
}
