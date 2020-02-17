#[doc = "Reader of register BB_BASETIMECNTCORR"]
pub type R = crate::R<u32, super::BB_BASETIMECNTCORR>;
#[doc = "Writer for register BB_BASETIMECNTCORR"]
pub type W = crate::W<u32, super::BB_BASETIMECNTCORR>;
#[doc = "Register BB_BASETIMECNTCORR `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_BASETIMECNTCORR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Base time counter correction value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum BASETIMECNTCORR_A {
    #[doc = "0: `0`"]
    BASETIMECNTCORR_0 = 0,
}
impl From<BASETIMECNTCORR_A> for u32 {
    #[inline(always)]
    fn from(variant: BASETIMECNTCORR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BASETIMECNTCORR`"]
pub type BASETIMECNTCORR_R = crate::R<u32, BASETIMECNTCORR_A>;
impl BASETIMECNTCORR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, BASETIMECNTCORR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BASETIMECNTCORR_A::BASETIMECNTCORR_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BASETIMECNTCORR_0`"]
    #[inline(always)]
    pub fn is_basetimecntcorr_0(&self) -> bool {
        *self == BASETIMECNTCORR_A::BASETIMECNTCORR_0
    }
}
#[doc = "Write proxy for field `BASETIMECNTCORR`"]
pub struct BASETIMECNTCORR_W<'a> {
    w: &'a mut W,
}
impl<'a> BASETIMECNTCORR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BASETIMECNTCORR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn basetimecntcorr_0(self) -> &'a mut W {
        self.variant(BASETIMECNTCORR_A::BASETIMECNTCORR_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff_ffff) | ((value as u32) & 0x07ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:26 - Base time counter correction value"]
    #[inline(always)]
    pub fn basetimecntcorr(&self) -> BASETIMECNTCORR_R {
        BASETIMECNTCORR_R::new((self.bits & 0x07ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:26 - Base time counter correction value"]
    #[inline(always)]
    pub fn basetimecntcorr(&mut self) -> BASETIMECNTCORR_W {
        BASETIMECNTCORR_W { w: self }
    }
}
