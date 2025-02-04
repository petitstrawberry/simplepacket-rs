# simplepacket-rs

[SimplePacket](https://github.com/petitstrawberry/SimplePacket) implementation in Rust.

## Usage

```rust
use simplepacket::{Frame, Packet};

fn main() {
    // Create some packets
    let packet1 = Packet::new(0x01, vec![0x01, 0x02, 0x03]);
    let packet2 = Packet::new(0x02, vec![0x01, 0x02]);

    // Create a frame with the packets
    let frame = Frame::new(vec![packet1, packet2]);

    // Encode the frame to bytes
    let encoded_data = frame.to_bytes();
    println!("Encoded data: {:?}", encoded_data);

    // Decode the frame from bytes
    let decoded_frame = Frame::from_bytes(&encoded_data).unwrap();
    println!("Decoded frame: {:?}", decoded_frame.packets());
}
```
