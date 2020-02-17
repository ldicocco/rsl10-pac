#[doc = "Reader of register FLASH_MAIN_CTRL"]
pub type R = crate::R<u32, super::FLASH_MAIN_CTRL>;
#[doc = "Writer for register FLASH_MAIN_CTRL"]
pub type W = crate::W<u32, super::FLASH_MAIN_CTRL>;
#[doc = "Register FLASH_MAIN_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_MAIN_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Authorize the write access to the high part of the Flash MAIN block through the FLASH_IF registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAIN_HIGH_W_EN_A {
    #[doc = "0: The high part of the Flash MAIN block is protected against write access"]
    MAIN_HIGH_W_DISABLE = 0,
    #[doc = "1: The high part of the Flash MAIN block can be written"]
    MAIN_HIGH_W_ENABLE = 1,
}
impl From<MAIN_HIGH_W_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MAIN_HIGH_W_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAIN_HIGH_W_EN`"]
pub type MAIN_HIGH_W_EN_R = crate::R<bool, MAIN_HIGH_W_EN_A>;
impl MAIN_HIGH_W_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAIN_HIGH_W_EN_A {
        match self.bits {
            false => MAIN_HIGH_W_EN_A::MAIN_HIGH_W_DISABLE,
            true => MAIN_HIGH_W_EN_A::MAIN_HIGH_W_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `MAIN_HIGH_W_DISABLE`"]
    #[inline(always)]
    pub fn is_main_high_w_disable(&self) -> bool {
        *self == MAIN_HIGH_W_EN_A::MAIN_HIGH_W_DISABLE
    }
    #[doc = "Checks if the value of the field is `MAIN_HIGH_W_ENABLE`"]
    #[inline(always)]
    pub fn is_main_high_w_enable(&self) -> bool {
        *self == MAIN_HIGH_W_EN_A::MAIN_HIGH_W_ENABLE
    }
}
#[doc = "Write proxy for field `MAIN_HIGH_W_EN`"]
pub struct MAIN_HIGH_W_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_HIGH_W_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAIN_HIGH_W_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The high part of the Flash MAIN block is protected against write access"]
    #[inline(always)]
    pub fn main_high_w_disable(self) -> &'a mut W {
        self.variant(MAIN_HIGH_W_EN_A::MAIN_HIGH_W_DISABLE)
    }
    #[doc = "The high part of the Flash MAIN block can be written"]
    #[inline(always)]
    pub fn main_high_w_enable(self) -> &'a mut W {
        self.variant(MAIN_HIGH_W_EN_A::MAIN_HIGH_W_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Authorize the write access to the middle part of the Flash MAIN block through the FLASH_IF registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAIN_MIDDLE_W_EN_A {
    #[doc = "0: The middle part of the Flash MAIN block is protected against write access"]
    MAIN_MIDDLE_W_DISABLE = 0,
    #[doc = "1: The middle part of the Flash MAIN block can be written"]
    MAIN_MIDDLE_W_ENABLE = 1,
}
impl From<MAIN_MIDDLE_W_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MAIN_MIDDLE_W_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAIN_MIDDLE_W_EN`"]
pub type MAIN_MIDDLE_W_EN_R = crate::R<bool, MAIN_MIDDLE_W_EN_A>;
impl MAIN_MIDDLE_W_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAIN_MIDDLE_W_EN_A {
        match self.bits {
            false => MAIN_MIDDLE_W_EN_A::MAIN_MIDDLE_W_DISABLE,
            true => MAIN_MIDDLE_W_EN_A::MAIN_MIDDLE_W_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `MAIN_MIDDLE_W_DISABLE`"]
    #[inline(always)]
    pub fn is_main_middle_w_disable(&self) -> bool {
        *self == MAIN_MIDDLE_W_EN_A::MAIN_MIDDLE_W_DISABLE
    }
    #[doc = "Checks if the value of the field is `MAIN_MIDDLE_W_ENABLE`"]
    #[inline(always)]
    pub fn is_main_middle_w_enable(&self) -> bool {
        *self == MAIN_MIDDLE_W_EN_A::MAIN_MIDDLE_W_ENABLE
    }
}
#[doc = "Write proxy for field `MAIN_MIDDLE_W_EN`"]
pub struct MAIN_MIDDLE_W_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_MIDDLE_W_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAIN_MIDDLE_W_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The middle part of the Flash MAIN block is protected against write access"]
    #[inline(always)]
    pub fn main_middle_w_disable(self) -> &'a mut W {
        self.variant(MAIN_MIDDLE_W_EN_A::MAIN_MIDDLE_W_DISABLE)
    }
    #[doc = "The middle part of the Flash MAIN block can be written"]
    #[inline(always)]
    pub fn main_middle_w_enable(self) -> &'a mut W {
        self.variant(MAIN_MIDDLE_W_EN_A::MAIN_MIDDLE_W_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Authorize the write access to the lower part of the Flash MAIN block through the FLASH_IF registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAIN_LOW_W_EN_A {
    #[doc = "0: The lower part of the Flash MAIN block is protected against write access"]
    MAIN_LOW_W_DISABLE = 0,
    #[doc = "1: The lower part of the Flash MAIN block can be written"]
    MAIN_LOW_W_ENABLE = 1,
}
impl From<MAIN_LOW_W_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MAIN_LOW_W_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAIN_LOW_W_EN`"]
pub type MAIN_LOW_W_EN_R = crate::R<bool, MAIN_LOW_W_EN_A>;
impl MAIN_LOW_W_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAIN_LOW_W_EN_A {
        match self.bits {
            false => MAIN_LOW_W_EN_A::MAIN_LOW_W_DISABLE,
            true => MAIN_LOW_W_EN_A::MAIN_LOW_W_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `MAIN_LOW_W_DISABLE`"]
    #[inline(always)]
    pub fn is_main_low_w_disable(&self) -> bool {
        *self == MAIN_LOW_W_EN_A::MAIN_LOW_W_DISABLE
    }
    #[doc = "Checks if the value of the field is `MAIN_LOW_W_ENABLE`"]
    #[inline(always)]
    pub fn is_main_low_w_enable(&self) -> bool {
        *self == MAIN_LOW_W_EN_A::MAIN_LOW_W_ENABLE
    }
}
#[doc = "Write proxy for field `MAIN_LOW_W_EN`"]
pub struct MAIN_LOW_W_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_LOW_W_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAIN_LOW_W_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The lower part of the Flash MAIN block is protected against write access"]
    #[inline(always)]
    pub fn main_low_w_disable(self) -> &'a mut W {
        self.variant(MAIN_LOW_W_EN_A::MAIN_LOW_W_DISABLE)
    }
    #[doc = "The lower part of the Flash MAIN block can be written"]
    #[inline(always)]
    pub fn main_low_w_enable(self) -> &'a mut W {
        self.variant(MAIN_LOW_W_EN_A::MAIN_LOW_W_ENABLE)
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
    #[doc = "Bit 2 - Authorize the write access to the high part of the Flash MAIN block through the FLASH_IF registers."]
    #[inline(always)]
    pub fn main_high_w_en(&self) -> MAIN_HIGH_W_EN_R {
        MAIN_HIGH_W_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Authorize the write access to the middle part of the Flash MAIN block through the FLASH_IF registers."]
    #[inline(always)]
    pub fn main_middle_w_en(&self) -> MAIN_MIDDLE_W_EN_R {
        MAIN_MIDDLE_W_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Authorize the write access to the lower part of the Flash MAIN block through the FLASH_IF registers."]
    #[inline(always)]
    pub fn main_low_w_en(&self) -> MAIN_LOW_W_EN_R {
        MAIN_LOW_W_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Authorize the write access to the high part of the Flash MAIN block through the FLASH_IF registers."]
    #[inline(always)]
    pub fn main_high_w_en(&mut self) -> MAIN_HIGH_W_EN_W {
        MAIN_HIGH_W_EN_W { w: self }
    }
    #[doc = "Bit 1 - Authorize the write access to the middle part of the Flash MAIN block through the FLASH_IF registers."]
    #[inline(always)]
    pub fn main_middle_w_en(&mut self) -> MAIN_MIDDLE_W_EN_W {
        MAIN_MIDDLE_W_EN_W { w: self }
    }
    #[doc = "Bit 0 - Authorize the write access to the lower part of the Flash MAIN block through the FLASH_IF registers."]
    #[inline(always)]
    pub fn main_low_w_en(&mut self) -> MAIN_LOW_W_EN_W {
        MAIN_LOW_W_EN_W { w: self }
    }
}
