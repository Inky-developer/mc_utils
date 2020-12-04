use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::io::Read;

use crate::{Error, Result};

/// Response of the minecraft server after a command was sent
#[derive(Debug)]
pub struct PacketResponse {
    /// The id of the packet. Useless right now
    pub packet_id: i32,
    /// The response message
    pub payload: String,
}

impl PacketResponse {
    pub fn from_reader(reader: &mut impl Read) -> Result<PacketResponse> {
        let length = reader.read_i32::<LittleEndian>()? as usize;

        let mut buffer = Cursor::new(vec![0; length]);
        reader.read_exact(buffer.get_mut())?;

        let id = buffer.read_i32::<LittleEndian>()?;
        let typ = buffer.read_i32::<LittleEndian>()?;

        let string_length = buffer.get_ref().len() - buffer.position() as usize - 2; // one bytes used to end the string and one byte used for padding
        let mut string_buf = vec![0; string_length];

        buffer.read_exact(&mut string_buf)?;

        let string = String::from_utf8(string_buf).map_err(|_error| {
            Error::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Could not decode string",
            ))
        })?;

        if typ == 2 && id == -1 {
            return Err(Error::LoginFailed);
        }

        Ok(PacketResponse {
            payload: string,
            packet_id: id,
        })
    }
}
