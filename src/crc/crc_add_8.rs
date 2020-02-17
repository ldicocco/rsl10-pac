#[doc = "Reader of register CRC_ADD_8"]
pub type R = crate::R<u32, super::CRC_ADD_8>;
#[doc = "Writer for register CRC_ADD_8"]
pub type W = crate::W<u32, super::CRC_ADD_8>;
#[doc = "Register CRC_ADD_8 `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC_ADD_8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CRC_ADD_8`"]
pub struct CRC_ADD_8_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_ADD_8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bits 0:7 - Add 1 byte (8 bits) to the CRC calculation"]
    #[inline(always)]
    pub fn crc_add_8(&mut self) -> CRC_ADD_8_W {
        CRC_ADD_8_W { w: self }
    }
}
