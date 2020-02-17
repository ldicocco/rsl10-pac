#[doc = "Reader of register BB_AESKEY63_32"]
pub type R = crate::R<u32, super::BB_AESKEY63_32>;
#[doc = "Writer for register BB_AESKEY63_32"]
pub type W = crate::W<u32, super::BB_AESKEY63_32>;
#[doc = "Register BB_AESKEY63_32 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_AESKEY63_32 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "AES encryption 128-bit key (bits 63 down to 32)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum AESKEY63_32_A {
    #[doc = "0: `0`"]
    AESKEY63_32_0 = 0,
}
impl From<AESKEY63_32_A> for u32 {
    #[inline(always)]
    fn from(variant: AESKEY63_32_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AESKEY63_32`"]
pub type AESKEY63_32_R = crate::R<u32, AESKEY63_32_A>;
impl AESKEY63_32_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, AESKEY63_32_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AESKEY63_32_A::AESKEY63_32_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AESKEY63_32_0`"]
    #[inline(always)]
    pub fn is_aeskey63_32_0(&self) -> bool {
        *self == AESKEY63_32_A::AESKEY63_32_0
    }
}
#[doc = "Write proxy for field `AESKEY63_32`"]
pub struct AESKEY63_32_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEY63_32_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESKEY63_32_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aeskey63_32_0(self) -> &'a mut W {
        self.variant(AESKEY63_32_A::AESKEY63_32_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AES encryption 128-bit key (bits 63 down to 32)"]
    #[inline(always)]
    pub fn aeskey63_32(&self) -> AESKEY63_32_R {
        AESKEY63_32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES encryption 128-bit key (bits 63 down to 32)"]
    #[inline(always)]
    pub fn aeskey63_32(&mut self) -> AESKEY63_32_W {
        AESKEY63_32_W { w: self }
    }
}
