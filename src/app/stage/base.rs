use core::{
    ffi::{c_char, c_void},
    mem::{self, MaybeUninit},
    ops,
};

use crate::{
    app::{
        GlobalStageParameter, StageDynamicObject, StageGroundCrackParam, StageID, grcol::AttrType,
    },
    cpp,
    lib::Rect,
    phx::{Hash40, Vector2f, Vector3f},
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
    _360: [u8; 0x3D1],
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

    /// Returns a reference to the working stage identifier.
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
    /// Returns the working stage identifier.
    pub get_stage_id: extern "C" fn(*const StageBase) -> StageID,

    /// Returns the hashed umbrella name of the working stage.
    pub get_stage_place_name: extern "C" fn(*const StageBase) -> Hash40,

    // TODO
    pub _10: extern "C" fn(*const StageBase) -> bool,

    /// Returns the working scrolling velocity of the stage.
    pub get_stage_position_delta: extern "C" fn(*const StageBase) -> Vector3f,

    /// Returns the number of fighter entry positions.
    pub get_fighter_start_position_num: extern "C" fn(*const StageBase) -> i32,

    /// Returns the entry position for a fighter at the given index.
    pub get_fighter_start_position: extern "C" fn(*const StageBase, i32) -> Vector3f,

    // TODO. Unused?
    pub _30: extern "C" fn(*const StageBase) -> Vector3f,

    // TODO. Unused?
    pub _38: extern "C" fn(*const StageBase) -> i32,

    // TODO. Unused?
    pub _40: extern "C" fn(*const StageBase) -> Vector3f,

    /// Returns the number of recovery platform target positions.
    pub get_fighter_restart_position_num: extern "C" fn(*const StageBase) -> usize,

    /// Returns the target position for a recovery platform at the given index.
    pub get_fighter_restart_position: extern "C" fn(*const StageBase, i32) -> Vector3f,

    /// Returns the working scrolling velocity for a recovery platform.
    pub get_fighter_restart_position_delta: extern "C" fn(*const StageBase) -> Vector3f,

    /// Returns the number of Pokémon Trainer initial positions.
    pub get_ptrainer_start_position_num: extern "C" fn(*const StageBase) -> i32,

    /// Returns the initial position for a Pokémon Trainer at the given index.
    pub get_ptrainer_start_position: extern "C" fn(*const StageBase, i32) -> Vector3f,

    /// Returns `true` if a Level Data-driven Final Smash locator contains a valid position.
    pub exists_kirifuda_position: extern "C" fn(*const StageBase, StageKirifudaKind) -> bool,

    /// Returns the target position of a Level Data-driven Final Smash locator.
    pub get_kirifuda_position: extern "C" fn(*const StageBase, StageKirifudaKind) -> Vector3f,

    /// Returns the target camera angle of a Level Data-driven Final Smash around the x-axis.
    pub get_kirifuda_angle: extern "C" fn(*const StageBase, StageKirifudaKind) -> f32,

    /// Returns the set of parameters corresponding to Ness's Final Smash.
    pub get_final_ness_param: extern "C" fn(*const StageBase) -> StageFinalNessParam,

    /// Returns the set of parameters corresponding to Bowser's Final Smash.
    pub get_final_koopa_param: extern "C" fn(*const StageBase) -> StageFinalKoopaParam,

    /// Returns the set of parameters corresponding to the Ice Climbers' Final Smash.
    pub get_final_iceclimber_param: extern "C" fn(*const StageBase) -> StageFinalIceclimberParam,

    // TODO
    pub _a0: extern "C" fn(*const StageBase, *const c_void, &Vector3f, &Vector3f),

    // TODO
    pub _a8: extern "C" fn(*const StageBase, i32, &Vector3f, &Vector3f),

    // TODO
    pub _b0: extern "C" fn(*const StageBase),

    /// Returns the set of parameters corresponding to constant wind.
    pub get_wind_param: extern "C" fn(*const StageBase) -> StageWindParam,

    /// Returns the position of an effect upon ground collision.
    pub get_ground_effect_position: extern "C" fn(*const StageBase, &Vector3f) -> Vector3f,

    /// Returns the unit normal vector of an effect upon ground collision.
    pub get_ground_effect_normal: extern "C" fn(*const StageBase, &Vector3f) -> Vector3f,

    /// Returns `true` if Bowser's Final Smash form should appear silhouetted behind stage models.
    pub is_final_koopa_silhouette: extern "C" fn(*const StageBase) -> bool,

    /// Returns `true` if items can appear.
    ///
    /// Unlike other adjacent item functions which return a `bool`, when this function returns `false`,
    /// it will be called every subsequent frame until it returns `true`.
    pub get_item_drop_status: extern "C" fn(*const StageBase) -> bool,

    /// Returns a coefficient to scale the interval of time when an item can randomly appear by.
    pub get_item_drop_interval_rate: extern "C" fn(*const StageBase, i32) -> f32,

    /// Returns `true` if the working `StageItemPopupInfo` instance exists.
    pub can_get_item_popup_info: extern "C" fn(*const StageBase) -> bool,

    /// Returns the number of enabled `StageItemPopupRegion` instances.
    pub get_item_popup_region_num: extern "C" fn(*const StageBase) -> u32,

    /// Returns a raw pointer to the enabled `StageItemPopupRegion` at the given index,
    /// or a null raw pointer if out of bounds.
    pub get_item_popup_region: extern "C" fn(*const StageBase, i32) -> *const c_void,

    /// Returns `true` if a position for a randomly-appearing item was computed.
    pub get_item_popup_position: extern "C" fn(*const StageBase, f32, &mut Vector3f) -> bool,

    /// Returns the uniform scale of Nightmare.
    pub get_nightmare_scale: extern "C" fn(*const StageBase) -> f32,

    /// Returns the position of Nightmare.
    pub get_nightmare_position: extern "C" fn(*const StageBase) -> Vector3f,

    /// Returns the target background position of Andross.
    pub get_andross_position: extern "C" fn(*const StageBase) -> Vector3f,

    /// Returns the target foreground position of Devil.
    pub get_devil_position: extern "C" fn(*const StageBase) -> Vector3f,

    // TODO
    pub get_dragoon_camera_rectangle: extern "C" fn(*const StageBase) -> Rect,

    // TODO
    pub get_smashball_rect: extern "C" fn(*const StageBase) -> Rect,

    // TODO
    pub get_camera_param: extern "C" fn(*const StageBase) -> *const c_void,

    /// Returns a pointer to the working camera boundary.
    pub get_camera_range: extern "C" fn(*const StageBase) -> *const Rect,

    /// Returns a pointer to the working blast zone.
    pub get_dead_range: extern "C" fn(*const StageBase) -> *const Rect,

    // TODO. Related to the camera. Executes once per frame during a performance effect. `Tomodachi` and `BalloonFight` both return `true`.
    pub _150: extern "C" fn(*const StageBase) -> bool,

    // TODO. Related to the camera. Executes once per frame. Always returns `false`.
    pub _158: extern "C" fn(*const StageBase) -> bool,

    /// Returns the minimum distance from the pivot the camera can zoom in towards using camera controls.
    pub get_pause_camera_min_distance: extern "C" fn(*const StageBase) -> f32,

    /// Returns the maximum distance from the pivot the camera can zoom out towards using camera controls.
    pub get_pause_camera_max_distance: extern "C" fn(*const StageBase) -> f32,

    /// Returns the minimum distance from the pivot the camera can zoom in towards in Training.
    pub get_training_camera_min_distance: extern "C" fn(*const StageBase) -> f32,

    // TODO. Executes upon the first frame of battle entry.
    pub _178: extern "C" fn(*const StageBase) -> (u64, f32),

    /// Returns `true` if screen KOs can occur.
    pub can_dead_front: extern "C" fn(*const StageBase) -> bool,

    /// Returns `true` if star KOs can occur.
    pub can_dead_back: extern "C" fn(*const StageBase) -> bool,

    /// Returns the distance a `BattleObject` can be pushed into the background upon being jostled.
    pub get_valid_range_z: extern "C" fn(*const StageBase) -> f32,

    // TODO
    pub is_horizontal_reverse_enabled: extern "C" fn(*const StageBase) -> bool,

    // TODO
    pub is_vertical_reverse_enabled: extern "C" fn(*const StageBase) -> bool,

    // TODO
    pub _1a8: extern "C" fn(*mut StageBase),

    // TODO
    pub _1b0: extern "C" fn(*mut StageBase),

    // TODO. Related to Smash Taunts.
    pub _1b8: extern "C" fn(*const StageBase) -> bool,

    /// Returns `true` if a Smash Taunt can be activated at this time.
    ///
    /// If this function returns `false`, a Smash Taunt can still be activated at a later time.
    pub is_start_appeal_timing: extern "C" fn(*const StageBase) -> bool,

    // TODO. Executes after a Smash Taunt has successfully been performed.
    pub _1c8: extern "C" fn(*mut StageBase) -> bool,

    /// Assigns the `FighterKind` of the Smash Taunt's subject fighter.
    pub set_appeal_subject_fighter_kind: extern "C" fn(*mut StageBase, i32),

    // TODO. Executes to force stop the Smash Taunt?
    pub _1d8: extern "C" fn(*const StageBase),

    // TODO. Related to Smash Taunts.
    pub _1e0: extern "C" fn(*const StageBase),

    // TODO. Related to Smash Taunts. Only called for `MG_Shadowmoses` and `Icarus_Angeland`.
    pub _1e8: extern "C" fn(*mut StageBase, u32),

    // TODO
    pub _1f0: extern "C" fn(*const StageBase) -> *const c_void,

    /// Returns a pointer to the set of parameters corresponding to the given material's attributes upon collision.
    pub get_ground_attr_param_set:
        extern "C" fn(*const StageBase, AttrType) -> *const StageGroundAttrParamSet,

    // TODO
    pub get_any_damage_floor_data: extern "C" fn(*const StageBase, u32, &mut c_void) -> bool,

    /// Returns a raw pointer to the set of parameters corresponding to color correction upon
    /// readying a Final Smash, or a null raw pointer if not customized.
    pub get_final_color_param: extern "C" fn(*const StageBase) -> *const StageFinalColorParam,

    /// Returns a raw pointer to the set of parameters corresponding to color correction upon
    /// readying a charged Final Smash, or a null raw pointer if not customized.
    pub get_charge_final_color_param:
        extern "C" fn(*const StageBase) -> *const StageFinalColorParam,

    // TODO
    pub set_vision: extern "C" fn(*mut StageBase, StageVisionMessageKind),

    // TODO
    pub _220: extern "C" fn(*const StageBase, u32) -> i32,

    // TODO
    pub _228: extern "C" fn(*mut StageBase, u32, i32),

    // TODO
    pub _230: extern "C" fn(*const StageBase, u32) -> f32,

    // TODO
    pub _238: extern "C" fn(*mut StageBase, u32, f32),

    // TODO
    pub _240: extern "C" fn(*const StageBase, u32) -> Vector3f,

    // TODO
    pub _248: extern "C" fn(*mut StageBase, u32, &Vector3f),

    _250: usize, // Never overriden, possibly unused.

    // TODO
    pub _258: extern "C" fn(*mut StageBase, u32, *mut c_void),

    // TODO
    pub _260: extern "C" fn(*const StageBase) -> *const c_void,

    // TODO
    pub _268: extern "C" fn(*const StageBase),

    /// Returns the distance of the near-clipping plane from the camera.
    pub get_near_clip: extern "C" fn(*const StageBase) -> f32,

    /// Returns the distance of the far-clipping plane from the camera.
    pub get_far_clip: extern "C" fn(*const StageBase) -> f32,

    // TODO
    pub _280: extern "C" fn(*const StageBase) -> Vector3f,

    // TODO. Related to stage scrolling.
    pub _288: extern "C" fn(*const StageBase) -> f32,

    // TODO. Executes once per frame when mining the ground with Steve's neutral special move.
    pub _290: extern "C" fn(*const StageBase, i32, i32, i32) -> f32,

    // TODO
    pub get_other_ground_crack_param:
        extern "C" fn(*const StageBase, &Vector3f, i32, &mut StageGroundCrackParam),

    _2a0: [usize; 52], // __cxa_deleted_virtual

    /// Frees the resources that may have been acquired.
    pub destroy: extern "C" fn(*mut StageBase),

    /// Deallocates the memory reserved for `this`.
    pub delete: extern "C" fn(*mut StageBase),

    // TODO
    pub _450: extern "C" fn(*mut StageBase),

    // TODO
    pub _458: extern "C" fn(*mut StageBase),

    // TODO
    pub create_related_move_floor:
        extern "C" fn(*const StageBase, *const c_char, *const StageDynamicObject) -> [u8; 17],

    // TODO
    pub _468: extern "C" fn(*mut StageBase),

    // TODO
    pub _470: extern "C" fn(*mut StageBase),

    // TODO
    pub _478: extern "C" fn(*mut StageBase),

    // TODO
    pub _480: extern "C" fn(*const StageBase) -> Rect,

    // TODO
    pub _488: extern "C" fn(*const StageBase) -> bool,

    // TODO
    pub _490: extern "C" fn(*const StageBase) -> bool,

    // TODO
    pub _498: extern "C" fn(*const StageBase) -> f32,

    // TODO
    pub _4a0: extern "C" fn(*const StageBase) -> bool,

    // TODO
    pub _4a8: extern "C" fn(*const StageBase, *mut c_void) -> bool,

    // TODO
    pub create_level_data: extern "C" fn(*const StageBase) -> *mut StageLevelData,

    // TODO
    pub create_camera_controller: extern "C" fn(*const StageBase) -> *mut c_void,

    // TODO
    pub create_render_controller: extern "C" fn(*const StageBase) -> *mut c_void,

    // TODO
    pub _4c8: extern "C" fn(*const StageBase) -> *mut c_void,

    // TODO
    pub create_draw_model_manager: extern "C" fn(*const StageBase) -> *mut StageDrawModelManager,

    // TODO
    pub _4d8: extern "C" fn(*const StageBase) -> *mut c_void,

    // TODO
    pub _4e0: extern "C" fn(*mut StageBase),

    // TODO
    pub _4e8: extern "C" fn(*mut StageBase),

    // TODO
    pub _4f0: extern "C" fn(*mut StageBase),

    // TODO
    pub _4f8: extern "C" fn(*mut StageBase),

    // TODO
    pub pre_update: extern "C" fn(*mut StageBase, f32),

    // TODO
    pub post_update: extern "C" fn(*mut StageBase, f32),

    // TODO
    pub _510: extern "C" fn(*mut StageBase, f32),

    // TODO
    pub _518: extern "C" fn(*mut StageBase),

    _520: [usize; 1], // __cxa_deleted_virtual

    // TODO
    pub pre_update_gimmick: extern "C" fn(*mut StageBase, f32),

    // TODO
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

    // TODO
    pub get_damage_floor_data: extern "C" fn(*const StageBase, u32, &mut c_void) -> bool,

    // TODO
    pub get_death_floor_data: extern "C" fn(*const StageBase, u32, &mut c_void) -> bool,

    // TODO
    pub initialize_gimmick: extern "C" fn(*const StageBase, *mut c_void, *const StageDynamicObject),

    // TODO
    pub _5a0: extern "C" fn(*mut StageBase),

    // TODO
    pub _5a8: extern "C" fn(*mut StageBase, i32),

    /// Executes upon each frame of the pre-match countdown.
    pub notify_event_info_ready: extern "C" fn(*mut StageBase, f32),

    /// Executes upon the first frame of user input.
    pub notify_event_info_go: extern "C" fn(*mut StageBase, bool),

    /// Executes upon the first frame of an effect background's appearance.
    pub notify_event_bg_effect_start: extern "C" fn(*mut StageBase),

    /// Executes upon the first frame of an effect background's disappearance.
    pub notify_event_bg_effect_end: extern "C" fn(*mut StageBase),

    /// Executes upon the first update frame after changing stages through the Stage Morph option.
    pub notify_event_transition: extern "C" fn(*mut StageBase),

    _5d8: [usize; 88], // __cxa_deleted_virtual
}

