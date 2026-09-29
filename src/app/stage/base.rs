use core::ffi::c_char;

use crate::{
    app::{GlobalStageParameter, StageDynamicObject, StageGroundCrackParam, StageID},
    cpp,
    phx::{Hash40, Vector3f},
};

#[cfg(feature = "impl")]
use crate::app::StageDrawModel;

mod stage_draw_model_manager;
mod stage_ground_attr_param;
mod stage_level_data;
mod stage_random;

pub use stage_draw_model_manager::*;
pub use stage_ground_attr_param::*;
pub use stage_level_data::*;
pub use stage_random::*;

#[repr(C)]
pub struct StageBase {
    vtable: &'static StageBaseVtable,
    pub create_param: StageCreateParam,
    _d0: [u8; 0x10],
    pub ground_attr_default_param_set: StageGroundAttrParamSet,
    _180: [u8; 0x180],
    pub level_data: cpp::UniquePtr<StageLevelData>,
    _308: usize,
    pub draw_model_manager: cpp::UniquePtr<StageDrawModelManager>,
    _318: usize,
    _320: usize,
    _328: usize,
    _330: usize,
    _338: usize,
    pub random: cpp::UniquePtr<StageRandom>,
    _348: usize,
    _350: usize,
    _358: usize,
    _360: [u8; 0x3D8],
}

#[cfg(feature = "impl")]
impl StageBase {
    pub fn create(&mut self, create_param: &StageCreateParam) {
        unsafe {
            #[skyline::from_offset(0x25D3210)]
            fn inner(this: *mut StageBase, create_param: *const StageCreateParam);

            inner(self, create_param);
        }
    }

    /// Returns the working stage identifier.
    pub const fn stage_id(&self) -> &StageID {
        self.create_param.global_stage_parameter.stage_id()
    }

    pub fn search_dynamic_object(&self, name: Hash40) -> Option<*mut StageDynamicObject> {
        self.level_data.search_dynamic_object(name)
    }

    pub fn search_draw_model(&self, name: Hash40) -> Option<&StageDrawModelAccessor> {
        self.draw_model_manager.search_model(name)
    }

    pub fn instance_draw_model(
        &mut self,
        accessor: &StageDrawModelAccessor,
    ) -> *mut StageDrawModel {
        self.draw_model_manager.instance_model(accessor)
    }

    pub fn create_related_move_floor(&self, dynamic_object: &StageDynamicObject) {
        (self.vtable.create_related_move_floor)(self, core::ptr::null(), dynamic_object);
    }
}

