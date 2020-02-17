#[doc = "Reader of register BB_SPIPTRCNTL1"]
pub type R = crate::R<u32, super::BB_SPIPTRCNTL1>;
#[doc = "Writer for register BB_SPIPTRCNTL1"]
pub type W = crate::W<u32, super::BB_SPIPTRCNTL1>;
#[doc = "Register BB_SPIPTRCNTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_SPIPTRCNTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pointer to the RxOFF sequence address section\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum RXOFFPTR_A {
    #[doc = "0: `0`"]
    RXOFFPTR_0 = 0,
}
impl From<RXOFFPTR_A> for u16 {
    #[inline(always)]
    fn from(variant: RXOFFPTR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXOFFPTR`"]
pub type RXOFFPTR_R = crate::R<u16, RXOFFPTR_A>;
impl RXOFFPTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, RXOFFPTR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXOFFPTR_A::RXOFFPTR_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RXOFFPTR_0`"]
    #[inline(always)]
    pub fn is_rxoffptr_0(&self) -> bool {
        *self == RXOFFPTR_A::RXOFFPTR_0
    }
}
#[doc = "Write proxy for field `RXOFFPTR`"]
pub struct RXOFFPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOFFPTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOFFPTR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rxoffptr_0(self) -> &'a mut W {
        self.variant(RXOFFPTR_A::RXOFFPTR_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Pointer to the RxON sequence address section\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum RXONPTR_A {
    #[doc = "0: `0`"]
    RXONPTR_0 = 0,
}
impl From<RXONPTR_A> for u16 {
    #[inline(always)]
    fn from(variant: RXONPTR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXONPTR`"]
pub type RXONPTR_R = crate::R<u16, RXONPTR_A>;
impl RXONPTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, RXONPTR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXONPTR_A::RXONPTR_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RXONPTR_0`"]
    #[inline(always)]
    pub fn is_rxonptr_0(&self) -> bool {
        *self == RXONPTR_A::RXONPTR_0
    }
}
#[doc = "Write proxy for field `RXONPTR`"]
pub struct RXONPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXONPTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXONPTR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rxonptr_0(self) -> &'a mut W {
        self.variant(RXONPTR_A::RXONPTR_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Pointer to the RxOFF sequence address section"]
    #[inline(always)]
    pub fn rxoffptr(&self) -> RXOFFPTR_R {
        RXOFFPTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Pointer to the RxON sequence address section"]
    #[inline(always)]
    pub fn rxonptr(&self) -> RXONPTR_R {
        RXONPTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Pointer to the RxOFF sequence address section"]
    #[inline(always)]
    pub fn rxoffptr(&mut self) -> RXOFFPTR_W {
        RXOFFPTR_W { w: self }
    }
    #[doc = "Bits 0:15 - Pointer to the RxON sequence address section"]
    #[inline(always)]
    pub fn rxonptr(&mut self) -> RXONPTR_W {
        RXONPTR_W { w: self }
    }
}