#[repr(C, align(16))]
pub struct StageBattleBase {
    pub base: StageBaseStorage,
    pub form_type: StageFormType,
    pub is_expanded_end: bool,
    pub expansion_scale: f32,
    pub camera_path: cpp::UniquePtr<()>,
    pub camera_path_frame: f32,
    pub camera_path_model: cpp::Vector<()>,
    pub kumite_item_base_controller: cpp::UniquePtr<()>,
}

#[repr(C)]
pub enum StageFormType {
    Battle,
    End,
}

#[repr(C)]
pub struct StageBaseStorage {
    inner: MaybeUninit<[u8; 0x731]>,
}

impl ops::Deref for StageBaseStorage {
    type Target = StageBase;

    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute::<&[u8; 0x731], &StageBase>(self.inner.assume_init_ref()) }
    }
}

impl ops::DerefMut for StageBaseStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { mem::transmute::<&mut [u8; 0x731], &mut StageBase>(self.inner.assume_init_mut()) }
    }
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

/// The set of parameters corresponding to color correction upon readying a Final Smash.
#[derive(Debug, Default)]
#[repr(C)]
pub struct StageFinalColorParam {
    /// The frame duration of the interpolation towards the initial target character color.
    pub frame_character_1: i32,

    /// The red component of the initial target character color scale.
    pub gain_r_character_1: f32,

