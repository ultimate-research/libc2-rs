#[repr(C)]
pub struct StageMarioMaker {
    _0: [u8; 0x9F0],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RandomSetting {
    /// Determines if the selected formation should represent solid ground
    /// instead of a floating platform.
    pub is_ground_type: bool,

    /// The collection of indices into `island_type_settings` or `ground_type_settings`
    /// for generating the left, middle, and right sides of the stage, respectively.
    pub type_settings: [i8; 3],

    /// The seed for a dedicated pseudorandom number generator used to select
    /// other random elements, such as the initial game style, the layout of
    /// floating platforms, and the design of the hand.
    pub seed: u32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageMarioMaker>(), 0x9F0);
        assert_eq!(offset_of!(StageMarioMaker, _0), 0x0);
    }
}
