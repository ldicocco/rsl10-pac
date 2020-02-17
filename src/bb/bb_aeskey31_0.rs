#[doc = "Reader of register BB_AESKEY31_0"]
pub type R = crate::R<u32, super::BB_AESKEY31_0>;
#[doc = "Writer for register BB_AESKEY31_0"]
pub type W = crate::W<u32, super::BB_AESKEY31_0>;
#[doc = "Register BB_AESKEY31_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_AESKEY31_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "AES encryption 128-bit key (bits 31 down to 0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum AESKEY31_0_A {
    #[doc = "0: `0`"]
    AESKEY31_0_0 = 0,
}
impl From<AESKEY31_0_A> for u32 {
    #[inline(always)]
    fn from(variant: AESKEY31_0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AESKEY31_0`"]
pub type AESKEY31_0_R = crate::R<u32, AESKEY31_0_A>;
impl AESKEY31_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, AESKEY31_0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AESKEY31_0_A::AESKEY31_0_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AESKEY31_0_0`"]
    #[inline(always)]
    pub fn is_aeskey31_0_0(&self) -> bool {
        *self == AESKEY31_0_A::AESKEY31_0_0
    }
}
#[doc = "Write proxy for field `AESKEY31_0`"]
pub struct AESKEY31_0_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEY31_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESKEY31_0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aeskey31_0_0(self) -> &'a mut W {
        self.variant(AESKEY31_0_A::AESKEY31_0_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AES encryption 128-bit key (bits 31 down to 0)"]
    #[inline(always)]
    pub fn aeskey31_0(&self) -> AESKEY31_0_R {
        AESKEY31_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES encryption 128-bit key (bits 31 down to 0)"]
    #[inline(always)]
    pub fn aeskey31_0(&mut self) -> AESKEY31_0_W {
        AESKEY31_0_W { w: self }
    }
}