    /// The green component of the initial target character color scale.
    pub gain_g_character_1: f32,

    /// The blue component of the initial target character color scale.
    pub gain_b_character_1: f32,

    /// The red component of the initial target character color displacement.
    pub offset_r_character_1: f32,

    /// The green component of the initial target character color displacement.
    pub offset_g_character_1: f32,

    /// The blue component of the initial target character color displacement.
    pub offset_b_character_1: f32,

    /// The saturation scale of the initial target character color.
    pub saturation_character_1: f32,

    /// The contrast scale of the initial target character color.
    pub contrast_character_1: f32,

    /// The frame duration of the interpolation towards the final target character color.
    pub frame_character_2: i32,

    /// The red component of the final target character color scale.
    pub gain_r_character_2: f32,

    /// The green component of the final target character color scale.
    pub gain_g_character_2: f32,

    /// The blue component of the final target character color scale.
    pub gain_b_character_2: f32,

    /// The red component of the final target character color displacement.
    pub offset_r_character_2: f32,

    /// The green component of the final target character color displacement.
    pub offset_g_character_2: f32,

    /// The blue component of the final target character color displacement.
    pub offset_b_character_2: f32,

    /// The saturation scale of the final target character color.
    pub saturation_character_2: f32,

    /// The contrast scale of the final target character color.
    pub contrast_character_2: f32,

