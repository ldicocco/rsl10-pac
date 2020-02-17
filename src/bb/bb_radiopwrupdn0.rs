#[doc = "Reader of register BB_RADIOPWRUPDN0"]
pub type R = crate::R<u32, super::BB_RADIOPWRUPDN0>;
#[doc = "Writer for register BB_RADIOPWRUPDN0"]
pub type W = crate::W<u32, super::BB_RADIOPWRUPDN0>;
#[doc = "Register BB_RADIOPWRUPDN0 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_RADIOPWRUPDN0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "This register holds the length in us of the RX power up phase for the current radio device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXPWRUP0_A {
    #[doc = "0: `0`"]
    RXPWRUP0_0 = 0,
}
impl From<RXPWRUP0_A> for u8 {
    #[inline(always)]
    fn from(variant: RXPWRUP0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXPWRUP0`"]
pub type RXPWRUP0_R = crate::R<u8, RXPWRUP0_A>;
impl RXPWRUP0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXPWRUP0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXPWRUP0_A::RXPWRUP0_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RXPWRUP0_0`"]
    #[inline(always)]
    pub fn is_rxpwrup0_0(&self) -> bool {
        *self == RXPWRUP0_A::RXPWRUP0_0
    }
}
#[doc = "Write proxy for field `RXPWRUP0`"]
pub struct RXPWRUP0_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPWRUP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPWRUP0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rxpwrup0_0(self) -> &'a mut W {
        self.variant(RXPWRUP0_A::RXPWRUP0_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "This register extends the length in us of the TX power down phase for the current radio device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXPWRDN0_A {
    #[doc = "0: `0`"]
    TXPWRDN0_0 = 0,
}
impl From<TXPWRDN0_A> for u8 {
    #[inline(always)]
    fn from(variant: TXPWRDN0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXPWRDN0`"]
pub type TXPWRDN0_R = crate::R<u8, TXPWRDN0_A>;
impl TXPWRDN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXPWRDN0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXPWRDN0_A::TXPWRDN0_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXPWRDN0_0`"]
    #[inline(always)]
    pub fn is_txpwrdn0_0(&self) -> bool {
        *self == TXPWRDN0_A::TXPWRDN0_0
    }
}
#[doc = "Write proxy for field `TXPWRDN0`"]
pub struct TXPWRDN0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPWRDN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPWRDN0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn txpwrdn0_0(self) -> &'a mut W {
        self.variant(TXPWRDN0_A::TXPWRDN0_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "This register holds the length in us of the TX power up phase for the current radio device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXPWRUP0_A {
    #[doc = "0: `0`"]
    TXPWRUP0_0 = 0,
}
impl From<TXPWRUP0_A> for u8 {
    #[inline(always)]
    fn from(variant: TXPWRUP0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXPWRUP0`"]
pub type TXPWRUP0_R = crate::R<u8, TXPWRUP0_A>;
impl TXPWRUP0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXPWRUP0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXPWRUP0_A::TXPWRUP0_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXPWRUP0_0`"]
    #[inline(always)]
    pub fn is_txpwrup0_0(&self) -> bool {
        *self == TXPWRUP0_A::TXPWRUP0_0
    }
}
#[doc = "Write proxy for field `TXPWRUP0`"]
pub struct TXPWRUP0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPWRUP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPWRUP0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn txpwrup0_0(self) -> &'a mut W {
        self.variant(TXPWRUP0_A::TXPWRUP0_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - This register holds the length in us of the RX power up phase for the current radio device"]
    #[inline(always)]
    pub fn rxpwrup0(&self) -> RXPWRUP0_R {
        RXPWRUP0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - This register extends the length in us of the TX power down phase for the current radio device"]
    #[inline(always)]
    pub fn txpwrdn0(&self) -> TXPWRDN0_R {
        TXPWRDN0_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:7 - This register holds the length in us of the TX power up phase for the current radio device"]
    #[inline(always)]
    pub fn txpwrup0(&self) -> TXPWRUP0_R {
        TXPWRUP0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - This register holds the length in us of the RX power up phase for the current radio device"]
    #[inline(always)]
    pub fn rxpwrup0(&mut self) -> RXPWRUP0_W {
        RXPWRUP0_W { w: self }
    }
    #[doc = "Bits 8:12 - This register extends the length in us of the TX power down phase for the current radio device"]
    #[inline(always)]
    pub fn txpwrdn0(&mut self) -> TXPWRDN0_W {
        TXPWRDN0_W { w: self }
    }
    #[doc = "Bits 0:7 - This register holds the length in us of the TX power up phase for the current radio device"]
    #[inline(always)]
    pub fn txpwrup0(&mut self) -> TXPWRUP0_W {
        TXPWRUP0_W { w: self }
    }
}
