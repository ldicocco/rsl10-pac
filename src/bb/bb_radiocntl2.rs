#[doc = "Reader of register BB_RADIOCNTL2"]
pub type R = crate::R<u32, super::BB_RADIOCNTL2>;
#[doc = "Writer for register BB_RADIOCNTL2"]
pub type W = crate::W<u32, super::BB_RADIOCNTL2>;
#[doc = "Register BB_RADIOCNTL2 `reset()`'s with value 0x40"]
impl crate::ResetValue for super::BB_RADIOCNTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x40
    }
}
#[doc = "Frequency table pointer\n\nValue on reset: 64"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum FREQTABLE_PTR_A {
    #[doc = "64: `1000000`"]
    FREQTABLE_PTR_64 = 64,
}
impl From<FREQTABLE_PTR_A> for u16 {
    #[inline(always)]
    fn from(variant: FREQTABLE_PTR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FREQTABLE_PTR`"]
pub type FREQTABLE_PTR_R = crate::R<u16, FREQTABLE_PTR_A>;
impl FREQTABLE_PTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, FREQTABLE_PTR_A> {
        use crate::Variant::*;
        match self.bits {
            64 => Val(FREQTABLE_PTR_A::FREQTABLE_PTR_64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FREQTABLE_PTR_64`"]
    #[inline(always)]
    pub fn is_freqtable_ptr_64(&self) -> bool {
        *self == FREQTABLE_PTR_A::FREQTABLE_PTR_64
    }
}
#[doc = "Write proxy for field `FREQTABLE_PTR`"]
pub struct FREQTABLE_PTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQTABLE_PTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQTABLE_PTR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1000000`"]
    #[inline(always)]
    pub fn freqtable_ptr_64(self) -> &'a mut W {
        self.variant(FREQTABLE_PTR_A::FREQTABLE_PTR_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Frequency table pointer"]
    #[inline(always)]
    pub fn freqtable_ptr(&self) -> FREQTABLE_PTR_R {
        FREQTABLE_PTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frequency table pointer"]
    #[inline(always)]
    pub fn freqtable_ptr(&mut self) -> FREQTABLE_PTR_W {
        FREQTABLE_PTR_W { w: self }
    }
}
