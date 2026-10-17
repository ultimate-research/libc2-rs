use crate::{
    cpp,
    phx::{Hash40, ParameterReceiverBase},
};

#[repr(C)]
pub struct StageDataVisual {
    pub base: ParameterReceiverBase,
    pub diving_period: f32,
    pub fading_period: f32,
    pub lightset_shift_period: f32,
    pub smoke_fading_period: f32,
    pub pipe_fading_delay: f32,
    pub pipe_fading_period: f32,
    pub model_basic_settings: cpp::Vector<ModelBasicSettings>,
    pub cruise_visibility_settings: cpp::Vector<CruiseVisibilitySettings>,
    pub cruise_animation_settings: cpp::Vector<CruiseAnimationSettings>,
    pub cruise_effect_settings: cpp::Vector<CruiseEffectSettings>,
    pub diving_visibility_settings: cpp::Vector<DivingVisibilitySettings>,
    pub diving_animation_settings: cpp::Vector<DivingAnimationSettings>,
    pub diving_effect_settings: cpp::Vector<DivingEffectSettings>,
    pub climax_visibility_settings: cpp::Vector<ClimaxVisibilitySettings>,
    pub climax_animation_settings: cpp::Vector<ClimaxAnimationSettings>,
    pub climax_effect_settings: cpp::Vector<ClimaxEffectSettings>,
    pub spirit_visibility_settings: cpp::Vector<SpiritVisibilitySettings>,
    pub spirit_animation_settings: cpp::Vector<SpiritAnimationSettings>,
    pub spirit_effect_settings: cpp::Vector<SpiritEffectSettings>,
}

#[repr(C)]
pub struct ModelBasicSettings {
    pub model_name: Hash40,
    pub stage_type: u8,
    pub is_hide_at_start: bool,
    pub is_apply_path: bool,
    pub is_blend_sh: bool,
    pub fade_type: u8,
    pub fade_factor: f32,
    pub fade_delay: f32,
    pub fade_r: f32,
    pub fade_g: f32,
    pub fade_b: f32,
}

#[repr(C)]
pub struct CruiseVisibilitySettings {
    pub model_name: Hash40,
    pub start_frame: f32,
    pub end_frame: f32,
}

#[repr(C)]
pub struct CruiseAnimationSettings {
    pub model_name: Hash40,
    pub anime_name: Hash40,
    pub start_frame: f32,
    pub is_loop: bool,
}

#[repr(C)]
pub struct CruiseEffectSettings {
    pub effect_name: Hash40,
    pub model_name: Hash40,
    pub joint_name: Hash40,
    pub scale: f32,
    pub start_frame: f32,
    pub end_frame: f32,
}

#[repr(C)]
pub struct DivingVisibilitySettings {
    pub scene: i32,
    pub model_name: Hash40,
    pub start_frame: f32,
    pub end_frame: f32,
}

#[repr(C)]
pub struct DivingAnimationSettings {
    pub scene: i32,
    pub model_name: Hash40,
    pub anime_name: Hash40,
    pub start_frame: f32,
    pub is_loop: bool,
}

#[repr(C)]
pub struct DivingEffectSettings {
    pub scene: i32,
    pub effect_name: Hash40,
    pub model_name: Hash40,
    pub joint_name: Hash40,
    pub scale: f32,
    pub start_frame: f32,
    pub end_frame: f32,
}

#[repr(C)]
pub struct ClimaxVisibilitySettings {
    pub scene: i32,
    pub model_name: Hash40,
    pub start_frame: f32,
    pub end_frame: f32,
}

#[repr(C)]
pub struct ClimaxAnimationSettings {
    pub scene: i32,
    pub model_name: Hash40,
    pub anime_name: Hash40,
    pub start_frame: f32,
    pub is_loop: bool,
}

#[repr(C)]
pub struct ClimaxEffectSettings {
    pub scene: i32,
    pub effect_name: Hash40,
    pub model_name: Hash40,
    pub joint_name: Hash40,
    pub scale: f32,
    pub start_frame: f32,
    pub end_frame: f32,
}

#[repr(C)]
pub struct SpiritVisibilitySettings {
    pub scene: i32,
    pub model_name: Hash40,
    pub start_frame: f32,
    pub end_frame: f32,
}

#[repr(C)]
pub struct SpiritAnimationSettings {
    pub scene: i32,
    pub model_name: Hash40,
    pub anime_name: Hash40,
    pub start_frame: f32,
    pub is_loop: bool,
}

