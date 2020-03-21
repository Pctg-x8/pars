
use libc::{c_void, c_char, c_int};
use super::pa_mainloop_api;
use super::pa_spawn_api;

pub enum pa_context {}
pub type pa_context_notify_cb_t = extern "C" fn(c: *mut pa_context, userdata: *mut c_void);

pub type pa_context_state_t = i32;
pub const PA_CONTEXT_UNCONNECTED: pa_context_state_t = 0;
pub const PA_CONTEXT_CONNECTING: pa_context_state_t = 1;
pub const PA_CONTEXT_AUTHORIZING: pa_context_state_t = 2;
pub const PA_CONTEXT_SETTING_NAME: pa_context_state_t = 3;
pub const PA_CONTEXT_READY: pa_context_state_t = 4;
pub const PA_CONTEXT_FAILED: pa_context_state_t = 5;
pub const PA_CONTEXT_TERMINATED: pa_context_state_t = 6;

pub type pa_context_flags_t = u16;
pub const PA_CONTEXT_NOFLAGS: pa_context_flags_t = 0;
pub const PA_CONTEXT_NOAUTOSPAWN: pa_context_flags_t = 0x01;
pub const PA_CONTEXT_NOFAIL: pa_context_flags_t = 0x02;

#[link(name = "pulse")]
extern "C"
{
	pub fn pa_context_new(mainloop: *mut pa_mainloop_api, name: *const c_char) -> *mut pa_context;
	pub fn pa_context_ref(c: *mut pa_context) -> *mut pa_context;
	pub fn pa_context_unref(c: *mut pa_context);

	pub fn pa_context_set_state_callback(c: *mut pa_context, cb: Option<pa_context_notify_cb_t>, userdata: *mut c_void);
	pub fn pa_context_get_state(c: *const pa_context) -> pa_context_state_t;

	pub fn pa_context_connect(c: *mut pa_context, server: *const c_char, flags: pa_context_flags_t, api: *const pa_spawn_api) -> c_int;
}
