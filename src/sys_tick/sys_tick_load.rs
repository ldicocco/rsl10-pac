#[doc = "Reader of register SysTick_LOAD"]
pub type R = crate::R<u32, super::SYSTICK_LOAD>;
#[doc = "Writer for register SysTick_LOAD"]
pub type W = crate::W<u32, super::SYSTICK_LOAD>;
#[doc = "Register SysTick_LOAD `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTICK_LOAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RELOAD`"]
pub type RELOAD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RELOAD`"]
pub struct RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Counter reload value for the SYSTICK timer when it reaches 0"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Counter reload value for the SYSTICK timer when it reaches 0"]
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W {
        RELOAD_W { w: self }
    }
}
