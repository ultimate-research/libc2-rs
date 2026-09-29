#[cfg(feature = "impl")]
use skyline::hooks::{Region, getRegionAddress};

mod global_common_parameter;
mod global_melee_parameter;
mod global_stage_parameter;

pub use global_common_parameter::*;
pub use global_melee_parameter::*;
pub use global_stage_parameter::*;

#[repr(C)]
pub struct GlobalParameter {
    common: GlobalCommonParameter,
    melee: GlobalMeleeParameter,
    stage: [GlobalStageParameter; 2],
    _5b0: [u8; 0x4EDA8],
}

#[cfg(feature = "impl")]
impl GlobalParameter {
    /// Returns a reference to the current instance of `GlobalParameter`.
    pub fn instance() -> &'static Self {
        unsafe {
            &*getRegionAddress(Region::Text)
                .byte_offset(0x52C5180)
                .cast::<Self>()
        }
    }

    pub fn global_common_parameter() -> &'static GlobalCommonParameter {
        &Self::instance().common
    }

    pub fn global_melee_parameter() -> &'static GlobalMeleeParameter {
        &Self::instance().melee
    }

    pub fn global_stage_parameter() -> &'static [GlobalStageParameter; 2] {
        &Self::instance().stage
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<GlobalParameter>(), 0x4F358);
        assert_eq!(offset_of!(GlobalParameter, common), 0x0);
        assert_eq!(offset_of!(GlobalParameter, melee), 0x30);
        assert_eq!(offset_of!(GlobalParameter, stage), 0x450);
    }
}
