use crate::{app::StageBase, cpp, lib::Color4f};

/// An animation controller for layered stage and character color correction.
///
/// # Initialization
///
/// ```ignore (pseudocode)
/// use libc2::app::StageColorCorrectionController;
///
/// pub fn pre_setup(&mut self) {
///     self.color_correction_controller = StageColorCorrectionController::new(
///         &self.base,
///         self.stage_data.color_correction_loop_length,
///     );
///
///     for entry in &self.stage_data.color_correction_entries {
///         self.color_correction_controller.push(
///             entry.start_frame,
///             entry.fadein_frame,
///             entry.end_frame,
///             entry.fadeout_frame,
///             entry.lut_name.as_str(),
///             entry.is_ground_layer,
///             entry.gain_color_r,
///             entry.gain_color_g,
///             entry.gain_color_b,
///             entry.offset_color_r,
///             entry.offset_color_g,
///             entry.offset_color_b,
///             entry.saturation,
///             entry.contrast,
///         );
///     }
///
///     self.color_correction_controller.start();
/// }
/// ```
///
/// # Updating
///
/// ```ignore (pseudocode)
/// pub fn pre_update(&mut self, step_frame: f32) {
///     self.color_correction_controller.update(step_frame);
/// }
/// ```
#[repr(C)]
pub struct StageColorCorrectionController {
    pub stage_base: *const StageBase,
    pub color_grading_controller: cpp::UniquePtr<StageColorGradingController>,
    pub color_correction_work: cpp::UniquePtr<StageColorCorrectionWork>,
    pub color_correction_entries: cpp::Vector<cpp::UniquePtr<StageColorCorrectionEntry>>,
    pub loop_length: i32,
    pub frame: f32,
}

#[cfg(feature = "impl")]
impl StageColorCorrectionController {
    /// Creates a new `StageColorCorrectionController`.
    pub fn new(stage_base: *const StageBase, loop_length: i32) -> Self {
        let mut this = core::mem::MaybeUninit::<Self>::uninit();

        // SAFETY: The C++ constructor will initialize the object for us.
        unsafe {
            this.assume_init_mut().create(stage_base, loop_length);
            this.assume_init()
        }
    }

    /// Creates a new `StageColorCorrectionController` in place.
    fn create(&mut self, stage_base: *const StageBase, loop_length: i32) {
        #[skyline::from_offset(0x317CF10)]
        fn inner(
            this: *mut StageColorCorrectionController,
            stage_base: *const StageBase,
            loop_length: i32,
        );

        unsafe {
            inner(self, stage_base, loop_length);
        }
    }

    /// Updates the state of this `StageColorCorrectionController`.
    pub fn update(&mut self, step_frame: f32) {
        #[skyline::from_offset(0x317D3E0)]
        fn inner(this: *mut StageColorCorrectionController, step_frame: f32);

        unsafe {
            inner(self, step_frame);
        }
    }

    /// Updates the state of this `StageColorCorrectionController` without progressing the animation frame counter.
    pub fn start(&mut self) {
        #[skyline::from_offset(0x317D600)]
        fn inner(this: *mut StageColorCorrectionController);

        unsafe {
            inner(self);
        }
    }

    /// Appends a color correction keyframe to the back of the collection.
    #[allow(clippy::too_many_arguments)]
    pub fn push(
        &mut self,
        start_frame: i32,
        fadein_frame: i32,
        end_frame: i32,
        fadeout_frame: i32,
        lut_name: &str,
        is_ground_layer: bool,
        gain_color_r: f32,
        gain_color_g: f32,
        gain_color_b: f32,
        offset_color_r: f32,
        offset_color_g: f32,
        offset_color_b: f32,
        saturation: f32,
        contrast: f32,
    ) {
        #[skyline::from_offset(0x317D810)]
        fn inner(
            this: *mut StageColorCorrectionController,
            start_frame: i32,
            fadein_frame: i32,
            end_frame: i32,
            fadeout_frame: i32,
            lut_name: *const core::ffi::c_char,
            lut_name_size: usize,
            is_ground_layer: bool,
            gain_color_r: f32,
            gain_color_g: f32,
            gain_color_b: f32,
            offset_color_r: f32,
            offset_color_g: f32,
            offset_color_b: f32,
            saturation: f32,
            contrast: f32,
        );

        unsafe {
            inner(
                self,
                start_frame,
                fadein_frame,
                end_frame,
                fadeout_frame,
                lut_name.as_ptr().cast::<core::ffi::c_char>(),
                lut_name.len(),
                is_ground_layer,
                gain_color_r,
                gain_color_g,
                gain_color_b,
                offset_color_r,
                offset_color_g,
                offset_color_b,
                saturation,
                contrast,
            );
        }
    }
}

#[cfg(feature = "impl")]
impl Drop for StageColorCorrectionController {
    fn drop(&mut self) {
        #[skyline::from_offset(0x317D300)]
        fn inner(this: *mut StageColorCorrectionController);

        unsafe {
            inner(self);
        }
    }
}