    /// The frame duration of the interpolation towards the initial target stage color.
    pub frame_ground_1: i32,

    /// The red component of the initial target stage color scale.
    pub gain_r_ground_1: f32,

    /// The green component of the initial target stage color scale.
    pub gain_g_ground_1: f32,

    /// The blue component of the initial target stage color scale.
    pub gain_b_ground_1: f32,

    /// The red component of the initial target stage color displacement.
    pub offset_r_ground_1: f32,

    /// The green component of the initial target stage color displacement.
    pub offset_g_ground_1: f32,

    /// The blue component of the initial target stage color displacement.
    pub offset_b_ground_1: f32,

    /// The saturation scale of the initial target stage color.
    pub saturation_ground_1: f32,

    /// The contrast scale of the initial target stage color.
    pub contrast_ground_1: f32,

    /// The frame duration of the interpolation towards the final target stage color.
    pub frame_ground_2: i32,

    /// The red component of the final target stage color scale.
    pub gain_r_ground_2: f32,

    /// The green component of the final target stage color scale.
    pub gain_g_ground_2: f32,

    /// The blue component of the final target stage color scale.
    pub gain_b_ground_2: f32,

    /// The red component of the final target stage color displacement.
    pub offset_r_ground_2: f32,

    /// The green component of the final target stage color displacement.
    pub offset_g_ground_2: f32,

