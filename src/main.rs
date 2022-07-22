// use std::collections::VecDeque;
use std::io::{BufReader, BufWriter, ErrorKind, Write};

mod clickhouse;
mod set_cover;

fn main() {
    let t_byte = (1 as u8).to_le_bytes();
    let f_byte = (0 as u8).to_le_bytes();
    let mut stream = BufWriter::new(std::io::stdout());
    let mut buffer = BufReader::new(std::io::stdin());

    loop {
        let attempt = clickhouse::try_deserialize_row(&mut buffer);
        match attempt {
            Err(err) => match err.kind() {
                ErrorKind::UnexpectedEof => break,
                _ => panic!("An error occurred"),
            },
            Ok(vec_of_hashsets) => {
                if set_cover::set_cover_possible(&vec_of_hashsets) {
                    stream.write(&t_byte).expect("Could not write to stream");
                } else {
                    stream.write(&f_byte).expect("Could not write to stream");
                }
                stream.flush().unwrap();
            }
        }
    }
}
