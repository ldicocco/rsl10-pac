#[doc = "Reader of register ASRC_OUTPUT_CNT"]
pub type R = crate::R<u32, super::ASRC_OUTPUT_CNT>;
#[doc = "Writer for register ASRC_OUTPUT_CNT"]
pub type W = crate::W<u32, super::ASRC_OUTPUT_CNT>;
#[doc = "Register ASRC_OUTPUT_CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::ASRC_OUTPUT_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASRC_OUTPUT_CNT`"]
pub type ASRC_OUTPUT_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ASRC_OUTPUT_CNT`"]
pub struct ASRC_OUTPUT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> ASRC_OUTPUT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - ASRC output sample counter"]
    #[inline(always)]
    pub fn asrc_output_cnt(&self) -> ASRC_OUTPUT_CNT_R {
        ASRC_OUTPUT_CNT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ASRC output sample counter"]
    #[inline(always)]
    pub fn asrc_output_cnt(&mut self) -> ASRC_OUTPUT_CNT_W {
        ASRC_OUTPUT_CNT_W { w: self }
    }
}