#[repr(C)]
pub struct StageColorGradingController {
    pub lut_prev_name: cpp::String,
    pub lut_next_name: cpp::String,
    pub intensity: f32,
    pub status: StageColorGradingControllerStatus,
}

#[repr(C)]
pub enum StageColorGradingControllerStatus {
    Init,
    Fadein,
    Wait,
    Fadeout,
}

#[repr(C)]
pub struct StageColorCorrectionWork {
    /// The working color correction for the stage layer.
    pub ground: StageColorCorrectionLayerColor,

    /// The working color correction for the character layer.
    pub chara: StageColorCorrectionLayerColor,
}

#[repr(C)]
pub struct StageColorCorrectionLayerColor {
    /// The scale of the color.
    pub gain_color: Color4f,

    /// The displacement of the color.
    pub offset_color: Color4f,

    /// The scale of the color saturation.
    pub saturation: f32,

    /// The scale of the color contrast.
    pub contrast: f32,
}

#[repr(C)]
pub struct StageColorCorrectionEntry {
    pub stage_base: *const StageBase,
    pub fadein_length: i32,
    pub fadein_start_frame: i32,
    pub fadein_end_frame: i32,
    pub fadeout_length: i32,
    pub fadeout_start_frame: i32,
    pub fadeout_end_frame: i32,
    pub lut_name: cpp::String,
    pub layer: StageColorCorrectionLayer,
    pub layer_color: StageColorCorrectionLayerColor,
    pub _70: i32,
}

#[repr(C)]
pub enum StageColorCorrectionLayer {
    Ground,
    Chara,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageColorCorrectionController>(), 0x38);
        assert_eq!(offset_of!(StageColorCorrectionController, stage_base), 0x0);
        assert_eq!(offset_of!(StageColorCorrectionController, color_grading_controller), 0x8);
        assert_eq!(offset_of!(StageColorCorrectionController, color_correction_work), 0x10);
        assert_eq!(offset_of!(StageColorCorrectionController, color_correction_entries), 0x18);
        assert_eq!(offset_of!(StageColorCorrectionController, loop_length), 0x30);
        assert_eq!(offset_of!(StageColorCorrectionController, frame), 0x34);

        assert_eq!(size_of::<StageColorGradingController>(), 0x38);
        assert_eq!(offset_of!(StageColorGradingController, lut_prev_name), 0x0);
        assert_eq!(offset_of!(StageColorGradingController, lut_next_name), 0x18);
        assert_eq!(offset_of!(StageColorGradingController, intensity), 0x30);
        assert_eq!(offset_of!(StageColorGradingController, status), 0x34);

        assert_eq!(size_of::<StageColorCorrectionWork>(), 0x60);
        assert_eq!(offset_of!(StageColorCorrectionWork, ground), 0x0);
        assert_eq!(offset_of!(StageColorCorrectionWork, chara), 0x30);

        assert_eq!(size_of::<StageColorCorrectionLayerColor>(), 0x30);
        assert_eq!(offset_of!(StageColorCorrectionLayerColor, gain_color), 0x0);
        assert_eq!(offset_of!(StageColorCorrectionLayerColor, offset_color), 0x10);
        assert_eq!(offset_of!(StageColorCorrectionLayerColor, saturation), 0x20);
        assert_eq!(offset_of!(StageColorCorrectionLayerColor, contrast), 0x24);

        assert_eq!(size_of::<StageColorCorrectionEntry>(), 0x80);
        assert_eq!(offset_of!(StageColorCorrectionEntry, stage_base), 0x0);
        assert_eq!(offset_of!(StageColorCorrectionEntry, fadein_length), 0x8);
        assert_eq!(offset_of!(StageColorCorrectionEntry, fadein_start_frame), 0xC);
        assert_eq!(offset_of!(StageColorCorrectionEntry, fadein_end_frame), 0x10);
        assert_eq!(offset_of!(StageColorCorrectionEntry, fadeout_length), 0x14);
        assert_eq!(offset_of!(StageColorCorrectionEntry, fadeout_start_frame), 0x18);
        assert_eq!(offset_of!(StageColorCorrectionEntry, fadeout_end_frame), 0x1C);
        assert_eq!(offset_of!(StageColorCorrectionEntry, lut_name), 0x20);
        assert_eq!(offset_of!(StageColorCorrectionEntry, layer), 0x38);
        assert_eq!(offset_of!(StageColorCorrectionEntry, layer_color), 0x40);
        assert_eq!(offset_of!(StageColorCorrectionEntry, _70), 0x70);

        assert_eq!(size_of::<StageColorCorrectionLayer>(), 0x4);
        assert_eq!(StageColorCorrectionLayer::Ground as i32, 0x0);
        assert_eq!(StageColorCorrectionLayer::Chara as i32, 0x1);
    }
}
