#[doc = "Reader of register SYSCTRL_RF_POWER_CFG"]
pub type R = crate::R<u32, super::SYSCTRL_RF_POWER_CFG>;
#[doc = "Writer for register SYSCTRL_RF_POWER_CFG"]
pub type W = crate::W<u32, super::SYSCTRL_RF_POWER_CFG>;
#[doc = "Register SYSCTRL_RF_POWER_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCTRL_RF_POWER_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RF power configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_POWER_A {
    #[doc = "0: RF power disabled"]
    RF_POWER_DISABLE = 0,
    #[doc = "1: RF power enabled"]
    RF_POWER_ENABLE = 1,
}
impl From<RF_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: RF_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RF_POWER`"]
pub type RF_POWER_R = crate::R<bool, RF_POWER_A>;
impl RF_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_POWER_A {
        match self.bits {
            false => RF_POWER_A::RF_POWER_DISABLE,
            true => RF_POWER_A::RF_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `RF_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_rf_power_disable(&self) -> bool {
        *self == RF_POWER_A::RF_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `RF_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_rf_power_enable(&self) -> bool {
        *self == RF_POWER_A::RF_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `RF_POWER`"]
pub struct RF_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RF power disabled"]
    #[inline(always)]
    pub fn rf_power_disable(self) -> &'a mut W {
        self.variant(RF_POWER_A::RF_POWER_DISABLE)
    }
    #[doc = "RF power enabled"]
    #[inline(always)]
    pub fn rf_power_enable(self) -> &'a mut W {
        self.variant(RF_POWER_A::RF_POWER_ENABLE)
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
    #[doc = "Bit 0 - RF power configuration"]
    #[inline(always)]
    pub fn rf_power(&self) -> RF_POWER_R {
        RF_POWER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RF power configuration"]
    #[inline(always)]
    pub fn rf_power(&mut self) -> RF_POWER_W {
        RF_POWER_W { w: self }
    }
}
