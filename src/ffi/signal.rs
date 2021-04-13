
use libc::*;
use super::pa_mainloop_api;

#[link(name = "pulse")]
extern "C"
{
    pub fn pa_signal_init(api: *mut pa_mainloop_api) -> c_int;
}