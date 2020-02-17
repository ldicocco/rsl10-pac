#[doc = "Reader of register BB_ADVTIM"]
pub type R = crate::R<u32, super::BB_ADVTIM>;
#[doc = "Writer for register BB_ADVTIM"]
pub type W = crate::W<u32, super::BB_ADVTIM>;
#[doc = "Register BB_ADVTIM `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_ADVTIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Advertising packet interval defines the time interval in between two ADV_xxx packet sent (value in us)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ADVINT_A {
    #[doc = "0: `0`"]
    ADVINT_0 = 0,
}
impl From<ADVINT_A> for u16 {
    #[inline(always)]
    fn from(variant: ADVINT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADVINT`"]
pub type ADVINT_R = crate::R<u16, ADVINT_A>;
impl ADVINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ADVINT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADVINT_A::ADVINT_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADVINT_0`"]
    #[inline(always)]
    pub fn is_advint_0(&self) -> bool {
        *self == ADVINT_A::ADVINT_0
    }
}
#[doc = "Write proxy for field `ADVINT`"]
pub struct ADVINT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADVINT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn advint_0(self) -> &'a mut W {
        self.variant(ADVINT_A::ADVINT_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Advertising packet interval defines the time interval in between two ADV_xxx packet sent (value in us)"]
    #[inline(always)]
    pub fn advint(&self) -> ADVINT_R {
        ADVINT_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Advertising packet interval defines the time interval in between two ADV_xxx packet sent (value in us)"]
    #[inline(always)]
    pub fn advint(&mut self) -> ADVINT_W {
        ADVINT_W { w: self }
    }
}
