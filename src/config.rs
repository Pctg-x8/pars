//! Channels and Samples

use crate::ffi as base;
use std::mem::MaybeUninit;

#[repr(transparent)]
pub struct ChannelMap(base::pa_channel_map);
impl ChannelMap
{
	pub fn new_mono() -> Self
	{
		let mut ch = MaybeUninit::uninit();
		unsafe { base::pa_channel_map_init_mono(ch.as_mut_ptr()); }
		
		Self(unsafe { ch.assume_init() })
	}
	pub fn new_stereo() -> Self
	{
		let mut ch = MaybeUninit::uninit();
		unsafe { base::pa_channel_map_init_stereo(ch.as_mut_ptr()); }

		Self(unsafe { ch.assume_init() })
	}

	pub fn as_ptr(&self) -> *const base::pa_channel_map { &self.0 }
}

pub type Sample = base::pa_sample_spec;
