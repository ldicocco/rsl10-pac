#[doc = "Reader of register CRC_ADD_32"]
pub type R = crate::R<u32, super::CRC_ADD_32>;
#[doc = "Writer for register CRC_ADD_32"]
pub type W = crate::W<u32, super::CRC_ADD_32>;
#[doc = "Register CRC_ADD_32 `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC_ADD_32 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CRC_ADD_32`"]
pub struct CRC_ADD_32_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_ADD_32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bits 0:31 - Add 1 word (32 bits) to the CRC calculation"]
    #[inline(always)]
    pub fn crc_add_32(&mut self) -> CRC_ADD_32_W {
        CRC_ADD_32_W { w: self }
    }
}