#[repr(C)]
struct StageBaseVtable {
    pub get_stage_id: extern "C" fn(*const StageBase) -> StageID,
    pub get_stage_place_name: extern "C" fn(*const StageBase) -> Hash40,
    _10: extern "C" fn(*const StageBase) -> bool,
    pub get_delta_position: extern "C" fn(*const StageBase) -> Vector3f,
    _20: extern "C" fn(*const StageBase) -> i32,
    pub get_fighter_start_position: extern "C" fn(*const StageBase, i32) -> Vector3f,
    _30: extern "C" fn(*const StageBase) -> Vector3f,
    _38: extern "C" fn(*const StageBase) -> i32,
    _40: extern "C" fn(*const StageBase) -> Vector3f,
    pub get_fighter_restart_position_num: extern "C" fn(*const StageBase) -> usize,
    pub get_fighter_restart_position: extern "C" fn(*const StageBase, u32) -> Vector3f,
    _58: extern "C" fn(*const StageBase) -> Vector3f,
    pub get_ptrainer_start_position_num: extern "C" fn(*const StageBase) -> i32,
    pub get_ptrainer_start_position: extern "C" fn(*const StageBase, i32) -> Vector3f,
    pub exists_kirifuda_position: extern "C" fn(*const StageBase, StageKirifudaKind) -> bool,
    pub get_kirifuda_position: extern "C" fn(*const StageBase, StageKirifudaKind) -> Vector3f,
    pub get_kirifuda_angle: extern "C" fn(*const StageBase, StageKirifudaKind) -> f32,
    pub get_final_ness_param: extern "C" fn(*const StageBase) -> StageFinalNessParam,
    pub get_final_koopa_param: extern "C" fn(*const StageBase) -> StageFinalKoopaParam,
    _98: usize,
    _a0: usize,
    _a8: usize,
    _b0: usize,
    pub get_wind_param: extern "C" fn(*const StageBase) -> StageWindParam,
    pub get_ground_effect_position: extern "C" fn(*const StageBase, &Vector3f) -> Vector3f,
    pub get_ground_effect_vector: extern "C" fn(*const StageBase, &Vector3f) -> Vector3f,
    _d0: usize,
    _d8: usize,
    _e0: usize,
    pub can_get_item_popup_info: extern "C" fn(*const StageBase) -> bool,
    pub get_item_popup_region_num: extern "C" fn(*const StageBase) -> u32,
    _f8: usize,
    _100: usize,
    pub get_nightmare_scale: extern "C" fn(*const StageBase) -> f32,
    pub get_nightmare_position: extern "C" fn(*const StageBase) -> Vector3f,
    pub get_andross_position: extern "C" fn(*const StageBase) -> Vector3f,
    pub get_devil_position: extern "C" fn(*const StageBase) -> Vector3f,
    pub get_dragoon_camera_rectangle: usize,
    pub get_smashball_rect: usize,
    pub get_camera_param: usize,
    pub get_camera_range: usize,
    pub get_dead_range: usize,
    _150: usize,
    _158: usize,
    pub get_pause_camera_min_distance: extern "C" fn(*const StageBase) -> f32,
    pub get_pause_camera_max_distance: extern "C" fn(*const StageBase) -> f32,
    pub get_training_camera_min_distance: extern "C" fn(*const StageBase) -> f32,
    _178: usize,
    pub can_dead_front: extern "C" fn(*const StageBase) -> bool,
    pub can_dead_back: extern "C" fn(*const StageBase) -> bool,
    pub get_valid_range_z: extern "C" fn(*const StageBase) -> f32,
    pub is_horizontal_reverse_enabled: extern "C" fn(*const StageBase) -> bool,
    pub is_vertical_reverse_enabled: extern "C" fn(*const StageBase) -> bool,
    _1a8: usize,
    _1b0: usize,
    _1b8: usize,
    _1c0: usize,
    _1c8: usize,
    _1d0: usize,
    _1d8: usize,
    _1e0: usize,
    _1e8: usize,
    _1f0: usize,
    _1f8: usize,
    _200: usize,
    pub get_final_color: extern "C" fn(*const StageBase) -> *const StageFinalColor,
    pub get_charge_final_color: extern "C" fn(*const StageBase) -> *const StageFinalColor,
    _218: usize,
    _220: usize,
    _228: usize,
    _230: usize,
    _238: usize,
    _240: usize,
    _248: usize,
    _250: usize,
    _258: usize,
    _260: usize,
    _268: usize,
    pub get_near_clip: extern "C" fn(*const StageBase) -> f32,
    pub get_far_clip: extern "C" fn(*const StageBase) -> f32,
    _280: usize,
    _288: usize,
    _290: usize,
    pub get_other_ground_crack_param:
        extern "C" fn(*const StageBase, *const Vector3f, i32, *mut StageGroundCrackParam),
    _2a0: [usize; 52], // __cxa_deleted_virtual
    pub destroy: extern "C" fn(*mut StageBase),
    pub delete: extern "C" fn(*mut StageBase),
    _450: usize,
    _458: usize,
    pub create_related_move_floor:
        extern "C" fn(*const StageBase, *const c_char, *const StageDynamicObject) -> [u8; 17],
    _468: usize,
    _470: usize,
    _478: usize,
    _480: usize,
    _488: usize,
    _490: usize,
    _498: usize,
    _4a0: usize,
    _4a8: usize,
    pub create_level_data: extern "C" fn(*const StageBase) -> *mut StageLevelData,
    _4b8: usize,
    _4c0: usize,
    _4c8: usize,
    pub create_draw_model_manager: extern "C" fn(*const StageBase) -> *mut StageDrawModelManager,
    _4d8: usize,
    _4e0: usize,
    _4e8: usize,
    _4f0: usize,
    _4f8: usize,
    pub pre_update: extern "C" fn(*mut StageBase, f32),
    pub post_update: extern "C" fn(*mut StageBase, f32),
    _510: usize,
    _518: usize,
    _520: usize,
    pub pre_update_gimmick: extern "C" fn(*mut StageBase, f32),
    pub post_update_gimmick: extern "C" fn(*mut StageBase, f32),
    _538: usize,
    _540: usize,
    _548: usize,
    _550: usize,
    _558: usize,
    _560: usize,
    _568: usize,
    _570: usize,
    _578: usize,
    _580: usize,
    _588: usize,
    _590: usize,
    _598: usize,
    _5a0: usize,
    _5a8: usize,
    pub notify_event_info_ready: extern "C" fn(*mut StageBase, f32),
    pub notify_event_info_go: extern "C" fn(*mut StageBase, bool),
    pub notify_event_bg_effect_start: extern "C" fn(*mut StageBase),
    pub notify_event_bg_effect_end: extern "C" fn(*mut StageBase),
    pub notify_event_transition: extern "C" fn(*mut StageBase),
    _5d8: [usize; 88], // __cxa_deleted_virtual
}

#[repr(C)]
pub struct StageBattleBase {
    pub base: StageBase,
    pub is_expanded_end: bool,
    pub expansion_scale: f32,
    pub camera_path: cpp::UniquePtr<()>,
    pub camera_path_frame: f32,
    pub camera_path_model: cpp::Vector<()>,
    pub kumite_item_base_controller: usize,
}

