use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};

use crate::{Packet, PacketResponse, Result};

/// A blocking mcrcon implementation
///
/// The Rcon automatically connects on initialization and
/// automatically disconnects when beeing dropped
#[derive(Debug)]
pub struct McRcon {
    stream: TcpStream,
}

impl McRcon {
    pub fn new(address: impl ToSocketAddrs, password: String) -> Result<Self> {
        let stream = TcpStream::connect(address)?;

        let mut mcrcon = McRcon { stream };
        mcrcon.send(Packet {
            id: 0,
            payload: password.into(),
            typ: crate::PacketType::Login,
        })?;

        Ok(mcrcon)
    }

    pub fn command(&mut self, command: impl Into<Vec<u8>>) -> Result<PacketResponse> {
        self.send(Packet {
            id: 0,
            payload: command.into(),
            typ: crate::PacketType::Command,
        })
    }

    pub fn disconnect(&mut self) -> Result<()> {
        Ok(self.stream.shutdown(std::net::Shutdown::Both)?)
    }

    fn send(&mut self, data: Packet) -> Result<PacketResponse> {
        let deserialized: Vec<u8> = data.into();
        self.stream.write_all(&deserialized)?;

        PacketResponse::from_reader(&mut self.stream)
    }
}

impl Drop for McRcon {
    fn drop(&mut self) {
        let _ = self.disconnect();
    }
}