#[repr(C)]
pub struct SpiritEffectSettings {
    pub scene: i32,
    pub effect_name: Hash40,
    pub model_name: Hash40,
    pub joint_name: Hash40,
    pub scale: f32,
    pub start_frame: f32,
    pub end_frame: f32,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageDataVisual>(), 0x158);
        assert_eq!(offset_of!(StageDataVisual, base), 0x0);
        assert_eq!(offset_of!(StageDataVisual, diving_period), 0x8);
        assert_eq!(offset_of!(StageDataVisual, fading_period), 0xC);
        assert_eq!(offset_of!(StageDataVisual, lightset_shift_period), 0x10);
        assert_eq!(offset_of!(StageDataVisual, smoke_fading_period), 0x14);
        assert_eq!(offset_of!(StageDataVisual, pipe_fading_delay), 0x18);
        assert_eq!(offset_of!(StageDataVisual, pipe_fading_period), 0x1C);
        assert_eq!(offset_of!(StageDataVisual, model_basic_settings), 0x20);
        assert_eq!(offset_of!(StageDataVisual, cruise_visibility_settings), 0x38);
        assert_eq!(offset_of!(StageDataVisual, cruise_animation_settings), 0x50);
        assert_eq!(offset_of!(StageDataVisual, cruise_effect_settings), 0x68);
        assert_eq!(offset_of!(StageDataVisual, diving_visibility_settings), 0x80);
        assert_eq!(offset_of!(StageDataVisual, diving_animation_settings), 0x98);
        assert_eq!(offset_of!(StageDataVisual, diving_effect_settings), 0xB0);
        assert_eq!(offset_of!(StageDataVisual, climax_visibility_settings), 0xC8);
        assert_eq!(offset_of!(StageDataVisual, climax_animation_settings), 0xE0);
        assert_eq!(offset_of!(StageDataVisual, climax_effect_settings), 0xF8);
        assert_eq!(offset_of!(StageDataVisual, spirit_visibility_settings), 0x110);
        assert_eq!(offset_of!(StageDataVisual, spirit_animation_settings), 0x128);
        assert_eq!(offset_of!(StageDataVisual, spirit_effect_settings), 0x140);

        assert_eq!(size_of::<ModelBasicSettings>(), 0x28);
        assert_eq!(offset_of!(ModelBasicSettings, model_name), 0x0);
        assert_eq!(offset_of!(ModelBasicSettings, stage_type), 0x8);
        assert_eq!(offset_of!(ModelBasicSettings, is_hide_at_start), 0x9);
        assert_eq!(offset_of!(ModelBasicSettings, is_apply_path), 0xA);
        assert_eq!(offset_of!(ModelBasicSettings, is_blend_sh), 0xB);
        assert_eq!(offset_of!(ModelBasicSettings, fade_type), 0xC);
        assert_eq!(offset_of!(ModelBasicSettings, fade_factor), 0x10);
        assert_eq!(offset_of!(ModelBasicSettings, fade_delay), 0x14);
        assert_eq!(offset_of!(ModelBasicSettings, fade_r), 0x18);
        assert_eq!(offset_of!(ModelBasicSettings, fade_g), 0x1C);
        assert_eq!(offset_of!(ModelBasicSettings, fade_b), 0x20);

        assert_eq!(size_of::<CruiseVisibilitySettings>(), 0x10);
        assert_eq!(offset_of!(CruiseVisibilitySettings, model_name), 0x0);
        assert_eq!(offset_of!(CruiseVisibilitySettings, start_frame), 0x8);
        assert_eq!(offset_of!(CruiseVisibilitySettings, end_frame), 0xC);

        assert_eq!(size_of::<CruiseAnimationSettings>(), 0x18);
        assert_eq!(offset_of!(CruiseAnimationSettings, model_name), 0x0);
        assert_eq!(offset_of!(CruiseAnimationSettings, anime_name), 0x8);
        assert_eq!(offset_of!(CruiseAnimationSettings, start_frame), 0x10);
        assert_eq!(offset_of!(CruiseAnimationSettings, is_loop), 0x14);

        assert_eq!(size_of::<CruiseEffectSettings>(), 0x28);
        assert_eq!(offset_of!(CruiseEffectSettings, effect_name), 0x0);
        assert_eq!(offset_of!(CruiseEffectSettings, model_name), 0x8);
        assert_eq!(offset_of!(CruiseEffectSettings, joint_name), 0x10);
        assert_eq!(offset_of!(CruiseEffectSettings, scale), 0x18);
        assert_eq!(offset_of!(CruiseEffectSettings, start_frame), 0x1C);
        assert_eq!(offset_of!(CruiseEffectSettings, end_frame), 0x20);

        assert_eq!(size_of::<DivingVisibilitySettings>(), 0x18);
        assert_eq!(offset_of!(DivingVisibilitySettings, scene), 0x0);
        assert_eq!(offset_of!(DivingVisibilitySettings, model_name), 0x8);
        assert_eq!(offset_of!(DivingVisibilitySettings, start_frame), 0x10);
        assert_eq!(offset_of!(DivingVisibilitySettings, end_frame), 0x14);

        assert_eq!(size_of::<DivingAnimationSettings>(), 0x20);
        assert_eq!(offset_of!(DivingAnimationSettings, scene), 0x0);
        assert_eq!(offset_of!(DivingAnimationSettings, model_name), 0x8);
        assert_eq!(offset_of!(DivingAnimationSettings, anime_name), 0x10);
        assert_eq!(offset_of!(DivingAnimationSettings, start_frame), 0x18);
        assert_eq!(offset_of!(DivingAnimationSettings, is_loop), 0x1C);

        assert_eq!(size_of::<DivingEffectSettings>(), 0x30);
        assert_eq!(offset_of!(DivingEffectSettings, scene), 0x0);
        assert_eq!(offset_of!(DivingEffectSettings, effect_name), 0x8);
        assert_eq!(offset_of!(DivingEffectSettings, model_name), 0x10);
        assert_eq!(offset_of!(DivingEffectSettings, joint_name), 0x18);
        assert_eq!(offset_of!(DivingEffectSettings, scale), 0x20);
        assert_eq!(offset_of!(DivingEffectSettings, start_frame), 0x24);
        assert_eq!(offset_of!(DivingEffectSettings, end_frame), 0x28);

        assert_eq!(size_of::<ClimaxVisibilitySettings>(), 0x18);
        assert_eq!(offset_of!(ClimaxVisibilitySettings, scene), 0x0);
        assert_eq!(offset_of!(ClimaxVisibilitySettings, model_name), 0x8);
        assert_eq!(offset_of!(ClimaxVisibilitySettings, start_frame), 0x10);
        assert_eq!(offset_of!(ClimaxVisibilitySettings, end_frame), 0x14);

        assert_eq!(size_of::<ClimaxAnimationSettings>(), 0x20);
        assert_eq!(offset_of!(ClimaxAnimationSettings, scene), 0x0);
        assert_eq!(offset_of!(ClimaxAnimationSettings, model_name), 0x8);
        assert_eq!(offset_of!(ClimaxAnimationSettings, anime_name), 0x10);
        assert_eq!(offset_of!(ClimaxAnimationSettings, start_frame), 0x18);
        assert_eq!(offset_of!(ClimaxAnimationSettings, is_loop), 0x1C);

        assert_eq!(size_of::<ClimaxEffectSettings>(), 0x30);
        assert_eq!(offset_of!(ClimaxEffectSettings, scene), 0x0);
        assert_eq!(offset_of!(ClimaxEffectSettings, effect_name), 0x8);
        assert_eq!(offset_of!(ClimaxEffectSettings, model_name), 0x10);
        assert_eq!(offset_of!(ClimaxEffectSettings, joint_name), 0x18);
        assert_eq!(offset_of!(ClimaxEffectSettings, scale), 0x20);
        assert_eq!(offset_of!(ClimaxEffectSettings, start_frame), 0x24);
        assert_eq!(offset_of!(ClimaxEffectSettings, end_frame), 0x28);

        assert_eq!(size_of::<SpiritVisibilitySettings>(), 0x18);
        assert_eq!(offset_of!(SpiritVisibilitySettings, scene), 0x0);
        assert_eq!(offset_of!(SpiritVisibilitySettings, model_name), 0x8);
        assert_eq!(offset_of!(SpiritVisibilitySettings, start_frame), 0x10);
        assert_eq!(offset_of!(SpiritVisibilitySettings, end_frame), 0x14);

        assert_eq!(size_of::<SpiritAnimationSettings>(), 0x20);
        assert_eq!(offset_of!(SpiritAnimationSettings, scene), 0x0);
        assert_eq!(offset_of!(SpiritAnimationSettings, model_name), 0x8);
        assert_eq!(offset_of!(SpiritAnimationSettings, anime_name), 0x10);
        assert_eq!(offset_of!(SpiritAnimationSettings, start_frame), 0x18);
        assert_eq!(offset_of!(SpiritAnimationSettings, is_loop), 0x1C);

        assert_eq!(size_of::<SpiritEffectSettings>(), 0x30);
        assert_eq!(offset_of!(SpiritEffectSettings, scene), 0x0);
        assert_eq!(offset_of!(SpiritEffectSettings, effect_name), 0x8);
        assert_eq!(offset_of!(SpiritEffectSettings, model_name), 0x10);
        assert_eq!(offset_of!(SpiritEffectSettings, joint_name), 0x18);
        assert_eq!(offset_of!(SpiritEffectSettings, scale), 0x20);
        assert_eq!(offset_of!(SpiritEffectSettings, start_frame), 0x24);
        assert_eq!(offset_of!(SpiritEffectSettings, end_frame), 0x28);
    }
}
