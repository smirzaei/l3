#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Frame {
    version: u8,
    // Placeholder for reserved bytes
    p1: u8,
    p2: u8,
    p3: u8,
    msg_length: u32,
}

impl Frame {
    pub fn from_bytes(buff: &[u8; 8]) -> Self {
        Frame {
            version: buff[0],
            p1: buff[1],
            p2: buff[2],
            p3: buff[3],
            msg_length: ((buff[4] as u32)
                | ((buff[5] as u32) << 8)
                | ((buff[6] as u32) << 16)
                | ((buff[7] as u32) << 24)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Frame;

    #[test]
    fn from_bytes_properly_constructs_a_frame() {
        let b: [u8; 8] = [0x1, 0x2, 0x3, 0x4, 0x05, 0x06, 0x07, 0x08];
        let expected = Frame {
            version: 1,
            p1: 2,
            p2: 3,
            p3: 4,
            msg_length: unsafe { std::mem::transmute::<[u8; 4], u32>([0x05, 0x06, 0x07, 0x08]) }
                .to_le(),
        };

        let result = Frame::from_bytes(&b);

        assert_eq!(expected, result);
    }
}
