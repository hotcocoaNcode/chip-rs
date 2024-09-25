pub fn get_low_nyb(byte: u8) -> u8 {
    byte & 0x0F
}

pub fn get_high_nyb(byte: u8) -> u8 {
    (byte & 0xF0) >> 4
}

pub fn get_3nyb_addr(first: u8, second: u8) -> u16 {
    ((first as u16 & 0x0F) << 8) | second as u16
}