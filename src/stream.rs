
use crate::ffi as base;
use std::ptr::{NonNull, null, null_mut};
use std::mem::transmute;
use std::ffi::{CString, CStr};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::task::{Context, Poll, Waker};
use std::pin::Pin;
use super::{SampleSpec, ChannelMap};
use libc::c_void;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
	pub fn new_stream(&mut self, name: &str, sample: &SampleSpec, channel_map: Option<&ChannelMap>) -> Option<Stream>
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
	pub fn await_new_state(&mut self) -> StreamStateChangeAwaiter
	{
		StreamStateChangeAwaiter
		{
			s: self,
			changed: Arc::new(AtomicBool::new(false)),
			callback_context: None
		}
	}
	pub async fn await_state_until(&mut self, state: State)
	{
		let mut current_st = self.state();
		while current_st != state { current_st = self.await_new_state().await; }
	}

	pub fn device_name(&self) -> &str
	{
		unsafe { CStr::from_ptr(base::pa_stream_get_device_name(self.0.as_ptr())).to_str().unwrap() }
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
	pub fn disconnect(&mut self)
	{
		unsafe { base::pa_stream_disconnect(self.0.as_ptr()); }
	}

	pub fn set_write_request_callback<F>(&mut self, callback: &mut F) where F: FnMut(usize) + 'static
	{
		extern "C" fn wcb_wrap<F>(_: *mut base::pa_stream, nbytes: libc::size_t, ctx: *mut c_void) where F: FnMut(usize) + 'static
		{
			unsafe { (*(ctx as *mut F))(nbytes as _); }
		}
		unsafe { base::pa_stream_set_write_callback(self.0.as_ptr(), Some(wcb_wrap::<F>), callback as *mut F as _) }
	}
}

struct CallbackContext { mux: Option<Waker>, flag: Arc<AtomicBool> }
pub struct StreamStateChangeAwaiter<'a>
{
	s: &'a mut Stream,
	changed: Arc<AtomicBool>,
	callback_context: Option<Pin<Box<CallbackContext>>>
}
impl<'a> std::future::Future for StreamStateChangeAwaiter<'a>
{
	type Output = State;
	fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<State>
	{
		if !self.changed.load(Ordering::Acquire)
		{
			let cbc = Box::pin(CallbackContext
			{
				mux: Some(cx.waker().clone()),
				flag: self.changed.clone()
			});
			extern "C" fn cb_internal(_: *mut base::pa_stream, ctx: *mut c_void)
			{
				let cbc = unsafe { &mut *(ctx as *mut CallbackContext) };
				cbc.flag.store(true, Ordering::Release);
				cbc.mux.take().unwrap().wake();
			}
			unsafe
			{
				base::pa_stream_set_state_callback(self.s.0.as_ptr(), Some(cb_internal), &*cbc as *const _ as *mut _);
			}
			self.get_mut().callback_context = Some(cbc);

			Poll::Pending
		}
		else
		{
			unsafe { base::pa_stream_set_state_callback(self.s.0.as_ptr(), None, null_mut()); }
			Poll::Ready(self.s.state())
		}
	}
}
