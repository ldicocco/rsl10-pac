#[doc = "Reader of register AUDIO_OD_DATA"]
pub type R = crate::R<u32, super::AUDIO_OD_DATA>;
#[doc = "Writer for register AUDIO_OD_DATA"]
pub type W = crate::W<u32, super::AUDIO_OD_DATA>;
#[doc = "Register AUDIO_OD_DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::AUDIO_OD_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
#[doc = "Reader of field `DATA_RD`"]
pub type DATA_RD_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - OD output data (LSB or MSB aligned according to OD_CFG); data is sign extended from 16-bit to 32-bit when read in LSB aligned mode or zero padded in MSB aligned mode"]
    #[inline(always)]
    pub fn data_rd(&self) -> DATA_RD_R {
        DATA_RD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - OD output data (LSB or MSB aligned according to OD_CFG); data is truncated to 16 bits when written in LSB aligned mode or rounded symmetrically with saturation when written in MSB aligned mode"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
}
