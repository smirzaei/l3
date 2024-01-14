use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrameError {
    #[error("invalid version {0} (expected 1)")]
    InvalidVersion(u8),
}

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
    pub fn from_bytes(buff: &[u8; 8]) -> Result<Self> {
        let version = buff[0];
        if version != 1 {
            return Err(FrameError::InvalidVersion(version).into());
        }

        let msg_length = ((buff[4] as u32)
            | ((buff[5] as u32) << 8)
            | ((buff[6] as u32) << 16)
            | ((buff[7] as u32) << 24));

        Ok(Frame {
            version,
            p1: buff[1],
            p2: buff[2],
            p3: buff[3],
            msg_length,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::frame::FrameError;

    use super::Frame;
    use anyhow::{Ok, Result};

    #[test]
    fn from_bytes_return_error_if_version_is_not_one() -> Result<()> {
        let b: [u8; 8] = [0x00, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let result = Frame::from_bytes(&b);
        match result.unwrap_err().downcast::<FrameError>().unwrap() {
            FrameError::InvalidVersion(0) => Ok(()),
            _ => {
                panic!("invalid error");
            }
        }
    }

    #[test]
    fn from_bytes_properly_constructs_a_frame() -> Result<()> {
        let b: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let expected = Frame {
            version: 1,
            p1: 2,
            p2: 3,
            p3: 4,
            msg_length: unsafe { std::mem::transmute::<[u8; 4], u32>([0x05, 0x06, 0x07, 0x08]) }
                .to_le(),
        };

        let result = Frame::from_bytes(&b)?;

        assert_eq!(expected, result);
        Ok(())
    }
}
