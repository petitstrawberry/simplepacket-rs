mod frame;
mod packet;

/// Decoder Error
#[derive(Debug)]
pub enum DecoderError {
    Frame(frame::FrameDecoderError),
    Packet(packet::PacketDecoderError),
}

impl std::error::Error for DecoderError {}

impl std::fmt::Display for DecoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DecoderError::Frame(e) => write!(f, "Frame decoder error: {:?}", e),
            DecoderError::Packet(e) => write!(f, "Packet decoder error: {:?}", e),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::Packet;
    use crate::frame::Frame;

    #[test]
    fn test_packet_new() {
        let payload = vec![1, 2, 3];
        let packet = Packet::new(1, payload.clone());
        assert_eq!(packet.packet_type, 1);
        assert_eq!(packet.length, 3);
        assert_eq!(packet.payload, payload);
    }

    #[test]
    fn test_packet_eof() {
        let packet = Packet::eof();
        assert_eq!(packet.packet_type, 0);
        assert_eq!(packet.length, 0);
        assert_eq!(packet.payload, Vec::new());
    }

    #[test]
    fn test_packet_to_bytes() {
        let payload = vec![1, 2, 3];
        let packet = Packet::new(1, payload.clone());
        let bytes = packet.to_bytes();
        assert_eq!(bytes, vec![1, 3, 0, 1, 2, 3]);
    }

    #[test]
    fn test_packet_from_bytes() {
        let bytes = vec![1, 3, 0, 1, 2, 3];
        let packet = Packet::from_bytes(&bytes).unwrap();
        assert_eq!(packet.packet_type, 1);
        assert_eq!(packet.length, 3);
        assert_eq!(packet.payload, vec![1, 2, 3]);
    }

    #[test]
    fn test_frame_new() {
        let packets = vec![Packet::new(1, vec![1, 2, 3])];
        let frame = Frame::new(packets.clone());
        assert_eq!(frame.packets(), &packets);
    }

    #[test]
    fn test_frame_encode() {
        let packets = vec![Packet::new(1, vec![1, 2, 3])];
        let frame = Frame::new(packets);
        let bytes = frame.to_bytes();
        assert_eq!(bytes, vec![1, 3, 0, 1, 2, 3, 0, 0, 0]);
    }

    #[test]
    fn test_frame_decode() {
        let bytes = vec![1, 3, 0, 1, 2, 3, 0, 0, 0];
        let frame = Frame::from_bytes(&bytes).unwrap();
        let packets = frame.packets();
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].packet_type, 1);
        assert_eq!(packets[0].length, 3);
        assert_eq!(packets[0].payload, vec![1, 2, 3]);
    }

    #[test]
    fn test_frame_encode_decode() {
        let packets = vec![
            Packet::new(0x01, vec![0x01, 0x02, 0x03]),
            Packet::new(0x02, vec![0x01, 0x02]),
        ];
        let frame = Frame::new(packets.clone());
        let data = frame.to_bytes();
        let decoded_frame = Frame::from_bytes(&data).unwrap();
        assert_eq!(decoded_frame.packets(), &packets);
    }

    #[test]
    fn test_invalid_frame() {
        let data = vec![0x01, 0x03];
        match Frame::from_bytes(&data) {
            Err(DecoderError::Frame(frame::FrameDecoderError::InvalidFrame)) => (),
            _ => panic!("Expected an error"),
        }
    }

    #[test]
    fn test_invalid_payload() {
        let data = vec![0x01, 0x03, 0x00, 0x01, 0x02, 0x03, 0x02, 0x02, 0x00, 0x01];
        match Frame::from_bytes(&data) {
            Err(DecoderError::Packet(packet::PacketDecoderError::InvalidPayload)) => (),
            _ => panic!("Expected an error"),
        }
    }
}