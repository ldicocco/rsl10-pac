#[doc = "Reader of register RF_REVISION"]
pub type R = crate::R<u32, super::RF_REVISION>;
#[doc = "Reader of field `CHIP_ID`"]
pub type CHIP_ID_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:29 - Version of the chip: 0x00: v1, 0x10: v2A, 0x11: v2B, 0x12: v2C, 0x13: v2D, 0x14: v2E, 0x20: v3"]
    #[inline(always)]
    pub fn chip_id(&self) -> CHIP_ID_R {
        CHIP_ID_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
