#[doc = "Reader of register SYSCTRL_LPDSP32_DEBUG_CFG"]
pub type R = crate::R<u32, super::SYSCTRL_LPDSP32_DEBUG_CFG>;
#[doc = "Writer for register SYSCTRL_LPDSP32_DEBUG_CFG"]
pub type W = crate::W<u32, super::SYSCTRL_LPDSP32_DEBUG_CFG>;
#[doc = "Register SYSCTRL_LPDSP32_DEBUG_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCTRL_LPDSP32_DEBUG_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LPDSP32 exit powerdown mode configuration when halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_A {
    #[doc = "0: LPDSP32 exit powerdown when halted disabled"]
    LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_DISABLED = 0,
    #[doc = "1: LPDSP32 exit powerdown when halted enabled"]
    LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_ENABLED = 1,
}
impl From<LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_A> for bool {
    #[inline(always)]
    fn from(variant: LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPDSP32_EXIT_POWERDOWN_WHEN_HALTED`"]
pub type LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_R =
    crate::R<bool, LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_A>;
impl LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_A {
        match self.bits {
            false => {
                LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_A::LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_DISABLED
            }
            true => {
                LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_A::LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_ENABLED
            }
        }
    }
    #[doc = "Checks if the value of the field is `LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_DISABLED`"]
    #[inline(always)]
    pub fn is_lpdsp32_exit_powerdown_when_halted_disabled(&self) -> bool {
        *self == LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_A::LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_DISABLED
    }
    #[doc = "Checks if the value of the field is `LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_ENABLED`"]
    #[inline(always)]
    pub fn is_lpdsp32_exit_powerdown_when_halted_enabled(&self) -> bool {
        *self == LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_A::LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_ENABLED
    }
}
#[doc = "Write proxy for field `LPDSP32_EXIT_POWERDOWN_WHEN_HALTED`"]
pub struct LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LPDSP32 exit powerdown when halted disabled"]
    #[inline(always)]
    pub fn lpdsp32_exit_powerdown_when_halted_disabled(self) -> &'a mut W {
        self.variant(
            LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_A::LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_DISABLED,
        )
    }
    #[doc = "LPDSP32 exit powerdown when halted enabled"]
    #[inline(always)]
    pub fn lpdsp32_exit_powerdown_when_halted_enabled(self) -> &'a mut W {
        self.variant(
            LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_A::LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_ENABLED,
        )
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
#[doc = "LPDSP32 debug port enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDSP32_DEBUG_ENABLE_A {
    #[doc = "0: LPDSP32 debug port disabled"]
    LPDSP32_DEBUG_DISABLED = 0,
    #[doc = "1: LPDSP32 debug port enabled"]
    LPDSP32_DEBUG_ENABLED = 1,
}
impl From<LPDSP32_DEBUG_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: LPDSP32_DEBUG_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPDSP32_DEBUG_ENABLE`"]
pub type LPDSP32_DEBUG_ENABLE_R = crate::R<bool, LPDSP32_DEBUG_ENABLE_A>;
impl LPDSP32_DEBUG_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDSP32_DEBUG_ENABLE_A {
        match self.bits {
            false => LPDSP32_DEBUG_ENABLE_A::LPDSP32_DEBUG_DISABLED,
            true => LPDSP32_DEBUG_ENABLE_A::LPDSP32_DEBUG_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `LPDSP32_DEBUG_DISABLED`"]
    #[inline(always)]
    pub fn is_lpdsp32_debug_disabled(&self) -> bool {
        *self == LPDSP32_DEBUG_ENABLE_A::LPDSP32_DEBUG_DISABLED
    }
    #[doc = "Checks if the value of the field is `LPDSP32_DEBUG_ENABLED`"]
    #[inline(always)]
    pub fn is_lpdsp32_debug_enabled(&self) -> bool {
        *self == LPDSP32_DEBUG_ENABLE_A::LPDSP32_DEBUG_ENABLED
    }
}
#[doc = "Write proxy for field `LPDSP32_DEBUG_ENABLE`"]
pub struct LPDSP32_DEBUG_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDSP32_DEBUG_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPDSP32_DEBUG_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LPDSP32 debug port disabled"]
    #[inline(always)]
    pub fn lpdsp32_debug_disabled(self) -> &'a mut W {
        self.variant(LPDSP32_DEBUG_ENABLE_A::LPDSP32_DEBUG_DISABLED)
    }
    #[doc = "LPDSP32 debug port enabled"]
    #[inline(always)]
    pub fn lpdsp32_debug_enabled(self) -> &'a mut W {
        self.variant(LPDSP32_DEBUG_ENABLE_A::LPDSP32_DEBUG_ENABLED)
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
    #[doc = "Bit 1 - LPDSP32 exit powerdown mode configuration when halted"]
    #[inline(always)]
    pub fn lpdsp32_exit_powerdown_when_halted(&self) -> LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_R {
        LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LPDSP32 debug port enable"]
    #[inline(always)]
    pub fn lpdsp32_debug_enable(&self) -> LPDSP32_DEBUG_ENABLE_R {
        LPDSP32_DEBUG_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - LPDSP32 exit powerdown mode configuration when halted"]
    #[inline(always)]
    pub fn lpdsp32_exit_powerdown_when_halted(&mut self) -> LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_W {
        LPDSP32_EXIT_POWERDOWN_WHEN_HALTED_W { w: self }
    }
    #[doc = "Bit 0 - LPDSP32 debug port enable"]
    #[inline(always)]
    pub fn lpdsp32_debug_enable(&mut self) -> LPDSP32_DEBUG_ENABLE_W {
        LPDSP32_DEBUG_ENABLE_W { w: self }
    }
}
