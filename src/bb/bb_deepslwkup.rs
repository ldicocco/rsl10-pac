#[doc = "Reader of register BB_DEEPSLWKUP"]
pub type R = crate::R<u32, super::BB_DEEPSLWKUP>;
#[doc = "Writer for register BB_DEEPSLWKUP"]
pub type W = crate::W<u32, super::BB_DEEPSLWKUP>;
#[doc = "Register BB_DEEPSLWKUP `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_DEEPSLWKUP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Determines the time in low_power_clk clock cycles to spend in deep sleep mode before waking-up the device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum DEEPSLTIME_A {
    #[doc = "0: `0`"]
    DEEPSLTIME_0 = 0,
}
impl From<DEEPSLTIME_A> for u32 {
    #[inline(always)]
    fn from(variant: DEEPSLTIME_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DEEPSLTIME`"]
pub type DEEPSLTIME_R = crate::R<u32, DEEPSLTIME_A>;
impl DEEPSLTIME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, DEEPSLTIME_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DEEPSLTIME_A::DEEPSLTIME_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEEPSLTIME_0`"]
    #[inline(always)]
    pub fn is_deepsltime_0(&self) -> bool {
        *self == DEEPSLTIME_A::DEEPSLTIME_0
    }
}
#[doc = "Write proxy for field `DEEPSLTIME`"]
pub struct DEEPSLTIME_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEPSLTIME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEEPSLTIME_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn deepsltime_0(self) -> &'a mut W {
        self.variant(DEEPSLTIME_A::DEEPSLTIME_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Determines the time in low_power_clk clock cycles to spend in deep sleep mode before waking-up the device"]
    #[inline(always)]
    pub fn deepsltime(&self) -> DEEPSLTIME_R {
        DEEPSLTIME_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Determines the time in low_power_clk clock cycles to spend in deep sleep mode before waking-up the device"]
    #[inline(always)]
    pub fn deepsltime(&mut self) -> DEEPSLTIME_W {
        DEEPSLTIME_W { w: self }
    }
}
