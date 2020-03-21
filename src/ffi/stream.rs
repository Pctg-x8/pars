
use libc::{c_char, c_int, size_t, c_void};
use super::{pa_context, pa_sample_spec, pa_channel_map, PA_CHANNELS_MAX};

pub enum pa_stream {}

pub type pa_stream_request_cb_t = extern "C" fn(p: *mut pa_stream, nbytes: size_t, userdata: *mut c_void);
pub type pa_stream_notify_cb_t = extern "C" fn(p: *mut pa_stream, userdata: *mut c_void);

pub type pa_stream_state_t = i32;
pub const PA_STREAM_UNCONNECTED: pa_stream_state_t = 0;
pub const PA_STREAM_CREATING: pa_stream_state_t = 1;
pub const PA_STREAM_READY: pa_stream_state_t = 2;
pub const PA_STREAM_FAILED: pa_stream_state_t = 3;
pub const PA_STREAM_TERMINATED: pa_stream_state_t = 4;

#[repr(C)]
pub struct pa_buffer_attr
{
	pub tlength: u32,
	pub prebuf: u32,
	pub minreq: u32,
	pub fragsize: u32
}

pub type pa_stream_flags_t = i32;
pub const PA_STREAM_NOFLAGS: pa_stream_flags_t = 0;
pub const PA_STREAM_START_CORKED: pa_stream_flags_t = 0x01;
pub const PA_STREAM_INTERPOLATE_TIMING: pa_stream_flags_t = 0x02;
pub const PA_STREAM_NOT_MONOTONIC: pa_stream_flags_t = 0x04;
pub const PA_STREAM_AUTO_TIMING_UPDATE: pa_stream_flags_t = 0x08;
pub const PA_STREAM_NO_REMAP_CHANNELS: pa_stream_flags_t = 0x10;
pub const PA_STREAM_NO_REMIX_CHANNELS: pa_stream_flags_t = 0x20;
pub const PA_STREAM_FIX_FORMAT: pa_stream_flags_t = 0x40;
pub const PA_STREAM_FIX_RATE: pa_stream_flags_t = 0x80;
pub const PA_STREAM_FIX_CHANNELS: pa_stream_flags_t = 0x100;
pub const PA_STREAM_DONT_MOVE: pa_stream_flags_t = 0x200;
pub const PA_STREAM_VARIABLE_RATE: pa_stream_flags_t = 0x400;
pub const PA_STREAM_PEAK_DETECT: pa_stream_flags_t = 0x800;
pub const PA_STREAM_START_MUTED: pa_stream_flags_t = 0x1000;
pub const PA_STREAM_ADJUST_LATENCY: pa_stream_flags_t = 0x2000;
pub const PA_STREAM_EARLY_REQUESTS: pa_stream_flags_t = 0x4000;
pub const PA_STREAM_DONT_INHIBIT_AUTO_SUSPEND: pa_stream_flags_t = 0x8000;
pub const PA_STREAM_START_UNMUTED: pa_stream_flags_t = 0x10000;
pub const PA_STREAM_FAIL_ON_SUSPEND: pa_stream_flags_t = 0x20000;
pub const PA_STREAM_RELATIVE_VOLUME: pa_stream_flags_t = 0x40000;
pub const PA_STREAM_PASSTHROUGH: pa_stream_flags_t = 0x80000;

pub type pa_volume_t = u32;
pub const PA_VOLUME_NORM: pa_volume_t = 0x10000;
pub const PA_VOLUME_MUTED: pa_volume_t = 0;
pub const PA_VOLUME_MAX: pa_volume_t = std::u32::MAX / 2;

#[repr(C)]
pub struct pa_cvolume
{
	pub channels: u8,
	pub values: [pa_volume_t; PA_CHANNELS_MAX]
}

#[link(name = "pulse")]
extern "C"
{
	pub fn pa_stream_new(c: *mut pa_context, name: *const c_char, ss: *const pa_sample_spec, map: *const pa_channel_map) -> *mut pa_stream;
	pub fn pa_stream_ref(s: *mut pa_stream) -> *mut pa_stream;
	pub fn pa_stream_unref(s: *mut pa_stream);

	pub fn pa_stream_get_state(s: *const pa_stream) -> pa_stream_state_t;
	pub fn pa_stream_get_device_name(s: *const pa_stream) -> *const c_char;
	pub fn pa_stream_connect_playback(s: *mut pa_stream, dev: *const c_char, attr: *const pa_buffer_attr, flags: pa_stream_flags_t, volume: *const pa_cvolume, sync_stream: *mut pa_stream) -> c_int;
	pub fn pa_stream_disconnect(s: *mut pa_stream);

	pub fn pa_stream_is_suspended(s: *const pa_stream) -> c_int;
	pub fn pa_stream_is_corked(s: *const pa_stream) -> c_int;

	pub fn pa_stream_set_state_callback(p: *mut pa_stream, cb: Option<pa_stream_notify_cb_t>, userdata: *mut c_void);
	pub fn pa_stream_set_write_callback(p: *mut pa_stream, cb: Option<pa_stream_request_cb_t>, userdata: *mut c_void);
}
