
use libc::c_void;
use crate::ffi as base;
use std::ptr::{NonNull, null_mut, null};
use std::mem::transmute;
use std::ffi::CString;
use std::sync::{Arc, atomic::AtomicBool, atomic::Ordering};
use std::task::{Context as TaskContext, Poll, Waker};
use super::mainloop::MainloopAPI;
use std::pin::Pin;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State
{
	Unconnected = base::PA_CONTEXT_UNCONNECTED as _,
	Connecting = base::PA_CONTEXT_CONNECTING as _,
	Authorizing = base::PA_CONTEXT_AUTHORIZING as _,
	SettingName = base::PA_CONTEXT_SETTING_NAME as _,
	Ready = base::PA_CONTEXT_READY as _,
	Failed = base::PA_CONTEXT_FAILED as _,
	Terminated = base::PA_CONTEXT_TERMINATED as _
}

pub type Flags = base::pa_context_flags_t;
pub const NOFLAGS: Flags = base::PA_CONTEXT_NOFLAGS;
pub const FLAG_NOAUTOSPAWN: Flags = base::PA_CONTEXT_NOAUTOSPAWN;
pub const FLAG_NOFAIL: Flags = base::PA_CONTEXT_NOFAIL;

pub type SpawnAPI = base::pa_spawn_api;

pub struct Context(NonNull<base::pa_context>);
impl Clone for Context
{
	fn clone(&self) -> Self
	{
		unsafe
		{
			Context(NonNull::new_unchecked(base::pa_context_ref(self.0.as_ptr())))
		}
	}
}
impl Drop for Context
{
	fn drop(&mut self)
	{
		unsafe { base::pa_context_unref(self.0.as_ptr()); }
	}
}
impl Context
{
	pub fn new(mainloop: &impl MainloopAPI, name: &str) -> Option<Self>
	{
		let name = CString::new(name).unwrap();
		NonNull::new(unsafe { base::pa_context_new(mainloop.vtable(), name.as_ptr()) }).map(Self)
	}

	/*fn set_state_callback<F>(&mut self, callback: Option<Pin<Box<F>>>) where F: FnMut()
	{
		if let Some(cb) = callback
		{
			extern "C" fn cb_internal<F>(_: *mut base::pa_context, cb: *mut c_void) where F: FnMut()
			{
				unsafe { (*(cb as *mut F))(); }
			}
			unsafe { base::pa_context_set_state_callback(self.0.as_ptr(), Some(cb_internal::<F>), &*cb as *const _ as *mut _); }
		}
		else
		{
			unsafe { base::pa_context_set_state_callback(self.0.as_ptr(), None, null_mut()); }
		}
	}*/

	pub fn state(&self) -> State
	{
		unsafe { transmute(base::pa_context_get_state(self.0.as_ptr())) }
	}
	pub fn await_new_state(&mut self) -> ContextStateChangeAwaiter
	{
		ContextStateChangeAwaiter
		{
			c: self,
			changed: Arc::new(AtomicBool::new(false)),
			callback_context: None
		}
	}
	pub async fn await_state_until(&mut self, state: State)
	{
		let mut current_st = self.state();
		while current_st != state { current_st = self.await_new_state().await; }
	}

	pub fn connect(&mut self, server: Option<&str>, flags: Flags, spawn_api: Option<&SpawnAPI>) -> Result<(), isize>
	{
		let server_c = server.map(|s| CString::new(s).unwrap());
		let rval = unsafe
		{
			base::pa_context_connect(
				self.0.as_ptr(),
				server_c.as_ref().map(|p| p.as_ptr()).unwrap_or_else(null),
				flags, spawn_api.map(|p| p as *const _).unwrap_or_else(null)
			)
		};
		if rval < 0 { Err(rval as _) } else { Ok(()) }
	}
}

struct CallbackContext { mux: Option<Waker>, flag: Arc<AtomicBool> }
pub struct ContextStateChangeAwaiter<'a>
{
	c: &'a mut Context,
	changed: Arc<AtomicBool>,
	callback_context: Option<Pin<Box<CallbackContext>>>
}
impl<'a> std::future::Future for ContextStateChangeAwaiter<'a>
{
	type Output = State;
	fn poll(self: Pin<&mut Self>, cx: &mut TaskContext) -> Poll<Self::Output>
	{
		if !self.changed.load(Ordering::Acquire)
		{
			let cbc = Box::pin(CallbackContext
			{
				mux: Some(cx.waker().clone()),
				flag: self.changed.clone()
			});
			extern "C" fn cb_internal(_: *mut base::pa_context, ctx: *mut c_void)
			{
				let cbc = unsafe { &mut *(ctx as *mut CallbackContext) };
				cbc.flag.store(true, Ordering::Release);
				cbc.mux.take().unwrap().wake();
			}
			unsafe
			{
				base::pa_context_set_state_callback(self.c.0.as_ptr(), Some(cb_internal), &*cbc as *const _ as *mut _);
			}
			self.get_mut().callback_context = Some(cbc);

			Poll::Pending
		}
		else
		{
			unsafe
			{
				base::pa_context_set_state_callback(self.c.0.as_ptr(), None, null_mut());
			}
			Poll::Ready(self.c.state())
		}
	}
}
