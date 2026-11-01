use crate::{cpp, phx::ParameterReceiverBase};

/// The specialized parameters for the stage where the credits take place.
#[repr(C)]
pub struct StageData {
    pub base: ParameterReceiverBase,

    /// The collection of general parameters for each stage color correction keyframe.
    pub key_frames: cpp::Vector<KeyFrame>,

    /// The duration of the stage color correction animation, in frames.
    pub end_frame: i32,
}

/// The general parameters for a stage color correction keyframe.
#[repr(C)]
pub struct KeyFrame {
    /// The animation frame index to stop interpolation toward the target color on.
    pub start_frame: i32,

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
        assert_eq!(offset_of!(StageData, key_frames), 0x8);
        assert_eq!(offset_of!(StageData, end_frame), 0x20);

        assert_eq!(size_of::<KeyFrame>(), 0x24);
        assert_eq!(offset_of!(KeyFrame, start_frame), 0x0);
        assert_eq!(offset_of!(KeyFrame, gain_color_r), 0x4);
        assert_eq!(offset_of!(KeyFrame, gain_color_g), 0x8);
        assert_eq!(offset_of!(KeyFrame, gain_color_b), 0xC);
        assert_eq!(offset_of!(KeyFrame, offset_color_r), 0x10);
        assert_eq!(offset_of!(KeyFrame, offset_color_g), 0x14);
        assert_eq!(offset_of!(KeyFrame, offset_color_b), 0x18);
        assert_eq!(offset_of!(KeyFrame, saturation), 0x1C);
        assert_eq!(offset_of!(KeyFrame, contrast), 0x20);
    }
}
