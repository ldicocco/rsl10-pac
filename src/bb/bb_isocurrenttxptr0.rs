#[doc = "Reader of register BB_ISOCURRENTTXPTR0"]
pub type R = crate::R<u32, super::BB_ISOCURRENTTXPTR0>;
#[doc = "Writer for register BB_ISOCURRENTTXPTR0"]
pub type W = crate::W<u32, super::BB_ISOCURRENTTXPTR0>;
#[doc = "Register BB_ISOCURRENTTXPTR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_ISOCURRENTTXPTR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Tx ISO Buffer pointer 0 of ISO Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ISO0TXPTR0_A {
    #[doc = "0: Tx ISO Buffer pointer 0 of ISO Channel 0"]
    ISO0TXPTR0_0 = 0,
}
impl From<ISO0TXPTR0_A> for u16 {
    #[inline(always)]
    fn from(variant: ISO0TXPTR0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ISO0TXPTR0`"]
pub type ISO0TXPTR0_R = crate::R<u16, ISO0TXPTR0_A>;
impl ISO0TXPTR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ISO0TXPTR0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ISO0TXPTR0_A::ISO0TXPTR0_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ISO0TXPTR0_0`"]
    #[inline(always)]
    pub fn is_iso0txptr0_0(&self) -> bool {
        *self == ISO0TXPTR0_A::ISO0TXPTR0_0
    }
}
#[doc = "Write proxy for field `ISO0TXPTR0`"]
pub struct ISO0TXPTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO0TXPTR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISO0TXPTR0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Tx ISO Buffer pointer 0 of ISO Channel 0"]
    #[inline(always)]
    pub fn iso0txptr0_0(self) -> &'a mut W {
        self.variant(ISO0TXPTR0_A::ISO0TXPTR0_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Tx ISO Buffer pointer 1 of ISO Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ISO0TXPTR1_A {
    #[doc = "0: Tx ISO Buffer pointer 1 of ISO Channel 0"]
    ISO0TXPTR1_0 = 0,
}
impl From<ISO0TXPTR1_A> for u16 {
    #[inline(always)]
    fn from(variant: ISO0TXPTR1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ISO0TXPTR1`"]
pub type ISO0TXPTR1_R = crate::R<u16, ISO0TXPTR1_A>;
impl ISO0TXPTR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ISO0TXPTR1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ISO0TXPTR1_A::ISO0TXPTR1_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ISO0TXPTR1_0`"]
    #[inline(always)]
    pub fn is_iso0txptr1_0(&self) -> bool {
        *self == ISO0TXPTR1_A::ISO0TXPTR1_0
    }
}
#[doc = "Write proxy for field `ISO0TXPTR1`"]
pub struct ISO0TXPTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO0TXPTR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISO0TXPTR1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Tx ISO Buffer pointer 1 of ISO Channel 0"]
    #[inline(always)]
    pub fn iso0txptr1_0(self) -> &'a mut W {
        self.variant(ISO0TXPTR1_A::ISO0TXPTR1_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Tx ISO Buffer pointer 0 of ISO Channel 0"]
    #[inline(always)]
    pub fn iso0txptr0(&self) -> ISO0TXPTR0_R {
        ISO0TXPTR0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Tx ISO Buffer pointer 1 of ISO Channel 0"]
    #[inline(always)]
    pub fn iso0txptr1(&self) -> ISO0TXPTR1_R {
        ISO0TXPTR1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Tx ISO Buffer pointer 0 of ISO Channel 0"]
    #[inline(always)]
    pub fn iso0txptr0(&mut self) -> ISO0TXPTR0_W {
        ISO0TXPTR0_W { w: self }
    }
    #[doc = "Bits 0:15 - Tx ISO Buffer pointer 1 of ISO Channel 0"]
    #[inline(always)]
    pub fn iso0txptr1(&mut self) -> ISO0TXPTR1_W {
        ISO0TXPTR1_W { w: self }
    }
}
