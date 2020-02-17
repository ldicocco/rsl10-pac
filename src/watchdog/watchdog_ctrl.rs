#[doc = "Reader of register WATCHDOG_CTRL"]
pub type R = crate::R<u32, super::WATCHDOG_CTRL>;
#[doc = "Writer for register WATCHDOG_CTRL"]
pub type W = crate::W<u32, super::WATCHDOG_CTRL>;
#[doc = "Register WATCHDOG_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::WATCHDOG_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write a key to reset the watchdog\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum WATCHDOG_REFRESH_AW {
    #[doc = "45212177: Write 32-bit key to reset the watchdog (others values have no effect)"]
    WATCHDOG_REFRESH = 45212177,
}
impl From<WATCHDOG_REFRESH_AW> for u32 {
    #[inline(always)]
    fn from(variant: WATCHDOG_REFRESH_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `WATCHDOG_REFRESH`"]
pub struct WATCHDOG_REFRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> WATCHDOG_REFRESH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WATCHDOG_REFRESH_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Write 32-bit key to reset the watchdog (others values have no effect)"]
    #[inline(always)]
    pub fn watchdog_refresh(self) -> &'a mut W {
        self.variant(WATCHDOG_REFRESH_AW::WATCHDOG_REFRESH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bits 0:31 - Write a key to reset the watchdog"]
    #[inline(always)]
    pub fn watchdog_refresh(&mut self) -> WATCHDOG_REFRESH_W {
        WATCHDOG_REFRESH_W { w: self }
    }
}
