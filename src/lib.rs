pub mod message;
pub mod server;

#[cfg(test)]
mod tests {
    use crate::message::Message;

    #[test]
    fn it_creates_and_serializes_messages() {
        let message = Message::new([0x01, 0x00, 0x10, 0x00], vec![0x00, 0x11, 0xFF, 0x00]);

        assert_eq!(message.to_bytes(), vec![0x00, 0x00, 0x00, 0x10, 0x01, 0x00, 0x10, 0x00, 0x00, 0x11, 0xFF, 0x00, 0x96, 0xA9, 0xB2, 0x2E])
    }

    #[test]
    fn it_deserializes_messages() {
        let bytes = vec![0x00, 0x00, 0x00, 0x10, 0x01, 0x00, 0x10, 0x00, 0x00, 0x11, 0xFF, 0x00, 0x96, 0xA9, 0xB2, 0x2E];
        let message = Message::from_bytes(&bytes);
        let other_message = Message::new([0x01, 0x00, 0x10, 0x00], vec![0x00, 0x11, 0xFF, 0x00]);

        assert_eq!(message, Ok(other_message))
    }
}
