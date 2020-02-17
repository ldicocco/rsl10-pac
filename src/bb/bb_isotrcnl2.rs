#[doc = "Reader of register BB_ISOTRCNL2"]
pub type R = crate::R<u32, super::BB_ISOTRCNL2>;
#[doc = "Writer for register BB_ISOTRCNL2"]
pub type W = crate::W<u32, super::BB_ISOTRCNL2>;
#[doc = "Register BB_ISOTRCNL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_ISOTRCNL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Negotiated, maximum expected number of bytes for ISO Channel 2 Rx payloads\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ISO2RXLEN_A {
    #[doc = "0: `0`"]
    ISO2RXLEN_0 = 0,
}
impl From<ISO2RXLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: ISO2RXLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ISO2RXLEN`"]
pub type ISO2RXLEN_R = crate::R<u8, ISO2RXLEN_A>;
impl ISO2RXLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ISO2RXLEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ISO2RXLEN_A::ISO2RXLEN_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ISO2RXLEN_0`"]
    #[inline(always)]
    pub fn is_iso2rxlen_0(&self) -> bool {
        *self == ISO2RXLEN_A::ISO2RXLEN_0
    }
}
#[doc = "Write proxy for field `ISO2RXLEN`"]
pub struct ISO2RXLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO2RXLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISO2RXLEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iso2rxlen_0(self) -> &'a mut W {
        self.variant(ISO2RXLEN_A::ISO2RXLEN_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Negotiated, number of bytes for ISO Channel 2 Tx payloads\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ISO2TXLEN_A {
    #[doc = "0: `0`"]
    ISO2TXLEN_0 = 0,
}
impl From<ISO2TXLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: ISO2TXLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ISO2TXLEN`"]
pub type ISO2TXLEN_R = crate::R<u8, ISO2TXLEN_A>;
impl ISO2TXLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ISO2TXLEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ISO2TXLEN_A::ISO2TXLEN_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ISO2TXLEN_0`"]
    #[inline(always)]
    pub fn is_iso2txlen_0(&self) -> bool {
        *self == ISO2TXLEN_A::ISO2TXLEN_0
    }
}
#[doc = "Write proxy for field `ISO2TXLEN`"]
pub struct ISO2TXLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO2TXLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISO2TXLEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iso2txlen_0(self) -> &'a mut W {
        self.variant(ISO2TXLEN_A::ISO2TXLEN_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - Negotiated, maximum expected number of bytes for ISO Channel 2 Rx payloads"]
    #[inline(always)]
    pub fn iso2rxlen(&self) -> ISO2RXLEN_R {
        ISO2RXLEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Negotiated, number of bytes for ISO Channel 2 Tx payloads"]
    #[inline(always)]
    pub fn iso2txlen(&self) -> ISO2TXLEN_R {
        ISO2TXLEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Negotiated, maximum expected number of bytes for ISO Channel 2 Rx payloads"]
    #[inline(always)]
    pub fn iso2rxlen(&mut self) -> ISO2RXLEN_W {
        ISO2RXLEN_W { w: self }
    }
    #[doc = "Bits 0:7 - Negotiated, number of bytes for ISO Channel 2 Tx payloads"]
    #[inline(always)]
    pub fn iso2txlen(&mut self) -> ISO2TXLEN_W {
        ISO2TXLEN_W { w: self }
    }
}
