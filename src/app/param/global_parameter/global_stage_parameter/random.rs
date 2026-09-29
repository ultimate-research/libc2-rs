use crate::app::stage::{
    animal_city, animal_island, balloonfight, dolly_stadium, end, mario_maker, nintendogs,
    pickel_world, punchout, sp_edit, trail_castle, xeno_alst,
};

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct StageRandomSetting {
    pub end: end::RandomSetting,
    pub punchout: punchout::RandomSetting,
    pub mario_maker: mario_maker::RandomSetting,
    pub balloonfight: balloonfight::RandomSetting,
    pub nintendogs: nintendogs::RandomSetting,
    pub animal_island: animal_island::RandomSetting,
    pub animal_city: animal_city::RandomSetting,
    pub sp_edit: sp_edit::RandomSetting,
    pub dolly_stadium: dolly_stadium::RandomSetting,
    pub pickel_world: pickel_world::RandomSetting,
    pub xeno_alst: xeno_alst::RandomSetting,
    pub trail_castle: trail_castle::RandomSetting,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageRandomSetting>(), 0x28);
        assert_eq!(offset_of!(StageRandomSetting, end), 0x0);
        assert_eq!(offset_of!(StageRandomSetting, punchout), 0x2);
        assert_eq!(offset_of!(StageRandomSetting, mario_maker), 0x4);
        assert_eq!(offset_of!(StageRandomSetting, balloonfight), 0xC);
        assert_eq!(offset_of!(StageRandomSetting, nintendogs), 0xD);
        assert_eq!(offset_of!(StageRandomSetting, animal_island), 0x11);
        assert_eq!(offset_of!(StageRandomSetting, animal_city), 0x13);
        assert_eq!(offset_of!(StageRandomSetting, sp_edit), 0x18);
        assert_eq!(offset_of!(StageRandomSetting, dolly_stadium), 0x19);
        assert_eq!(offset_of!(StageRandomSetting, pickel_world), 0x21);
        assert_eq!(offset_of!(StageRandomSetting, xeno_alst), 0x22);
        assert_eq!(offset_of!(StageRandomSetting, trail_castle), 0x24);
    }
}
