use crate::{app::grcol::AttrType, phx::Hash40};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct StageGroundAttrParamSet {
    pub material_type: AttrType,
    pub friction: f32,
    pub damage_id: u32,
    pub attr_param: [StageGroundAttrParam; Self::ATTR_PARAM_NUM],
}

impl StageGroundAttrParamSet {
    const ATTR_PARAM_NUM: usize = 3;
}

impl Default for StageGroundAttrParamSet {
    fn default() -> Self {
        Self {
            material_type: AttrType::None,
            friction: 1.0,
            damage_id: 0,
            attr_param: [StageGroundAttrParam::default(); Self::ATTR_PARAM_NUM],
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct StageGroundAttrParam {
    pub overlap_effect: bool,
    pub overlap_se: bool,
    pub effect: Hash40,
    pub se_l: Hash40,
    pub se_m: Hash40,
    pub se_s: Hash40,
    pub se_ss: Hash40,
}

impl Default for StageGroundAttrParam {
    fn default() -> Self {
        Self {
            overlap_effect: false,
            overlap_se: false,
            effect: Hash40::new("ef_null"),
            se_l: Hash40::new("invalid"),
            se_m: Hash40::new("invalid"),
            se_s: Hash40::new("invalid"),
            se_ss: Hash40::new("invalid"),
        }
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageGroundAttrParamSet>(), 0xA0);
        assert_eq!(offset_of!(StageGroundAttrParamSet, material_type), 0x0);
        assert_eq!(offset_of!(StageGroundAttrParamSet, friction), 0x4);
        assert_eq!(offset_of!(StageGroundAttrParamSet, damage_id), 0x8);
        assert_eq!(offset_of!(StageGroundAttrParamSet, attr_param), 0x10);

        assert_eq!(size_of::<StageGroundAttrParam>(), 0x30);
        assert_eq!(offset_of!(StageGroundAttrParam, overlap_effect), 0x0);
        assert_eq!(offset_of!(StageGroundAttrParam, overlap_se), 0x1);
        assert_eq!(offset_of!(StageGroundAttrParam, effect), 0x8);
        assert_eq!(offset_of!(StageGroundAttrParam, se_l), 0x10);
        assert_eq!(offset_of!(StageGroundAttrParam, se_m), 0x18);
        assert_eq!(offset_of!(StageGroundAttrParam, se_s), 0x20);
        assert_eq!(offset_of!(StageGroundAttrParam, se_ss), 0x28);
    }
}
