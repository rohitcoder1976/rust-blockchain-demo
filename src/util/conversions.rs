use hex;

pub fn hex_string_to_bit_vector(hex: String) -> Vec<u8> {
    let bytes = hex::decode(hex).expect("Invalid hex string...");

    let mut bit_array: Vec<u8> = Vec::new();
    for byte in bytes {
        for i in (0..8).rev() {
            bit_array.push((byte >> i) & 1);
        }
    }

    return bit_array;
}