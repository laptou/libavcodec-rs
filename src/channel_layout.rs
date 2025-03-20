use std::{fmt::Debug, mem::MaybeUninit};

use libavcodec_sys::av_channel_layout_default;
use num_traits::FromPrimitive;

use crate::{AVChannelOrder, sys};

#[derive(Clone, Copy)]
pub struct ChannelLayout(pub sys::AVChannelLayout);

impl ChannelLayout {
    pub fn count(&self) -> usize {
        self.0.nb_channels as usize
    }

    pub fn set_count(&mut self, count: usize) {
        self.0.nb_channels = count as i32;
    }

    pub fn order(&self) -> AVChannelOrder {
      AVChannelOrder::from_u32(self.0.order).unwrap()
    }

    pub fn set_order(&mut self, order: AVChannelOrder) {
      self.0.order = order as u32;
    }

    pub fn new(channel_count: usize) -> Self {
        Self(unsafe {
            let mut cl = MaybeUninit::zeroed();
            av_channel_layout_default(cl.as_mut_ptr(), channel_count as i32);
            cl.assume_init()
        })
    }
}

impl Debug for ChannelLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChannelLayout")
            .field("count", &self.count())
            .field("order", &self.order())
            .finish()
    }
}
