#[doc = "Reader of register SYSCTRL_SYSCLK_CNT"]
pub type R = crate::R<u32, super::SYSCTRL_SYSCLK_CNT>;
#[doc = "Writer for register SYSCTRL_SYSCLK_CNT"]
pub type W = crate::W<u32, super::SYSCTRL_SYSCLK_CNT>;
#[doc = "Register SYSCTRL_SYSCLK_CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCTRL_SYSCLK_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCLK_CNT`"]
pub type SYSCLK_CNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SYSCLK_CNT`"]
pub struct SYSCLK_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCLK_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - System clock counter value"]
    #[inline(always)]
    pub fn sysclk_cnt(&self) -> SYSCLK_CNT_R {
        SYSCLK_CNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - System clock counter value"]
    #[inline(always)]
    pub fn sysclk_cnt(&mut self) -> SYSCLK_CNT_W {
        SYSCLK_CNT_W { w: self }
    }
}
