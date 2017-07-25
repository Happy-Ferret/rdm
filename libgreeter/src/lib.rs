extern crate log;

extern crate rdmcommon;

use std::io::{Read, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;

use rdmcommon::ipc;

#[derive(Debug)]
pub struct RdmGreeter {
    sock: UnixStream
}

impl RdmGreeter {
    pub fn new() -> Option<RdmGreeter> {
        let mut sock = match UnixStream::connect("/home/florian/tmp/sock") {
            Ok(s)   => s,
            Err(_)  => return None
        };

        ::std::thread::sleep_ms(200);

        let mut msg = Vec::with_capacity(8);
        msg.extend(b"1wCH\0\0\0\0");
        println!("Writing ClientHello: {:?}", &msg);
        match sock.write_all(&msg) {
            Ok(())  => println!("Wrote ClientHello"),
            Err(e)  => println!("Error during ServerHello: {:?}", e)
        }

        match sock.flush() {
            Ok(())  => println!("Successful flush"),
            Err(e)  => println!("Error during flush: {:?}", e)
        }

        msg.clear();
        unsafe { msg.set_len(8) };
        println!("reading...");
        sock.read_exact(&mut msg);
        println!("reading done..");
        match ipc::IpcMessage::from_bytes(&msg[2..4]) {
            Some(ipc::IpcMessage::ServerHello) => (),
            m => {
                println!("Did not get ServerHello: {:?}", m);
                return None;
            }
        }

        Some(RdmGreeter {
            sock: sock
        })
    }

    pub fn request_authentication(&mut self, user: &str, password: &str) {
        let mut msg = Vec::with_capacity(8);
        msg.extend(b"1wRA\0\0\0\0");
        println!("Writing RA: {:?}", &msg);
        match self.sock.write_all(&msg) {
            Ok(())  => println!("Wrote RA"),
            Err(e)  => println!("Error during ServerHello: {:?}", e)
        }

        match self.sock.flush() {
            Ok(())  => println!("Successful flush"),
            Err(e)  => println!("Error during flush: {:?}", e)
        }
    }
}

impl Drop for RdmGreeter {
    fn drop(&mut self) {
        println!("Dropping RdmGreeter..");
        self.sock.shutdown(Shutdown::Both).unwrap();
    }
}