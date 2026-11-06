/// A decrementing timer, measured in frames.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct StageSimpleTimer {
    /// The duration of the timer, in frames.
    pub length: f32,

    /// The remaining amount of time, in frames.
    pub frame: f32,

    /// Determines if the timer can be decremented.
    pub is_update: bool,

    /// Determines if the timer just reached zero.
    pub is_end: bool,
}

impl StageSimpleTimer {
    /// Creates a new `StageSimpleTimer`.
    pub const fn new() -> Self {
        Self {
            length: 0.0,
            frame: 0.0,
            is_update: false,
            is_end: false,
        }
    }

    /// Initializes the timer to the given duration.
    pub const fn reset(&mut self, frame: f32) {
        self.length = frame;
        self.frame = frame;
        self.is_update = true;
        self.is_end = false;
    }

    /// Updates the state of this `StageSimpleTimer`.
    pub const fn update(&mut self, step_frame: f32) {
        if self.is_update {
            self.frame -= step_frame;

            if self.frame < 0.0 {
                self.frame = 0.0;
                self.is_update = false;
                self.is_end = true;
            }
        } else if self.is_end {
            self.is_end = false;
        }
    }

    /// Returns a reference to the duration, in frames.
    pub const fn length(&self) -> &f32 {
        &self.length
    }

    /// Returns a reference to the remaining amount of time, in frames.
    pub const fn frame(&self) -> &f32 {
        &self.frame
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageSimpleTimer>(), 0xC);
        assert_eq!(offset_of!(StageSimpleTimer, length), 0x0);
        assert_eq!(offset_of!(StageSimpleTimer, frame), 0x4);
        assert_eq!(offset_of!(StageSimpleTimer, is_update), 0x8);
        assert_eq!(offset_of!(StageSimpleTimer, is_end), 0x9);
    }
}
