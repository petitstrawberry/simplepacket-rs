use crate::DecoderError;

#[derive(Debug, PartialEq, Eq)]
pub enum PacketDecoderError {
    InvalidPacket,
    InvalidPayload,
}

impl std::error::Error for PacketDecoderError {}

impl std::fmt::Display for PacketDecoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PacketDecoderError::InvalidPacket => write!(f, "Invalid packet"),
            PacketDecoderError::InvalidPayload => write!(f, "Invalid payload"),
        }
    }
}

impl From<PacketDecoderError> for DecoderError {
    fn from(error: PacketDecoderError) -> Self {
        DecoderError::Packet(error)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet {
    pub packet_type: u8,
    pub length: u16,
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn new(packet_type: u8, payload: Vec<u8>) -> Self {
        let length = payload.len() as u16;
        Packet {
            packet_type,
            length,
            payload,
        }
    }

    pub fn eof() -> Self {
        Packet {
            packet_type: 0,
            length: 0,
            payload: Vec::new(),
        }
    }

    /// Encode the packet into a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.packet_type);
        bytes.push((self.length & 0xff) as u8);
        bytes.push((self.length >> 8) as u8);
        bytes.extend(&self.payload);
        bytes
    }

    /// Decode a packet from a byte slice.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PacketDecoderError> {
        if bytes.len() < 3 {
            return Err(PacketDecoderError::InvalidPacket);
        }
        let packet_type = bytes[0];
        let length = u16::from(bytes[1]) | (u16::from(bytes[2]) << 8);
        if bytes.len() < 3 + length as usize {
            return Err(PacketDecoderError::InvalidPayload);
        }
        let payload = bytes[3..3 + length as usize].to_vec();
        Ok(Packet {
            packet_type,
            length,
            payload,
        })
    }
}