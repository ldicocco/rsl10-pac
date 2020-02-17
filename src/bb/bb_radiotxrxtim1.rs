#[doc = "Reader of register BB_RADIOTXRXTIM1"]
pub type R = crate::R<u32, super::BB_RADIOTXRXTIM1>;
#[doc = "Writer for register BB_RADIOTXRXTIM1"]
pub type W = crate::W<u32, super::BB_RADIOTXRXTIM1>;
#[doc = "Register BB_RADIOTXRXTIM1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_RADIOTXRXTIM1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXPATHDLY1_A {
    #[doc = "0: `0`"]
    TXPATHDLY1_0 = 0,
}
impl From<TXPATHDLY1_A> for u8 {
    #[inline(always)]
    fn from(variant: TXPATHDLY1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXPATHDLY1`"]
pub type TXPATHDLY1_R = crate::R<u8, TXPATHDLY1_A>;
impl TXPATHDLY1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXPATHDLY1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXPATHDLY1_A::TXPATHDLY1_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXPATHDLY1_0`"]
    #[inline(always)]
    pub fn is_txpathdly1_0(&self) -> bool {
        *self == TXPATHDLY1_A::TXPATHDLY1_0
    }
}
#[doc = "Write proxy for field `TXPATHDLY1`"]
pub struct TXPATHDLY1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPATHDLY1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPATHDLY1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn txpathdly1_0(self) -> &'a mut W {
        self.variant(TXPATHDLY1_A::TXPATHDLY1_0)
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
pub enum RXPATHDLY1_A {
    #[doc = "0: `0`"]
    RXPATHDLY1_0 = 0,
}
impl From<RXPATHDLY1_A> for u8 {
    #[inline(always)]
    fn from(variant: RXPATHDLY1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXPATHDLY1`"]
pub type RXPATHDLY1_R = crate::R<u8, RXPATHDLY1_A>;
impl RXPATHDLY1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXPATHDLY1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXPATHDLY1_A::RXPATHDLY1_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RXPATHDLY1_0`"]
    #[inline(always)]
    pub fn is_rxpathdly1_0(&self) -> bool {
        *self == RXPATHDLY1_A::RXPATHDLY1_0
    }
}
#[doc = "Write proxy for field `RXPATHDLY1`"]
pub struct RXPATHDLY1_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPATHDLY1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPATHDLY1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rxpathdly1_0(self) -> &'a mut W {
        self.variant(RXPATHDLY1_A::RXPATHDLY1_0)
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
pub enum RFRXTMDA1_A {
    #[doc = "0: `0`"]
    RFRXTMDA1_0 = 0,
}
impl From<RFRXTMDA1_A> for u8 {
    #[inline(always)]
    fn from(variant: RFRXTMDA1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RFRXTMDA1`"]
pub type RFRXTMDA1_R = crate::R<u8, RFRXTMDA1_A>;
impl RFRXTMDA1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RFRXTMDA1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RFRXTMDA1_A::RFRXTMDA1_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RFRXTMDA1_0`"]
    #[inline(always)]
    pub fn is_rfrxtmda1_0(&self) -> bool {
        *self == RFRXTMDA1_A::RFRXTMDA1_0
    }
}
#[doc = "Write proxy for field `RFRXTMDA1`"]
pub struct RFRXTMDA1_W<'a> {
    w: &'a mut W,
}
impl<'a> RFRXTMDA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFRXTMDA1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rfrxtmda1_0(self) -> &'a mut W {
        self.variant(RFRXTMDA1_A::RFRXTMDA1_0)
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
pub enum SYNC_POSITION1_A {
    #[doc = "0: `0`"]
    SYNC_POSITION1_0 = 0,
}
impl From<SYNC_POSITION1_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNC_POSITION1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNC_POSITION1`"]
pub type SYNC_POSITION1_R = crate::R<u8, SYNC_POSITION1_A>;
impl SYNC_POSITION1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYNC_POSITION1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYNC_POSITION1_A::SYNC_POSITION1_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYNC_POSITION1_0`"]
    #[inline(always)]
    pub fn is_sync_position1_0(&self) -> bool {
        *self == SYNC_POSITION1_A::SYNC_POSITION1_0
    }
}
#[doc = "Write proxy for field `SYNC_POSITION1`"]
pub struct SYNC_POSITION1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_POSITION1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_POSITION1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sync_position1_0(self) -> &'a mut W {
        self.variant(SYNC_POSITION1_A::SYNC_POSITION1_0)
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
    pub fn txpathdly1(&self) -> TXPATHDLY1_R {
        TXPATHDLY1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn rxpathdly1(&self) -> RXPATHDLY1_R {
        RXPATHDLY1_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn rfrxtmda1(&self) -> RFRXTMDA1_R {
        RFRXTMDA1_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn sync_position1(&self) -> SYNC_POSITION1_R {
        SYNC_POSITION1_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn txpathdly1(&mut self) -> TXPATHDLY1_W {
        TXPATHDLY1_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn rxpathdly1(&mut self) -> RXPATHDLY1_W {
        RXPATHDLY1_W { w: self }
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn rfrxtmda1(&mut self) -> RFRXTMDA1_W {
        RFRXTMDA1_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn sync_position1(&mut self) -> SYNC_POSITION1_W {
        SYNC_POSITION1_W { w: self }
    }
}
