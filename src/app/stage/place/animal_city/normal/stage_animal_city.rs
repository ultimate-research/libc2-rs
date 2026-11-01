#[repr(C)]
pub struct StageAnimalCity {
    _0: [u8; 0x9F0],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RandomSetting {
    /// The selected combination of residents to appear in the background of the town platform.
    pub village_guest_set: u8,

    /// The selected combination of residents to appear in the background of the city platform.
    pub city_guest_set: u8,

    /// The fixed hour of the day.
    pub hour: u8,

    /// The fixed minute within the hour.
    pub minute: u8,

    /// The fixed day of the week.
    pub day_of_week: u8,
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageAnimalCity>(), 0x9F0);
        assert_eq!(offset_of!(StageAnimalCity, _0), 0x0);
    }
}
