use crate::{frame::Frame, id::CanId, mode::{Fixed, Mode, Variable}, parse::FrameSizeIndicator};
use crate::parse::proto::{START, fixed, variable};
use core::marker::PhantomData;

pub trait Encode<T> {
    fn encode(&self, frame: &T, buf: &mut [u8]) -> Result<usize, EncoderError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum EncoderError {
    BufferTooSmall { expected: usize, provided: usize }
}

pub struct Encoder<M: Mode> {
    _mode: PhantomData<M>
}

impl<M: Mode> Encoder<M> {
    pub(crate) const fn new() -> Encoder<M> {
        Encoder { _mode: PhantomData }
    }
}

impl Encode<Frame> for Encoder<Fixed> {
    fn encode(&self, frame: &Frame, buf: &mut [u8]) -> Result<usize, EncoderError> {
        let size = FrameSizeIndicator::<Fixed>::size(frame);
        if buf.len() < size {
            return Err(EncoderError::BufferTooSmall { expected: size, provided: buf.len() });
        }

        // dlc should be frame data length
        debug_assert_eq!(frame.data().len(), frame.dlc());

        // clear buffer 
        buf[..size].iter_mut().for_each(|b| *b = 0);

        // set start
        let mut idx = 0;
        buf[idx] = START;
        idx += 1;

        // set header 2
        buf[idx] = fixed::HEADER2;
        idx += 1;

        // set type
        buf[idx] = fixed::TYPE;
        idx += 1;

        // set standard/extended 
        buf[idx] = if frame.is_standard() {
            fixed::STD
        } else {
            fixed::EXT
        };
        idx += 1;

        // set data/remote frame
        buf[idx] = if frame.is_data_frame() {
            fixed::DATA
        } else {
            fixed::REMOTE
        };
        idx += 1; 

        // set id
        let le_bytes = match frame.id() {
            CanId::Standard(id) => (id.as_raw() as u32).to_le_bytes(),
            CanId::Extended(id) => id.as_raw().to_le_bytes()
        };
        for byte in le_bytes {
            buf[idx] = byte;
            idx += 1; 
        }

        // set dlc
        buf[idx] = frame.dlc() as u8;
        idx += 1;

        // write data
        for &data in frame.data() {
            buf[idx] = data;
            idx += 1;
        }

        // set reserve
        buf[fixed::RESERVED_IDX] = 0x00;
        idx = fixed::CHECKSUM_IDX;

        // set check code
        let sum = {
            let mut sum: u8 = 0;
            for &value in &buf[fixed::CHECKSUM_START..=fixed::RESERVED_IDX] {
               sum = sum.wrapping_add(value);
            }
            sum
        };
        buf[idx] = sum;
        idx += 1;

        Ok(idx)
    }   
}

impl Encode<Frame> for Encoder<Variable> {
    fn encode(&self, frame: &Frame, buf: &mut [u8]) -> Result<usize, EncoderError> {
        let size = FrameSizeIndicator::<Variable>::size(frame);
        if buf.len() < size {
            return Err(EncoderError::BufferTooSmall { expected: size, provided: buf.len() });
        }

        // set start
        let mut idx = 0;
        buf[idx] = START;
        idx += 1;

        // set/unset RTR bit
        let mut marker = if frame.is_remote_frame() {
            variable::TYPE_MARKER | variable::RTR_BIT
        } else {
            variable::TYPE_MARKER
        };

        // set/unset EXT bit
        marker |= if frame.is_extended() {
            variable::EXT_BIT
        } else {
            0
        };

        // set dlc
        marker |= (frame.dlc() as u8) & variable::DLC_MASK;
        buf[idx] = marker;
        idx += 1;

        // set id
        match frame.id() {
            CanId::Standard(id) => {
                let le_bytes = id.as_raw().to_le_bytes();
                for byte in le_bytes {
                    buf[idx] = byte;
                    idx += 1; 
                }
            },
            CanId::Extended(id) => {
                let le_bytes = id.as_raw().to_le_bytes();
                for byte in le_bytes {
                    buf[idx] = byte;
                    idx += 1; 
                }
            },
        }

        // write data
        for &data in frame.data() {
            buf[idx] = data;
            idx += 1;
        }

        // set end
        buf[idx] = variable::END;
        idx += 1;

        Ok(idx)
    }   
}
