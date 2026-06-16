#![no_std]

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StandardId(u16);

impl StandardId {
    pub const MAX: u16 = 0x7FF;
    
    pub const fn new(raw: u16) -> Option<Self> {
        if raw <= Self::MAX {
            Some(Self(raw))
        } else {
            None
        }
    }

    pub const fn as_raw(self) -> u16 {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ExtendedId(u32);

impl ExtendedId {
    pub const MAX: u32 = 0x1FFF_FFFF;
    
    pub const fn new(raw: u32) -> Option<Self> {
        if raw <= Self::MAX {
            Some(Self(raw))
        } else {
            None
        }
    }

    pub const fn as_raw(self) -> u32 {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CanId {
    Standard(StandardId),
    Extended(ExtendedId)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DataFrame {
    pub id: CanId,
    pub data: Data
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Data {
    bytes: [u8; 8],
    len: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RemoteFrame { 
    pub id: CanId, 
    pub dlc: u8 
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Frame {
    Data(DataFrame),
    Remote(RemoteFrame)
}

impl Frame {
    pub const fn as_data_frame(&self) -> Option<&DataFrame> {
        match self {
            Frame::Data(frame) => Some(frame),
            Frame::Remote(_) => None,
        }
    }

    pub const fn as_remote_frame(&self) -> Option<&RemoteFrame> {
        match self {
            Frame::Data(_) => None,
            Frame::Remote(frame) => Some(frame),
        }
    }
}

impl From<DataFrame> for Frame {
    fn from(frame: DataFrame) -> Self {
        Frame::Data(frame)
    }
}

impl From<RemoteFrame> for Frame {
    fn from(frame: RemoteFrame) -> Self {
        Frame::Remote(frame)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NotADataFrame(pub Frame);

impl TryFrom<Frame> for DataFrame {
    type Error = NotADataFrame;

    fn try_from(value: Frame) -> Result<Self, Self::Error> {
        match value {
            Frame::Data(frame) => Ok(frame),
            Frame::Remote(_) => Err(NotADataFrame(value))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NotARemoteFrame(pub Frame);

impl TryFrom<Frame> for RemoteFrame {
    type Error = NotARemoteFrame;

    fn try_from(value: Frame) -> Result<Self, Self::Error> {
        match value {
            Frame::Data(_) => Err(NotARemoteFrame(value)),
            Frame::Remote(frame) => Ok(frame)
        }
    }
}

mod sealed {
    pub trait Sealed {}
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
