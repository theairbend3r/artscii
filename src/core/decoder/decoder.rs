use anyhow::Result;

use crate::core::frame::Frame;

pub trait Decoder {
    fn decode(&self) -> Result<Frame>;
}
