use crate::phx::ParameterReceiverBase;

/// The specialized parameters for the Kongo Jungle stage.
#[repr(C)]
pub struct StageData {
    pub base: ParameterReceiverBase,

    /// The minimum amount of time a flock of Neckys wait to appear, in frames.
    pub rframe_bird_wait_a: f32,

    /// The maximum amount of time a flock of Neckys wait to appear, in frames.
    pub rframe_bird_wait_b: f32,

    /// The maximum displacement of a Necky along the y-axis.
    pub rrange_bird_random_offset_y: f32,

    /// The position of a Necky along the x-axis.
    pub rpos_bird_x: f32,

    /// The position of a Necky along the y-axis.
    pub rpos_bird_y: f32,

    /// The position of a Necky along the z-axis.
    pub rpos_bird_z: f32,

    /// The distance between each Necky along the x-axis.
    pub rpos_bird_interval_x: f32,

    /// The probability of two Neckys appearing in a flock.
    pub rrate_bird_appear_2: f32,

    /// The probability of three Neckys appearing in a flock.
    pub rrate_bird_appear_3: f32,

    /// The minimum amount of time a flock of Neckys wait to appear on the stage's Ω form, in frames.
    ///
    /// This parameter goes unused in the final game, but is used in Super Smash Bros. for Wii U.
    pub rframe_bird_wait_a_end: f32,

    /// The maximum amount of time a flock of Neckys wait to appear on the stage's Ω form, in frames.
    ///
    /// This parameter goes unused in the final game, but is used in Super Smash Bros. for Wii U.
    pub rframe_bird_wait_b_end: f32,

    /// The maximum displacement of a Necky along the y-axis on the stage's Ω form.
    ///
    /// This parameter goes unused in the final game, but is used in Super Smash Bros. for Wii U.
    pub rrange_bird_random_offset_y_end: f32,

    /// The position of a Necky along the x-axis on the stage's Ω form.
    ///
    /// This parameter goes unused in the final game, but is used in Super Smash Bros. for Wii U.
    pub rpos_bird_x_end: f32,

    /// The position of a Necky along the y-axis on the stage's Ω form.
    ///
    /// This parameter goes unused in the final game, but is used in Super Smash Bros. for Wii U.
    pub rpos_bird_y_end: f32,

    /// The position of a Necky along the z-axis on the stage's Ω form.
    ///
    /// This parameter goes unused in the final game, but is used in Super Smash Bros. for Wii U.
    pub rpos_bird_z_end: f32,

    /// The distance between each Necky on the stage's Ω form.
    ///
    /// This parameter goes unused in the final game, but is used in Super Smash Bros. for Wii U.
    pub rpos_bird_interval_x_end: f32,

    /// The probability of two Neckys appearing in a flock on the stage's Ω form.
    ///
    /// This parameter goes unused in the final game, but is used in Super Smash Bros. for Wii U.
    pub rrate_bird_appear_2_end: f32,

    /// The probability of three Neckys appearing in a flock on the stage's Ω form.
    ///
    /// This parameter goes unused in the final game, but is used in Super Smash Bros. for Wii U.
    pub rrate_bird_appear_3_end: f32,

    /// The minimum amount of time that the barrel cannon waits before automatically firing the contained fighter, in frames.
    pub rframe_barrel_shoot_a: f32,

    /// The maximum amount of time that the barrel cannon waits before automatically firing the contained fighter, in frames.
    pub rframe_barrel_shoot_b: f32,

    /// The width and height of the barrel's search collision.
    ///
    /// This parameter goes unused in the final game.
    pub rrange_barrel_in: f32,

    /// The minimum amount of time that the barrel cannon waits before rotating, in frames.
    pub rframe_barrel_wait_a: f32,

    /// The maximum amount of time that the barrel cannon waits before rotating, in frames.
    pub rframe_barrel_wait_b: f32,

    /// The angular acceleration of the barrel cannon when rotating around the z-axis.
    pub rspeed_barrel_rot_accel: f32,

    /// The maximum angular velocity of the barrel cannon when rotating around the z-axis.
    pub rspeed_barrel_rot_max: f32,

    /// The minimum amount of time that the barrel cannon rotates for, in frames.
    pub rframe_barrel_roll_a: f32,

    /// The maximum amount of time that the barrel cannon rotates for, in frames.
    pub rframe_barrel_roll_b: f32,

