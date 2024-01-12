use std::fs::File;
use std::io::{self, Write};

struct MyWriter<X> {
    _temp: X,
}

impl<X: Write> MyWriter<X> {
    fn new(_temp: X) -> MyWriter<X> {
        MyWriter { _temp }
    }
}

impl<W: Write> Write for MyWriter<W> {
    fn write(&mut self, _internal_buffer: &[u8]) -> io::Result<usize> {
        let mut _dup_internal_buffer = Vec::with_capacity(_internal_buffer.len() * 2);
        for &oct in _internal_buffer {
            _dup_internal_buffer.push(oct);
            _dup_internal_buffer.push(oct);
        }
        self._temp.write_all(&_dup_internal_buffer)?;
        Ok(_internal_buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self._temp.flush()
    }
}

fn main() -> Result<(), io::Error> {
    let mut writer = MyWriter::new(File::create("a.txt")?);
    writer.write_all(b"abc")?;

    Ok(())
}
