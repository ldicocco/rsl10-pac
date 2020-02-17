#[doc = "Reader of register SYSCTRL_DSS_CTRL"]
pub type R = crate::R<u32, super::SYSCTRL_DSS_CTRL>;
#[doc = "Writer for register SYSCTRL_DSS_CTRL"]
pub type W = crate::W<u32, super::SYSCTRL_DSS_CTRL>;
#[doc = "Register SYSCTRL_DSS_CTRL `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::SYSCTRL_DSS_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "LPDSP32 feature status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDSP32_STATUS_A {
    #[doc = "0: Device does not have the LPDSP32 feature"]
    LPDSP32_DISABLED_DEVICE = 0,
    #[doc = "1: Device has the LPDSP32 feature"]
    LPDSP32_ENABLED_DEVICE = 1,
}
impl From<LPDSP32_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: LPDSP32_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPDSP32_STATUS`"]
pub type LPDSP32_STATUS_R = crate::R<bool, LPDSP32_STATUS_A>;
impl LPDSP32_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDSP32_STATUS_A {
        match self.bits {
            false => LPDSP32_STATUS_A::LPDSP32_DISABLED_DEVICE,
            true => LPDSP32_STATUS_A::LPDSP32_ENABLED_DEVICE,
        }
    }
    #[doc = "Checks if the value of the field is `LPDSP32_DISABLED_DEVICE`"]
    #[inline(always)]
    pub fn is_lpdsp32_disabled_device(&self) -> bool {
        *self == LPDSP32_STATUS_A::LPDSP32_DISABLED_DEVICE
    }
    #[doc = "Checks if the value of the field is `LPDSP32_ENABLED_DEVICE`"]
    #[inline(always)]
    pub fn is_lpdsp32_enabled_device(&self) -> bool {
        *self == LPDSP32_STATUS_A::LPDSP32_ENABLED_DEVICE
    }
}
#[doc = "Write a 1 to reset pending CSS interrupts in the DSS interrupt controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSS_CSS_INT_RESET_AW {
    #[doc = "1: Reset CSS interrupts in the DSS interrupt controller"]
    DSS_CSS_INT_RESET = 1,
}
impl From<DSS_CSS_INT_RESET_AW> for bool {
    #[inline(always)]
    fn from(variant: DSS_CSS_INT_RESET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DSS_CSS_INT_RESET`"]
