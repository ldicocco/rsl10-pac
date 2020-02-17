#[doc = "Reader of register BB_RADIOTXRXTIM0"]
pub type R = crate::R<u32, super::BB_RADIOTXRXTIM0>;
#[doc = "Writer for register BB_RADIOTXRXTIM0"]
pub type W = crate::W<u32, super::BB_RADIOTXRXTIM0>;
#[doc = "Register BB_RADIOTXRXTIM0 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_RADIOTXRXTIM0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXPATHDLY0_A {
    #[doc = "0: `0`"]
    TXPATHDLY0_0 = 0,
}
impl From<TXPATHDLY0_A> for u8 {
    #[inline(always)]
    fn from(variant: TXPATHDLY0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXPATHDLY0`"]
pub type TXPATHDLY0_R = crate::R<u8, TXPATHDLY0_A>;
impl TXPATHDLY0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXPATHDLY0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXPATHDLY0_A::TXPATHDLY0_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXPATHDLY0_0`"]
    #[inline(always)]
    pub fn is_txpathdly0_0(&self) -> bool {
        *self == TXPATHDLY0_A::TXPATHDLY0_0
    }
}
#[doc = "Write proxy for field `TXPATHDLY0`"]
pub struct TXPATHDLY0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPATHDLY0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPATHDLY0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn txpathdly0_0(self) -> &'a mut W {
        self.variant(TXPATHDLY0_A::TXPATHDLY0_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXPATHDLY0_A {
    #[doc = "0: `0`"]
    RXPATHDLY0_0 = 0,
}
impl From<RXPATHDLY0_A> for u8 {
    #[inline(always)]
    fn from(variant: RXPATHDLY0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXPATHDLY0`"]
pub type RXPATHDLY0_R = crate::R<u8, RXPATHDLY0_A>;
impl RXPATHDLY0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXPATHDLY0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXPATHDLY0_A::RXPATHDLY0_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RXPATHDLY0_0`"]
    #[inline(always)]
    pub fn is_rxpathdly0_0(&self) -> bool {
        *self == RXPATHDLY0_A::RXPATHDLY0_0
    }
}
#[doc = "Write proxy for field `RXPATHDLY0`"]
pub struct RXPATHDLY0_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPATHDLY0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPATHDLY0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rxpathdly0_0(self) -> &'a mut W {
        self.variant(RXPATHDLY0_A::RXPATHDLY0_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RFRXTMDA0_A {
    #[doc = "0: `0`"]
    RFRXTMDA0_0 = 0,
}
impl From<RFRXTMDA0_A> for u8 {
    #[inline(always)]
    fn from(variant: RFRXTMDA0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RFRXTMDA0`"]
pub type RFRXTMDA0_R = crate::R<u8, RFRXTMDA0_A>;
impl RFRXTMDA0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RFRXTMDA0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RFRXTMDA0_A::RFRXTMDA0_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RFRXTMDA0_0`"]
    #[inline(always)]
    pub fn is_rfrxtmda0_0(&self) -> bool {
        *self == RFRXTMDA0_A::RFRXTMDA0_0
    }
}
#[doc = "Write proxy for field `RFRXTMDA0`"]
pub struct RFRXTMDA0_W<'a> {
    w: &'a mut W,
}
impl<'a> RFRXTMDA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFRXTMDA0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rfrxtmda0_0(self) -> &'a mut W {
        self.variant(RFRXTMDA0_A::RFRXTMDA0_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNC_POSITION0_A {
    #[doc = "0: `0`"]
    SYNC_POSITION0_0 = 0,
}
impl From<SYNC_POSITION0_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNC_POSITION0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNC_POSITION0`"]
pub type SYNC_POSITION0_R = crate::R<u8, SYNC_POSITION0_A>;
impl SYNC_POSITION0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYNC_POSITION0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYNC_POSITION0_A::SYNC_POSITION0_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYNC_POSITION0_0`"]
    #[inline(always)]
    pub fn is_sync_position0_0(&self) -> bool {
        *self == SYNC_POSITION0_A::SYNC_POSITION0_0
    }
}
#[doc = "Write proxy for field `SYNC_POSITION0`"]
pub struct SYNC_POSITION0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_POSITION0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_POSITION0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sync_position0_0(self) -> &'a mut W {
        self.variant(SYNC_POSITION0_A::SYNC_POSITION0_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn txpathdly0(&self) -> TXPATHDLY0_R {
        TXPATHDLY0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn rxpathdly0(&self) -> RXPATHDLY0_R {
        RXPATHDLY0_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn rfrxtmda0(&self) -> RFRXTMDA0_R {
        RFRXTMDA0_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn sync_position0(&self) -> SYNC_POSITION0_R {
        SYNC_POSITION0_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn txpathdly0(&mut self) -> TXPATHDLY0_W {
        TXPATHDLY0_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn rxpathdly0(&mut self) -> RXPATHDLY0_W {
        RXPATHDLY0_W { w: self }
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn rfrxtmda0(&mut self) -> RFRXTMDA0_W {
        RFRXTMDA0_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn sync_position0(&mut self) -> SYNC_POSITION0_W {
        SYNC_POSITION0_W { w: self }
    }
}
