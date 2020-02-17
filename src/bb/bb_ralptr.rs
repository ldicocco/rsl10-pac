#[doc = "Reader of register BB_RALPTR"]
pub type R = crate::R<u32, super::BB_RALPTR>;
#[doc = "Writer for register BB_RALPTR"]
pub type W = crate::W<u32, super::BB_RALPTR>;
#[doc = "Register BB_RALPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_RALPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Start address pointer of the RAL structure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum RALPTR_A {
    #[doc = "0: `0`"]
    RALPTR_0 = 0,
}
impl From<RALPTR_A> for u16 {
    #[inline(always)]
    fn from(variant: RALPTR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RALPTR`"]
pub type RALPTR_R = crate::R<u16, RALPTR_A>;
impl RALPTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, RALPTR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RALPTR_A::RALPTR_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RALPTR_0`"]
    #[inline(always)]
    pub fn is_ralptr_0(&self) -> bool {
        *self == RALPTR_A::RALPTR_0
    }
}
#[doc = "Write proxy for field `RALPTR`"]
pub struct RALPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RALPTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RALPTR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ralptr_0(self) -> &'a mut W {
        self.variant(RALPTR_A::RALPTR_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Start address pointer of the RAL structure"]
    #[inline(always)]
    pub fn ralptr(&self) -> RALPTR_R {
        RALPTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Start address pointer of the RAL structure"]
    #[inline(always)]
    pub fn ralptr(&mut self) -> RALPTR_W {
        RALPTR_W { w: self }
    }
}
