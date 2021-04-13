
use crate::ffi::{self as base, pa_signal_init};
use std::ptr::NonNull;

pub trait MainloopAPI
{
	fn vtable(&self) -> *mut base::pa_mainloop_api;

	fn init_signal(&self) {
		unsafe { pa_signal_init(self.vtable()); }
	}
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

	pub fn lock(&mut self) {
		unsafe { base::pa_threaded_mainloop_lock(self.0.as_ptr()) }
	}
	pub fn unlock(&mut self) {
		unsafe { base::pa_threaded_mainloop_unlock(self.0.as_ptr()) }
	}

	pub fn wait(&mut self) {
		unsafe { base::pa_threaded_mainloop_wait(self.0.as_ptr()); }
	}
	pub fn signal(&mut self, wait_for_accept: bool) {
		unsafe { base::pa_threaded_mainloop_signal(self.0.as_ptr(), if wait_for_accept { 1 } else { 0 }); }
	}
}
impl Drop for Threaded
{
	fn drop(&mut self)
	{
		unsafe { base::pa_threaded_mainloop_free(self.0.as_ptr()); }
	}
}

#[repr(transparent)]
pub struct LockedLoop<'a>(NonNull<base::pa_threaded_mainloop>, std::marker::PhantomData<&'a Threaded>);
impl<'a> LockedLoop<'a> {
	pub fn new(t: &'a Threaded) -> Self {
		unsafe { base::pa_threaded_mainloop_lock(t.0.as_ptr()) };
		Self(t.0, std::marker::PhantomData)
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

	pub fn wait(&mut self) {
		unsafe { base::pa_threaded_mainloop_wait(self.0.as_ptr()); }
	}
	pub fn signal(&mut self, wait_for_accept: bool) {
		unsafe { base::pa_threaded_mainloop_signal(self.0.as_ptr(), if wait_for_accept { 1 } else { 0 }); }
	}
}
impl Drop for LockedLoop<'_> {
	fn drop(&mut self) {
		unsafe { base::pa_threaded_mainloop_unlock(self.0.as_ptr()) };
	}
}
impl MainloopAPI for LockedLoop<'_> {
	fn vtable(&self) -> *mut base::pa_mainloop_api {
		unsafe { base::pa_threaded_mainloop_get_api(self.0.as_ptr()) }
	}
}

pub struct LockedScope<'a>(&'a mut Threaded);
impl<'a> LockedScope<'a> {
	pub fn enter(t: &'a mut Threaded) -> Self {
		t.lock();
		Self(t)
	}
}
impl Drop for LockedScope<'_> {
	fn drop(&mut self) {
		self.0.unlock();
	}
}
impl std::ops::Deref for LockedScope<'_> {
	type Target = Threaded;
	fn deref(&self) -> &Threaded { self.0 }
}
impl std::ops::DerefMut for LockedScope<'_> {
	fn deref_mut(&mut self) -> &mut Threaded { self.0 }
}
