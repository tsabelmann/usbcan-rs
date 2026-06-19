use crate::parse::{encode::Encoder, decode::Decoder};

mod sealed {
    pub trait Sealed {}
}

pub trait Mode: sealed::Sealed {}

pub struct Fixed;
impl sealed::Sealed for Fixed {}
impl Mode for Fixed {}

impl Fixed {
    pub const fn encoder() -> Encoder<Self> {
        Encoder::<Self>::new()
    }

    pub const fn decoder() -> Decoder<Self> {
        Decoder::<Self>::new()
    }
}

pub struct Variable;
impl sealed::Sealed for Variable {}
impl Mode for Variable {}

impl Variable {
    pub const fn encoder() -> Encoder<Self> {
        Encoder::<Self>::new()
    }

    pub const fn decoder() -> Decoder<Self> {
        Decoder::<Self>::new()
    }
}
