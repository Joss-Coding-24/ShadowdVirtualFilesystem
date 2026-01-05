fn to_big_endian<T>(value: T, bytes: usize) -> Vec<u8>
where
    T: Copy
        + Into<u128>
        + From<u8>,
{
    let mut out = vec![0u8; bytes];
    let mut v: u128 = value.into();

    for i in 0..bytes {
        out[bytes - 1 - i] = (v & 0xFF) as u8;
        v >>= 8;
    }
    out
}
fn from_big_endian<T>(data: &[u8], bytes: usize) -> T
where
    T: From<u8> + std::ops::Shl<u32, Output = T> + std::ops::BitOr<Output = T>,
{
    let mut value = T::from(0u8);

    for i in 0..bytes {
        value = (value << 8) | T::from(data[i]);
    }
    value
}
pub fn int_to_be(v: i32, b: usize) -> Vec<u8> {
    to_big_endian(v as u128, b)
}

pub fn be_to_int(v: &[u8], b: usize) -> i32 {
    from_big_endian::<i32>(v, b)
}

pub fn size_to_be(v: usize, b: usize) -> Vec<u8> {
    to_big_endian(v as u128, b)
}

pub fn be_to_size(v: &[u8], b: usize) -> usize {
    from_big_endian::<usize>(v, b)
}

pub fn u32_to_be(v: u32, b: usize) -> Vec<u8> {
    to_big_endian(v as u128, b)
}

pub fn be_to_u32(v: &[u8], b: usize) -> u32 {
    from_big_endian::<u32>(v, b)
}

pub fn u64_to_be(v: u64, b: usize) -> Vec<u8> {
    to_big_endian(v as u128, b)
}

pub fn be_to_u64(v: &[u8], b: usize) -> u64 {
    from_big_endian::<u64>(v, b)
}