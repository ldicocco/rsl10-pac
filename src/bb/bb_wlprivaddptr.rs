#[doc = "Reader of register BB_WLPRIVADDPTR"]
pub type R = crate::R<u32, super::BB_WLPRIVADDPTR>;
#[doc = "Writer for register BB_WLPRIVADDPTR"]
pub type W = crate::W<u32, super::BB_WLPRIVADDPTR>;
#[doc = "Register BB_WLPRIVADDPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_WLPRIVADDPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Start address pointer of the private devices white list\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum WLPRIVADDPTR_A {
    #[doc = "0: `0`"]
    WLPRIVADDPTR_0 = 0,
}
impl From<WLPRIVADDPTR_A> for u16 {
    #[inline(always)]
    fn from(variant: WLPRIVADDPTR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WLPRIVADDPTR`"]
pub type WLPRIVADDPTR_R = crate::R<u16, WLPRIVADDPTR_A>;
impl WLPRIVADDPTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, WLPRIVADDPTR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WLPRIVADDPTR_A::WLPRIVADDPTR_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WLPRIVADDPTR_0`"]
    #[inline(always)]
    pub fn is_wlprivaddptr_0(&self) -> bool {
        *self == WLPRIVADDPTR_A::WLPRIVADDPTR_0
    }
}
#[doc = "Write proxy for field `WLPRIVADDPTR`"]
pub struct WLPRIVADDPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> WLPRIVADDPTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLPRIVADDPTR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn wlprivaddptr_0(self) -> &'a mut W {
        self.variant(WLPRIVADDPTR_A::WLPRIVADDPTR_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Start address pointer of the private devices white list"]
    #[inline(always)]
    pub fn wlprivaddptr(&self) -> WLPRIVADDPTR_R {
        WLPRIVADDPTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Start address pointer of the private devices white list"]
    #[inline(always)]
    pub fn wlprivaddptr(&mut self) -> WLPRIVADDPTR_W {
        WLPRIVADDPTR_W { w: self }
    }
}
