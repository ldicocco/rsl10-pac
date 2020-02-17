#[doc = "Reader of register BB_SWPROFILING"]
pub type R = crate::R<u32, super::BB_SWPROFILING>;
#[doc = "Writer for register BB_SWPROFILING"]
pub type W = crate::W<u32, super::BB_SWPROFILING>;
#[doc = "Register BB_SWPROFILING `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_SWPROFILING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Software profiling register: used by RW-BLE software for profiling purpose\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SWPROF_A {
    #[doc = "0: `0`"]
    SWPROF_0 = 0,
}
impl From<SWPROF_A> for u32 {
    #[inline(always)]
    fn from(variant: SWPROF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SWPROF`"]
pub type SWPROF_R = crate::R<u32, SWPROF_A>;
impl SWPROF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, SWPROF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SWPROF_A::SWPROF_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWPROF_0`"]
    #[inline(always)]
    pub fn is_swprof_0(&self) -> bool {
        *self == SWPROF_A::SWPROF_0
    }
}
#[doc = "Write proxy for field `SWPROF`"]
pub struct SWPROF_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPROF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWPROF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn swprof_0(self) -> &'a mut W {
        self.variant(SWPROF_A::SWPROF_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Software profiling register: used by RW-BLE software for profiling purpose"]
    #[inline(always)]
    pub fn swprof(&self) -> SWPROF_R {
        SWPROF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Software profiling register: used by RW-BLE software for profiling purpose"]
    #[inline(always)]
    pub fn swprof(&mut self) -> SWPROF_W {
        SWPROF_W { w: self }
    }
}
