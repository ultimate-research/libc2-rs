use crate::phx::ParameterReceiverBase;

/// The specialized parameters for the Hollow Bastion stage.
#[repr(C)]
pub struct StageData {
    pub base: ParameterReceiverBase,
    pub climax_delay_seconds: f32,
    pub climax_rule_time_threshold_seconds: f32,
    pub climax_rule_refill_threshold_hp: f32,
    pub climax_rule_stock_coef: f32,
    pub climax_rule_stock_upper_stock: u32,
    pub climax_rule_stock_thresholds2_0: u32,
    pub climax_rule_stock_thresholds2_1: u32,
    pub climax_rule_stock_thresholds2_2: u32,
    pub climax_rule_stock_thresholds3_0: u32,
    pub climax_rule_stock_thresholds3_1: u32,
    pub climax_rule_hp_coef: f32,
    pub climax_rule_hp_upper_stock: u32,
    pub climax_rule_hp_thresholds2_0: u32,
    pub climax_rule_hp_thresholds2_1: u32,
    pub climax_rule_hp_thresholds2_2: u32,
    pub climax_rule_hp_thresholds2_3: u32,
    pub climax_rule_hp_thresholds3_0: u32,
    pub climax_rule_hp_thresholds3_1: u32,
    pub climax_rule_hp_thresholds3_2: u32,
    pub climax_rule_team_stock_coef: f32,
    pub climax_rule_team_stock_upper_stock: u32,
    pub climax_rule_team_stock_thresholds2_0: u32,
    pub climax_rule_team_stock_thresholds2_1: u32,
    pub climax_rule_team_stock_thresholds2_2: u32,
    pub climax_rule_team_stock_thresholds3_0: u32,
    pub climax_rule_team_stock_thresholds3_1: u32,
    pub climax_rule_team_hp_coef: f32,
    pub climax_rule_team_hp_upper_stock: u32,
    pub climax_rule_team_hp_thresholds2_0: u32,
    pub climax_rule_team_hp_thresholds2_1: u32,
    pub climax_rule_team_hp_thresholds2_2: u32,
    pub climax_rule_team_hp_thresholds2_3: u32,
    pub climax_rule_team_hp_thresholds3_0: u32,
    pub climax_rule_team_hp_thresholds3_1: u32,
    pub climax_rule_team_hp_thresholds3_2: u32,
    pub stained_glass_se_timing: f32,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageData>(), 0x98);
        assert_eq!(offset_of!(StageData, base), 0x0);
        assert_eq!(offset_of!(StageData, climax_delay_seconds), 0x8);
        assert_eq!(offset_of!(StageData, climax_rule_time_threshold_seconds), 0xC);
        assert_eq!(offset_of!(StageData, climax_rule_refill_threshold_hp), 0x10);
        assert_eq!(offset_of!(StageData, climax_rule_stock_coef), 0x14);
        assert_eq!(offset_of!(StageData, climax_rule_stock_upper_stock), 0x18);
        assert_eq!(offset_of!(StageData, climax_rule_stock_thresholds2_0), 0x1C);
        assert_eq!(offset_of!(StageData, climax_rule_stock_thresholds2_1), 0x20);
        assert_eq!(offset_of!(StageData, climax_rule_stock_thresholds2_2), 0x24);
        assert_eq!(offset_of!(StageData, climax_rule_stock_thresholds3_0), 0x28);
        assert_eq!(offset_of!(StageData, climax_rule_stock_thresholds3_1), 0x2C);
        assert_eq!(offset_of!(StageData, climax_rule_hp_coef), 0x30);
        assert_eq!(offset_of!(StageData, climax_rule_hp_upper_stock), 0x34);
        assert_eq!(offset_of!(StageData, climax_rule_hp_thresholds2_0), 0x38);
        assert_eq!(offset_of!(StageData, climax_rule_hp_thresholds2_1), 0x3C);
        assert_eq!(offset_of!(StageData, climax_rule_hp_thresholds2_2), 0x40);
        assert_eq!(offset_of!(StageData, climax_rule_hp_thresholds2_3), 0x44);
        assert_eq!(offset_of!(StageData, climax_rule_hp_thresholds3_0), 0x48);
        assert_eq!(offset_of!(StageData, climax_rule_hp_thresholds3_1), 0x4C);
        assert_eq!(offset_of!(StageData, climax_rule_hp_thresholds3_2), 0x50);
        assert_eq!(offset_of!(StageData, climax_rule_team_stock_coef), 0x54);
        assert_eq!(offset_of!(StageData, climax_rule_team_stock_upper_stock), 0x58);
        assert_eq!(offset_of!(StageData, climax_rule_team_stock_thresholds2_0), 0x5C);
        assert_eq!(offset_of!(StageData, climax_rule_team_stock_thresholds2_1), 0x60);
        assert_eq!(offset_of!(StageData, climax_rule_team_stock_thresholds2_2), 0x64);
        assert_eq!(offset_of!(StageData, climax_rule_team_stock_thresholds3_0), 0x68);
        assert_eq!(offset_of!(StageData, climax_rule_team_stock_thresholds3_1), 0x6C);
        assert_eq!(offset_of!(StageData, climax_rule_team_hp_coef), 0x70);
        assert_eq!(offset_of!(StageData, climax_rule_team_hp_upper_stock), 0x74);
        assert_eq!(offset_of!(StageData, climax_rule_team_hp_thresholds2_0), 0x78);
        assert_eq!(offset_of!(StageData, climax_rule_team_hp_thresholds2_1), 0x7C);
        assert_eq!(offset_of!(StageData, climax_rule_team_hp_thresholds2_2), 0x80);
        assert_eq!(offset_of!(StageData, climax_rule_team_hp_thresholds2_3), 0x84);
        assert_eq!(offset_of!(StageData, climax_rule_team_hp_thresholds3_0), 0x88);
        assert_eq!(offset_of!(StageData, climax_rule_team_hp_thresholds3_1), 0x8C);
        assert_eq!(offset_of!(StageData, climax_rule_team_hp_thresholds3_2), 0x90);
        assert_eq!(offset_of!(StageData, stained_glass_se_timing), 0x94);
    }
}
