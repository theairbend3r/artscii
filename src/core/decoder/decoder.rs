use anyhow::Result;

use crate::core::frame::Frame;

pub trait Decoder {
    fn decode(&self) -> Result<Frame>;
}

pub trait DecoderIter {
    type Iter: Iterator<Item = Result<Frame, String>>;

    fn decode(&mut self) -> Self::Iter;
}