    /// The blue component of the final target stage color displacement.
    pub offset_b_ground_2: f32,

    /// The saturation scale of the final target stage color.
    pub saturation_ground_2: f32,

    /// The contrast scale of the final target stage color.
    pub contrast_ground_2: f32,
}

/// A numeric identifier for a Level Data-driven Final Smash element locator.
#[derive(Debug)]
#[repr(C)]
pub enum StageKirifudaKind {
    /// Unused.
    Pit,

    /// The target point for the ship in Olimar's Final Smash.
    ///
    /// If [`Ike2`] exists in the Level Data, [`PikminExtra1`] and [`PikminExtra2`] are used instead.
    ///
    /// [`Ike2`]: Self::Ike2
    /// [`PikminExtra1`]: Self::PikminExtra1
    /// [`PikminExtra2`]: Self::PikminExtra2
    Pikmin,

    /// The primary target point for Ike's Final Smash.
    ///
    /// **Additionally used by**:
    ///
    /// - Meta Knight (Final Smash)
    /// - Zero Suit Samus (Final Smash)
    /// - Diddy Kong (Final Smash; right before the final hit)
    /// - Lucario (Final Smash)
    /// - Little Mac (Final Smash; unused?)
    /// - Greninja (Final Smash)
    /// - Mii Brawler (Final Smash)
    /// - Robin (Final Smash)
    /// - Cloud (Final Smash)
    /// - Ridley (Final Smash; for the victims' drop point)
    /// - Zoroark (after a successful initial attack)
    Ike1,

