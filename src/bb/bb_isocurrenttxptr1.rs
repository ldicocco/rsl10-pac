#[doc = "Reader of register BB_ISOCURRENTTXPTR1"]
pub type R = crate::R<u32, super::BB_ISOCURRENTTXPTR1>;
#[doc = "Writer for register BB_ISOCURRENTTXPTR1"]
pub type W = crate::W<u32, super::BB_ISOCURRENTTXPTR1>;
#[doc = "Register BB_ISOCURRENTTXPTR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_ISOCURRENTTXPTR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Tx ISO Buffer pointer 0 of ISO Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ISO1TXPTR0_A {
    #[doc = "0: Tx ISO Buffer pointer 0 of ISO Channel 1"]
    ISO1TXPTR0_0 = 0,
}
impl From<ISO1TXPTR0_A> for u16 {
    #[inline(always)]
    fn from(variant: ISO1TXPTR0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ISO1TXPTR0`"]
pub type ISO1TXPTR0_R = crate::R<u16, ISO1TXPTR0_A>;
impl ISO1TXPTR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ISO1TXPTR0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ISO1TXPTR0_A::ISO1TXPTR0_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ISO1TXPTR0_0`"]
    #[inline(always)]
    pub fn is_iso1txptr0_0(&self) -> bool {
        *self == ISO1TXPTR0_A::ISO1TXPTR0_0
    }
}
#[doc = "Write proxy for field `ISO1TXPTR0`"]
pub struct ISO1TXPTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO1TXPTR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISO1TXPTR0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Tx ISO Buffer pointer 0 of ISO Channel 1"]
    #[inline(always)]
    pub fn iso1txptr0_0(self) -> &'a mut W {
        self.variant(ISO1TXPTR0_A::ISO1TXPTR0_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Tx ISO Buffer pointer 1 of ISO Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ISO1TXPTR1_A {
    #[doc = "0: Tx ISO Buffer pointer 1 of ISO Channel 1"]
    ISO1TXPTR1_0 = 0,
}
impl From<ISO1TXPTR1_A> for u16 {
    #[inline(always)]
    fn from(variant: ISO1TXPTR1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ISO1TXPTR1`"]
pub type ISO1TXPTR1_R = crate::R<u16, ISO1TXPTR1_A>;
impl ISO1TXPTR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ISO1TXPTR1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ISO1TXPTR1_A::ISO1TXPTR1_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ISO1TXPTR1_0`"]
    #[inline(always)]
    pub fn is_iso1txptr1_0(&self) -> bool {
        *self == ISO1TXPTR1_A::ISO1TXPTR1_0
    }
}
#[doc = "Write proxy for field `ISO1TXPTR1`"]
pub struct ISO1TXPTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO1TXPTR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISO1TXPTR1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Tx ISO Buffer pointer 1 of ISO Channel 1"]
    #[inline(always)]
    pub fn iso1txptr1_0(self) -> &'a mut W {
        self.variant(ISO1TXPTR1_A::ISO1TXPTR1_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Tx ISO Buffer pointer 0 of ISO Channel 1"]
    #[inline(always)]
    pub fn iso1txptr0(&self) -> ISO1TXPTR0_R {
        ISO1TXPTR0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Tx ISO Buffer pointer 1 of ISO Channel 1"]
    #[inline(always)]
    pub fn iso1txptr1(&self) -> ISO1TXPTR1_R {
        ISO1TXPTR1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Tx ISO Buffer pointer 0 of ISO Channel 1"]
    #[inline(always)]
    pub fn iso1txptr0(&mut self) -> ISO1TXPTR0_W {
        ISO1TXPTR0_W { w: self }
    }
    #[doc = "Bits 0:15 - Tx ISO Buffer pointer 1 of ISO Channel 1"]
    #[inline(always)]
    pub fn iso1txptr1(&mut self) -> ISO1TXPTR1_W {
        ISO1TXPTR1_W { w: self }
    }
}
