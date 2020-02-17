#[doc = "Reader of register PWM_CFG[%s]"]
pub type R = crate::R<u32, super::PWM_CFG>;
#[doc = "Writer for register PWM_CFG[%s]"]
pub type W = crate::W<u32, super::PWM_CFG>;
#[doc = "Register PWM_CFG[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWM_HIGH`"]
pub type PWM_HIGH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PWM_HIGH`"]
pub struct PWM_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PWM_PERIOD`"]
pub type PWM_PERIOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PWM_PERIOD`"]
pub struct PWM_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - PWM high duty cycle"]
    #[inline(always)]
    pub fn pwm_high(&self) -> PWM_HIGH_R {
        PWM_HIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - PWM period"]
    #[inline(always)]
    pub fn pwm_period(&self) -> PWM_PERIOD_R {
        PWM_PERIOD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - PWM high duty cycle"]
    #[inline(always)]
    pub fn pwm_high(&mut self) -> PWM_HIGH_W {
        PWM_HIGH_W { w: self }
    }
    #[doc = "Bits 0:7 - PWM period"]
    #[inline(always)]
    pub fn pwm_period(&mut self) -> PWM_PERIOD_W {
        PWM_PERIOD_W { w: self }
    }
}
