#[doc = "Reader of register BB_DEEPSLCNTL"]
pub type R = crate::R<u32, super::BB_DEEPSLCNTL>;
#[doc = "Writer for register BB_DEEPSLCNTL"]
pub type W = crate::W<u32, super::BB_DEEPSLCNTL>;
#[doc = "Register BB_DEEPSLCNTL `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_DEEPSLCNTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "External wake-up disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTWKUPDSB_A {
    #[doc = "0: RW-BLE core can be woken by external wake-up"]
    EXTWKUPDSB_0 = 0,
    #[doc = "1: RW-BLE core cannot be woken up by external wake-up"]
    EXTWKUPDSB_1 = 1,
}
impl From<EXTWKUPDSB_A> for bool {
    #[inline(always)]
    fn from(variant: EXTWKUPDSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTWKUPDSB`"]
pub type EXTWKUPDSB_R = crate::R<bool, EXTWKUPDSB_A>;
impl EXTWKUPDSB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTWKUPDSB_A {
        match self.bits {
            false => EXTWKUPDSB_A::EXTWKUPDSB_0,
            true => EXTWKUPDSB_A::EXTWKUPDSB_1,
        }
    }
    #[doc = "Checks if the value of the field is `EXTWKUPDSB_0`"]
    #[inline(always)]
    pub fn is_extwkupdsb_0(&self) -> bool {
        *self == EXTWKUPDSB_A::EXTWKUPDSB_0
    }
    #[doc = "Checks if the value of the field is `EXTWKUPDSB_1`"]
    #[inline(always)]
    pub fn is_extwkupdsb_1(&self) -> bool {
        *self == EXTWKUPDSB_A::EXTWKUPDSB_1
    }
}
#[doc = "Write proxy for field `EXTWKUPDSB`"]
pub struct EXTWKUPDSB_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTWKUPDSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTWKUPDSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RW-BLE core can be woken by external wake-up"]
    #[inline(always)]
    pub fn extwkupdsb_0(self) -> &'a mut W {
        self.variant(EXTWKUPDSB_A::EXTWKUPDSB_0)
    }
    #[doc = "RW-BLE core cannot be woken up by external wake-up"]
    #[inline(always)]
    pub fn extwkupdsb_1(self) -> &'a mut W {
        self.variant(EXTWKUPDSB_A::EXTWKUPDSB_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Indicator of current deep sleep clock mux status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEEP_SLEEP_STAT_A {
    #[doc = "0: RW-BLE core is not yet in deep sleep mode"]
    DEEP_SLEEP_STAT_0 = 0,
    #[doc = "1: RW-BLE core is in deep sleep mode (only low_power_clk is running)"]
    DEEP_SLEEP_STAT_1 = 1,
}
impl From<DEEP_SLEEP_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: DEEP_SLEEP_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEEP_SLEEP_STAT`"]
pub type DEEP_SLEEP_STAT_R = crate::R<bool, DEEP_SLEEP_STAT_A>;
impl DEEP_SLEEP_STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEEP_SLEEP_STAT_A {
        match self.bits {
            false => DEEP_SLEEP_STAT_A::DEEP_SLEEP_STAT_0,
            true => DEEP_SLEEP_STAT_A::DEEP_SLEEP_STAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEEP_SLEEP_STAT_0`"]
    #[inline(always)]
    pub fn is_deep_sleep_stat_0(&self) -> bool {
        *self == DEEP_SLEEP_STAT_A::DEEP_SLEEP_STAT_0
    }
    #[doc = "Checks if the value of the field is `DEEP_SLEEP_STAT_1`"]
    #[inline(always)]
    pub fn is_deep_sleep_stat_1(&self) -> bool {
        *self == DEEP_SLEEP_STAT_A::DEEP_SLEEP_STAT_1
    }
}
#[doc = "Wake up request from RW-BLE software applying when system is in deep sleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFT_WAKEUP_REQ_A {
    #[doc = "0: No action happens if it is written with 0"]
    SOFT_WAKEUP_REQ_0 = 0,
    #[doc = "1: Wake up request from RW-BLE software"]
    SOFT_WAKEUP_REQ_1 = 1,
}
impl From<SOFT_WAKEUP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: SOFT_WAKEUP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOFT_WAKEUP_REQ`"]
pub type SOFT_WAKEUP_REQ_R = crate::R<bool, SOFT_WAKEUP_REQ_A>;
impl SOFT_WAKEUP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFT_WAKEUP_REQ_A {
        match self.bits {
            false => SOFT_WAKEUP_REQ_A::SOFT_WAKEUP_REQ_0,
            true => SOFT_WAKEUP_REQ_A::SOFT_WAKEUP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SOFT_WAKEUP_REQ_0`"]
    #[inline(always)]
    pub fn is_soft_wakeup_req_0(&self) -> bool {
        *self == SOFT_WAKEUP_REQ_A::SOFT_WAKEUP_REQ_0
    }
    #[doc = "Checks if the value of the field is `SOFT_WAKEUP_REQ_1`"]
    #[inline(always)]
    pub fn is_soft_wakeup_req_1(&self) -> bool {
        *self == SOFT_WAKEUP_REQ_A::SOFT_WAKEUP_REQ_1
    }
}
#[doc = "Write proxy for field `SOFT_WAKEUP_REQ`"]
pub struct SOFT_WAKEUP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_WAKEUP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOFT_WAKEUP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action happens if it is written with 0"]
    #[inline(always)]
    pub fn soft_wakeup_req_0(self) -> &'a mut W {
        self.variant(SOFT_WAKEUP_REQ_A::SOFT_WAKEUP_REQ_0)
    }
    #[doc = "Wake up request from RW-BLE software"]
    #[inline(always)]
    pub fn soft_wakeup_req_1(self) -> &'a mut W {
        self.variant(SOFT_WAKEUP_REQ_A::SOFT_WAKEUP_REQ_1)
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
#[doc = "625us base time reference integer and fractional part correction applying when system has been woken-up from deep sleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEEP_SLEEP_CORR_EN_A {
    #[doc = "0: No action happens if it is written with 0"]
    DEEP_SLEEP_CORR_EN_0 = 0,
    #[doc = "1: Enables fine counter and base time counter when written"]
    DEEP_SLEEP_CORR_EN_1 = 1,
}
impl From<DEEP_SLEEP_CORR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DEEP_SLEEP_CORR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEEP_SLEEP_CORR_EN`"]
pub type DEEP_SLEEP_CORR_EN_R = crate::R<bool, DEEP_SLEEP_CORR_EN_A>;
impl DEEP_SLEEP_CORR_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEEP_SLEEP_CORR_EN_A {
        match self.bits {
            false => DEEP_SLEEP_CORR_EN_A::DEEP_SLEEP_CORR_EN_0,
            true => DEEP_SLEEP_CORR_EN_A::DEEP_SLEEP_CORR_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEEP_SLEEP_CORR_EN_0`"]
    #[inline(always)]
    pub fn is_deep_sleep_corr_en_0(&self) -> bool {
        *self == DEEP_SLEEP_CORR_EN_A::DEEP_SLEEP_CORR_EN_0
    }
    #[doc = "Checks if the value of the field is `DEEP_SLEEP_CORR_EN_1`"]
    #[inline(always)]
    pub fn is_deep_sleep_corr_en_1(&self) -> bool {
        *self == DEEP_SLEEP_CORR_EN_A::DEEP_SLEEP_CORR_EN_1
    }
}
#[doc = "Write proxy for field `DEEP_SLEEP_CORR_EN`"]
pub struct DEEP_SLEEP_CORR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEP_SLEEP_CORR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEEP_SLEEP_CORR_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action happens if it is written with 0"]
    #[inline(always)]
    pub fn deep_sleep_corr_en_0(self) -> &'a mut W {
        self.variant(DEEP_SLEEP_CORR_EN_A::DEEP_SLEEP_CORR_EN_0)
    }
    #[doc = "Enables fine counter and base time counter when written"]
    #[inline(always)]
    pub fn deep_sleep_corr_en_1(self) -> &'a mut W {
        self.variant(DEEP_SLEEP_CORR_EN_A::DEEP_SLEEP_CORR_EN_1)
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
#[doc = "RW-BLE core power mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEEP_SLEEP_ON_A {
    #[doc = "0: RW-BLE core in normal active mode"]
    DEEP_SLEEP_ON_0 = 0,
    #[doc = "1: Request RW-BLE core to switch in deep sleep mode"]
    DEEP_SLEEP_ON_1 = 1,
}
impl From<DEEP_SLEEP_ON_A> for bool {
    #[inline(always)]
    fn from(variant: DEEP_SLEEP_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEEP_SLEEP_ON`"]
pub type DEEP_SLEEP_ON_R = crate::R<bool, DEEP_SLEEP_ON_A>;
impl DEEP_SLEEP_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEEP_SLEEP_ON_A {
        match self.bits {
            false => DEEP_SLEEP_ON_A::DEEP_SLEEP_ON_0,
            true => DEEP_SLEEP_ON_A::DEEP_SLEEP_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEEP_SLEEP_ON_0`"]
    #[inline(always)]
    pub fn is_deep_sleep_on_0(&self) -> bool {
        *self == DEEP_SLEEP_ON_A::DEEP_SLEEP_ON_0
    }
    #[doc = "Checks if the value of the field is `DEEP_SLEEP_ON_1`"]
    #[inline(always)]
    pub fn is_deep_sleep_on_1(&self) -> bool {
        *self == DEEP_SLEEP_ON_A::DEEP_SLEEP_ON_1
    }
}
#[doc = "Write proxy for field `DEEP_SLEEP_ON`"]
pub struct DEEP_SLEEP_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEP_SLEEP_ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEEP_SLEEP_ON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RW-BLE core in normal active mode"]
    #[inline(always)]
    pub fn deep_sleep_on_0(self) -> &'a mut W {
        self.variant(DEEP_SLEEP_ON_A::DEEP_SLEEP_ON_0)
    }
    #[doc = "Request RW-BLE core to switch in deep sleep mode"]
    #[inline(always)]
    pub fn deep_sleep_on_1(self) -> &'a mut W {
        self.variant(DEEP_SLEEP_ON_A::DEEP_SLEEP_ON_1)
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
#[doc = "Controls the radio module\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RADIO_SLEEP_EN_A {
    #[doc = "0: radio stands in normal active mode"]
    RADIO_SLEEP_EN_0 = 0,
    #[doc = "1: Allow to disable radio"]
    RADIO_SLEEP_EN_1 = 1,
}
impl From<RADIO_SLEEP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RADIO_SLEEP_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RADIO_SLEEP_EN`"]
pub type RADIO_SLEEP_EN_R = crate::R<bool, RADIO_SLEEP_EN_A>;
impl RADIO_SLEEP_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RADIO_SLEEP_EN_A {
        match self.bits {
            false => RADIO_SLEEP_EN_A::RADIO_SLEEP_EN_0,
            true => RADIO_SLEEP_EN_A::RADIO_SLEEP_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RADIO_SLEEP_EN_0`"]
    #[inline(always)]
    pub fn is_radio_sleep_en_0(&self) -> bool {
        *self == RADIO_SLEEP_EN_A::RADIO_SLEEP_EN_0
    }
    #[doc = "Checks if the value of the field is `RADIO_SLEEP_EN_1`"]
    #[inline(always)]
    pub fn is_radio_sleep_en_1(&self) -> bool {
        *self == RADIO_SLEEP_EN_A::RADIO_SLEEP_EN_1
    }
}
#[doc = "Write proxy for field `RADIO_SLEEP_EN`"]
pub struct RADIO_SLEEP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RADIO_SLEEP_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RADIO_SLEEP_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "radio stands in normal active mode"]
    #[inline(always)]
    pub fn radio_sleep_en_0(self) -> &'a mut W {
        self.variant(RADIO_SLEEP_EN_A::RADIO_SLEEP_EN_0)
    }
    #[doc = "Allow to disable radio"]
    #[inline(always)]
    pub fn radio_sleep_en_1(self) -> &'a mut W {
        self.variant(RADIO_SLEEP_EN_A::RADIO_SLEEP_EN_1)
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
#[doc = "Controls the RF high frequency crystal oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC_SLEEP_EN_A {
    #[doc = "0: High frequency crystal oscillator stands in normal active mode"]
    OSC_SLEEP_EN_0 = 0,
    #[doc = "1: Allow to disable high frequency crystal oscillator"]
    OSC_SLEEP_EN_1 = 1,
}
impl From<OSC_SLEEP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OSC_SLEEP_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OSC_SLEEP_EN`"]
pub type OSC_SLEEP_EN_R = crate::R<bool, OSC_SLEEP_EN_A>;
impl OSC_SLEEP_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSC_SLEEP_EN_A {
        match self.bits {
            false => OSC_SLEEP_EN_A::OSC_SLEEP_EN_0,
            true => OSC_SLEEP_EN_A::OSC_SLEEP_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `OSC_SLEEP_EN_0`"]
    #[inline(always)]
    pub fn is_osc_sleep_en_0(&self) -> bool {
        *self == OSC_SLEEP_EN_A::OSC_SLEEP_EN_0
    }
    #[doc = "Checks if the value of the field is `OSC_SLEEP_EN_1`"]
    #[inline(always)]
    pub fn is_osc_sleep_en_1(&self) -> bool {
        *self == OSC_SLEEP_EN_A::OSC_SLEEP_EN_1
    }
}
#[doc = "Write proxy for field `OSC_SLEEP_EN`"]
pub struct OSC_SLEEP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_SLEEP_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSC_SLEEP_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "High frequency crystal oscillator stands in normal active mode"]
    #[inline(always)]
    pub fn osc_sleep_en_0(self) -> &'a mut W {
        self.variant(OSC_SLEEP_EN_A::OSC_SLEEP_EN_0)
    }
    #[doc = "Allow to disable high frequency crystal oscillator"]
    #[inline(always)]
    pub fn osc_sleep_en_1(self) -> &'a mut W {
        self.variant(OSC_SLEEP_EN_A::OSC_SLEEP_EN_1)
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
    #[doc = "Bit 31 - External wake-up disable"]
    #[inline(always)]
    pub fn extwkupdsb(&self) -> EXTWKUPDSB_R {
        EXTWKUPDSB_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Indicator of current deep sleep clock mux status"]
    #[inline(always)]
    pub fn deep_sleep_stat(&self) -> DEEP_SLEEP_STAT_R {
        DEEP_SLEEP_STAT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wake up request from RW-BLE software applying when system is in deep sleep mode"]
    #[inline(always)]
    pub fn soft_wakeup_req(&self) -> SOFT_WAKEUP_REQ_R {
        SOFT_WAKEUP_REQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 625us base time reference integer and fractional part correction applying when system has been woken-up from deep sleep mode"]
    #[inline(always)]
    pub fn deep_sleep_corr_en(&self) -> DEEP_SLEEP_CORR_EN_R {
        DEEP_SLEEP_CORR_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RW-BLE core power mode control"]
    #[inline(always)]
    pub fn deep_sleep_on(&self) -> DEEP_SLEEP_ON_R {
        DEEP_SLEEP_ON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Controls the radio module"]
    #[inline(always)]
    pub fn radio_sleep_en(&self) -> RADIO_SLEEP_EN_R {
        RADIO_SLEEP_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Controls the RF high frequency crystal oscillator"]
    #[inline(always)]
    pub fn osc_sleep_en(&self) -> OSC_SLEEP_EN_R {
        OSC_SLEEP_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - External wake-up disable"]
    #[inline(always)]
    pub fn extwkupdsb(&mut self) -> EXTWKUPDSB_W {
        EXTWKUPDSB_W { w: self }
    }
    #[doc = "Bit 4 - Wake up request from RW-BLE software applying when system is in deep sleep mode"]
    #[inline(always)]
    pub fn soft_wakeup_req(&mut self) -> SOFT_WAKEUP_REQ_W {
        SOFT_WAKEUP_REQ_W { w: self }
    }
    #[doc = "Bit 3 - 625us base time reference integer and fractional part correction applying when system has been woken-up from deep sleep mode"]
    #[inline(always)]
    pub fn deep_sleep_corr_en(&mut self) -> DEEP_SLEEP_CORR_EN_W {
        DEEP_SLEEP_CORR_EN_W { w: self }
    }
    #[doc = "Bit 2 - RW-BLE core power mode control"]
    #[inline(always)]
    pub fn deep_sleep_on(&mut self) -> DEEP_SLEEP_ON_W {
        DEEP_SLEEP_ON_W { w: self }
    }
    #[doc = "Bit 1 - Controls the radio module"]
    #[inline(always)]
    pub fn radio_sleep_en(&mut self) -> RADIO_SLEEP_EN_W {
        RADIO_SLEEP_EN_W { w: self }
    }
    #[doc = "Bit 0 - Controls the RF high frequency crystal oscillator"]
    #[inline(always)]
    pub fn osc_sleep_en(&mut self) -> OSC_SLEEP_EN_W {
        OSC_SLEEP_EN_W { w: self }
    }
}
