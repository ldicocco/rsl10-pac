#[doc = "Reader of register PWM_CTRL"]
pub type R = crate::R<u32, super::PWM_CTRL>;
#[doc = "Writer for register PWM_CTRL"]
pub type W = crate::W<u32, super::PWM_CTRL>;
#[doc = "Register PWM_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable/disable the PWM offset function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_OFFSET_ENABLE_A {
    #[doc = "0: Disable the PWM offset"]
    PWM_OFFSET_DISABLE = 0,
    #[doc = "1: Enable the PWM offset"]
    PWM_OFFSET_ENABLE = 1,
}
impl From<PWM_OFFSET_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PWM_OFFSET_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWM_OFFSET_ENABLE`"]
pub type PWM_OFFSET_ENABLE_R = crate::R<bool, PWM_OFFSET_ENABLE_A>;
impl PWM_OFFSET_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_OFFSET_ENABLE_A {
        match self.bits {
            false => PWM_OFFSET_ENABLE_A::PWM_OFFSET_DISABLE,
            true => PWM_OFFSET_ENABLE_A::PWM_OFFSET_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_OFFSET_DISABLE`"]
    #[inline(always)]
    pub fn is_pwm_offset_disable(&self) -> bool {
        *self == PWM_OFFSET_ENABLE_A::PWM_OFFSET_DISABLE
    }
    #[doc = "Checks if the value of the field is `PWM_OFFSET_ENABLE`"]
    #[inline(always)]
    pub fn is_pwm_offset_enable(&self) -> bool {
        *self == PWM_OFFSET_ENABLE_A::PWM_OFFSET_ENABLE
    }
}
#[doc = "Write proxy for field `PWM_OFFSET_ENABLE`"]
pub struct PWM_OFFSET_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_OFFSET_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_OFFSET_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the PWM offset"]
    #[inline(always)]
    pub fn pwm_offset_disable(self) -> &'a mut W {
        self.variant(PWM_OFFSET_ENABLE_A::PWM_OFFSET_DISABLE)
    }
    #[doc = "Enable the PWM offset"]
    #[inline(always)]
    pub fn pwm_offset_enable(self) -> &'a mut W {
        self.variant(PWM_OFFSET_ENABLE_A::PWM_OFFSET_ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `PWM_OFFSET`"]
pub type PWM_OFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PWM_OFFSET`"]
pub struct PWM_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Enable/disable the PWM1 block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM1_ENABLE_A {
    #[doc = "0: Disable the PWM1 block"]
    PWM1_DISABLE = 0,
    #[doc = "1: Enable the PWM1 block"]
    PWM1_ENABLE = 1,
}
impl From<PWM1_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PWM1_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWM1_ENABLE`"]
pub type PWM1_ENABLE_R = crate::R<bool, PWM1_ENABLE_A>;
impl PWM1_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM1_ENABLE_A {
        match self.bits {
            false => PWM1_ENABLE_A::PWM1_DISABLE,
            true => PWM1_ENABLE_A::PWM1_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PWM1_DISABLE`"]
    #[inline(always)]
    pub fn is_pwm1_disable(&self) -> bool {
        *self == PWM1_ENABLE_A::PWM1_DISABLE
    }
    #[doc = "Checks if the value of the field is `PWM1_ENABLE`"]
    #[inline(always)]
    pub fn is_pwm1_enable(&self) -> bool {
        *self == PWM1_ENABLE_A::PWM1_ENABLE
    }
}
#[doc = "Write proxy for field `PWM1_ENABLE`"]
pub struct PWM1_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM1_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM1_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the PWM1 block"]
    #[inline(always)]
    pub fn pwm1_disable(self) -> &'a mut W {
        self.variant(PWM1_ENABLE_A::PWM1_DISABLE)
    }
    #[doc = "Enable the PWM1 block"]
    #[inline(always)]
    pub fn pwm1_enable(self) -> &'a mut W {
        self.variant(PWM1_ENABLE_A::PWM1_ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Enable/disable the PWM0 block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM0_ENABLE_A {
    #[doc = "0: Disable the PWM0 block"]
    PWM0_DISABLE = 0,
    #[doc = "1: Enable the PWM0 block"]
    PWM0_ENABLE = 1,
}
impl From<PWM0_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PWM0_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWM0_ENABLE`"]
pub type PWM0_ENABLE_R = crate::R<bool, PWM0_ENABLE_A>;
impl PWM0_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM0_ENABLE_A {
        match self.bits {
            false => PWM0_ENABLE_A::PWM0_DISABLE,
            true => PWM0_ENABLE_A::PWM0_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PWM0_DISABLE`"]
    #[inline(always)]
    pub fn is_pwm0_disable(&self) -> bool {
        *self == PWM0_ENABLE_A::PWM0_DISABLE
    }
    #[doc = "Checks if the value of the field is `PWM0_ENABLE`"]
    #[inline(always)]
    pub fn is_pwm0_enable(&self) -> bool {
        *self == PWM0_ENABLE_A::PWM0_ENABLE
    }
}
#[doc = "Write proxy for field `PWM0_ENABLE`"]
pub struct PWM0_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM0_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM0_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the PWM0 block"]
    #[inline(always)]
    pub fn pwm0_disable(self) -> &'a mut W {
        self.variant(PWM0_ENABLE_A::PWM0_DISABLE)
    }
    #[doc = "Enable the PWM0 block"]
    #[inline(always)]
    pub fn pwm0_enable(self) -> &'a mut W {
        self.variant(PWM0_ENABLE_A::PWM0_ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Enable/disable the PWM offset function"]
    #[inline(always)]
    pub fn pwm_offset_enable(&self) -> PWM_OFFSET_ENABLE_R {
        PWM_OFFSET_ENABLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - PWM0 to PWM1 offset"]
    #[inline(always)]
    pub fn pwm_offset(&self) -> PWM_OFFSET_R {
        PWM_OFFSET_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 4 - Enable/disable the PWM1 block"]
    #[inline(always)]
    pub fn pwm1_enable(&self) -> PWM1_ENABLE_R {
        PWM1_ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable/disable the PWM0 block"]
    #[inline(always)]
    pub fn pwm0_enable(&self) -> PWM0_ENABLE_R {
        PWM0_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Enable/disable the PWM offset function"]
    #[inline(always)]
    pub fn pwm_offset_enable(&mut self) -> PWM_OFFSET_ENABLE_W {
        PWM_OFFSET_ENABLE_W { w: self }
    }
    #[doc = "Bits 8:15 - PWM0 to PWM1 offset"]
    #[inline(always)]
    pub fn pwm_offset(&mut self) -> PWM_OFFSET_W {
        PWM_OFFSET_W { w: self }
    }
    #[doc = "Bit 4 - Enable/disable the PWM1 block"]
    #[inline(always)]
    pub fn pwm1_enable(&mut self) -> PWM1_ENABLE_W {
        PWM1_ENABLE_W { w: self }
    }
    #[doc = "Bit 0 - Enable/disable the PWM0 block"]
    #[inline(always)]
    pub fn pwm0_enable(&mut self) -> PWM0_ENABLE_W {
        PWM0_ENABLE_W { w: self }
    }
}
