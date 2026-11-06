use crate::app::stage::{end, pickel_world, poke_kalos, trail_castle};

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct GlobalStageExSetting {
    pub common_end: i8,
    pub animal_city: i8,
    pub animal_island: i8,
    pub animal_village: i8,
    pub end: end::ExSetting,
    pub fe_siege: i8,
    pub flatzonex: i8,
    pub icarus_skyworld: i8,
    pub icarus_uprising: i8,
    pub kirby_gameboy: i8,
    pub kirby_halberd: i8,
    pub mario_castledx: i8,
    pub mario_dolpic: i8,
    pub mario_maker: i8,
    pub mario_paper: i8,
    pub mario_uworld: i8,
    pub mg_shadowmoses: i8,
    pub pikmin_planet: i8,
    pub poke_stadium: i8,
    pub poke_stadium2: i8,
    pub poke_kalos: poke_kalos::ExSetting,
    pub poke_tengam: i8,
    pub poke_unova: i8,
    pub rock_wily: i8,
    pub wario_gamer: i8,
    pub wario_madein: i8,
    pub wufuisland: i8,
    pub xeno_gaur: i8,
    pub yoshi_island: i8,
    pub zelda_hyrule: i8,
    pub bossstage_final2: i8,
    pub jack_mementoes: i8,
    pub brave_altar: i8,
    pub buddy_spiral: i8,
    pub dolly_stadium: i8,
    pub fe_shrine: i8,
    pub tantan_spring: i8,
    pub poke_tower: i8,
    pub pickel_world: pickel_world::ExSetting,
    pub demon_dojo: i8,
    pub trail_castle: trail_castle::ExSetting,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<GlobalStageExSetting>(), 0x30);
        assert_eq!(offset_of!(GlobalStageExSetting, common_end), 0x0);
        assert_eq!(offset_of!(GlobalStageExSetting, animal_city), 0x1);
        assert_eq!(offset_of!(GlobalStageExSetting, animal_island), 0x2);
        assert_eq!(offset_of!(GlobalStageExSetting, animal_village), 0x3);
        assert_eq!(offset_of!(GlobalStageExSetting, end), 0x4);
        assert_eq!(offset_of!(GlobalStageExSetting, fe_siege), 0x7);
        assert_eq!(offset_of!(GlobalStageExSetting, flatzonex), 0x8);
        assert_eq!(offset_of!(GlobalStageExSetting, icarus_skyworld), 0x9);
        assert_eq!(offset_of!(GlobalStageExSetting, icarus_uprising), 0xA);
        assert_eq!(offset_of!(GlobalStageExSetting, kirby_gameboy), 0xB);
        assert_eq!(offset_of!(GlobalStageExSetting, kirby_halberd), 0xC);
        assert_eq!(offset_of!(GlobalStageExSetting, mario_castledx), 0xD);
        assert_eq!(offset_of!(GlobalStageExSetting, mario_dolpic), 0xE);
        assert_eq!(offset_of!(GlobalStageExSetting, mario_maker), 0xF);
        assert_eq!(offset_of!(GlobalStageExSetting, mario_paper), 0x10);
        assert_eq!(offset_of!(GlobalStageExSetting, mario_uworld), 0x11);
        assert_eq!(offset_of!(GlobalStageExSetting, mg_shadowmoses), 0x12);
        assert_eq!(offset_of!(GlobalStageExSetting, pikmin_planet), 0x13);
        assert_eq!(offset_of!(GlobalStageExSetting, poke_stadium), 0x14);
        assert_eq!(offset_of!(GlobalStageExSetting, poke_stadium2), 0x15);
        assert_eq!(offset_of!(GlobalStageExSetting, poke_kalos), 0x16);
        assert_eq!(offset_of!(GlobalStageExSetting, poke_tengam), 0x18);
        assert_eq!(offset_of!(GlobalStageExSetting, poke_unova), 0x19);
        assert_eq!(offset_of!(GlobalStageExSetting, rock_wily), 0x1A);
        assert_eq!(offset_of!(GlobalStageExSetting, wario_gamer), 0x1B);
        assert_eq!(offset_of!(GlobalStageExSetting, wario_madein), 0x1C);
        assert_eq!(offset_of!(GlobalStageExSetting, wufuisland), 0x1D);
        assert_eq!(offset_of!(GlobalStageExSetting, xeno_gaur), 0x1E);
        assert_eq!(offset_of!(GlobalStageExSetting, yoshi_island), 0x1F);
        assert_eq!(offset_of!(GlobalStageExSetting, zelda_hyrule), 0x20);
        assert_eq!(offset_of!(GlobalStageExSetting, bossstage_final2), 0x21);
        assert_eq!(offset_of!(GlobalStageExSetting, jack_mementoes), 0x22);
        assert_eq!(offset_of!(GlobalStageExSetting, brave_altar), 0x23);
        assert_eq!(offset_of!(GlobalStageExSetting, buddy_spiral), 0x24);
        assert_eq!(offset_of!(GlobalStageExSetting, dolly_stadium), 0x25);
        assert_eq!(offset_of!(GlobalStageExSetting, fe_shrine), 0x26);
        assert_eq!(offset_of!(GlobalStageExSetting, tantan_spring), 0x27);
        assert_eq!(offset_of!(GlobalStageExSetting, poke_tower), 0x28);
        assert_eq!(offset_of!(GlobalStageExSetting, pickel_world), 0x29);
        assert_eq!(offset_of!(GlobalStageExSetting, demon_dojo), 0x2D);
        assert_eq!(offset_of!(GlobalStageExSetting, trail_castle), 0x2E);
    }
}
