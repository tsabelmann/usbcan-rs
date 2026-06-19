use crate::{frame::Frame, mode::Mode, parse::{decode::{Decoder, DecoderError, PushDecode}, encode::{Encode, Encoder, EncoderError}}};
use core::marker::PhantomData;
use std::hint::black_box;

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
    encoder: Encoder<M>,
    recv_buf: [u8; 20],
    send_buf: [u8; 20],
    _io: PhantomData<IO>
}

impl<T: SynchRead + SynchWrite, M: Mode> Interface<T, M, Synch> {
    pub fn new_sync(io: T) -> Interface<T, M, Synch> {
        Interface {
            io, 
            decoder: Decoder::new(), 
            encoder: Encoder::new(),
            recv_buf: [0u8; 20],
            send_buf: [0u8; 20],
            _io: PhantomData
        }
    }


    pub fn try_send(&mut self, frame: &Frame) -> Result<(), InterfaceError<T::Error>> 
    where 
        Encoder<M>: Encode
    {
        let n = self.encoder.encode(frame, &mut self.send_buf).map_err(InterfaceError::Encode)?;
        println!("buf = {:#02X?}", &self.send_buf[..n]);
        println!("n = {:?}", n);
        let n = self.io.write(&self.send_buf[..n]).map_err(InterfaceError::Io);
        println!("send = {:?}", n);
        Ok(())
    }

    pub fn recv(&mut self) -> Result<Frame, InterfaceError<T::Error>>
    where 
        Decoder<M>: PushDecode
    {
        loop {
            match self.try_recv() {
                Ok(None) => continue,
                Ok(Some(frame)) => return Ok(frame),
                Err(err) => return Err(err)
            }
        }
    }

    pub fn try_recv(&mut self) -> Result<Option<Frame>, InterfaceError<T::Error>>
    where 
        Decoder<M>: PushDecode
    {
        let n = self.io.read(&mut self.recv_buf).map_err(InterfaceError::Io)?;
        let mut result = None;
        for iter in self.decoder.decode_slice(&self.recv_buf[..n]){
            let tmp = match iter {
                Ok(frame) => Ok(frame),
                Err(err) => Err(InterfaceError::Decode(err)),
            };

            let _ = result.get_or_insert(tmp);
        };

        match result {
            Some(tmp) => tmp.map(Some),
            None => Ok(None),
        }
    }

}

impl<T: AsynchRead + AsynchWrite, M: Mode> Interface<T, M, Asynch> {
    pub fn new_async(io: T) -> Interface<T, M, Synch> {
        Interface {
            io, 
            decoder: Decoder::new(), 
            encoder: Encoder::new(),
            recv_buf: [0u8; 20],
            send_buf: [0u8; 20],
            _io: PhantomData
        }
    }
}