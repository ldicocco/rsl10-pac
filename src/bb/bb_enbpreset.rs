#[doc = "Reader of register BB_ENBPRESET"]
pub type R = crate::R<u32, super::BB_ENBPRESET>;
#[doc = "Writer for register BB_ENBPRESET"]
pub type W = crate::W<u32, super::BB_ENBPRESET>;
#[doc = "Register BB_ENBPRESET `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_ENBPRESET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Time in low power oscillator cycles allowed for stabilization of the high frequency oscillator when the deep-sleep mode has been left due to sleep-timer expiry (DEEPSLWKUP-DEEPSLTIME\\])\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum TWOSC_A {
    #[doc = "0: `0`"]
    TWOSC_0 = 0,
}
impl From<TWOSC_A> for u16 {
    #[inline(always)]
    fn from(variant: TWOSC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TWOSC`"]
pub type TWOSC_R = crate::R<u16, TWOSC_A>;
impl TWOSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, TWOSC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TWOSC_A::TWOSC_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TWOSC_0`"]
    #[inline(always)]
    pub fn is_twosc_0(&self) -> bool {
        *self == TWOSC_A::TWOSC_0
    }
}
#[doc = "Write proxy for field `TWOSC`"]
pub struct TWOSC_W<'a> {
    w: &'a mut W,
}
impl<'a> TWOSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWOSC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn twosc_0(self) -> &'a mut W {
        self.variant(TWOSC_A::TWOSC_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 10)) | (((value as u32) & 0x07ff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:20 - Time in low power oscillator cycles allowed for stabilization of the high frequency oscillator when the deep-sleep mode has been left due to sleep-timer expiry (DEEPSLWKUP-DEEPSLTIME\\])"]
    #[inline(always)]
    pub fn twosc(&self) -> TWOSC_R {
        TWOSC_R::new(((self.bits >> 10) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 10:20 - Time in low power oscillator cycles allowed for stabilization of the high frequency oscillator when the deep-sleep mode has been left due to sleep-timer expiry (DEEPSLWKUP-DEEPSLTIME\\])"]
    #[inline(always)]
    pub fn twosc(&mut self) -> TWOSC_W {
        TWOSC_W { w: self }
    }
}
