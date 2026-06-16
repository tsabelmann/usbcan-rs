use crate::{frame::Frame, mode::{Mode, Variable}, parse::FrameSizeIndicator};
use core::marker::PhantomData;


pub enum EncoderResult<'a> {
    Ok(&'a mut [u8]),
    NotEnoughData { expected: usize, provided: usize }
}


pub struct Encoder<M: Mode> {
    _mode: PhantomData<M>
}

impl<M: Mode> Encoder<M> {
    const START:            u8 = 0xAA;
    const END:              u8 = 0x55;
    const TYPE_MARKER:      u8 = 0b1100_0000;
    const RTR_BIT:          u8 = 0b0001_0000;
    const EXT_BIT:          u8 = 0b0010_0000;
    const DLC_MASK:         u8 = 0b0000_1111;
}


impl Encoder<Variable> {
    pub fn encode<'a>(&mut self, frame: &Frame, buf: &'a mut [u8]) -> EncoderResult<'a> {
        let size = FrameSizeIndicator::<Variable>::size(frame);
        if buf.len() < size {
            return EncoderResult::NotEnoughData { expected: size, provided: buf.len() };
        }

        // set start
        let mut idx = 0;
        buf[idx] = Self::START;
        idx += 1;

        // set/unset RTR bit
        let mut marker = if frame.is_remote_frame() {
            Self::TYPE_MARKER | Self::RTR_BIT
        } else {
            Self::TYPE_MARKER
        };

        // set/unset EXT bit
        marker |= if frame.is_extended() {
            Self::EXT_BIT
        } else {
            0
        };

        // set dlc
        marker |= (frame.dlc() as u8) & Self::DLC_MASK;
        buf[idx] = marker;
        idx += 1;

        // deserialize data
        for &data in frame.data() {
            buf[idx] = data;
            idx += 1;
        }

        // set end
        buf[idx] = Self::END;
        idx += 1;

        return EncoderResult::Ok(&mut buf[..idx]);
    }   
}

#[cfg(test)]
mod encode_tests {    
    use super::*;

    #[test]
    fn decode_frame_001() {

    }
}