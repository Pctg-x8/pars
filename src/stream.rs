
use crate::ffi as base;
use std::ptr::{NonNull, null, null_mut};
use std::mem::{transmute, MaybeUninit};
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
pub const NO_REMIX_CHANNELS: Flags = base::PA_STREAM_NO_REMIX_CHANNELS;
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

pub type SeekMode = base::pa_seek_mode_t;
pub const SEEK_RELATIVE: SeekMode = base::PA_SEEK_RELATIVE;
pub const SEEK_ABSOLUTE: SeekMode = base::PA_SEEK_ABSOLUTE;
pub const SEEK_RELATIVE_ON_READ: SeekMode = base::PA_SEEK_RELATIVE_ON_READ;
pub const SEEK_RELATIVE_END: SeekMode = base::PA_SEEK_RELATIVE_END;

pub struct StreamRef(*mut base::pa_stream);
pub struct Stream(NonNull<base::pa_stream>);
impl std::ops::Deref for Stream
{
	type Target = StreamRef;
	fn deref(&self) -> &StreamRef { unsafe { transmute(self) } }
}
impl std::ops::DerefMut for Stream
{
	fn deref_mut(&mut self) -> &mut StreamRef { unsafe { transmute(self) } }
}
impl Clone for Stream
{
	fn clone(&self) -> Self
	{
		Self(unsafe { NonNull::new_unchecked(base::pa_stream_ref(self.0.as_ptr())) })
	}
}
impl std::borrow::Borrow<StreamRef> for Stream
{
	fn borrow(&self) -> &StreamRef { std::ops::Deref::deref(self) }
}
impl ToOwned for StreamRef
{
	type Owned = Stream;

	fn to_owned(&self) -> Stream
	{
		Stream(unsafe { NonNull::new_unchecked(base::pa_stream_ref(self.0)) })
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
impl StreamRef
{
	pub fn state(&self) -> State
	{
		unsafe { transmute(base::pa_stream_get_state(self.0)) }
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
		unsafe { CStr::from_ptr(base::pa_stream_get_device_name(self.0)).to_str().unwrap() }
	}
	pub fn is_suspended(&self) -> Result<bool, isize>
	{
		let r = unsafe { base::pa_stream_is_suspended(self.0) };
		if r < 0 { Err(r as _) } else { Ok(r == 1) }
	}
	pub fn is_corked(&self) -> Result<bool, isize>
	{
		let r = unsafe { base::pa_stream_is_corked(self.0) };
		if r < 0 { Err(r as _) } else { Ok(r == 1) }
	}

	pub fn connect_playback(&mut self, dev: Option<&str>, attr: Option<&BufferAttr>, flags: Flags, volume: Option<&CVolume>, sync_stream: Option<&mut Stream>) -> Result<(), isize>
	{
		let dev_c = dev.map(|n| CString::new(n).unwrap());
		let r = unsafe
		{
			base::pa_stream_connect_playback(self.0,
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
		unsafe { base::pa_stream_disconnect(self.0); }
	}

	pub fn set_write_request_callback<W>(&mut self, handler: Pin<&mut W>) where W: WriteRequestHandler + Unpin
	{
		extern "C" fn wcb_wrap<W>(sref: *mut base::pa_stream, nbytes: libc::size_t, ctx: *mut c_void) where W: WriteRequestHandler
		{
			unsafe { (*(ctx as *mut W)).callback(&mut StreamRef(sref), nbytes); }
		}
		unsafe { base::pa_stream_set_write_callback(self.0, Some(wcb_wrap::<W>), handler.get_mut() as *mut W as _) }
	}
	pub fn begin_write(&mut self, request_buffer_size: usize) -> Result<(*mut c_void, usize), isize>
	{
		let mut buffer = MaybeUninit::uninit();
		let mut nbytes: libc::size_t = request_buffer_size as _;
		let r = unsafe { base::pa_stream_begin_write(self.0, buffer.as_mut_ptr(), &mut nbytes) };
		if r != 0 { Err(r as _) }
		else
		{
			unsafe { Ok((buffer.assume_init(), nbytes as usize)) }
		}
	}
	pub fn cancel_write(&mut self) -> Result<(), isize>
	{
		let r = unsafe { base::pa_stream_cancel_write(self.0) };
		if r != 0 { Err(r as _) } else { Ok(()) }
	}
	pub fn write_slice<D>(&mut self, data: &[D], seek_offset: Option<(SeekMode, i64)>) -> Result<(), isize>
	{
		let (seek, offset) = seek_offset.unwrap_or((0, 0));
		let r = unsafe
		{
			base::pa_stream_write(self.0, data.as_ptr() as *const _, std::mem::size_of::<D>() * data.len() as usize, None, offset, seek)
		};
		if r != 0 { Err(r as _) } else { Ok(()) }
	}
}

pub trait WriteRequestHandler
{
	fn callback(&mut self, stream: &mut StreamRef, nbytes: usize);
}

struct CallbackContext { mux: Option<Waker>, flag: Arc<AtomicBool> }
pub struct StreamStateChangeAwaiter<'a>
{
	s: &'a mut StreamRef,
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
				println!("cb_internal");
				let cbc = unsafe { &mut *(ctx as *mut CallbackContext) };
				cbc.flag.store(true, Ordering::Release);
				cbc.mux.take().unwrap().wake();
			}
			println!("pa_stream_set_state_callback");
			unsafe
			{
				base::pa_stream_set_state_callback(self.s.0, Some(cb_internal), &*cbc as *const _ as *mut _);
			}
			self.get_mut().callback_context = Some(cbc);

			Poll::Pending
		}
		else
		{
			unsafe { base::pa_stream_set_state_callback(self.s.0, None, null_mut()); }
			Poll::Ready(self.s.state())
		}
	}
}
