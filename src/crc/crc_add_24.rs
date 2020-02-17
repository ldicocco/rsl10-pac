#[doc = "Reader of register CRC_ADD_24"]
pub type R = crate::R<u32, super::CRC_ADD_24>;
#[doc = "Writer for register CRC_ADD_24"]
pub type W = crate::W<u32, super::CRC_ADD_24>;
#[doc = "Register CRC_ADD_24 `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC_ADD_24 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CRC_ADD_24_BITS`"]
pub struct CRC_ADD_24_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_ADD_24_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bits 0:23 - Add 3 bytes (24 bits) to the CRC calculation"]
    #[inline(always)]
    pub fn crc_add_24_bits(&mut self) -> CRC_ADD_24_BITS_W {
        CRC_ADD_24_BITS_W { w: self }
    }
}