pub struct DSS_CSS_INT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> DSS_CSS_INT_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSS_CSS_INT_RESET_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset CSS interrupts in the DSS interrupt controller"]
    #[inline(always)]
    pub fn dss_css_int_reset(self) -> &'a mut W {
        self.variant(DSS_CSS_INT_RESET_AW::DSS_CSS_INT_RESET)
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
#[doc = "Write a 1 to reset pending DMA interrupts in the DSS interrupt controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSS_DMA_INT_RESET_AW {
    #[doc = "1: Reset DMA interrupts in the DSS interrupt controller"]
    DSS_DMA_INT_RESET = 1,
}
impl From<DSS_DMA_INT_RESET_AW> for bool {
    #[inline(always)]
    fn from(variant: DSS_DMA_INT_RESET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DSS_DMA_INT_RESET`"]
pub struct DSS_DMA_INT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> DSS_DMA_INT_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSS_DMA_INT_RESET_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset DMA interrupts in the DSS interrupt controller"]
    #[inline(always)]
    pub fn dss_dma_int_reset(self) -> &'a mut W {
        self.variant(DSS_DMA_INT_RESET_AW::DSS_DMA_INT_RESET)
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
#[doc = "Write a 1 to reset DSS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSS_RESET_AW {
    #[doc = "1: Reset DSS"]
    DSS_RESET = 1,
}
impl From<DSS_RESET_AW> for bool {
    #[inline(always)]
    fn from(variant: DSS_RESET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DSS_RESET`"]
pub struct DSS_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> DSS_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSS_RESET_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset DSS"]
    #[inline(always)]
    pub fn dss_reset(self) -> &'a mut W {
        self.variant(DSS_RESET_AW::DSS_RESET)
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
#[doc = "Write a 1 to pause LPDSP32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDSP32_PAUSE_AW {
    #[doc = "1: Pause LPDSP32"]
    DSS_LPDSP32_PAUSE = 1,
}
impl From<LPDSP32_PAUSE_AW> for bool {
    #[inline(always)]
    fn from(variant: LPDSP32_PAUSE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `LPDSP32_PAUSE`"]
pub struct LPDSP32_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDSP32_PAUSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPDSP32_PAUSE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pause LPDSP32"]
    #[inline(always)]
    pub fn dss_lpdsp32_pause(self) -> &'a mut W {
        self.variant(LPDSP32_PAUSE_AW::DSS_LPDSP32_PAUSE)
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
#[doc = "Write a 1 to run LPDSP32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDSP32_RESUME_AW {
    #[doc = "1: Resume LPDSP32"]
    DSS_LPDSP32_RESUME = 1,
}
impl From<LPDSP32_RESUME_AW> for bool {
    #[inline(always)]
    fn from(variant: LPDSP32_RESUME_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `LPDSP32_RESUME`"]
pub struct LPDSP32_RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDSP32_RESUME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPDSP32_RESUME_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Resume LPDSP32"]
    #[inline(always)]
    pub fn dss_lpdsp32_resume(self) -> &'a mut W {
        self.variant(LPDSP32_RESUME_AW::DSS_LPDSP32_RESUME)
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
#[doc = "LPDSP32 running status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDSP32_RUNNING_A {
    #[doc = "0: LPDSP32 paused"]
    DSS_LPDSP32_STATE_PAUSE = 0,
    #[doc = "1: LPDSP32 running"]
    DSS_LPDSP32_STATE_RUN = 1,
}
impl From<LPDSP32_RUNNING_A> for bool {
    #[inline(always)]
    fn from(variant: LPDSP32_RUNNING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPDSP32_RUNNING`"]
pub type LPDSP32_RUNNING_R = crate::R<bool, LPDSP32_RUNNING_A>;
impl LPDSP32_RUNNING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDSP32_RUNNING_A {
        match self.bits {
            false => LPDSP32_RUNNING_A::DSS_LPDSP32_STATE_PAUSE,
            true => LPDSP32_RUNNING_A::DSS_LPDSP32_STATE_RUN,
        }
    }
    #[doc = "Checks if the value of the field is `DSS_LPDSP32_STATE_PAUSE`"]
    #[inline(always)]
    pub fn is_dss_lpdsp32_state_pause(&self) -> bool {
        *self == LPDSP32_RUNNING_A::DSS_LPDSP32_STATE_PAUSE
    }
    #[doc = "Checks if the value of the field is `DSS_LPDSP32_STATE_RUN`"]
    #[inline(always)]
    pub fn is_dss_lpdsp32_state_run(&self) -> bool {
        *self == LPDSP32_RUNNING_A::DSS_LPDSP32_STATE_RUN
    }
}
impl R {
    #[doc = "Bit 8 - LPDSP32 feature status"]
    #[inline(always)]
    pub fn lpdsp32_status(&self) -> LPDSP32_STATUS_R {
        LPDSP32_STATUS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LPDSP32 running status"]
    #[inline(always)]
    pub fn lpdsp32_running(&self) -> LPDSP32_RUNNING_R {
        LPDSP32_RUNNING_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Write a 1 to reset pending CSS interrupts in the DSS interrupt controller"]
    #[inline(always)]
    pub fn dss_css_int_reset(&mut self) -> DSS_CSS_INT_RESET_W {
        DSS_CSS_INT_RESET_W { w: self }
    }
    #[doc = "Bit 4 - Write a 1 to reset pending DMA interrupts in the DSS interrupt controller"]
    #[inline(always)]
    pub fn dss_dma_int_reset(&mut self) -> DSS_DMA_INT_RESET_W {
        DSS_DMA_INT_RESET_W { w: self }
    }
    #[doc = "Bit 3 - Write a 1 to reset DSS"]
    #[inline(always)]
    pub fn dss_reset(&mut self) -> DSS_RESET_W {
        DSS_RESET_W { w: self }
    }
    #[doc = "Bit 2 - Write a 1 to pause LPDSP32"]
    #[inline(always)]
    pub fn lpdsp32_pause(&mut self) -> LPDSP32_PAUSE_W {
        LPDSP32_PAUSE_W { w: self }
    }
    #[doc = "Bit 1 - Write a 1 to run LPDSP32"]
    #[inline(always)]
    pub fn lpdsp32_resume(&mut self) -> LPDSP32_RESUME_W {
        LPDSP32_RESUME_W { w: self }
    }
}
