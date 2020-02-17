#[doc = "Reader of register RF_SYNC_PATTERN"]
pub type R = crate::R<u32, super::RF_SYNC_PATTERN>;
#[doc = "Writer for register RF_SYNC_PATTERN"]
pub type W = crate::W<u32, super::RF_SYNC_PATTERN>;
#[doc = "Register RF_SYNC_PATTERN `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_SYNC_PATTERN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pattern (sync word) to be inserted or recognized.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PATTERN_A {
    #[doc = "0: `0`"]
    PATTERN_DEFAULT = 0,
}
impl From<PATTERN_A> for u32 {
    #[inline(always)]
    fn from(variant: PATTERN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PATTERN`"]
pub type PATTERN_R = crate::R<u32, PATTERN_A>;
impl PATTERN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PATTERN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PATTERN_A::PATTERN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PATTERN_DEFAULT`"]
    #[inline(always)]
    pub fn is_pattern_default(&self) -> bool {
        *self == PATTERN_A::PATTERN_DEFAULT
    }
}
#[doc = "Write proxy for field `PATTERN`"]
pub struct PATTERN_W<'a> {
    w: &'a mut W,
}
impl<'a> PATTERN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PATTERN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pattern_default(self) -> &'a mut W {
        self.variant(PATTERN_A::PATTERN_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pattern (sync word) to be inserted or recognized."]
    #[inline(always)]
    pub fn pattern(&self) -> PATTERN_R {
        PATTERN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pattern (sync word) to be inserted or recognized."]
    #[inline(always)]
    pub fn pattern(&mut self) -> PATTERN_W {
        PATTERN_W { w: self }
    }
}
