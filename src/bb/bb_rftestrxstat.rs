#[doc = "Reader of register BB_RFTESTRXSTAT"]
pub type R = crate::R<u32, super::BB_RFTESTRXSTAT>;
#[doc = "Writer for register BB_RFTESTRXSTAT"]
pub type W = crate::W<u32, super::BB_RFTESTRXSTAT>;
#[doc = "Register BB_RFTESTRXSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_RFTESTRXSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reports number of correctly received packet during test modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum RXPKTCNT_A {
    #[doc = "0: `0`"]
    RXPKTCNTX_0 = 0,
}
impl From<RXPKTCNT_A> for u32 {
    #[inline(always)]
    fn from(variant: RXPKTCNT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXPKTCNT`"]
pub type RXPKTCNT_R = crate::R<u32, RXPKTCNT_A>;
impl RXPKTCNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, RXPKTCNT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXPKTCNT_A::RXPKTCNTX_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RXPKTCNTX_0`"]
    #[inline(always)]
    pub fn is_rxpktcntx_0(&self) -> bool {
        *self == RXPKTCNT_A::RXPKTCNTX_0
    }
}
#[doc = "Write proxy for field `RXPKTCNT`"]
pub struct RXPKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPKTCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPKTCNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rxpktcntx_0(self) -> &'a mut W {
        self.variant(RXPKTCNT_A::RXPKTCNTX_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Reports number of correctly received packet during test modes"]
    #[inline(always)]
    pub fn rxpktcnt(&self) -> RXPKTCNT_R {
        RXPKTCNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reports number of correctly received packet during test modes"]
    #[inline(always)]
    pub fn rxpktcnt(&mut self) -> RXPKTCNT_W {
        RXPKTCNT_W { w: self }
    }
}
