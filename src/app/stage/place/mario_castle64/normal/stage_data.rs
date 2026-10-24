use crate::{cpp, phx::ParameterReceiverBase};

/// The specialized parameters for the Peach's Castle stage.
#[repr(C)]
pub struct StageData {
    pub base: ParameterReceiverBase,

    /// The radius of the bumper's search collision.
    pub bumper_search_shape_radius: f32,

    /// The radius of the bumper's attack collision.
    pub bumper_attack_shape_radius: f32,

    /// The amount of damage to be inflicted by the bumper.
    pub bumper_attack_power: f32,

    /// The angle of knockback to be inflicted by the bumper, in degrees.
    pub bumper_attack_vector: i32,

    /// The coefficient for scaling knockback to be inflicted by the bumper.
    pub bumper_attack_r_eff: i32,

    /// The fixed amount of knockback to be inflicted by the bumper.
    pub bumper_attack_r_fix: i32,

    /// The minimum amount of knockback to be inflicted by the bumper.
    pub bumper_attack_r_add: i32,

    /// The amount of time that the bumper must wait before it can damage an attacked opponent again, in frames.
    pub bumper_attack_serial_hit_frame: u32,

    /// The amount of time that the bumper's collision is intangible for after the attack period, in frames.
    pub bumper_attack_interval: f32,

    /// The position of the bumper along the y-axis.
    pub bumper_y: f32,

    /// The position of the bumper along the z-axis.
    pub bumper_z: f32,

    /// The home position of the bumper along the x-axis.
    pub bumper_x_base: f32,

    /// The length of the bumper's range of motion along the x-axis.
    pub bumper_x_range: f32,

    /// The amount of time that the bumper starts moving for, in frames.
    pub bumper_loop_length: f32,

    /// The amount of time that the bumper stops moving for, in frames.
    pub bumper_wait_length: f32,

    /// The animation frame index to stop animation playback on when the bumper is not attacking.
    pub bumper_wait_animation_frame: i32,

    /// The animation frame index to start animation playback on when the bumper is set to attack.
    pub bumper_attack_animation_start_frame: i32,

    /// The amount of time that the bumper's attack collision is tangible for after sensing a fighter, in frames.
    pub bumper_attack_length: f32,

    /// The collection of time intervals representing when the sliding platform is moving.
    pub floor_moving_intervals: cpp::Vector<FloorMovingInterval>,

    /// The parameters for controlling when the left side of the center platform can be grabbed.
    pub cliff_validity_ring_l: CliffValidity,

    /// The parameters for controlling when the right side of the center platform can be grabbed.
    pub cliff_validity_ring_r: CliffValidity,

    /// The parameters for controlling when the left side of the sliding platform can be grabbed.
    pub cliff_validity_floor_l: CliffValidity,

    /// The parameters for controlling when the right side of the sliding platform can be grabbed.
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
    /// Determines if the assigned edge can be grabbed during the specified interval of time.
    pub validity: bool,

    /// The animation frame index representing when to start using the [`validity`](#structfield.validity) state.
    pub start: f32,

    /// The animation frame index representing when to stop using the [`validity`](#structfield.validity) state.
    pub end: f32,
}

/// The set of parameters corresponding to the position of a `CameraSubject`.
#[repr(C)]
pub struct CameraSubPos {
    /// The position along the x-axis.
    pub x: f32,

    /// The position along the y-axis.
    pub y: f32,

    /// The position along the z-axis.
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
