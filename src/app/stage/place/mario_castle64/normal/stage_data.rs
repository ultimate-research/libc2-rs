use crate::{cpp, phx::ParameterReceiverBase};

/// The specialized parameters for the Peach's Castle stage.
#[repr(C)]
pub struct StageData {
    pub base: ParameterReceiverBase,

    /// The radius of the Bumper's search collision.
    pub bumper_search_shape_radius: f32,

    /// The radius of the Bumper's attack collision.
    pub bumper_attack_shape_radius: f32,

    /// The damage dealt by the Bumper's attack collision.
    pub bumper_attack_power: f32,

    /// The launch angle of the Bumper's attack collision.
    pub bumper_attack_vector: i32,

    /// The knockback scaling of the Bumper's attack collision.
    pub bumper_attack_r_eff: i32,

    /// The fixed knockback of the Bumper's attack collision.
    pub bumper_attack_r_fix: i32,

    /// The base knockback of the Bumper's attack collision.
    pub bumper_attack_r_add: i32,

    /// The number of frames to wait before the Bumper's attack collision can damage an attacked opponent again.
    pub bumper_attack_serial_hit_frame: u32,

    /// The number of frames to put the Bumper's collisions to sleep for after attacking.
    pub bumper_attack_interval: f32,

    /// The position of the Bumper along the y-axis.
    pub bumper_y: f32,

    /// The position of the Bumper along the z-axis.
    pub bumper_z: f32,

    /// The starting and ending position of the Bumper along the x-axis.
    pub bumper_x_base: f32,

    /// The range that the Bumper can move between along the x-axis.
    pub bumper_x_range: f32,

    /// The number of frames that the Bumper's move cycle lasts for.
    pub bumper_loop_length: f32,

    /// The number of frames that the Bumper waits at its base position for before repeating its move cycle.
    pub bumper_wait_length: f32,

    /// The animation frame index to stop animation playback on when the Bumper is not attacking.
    pub bumper_wait_animation_frame: i32,

    /// The animation frame index to start animation playback on when the Bumper is attacking.
    pub bumper_attack_animation_start_frame: i32,

    /// The number of frames that the Bumper's attack collision is tangible for.
    pub bumper_attack_length: f32,

    /// The collection of animation frame index intervals representing when the sliding platform is moving.
    ///
    /// These parameters are only used for playing sound effects as the sliding platform moves using an animation file.
    pub floor_moving_intervals: cpp::Vector<FloorMovingInterval>,

    /// The parameters for controlling when the left side of the stage can be grabbed.
    ///
    /// These parameters are assigned to the cliff object with the dynamic name of `transform2` from the ground collision object with the dynamic name of `stc_ring_set`.
    pub cliff_validity_ring_l: CliffValidity,

    /// The parameters for controlling when the right side of the stage can be grabbed.
    ///
    /// These parameters are assigned to the cliff object with the dynamic name of `transform1` from the ground collision object with the dynamic name of `stc_ring_set`.
    pub cliff_validity_ring_r: CliffValidity,

    /// The parameters for controlling when the left side of the sliding platform can be grabbed.
    ///
    /// These parameters are assigned to the cliff object with the dynamic name of `transform2` from the ground collision object with the dynamic name of `dyr_floor_set`.
    pub cliff_validity_floor_l: CliffValidity,

    /// The parameters for controlling when the right side of the sliding platform can be grabbed.
    ///
    /// These parameters are assigned to the cliff object with the dynamic name of `transform1` from the ground collision object with the dynamic name of `dyr_floor_set`.
    pub cliff_validity_floor_r: CliffValidity,

    /// The collection of positions for each camera subject.
    pub camera_sub_pos: cpp::Vector<CameraSubPos>,
}

/// The set of parameters corresponding to when the sliding platform is moving.
#[repr(C)]
pub struct FloorMovingInterval {
    /// The animation frame index representing when the sliding platform has started moving.
    pub start: f32,

