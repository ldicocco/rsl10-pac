#[doc = "Reader of register BB_RADIOPWRUPDN1"]
pub type R = crate::R<u32, super::BB_RADIOPWRUPDN1>;
#[doc = "Writer for register BB_RADIOPWRUPDN1"]
pub type W = crate::W<u32, super::BB_RADIOPWRUPDN1>;
#[doc = "Register BB_RADIOPWRUPDN1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_RADIOPWRUPDN1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "This register holds the length in us of the RX power up phase for the current radio device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXPWRUP1_A {
    #[doc = "0: `0`"]
    RXPWRUP1_0 = 0,
}
impl From<RXPWRUP1_A> for u8 {
    #[inline(always)]
    fn from(variant: RXPWRUP1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXPWRUP1`"]
pub type RXPWRUP1_R = crate::R<u8, RXPWRUP1_A>;
impl RXPWRUP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXPWRUP1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXPWRUP1_A::RXPWRUP1_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RXPWRUP1_0`"]
    #[inline(always)]
    pub fn is_rxpwrup1_0(&self) -> bool {
        *self == RXPWRUP1_A::RXPWRUP1_0
    }
}
#[doc = "Write proxy for field `RXPWRUP1`"]
pub struct RXPWRUP1_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPWRUP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPWRUP1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rxpwrup1_0(self) -> &'a mut W {
        self.variant(RXPWRUP1_A::RXPWRUP1_0)
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
pub enum TXPWRDN1_A {
    #[doc = "0: `0`"]
    TXPWRDN1_0 = 0,
}
impl From<TXPWRDN1_A> for u8 {
    #[inline(always)]
    fn from(variant: TXPWRDN1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXPWRDN1`"]
pub type TXPWRDN1_R = crate::R<u8, TXPWRDN1_A>;
impl TXPWRDN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXPWRDN1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXPWRDN1_A::TXPWRDN1_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXPWRDN1_0`"]
    #[inline(always)]
    pub fn is_txpwrdn1_0(&self) -> bool {
        *self == TXPWRDN1_A::TXPWRDN1_0
    }
}
#[doc = "Write proxy for field `TXPWRDN1`"]
pub struct TXPWRDN1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPWRDN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPWRDN1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn txpwrdn1_0(self) -> &'a mut W {
        self.variant(TXPWRDN1_A::TXPWRDN1_0)
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
pub enum TXPWRUP1_A {
    #[doc = "0: `0`"]
    TXPWRUP1_0 = 0,
}
impl From<TXPWRUP1_A> for u8 {
    #[inline(always)]
    fn from(variant: TXPWRUP1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXPWRUP1`"]
pub type TXPWRUP1_R = crate::R<u8, TXPWRUP1_A>;
impl TXPWRUP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXPWRUP1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXPWRUP1_A::TXPWRUP1_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXPWRUP1_0`"]
    #[inline(always)]
    pub fn is_txpwrup1_0(&self) -> bool {
        *self == TXPWRUP1_A::TXPWRUP1_0
    }
}
#[doc = "Write proxy for field `TXPWRUP1`"]
pub struct TXPWRUP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPWRUP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPWRUP1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn txpwrup1_0(self) -> &'a mut W {
        self.variant(TXPWRUP1_A::TXPWRUP1_0)
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
    pub fn rxpwrup1(&self) -> RXPWRUP1_R {
        RXPWRUP1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - This register extends the length in us of the TX power down phase for the current radio device"]
    #[inline(always)]
    pub fn txpwrdn1(&self) -> TXPWRDN1_R {
        TXPWRDN1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:7 - This register holds the length in us of the TX power up phase for the current radio device"]
    #[inline(always)]
    pub fn txpwrup1(&self) -> TXPWRUP1_R {
        TXPWRUP1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - This register holds the length in us of the RX power up phase for the current radio device"]
    #[inline(always)]
    pub fn rxpwrup1(&mut self) -> RXPWRUP1_W {
        RXPWRUP1_W { w: self }
    }
    #[doc = "Bits 8:12 - This register extends the length in us of the TX power down phase for the current radio device"]
    #[inline(always)]
    pub fn txpwrdn1(&mut self) -> TXPWRDN1_W {
        TXPWRDN1_W { w: self }
    }
    #[doc = "Bits 0:7 - This register holds the length in us of the TX power up phase for the current radio device"]
    #[inline(always)]
    pub fn txpwrup1(&mut self) -> TXPWRUP1_W {
        TXPWRUP1_W { w: self }
    }
}
