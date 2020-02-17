#[doc = "Reader of register BB_AESPTR"]
pub type R = crate::R<u32, super::BB_AESPTR>;
#[doc = "Writer for register BB_AESPTR"]
pub type W = crate::W<u32, super::BB_AESPTR>;
#[doc = "Register BB_AESPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_AESPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pointer to the memory zone where the block to cipher/decipher using AES-128 is stored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum AESPTR_A {
    #[doc = "0: `0`"]
    AESPTR_0 = 0,
}
impl From<AESPTR_A> for u16 {
    #[inline(always)]
    fn from(variant: AESPTR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AESPTR`"]
pub type AESPTR_R = crate::R<u16, AESPTR_A>;
impl AESPTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, AESPTR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AESPTR_A::AESPTR_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AESPTR_0`"]
    #[inline(always)]
    pub fn is_aesptr_0(&self) -> bool {
        *self == AESPTR_A::AESPTR_0
    }
}
#[doc = "Write proxy for field `AESPTR`"]
pub struct AESPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> AESPTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESPTR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aesptr_0(self) -> &'a mut W {
        self.variant(AESPTR_A::AESPTR_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Pointer to the memory zone where the block to cipher/decipher using AES-128 is stored."]
    #[inline(always)]
    pub fn aesptr(&self) -> AESPTR_R {
        AESPTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pointer to the memory zone where the block to cipher/decipher using AES-128 is stored."]
    #[inline(always)]
    pub fn aesptr(&mut self) -> AESPTR_W {
        AESPTR_W { w: self }
    }
}
