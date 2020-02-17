#[doc = "Reader of register BB_FINETIMTGT"]
pub type R = crate::R<u32, super::BB_FINETIMTGT>;
#[doc = "Writer for register BB_FINETIMTGT"]
pub type W = crate::W<u32, super::BB_FINETIMTGT>;
#[doc = "Register BB_FINETIMTGT `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_FINETIMTGT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Fine timer target value on which a ble_finetgtim_irq must be generated (precision of 625us)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum FINETARGET_A {
    #[doc = "0: `0`"]
    FINETARGET_0 = 0,
}
impl From<FINETARGET_A> for u32 {
    #[inline(always)]
    fn from(variant: FINETARGET_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FINETARGET`"]
pub type FINETARGET_R = crate::R<u32, FINETARGET_A>;
impl FINETARGET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, FINETARGET_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FINETARGET_A::FINETARGET_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FINETARGET_0`"]
    #[inline(always)]
    pub fn is_finetarget_0(&self) -> bool {
        *self == FINETARGET_A::FINETARGET_0
    }
}
#[doc = "Write proxy for field `FINETARGET`"]
pub struct FINETARGET_W<'a> {
    w: &'a mut W,
}
impl<'a> FINETARGET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FINETARGET_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn finetarget_0(self) -> &'a mut W {
        self.variant(FINETARGET_A::FINETARGET_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff_ffff) | ((value as u32) & 0x07ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:26 - Fine timer target value on which a ble_finetgtim_irq must be generated (precision of 625us)"]
    #[inline(always)]
    pub fn finetarget(&self) -> FINETARGET_R {
        FINETARGET_R::new((self.bits & 0x07ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:26 - Fine timer target value on which a ble_finetgtim_irq must be generated (precision of 625us)"]
    #[inline(always)]
    pub fn finetarget(&mut self) -> FINETARGET_W {
        FINETARGET_W { w: self }
    }
}
