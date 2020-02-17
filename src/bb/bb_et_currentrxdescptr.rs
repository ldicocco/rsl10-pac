#[doc = "Reader of register BB_ET_CURRENTRXDESCPTR"]
pub type R = crate::R<u32, super::BB_ET_CURRENTRXDESCPTR>;
#[doc = "Writer for register BB_ET_CURRENTRXDESCPTR"]
pub type W = crate::W<u32, super::BB_ET_CURRENTRXDESCPTR>;
#[doc = "Register BB_ET_CURRENTRXDESCPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_ET_CURRENTRXDESCPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Exchange table pointer that determines the starting point of the exchange table\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ETPTR_A {
    #[doc = "0: `0`"]
    ETPTR_0 = 0,
}
impl From<ETPTR_A> for u16 {
    #[inline(always)]
    fn from(variant: ETPTR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ETPTR`"]
pub type ETPTR_R = crate::R<u16, ETPTR_A>;
impl ETPTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ETPTR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ETPTR_A::ETPTR_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ETPTR_0`"]
    #[inline(always)]
    pub fn is_etptr_0(&self) -> bool {
        *self == ETPTR_A::ETPTR_0
    }
}
#[doc = "Write proxy for field `ETPTR`"]
pub struct ETPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ETPTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETPTR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn etptr_0(self) -> &'a mut W {
        self.variant(ETPTR_A::ETPTR_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Rx descriptor pointer that determines the starting point of the receive buffer chained list\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CURRENTRXDESCPTR_A {
    #[doc = "0: `0`"]
    CURRENTRXDESCPTR_0 = 0,
}
impl From<CURRENTRXDESCPTR_A> for u16 {
    #[inline(always)]
    fn from(variant: CURRENTRXDESCPTR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CURRENTRXDESCPTR`"]
pub type CURRENTRXDESCPTR_R = crate::R<u16, CURRENTRXDESCPTR_A>;
impl CURRENTRXDESCPTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, CURRENTRXDESCPTR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CURRENTRXDESCPTR_A::CURRENTRXDESCPTR_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CURRENTRXDESCPTR_0`"]
    #[inline(always)]
    pub fn is_currentrxdescptr_0(&self) -> bool {
        *self == CURRENTRXDESCPTR_A::CURRENTRXDESCPTR_0
    }
}
#[doc = "Write proxy for field `CURRENTRXDESCPTR`"]
pub struct CURRENTRXDESCPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRENTRXDESCPTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CURRENTRXDESCPTR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn currentrxdescptr_0(self) -> &'a mut W {
        self.variant(CURRENTRXDESCPTR_A::CURRENTRXDESCPTR_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Exchange table pointer that determines the starting point of the exchange table"]
    #[inline(always)]
    pub fn etptr(&self) -> ETPTR_R {
        ETPTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:14 - Rx descriptor pointer that determines the starting point of the receive buffer chained list"]
    #[inline(always)]
    pub fn currentrxdescptr(&self) -> CURRENTRXDESCPTR_R {
        CURRENTRXDESCPTR_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Exchange table pointer that determines the starting point of the exchange table"]
    #[inline(always)]
    pub fn etptr(&mut self) -> ETPTR_W {
        ETPTR_W { w: self }
    }
    #[doc = "Bits 0:14 - Rx descriptor pointer that determines the starting point of the receive buffer chained list"]
    #[inline(always)]
    pub fn currentrxdescptr(&mut self) -> CURRENTRXDESCPTR_W {
        CURRENTRXDESCPTR_W { w: self }
    }
}
