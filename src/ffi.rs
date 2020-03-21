#![allow(non_camel_case_types)]

mod event; pub use self::event::*;
mod spawn; pub use self::spawn::*;
mod context; pub use self::context::*;
mod stream; pub use self::stream::*;
mod sample; pub use self::sample::*;
mod channelmap; pub use self::channelmap::*;

use libc::c_void;

pub type pa_free_cb_t = extern "C" fn(p: *mut c_void);
pub type pa_seek_mode_t = i32;
pub const PA_SEEK_RELATIVE: pa_seek_mode_t = 0;
pub const PA_SEEK_ABSOLUTE: pa_seek_mode_t = 1;
pub const PA_SEEK_RELATIVE_ON_READ: pa_seek_mode_t = 2;
pub const PA_SEEK_RELATIVE_END: pa_seek_mode_t = 3;
