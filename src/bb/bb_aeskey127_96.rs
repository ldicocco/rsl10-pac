#[doc = "Reader of register BB_AESKEY127_96"]
pub type R = crate::R<u32, super::BB_AESKEY127_96>;
#[doc = "Writer for register BB_AESKEY127_96"]
pub type W = crate::W<u32, super::BB_AESKEY127_96>;
#[doc = "Register BB_AESKEY127_96 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_AESKEY127_96 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "AES encryption 128-bit key (bits 127 down to 96)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum AESKEY127_96_A {
    #[doc = "0: `0`"]
    AESKEY127_96_0 = 0,
}
impl From<AESKEY127_96_A> for u32 {
    #[inline(always)]
    fn from(variant: AESKEY127_96_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AESKEY127_96`"]
pub type AESKEY127_96_R = crate::R<u32, AESKEY127_96_A>;
impl AESKEY127_96_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, AESKEY127_96_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AESKEY127_96_A::AESKEY127_96_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AESKEY127_96_0`"]
    #[inline(always)]
    pub fn is_aeskey127_96_0(&self) -> bool {
        *self == AESKEY127_96_A::AESKEY127_96_0
    }
}
#[doc = "Write proxy for field `AESKEY127_96`"]
pub struct AESKEY127_96_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEY127_96_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESKEY127_96_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aeskey127_96_0(self) -> &'a mut W {
        self.variant(AESKEY127_96_A::AESKEY127_96_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AES encryption 128-bit key (bits 127 down to 96)"]
    #[inline(always)]
    pub fn aeskey127_96(&self) -> AESKEY127_96_R {
        AESKEY127_96_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES encryption 128-bit key (bits 127 down to 96)"]
    #[inline(always)]
    pub fn aeskey127_96(&mut self) -> AESKEY127_96_W {
        AESKEY127_96_W { w: self }
    }
}
