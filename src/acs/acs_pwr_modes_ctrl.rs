#[doc = "Reader of register ACS_PWR_MODES_CTRL"]
pub type R = crate::R<u32, super::ACS_PWR_MODES_CTRL>;
#[doc = "Writer for register ACS_PWR_MODES_CTRL"]
pub type W = crate::W<u32, super::ACS_PWR_MODES_CTRL>;
#[doc = "Register ACS_PWR_MODES_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ACS_PWR_MODES_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "32-bit key to enter RUN, STANDBY or SLEEP mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum POWER_MODE_AW {
    #[doc = "0: Keep the system in normal RUN mode"]
    PWR_RUN_MODE = 0,
    #[doc = "2602400160: Enter STANDBY mode"]
    PWR_STANDBY_MODE = 2602400160,
    #[doc = "3758380624: Enter SLEEP mode"]
    PWR_SLEEP_MODE = 3758380624,
}
impl From<POWER_MODE_AW> for u32 {
    #[inline(always)]
    fn from(variant: POWER_MODE_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `POWER_MODE`"]
pub struct POWER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POWER_MODE_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Keep the system in normal RUN mode"]
    #[inline(always)]
    pub fn pwr_run_mode(self) -> &'a mut W {
        self.variant(POWER_MODE_AW::PWR_RUN_MODE)
    }
    #[doc = "Enter STANDBY mode"]
    #[inline(always)]
    pub fn pwr_standby_mode(self) -> &'a mut W {
        self.variant(POWER_MODE_AW::PWR_STANDBY_MODE)
    }
    #[doc = "Enter SLEEP mode"]
    #[inline(always)]
    pub fn pwr_sleep_mode(self) -> &'a mut W {
        self.variant(POWER_MODE_AW::PWR_SLEEP_MODE)
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
    #[doc = "Bits 0:31 - 32-bit key to enter RUN, STANDBY or SLEEP mode"]
    #[inline(always)]
    pub fn power_mode(&mut self) -> POWER_MODE_W {
        POWER_MODE_W { w: self }
    }
}
