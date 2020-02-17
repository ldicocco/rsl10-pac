#[doc = "Reader of register ASRC_PHASE_CNT"]
pub type R = crate::R<u32, super::ASRC_PHASE_CNT>;
#[doc = "Writer for register ASRC_PHASE_CNT"]
pub type W = crate::W<u32, super::ASRC_PHASE_CNT>;
#[doc = "Register ASRC_PHASE_CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::ASRC_PHASE_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASRC_PHASE_CNT`"]
pub type ASRC_PHASE_CNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ASRC_PHASE_CNT`"]
pub struct ASRC_PHASE_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> ASRC_PHASE_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ASRC phase counter"]
    #[inline(always)]
    pub fn asrc_phase_cnt(&self) -> ASRC_PHASE_CNT_R {
        ASRC_PHASE_CNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ASRC phase counter"]
    #[inline(always)]
    pub fn asrc_phase_cnt(&mut self) -> ASRC_PHASE_CNT_W {
        ASRC_PHASE_CNT_W { w: self }
    }
}
