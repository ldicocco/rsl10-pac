#[doc = "Reader of register BB_SPIPTRCNTL2"]
pub type R = crate::R<u32, super::BB_SPIPTRCNTL2>;
#[doc = "Writer for register BB_SPIPTRCNTL2"]
pub type W = crate::W<u32, super::BB_SPIPTRCNTL2>;
#[doc = "Register BB_SPIPTRCNTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_SPIPTRCNTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pointer to the RSSI read sequence address section\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum RSSIPTR_A {
    #[doc = "0: `0`"]
    RSSIPTR_0 = 0,
}
impl From<RSSIPTR_A> for u16 {
    #[inline(always)]
    fn from(variant: RSSIPTR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSSIPTR`"]
pub type RSSIPTR_R = crate::R<u16, RSSIPTR_A>;
impl RSSIPTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, RSSIPTR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RSSIPTR_A::RSSIPTR_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSIPTR_0`"]
    #[inline(always)]
    pub fn is_rssiptr_0(&self) -> bool {
        *self == RSSIPTR_A::RSSIPTR_0
    }
}
#[doc = "Write proxy for field `RSSIPTR`"]
pub struct RSSIPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSIPTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSIPTR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssiptr_0(self) -> &'a mut W {
        self.variant(RSSIPTR_A::RSSIPTR_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Pointer to the RSSI read sequence address section"]
    #[inline(always)]
    pub fn rssiptr(&self) -> RSSIPTR_R {
        RSSIPTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pointer to the RSSI read sequence address section"]
    #[inline(always)]
    pub fn rssiptr(&mut self) -> RSSIPTR_W {
        RSSIPTR_W { w: self }
    }
}
