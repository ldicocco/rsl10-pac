#[doc = "Reader of register BB_ISOTRCNL1"]
pub type R = crate::R<u32, super::BB_ISOTRCNL1>;
#[doc = "Writer for register BB_ISOTRCNL1"]
pub type W = crate::W<u32, super::BB_ISOTRCNL1>;
#[doc = "Register BB_ISOTRCNL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_ISOTRCNL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Negotiated, maximum expected number of bytes for ISO Channel 0 Rx payloads\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ISO1RXLEN_A {
    #[doc = "0: `0`"]
    ISO1RXLEN_0 = 0,
}
impl From<ISO1RXLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: ISO1RXLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ISO1RXLEN`"]
pub type ISO1RXLEN_R = crate::R<u8, ISO1RXLEN_A>;
impl ISO1RXLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ISO1RXLEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ISO1RXLEN_A::ISO1RXLEN_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ISO1RXLEN_0`"]
    #[inline(always)]
    pub fn is_iso1rxlen_0(&self) -> bool {
        *self == ISO1RXLEN_A::ISO1RXLEN_0
    }
}
#[doc = "Write proxy for field `ISO1RXLEN`"]
pub struct ISO1RXLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO1RXLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISO1RXLEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iso1rxlen_0(self) -> &'a mut W {
        self.variant(ISO1RXLEN_A::ISO1RXLEN_0)
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
pub enum ISO1TXLEN_A {
    #[doc = "0: `0`"]
    ISO1TXLEN_0 = 0,
}
impl From<ISO1TXLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: ISO1TXLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ISO1TXLEN`"]
pub type ISO1TXLEN_R = crate::R<u8, ISO1TXLEN_A>;
impl ISO1TXLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ISO1TXLEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ISO1TXLEN_A::ISO1TXLEN_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ISO1TXLEN_0`"]
    #[inline(always)]
    pub fn is_iso1txlen_0(&self) -> bool {
        *self == ISO1TXLEN_A::ISO1TXLEN_0
    }
}
#[doc = "Write proxy for field `ISO1TXLEN`"]
pub struct ISO1TXLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO1TXLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISO1TXLEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn iso1txlen_0(self) -> &'a mut W {
        self.variant(ISO1TXLEN_A::ISO1TXLEN_0)
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
    pub fn iso1rxlen(&self) -> ISO1RXLEN_R {
        ISO1RXLEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Negotiated, number of bytes for ISO Channel 0 Tx payloads"]
    #[inline(always)]
    pub fn iso1txlen(&self) -> ISO1TXLEN_R {
        ISO1TXLEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Negotiated, maximum expected number of bytes for ISO Channel 0 Rx payloads"]
    #[inline(always)]
    pub fn iso1rxlen(&mut self) -> ISO1RXLEN_W {
        ISO1RXLEN_W { w: self }
    }
    #[doc = "Bits 0:7 - Negotiated, number of bytes for ISO Channel 0 Tx payloads"]
    #[inline(always)]
    pub fn iso1txlen(&mut self) -> ISO1TXLEN_W {
        ISO1TXLEN_W { w: self }
    }
}
