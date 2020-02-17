#[doc = "Reader of register CRC_FINAL"]
pub type R = crate::R<u32, super::CRC_FINAL>;
#[doc = "Reader of field `FINAL_CRC`"]
pub type FINAL_CRC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC generator final value: After XOR for CCITT or byte reversal for CRC-32"]
    #[inline(always)]
    pub fn final_crc(&self) -> FINAL_CRC_R {
        FINAL_CRC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
