use std::{io::Read, net::{Ipv4Addr, SocketAddr, TcpStream}, thread::sleep, time::{self, Duration}};

use embedded_io_adapters::std::FromStd;
use usbcan::{config::Config, frame::Frame, id::{CanId, ExtendedId}, mode::Variable};
use usbcan::interface::Interface;

fn main() {
    // let addr = SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::from_octets([10, 0, 1, 5])), 2001);
    // let mut stream = TcpStream::connect(addr).expect("could not connect to stream");
    // let mut buf = [0x00; 20];

    // let mut decoder = Variable::decoder();
    // let mut idx = 0usize;
    // loop {
    //     if let Ok(size) = stream.read(&mut buf) {
    //         // println!("size = {:?}", size);
    //         for frame in decoder.decode_slice(&buf[..size]) {
    //             if let Ok(f) = frame {
    //                 println!("{:08?} {:?}", idx, f);
    //                 idx += 1;
    //             }
    //         }
    //     }
    // }

    let addr = SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::from_octets([10, 0, 1, 5])), 2001);
    let stream = TcpStream::connect(addr).expect("could not connect to stream");
    let mut interface: Interface<FromStd<TcpStream>, Variable, usbcan::interface::Synch> = Interface::new_sync(FromStd::new(stream));
    
    let my_frame = Frame::new(CanId::Extended(ExtendedId::new(0x11223345).unwrap()), &[0x11, 0x22, 0x33, 0x44]).unwrap();
    
    let config = Config {
        baud: usbcan::config::Baudrate::Baud500K,
        frame_type: usbcan::config::FrameType::Standard,
        op_mode: usbcan::config::OpMode::Normal,
        filter_id: 0,
        filter_mask: 0,
    };


    let _ = interface.configure(config);
    loop {
        if let Ok(frame) = interface.recv() {
            println!("{:?}", frame);
        }

        // match interface.try_send(&my_frame) {
        //     Ok(_) => println!("Ok"),
        //     Err(err) => println!("{:?}", err),
        // }
        // sleep(Duration::from_secs(1));
    }
}