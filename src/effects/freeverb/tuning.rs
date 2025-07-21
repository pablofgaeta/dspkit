// Freeverb model tuning from: https://github.com/sinshu/freeverb/blob/main/Components/tuning.h

// Fixed gain to scale the input to the comb filters when not in freeze mode
// This also helps keep the output of the reverb from exceeding unity gain
pub const FIXED_GAIN: f32 = 0.03;

// 0.0 = small room, to 1.0 = large room; maps to 0.7 to 0.98
pub const SCALE_ROOM: f32 = 0.28;
pub const OFFSET_ROOM: f32 = 0.7;
pub const INITIAL_ROOM: f32 = 0.5;

// 0.0 = no damping, 1.0 = full damping; maps to 0.0 to 0.4
pub const INITIAL_DAMP: f32 = 0.5;
pub const SCALE_DAMP: f32 = 0.4;

// 0.0 = no wet, 1.0 = full wet; maps to 0.0 to 3.0
pub const INITIAL_WET: f32 = 0.5;
pub const SCALE_WET: f32 = 3.0;

// 0.0 = no dry, 1.0 = full dry; maps to 0.0 to 2.0
pub const INITIAL_DRY: f32 = 0.0;
pub const SCALE_DRY: f32 = 2.0;

pub const INITIAL_WIDTH: f32 = 0.0; // 0.0 = mono, 1.0 = stereo

// Spread between left and right delay times for stereo effect
pub const STEREO_SPREAD_SEC: f32 = 0.000_521_541_9; // 23 samples (at 44.1kHz)

pub const NUM_COMBS: usize = 8;
pub const COMB_SECOND_TUNINGS: [f32; NUM_COMBS] = [
    0.025_306_122, // 1116 samples at 44.1kHz
    0.026_938_775, // 1188 samples at 44.1kHz
    0.028_956_916, // 1277 samples at 44.1kHz
    0.030_748_299, // 1356 samples at 44.1kHz
    0.032_244_9,   // 1422 samples at 44.1kHz
    0.033_809_524, // 1491 samples at 44.1kHz
    0.035_306_122, // 1557 samples at 44.1kHz
    0.036_666_666, // 1617 samples at 44.1kHz
];

pub const NUM_ALLPASS: usize = 4;
pub const ALLPASS_SECOND_TUNINGS: [f32; NUM_ALLPASS] = [
    0.012_607_709_7, // 556 samples at 44.1kHz
    0.01,            // 441 samples at 44.1kHz
    0.007_732_426_3, // 341 samples at 44.1kHz
    0.005_102_040_8, // 225 samples at 44.1kHz
];