    /// The secondary target point for Ike's Final Smash.
    /// This point will be selected if it exists and is closer to the user.
    ///
    /// **Additionally used by**:
    ///
    /// - Meta Knight (Final Smash)
    /// - Zero Suit Samus (Final Smash)
    /// - Little Mac (Final Smash; unused?)
    /// - Robin (Final Smash)
    /// - Cloud (Final Smash)
    /// - Ridley (Final Smash; for the victims' drop point)
    /// - Zoroark (after a successful initial attack)
    Ike2,

    /// The meteor emission point for Ness's Final Smash.
    ///
    /// **Additionally used by**:
    ///
    /// - Lucas (Final Smash)
    Ness,

    /// A substitute target point over [`Pikmin`] for the ship in Olimar's Final Smash,
    /// selected at random between this and [`PikminExtra2`].
    ///
    /// Shares an x-position with [`Ike1`] and both a y-position and z-position with [`Pikmin`].
    ///
    /// [`Ike1`]: Self::Ike1
    /// [`Pikmin`]: Self::Pikmin
    /// [`PikminExtra2`]: Self::PikminExtra2
    PikminExtra1,

    /// A substitute target point over [`Pikmin`] for the ship in Olimar's Final Smash,
    /// selected at random between this and [`PikminExtra1`].
    ///
    /// Shares an x-position with [`Ike2`] and both a y-position and z-position with [`Pikmin`].
    ///
    /// [`Ike2`]: Self::Ike2
    /// [`Pikmin`]: Self::Pikmin
    /// [`PikminExtra1`]: Self::PikminExtra1
    PikminExtra2,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct StageFinalNessParam {
    pub length_scale: f32,
    pub width_scale: f32,
    pub angular_velocity: f32,
    pub max_angle: f32,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct StageFinalKoopaParam {
    pub x286f4d767b: f32,
    pub scale: f32,
    pub offset: Vector3f,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct StageFinalIceclimberParam {
    pub offset: Vector2f,
    pub scale: Vector2f,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct StageWindParam {
    pub speed: f32,
    pub angle: f32,
    pub cycle: f32,
    pub amplitude: f32,
    pub rand_coord: u32,
}

#[derive(Debug)]
#[repr(C)]
pub enum StageVisionMessageKind {
    None,
    Ready,
    Countdown5,
    Countdown4,
    Countdown3,
    Countdown2,
    Countdown1,
    Go,
    SuddenDeath,
    TimeUp,
    Complete,
    Lose,
    GameSet,
    Unk13,
    Unk14,
    Defeated1P,
    Defeated2P,
    Defeated3P,
    Defeated4P,
    Defeated5P,
    Defeated6P,
    Defeated7P,
    Defeated8P,
    Victory,
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
        assert_eq!(offset_of!(StageBattleBase, form_type), 0x734);
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
