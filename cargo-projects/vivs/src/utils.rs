// TODO: Investigate generic solution
// trait ToBeLeBytes {
//     type ByteArray: AsRef<u8>;
//     fn to_be_bytes(&self) -> Self::ByteArray;
//     fn to_le_bytes(&self) -> Self::ByteArray;
// }

pub fn u64_as_bytes(integer: u64) -> [u8; 8] {
    if cfg!(target_endian = "big") {
        integer.to_be_bytes()
    } else {
        integer.to_le_bytes()
    }
}

pub fn u8_as_bytes(integer: u8) -> [u8; 1] {
    if cfg!(target_endian = "big") {
        integer.to_be_bytes()
    } else {
        integer.to_le_bytes()
    }
}
