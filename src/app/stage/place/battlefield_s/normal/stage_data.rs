use crate::{cpp, phx::ParameterReceiverBase};

/// The specialized parameters for the Small Battlefield stage.
#[repr(C)]
pub struct StageData {
    pub base: ParameterReceiverBase,

    /// The duration of the color correction animation, in frames.
    pub color_correction_loop_length: i32,

    /// The collection of general parameters for each color correction keyframe.
    pub color_correction_entries: cpp::Vector<ColorCorrectionEntry>,
}

/// The general parameters for a color correction keyframe.
#[repr(C)]
pub struct ColorCorrectionEntry {
    /// The animation frame index to start interpolation toward the target color on.
    pub start_frame: i32,

    /// The amount of time to interpolate toward the target color, in frames.
    pub fadein_frame: i32,

    /// The animation frame index to stop interpolation away from the target color on.
    pub end_frame: i32,

    /// The amount of time to interpolate away from the target color, in frames.
    pub fadeout_frame: i32,

    /// The name of the color lookup table resource to interpolate toward.
    pub lut_name: cpp::String,

    /// Determines if this keyframe should apply to the stage instead of the characters.
    pub is_ground_layer: bool,

    /// The red component of the color scale.
    pub gain_color_r: f32,

    /// The green component of the color scale.
    pub gain_color_g: f32,

    /// The blue component of the color scale.
    pub gain_color_b: f32,

    /// The red component of the color offset.
    pub offset_color_r: f32,

    /// The green component of the color offset.
    pub offset_color_g: f32,

    /// The blue component of the color offset.
    pub offset_color_b: f32,

    /// The color saturation scale.
    pub saturation: f32,

    /// The color contrast scale.
    pub contrast: f32,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageData>(), 0x28);
        assert_eq!(offset_of!(StageData, base), 0x0);
        assert_eq!(offset_of!(StageData, color_correction_loop_length), 0x8);
        assert_eq!(offset_of!(StageData, color_correction_entries), 0x10);

        assert_eq!(size_of::<ColorCorrectionEntry>(), 0x50);
        assert_eq!(offset_of!(ColorCorrectionEntry, start_frame), 0x0);
        assert_eq!(offset_of!(ColorCorrectionEntry, fadein_frame), 0x4);
        assert_eq!(offset_of!(ColorCorrectionEntry, end_frame), 0x8);
        assert_eq!(offset_of!(ColorCorrectionEntry, fadeout_frame), 0xC);
        assert_eq!(offset_of!(ColorCorrectionEntry, lut_name), 0x10);
        assert_eq!(offset_of!(ColorCorrectionEntry, is_ground_layer), 0x28);
        assert_eq!(offset_of!(ColorCorrectionEntry, gain_color_r), 0x2C);
        assert_eq!(offset_of!(ColorCorrectionEntry, gain_color_g), 0x30);
        assert_eq!(offset_of!(ColorCorrectionEntry, gain_color_b), 0x34);
        assert_eq!(offset_of!(ColorCorrectionEntry, offset_color_r), 0x38);
        assert_eq!(offset_of!(ColorCorrectionEntry, offset_color_g), 0x3C);
        assert_eq!(offset_of!(ColorCorrectionEntry, offset_color_b), 0x40);
        assert_eq!(offset_of!(ColorCorrectionEntry, saturation), 0x44);
        assert_eq!(offset_of!(ColorCorrectionEntry, contrast), 0x48);
    }
}
