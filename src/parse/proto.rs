pub const START: u8 = 0xAA;

pub mod fixed {
    pub const HEADER2: u8 = 0x55;
    pub const TYPE:    u8 = 0x01;
    pub const STD:     u8 = 0x01;
    pub const EXT:     u8 = 0x02;
    pub const DATA:    u8 = 0x01;
    pub const REMOTE:  u8 = 0x02;

    pub const CHECKSUM_START: usize = 2;
    pub const RESERVED_IDX: usize = 18;
    pub const CHECKSUM_IDX: usize = 19;
}

pub mod variable {
    pub const END:          u8 = 0x55;
    pub const TYPE_MARKER:  u8 = 0b1100_0000;
    pub const RTR_BIT:      u8 = 0b0001_0000;
    pub const EXT_BIT:      u8 = 0b0010_0000;
    pub const DLC_MASK:     u8 = 0b0000_1111;
}
