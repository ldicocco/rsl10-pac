#[doc = "Reader of register BB_RFTESTTXSTAT"]
pub type R = crate::R<u32, super::BB_RFTESTTXSTAT>;
#[doc = "Writer for register BB_RFTESTTXSTAT"]
pub type W = crate::W<u32, super::BB_RFTESTTXSTAT>;
#[doc = "Register BB_RFTESTTXSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_RFTESTTXSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reports number of transmitted packet during test modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum TXPKTCNT_A {
    #[doc = "0: `0`"]
    TXPKTCNT_00 = 0,
}
impl From<TXPKTCNT_A> for u32 {
    #[inline(always)]
    fn from(variant: TXPKTCNT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXPKTCNT`"]
pub type TXPKTCNT_R = crate::R<u32, TXPKTCNT_A>;
impl TXPKTCNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, TXPKTCNT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXPKTCNT_A::TXPKTCNT_00),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXPKTCNT_00`"]
    #[inline(always)]
    pub fn is_txpktcnt_00(&self) -> bool {
        *self == TXPKTCNT_A::TXPKTCNT_00
    }
}
#[doc = "Write proxy for field `TXPKTCNT`"]
pub struct TXPKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPKTCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPKTCNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn txpktcnt_00(self) -> &'a mut W {
        self.variant(TXPKTCNT_A::TXPKTCNT_00)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Reports number of transmitted packet during test modes"]
    #[inline(always)]
    pub fn txpktcnt(&self) -> TXPKTCNT_R {
        TXPKTCNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reports number of transmitted packet during test modes"]
    #[inline(always)]
    pub fn txpktcnt(&mut self) -> TXPKTCNT_W {
        TXPKTCNT_W { w: self }
    }
}
