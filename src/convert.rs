pub fn u8_to_bytes(data: u8) -> [u8; 1] {
    [data]
}

pub fn u16_to_bytes(data: u16) -> [u8; 2] {
    [data as u8, (data >> 8) as u8]
}

pub fn u32_to_bytes(data: u32) -> [u8; 4] {
    [
        data as u8,
        (data >> 8) as u8,
        (data >> 16) as u8,
        (data >> 24) as u8,
    ]
}

pub fn bytes_to_u8(data: &[u8]) -> u8 {
    data[0]
}

pub fn bytes_to_u16(data: &[u8]) -> u16 {
    data[0] as u16 + ((data[1] as u16) << 8)
}

pub fn bytes_to_u32(data: &[u8]) -> u32 {
    data[0] as u32 + ((data[1] as u32) << 8) + ((data[2] as u32) << 16) + ((data[3] as u32) << 24)
}

pub fn distance_u32(a: u32, b: u32) -> u32 {
    if a >= b {
        a - b
    } else {
        b - a
    }
}
