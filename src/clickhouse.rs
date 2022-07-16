use std::collections::{HashSet, VecDeque};

fn get_unsigned_leb128(buffer: &mut VecDeque<u8>) -> u64 {
    let mut value = 0u64;
    let mut shift = 0;

    while buffer.len() > 0 {
        let byte = buffer.pop_front().expect("Couldn't get byte for leb128");

        value |= (byte as u64 & 0x7f) << shift;

        if byte & 0x80 == 0 {
            break;
        }

        shift += 7;
        if shift > 57 {
            panic!("Not enough data");
        }
    }
    value
}

fn read_size(buffer: &mut VecDeque<u8>) -> usize {
    let size = get_unsigned_leb128(buffer);
    let usize_from = usize::try_from(size);
    match usize_from {
        Ok(result) => return result,
        Err(_) => panic!("Couldn't convert leb128 value to length. Not enough data?"),
    }
}

pub fn deserialize(rowbin: &mut VecDeque<u8>) -> VecDeque<Vec<HashSet<u32>>> {
    let mut deserialized_data: Vec<Vec<HashSet<u32>>> = Vec::new();
    while rowbin.len() > 0 {
        // In CH rowbinary, rows are printed out sequentially
        let mut row: Vec<HashSet<u32>> = Vec::new();
        let outer_array_size = read_size(rowbin);
        for _ in 0..outer_array_size {
            let mut row_cell: HashSet<u32> = HashSet::new();
            let inner_array_size = read_size(rowbin);
            for _ in 0..inner_array_size {
                let u32bytes = [
                    rowbin.pop_front().expect("Could not read first u32 byte"),
                    rowbin.pop_front().expect("Could not read second u32 byte"),
                    rowbin.pop_front().expect("Could not read third u32 byte"),
                    rowbin.pop_front().expect("Could not read fourth u32 byte"),
                ];
                row_cell.insert(u32::from_le_bytes(u32bytes));
            }
            row.push(row_cell);
        }
        deserialized_data.push(row);
    }
    VecDeque::from(deserialized_data)
}
