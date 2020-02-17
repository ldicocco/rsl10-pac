#[doc = "Reader of register CRC_ADD_1"]
pub type R = crate::R<u32, super::CRC_ADD_1>;
#[doc = "Writer for register CRC_ADD_1"]
pub type W = crate::W<u32, super::CRC_ADD_1>;
#[doc = "Register CRC_ADD_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC_ADD_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CRC_ADD_1_BIT`"]
pub struct CRC_ADD_1_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_ADD_1_BIT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bit 0 - Add 1 bit to the CRC calculation"]
    #[inline(always)]
    pub fn crc_add_1_bit(&mut self) -> CRC_ADD_1_BIT_W {
        CRC_ADD_1_BIT_W { w: self }
    }
}
