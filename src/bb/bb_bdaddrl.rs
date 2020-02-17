#[doc = "Reader of register BB_BDADDRL"]
pub type R = crate::R<u32, super::BB_BDADDRL>;
#[doc = "Writer for register BB_BDADDRL"]
pub type W = crate::W<u32, super::BB_BDADDRL>;
#[doc = "Register BB_BDADDRL `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_BDADDRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "BLE device address (LSB part)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum BDADDRL_A {
    #[doc = "0: `0`"]
    BDADDRL_0 = 0,
}
impl From<BDADDRL_A> for u32 {
    #[inline(always)]
    fn from(variant: BDADDRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BDADDRL`"]
pub type BDADDRL_R = crate::R<u32, BDADDRL_A>;
impl BDADDRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, BDADDRL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BDADDRL_A::BDADDRL_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BDADDRL_0`"]
    #[inline(always)]
    pub fn is_bdaddrl_0(&self) -> bool {
        *self == BDADDRL_A::BDADDRL_0
    }
}
#[doc = "Write proxy for field `BDADDRL`"]
pub struct BDADDRL_W<'a> {
    w: &'a mut W,
}
impl<'a> BDADDRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BDADDRL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bdaddrl_0(self) -> &'a mut W {
        self.variant(BDADDRL_A::BDADDRL_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - BLE device address (LSB part)"]
    #[inline(always)]
    pub fn bdaddrl(&self) -> BDADDRL_R {
        BDADDRL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - BLE device address (LSB part)"]
    #[inline(always)]
    pub fn bdaddrl(&mut self) -> BDADDRL_W {
        BDADDRL_W { w: self }
    }
}
