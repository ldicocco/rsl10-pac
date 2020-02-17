#[doc = "Reader of register SYSCTRL_FLASH_OVERLAY_CFG"]
pub type R = crate::R<u32, super::SYSCTRL_FLASH_OVERLAY_CFG>;
#[doc = "Writer for register SYSCTRL_FLASH_OVERLAY_CFG"]
pub type W = crate::W<u32, super::SYSCTRL_FLASH_OVERLAY_CFG>;
#[doc = "Register SYSCTRL_FLASH_OVERLAY_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCTRL_FLASH_OVERLAY_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DSP_PRAM0 Flash overlay configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_PRAM0_OVERLAY_CFG_A {
    #[doc = "0: DSP_PRAM0 is not mapped on the Flash addressing range"]
    DSP_PRAM0_OVERLAY_DISABLE = 0,
    #[doc = "1: DSP_PRAM0 is also mapped on the Flash addressing range"]
    DSP_PRAM0_OVERLAY_ENABLE = 1,
}
impl From<DSP_PRAM0_OVERLAY_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_PRAM0_OVERLAY_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_PRAM0_OVERLAY_CFG`"]
pub type DSP_PRAM0_OVERLAY_CFG_R = crate::R<bool, DSP_PRAM0_OVERLAY_CFG_A>;
impl DSP_PRAM0_OVERLAY_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_PRAM0_OVERLAY_CFG_A {
        match self.bits {
            false => DSP_PRAM0_OVERLAY_CFG_A::DSP_PRAM0_OVERLAY_DISABLE,
            true => DSP_PRAM0_OVERLAY_CFG_A::DSP_PRAM0_OVERLAY_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM0_OVERLAY_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram0_overlay_disable(&self) -> bool {
        *self == DSP_PRAM0_OVERLAY_CFG_A::DSP_PRAM0_OVERLAY_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM0_OVERLAY_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram0_overlay_enable(&self) -> bool {
        *self == DSP_PRAM0_OVERLAY_CFG_A::DSP_PRAM0_OVERLAY_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_PRAM0_OVERLAY_CFG`"]
pub struct DSP_PRAM0_OVERLAY_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM0_OVERLAY_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM0_OVERLAY_CFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP_PRAM0 is not mapped on the Flash addressing range"]
    #[inline(always)]
    pub fn dsp_pram0_overlay_disable(self) -> &'a mut W {
        self.variant(DSP_PRAM0_OVERLAY_CFG_A::DSP_PRAM0_OVERLAY_DISABLE)
    }
    #[doc = "DSP_PRAM0 is also mapped on the Flash addressing range"]
    #[inline(always)]
    pub fn dsp_pram0_overlay_enable(self) -> &'a mut W {
        self.variant(DSP_PRAM0_OVERLAY_CFG_A::DSP_PRAM0_OVERLAY_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "DSP_PRAM1 Flash overlay configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_PRAM1_OVERLAY_CFG_A {
    #[doc = "0: DSP_PRAM1 is not mapped on the Flash addressing range"]
    DSP_PRAM1_OVERLAY_DISABLE = 0,
    #[doc = "1: DSP_PRAM1 is also mapped on the Flash addressing range"]
    DSP_PRAM1_OVERLAY_ENABLE = 1,
}
impl From<DSP_PRAM1_OVERLAY_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_PRAM1_OVERLAY_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_PRAM1_OVERLAY_CFG`"]
pub type DSP_PRAM1_OVERLAY_CFG_R = crate::R<bool, DSP_PRAM1_OVERLAY_CFG_A>;
impl DSP_PRAM1_OVERLAY_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_PRAM1_OVERLAY_CFG_A {
        match self.bits {
            false => DSP_PRAM1_OVERLAY_CFG_A::DSP_PRAM1_OVERLAY_DISABLE,
            true => DSP_PRAM1_OVERLAY_CFG_A::DSP_PRAM1_OVERLAY_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM1_OVERLAY_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram1_overlay_disable(&self) -> bool {
        *self == DSP_PRAM1_OVERLAY_CFG_A::DSP_PRAM1_OVERLAY_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM1_OVERLAY_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram1_overlay_enable(&self) -> bool {
        *self == DSP_PRAM1_OVERLAY_CFG_A::DSP_PRAM1_OVERLAY_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_PRAM1_OVERLAY_CFG`"]
