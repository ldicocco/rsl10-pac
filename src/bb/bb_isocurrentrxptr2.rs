#[doc = "Reader of register BB_ISOCURRENTRXPTR2"]
pub type R = crate::R<u32, super::BB_ISOCURRENTRXPTR2>;
#[doc = "Writer for register BB_ISOCURRENTRXPTR2"]
pub type W = crate::W<u32, super::BB_ISOCURRENTRXPTR2>;
#[doc = "Register BB_ISOCURRENTRXPTR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_ISOCURRENTRXPTR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Rx ISO Buffer pointer 0 of ISO Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ISO2RXPTR0_A {
    #[doc = "0: Rx ISO Buffer pointer 0 of ISO Channel 2"]
    ISO2RXPTR0_0 = 0,
}
impl From<ISO2RXPTR0_A> for u16 {
    #[inline(always)]
    fn from(variant: ISO2RXPTR0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ISO2RXPTR0`"]
pub type ISO2RXPTR0_R = crate::R<u16, ISO2RXPTR0_A>;
impl ISO2RXPTR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ISO2RXPTR0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ISO2RXPTR0_A::ISO2RXPTR0_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ISO2RXPTR0_0`"]
    #[inline(always)]
    pub fn is_iso2rxptr0_0(&self) -> bool {
        *self == ISO2RXPTR0_A::ISO2RXPTR0_0
    }
}
#[doc = "Write proxy for field `ISO2RXPTR0`"]
pub struct ISO2RXPTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO2RXPTR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISO2RXPTR0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Rx ISO Buffer pointer 0 of ISO Channel 2"]
    #[inline(always)]
    pub fn iso2rxptr0_0(self) -> &'a mut W {
        self.variant(ISO2RXPTR0_A::ISO2RXPTR0_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Rx ISO Buffer pointer 1 of ISO Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ISO2RXPTR1_A {
    #[doc = "0: TR ISO Buffer pointer 1 of ISO Channel 2"]
    ISO2RXPTR1_0 = 0,
}
impl From<ISO2RXPTR1_A> for u16 {
    #[inline(always)]
    fn from(variant: ISO2RXPTR1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ISO2RXPTR1`"]
pub type ISO2RXPTR1_R = crate::R<u16, ISO2RXPTR1_A>;
impl ISO2RXPTR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ISO2RXPTR1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ISO2RXPTR1_A::ISO2RXPTR1_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ISO2RXPTR1_0`"]
    #[inline(always)]
    pub fn is_iso2rxptr1_0(&self) -> bool {
        *self == ISO2RXPTR1_A::ISO2RXPTR1_0
    }
}
#[doc = "Write proxy for field `ISO2RXPTR1`"]
pub struct ISO2RXPTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO2RXPTR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISO2RXPTR1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TR ISO Buffer pointer 1 of ISO Channel 2"]
    #[inline(always)]
    pub fn iso2rxptr1_0(self) -> &'a mut W {
        self.variant(ISO2RXPTR1_A::ISO2RXPTR1_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Rx ISO Buffer pointer 0 of ISO Channel 2"]
    #[inline(always)]
    pub fn iso2rxptr0(&self) -> ISO2RXPTR0_R {
        ISO2RXPTR0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Rx ISO Buffer pointer 1 of ISO Channel 2"]
    #[inline(always)]
    pub fn iso2rxptr1(&self) -> ISO2RXPTR1_R {
        ISO2RXPTR1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Rx ISO Buffer pointer 0 of ISO Channel 2"]
    #[inline(always)]
    pub fn iso2rxptr0(&mut self) -> ISO2RXPTR0_W {
        ISO2RXPTR0_W { w: self }
    }
    #[doc = "Bits 0:15 - Rx ISO Buffer pointer 1 of ISO Channel 2"]
    #[inline(always)]
    pub fn iso2rxptr1(&mut self) -> ISO2RXPTR1_W {
        ISO2RXPTR1_W { w: self }
    }
}
