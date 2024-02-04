use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrameError {
    #[error("invalid version {0} (expected 1)")]
    InvalidVersion(u8),
    #[error("message length cannot be 0")]
    ZeroMessageLength,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Frame {
    version: u8,
    // Placeholder for reserved bytes
    p1: u8,
    p2: u8,
    p3: u8,
    pub msg_len: u32,
}

impl Frame {
    pub fn new(version: u8, msg_len: u32) -> Self {
        Frame {
            version,
            p1: 0,
            p2: 0,
            p3: 0,
            msg_len,
        }
    }

    pub fn from_bytes(buff: &[u8; 8]) -> Result<Self, FrameError> {
        let version = buff[0];
        if version != 1 {
            return Err(FrameError::InvalidVersion(version));
        }

        let msg_length = (buff[4] as u32)
            | ((buff[5] as u32) << 8)
            | ((buff[6] as u32) << 16)
            | ((buff[7] as u32) << 24);

        if msg_length == 0 {
            return Err(FrameError::ZeroMessageLength);
        }

        Ok(Frame {
            version,
            p1: buff[1],
            p2: buff[2],
            p3: buff[3],
            msg_len: msg_length,
        })
    }

    pub fn as_bytes(&self) -> [u8; 8] {
        let [b1, b2, b3, b4] = self.msg_len.to_le_bytes();
        [self.version, self.p1, self.p2, self.p3, b1, b2, b3, b4]
    }
}

#[cfg(test)]
mod test {
    use crate::frame::FrameError;

    use super::Frame;

    #[test]
    fn from_bytes_return_error_if_version_is_not_one() -> Result<(), FrameError> {
        let b: [u8; 8] = [0x00, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let result = Frame::from_bytes(&b);
        match result.unwrap_err() {
            FrameError::InvalidVersion(0) => Ok(()),
            _ => {
                panic!("invalid error");
            }
        }
    }

    #[test]
    fn from_bytes_return_error_if_message_len_is_zero() -> Result<(), FrameError> {
        let b: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x00, 0x00, 0x00, 0x00];
        let result = Frame::from_bytes(&b);
        match result.unwrap_err() {
            FrameError::ZeroMessageLength => Ok(()),
            _ => {
                panic!("invalid error");
            }
        }
    }

    #[test]
    fn from_bytes_properly_constructs_a_frame() -> Result<(), FrameError> {
        let b: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let expected = Frame {
            version: 1,
            p1: 2,
            p2: 3,
            p3: 4,
            msg_len: unsafe { std::mem::transmute::<[u8; 4], u32>([0x05, 0x06, 0x07, 0x08]) }
                .to_le(),
        };

        let result = Frame::from_bytes(&b)?;

        assert_eq!(expected, result);
        Ok(())
    }
}
