#[doc = "Reader of register ASRC_PHASE_INC"]
pub type R = crate::R<u32, super::ASRC_PHASE_INC>;
#[doc = "Writer for register ASRC_PHASE_INC"]
pub type W = crate::W<u32, super::ASRC_PHASE_INC>;
#[doc = "Register ASRC_PHASE_INC `reset()`'s with value 0"]
impl crate::ResetValue for super::ASRC_PHASE_INC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASRC_STEP`"]
pub type ASRC_STEP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ASRC_STEP`"]
pub struct ASRC_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> ASRC_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ASRC_PHASE_INC"]
    #[inline(always)]
    pub fn asrc_step(&self) -> ASRC_STEP_R {
        ASRC_STEP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ASRC_PHASE_INC"]
    #[inline(always)]
    pub fn asrc_step(&mut self) -> ASRC_STEP_W {
        ASRC_STEP_W { w: self }
    }
}
