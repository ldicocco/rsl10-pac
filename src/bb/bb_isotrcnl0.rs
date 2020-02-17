#[doc = "Reader of register BB_ISOTRCNL0"]
pub type R = crate::R<u32, super::BB_ISOTRCNL0>;
#[doc = "Writer for register BB_ISOTRCNL0"]
pub type W = crate::W<u32, super::BB_ISOTRCNL0>;
#[doc = "Register BB_ISOTRCNL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_ISOTRCNL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Negotiated, maximum expected number of bytes for ISO Channel 0 Rx payloads\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ISO0RXLEN_A {
    #[doc = "0: `0`"]
    ISO0RXLEN_0 = 0,
}
impl From<ISO0RXLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: ISO0RXLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ISO0RXLEN`"]
pub type ISO0RXLEN_R = crate::R<u8, ISO0RXLEN_A>;
impl ISO0RXLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ISO0RXLEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ISO0RXLEN_A::ISO0RXLEN_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ISO0RXLEN_0`"]
    #[inline(always)]
    pub fn is_iso0rxlen_0(&self) -> bool {
        *self == ISO0RXLEN_A::ISO0RXLEN_0
    }
}
#[doc = "Write proxy for field `ISO0RXLEN`"]
pub struct ISO0RXLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO0RXLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISO0RXLEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iso0rxlen_0(self) -> &'a mut W {
        self.variant(ISO0RXLEN_A::ISO0RXLEN_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Negotiated, number of bytes for ISO Channel 0 Tx payloads\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ISO0TXLEN_A {
    #[doc = "0: `0`"]
    ISO0TXLEN_0 = 0,
}
impl From<ISO0TXLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: ISO0TXLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ISO0TXLEN`"]
pub type ISO0TXLEN_R = crate::R<u8, ISO0TXLEN_A>;
impl ISO0TXLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ISO0TXLEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ISO0TXLEN_A::ISO0TXLEN_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ISO0TXLEN_0`"]
    #[inline(always)]
    pub fn is_iso0txlen_0(&self) -> bool {
        *self == ISO0TXLEN_A::ISO0TXLEN_0
    }
}
#[doc = "Write proxy for field `ISO0TXLEN`"]
pub struct ISO0TXLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO0TXLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISO0TXLEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iso0txlen_0(self) -> &'a mut W {
        self.variant(ISO0TXLEN_A::ISO0TXLEN_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - Negotiated, maximum expected number of bytes for ISO Channel 0 Rx payloads"]
    #[inline(always)]
    pub fn iso0rxlen(&self) -> ISO0RXLEN_R {
        ISO0RXLEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Negotiated, number of bytes for ISO Channel 0 Tx payloads"]
    #[inline(always)]
    pub fn iso0txlen(&self) -> ISO0TXLEN_R {
        ISO0TXLEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Negotiated, maximum expected number of bytes for ISO Channel 0 Rx payloads"]
    #[inline(always)]
    pub fn iso0rxlen(&mut self) -> ISO0RXLEN_W {
        ISO0RXLEN_W { w: self }
    }
    #[doc = "Bits 0:7 - Negotiated, number of bytes for ISO Channel 0 Tx payloads"]
    #[inline(always)]
    pub fn iso0txlen(&mut self) -> ISO0TXLEN_W {
        ISO0TXLEN_W { w: self }
    }
}
