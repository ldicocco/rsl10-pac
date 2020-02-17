#[doc = "Reader of register SYSCTRL_RF_ACCESS_CFG"]
pub type R = crate::R<u32, super::SYSCTRL_RF_ACCESS_CFG>;
#[doc = "Writer for register SYSCTRL_RF_ACCESS_CFG"]
pub type W = crate::W<u32, super::SYSCTRL_RF_ACCESS_CFG>;
#[doc = "Register SYSCTRL_RF_ACCESS_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCTRL_RF_ACCESS_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RF IRQ access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_IRQ_ACCESS_A {
    #[doc = "0: RF IRQ access disabled"]
    RF_IRQ_ACCESS_DISABLE = 0,
    #[doc = "1: RF IRQ access enabled"]
    RF_IRQ_ACCESS_ENABLE = 1,
}
impl From<RF_IRQ_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: RF_IRQ_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RF_IRQ_ACCESS`"]
pub type RF_IRQ_ACCESS_R = crate::R<bool, RF_IRQ_ACCESS_A>;
impl RF_IRQ_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_IRQ_ACCESS_A {
        match self.bits {
            false => RF_IRQ_ACCESS_A::RF_IRQ_ACCESS_DISABLE,
            true => RF_IRQ_ACCESS_A::RF_IRQ_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `RF_IRQ_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_rf_irq_access_disable(&self) -> bool {
        *self == RF_IRQ_ACCESS_A::RF_IRQ_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `RF_IRQ_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_rf_irq_access_enable(&self) -> bool {
        *self == RF_IRQ_ACCESS_A::RF_IRQ_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `RF_IRQ_ACCESS`"]
pub struct RF_IRQ_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_IRQ_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF_IRQ_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RF IRQ access disabled"]
    #[inline(always)]
    pub fn rf_irq_access_disable(self) -> &'a mut W {
        self.variant(RF_IRQ_ACCESS_A::RF_IRQ_ACCESS_DISABLE)
    }
    #[doc = "RF IRQ access enabled"]
    #[inline(always)]
    pub fn rf_irq_access_enable(self) -> &'a mut W {
        self.variant(RF_IRQ_ACCESS_A::RF_IRQ_ACCESS_ENABLE)
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
#[doc = "RF access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_ACCESS_A {
    #[doc = "0: RF access disabled"]
    RF_ACCESS_DISABLE = 0,
    #[doc = "1: RF access enabled"]
    RF_ACCESS_ENABLE = 1,
}
impl From<RF_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: RF_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RF_ACCESS`"]
pub type RF_ACCESS_R = crate::R<bool, RF_ACCESS_A>;
impl RF_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_ACCESS_A {
        match self.bits {
            false => RF_ACCESS_A::RF_ACCESS_DISABLE,
            true => RF_ACCESS_A::RF_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `RF_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_rf_access_disable(&self) -> bool {
        *self == RF_ACCESS_A::RF_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `RF_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_rf_access_enable(&self) -> bool {
        *self == RF_ACCESS_A::RF_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `RF_ACCESS`"]
pub struct RF_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RF access disabled"]
    #[inline(always)]
    pub fn rf_access_disable(self) -> &'a mut W {
        self.variant(RF_ACCESS_A::RF_ACCESS_DISABLE)
    }
    #[doc = "RF access enabled"]
    #[inline(always)]
    pub fn rf_access_enable(self) -> &'a mut W {
        self.variant(RF_ACCESS_A::RF_ACCESS_ENABLE)
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
    #[doc = "Bit 1 - RF IRQ access configuration"]
    #[inline(always)]
    pub fn rf_irq_access(&self) -> RF_IRQ_ACCESS_R {
        RF_IRQ_ACCESS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RF access configuration"]
    #[inline(always)]
    pub fn rf_access(&self) -> RF_ACCESS_R {
        RF_ACCESS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RF IRQ access configuration"]
    #[inline(always)]
    pub fn rf_irq_access(&mut self) -> RF_IRQ_ACCESS_W {
        RF_IRQ_ACCESS_W { w: self }
    }
    #[doc = "Bit 0 - RF access configuration"]
    #[inline(always)]
    pub fn rf_access(&mut self) -> RF_ACCESS_W {
        RF_ACCESS_W { w: self }
    }
}
