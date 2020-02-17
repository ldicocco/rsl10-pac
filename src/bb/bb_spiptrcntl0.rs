#[doc = "Reader of register BB_SPIPTRCNTL0"]
pub type R = crate::R<u32, super::BB_SPIPTRCNTL0>;
#[doc = "Writer for register BB_SPIPTRCNTL0"]
pub type W = crate::W<u32, super::BB_SPIPTRCNTL0>;
#[doc = "Register BB_SPIPTRCNTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_SPIPTRCNTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pointer to the TxOFF sequence address section\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum TXOFFPTR_A {
    #[doc = "0: `0`"]
    TXOFFPTR_0 = 0,
}
impl From<TXOFFPTR_A> for u16 {
    #[inline(always)]
    fn from(variant: TXOFFPTR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXOFFPTR`"]
pub type TXOFFPTR_R = crate::R<u16, TXOFFPTR_A>;
impl TXOFFPTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, TXOFFPTR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXOFFPTR_A::TXOFFPTR_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXOFFPTR_0`"]
    #[inline(always)]
    pub fn is_txoffptr_0(&self) -> bool {
        *self == TXOFFPTR_A::TXOFFPTR_0
    }
}
#[doc = "Write proxy for field `TXOFFPTR`"]
pub struct TXOFFPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOFFPTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXOFFPTR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn txoffptr_0(self) -> &'a mut W {
        self.variant(TXOFFPTR_A::TXOFFPTR_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Pointer to the TxON sequence address section\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum TXONPTR_A {
    #[doc = "0: `0`"]
    TXONPTR_0 = 0,
}
impl From<TXONPTR_A> for u16 {
    #[inline(always)]
    fn from(variant: TXONPTR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXONPTR`"]
pub type TXONPTR_R = crate::R<u16, TXONPTR_A>;
impl TXONPTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, TXONPTR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXONPTR_A::TXONPTR_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXONPTR_0`"]
    #[inline(always)]
    pub fn is_txonptr_0(&self) -> bool {
        *self == TXONPTR_A::TXONPTR_0
    }
}
#[doc = "Write proxy for field `TXONPTR`"]
pub struct TXONPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXONPTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXONPTR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn txonptr_0(self) -> &'a mut W {
        self.variant(TXONPTR_A::TXONPTR_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Pointer to the TxOFF sequence address section"]
    #[inline(always)]
    pub fn txoffptr(&self) -> TXOFFPTR_R {
        TXOFFPTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Pointer to the TxON sequence address section"]
    #[inline(always)]
    pub fn txonptr(&self) -> TXONPTR_R {
        TXONPTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Pointer to the TxOFF sequence address section"]
    #[inline(always)]
    pub fn txoffptr(&mut self) -> TXOFFPTR_W {
        TXOFFPTR_W { w: self }
    }
    #[doc = "Bits 0:15 - Pointer to the TxON sequence address section"]
    #[inline(always)]
    pub fn txonptr(&mut self) -> TXONPTR_W {
        TXONPTR_W { w: self }
    }
}