    /// The probability of the barrel cannon facing downward and diagonally to the left after the rotation period.
    pub rrate_barrel_ld: f32,

    /// The probability of the barrel cannon facing left after the rotation period.
    pub rrate_barrel_l: f32,

    /// The probability of the barrel cannon facing upward and diagonally to the left after the rotation period.
    pub rrate_barrel_lu: f32,

    /// The probability of the barrel cannon facing up after the rotation period.
    pub rrate_barrel_u: f32,

    /// The probability of the barrel cannon facing upward and diagonally to the right after the rotation period.
    pub rrate_barrel_ru: f32,

    /// The probability of the barrel cannon facing right after the rotation period.
    pub rrate_barrel_r: f32,

    /// The probability of the barrel cannon facing downward and diagonally to the right after the rotation period.
    pub rrate_barrel_rd: f32,

    /// The probability of the barrel cannon facing down after the rotation period.
    pub rrate_barrel_d: f32,

    /// The minimum amount of time the barrel cannon stops moving for, in frames.
    ///
    /// The behavior that this parameter is associated with is removed in the final game.
    pub rframe_barrel_interval_a: f32,

    /// The maximum amount of time the barrel cannon stops moving for, in frames.
    ///
    /// The behavior that this parameter is associated with is removed in the final game.
    pub rframe_barrel_interval_b: f32,

    /// The acceleration and deceleration of the barrel cannon along the x-axis when starting and stopping movement, respectively.
    ///
    /// The behavior that this parameter is associated with is removed in the final game.
    pub rspeed_barrel_move_accel: f32,

    /// The speed of the barrel cannon's movement.
    ///
    /// The behavior that this parameter is associated with is removed in the final game.
    pub rspeed_barrel_move_max: f32,

    /// The minimum amount of time that the barrel cannon waits before decelerating its movement, in frames.
    ///
    /// The behavior that this parameter is associated with is removed in the final game.
    pub rframe_barrel_move_stop_a: f32,

    /// The maximum amount of time that the barrel cannon waits before decelerating its movement, in frames.
    ///
    /// The behavior that this parameter is associated with is removed in the final game.
    pub rframe_barrel_move_stop_b: f32,

    /// The amount of damage to be inflicted by the fighter fired from the barrel cannon.
    pub rpower_barrel_attack: f32,

    /// The angle of knockback to be inflicted by the fighter fired from the barrel cannon, in degrees.
    pub rvector_barrel_attack: f32,

    /// The coefficient for scaling knockback to be inflicted by the fighter fired from the barrel cannon.
    pub rreff_barrel_attack: f32,

    /// The fixed amount of knockback to be inflicted by the fighter fired from the barrel cannon.
    pub rrfix_barrel_attack: f32,

    /// The minimum amount of knockback to be inflicted by the fighter fired from the barrel cannon.
    pub rradd_barrel_attack: f32,

    /// The probability of the fighter fired from the barrel cannon knocking over an attacked opponent.
    pub rslip_barrel_attack: f32,

    /// The coefficient for scaling hitstun to be inflicted by the fighter fired from the barrel cannon.
    pub rhit_stop_frame_barrel_attack: f32,

    /// The coefficient for scaling the shuffling from hitstun to be inflicted by the fighter fired from the barrel cannon.
    pub rhit_stop_delay_barrel_attack: f32,

    /// The initial speed of the fighter fired from the barrel cannon.
    pub rspeed_barrel_shoot: f32,

    /// The deceleration of the fighter fired from the barrel cannon.
    pub rdecrease_barrel_shoot: f32,

    /// The gravitational acceleration of the fighter fired from the barrel cannon.
    pub rgravity_barrel_shoot: f32,

    // TODO: Affects when control is returned to the player after their fighter is fired from the barrel cannon.
    pub rspeed_to_fall_barrel_shoot: f32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use std::mem::offset_of;

