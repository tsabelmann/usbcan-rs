use crate::mode::Mode;

pub trait Parser<T: Mode> {
    fn parser(&mut self, bytes: &[u8]);
}

