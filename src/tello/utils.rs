pub fn as_i16(data: &[u8]) -> i16 {
    return (data[0] as i16) & 0xff | ((data[0] >> 8) as i16) & 0xff;
}

pub fn as_u16(data: &[u8]) -> u16 {
    return (data[0] as u16) & 0xff | ((data[0] >> 8) as u16) & 0xff;
}