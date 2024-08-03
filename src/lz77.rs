//Based on: https://github.com/neoxephon/darthnemesis/blob/master/DarthNemesis/CompressionManager.cs
pub fn decompress(compressed_data: &Vec<u8>) -> Result<Vec<u8>, String> {
    if compressed_data[0] == 0x10 {
        let size = compressed_data[1] as usize + ((compressed_data[2] as usize) << 8) + ((compressed_data[3] as usize) << 16);
        let mut current_position = 4;
        let distance = 1;
        let mut uncompressed_data = Vec::new();

        while uncompressed_data.len() <= size && current_position < compressed_data.len() {
            let mut forward = 1;
            for i in 0..8 {
                if current_position + forward >= compressed_data.len() {
                    break;
                }

                if uncompressed_data.len() >= size {
                    break;
                }

                if ((compressed_data[current_position] >> (7 - i)) & 1) == 1 {
                    let amount_to_copy = 3 + ((compressed_data[current_position + forward] >> 4) & 0xF) as usize;
                    let copy_from = distance + (((compressed_data[current_position + forward] & 0xF) as u16) << 8) as usize
                        + compressed_data[current_position + forward + 1] as usize;
                    let copy_position = uncompressed_data.len().wrapping_sub(copy_from);
                    for u in 0..amount_to_copy {
                        if (copy_position + (u % copy_from)) < uncompressed_data.len() {
                            uncompressed_data.push(uncompressed_data[copy_position + (u % copy_from)]);
                        } else {
                            return Ok(uncompressed_data);
                        }
                    }

                    forward += 2;
                } else {
                    uncompressed_data.push(compressed_data[current_position + forward]);
                    forward += 1;
                }
            }

            current_position += forward;
        }

        while uncompressed_data.len() < size {
            uncompressed_data.push(0);
        }

        Ok(uncompressed_data)
    } else {
        Err("Invalid compressed data format".to_string())
    }
}