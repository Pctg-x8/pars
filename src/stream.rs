
use crate::ffi as base;
use std::ptr::{NonNull, null, null_mut};
use std::mem::transmute;
use std::ffi::CString;
use super::{Sample, ChannelMap};

#[repr(C)]
pub enum State
{
	Unconnected = base::PA_STREAM_UNCONNECTED as _,
	Creating = base::PA_STREAM_CREATING as _,
	Ready = base::PA_STREAM_READY as _,
	Failed = base::PA_STREAM_FAILED as _,
	Terminated = base::PA_STREAM_TERMINATED as _
}

pub type Flags = base::pa_stream_flags_t;
pub const NOFLAGS: Flags = base::PA_STREAM_NOFLAGS;
pub const START_CORKED: Flags = base::PA_STREAM_START_CORKED;
pub const INTERPOLATE_TIMING: Flags = base::PA_STREAM_INTERPOLATE_TIMING;
pub const NOT_MONOTONIC: Flags = base::PA_STREAM_NOT_MONOTONIC;
pub const AUTO_TIMING_UPDATE: Flags = base::PA_STREAM_AUTO_TIMING_UPDATE;
pub const NO_REMAP_CHANNELS: Flags = base::PA_STREAM_NO_REMAP_CHANNELS;
pub const NO_REMIX_CHANNELS: Flags = base::PA_STREAM_NO_REMAP_CHANNELS;
pub const FIX_FORMAT: Flags = base::PA_STREAM_FIX_FORMAT;
pub const FIX_RATE: Flags = base::PA_STREAM_FIX_RATE;
pub const FIX_CHANNELS: Flags = base::PA_STREAM_FIX_CHANNELS;
pub const DONT_MOVE: Flags = base::PA_STREAM_DONT_MOVE;
pub const VARIABLE_RATE: Flags = base::PA_STREAM_VARIABLE_RATE;
pub const PEAK_DETECT: Flags = base::PA_STREAM_PEAK_DETECT;
pub const START_MUTED: Flags = base::PA_STREAM_START_MUTED;
pub const ADJUST_LATENCY: Flags = base::PA_STREAM_ADJUST_LATENCY;
pub const EARLY_REQUESTS: Flags = base::PA_STREAM_EARLY_REQUESTS;
pub const DONT_INHIBIT_AUTO_SUSPEND: Flags = base::PA_STREAM_DONT_INHIBIT_AUTO_SUSPEND;
pub const START_UNMUTED: Flags = base::PA_STREAM_START_UNMUTED;
pub const FAIL_ON_SUSPEND: Flags = base::PA_STREAM_FAIL_ON_SUSPEND;
pub const RELATIVE_VOLUME: Flags = base::PA_STREAM_RELATIVE_VOLUME;
pub const PASSTHROUGH: Flags = base::PA_STREAM_PASSTHROUGH;

pub type BufferAttr = base::pa_buffer_attr;
pub type CVolume = base::pa_cvolume;

pub struct Stream(NonNull<base::pa_stream>);
impl Clone for Stream
{
	fn clone(&self) -> Self
	{
		Self(unsafe { NonNull::new_unchecked(base::pa_stream_ref(self.0.as_ptr())) })
	}
}
impl Drop for Stream
{
	fn drop(&mut self)
	{
		unsafe { base::pa_stream_unref(self.0.as_ptr()); }
	}
}
impl super::Context
{
	pub fn new_stream(&mut self, name: &str, sample: &Sample, channel_map: Option<&ChannelMap>) -> Option<Stream>
	{
		let name_c = CString::new(name).unwrap();
		let p = unsafe
		{
			base::pa_stream_new(self.as_mut_ptr(), name_c.as_ptr(), sample, channel_map.map(|p| p.as_ptr()).unwrap_or_else(null))
		};

		NonNull::new(p).map(Stream)
	}
}
impl Stream
{
	pub fn state(&self) -> State
	{
		unsafe { transmute(base::pa_stream_get_state(self.0.as_ptr())) }
	}

	pub fn connect_playback(&mut self, dev: Option<&str>, attr: Option<&BufferAttr>, flags: Flags, volume: Option<&CVolume>, sync_stream: Option<&mut Stream>) -> Result<(), isize>
	{
		let dev_c = dev.map(|n| CString::new(n).unwrap());
		let r = unsafe
		{
			base::pa_stream_connect_playback(self.0.as_ptr(),
				dev_c.as_ref().map(|cs| cs.as_ptr()).unwrap_or_else(null),
				attr.map(|p| p as *const _).unwrap_or_else(null),
				flags,
				volume.map(|p| p as *const _).unwrap_or_else(null),
				sync_stream.map(|p| p.0.as_ptr()).unwrap_or_else(null_mut))
		};
		if r == 0 { Ok(()) } else { Err(r as isize) }
	}
}
