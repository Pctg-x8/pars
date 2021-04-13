
use libc::{c_void, c_int, timeval};

pub type pa_io_event_flags_t = u8;

pub enum pa_io_event {}
pub type pa_io_event_cb_t = extern "C" fn(ea: *mut pa_mainloop_api, e: *mut pa_io_event, fd: c_int, events: pa_io_event_flags_t, userdata: *mut c_void);
pub type pa_io_event_destroy_cb_t = extern "C" fn(a: *mut pa_mainloop_api, e: *mut pa_io_event, userdata: *mut c_void);

pub enum pa_time_event {}
pub type pa_time_event_cb_t = extern "C" fn(a: *mut pa_mainloop_api, e: *mut pa_time_event, tv: *const timeval, userdata: *mut c_void);
pub type pa_time_event_destroy_cb_t = extern "C" fn(a: *mut pa_mainloop_api, e: *mut pa_time_event, userdata: *mut c_void);

pub enum pa_defer_event {}
pub type pa_defer_event_cb_t = extern "C" fn(a: *mut pa_mainloop_api, e: *mut pa_defer_event, userdata: *mut c_void);
pub type pa_defer_event_destroy_cb_t = extern "C" fn(a: *mut pa_mainloop_api, e: *mut pa_defer_event, userdata: *mut c_void);

#[repr(C)]
pub struct pa_mainloop_api
{
	pub userdata: *mut c_void,
	pub io_new: extern "C" fn(a: *mut pa_mainloop_api, fd: c_int, events: pa_io_event_flags_t, cb: pa_io_event_cb_t, userdata: *mut c_void) -> *mut pa_io_event,
	pub io_enable: extern "C" fn(e: *mut pa_io_event, events: pa_io_event_flags_t),
	pub io_free: extern "C" fn(e: *mut pa_io_event),
	pub io_set_destroy: extern "C" fn(e: *mut pa_io_event, cb: pa_io_event_destroy_cb_t),
	pub time_new: extern "C" fn(a: *mut pa_mainloop_api, tv: *const timeval, cb: pa_time_event_cb_t, userdata: *mut c_void) -> *mut pa_time_event,
	pub time_restart: extern "C" fn(e: *mut pa_time_event, tv: *const timeval),
	pub time_free: extern "C" fn(e: *mut pa_time_event),
	pub time_set_destroy: extern "C" fn(e: *mut pa_time_event, cb: pa_time_event_destroy_cb_t),
	pub defer_new: extern "C" fn(a: *mut pa_mainloop_api, cb: pa_defer_event_cb_t, userdata: *mut c_void) -> *mut pa_defer_event,
	pub defer_enable: extern "C" fn(e: *mut pa_defer_event, b: c_int),
	pub defer_free: extern "C" fn(e: *mut pa_defer_event),
	pub defer_set_destroy: extern "C" fn(e: *mut pa_defer_event, cb: pa_defer_event_destroy_cb_t),
	pub quit: extern "C" fn(a: *mut pa_mainloop_api, retval: c_int)
}

pub enum pa_threaded_mainloop {}

#[link(name = "pulse")]
extern "C"
{
	pub fn pa_threaded_mainloop_new() -> *mut pa_threaded_mainloop;
	pub fn pa_threaded_mainloop_free(m: *mut pa_threaded_mainloop);
	pub fn pa_threaded_mainloop_start(m: *mut pa_threaded_mainloop) -> c_int;
	pub fn pa_threaded_mainloop_stop(m: *mut pa_threaded_mainloop);
	pub fn pa_threaded_mainloop_get_api(m: *mut pa_threaded_mainloop) -> *mut pa_mainloop_api;
	pub fn pa_threaded_mainloop_lock(m: *mut pa_threaded_mainloop);
	pub fn pa_threaded_mainloop_unlock(m: *mut pa_threaded_mainloop);
	pub fn pa_threaded_mainloop_wait(m: *mut pa_threaded_mainloop);
	pub fn pa_threaded_mainloop_signal(m: *mut pa_threaded_mainloop, wait_for_accept: c_int);
}
