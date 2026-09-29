#[repr(C)]
pub struct StageXenoAlst {
    _0: [u8; 0x740],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum Guest {
    /// Mòrag and Brighid
    #[default]
    MelephKagutsuchi,

    /// Nia and Dromarch
    NiyahByakko,

    /// Tora and Poppi
    ToraHana,

    /// Zeke and Pandoria
    ZekeSaika,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RandomSetting {
    pub guest_info: u8,
    pub scene_start_frame_index: u8,
}

impl RandomSetting {
    pub const fn guest(self) -> Option<Guest> {
        match self.guest_info & 0b00001111 {
            0 => Some(Guest::MelephKagutsuchi),
            1 => Some(Guest::NiyahByakko),
            2 => Some(Guest::ToraHana),
            3 => Some(Guest::ZekeSaika),
            _ => None,
        }
    }

    pub const fn guest_position_id(self) -> u8 {
        (self.guest_info & 0b11110000) >> 4
    }

    pub const fn set_guest(&mut self, guest: Guest) {
        self.guest_info = (self.guest_info & !0b00001111) | guest as u8;
    }

    pub const fn set_guest_position_id(&mut self, position_id: u8) {
        self.guest_info = (self.guest_info & !0b11110000) | (position_id << 4);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageXenoAlst>(), 0x740);
        assert_eq!(offset_of!(StageXenoAlst, _0), 0x0);
    }
}
