#[inline(always)]
pub fn read_u16_le(bytes: &[u8], offset: usize) -> u16 {
    let src = &bytes[offset..offset + 2];
    u16::from_le_bytes([src[0], src[1]])
}

#[inline(always)]
pub fn read_i32_le(bytes: &[u8], offset: usize) -> i32 {
    let src = &bytes[offset..offset + 4];
    i32::from_le_bytes([src[0], src[1], src[2], src[3]])
}

#[inline(always)]
pub fn read_i64_le(bytes: &[u8], offset: usize) -> i64 {
    let src = &bytes[offset..offset + 8];
    i64::from_le_bytes([
        src[0], src[1], src[2], src[3],
        src[4], src[5], src[6], src[7],
    ])
}

#[inline(always)]
pub fn read_u64_le(bytes: &[u8], offset: usize) -> u64 {
    let src = &bytes[offset..offset + 8];
    u64::from_le_bytes([
        src[0], src[1], src[2], src[3],
        src[4], src[5], src[6], src[7],
    ])
}

#[inline(always)]
pub fn write_u16_le(bytes: &mut [u8], offset: usize, value: u16) {
    let dst = &mut bytes[offset..offset + 2];
    dst.copy_from_slice(&value.to_le_bytes());
}

#[inline(always)]
pub fn write_i64_le(bytes: &mut [u8], offset: usize, value: i64) {
    let dst = &mut bytes[offset..offset + 8];
    dst.copy_from_slice(&value.to_le_bytes());
}

#[inline(always)]
pub fn write_u64_le(bytes: &mut [u8], offset: usize, value: u64) {
    let dst = &mut bytes[offset..offset + 8];
    dst.copy_from_slice(&value.to_le_bytes());
}