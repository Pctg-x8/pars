
pub type pa_channel_position_t = i32;
pub const PA_CHANNEL_POSITION_INVALID: pa_channel_position_t = -1;
pub const PA_CHANNEL_POSITION_MONO: pa_channel_position_t = 0;

pub const PA_CHANNEL_POSITION_FRONT_LEFT: pa_channel_position_t = 1;
pub const PA_CHANNEL_POSITION_FRONT_RIGHT: pa_channel_position_t = 2;
pub const PA_CHANNEL_POSITION_FRONT_CENTER: pa_channel_position_t = 3;

pub const PA_CHANNEL_POSITION_LEFT: pa_channel_position_t = PA_CHANNEL_POSITION_FRONT_LEFT;
pub const PA_CHANNEL_POSITION_RIGHT: pa_channel_position_t = PA_CHANNEL_POSITION_FRONT_RIGHT;
pub const PA_CHANNEL_POSITION_CENTER: pa_channel_position_t = PA_CHANNEL_POSITION_FRONT_CENTER;

pub const PA_CHANNEL_POSITION_REAR_CENTER: pa_channel_position_t = 4;
pub const PA_CHANNEL_POSITION_REAR_LEFT: pa_channel_position_t = 5;
pub const PA_CHANNEL_POSITION_REAR_RIGHT: pa_channel_position_t = 6;

pub const PA_CHANNEL_POSITION_LFE: pa_channel_position_t = 7;
pub const PA_CHANNEL_POSITION_SUBWOOFER: pa_channel_position_t = PA_CHANNEL_POSITION_LFE;

pub const PA_CHANNEL_POSITION_FRONT_LEFT_OF_CENTER: pa_channel_position_t = 8;
pub const PA_CHANNEL_POSITION_FRONT_RIGHT_OF_CENTER: pa_channel_position_t = 9;

pub const PA_CHANNEL_POSITION_SIDE_LEFT: pa_channel_position_t = 10;
pub const PA_CHANNEL_POSITION_SIDE_RIGHT: pa_channel_position_t = 11;

/// AUX0-AUX31
pub const PA_CHANNEL_POSITION_AUX: pa_channel_position_t = 12;
pub const PA_CHANNEL_POSITION_TOP_CENTER: pa_channel_position_t = 44;

pub const PA_CHANNEL_POSITION_TOP_FRONT_LEFT: pa_channel_position_t = 45;
pub const PA_CHANNEL_POSITION_TOP_FRONT_RIGHT: pa_channel_position_t = 46;
pub const PA_CHANNEL_POSITION_TOP_FRONT_CENTER: pa_channel_position_t = 47;

pub const PA_CHANNEL_POSITION_TOP_REAR_LEFT: pa_channel_position_t = 48;
pub const PA_CHANNEL_POSITION_TOP_REAR_RIGHT: pa_channel_position_t = 49;
pub const PA_CHANNEL_POSITION_TOP_REAR_CENTER: pa_channel_position_t = 50;

pub const PA_CHANNELS_MAX: usize = 32;

#[repr(C)]
pub struct pa_channel_map
{
	pub channels: u8,
	pub map: [pa_channel_position_t; PA_CHANNELS_MAX]
}

#[link(name = "pulse")]
extern "C"
{
	pub fn pa_channel_map_init(m: *mut pa_channel_map) -> *mut pa_channel_map;
	pub fn pa_channel_map_init_mono(m: *mut pa_channel_map) -> *mut pa_channel_map;
	pub fn pa_channel_map_init_stereo(m: *mut pa_channel_map) -> *mut pa_channel_map;
}
