use crate::{mode::Mode, parse::{decode::{Decoder, DecoderError}, encode::EncoderError}};
use core::marker::PhantomData;

use embedded_io::{Read as SynchRead, Write as SynchWrite};
use embedded_io_async::{Read as AsynchRead, Write as AsynchWrite};

mod sealed {
    pub trait Sealed {}
}

pub trait Io: sealed::Sealed {}


pub struct Synch;
impl sealed::Sealed for Synch {}
impl Io for Synch {}

pub struct Asynch;
impl sealed::Sealed for Asynch {}
impl Io for Asynch {}

#[derive(Debug)]
pub enum InterfaceError<E> {
    Io(E),
    Encode(EncoderError),
    Decode(DecoderError),
}

pub struct Interface<T, M, IO> 
where
    M: Mode,
    IO: Io
{
    io: T,
    decoder: Decoder<M>,
    _io: PhantomData<IO>
}

impl<T: SynchRead + SynchWrite, M: Mode> Interface<T, M, Synch> {
    pub fn new_sync(io: T) -> Interface<T, M, Synch> {
        Interface { io, decoder: Decoder::new(), _io: PhantomData }
    }
}

impl<T: AsynchRead + AsynchWrite, M: Mode> Interface<T, M, Asynch> {
    pub fn new_async(io: T) -> Interface<T, M, Synch> {
        Interface { io, decoder: Decoder::new(), _io: PhantomData }
    }
}