        assert_eq!(size_of::<StageData>(), 0xE0);
        assert_eq!(offset_of!(StageData, base), 0x0);
        assert_eq!(offset_of!(StageData, rframe_bird_wait_a), 0x8);
        assert_eq!(offset_of!(StageData, rframe_bird_wait_b), 0xC);
        assert_eq!(offset_of!(StageData, rrange_bird_random_offset_y), 0x10);
        assert_eq!(offset_of!(StageData, rpos_bird_x), 0x14);
        assert_eq!(offset_of!(StageData, rpos_bird_y), 0x18);
        assert_eq!(offset_of!(StageData, rpos_bird_z), 0x1C);
        assert_eq!(offset_of!(StageData, rpos_bird_interval_x), 0x20);
        assert_eq!(offset_of!(StageData, rrate_bird_appear_2), 0x24);
        assert_eq!(offset_of!(StageData, rrate_bird_appear_3), 0x28);
        assert_eq!(offset_of!(StageData, rframe_bird_wait_a_end), 0x2C);
        assert_eq!(offset_of!(StageData, rframe_bird_wait_b_end), 0x30);
        assert_eq!(offset_of!(StageData, rrange_bird_random_offset_y_end), 0x34);
        assert_eq!(offset_of!(StageData, rpos_bird_x_end), 0x38);
        assert_eq!(offset_of!(StageData, rpos_bird_y_end), 0x3C);
        assert_eq!(offset_of!(StageData, rpos_bird_z_end), 0x40);
        assert_eq!(offset_of!(StageData, rpos_bird_interval_x_end), 0x44);
        assert_eq!(offset_of!(StageData, rrate_bird_appear_2_end), 0x48);
        assert_eq!(offset_of!(StageData, rrate_bird_appear_3_end), 0x4C);
        assert_eq!(offset_of!(StageData, rframe_barrel_shoot_a), 0x50);
        assert_eq!(offset_of!(StageData, rframe_barrel_shoot_b), 0x54);
        assert_eq!(offset_of!(StageData, rrange_barrel_in), 0x58);
        assert_eq!(offset_of!(StageData, rframe_barrel_wait_a), 0x5C);
        assert_eq!(offset_of!(StageData, rframe_barrel_wait_b), 0x60);
        assert_eq!(offset_of!(StageData, rspeed_barrel_rot_accel), 0x64);
        assert_eq!(offset_of!(StageData, rspeed_barrel_rot_max), 0x68);
        assert_eq!(offset_of!(StageData, rframe_barrel_roll_a), 0x6C);
        assert_eq!(offset_of!(StageData, rframe_barrel_roll_b), 0x70);
        assert_eq!(offset_of!(StageData, rrate_barrel_ld), 0x74);
        assert_eq!(offset_of!(StageData, rrate_barrel_l), 0x78);
        assert_eq!(offset_of!(StageData, rrate_barrel_lu), 0x7C);
        assert_eq!(offset_of!(StageData, rrate_barrel_u), 0x80);
        assert_eq!(offset_of!(StageData, rrate_barrel_ru), 0x84);
        assert_eq!(offset_of!(StageData, rrate_barrel_r), 0x88);
        assert_eq!(offset_of!(StageData, rrate_barrel_rd), 0x8C);
        assert_eq!(offset_of!(StageData, rrate_barrel_d), 0x90);
        assert_eq!(offset_of!(StageData, rframe_barrel_interval_a), 0x94);
        assert_eq!(offset_of!(StageData, rframe_barrel_interval_b), 0x98);
        assert_eq!(offset_of!(StageData, rspeed_barrel_move_accel), 0x9C);
        assert_eq!(offset_of!(StageData, rspeed_barrel_move_max), 0xA0);
        assert_eq!(offset_of!(StageData, rframe_barrel_move_stop_a), 0xA4);
        assert_eq!(offset_of!(StageData, rframe_barrel_move_stop_b), 0xA8);
        assert_eq!(offset_of!(StageData, rpower_barrel_attack), 0xAC);
        assert_eq!(offset_of!(StageData, rvector_barrel_attack), 0xB0);
        assert_eq!(offset_of!(StageData, rreff_barrel_attack), 0xB4);
        assert_eq!(offset_of!(StageData, rrfix_barrel_attack), 0xB8);
        assert_eq!(offset_of!(StageData, rradd_barrel_attack), 0xBC);
        assert_eq!(offset_of!(StageData, rslip_barrel_attack), 0xC0);
        assert_eq!(offset_of!(StageData, rhit_stop_frame_barrel_attack), 0xC4);
        assert_eq!(offset_of!(StageData, rhit_stop_delay_barrel_attack), 0xC8);
        assert_eq!(offset_of!(StageData, rspeed_barrel_shoot), 0xCC);
        assert_eq!(offset_of!(StageData, rdecrease_barrel_shoot), 0xD0);
        assert_eq!(offset_of!(StageData, rgravity_barrel_shoot), 0xD4);
        assert_eq!(offset_of!(StageData, rspeed_to_fall_barrel_shoot), 0xD8);
    }
}