pub struct DSP_PRAM1_OVERLAY_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM1_OVERLAY_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM1_OVERLAY_CFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP_PRAM1 is not mapped on the Flash addressing range"]
    #[inline(always)]
    pub fn dsp_pram1_overlay_disable(self) -> &'a mut W {
        self.variant(DSP_PRAM1_OVERLAY_CFG_A::DSP_PRAM1_OVERLAY_DISABLE)
    }
    #[doc = "DSP_PRAM1 is also mapped on the Flash addressing range"]
    #[inline(always)]
    pub fn dsp_pram1_overlay_enable(self) -> &'a mut W {
        self.variant(DSP_PRAM1_OVERLAY_CFG_A::DSP_PRAM1_OVERLAY_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "DSP_PRAM2 Flash overlay configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_PRAM2_OVERLAY_CFG_A {
    #[doc = "0: DSP_PRAM2 is not mapped on the Flash addressing range"]
    DSP_PRAM2_OVERLAY_DISABLE = 0,
    #[doc = "1: DSP_PRAM2 is also mapped on the Flash addressing range"]
    DSP_PRAM2_OVERLAY_ENABLE = 1,
}
impl From<DSP_PRAM2_OVERLAY_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_PRAM2_OVERLAY_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_PRAM2_OVERLAY_CFG`"]
pub type DSP_PRAM2_OVERLAY_CFG_R = crate::R<bool, DSP_PRAM2_OVERLAY_CFG_A>;
impl DSP_PRAM2_OVERLAY_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_PRAM2_OVERLAY_CFG_A {
        match self.bits {
            false => DSP_PRAM2_OVERLAY_CFG_A::DSP_PRAM2_OVERLAY_DISABLE,
            true => DSP_PRAM2_OVERLAY_CFG_A::DSP_PRAM2_OVERLAY_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM2_OVERLAY_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram2_overlay_disable(&self) -> bool {
        *self == DSP_PRAM2_OVERLAY_CFG_A::DSP_PRAM2_OVERLAY_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM2_OVERLAY_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram2_overlay_enable(&self) -> bool {
        *self == DSP_PRAM2_OVERLAY_CFG_A::DSP_PRAM2_OVERLAY_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_PRAM2_OVERLAY_CFG`"]
pub struct DSP_PRAM2_OVERLAY_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM2_OVERLAY_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM2_OVERLAY_CFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP_PRAM2 is not mapped on the Flash addressing range"]
    #[inline(always)]
    pub fn dsp_pram2_overlay_disable(self) -> &'a mut W {
        self.variant(DSP_PRAM2_OVERLAY_CFG_A::DSP_PRAM2_OVERLAY_DISABLE)
    }
    #[doc = "DSP_PRAM2 is also mapped on the Flash addressing range"]
    #[inline(always)]
    pub fn dsp_pram2_overlay_enable(self) -> &'a mut W {
        self.variant(DSP_PRAM2_OVERLAY_CFG_A::DSP_PRAM2_OVERLAY_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "DSP_PRAM3 Flash overlay configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_PRAM3_OVERLAY_CFG_A {
    #[doc = "0: DSP_PRAM3 is not mapped on the Flash addressing range"]
    DSP_PRAM3_OVERLAY_DISABLE = 0,
    #[doc = "1: DSP_PRAM3 is also mapped on the Flash addressing range"]
    DSP_PRAM3_OVERLAY_ENABLE = 1,
}
impl From<DSP_PRAM3_OVERLAY_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_PRAM3_OVERLAY_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_PRAM3_OVERLAY_CFG`"]
pub type DSP_PRAM3_OVERLAY_CFG_R = crate::R<bool, DSP_PRAM3_OVERLAY_CFG_A>;
impl DSP_PRAM3_OVERLAY_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_PRAM3_OVERLAY_CFG_A {
        match self.bits {
            false => DSP_PRAM3_OVERLAY_CFG_A::DSP_PRAM3_OVERLAY_DISABLE,
            true => DSP_PRAM3_OVERLAY_CFG_A::DSP_PRAM3_OVERLAY_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM3_OVERLAY_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram3_overlay_disable(&self) -> bool {
        *self == DSP_PRAM3_OVERLAY_CFG_A::DSP_PRAM3_OVERLAY_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM3_OVERLAY_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram3_overlay_enable(&self) -> bool {
        *self == DSP_PRAM3_OVERLAY_CFG_A::DSP_PRAM3_OVERLAY_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_PRAM3_OVERLAY_CFG`"]
