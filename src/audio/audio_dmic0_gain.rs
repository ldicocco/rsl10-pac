#[doc = "Reader of register AUDIO_DMIC0_GAIN"]
pub type R = crate::R<u32, super::AUDIO_DMIC0_GAIN>;
#[doc = "Writer for register AUDIO_DMIC0_GAIN"]
pub type W = crate::W<u32, super::AUDIO_DMIC0_GAIN>;
#[doc = "Register AUDIO_DMIC0_GAIN `reset()`'s with value 0x0800"]
impl crate::ResetValue for super::AUDIO_DMIC0_GAIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0800
    }
}
#[doc = "DMIC calibration gain (unsigned value from 0 to +2)\n\nValue on reset: 2048"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum GAIN_A {
    #[doc = "2048: Nominal gain"]
    DMIC1_NOMINAL_GAIN = 2048,
}
impl From<GAIN_A> for u16 {
    #[inline(always)]
    fn from(variant: GAIN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN`"]
pub type GAIN_R = crate::R<u16, GAIN_A>;
impl GAIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, GAIN_A> {
        use crate::Variant::*;
        match self.bits {
            2048 => Val(GAIN_A::DMIC1_NOMINAL_GAIN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMIC1_NOMINAL_GAIN`"]
    #[inline(always)]
    pub fn is_dmic1_nominal_gain(&self) -> bool {
        *self == GAIN_A::DMIC1_NOMINAL_GAIN
    }
}
#[doc = "Write proxy for field `GAIN`"]
pub struct GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Nominal gain"]
    #[inline(always)]
    pub fn dmic1_nominal_gain(self) -> &'a mut W {
        self.variant(GAIN_A::DMIC1_NOMINAL_GAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - DMIC calibration gain (unsigned value from 0 to +2)"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DMIC calibration gain (unsigned value from 0 to +2)"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W { w: self }
    }
}
