#[doc = "Reader of register BB_FINECNTCORR"]
pub type R = crate::R<u32, super::BB_FINECNTCORR>;
#[doc = "Writer for register BB_FINECNTCORR"]
pub type W = crate::W<u32, super::BB_FINECNTCORR>;
#[doc = "Register BB_FINECNTCORR `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_FINECNTCORR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Phase correction value for the 625us reference counter (i.e. fine counter) in us\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum FINECNTCORR_A {
    #[doc = "0: `0`"]
    FINECNTCORR_0 = 0,
}
impl From<FINECNTCORR_A> for u16 {
    #[inline(always)]
    fn from(variant: FINECNTCORR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FINECNTCORR`"]
pub type FINECNTCORR_R = crate::R<u16, FINECNTCORR_A>;
impl FINECNTCORR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, FINECNTCORR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FINECNTCORR_A::FINECNTCORR_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FINECNTCORR_0`"]
    #[inline(always)]
    pub fn is_finecntcorr_0(&self) -> bool {
        *self == FINECNTCORR_A::FINECNTCORR_0
    }
}
#[doc = "Write proxy for field `FINECNTCORR`"]
pub struct FINECNTCORR_W<'a> {
    w: &'a mut W,
}
impl<'a> FINECNTCORR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FINECNTCORR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn finecntcorr_0(self) -> &'a mut W {
        self.variant(FINECNTCORR_A::FINECNTCORR_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Phase correction value for the 625us reference counter (i.e. fine counter) in us"]
    #[inline(always)]
    pub fn finecntcorr(&self) -> FINECNTCORR_R {
        FINECNTCORR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Phase correction value for the 625us reference counter (i.e. fine counter) in us"]
    #[inline(always)]
    pub fn finecntcorr(&mut self) -> FINECNTCORR_W {
        FINECNTCORR_W { w: self }
    }
}
