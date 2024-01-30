pub enum Region {
    NAE,
    NAW,
    SA,
    EU,
    AS,
    AU,
    ME,
    AF,
    EVERYWHERE
}

impl Region {
    pub fn value(&self) -> u8 {
        match *self {
            Region::NAE => 0x00,
            Region::NAW => 0x01,
            Region::SA => 0x02,
            Region::EU => 0x03,
            Region::AS => 0x04,
            Region::AU => 0x05,
            Region::ME => 0x06,
            Region::AF => 0x07,
            Region::EVERYWHERE => 0xFF
        }
    }
}