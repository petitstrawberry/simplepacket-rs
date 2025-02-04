use crate::DecoderError;
use crate::packet::Packet;

#[derive(Debug)]
pub enum FrameDecoderError {
    InvalidFrame,
    EofNotFound,
}

impl std::error::Error for FrameDecoderError {}

impl std::fmt::Display for FrameDecoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FrameDecoderError::InvalidFrame => write!(f, "Invalid frame"),
            FrameDecoderError::EofNotFound => write!(f, "EOF packet not found"),
        }
    }
}

impl From<FrameDecoderError> for DecoderError {
    fn from(error: FrameDecoderError) -> Self {
        DecoderError::Frame(error)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    packets: Vec<Packet>,
}

impl Frame {
    pub fn new(packets: Vec<Packet>) -> Self {
        Frame { packets }
    }

    /// Encode the frame into a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for packet in &self.packets {
            data.extend(packet.to_bytes());
        }
        data.extend(Packet::eof().to_bytes());
        data
    }

    /// Decode a frame from a byte slice.
    pub fn from_bytes(data: &[u8]) -> Result<Self, DecoderError> {
        let mut packets = Vec::new();
        let mut i = 0;
        if data.len() < 3 {
            return Err(FrameDecoderError::InvalidFrame.into());
        }
        while i < data.len() {
            let packet = Packet::from_bytes(&data[i..])?;
            if packet.packet_type == 0 {
                break; // EOF packet
            }
            i += 3 + packet.length as usize;
            packets.push(packet);
        }
        Ok(Frame { packets })
    }

    pub fn packets(&self) -> &Vec<Packet> {
        &self.packets
    }
}
