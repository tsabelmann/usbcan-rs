use crate::{frame::Frame, mode::{Mode, Fixed, Variable}};
use core::marker::PhantomData;

pub mod decode;
pub mod encode;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FrameSizeIndicator<M: Mode> {
    _mode: PhantomData<M>
}

impl FrameSizeIndicator<Fixed> {
    pub fn size(_frame: &Frame) -> usize {
        20
    }
}

impl FrameSizeIndicator<Variable> {
    pub fn size(frame: &Frame) -> usize {
        let mut length = 3; // two marker bytes + type byte
        length += if frame.is_standard() {
            2 // two bytes for standard ID
        } else {
            4 // four bytes for extended ID
        };

        length += frame.dlc(); // up to 8 bytes for the downloadable content
        length
    }
}
