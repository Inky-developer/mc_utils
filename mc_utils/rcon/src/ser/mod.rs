use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Write;

pub enum PacketType {
    Login = 3,
    Command = 2,
}

pub struct Packet {
    pub id: i32,
    pub typ: PacketType,
    pub payload: Vec<u8>,
}

impl Packet {}

impl From<Packet> for Vec<u8> {
    fn from(packet: Packet) -> Self {
        const LENGTH_WITHOUT_PAYLOAD: i32 = 10;

        let mut wtr = Vec::new();
        wtr.write_i32::<LittleEndian>(LENGTH_WITHOUT_PAYLOAD + packet.payload.len() as i32)
            .unwrap();
        wtr.write_i32::<LittleEndian>(packet.id).unwrap();
        wtr.write_i32::<LittleEndian>(packet.typ as i32).unwrap();
        wtr.write_all(&packet.payload).unwrap();
        wtr.write_all(&[0, 0]).unwrap();

        wtr
    }
}

#[cfg(test)]
mod test {
    use super::{Packet, PacketType};

    #[test]
    fn test_packet_to_bytes() {
        let packet = Packet {
            id: 0,
            typ: PacketType::Login,
            payload: b"1234".to_vec(),
        };

        let result: Vec<u8> = packet.into();

        assert_eq!(
            result.as_slice(),
            [14, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 49, 50, 51, 52, 0, 0]
        );
    }
}
