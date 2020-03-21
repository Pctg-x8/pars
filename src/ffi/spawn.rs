
#[repr(C)]
pub struct pa_spawn_api
{
	pub prefork: extern "C" fn(),
	pub postfork: extern "C" fn(),
	pub atfork: extern "C" fn()
}
