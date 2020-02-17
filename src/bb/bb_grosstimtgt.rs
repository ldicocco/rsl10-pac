#[doc = "Reader of register BB_GROSSTIMTGT"]
pub type R = crate::R<u32, super::BB_GROSSTIMTGT>;
#[doc = "Writer for register BB_GROSSTIMTGT"]
pub type W = crate::W<u32, super::BB_GROSSTIMTGT>;
#[doc = "Register BB_GROSSTIMTGT `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_GROSSTIMTGT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Gross timer target value on which a ble_grosstgtim_irq must be generated (precision of 10ms)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GROSSTARGET_A {
    #[doc = "0: `0`"]
    GROSSTARGET_0 = 0,
}
impl From<GROSSTARGET_A> for u32 {
    #[inline(always)]
    fn from(variant: GROSSTARGET_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GROSSTARGET`"]
pub type GROSSTARGET_R = crate::R<u32, GROSSTARGET_A>;
impl GROSSTARGET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, GROSSTARGET_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GROSSTARGET_A::GROSSTARGET_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GROSSTARGET_0`"]
    #[inline(always)]
    pub fn is_grosstarget_0(&self) -> bool {
        *self == GROSSTARGET_A::GROSSTARGET_0
    }
}
#[doc = "Write proxy for field `GROSSTARGET`"]
pub struct GROSSTARGET_W<'a> {
    w: &'a mut W,
}
impl<'a> GROSSTARGET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GROSSTARGET_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn grosstarget_0(self) -> &'a mut W {
        self.variant(GROSSTARGET_A::GROSSTARGET_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x007f_ffff) | ((value as u32) & 0x007f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:22 - Gross timer target value on which a ble_grosstgtim_irq must be generated (precision of 10ms)"]
    #[inline(always)]
    pub fn grosstarget(&self) -> GROSSTARGET_R {
        GROSSTARGET_R::new((self.bits & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:22 - Gross timer target value on which a ble_grosstgtim_irq must be generated (precision of 10ms)"]
    #[inline(always)]
    pub fn grosstarget(&mut self) -> GROSSTARGET_W {
        GROSSTARGET_W { w: self }
    }
}
