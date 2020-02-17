#[doc = "Reader of register BB_AESKEY95_64"]
pub type R = crate::R<u32, super::BB_AESKEY95_64>;
#[doc = "Writer for register BB_AESKEY95_64"]
pub type W = crate::W<u32, super::BB_AESKEY95_64>;
#[doc = "Register BB_AESKEY95_64 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_AESKEY95_64 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "AES encryption 128-bit key (bits 95 down to 64)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum AESKEY95_64_A {
    #[doc = "0: `0`"]
    AESKEY95_64_0 = 0,
}
impl From<AESKEY95_64_A> for u32 {
    #[inline(always)]
    fn from(variant: AESKEY95_64_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AESKEY95_64`"]
pub type AESKEY95_64_R = crate::R<u32, AESKEY95_64_A>;
impl AESKEY95_64_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, AESKEY95_64_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AESKEY95_64_A::AESKEY95_64_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AESKEY95_64_0`"]
    #[inline(always)]
    pub fn is_aeskey95_64_0(&self) -> bool {
        *self == AESKEY95_64_A::AESKEY95_64_0
    }
}
#[doc = "Write proxy for field `AESKEY95_64`"]
pub struct AESKEY95_64_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEY95_64_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESKEY95_64_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aeskey95_64_0(self) -> &'a mut W {
        self.variant(AESKEY95_64_A::AESKEY95_64_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AES encryption 128-bit key (bits 95 down to 64)"]
    #[inline(always)]
    pub fn aeskey95_64(&self) -> AESKEY95_64_R {
        AESKEY95_64_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES encryption 128-bit key (bits 95 down to 64)"]
    #[inline(always)]
    pub fn aeskey95_64(&mut self) -> AESKEY95_64_W {
        AESKEY95_64_W { w: self }
    }
}