#[repr(C)]
pub struct StageCreateParam {
    pub global_stage_parameter: GlobalStageParameter,
    _b0: i32,
    _b8: usize,
    _c0: bool,
    _c1: bool,
}

impl StageCreateParam {
    /// Creates a new `StageCreateParam`.
    pub const fn new(
        global_stage_parameter: &GlobalStageParameter,
        param_2: i32,
        param_3: usize,
        param_4: bool,
        param_5: bool,
    ) -> Self {
        Self {
            global_stage_parameter: *global_stage_parameter,
            _b0: param_2,
            _b8: param_3,
            _c0: param_4,
            _c1: param_5,
        }
    }
}

#[derive(Default)]
#[repr(C)]
pub struct StageFinalColor {
    pub frame_character_1: i32,
    pub gain_r_character_1: f32,
    pub gain_g_character_1: f32,
    pub gain_b_character_1: f32,
    pub offset_r_character_1: f32,
    pub offset_g_character_1: f32,
    pub offset_b_character_1: f32,
    pub saturation_character_1: f32,
    pub contrast_character_1: f32,
    pub frame_character_2: i32,
    pub gain_r_character_2: f32,
    pub gain_g_character_2: f32,
    pub gain_b_character_2: f32,
    pub offset_r_character_2: f32,
    pub offset_g_character_2: f32,
    pub offset_b_character_2: f32,
    pub saturation_character_2: f32,
    pub contrast_character_2: f32,
    pub frame_ground_1: i32,
    pub gain_r_ground_1: f32,
    pub gain_g_ground_1: f32,
    pub gain_b_ground_1: f32,
    pub offset_r_ground_1: f32,
    pub offset_g_ground_1: f32,
    pub offset_b_ground_1: f32,
    pub saturation_ground_1: f32,
    pub contrast_ground_1: f32,
    pub frame_ground_2: i32,
    pub gain_r_ground_2: f32,
    pub gain_g_ground_2: f32,
    pub gain_b_ground_2: f32,
    pub offset_r_ground_2: f32,
    pub offset_g_ground_2: f32,
    pub offset_b_ground_2: f32,
    pub saturation_ground_2: f32,
    pub contrast_ground_2: f32,
}

#[derive(Debug)]
#[repr(C)]
pub enum StageKirifudaKind {
    Pit,
    Pikmin,
    Ike1,
    Ike2,
    Ness,
    PikminExtra1,
    PikminExtra2,
}

#[derive(Debug)]
#[repr(C)]
pub struct StageFinalNessParam {
    pub length_scale: f32,
    pub width_scale: f32,
    pub angular_velocity: f32,
    pub max_angle: f32,
}

#[derive(Debug)]
#[repr(C)]
pub struct StageFinalKoopaParam {
    pub x286f4d767b: f32,
    pub scale: f32,
    pub offset: Vector3f,
}

#[derive(Debug)]
#[repr(C)]
pub struct StageWindParam {
    pub speed: f32,
    pub angle: f32,
    pub cycle: f32,
    pub amplitude: f32,
    pub rand_coord: u32,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBase>(), 0x738);
        assert_eq!(offset_of!(StageBase, vtable), 0x0);
        assert_eq!(offset_of!(StageBase, create_param), 0x8);
        assert_eq!(offset_of!(StageBase, ground_attr_default_param_set), 0xE0);
        assert_eq!(offset_of!(StageBase, level_data), 0x300);
        assert_eq!(offset_of!(StageBase, draw_model_manager), 0x310);
        assert_eq!(offset_of!(StageBase, random), 0x340);

        assert_eq!(size_of::<StageBattleBase>(), 0x770);
        assert_eq!(offset_of!(StageBattleBase, base), 0x0);
        assert_eq!(offset_of!(StageBattleBase, is_expanded_end), 0x738);
        assert_eq!(offset_of!(StageBattleBase, expansion_scale), 0x73C);
        assert_eq!(offset_of!(StageBattleBase, camera_path), 0x740);
        assert_eq!(offset_of!(StageBattleBase, camera_path_frame), 0x748);
        assert_eq!(offset_of!(StageBattleBase, camera_path_model), 0x750);
        assert_eq!(offset_of!(StageBattleBase, kumite_item_base_controller), 0x768);

        assert_eq!(size_of::<StageCreateParam>(), 0xC8);
        assert_eq!(offset_of!(StageCreateParam, global_stage_parameter), 0x0);
        assert_eq!(offset_of!(StageCreateParam, _b0), 0xB0);
        assert_eq!(offset_of!(StageCreateParam, _b8), 0xB8);
        assert_eq!(offset_of!(StageCreateParam, _c0), 0xC0);
        assert_eq!(offset_of!(StageCreateParam, _c1), 0xC1);
    }
}
