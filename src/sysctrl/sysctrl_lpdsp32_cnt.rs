#[doc = "Reader of register SYSCTRL_LPDSP32_CNT"]
pub type R = crate::R<u32, super::SYSCTRL_LPDSP32_CNT>;
#[doc = "Writer for register SYSCTRL_LPDSP32_CNT"]
pub type W = crate::W<u32, super::SYSCTRL_LPDSP32_CNT>;
#[doc = "Register SYSCTRL_LPDSP32_CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCTRL_LPDSP32_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPDSP32_CNT`"]
pub type LPDSP32_CNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LPDSP32_CNT`"]
pub struct LPDSP32_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDSP32_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - LPDSP32 activity counter value"]
    #[inline(always)]
    pub fn lpdsp32_cnt(&self) -> LPDSP32_CNT_R {
        LPDSP32_CNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - LPDSP32 activity counter value"]
    #[inline(always)]
    pub fn lpdsp32_cnt(&mut self) -> LPDSP32_CNT_W {
        LPDSP32_CNT_W { w: self }
    }
}
