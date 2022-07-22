use std::collections::HashSet;
use std::io::{BufReader, Error, ErrorKind, Read, Stdin};

fn get_unsigned_leb128(buffer: &mut BufReader<Stdin>) -> Result<u64, Error> {
    let mut value = 0u64;
    let mut shift = 0;

    loop {
        let mut byte: [u8; 1] = [0; 1];
        buffer.read_exact(&mut byte)?;

        value |= (byte[0] as u64 & 0x7f) << shift;

        if byte[0] & 0x80 == 0 {
            break;
        }

        shift += 7;
        if shift > 57 {
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }
    Ok(value)
}

fn read_size(buffer: &mut BufReader<Stdin>) -> Result<usize, Error> {
    let size = get_unsigned_leb128(buffer)?;
    let usize_from = usize::try_from(size);
    match usize_from {
        Ok(result) => return Ok(result),
        Err(_) => Err(Error::from(ErrorKind::Other)),
    }
}

pub fn try_deserialize_row(buffer: &mut BufReader<Stdin>) -> Result<Vec<HashSet<u32>>, Error> {
    let mut row: Vec<HashSet<u32>> = Vec::new();
    let outer_array_size = read_size(buffer)?;
    for _ in 0..outer_array_size {
        let mut row_cell: HashSet<u32> = HashSet::new();
        let inner_array_size = read_size(buffer)?;
        for _ in 0..inner_array_size {
            let mut u32_bytes: [u8; 4] = [0; 4];
            buffer
                .read_exact(&mut u32_bytes)
                .expect("Could not read 4 bytes for u32 int");
            row_cell.insert(u32::from_le_bytes(u32_bytes));
        }
        row.push(row_cell);
    }
    Ok(row)
}