pub struct DSP_PRAM3_OVERLAY_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM3_OVERLAY_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM3_OVERLAY_CFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP_PRAM3 is not mapped on the Flash addressing range"]
    #[inline(always)]
    pub fn dsp_pram3_overlay_disable(self) -> &'a mut W {
        self.variant(DSP_PRAM3_OVERLAY_CFG_A::DSP_PRAM3_OVERLAY_DISABLE)
    }
    #[doc = "DSP_PRAM3 is also mapped on the Flash addressing range"]
    #[inline(always)]
    pub fn dsp_pram3_overlay_enable(self) -> &'a mut W {
        self.variant(DSP_PRAM3_OVERLAY_CFG_A::DSP_PRAM3_OVERLAY_ENABLE)
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
#[doc = "PRAM3 Flash overlay configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRAM3_OVERLAY_CFG_A {
    #[doc = "0: PRAM3 is not mapped on the Flash addressing range"]
    PRAM3_OVERLAY_DISABLE = 0,
    #[doc = "1: PRAM3 is also mapped on the Flash addressing range"]
    PRAM3_OVERLAY_ENABLE = 1,
}
impl From<PRAM3_OVERLAY_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: PRAM3_OVERLAY_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRAM3_OVERLAY_CFG`"]
pub type PRAM3_OVERLAY_CFG_R = crate::R<bool, PRAM3_OVERLAY_CFG_A>;
impl PRAM3_OVERLAY_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRAM3_OVERLAY_CFG_A {
        match self.bits {
            false => PRAM3_OVERLAY_CFG_A::PRAM3_OVERLAY_DISABLE,
            true => PRAM3_OVERLAY_CFG_A::PRAM3_OVERLAY_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PRAM3_OVERLAY_DISABLE`"]
    #[inline(always)]
    pub fn is_pram3_overlay_disable(&self) -> bool {
        *self == PRAM3_OVERLAY_CFG_A::PRAM3_OVERLAY_DISABLE
    }
    #[doc = "Checks if the value of the field is `PRAM3_OVERLAY_ENABLE`"]
    #[inline(always)]
    pub fn is_pram3_overlay_enable(&self) -> bool {
        *self == PRAM3_OVERLAY_CFG_A::PRAM3_OVERLAY_ENABLE
    }
}
#[doc = "Write proxy for field `PRAM3_OVERLAY_CFG`"]
pub struct PRAM3_OVERLAY_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRAM3_OVERLAY_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRAM3_OVERLAY_CFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PRAM3 is not mapped on the Flash addressing range"]
    #[inline(always)]
    pub fn pram3_overlay_disable(self) -> &'a mut W {
        self.variant(PRAM3_OVERLAY_CFG_A::PRAM3_OVERLAY_DISABLE)
    }
    #[doc = "PRAM3 is also mapped on the Flash addressing range"]
    #[inline(always)]
    pub fn pram3_overlay_enable(self) -> &'a mut W {
        self.variant(PRAM3_OVERLAY_CFG_A::PRAM3_OVERLAY_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "PRAM2 Flash overlay configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRAM2_OVERLAY_CFG_A {
    #[doc = "0: PRAM2 is not mapped on the Flash addressing range"]
    PRAM2_OVERLAY_DISABLE = 0,
    #[doc = "1: PRAM2 is also mapped on the Flash addressing range"]
    PRAM2_OVERLAY_ENABLE = 1,
}
impl From<PRAM2_OVERLAY_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: PRAM2_OVERLAY_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRAM2_OVERLAY_CFG`"]
pub type PRAM2_OVERLAY_CFG_R = crate::R<bool, PRAM2_OVERLAY_CFG_A>;
impl PRAM2_OVERLAY_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRAM2_OVERLAY_CFG_A {
        match self.bits {
            false => PRAM2_OVERLAY_CFG_A::PRAM2_OVERLAY_DISABLE,
            true => PRAM2_OVERLAY_CFG_A::PRAM2_OVERLAY_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PRAM2_OVERLAY_DISABLE`"]
    #[inline(always)]
    pub fn is_pram2_overlay_disable(&self) -> bool {
        *self == PRAM2_OVERLAY_CFG_A::PRAM2_OVERLAY_DISABLE
    }
    #[doc = "Checks if the value of the field is `PRAM2_OVERLAY_ENABLE`"]
    #[inline(always)]
    pub fn is_pram2_overlay_enable(&self) -> bool {
        *self == PRAM2_OVERLAY_CFG_A::PRAM2_OVERLAY_ENABLE
    }
}
#[doc = "Write proxy for field `PRAM2_OVERLAY_CFG`"]
pub struct PRAM2_OVERLAY_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRAM2_OVERLAY_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRAM2_OVERLAY_CFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PRAM2 is not mapped on the Flash addressing range"]
    #[inline(always)]
    pub fn pram2_overlay_disable(self) -> &'a mut W {
        self.variant(PRAM2_OVERLAY_CFG_A::PRAM2_OVERLAY_DISABLE)
    }
    #[doc = "PRAM2 is also mapped on the Flash addressing range"]
    #[inline(always)]
    pub fn pram2_overlay_enable(self) -> &'a mut W {
        self.variant(PRAM2_OVERLAY_CFG_A::PRAM2_OVERLAY_ENABLE)
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
#[doc = "PRAM1 Flash overlay configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRAM1_OVERLAY_CFG_A {
    #[doc = "0: PRAM1 is not mapped on the Flash addressing range"]
    PRAM1_OVERLAY_DISABLE = 0,
    #[doc = "1: PRAM1 is also mapped on the Flash addressing range"]
    PRAM1_OVERLAY_ENABLE = 1,
}
impl From<PRAM1_OVERLAY_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: PRAM1_OVERLAY_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRAM1_OVERLAY_CFG`"]
pub type PRAM1_OVERLAY_CFG_R = crate::R<bool, PRAM1_OVERLAY_CFG_A>;
impl PRAM1_OVERLAY_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRAM1_OVERLAY_CFG_A {
        match self.bits {
            false => PRAM1_OVERLAY_CFG_A::PRAM1_OVERLAY_DISABLE,
            true => PRAM1_OVERLAY_CFG_A::PRAM1_OVERLAY_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PRAM1_OVERLAY_DISABLE`"]
    #[inline(always)]
    pub fn is_pram1_overlay_disable(&self) -> bool {
        *self == PRAM1_OVERLAY_CFG_A::PRAM1_OVERLAY_DISABLE
    }
    #[doc = "Checks if the value of the field is `PRAM1_OVERLAY_ENABLE`"]
    #[inline(always)]
    pub fn is_pram1_overlay_enable(&self) -> bool {
        *self == PRAM1_OVERLAY_CFG_A::PRAM1_OVERLAY_ENABLE
    }
}
#[doc = "Write proxy for field `PRAM1_OVERLAY_CFG`"]
pub struct PRAM1_OVERLAY_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRAM1_OVERLAY_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRAM1_OVERLAY_CFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PRAM1 is not mapped on the Flash addressing range"]
    #[inline(always)]
    pub fn pram1_overlay_disable(self) -> &'a mut W {
        self.variant(PRAM1_OVERLAY_CFG_A::PRAM1_OVERLAY_DISABLE)
    }
    #[doc = "PRAM1 is also mapped on the Flash addressing range"]
    #[inline(always)]
    pub fn pram1_overlay_enable(self) -> &'a mut W {
        self.variant(PRAM1_OVERLAY_CFG_A::PRAM1_OVERLAY_ENABLE)
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
#[doc = "PRAM0 Flash overlay configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRAM0_OVERLAY_CFG_A {
    #[doc = "0: PRAM0 is not mapped on the Flash addressing range"]
    PRAM0_OVERLAY_DISABLE = 0,
    #[doc = "1: PRAM0 is also mapped on the Flash addressing range"]
    PRAM0_OVERLAY_ENABLE = 1,
}
impl From<PRAM0_OVERLAY_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: PRAM0_OVERLAY_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRAM0_OVERLAY_CFG`"]
pub type PRAM0_OVERLAY_CFG_R = crate::R<bool, PRAM0_OVERLAY_CFG_A>;
impl PRAM0_OVERLAY_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRAM0_OVERLAY_CFG_A {
        match self.bits {
            false => PRAM0_OVERLAY_CFG_A::PRAM0_OVERLAY_DISABLE,
            true => PRAM0_OVERLAY_CFG_A::PRAM0_OVERLAY_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PRAM0_OVERLAY_DISABLE`"]
    #[inline(always)]
    pub fn is_pram0_overlay_disable(&self) -> bool {
        *self == PRAM0_OVERLAY_CFG_A::PRAM0_OVERLAY_DISABLE
    }
    #[doc = "Checks if the value of the field is `PRAM0_OVERLAY_ENABLE`"]
    #[inline(always)]
    pub fn is_pram0_overlay_enable(&self) -> bool {
        *self == PRAM0_OVERLAY_CFG_A::PRAM0_OVERLAY_ENABLE
    }
}
#[doc = "Write proxy for field `PRAM0_OVERLAY_CFG`"]
pub struct PRAM0_OVERLAY_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRAM0_OVERLAY_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRAM0_OVERLAY_CFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PRAM0 is not mapped on the Flash addressing range"]
    #[inline(always)]
    pub fn pram0_overlay_disable(self) -> &'a mut W {
        self.variant(PRAM0_OVERLAY_CFG_A::PRAM0_OVERLAY_DISABLE)
    }
    #[doc = "PRAM0 is also mapped on the Flash addressing range"]
    #[inline(always)]
    pub fn pram0_overlay_enable(self) -> &'a mut W {
        self.variant(PRAM0_OVERLAY_CFG_A::PRAM0_OVERLAY_ENABLE)
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
    #[doc = "Bit 7 - DSP_PRAM0 Flash overlay configuration"]
    #[inline(always)]
    pub fn dsp_pram0_overlay_cfg(&self) -> DSP_PRAM0_OVERLAY_CFG_R {
        DSP_PRAM0_OVERLAY_CFG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DSP_PRAM1 Flash overlay configuration"]
    #[inline(always)]
    pub fn dsp_pram1_overlay_cfg(&self) -> DSP_PRAM1_OVERLAY_CFG_R {
        DSP_PRAM1_OVERLAY_CFG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DSP_PRAM2 Flash overlay configuration"]
    #[inline(always)]
    pub fn dsp_pram2_overlay_cfg(&self) -> DSP_PRAM2_OVERLAY_CFG_R {
        DSP_PRAM2_OVERLAY_CFG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DSP_PRAM3 Flash overlay configuration"]
    #[inline(always)]
    pub fn dsp_pram3_overlay_cfg(&self) -> DSP_PRAM3_OVERLAY_CFG_R {
        DSP_PRAM3_OVERLAY_CFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PRAM3 Flash overlay configuration"]
    #[inline(always)]
    pub fn pram3_overlay_cfg(&self) -> PRAM3_OVERLAY_CFG_R {
        PRAM3_OVERLAY_CFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PRAM2 Flash overlay configuration"]
    #[inline(always)]
    pub fn pram2_overlay_cfg(&self) -> PRAM2_OVERLAY_CFG_R {
        PRAM2_OVERLAY_CFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - PRAM1 Flash overlay configuration"]
    #[inline(always)]
    pub fn pram1_overlay_cfg(&self) -> PRAM1_OVERLAY_CFG_R {
        PRAM1_OVERLAY_CFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PRAM0 Flash overlay configuration"]
    #[inline(always)]
    pub fn pram0_overlay_cfg(&self) -> PRAM0_OVERLAY_CFG_R {
        PRAM0_OVERLAY_CFG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - DSP_PRAM0 Flash overlay configuration"]
    #[inline(always)]
    pub fn dsp_pram0_overlay_cfg(&mut self) -> DSP_PRAM0_OVERLAY_CFG_W {
        DSP_PRAM0_OVERLAY_CFG_W { w: self }
    }
    #[doc = "Bit 6 - DSP_PRAM1 Flash overlay configuration"]
    #[inline(always)]
    pub fn dsp_pram1_overlay_cfg(&mut self) -> DSP_PRAM1_OVERLAY_CFG_W {
        DSP_PRAM1_OVERLAY_CFG_W { w: self }
    }
    #[doc = "Bit 5 - DSP_PRAM2 Flash overlay configuration"]
    #[inline(always)]
    pub fn dsp_pram2_overlay_cfg(&mut self) -> DSP_PRAM2_OVERLAY_CFG_W {
        DSP_PRAM2_OVERLAY_CFG_W { w: self }
    }
    #[doc = "Bit 4 - DSP_PRAM3 Flash overlay configuration"]
    #[inline(always)]
    pub fn dsp_pram3_overlay_cfg(&mut self) -> DSP_PRAM3_OVERLAY_CFG_W {
        DSP_PRAM3_OVERLAY_CFG_W { w: self }
    }
    #[doc = "Bit 3 - PRAM3 Flash overlay configuration"]
    #[inline(always)]
    pub fn pram3_overlay_cfg(&mut self) -> PRAM3_OVERLAY_CFG_W {
        PRAM3_OVERLAY_CFG_W { w: self }
    }
    #[doc = "Bit 2 - PRAM2 Flash overlay configuration"]
    #[inline(always)]
    pub fn pram2_overlay_cfg(&mut self) -> PRAM2_OVERLAY_CFG_W {
        PRAM2_OVERLAY_CFG_W { w: self }
    }
    #[doc = "Bit 1 - PRAM1 Flash overlay configuration"]
    #[inline(always)]
    pub fn pram1_overlay_cfg(&mut self) -> PRAM1_OVERLAY_CFG_W {
        PRAM1_OVERLAY_CFG_W { w: self }
    }
    #[doc = "Bit 0 - PRAM0 Flash overlay configuration"]
    #[inline(always)]
    pub fn pram0_overlay_cfg(&mut self) -> PRAM0_OVERLAY_CFG_W {
        PRAM0_OVERLAY_CFG_W { w: self }
    }
}
