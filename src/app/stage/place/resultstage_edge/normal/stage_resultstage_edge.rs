use crate::app::{StageBaseStorage, StageDrawModel};

/// The stage where the results take place when Sephiroth is the winner.
#[repr(C, align(16))]
pub struct StageResultStageEdge {
    pub base: StageBaseStorage,
    pub _738: [*mut StageDrawModel; 3],
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageResultStageEdge>(), 0x750);
        assert_eq!(offset_of!(StageResultStageEdge, base), 0x0);
        assert_eq!(offset_of!(StageResultStageEdge, _738), 0x738);
    }
}