    /// The animation frame index representing when the sliding platform has stopped moving.
    pub end: f32,
}

/// The set of parameters corresponding to when an edge can be grabbed.
#[repr(C)]
pub struct CliffValidity {
    /// Determines if the cliff can be grabbed during the specified right-open interval of time.
    ///
    /// Animation frame indices outside of the interval use the inverse of the [`validity`](#structfield.validity) state.
    pub validity: bool,

    /// The animation frame index representing when to start using the [`validity`](#structfield.validity) state.
    pub start: f32,

    /// The animation frame index representing when to stop using the [`validity`](#structfield.validity) state.
    pub end: f32,
}

/// The set of parameters corresponding to the position of a `CameraSubject`.
#[repr(C)]
pub struct CameraSubPos {
    /// The component along the x-axis.
    pub x: f32,

    /// The component along the y-axis.
    pub y: f32,

    /// The component along the z-axis.
    pub z: f32,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageData>(), 0xB0);
        assert_eq!(offset_of!(StageData, base), 0x0);
        assert_eq!(offset_of!(StageData, bumper_search_shape_radius), 0x8);
        assert_eq!(offset_of!(StageData, bumper_attack_shape_radius), 0xC);
        assert_eq!(offset_of!(StageData, bumper_attack_power), 0x10);
        assert_eq!(offset_of!(StageData, bumper_attack_vector), 0x14);
        assert_eq!(offset_of!(StageData, bumper_attack_r_eff), 0x18);
        assert_eq!(offset_of!(StageData, bumper_attack_r_fix), 0x1C);
        assert_eq!(offset_of!(StageData, bumper_attack_r_add), 0x20);
        assert_eq!(offset_of!(StageData, bumper_attack_serial_hit_frame), 0x24);
        assert_eq!(offset_of!(StageData, bumper_attack_interval), 0x28);
        assert_eq!(offset_of!(StageData, bumper_y), 0x2C);
        assert_eq!(offset_of!(StageData, bumper_z), 0x30);
        assert_eq!(offset_of!(StageData, bumper_x_base), 0x34);
        assert_eq!(offset_of!(StageData, bumper_x_range), 0x38);
        assert_eq!(offset_of!(StageData, bumper_loop_length), 0x3C);
        assert_eq!(offset_of!(StageData, bumper_wait_length), 0x40);
        assert_eq!(offset_of!(StageData, bumper_wait_animation_frame), 0x44);
        assert_eq!(offset_of!(StageData, bumper_attack_animation_start_frame), 0x48);
        assert_eq!(offset_of!(StageData, bumper_attack_length), 0x4C);
        assert_eq!(offset_of!(StageData, floor_moving_intervals), 0x50);
        assert_eq!(offset_of!(StageData, cliff_validity_ring_l), 0x68);
        assert_eq!(offset_of!(StageData, cliff_validity_ring_r), 0x74);
        assert_eq!(offset_of!(StageData, cliff_validity_floor_l), 0x80);
        assert_eq!(offset_of!(StageData, cliff_validity_floor_r), 0x8C);
        assert_eq!(offset_of!(StageData, camera_sub_pos), 0x98);

        assert_eq!(size_of::<FloorMovingInterval>(), 0x8);
        assert_eq!(offset_of!(FloorMovingInterval, start), 0x0);
        assert_eq!(offset_of!(FloorMovingInterval, end), 0x4);

        assert_eq!(size_of::<CliffValidity>(), 0xC);
        assert_eq!(offset_of!(CliffValidity, validity), 0x0);
        assert_eq!(offset_of!(CliffValidity, start), 0x4);
        assert_eq!(offset_of!(CliffValidity, end), 0x8);

        assert_eq!(size_of::<CameraSubPos>(), 0xC);
        assert_eq!(offset_of!(CameraSubPos, x), 0x0);
        assert_eq!(offset_of!(CameraSubPos, y), 0x4);
        assert_eq!(offset_of!(CameraSubPos, z), 0x8);
    }
}
