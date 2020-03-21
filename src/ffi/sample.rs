
#[repr(C)]
pub struct pa_sample_spec
{
	pub format: pa_sample_format_t,
	pub rate: u32,
	pub channels: u8
}

pub type pa_sample_format_t = i32;
pub const PA_SAMPLE_U8: pa_sample_format_t = 0;
pub const PA_SAMPLE_ALAW: pa_sample_format_t = 1;
pub const PA_SAMPLE_ULAW: pa_sample_format_t = 2;
pub const PA_SAMPLE_S16LE: pa_sample_format_t = 3;
pub const PA_SAMPLE_S16BE: pa_sample_format_t = 4;
pub const PA_SAMPLE_FLOAT32LE: pa_sample_format_t = 5;
pub const PA_SAMPLE_FLOAT32BE: pa_sample_format_t = 6;
pub const PA_SAMPLE_S32LE: pa_sample_format_t = 7;
pub const PA_SAMPLE_S32BE: pa_sample_format_t = 8;
pub const PA_SAMPLE_S24LE: pa_sample_format_t = 9;
pub const PA_SAMPLE_S24BE: pa_sample_format_t = 10;
pub const PA_SAMPLE_S24_32LE: pa_sample_format_t = 11;
pub const PA_SAMPLE_S24_32BE: pa_sample_format_t = 12;
