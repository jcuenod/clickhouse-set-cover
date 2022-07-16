use std::collections::VecDeque;
use std::io::{BufWriter, Read, Write};

mod clickhouse;
mod set_cover;

fn main() {
    let t_byte = (1 as u8).to_le_bytes();
    let f_byte = (0 as u8).to_le_bytes();

    let byte_vec: Result<VecDeque<u8>, _> = std::io::stdin().bytes().collect();
    let mut rowbinary_data = byte_vec.expect("Unable to read data");
    let mut d = clickhouse::deserialize(&mut rowbinary_data);

    let mut stream = BufWriter::new(std::io::stdout());
    while d.len() > 0 {
        let set = d.pop_front();
        if set_cover::set_cover_possible(&set.unwrap()) {
            stream.write(&t_byte).expect("Could not write to stream");
        } else {
            stream.write(&f_byte).expect("Could not write to stream");
        }
    }
    stream.flush().unwrap();
}
