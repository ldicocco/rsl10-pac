#[doc = "Reader of register BB_WLPUBADDPTR"]
pub type R = crate::R<u32, super::BB_WLPUBADDPTR>;
#[doc = "Writer for register BB_WLPUBADDPTR"]
pub type W = crate::W<u32, super::BB_WLPUBADDPTR>;
#[doc = "Register BB_WLPUBADDPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_WLPUBADDPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Start address pointer of the public devices white list\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum WLPUBADDPTR_A {
    #[doc = "0: `0`"]
    WLPUBADDPTR_0 = 0,
}
impl From<WLPUBADDPTR_A> for u16 {
    #[inline(always)]
    fn from(variant: WLPUBADDPTR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WLPUBADDPTR`"]
pub type WLPUBADDPTR_R = crate::R<u16, WLPUBADDPTR_A>;
impl WLPUBADDPTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, WLPUBADDPTR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WLPUBADDPTR_A::WLPUBADDPTR_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WLPUBADDPTR_0`"]
    #[inline(always)]
    pub fn is_wlpubaddptr_0(&self) -> bool {
        *self == WLPUBADDPTR_A::WLPUBADDPTR_0
    }
}
#[doc = "Write proxy for field `WLPUBADDPTR`"]
pub struct WLPUBADDPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> WLPUBADDPTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLPUBADDPTR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn wlpubaddptr_0(self) -> &'a mut W {
        self.variant(WLPUBADDPTR_A::WLPUBADDPTR_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Start address pointer of the public devices white list"]
    #[inline(always)]
    pub fn wlpubaddptr(&self) -> WLPUBADDPTR_R {
        WLPUBADDPTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Start address pointer of the public devices white list"]
    #[inline(always)]
    pub fn wlpubaddptr(&mut self) -> WLPUBADDPTR_W {
        WLPUBADDPTR_W { w: self }
    }
}
