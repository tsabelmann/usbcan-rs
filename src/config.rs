

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Baudrate {
    Baud1M = 0x01, 
    Baud800K = 0x02, 
    Baud500K = 0x03, 
    Baud400K = 0x04,
    Baud250K = 0x05, 
    Baud200K = 0x06, 
    Baud125K = 0x07, 
    Baud100K = 0x08,
    Baud50K = 0x09, 
    Baud20K = 0x0A, 
    Baud10K = 0x0B, 
    Baud5K = 0x0C
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum FrameType {
    Standard = 0x01,
    Extended = 0x02
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum OpMode { 
    Normal = 0x00, 
    Loopback = 0x01, 
    Silent = 0x02, 
    LoopbackSilent = 0x03
}

#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    pub baud: Baudrate,
    pub frame_type: FrameType,
    pub op_mode: OpMode,
    pub filter_id: u32,
    pub filter_mask: u32
}

impl Config {
    pub fn to_command(self) -> [u8; 20] {
        let mut f = [0u8; 20];
        f[0] = 0xAA;
        f[1] = 0x55;
        f[2] = 0x12;
        f[3] = self.baud as u8;
        f[4] = self.frame_type as u8;
        f[5..9].copy_from_slice(&self.filter_id.to_le_bytes());
        f[9..13].copy_from_slice(&self.filter_mask.to_le_bytes());
        f[13] = self.op_mode as u8;
        f[14] = 0x01;
        // f[15..19] reserviert = 0
        f[19] = f[2..19].iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
        f
    }
}
