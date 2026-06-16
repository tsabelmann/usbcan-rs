use crate::id::CanId;
use core::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DataFrame {
    pub id: CanId,
    pub data: Data
}

impl Index<usize> for DataFrame {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for DataFrame {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Data {
    bytes: [u8; 8],
    len: u8,
}

impl Index<usize> for Data {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        let slice = &self.bytes[..self.len as usize];
        &slice[index]
    }
}

impl IndexMut<usize> for Data {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let slice = &mut self.bytes[..self.len as usize];
        &mut slice[index]
    }
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
    pub fn is_standard(&self) -> bool {
        match self {
            Frame::Data(frame) => frame.id.is_standard(),
            Frame::Remote(frame) => frame.id.is_standard(),
        }
    }

    pub fn is_extended(&self) -> bool {
        !self.is_standard()
    }

    pub fn is_data_frame(&self) -> bool {
        matches!(self, Frame::Data(_))
    }

    pub fn is_remote_frame(&self) -> bool {
        !self.is_data_frame()
    }

    pub fn id(&self) -> CanId {
        match self {
            Frame::Data(frame) => frame.id,
            Frame::Remote(frame) => frame.id,
        }
    }

    pub fn dlc(&self) -> usize {
        match self {
            Frame::Data(frame) => frame.data.len as usize,
            Frame::Remote(frame) => frame.dlc as usize,
        }
    }

    pub fn data(&self) -> &[u8] {
        match self {
            Frame::Data(frame) => &frame.data.bytes[..frame.data.len as usize],
            Frame::Remote(_) => &[],
        }
    }

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

impl core::fmt::Display for NotADataFrame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "expected a data frame, got a remote frame")
    }
}

impl core::error::Error for NotADataFrame {}

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

impl core::fmt::Display for NotARemoteFrame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "expected a remote frame, got a data frame")
    }
}

impl core::error::Error for NotARemoteFrame {}

impl TryFrom<Frame> for RemoteFrame {
    type Error = NotARemoteFrame;

    fn try_from(value: Frame) -> Result<Self, Self::Error> {
        match value {
            Frame::Data(_) => Err(NotARemoteFrame(value)),
            Frame::Remote(frame) => Ok(frame)
        }
    }
}
