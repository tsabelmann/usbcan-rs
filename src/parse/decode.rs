use crate::{frame::Frame, id::{CanId, ExtendedId, StandardId}, mode::{Fixed, Mode, Variable}, parse::proto::{START, variable}};
use core::marker::PhantomData;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DecoderError {
    Overflow,
    InvalidDlc,
    MissingEnd,
    InvalidId,
    InvalidFrame
}

mod sealed {
    use super::*;
    pub trait PushDecode {
        fn push(&mut self, byte: u8) -> Result<Option<Frame>, DecoderError>;
    }
}

pub struct Decoder<M: Mode> {
    buf: [u8; 20],
    len: usize,
    _mode: PhantomData<M>
}

impl Decoder<Fixed> {
    pub(crate) const fn new() -> Decoder<Fixed> {
        Decoder { buf: [0x00; 20], len: 0, _mode: PhantomData }
    }

    #[allow(unused)]
    pub fn push(&mut self, byte: u8) -> Result<Option<Frame>, DecoderError> {
        Ok(None)
    }
}

impl sealed::PushDecode for Decoder<Fixed> {
    fn push(&mut self, byte: u8) -> Result<Option<Frame>, DecoderError> {
        Decoder::<Fixed>::push(self, byte)
    }
}

impl Decoder<Variable> {
    pub(crate) const fn new() -> Decoder<Variable> {
        Decoder { buf: [0x00; 20], len: 0, _mode: PhantomData }
    }
    
    pub fn push(&mut self, byte: u8) -> Result<Option<Frame>, DecoderError> {
        // resync 
        if self.len == 0 {
            if byte != START {
                return Ok(None);
            }
            self.buf[0] = START;
            self.len = 1;
            return Ok(None);
        }

        // overflow protection
        if self.len >= self.buf.len() {
            self.reset();
            return Err(DecoderError::Overflow);
        }

        // push data
        self.buf[self.len] = byte;
        self.len += 1;

        // type byte 
        if self.len == 2 {
            let type_byte = self.buf[1];
            if type_byte & variable::TYPE_MARKER != variable::TYPE_MARKER {
                self.reset();
                if type_byte == START {
                    self.buf[0] = START;
                    self.len = 1;
                }
                return Ok(None);
            }

            if (type_byte & variable::DLC_MASK) as usize > 8 {
                self.reset();
                return Err(DecoderError::InvalidDlc);
            }
        }
        
        // expected
        let Some(expected) = self.expected_len() else {
            return Ok(None);
        };

        // not finished yet
        if self.len < expected {
            return Ok(None)
        }

        // check end marker
        if self.buf[expected - 1] != variable::END {
            self.reset();
            return Err(DecoderError::MissingEnd);
        }

        // construct frame
        let frame = self.build_frame()?;
        self.reset();
        Ok(Some(frame))
    }

    pub fn reset(&mut self) {
        self.len = 0;
    }

    fn expected_len(&self) -> Option<usize> {
        if self.len < 2 {
            return None;
        } else {
            let type_byte = self.buf[1];
            
            // id length
            let id_len = if type_byte & variable::EXT_BIT != 0 {
                4
            } else {
                2
            };

            // dlc length
            let dlc = if type_byte & variable::RTR_BIT == 0 {
                (type_byte & variable::DLC_MASK) as usize
            } else {
                0
            };

            Some(1 + 1 + id_len + dlc + 1)
        }
    }

    fn build_frame(&self) -> Result<Frame, DecoderError> {
        let type_byte = self.buf[1];
        let extended = type_byte & variable::EXT_BIT  != 0;
        let remote   = type_byte & variable::RTR_BIT  != 0;
        let dlc      = (type_byte & variable::DLC_MASK) as usize;

        // 4 bytes extended or 2 bytes standard id
        let id = if extended {
            let raw = u32::from_le_bytes([
                self.buf[2], self.buf[3], self.buf[4], self.buf[5],
            ]);
            CanId::Extended(ExtendedId::new(raw).ok_or(DecoderError::InvalidId)?)
        } else {
            let raw = u16::from_le_bytes([self.buf[2], self.buf[3]]);
            CanId::Standard(StandardId::new(raw).ok_or(DecoderError::InvalidId)?)
        };

        if remote {
            // remote frame is without data
            return Frame::new_remote(id, dlc as u8).ok_or(DecoderError::InvalidFrame);
        }

        let data_start = 2 + if extended { 4 } else { 2 };
        let data = &self.buf[data_start..data_start + dlc];
        Frame::new(id, data).ok_or(DecoderError::InvalidFrame)
    }
}

impl sealed::PushDecode for Decoder<Variable> {
    fn push(&mut self, byte: u8) -> Result<Option<Frame>, DecoderError> {
        Decoder::<Variable>::push(self, byte)
    }
}

pub struct Frames<'a, M: Mode>
where 
    Decoder<M>: sealed::PushDecode
{
    decoder: &'a mut Decoder<M>,
    input: &'a [u8],
    pos: usize,
}

impl<M: Mode> Iterator for Frames<'_, M> 
where 
    Decoder<M>: sealed::PushDecode    
{
    type Item = Result<Frame, DecoderError>;
    fn next(&mut self) -> Option<Self::Item> {
        use sealed::PushDecode;
        
        while let Some(&byte) = self.input.get(self.pos) {
            self.pos += 1;
            match self.decoder.push(byte) {
                Ok(None)     => continue,
                Ok(Some(f))  => return Some(Ok(f)),
                Err(e)       => return Some(Err(e)),
            }
        }
        None
    }
}

impl<M: Mode> Decoder<M>
where 
    Decoder<M>: sealed::PushDecode
{
    pub fn decode_slice<'a>(&'a mut self, input: &'a [u8]) -> Frames<'a, M> {
        Frames { decoder: self, input, pos: 0 }
    }
}

#[cfg(test)]
mod decode_tests {    
    use super::*;

    #[test]
    fn decode_frame_001() {
        let mut decoder = Variable::decoder();
        let buf = [
            0xAA,
            0xE2,
            0x21,
            0x30,
            0x03,
            0x01,
            0x11,
            0x22,
            0x55
        ];

        for frame in decoder.decode_slice(&buf) {
            match frame {
                Ok(lhs)  => {
                    let rhs = Frame::new(CanId::Extended(ExtendedId::new(0x1033021).unwrap()), &[0x11, 0x22]).unwrap();
                    assert_eq!(lhs, rhs)
                },
                Err(_) => panic!()
            }
        }
    }
}
