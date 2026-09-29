#[repr(C, align(8))]
pub struct String {
    inner: [u8; 0x18],
}
