use crate::{cpp, phx::ParameterReceiverBase};

/// The specialized parameters for the Small Battlefield stage.
#[repr(C)]
pub struct StageData {
    pub base: ParameterReceiverBase,

    /// The number of frames representing the duration of the color correction sequence.
    pub color_correction_loop_length: i32,

    /// The collection of general parameters for each color correction key frame.
    pub color_correction_entries: cpp::Vector<ColorCorrectionEntry>,
}

/// The general parameters for a color correction key frame.
#[repr(C)]
pub struct ColorCorrectionEntry {
    /// The frame to start the fade-in of this key frame on.
    pub start_frame: i32,

    /// The number of frames representing the duration of the fade-in.
    pub fadein_frame: i32,

    /// The frame to start the fade-out of this key frame on.
    pub end_frame: i32,

    /// The number of frames representing the duration of the fade-out.
    pub fadeout_frame: i32,

    /// The name of the color grading lookup table resource to fade into.
    pub lut_name: cpp::String,

    /// Determines if this key frame should apply to the stage layer.
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
