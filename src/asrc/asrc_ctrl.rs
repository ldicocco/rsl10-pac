#[doc = "Reader of register ASRC_CTRL"]
pub type R = crate::R<u32, super::ASRC_CTRL>;
#[doc = "Writer for register ASRC_CTRL"]
pub type W = crate::W<u32, super::ASRC_CTRL>;
#[doc = "Register ASRC_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ASRC_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "The ASRC processing state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASRC_PROC_STATUS_A {
    #[doc = "0: The ASRC is idle"]
    ASRC_IDLE = 0,
    #[doc = "1: The ASRC is busy processing a sample"]
    ASRC_BUSY = 1,
}
impl From<ASRC_PROC_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: ASRC_PROC_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASRC_PROC_STATUS`"]
pub type ASRC_PROC_STATUS_R = crate::R<bool, ASRC_PROC_STATUS_A>;
impl ASRC_PROC_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASRC_PROC_STATUS_A {
        match self.bits {
            false => ASRC_PROC_STATUS_A::ASRC_IDLE,
            true => ASRC_PROC_STATUS_A::ASRC_BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `ASRC_IDLE`"]
    #[inline(always)]
    pub fn is_asrc_idle(&self) -> bool {
        *self == ASRC_PROC_STATUS_A::ASRC_IDLE
    }
    #[doc = "Checks if the value of the field is `ASRC_BUSY`"]
    #[inline(always)]
    pub fn is_asrc_busy(&self) -> bool {
        *self == ASRC_PROC_STATUS_A::ASRC_BUSY
    }
}
#[doc = "The ASRC_OUT register status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASRC_OUT_REQ_A {
    #[doc = "0: The ASRC_OUT register is idle"]
    INT_IDLE_ASRC_OUT = 0,
    #[doc = "1: The ASRC_OUT register is ready to be read"]
    INT_RDY_ASRC_OUT = 1,
}
impl From<ASRC_OUT_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: ASRC_OUT_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASRC_OUT_REQ`"]
pub type ASRC_OUT_REQ_R = crate::R<bool, ASRC_OUT_REQ_A>;
impl ASRC_OUT_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASRC_OUT_REQ_A {
        match self.bits {
            false => ASRC_OUT_REQ_A::INT_IDLE_ASRC_OUT,
            true => ASRC_OUT_REQ_A::INT_RDY_ASRC_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `INT_IDLE_ASRC_OUT`"]
    #[inline(always)]
    pub fn is_int_idle_asrc_out(&self) -> bool {
        *self == ASRC_OUT_REQ_A::INT_IDLE_ASRC_OUT
    }
    #[doc = "Checks if the value of the field is `INT_RDY_ASRC_OUT`"]
    #[inline(always)]
    pub fn is_int_rdy_asrc_out(&self) -> bool {
        *self == ASRC_OUT_REQ_A::INT_RDY_ASRC_OUT
    }
}
#[doc = "The ASRC_IN register status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASRC_IN_REQ_A {
    #[doc = "0: The ASRC_IN register is idle"]
    INT_IDLE_ASRC_IN = 0,
    #[doc = "1: The ASRC_IN register is requesting an input"]
    INT_RDY_ASRC_IN = 1,
}
impl From<ASRC_IN_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: ASRC_IN_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASRC_IN_REQ`"]
pub type ASRC_IN_REQ_R = crate::R<bool, ASRC_IN_REQ_A>;
impl ASRC_IN_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASRC_IN_REQ_A {
        match self.bits {
            false => ASRC_IN_REQ_A::INT_IDLE_ASRC_IN,
            true => ASRC_IN_REQ_A::INT_RDY_ASRC_IN,
        }
    }
    #[doc = "Checks if the value of the field is `INT_IDLE_ASRC_IN`"]
    #[inline(always)]
    pub fn is_int_idle_asrc_in(&self) -> bool {
        *self == ASRC_IN_REQ_A::INT_IDLE_ASRC_IN
    }
    #[doc = "Checks if the value of the field is `INT_RDY_ASRC_IN`"]
    #[inline(always)]
    pub fn is_int_rdy_asrc_in(&self) -> bool {
        *self == ASRC_IN_REQ_A::INT_RDY_ASRC_IN
    }
}
#[doc = "The ASRC state/configuration update error interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASRC_UPDATE_ERR_A {
    #[doc = "0: This source has not set an interrupt"]
    INT_IDLE_ASRC_PH_CNT_ERR = 0,
    #[doc = "1: The internal states or configuration updated while the accelerator is busy"]
    INT_PENDING_ASRC_PH_CNT_ERR = 1,
}
impl From<ASRC_UPDATE_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ASRC_UPDATE_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASRC_UPDATE_ERR`"]
pub type ASRC_UPDATE_ERR_R = crate::R<bool, ASRC_UPDATE_ERR_A>;
impl ASRC_UPDATE_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASRC_UPDATE_ERR_A {
        match self.bits {
            false => ASRC_UPDATE_ERR_A::INT_IDLE_ASRC_PH_CNT_ERR,
            true => ASRC_UPDATE_ERR_A::INT_PENDING_ASRC_PH_CNT_ERR,
        }
    }
    #[doc = "Checks if the value of the field is `INT_IDLE_ASRC_PH_CNT_ERR`"]
    #[inline(always)]
    pub fn is_int_idle_asrc_ph_cnt_err(&self) -> bool {
        *self == ASRC_UPDATE_ERR_A::INT_IDLE_ASRC_PH_CNT_ERR
    }
    #[doc = "Checks if the value of the field is `INT_PENDING_ASRC_PH_CNT_ERR`"]
    #[inline(always)]
    pub fn is_int_pending_asrc_ph_cnt_err(&self) -> bool {
        *self == ASRC_UPDATE_ERR_A::INT_PENDING_ASRC_PH_CNT_ERR
    }
}
#[doc = "The ASRC input interface error interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASRC_IN_ERR_A {
    #[doc = "0: This source has not set an interrupt"]
    INT_IDLE_ASRC_IN_ERR = 0,
    #[doc = "1: The ASRC input interface register overwritten before the accelerator finished its processing"]
    INT_PENDING_ASRC_IN_ERR = 1,
}
impl From<ASRC_IN_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ASRC_IN_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASRC_IN_ERR`"]
pub type ASRC_IN_ERR_R = crate::R<bool, ASRC_IN_ERR_A>;
impl ASRC_IN_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASRC_IN_ERR_A {
        match self.bits {
            false => ASRC_IN_ERR_A::INT_IDLE_ASRC_IN_ERR,
            true => ASRC_IN_ERR_A::INT_PENDING_ASRC_IN_ERR,
        }
    }
    #[doc = "Checks if the value of the field is `INT_IDLE_ASRC_IN_ERR`"]
    #[inline(always)]
    pub fn is_int_idle_asrc_in_err(&self) -> bool {
        *self == ASRC_IN_ERR_A::INT_IDLE_ASRC_IN_ERR
    }
    #[doc = "Checks if the value of the field is `INT_PENDING_ASRC_IN_ERR`"]
    #[inline(always)]
    pub fn is_int_pending_asrc_in_err(&self) -> bool {
        *self == ASRC_IN_ERR_A::INT_PENDING_ASRC_IN_ERR
    }
}
#[doc = "Clear the ASRC update/configuration error interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASRC_UPDATE_ERR_CLR_AW {
    #[doc = "1: Clear the ASRC update error interrupt"]
    CLR_ASRC_UPDATE_ERR = 1,
}
impl From<ASRC_UPDATE_ERR_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: ASRC_UPDATE_ERR_CLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ASRC_UPDATE_ERR_CLR`"]
pub struct ASRC_UPDATE_ERR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ASRC_UPDATE_ERR_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASRC_UPDATE_ERR_CLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the ASRC update error interrupt"]
    #[inline(always)]
    pub fn clr_asrc_update_err(self) -> &'a mut W {
        self.variant(ASRC_UPDATE_ERR_CLR_AW::CLR_ASRC_UPDATE_ERR)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Clear the ASRC input interface error interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASRC_IN_ERR_CLR_AW {
    #[doc = "1: Clear the ASRC IN error interrupt"]
    CLR_ASRC_IN_ERR = 1,
}
impl From<ASRC_IN_ERR_CLR_AW> for bool {
    #[inline(always)]
    fn from(variant: ASRC_IN_ERR_CLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ASRC_IN_ERR_CLR`"]
pub struct ASRC_IN_ERR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ASRC_IN_ERR_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASRC_IN_ERR_CLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the ASRC IN error interrupt"]
    #[inline(always)]
    pub fn clr_asrc_in_err(self) -> &'a mut W {
        self.variant(ASRC_IN_ERR_CLR_AW::CLR_ASRC_IN_ERR)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write a 1 to reset ASRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASRC_RESET_AW {
    #[doc = "1: Reset ASRC"]
    ASRC_RESET = 1,
}
impl From<ASRC_RESET_AW> for bool {
    #[inline(always)]
    fn from(variant: ASRC_RESET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ASRC_RESET`"]
pub struct ASRC_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> ASRC_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASRC_RESET_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset ASRC"]
    #[inline(always)]
    pub fn asrc_reset(self) -> &'a mut W {
        self.variant(ASRC_RESET_AW::ASRC_RESET)
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
#[doc = "Enable status of the re-sampler block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASRC_EN_STATUS_A {
    #[doc = "0: The re-sampler is disabled"]
    ASRC_DISABLED = 0,
    #[doc = "1: The re-sampler is enabled"]
    ASRC_ENABLED = 1,
}
impl From<ASRC_EN_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: ASRC_EN_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASRC_EN_STATUS`"]
pub type ASRC_EN_STATUS_R = crate::R<bool, ASRC_EN_STATUS_A>;
impl ASRC_EN_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASRC_EN_STATUS_A {
        match self.bits {
            false => ASRC_EN_STATUS_A::ASRC_DISABLED,
            true => ASRC_EN_STATUS_A::ASRC_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ASRC_DISABLED`"]
    #[inline(always)]
    pub fn is_asrc_disabled(&self) -> bool {
        *self == ASRC_EN_STATUS_A::ASRC_DISABLED
    }
    #[doc = "Checks if the value of the field is `ASRC_ENABLED`"]
    #[inline(always)]
    pub fn is_asrc_enabled(&self) -> bool {
        *self == ASRC_EN_STATUS_A::ASRC_ENABLED
    }
}
#[doc = "Disable the re-sampler block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASRC_DISABLE_AW {
    #[doc = "1: Disable the re-sampler"]
    ASRC_DISABLE = 1,
}
impl From<ASRC_DISABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: ASRC_DISABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ASRC_DISABLE`"]
pub struct ASRC_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASRC_DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASRC_DISABLE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the re-sampler"]
    #[inline(always)]
    pub fn asrc_disable(self) -> &'a mut W {
        self.variant(ASRC_DISABLE_AW::ASRC_DISABLE)
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
#[doc = "Enable the re-sampler block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASRC_ENABLE_AW {
    #[doc = "1: Enable the re-sampler block"]
    ASRC_ENABLE = 1,
}
impl From<ASRC_ENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: ASRC_ENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ASRC_ENABLE`"]
pub struct ASRC_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASRC_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASRC_ENABLE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the re-sampler block"]
    #[inline(always)]
    pub fn asrc_enable(self) -> &'a mut W {
        self.variant(ASRC_ENABLE_AW::ASRC_ENABLE)
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
    #[doc = "Bit 14 - The ASRC processing state"]
    #[inline(always)]
    pub fn asrc_proc_status(&self) -> ASRC_PROC_STATUS_R {
        ASRC_PROC_STATUS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The ASRC_OUT register status"]
    #[inline(always)]
    pub fn asrc_out_req(&self) -> ASRC_OUT_REQ_R {
        ASRC_OUT_REQ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The ASRC_IN register status"]
    #[inline(always)]
    pub fn asrc_in_req(&self) -> ASRC_IN_REQ_R {
        ASRC_IN_REQ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The ASRC state/configuration update error interrupt status"]
    #[inline(always)]
    pub fn asrc_update_err(&self) -> ASRC_UPDATE_ERR_R {
        ASRC_UPDATE_ERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The ASRC input interface error interrupt status"]
    #[inline(always)]
    pub fn asrc_in_err(&self) -> ASRC_IN_ERR_R {
        ASRC_IN_ERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable status of the re-sampler block"]
    #[inline(always)]
    pub fn asrc_en_status(&self) -> ASRC_EN_STATUS_R {
        ASRC_EN_STATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Clear the ASRC update/configuration error interrupt status"]
    #[inline(always)]
    pub fn asrc_update_err_clr(&mut self) -> ASRC_UPDATE_ERR_CLR_W {
        ASRC_UPDATE_ERR_CLR_W { w: self }
    }
    #[doc = "Bit 8 - Clear the ASRC input interface error interrupt"]
    #[inline(always)]
    pub fn asrc_in_err_clr(&mut self) -> ASRC_IN_ERR_CLR_W {
        ASRC_IN_ERR_CLR_W { w: self }
    }
    #[doc = "Bit 3 - Write a 1 to reset ASRC"]
    #[inline(always)]
    pub fn asrc_reset(&mut self) -> ASRC_RESET_W {
        ASRC_RESET_W { w: self }
    }
    #[doc = "Bit 1 - Disable the re-sampler block"]
    #[inline(always)]
    pub fn asrc_disable(&mut self) -> ASRC_DISABLE_W {
        ASRC_DISABLE_W { w: self }
    }
    #[doc = "Bit 0 - Enable the re-sampler block"]
    #[inline(always)]
    pub fn asrc_enable(&mut self) -> ASRC_ENABLE_W {
        ASRC_ENABLE_W { w: self }
    }
}
