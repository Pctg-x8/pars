
use crate::ffi as base;
use std::ptr::NonNull;

pub trait MainloopAPI
{
	fn vtable(&self) -> *mut base::pa_mainloop_api;
}

pub struct Threaded(NonNull<base::pa_threaded_mainloop>);
impl MainloopAPI for Threaded
{
	fn vtable(&self) -> *mut base::pa_mainloop_api
	{
		unsafe { base::pa_threaded_mainloop_get_api(self.0.as_ptr()) }
	}
}
impl Threaded
{
	pub fn new() -> Option<Self>
	{
		NonNull::new(unsafe { base::pa_threaded_mainloop_new() }).map(Self)
	}
	pub fn start(&mut self) -> Result<(), isize>
	{
		let rval = unsafe { base::pa_threaded_mainloop_start(self.0.as_ptr()) };
		if rval < 0 { Err(rval as _) } else { Ok(()) }
	}
	pub fn stop(&mut self)
	{
		unsafe { base::pa_threaded_mainloop_stop(self.0.as_ptr()); }
	}
}
impl Drop for Threaded
{
	fn drop(&mut self)
	{
		unsafe { base::pa_threaded_mainloop_free(self.0.as_ptr()); }
	}
}
