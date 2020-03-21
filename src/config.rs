//! Channels and Samples

use crate::ffi as base;
use std::mem::MaybeUninit;

#[repr(C)]
pub enum SampleFormat
{
	U8 = base::PA_SAMPLE_U8 as _,
	ALAW = base::PA_SAMPLE_ALAW as _,
	ULAW = base::PA_SAMPLE_ULAW as _,
	S16LE = base::PA_SAMPLE_S16LE as _,
	S16BE = base::PA_SAMPLE_S16BE as _,
	FLOAT32LE = base::PA_SAMPLE_FLOAT32LE as _,
	FLOAT32BE = base::PA_SAMPLE_FLOAT32BE as _,
	S32LE = base::PA_SAMPLE_S32LE as _,
	S32BE = base::PA_SAMPLE_S32BE as _,
	S24LE = base::PA_SAMPLE_S24LE as _,
	S23BE = base::PA_SAMPLE_S24BE as _,
	S24_32LE = base::PA_SAMPLE_S24_32LE as _,
	S24_32BE = base::PA_SAMPLE_S24_32BE as _
}

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

pub type SampleSpec = base::pa_sample_spec;
