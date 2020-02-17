#[doc = "Reader of register ADC_BATMON_STATUS"]
pub type R = crate::R<u32, super::ADC_BATMON_STATUS>;
#[doc = "Writer for register ADC_BATMON_STATUS"]
pub type W = crate::W<u32, super::ADC_BATMON_STATUS>;
#[doc = "Register ADC_BATMON_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_BATMON_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Battery monitoring alarm status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BATMON_ALARM_CLEAR_AW {
    #[doc = "1: Writing a 1 clears the BATMON Alarm status bit"]
    BATMON_ALARM_CLEAR = 1,
}
impl From<BATMON_ALARM_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: BATMON_ALARM_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BATMON_ALARM_CLEAR`"]
pub struct BATMON_ALARM_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> BATMON_ALARM_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BATMON_ALARM_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing a 1 clears the BATMON Alarm status bit"]
    #[inline(always)]
    pub fn batmon_alarm_clear(self) -> &'a mut W {
        self.variant(BATMON_ALARM_CLEAR_AW::BATMON_ALARM_CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "ADC Overrun condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_OVERRUN_CLEAR_AW {
    #[doc = "1: Writing a 1 clears the ADC Overrun status bit"]
    ADC_OVERRUN_CLEAR = 1,
}
impl From<ADC_OVERRUN_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: ADC_OVERRUN_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ADC_OVERRUN_CLEAR`"]
pub struct ADC_OVERRUN_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_OVERRUN_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_OVERRUN_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing a 1 clears the ADC Overrun status bit"]
    #[inline(always)]
    pub fn adc_overrun_clear(self) -> &'a mut W {
        self.variant(ADC_OVERRUN_CLEAR_AW::ADC_OVERRUN_CLEAR)
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
#[doc = "ADC new sample Ready status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_READY_CLEAR_AW {
    #[doc = "1: Writing a 1 clears the ADC Ready status bit"]
    ADC_READY_CLEAR = 1,
}
impl From<ADC_READY_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: ADC_READY_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ADC_READY_CLEAR`"]
pub struct ADC_READY_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_READY_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_READY_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing a 1 clears the ADC Ready status bit"]
    #[inline(always)]
    pub fn adc_ready_clear(self) -> &'a mut W {
        self.variant(ADC_READY_CLEAR_AW::ADC_READY_CLEAR)
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
#[doc = "Battery monitoring alarm status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BATMON_ALARM_STAT_A {
    #[doc = "0: BATMON Alarm flag not set"]
    BATMON_ALARM_FALSE = 0,
    #[doc = "1: BATMON Alarm has been triggered"]
    BATMON_ALARM_TRUE = 1,
}
impl From<BATMON_ALARM_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: BATMON_ALARM_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BATMON_ALARM_STAT`"]
pub type BATMON_ALARM_STAT_R = crate::R<bool, BATMON_ALARM_STAT_A>;
impl BATMON_ALARM_STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BATMON_ALARM_STAT_A {
        match self.bits {
            false => BATMON_ALARM_STAT_A::BATMON_ALARM_FALSE,
            true => BATMON_ALARM_STAT_A::BATMON_ALARM_TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `BATMON_ALARM_FALSE`"]
    #[inline(always)]
    pub fn is_batmon_alarm_false(&self) -> bool {
        *self == BATMON_ALARM_STAT_A::BATMON_ALARM_FALSE
    }
    #[doc = "Checks if the value of the field is `BATMON_ALARM_TRUE`"]
    #[inline(always)]
    pub fn is_batmon_alarm_true(&self) -> bool {
        *self == BATMON_ALARM_STAT_A::BATMON_ALARM_TRUE
    }
}
#[doc = "ADC Overrun condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_OVERRUN_STAT_A {
    #[doc = "0: No ADC Overrun detected"]
    ADC_OVERRUN_FALSE = 0,
    #[doc = "1: ADC Overrun detected"]
    ADC_OVERRUN_TRUE = 1,
}
impl From<ADC_OVERRUN_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_OVERRUN_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_OVERRUN_STAT`"]
pub type ADC_OVERRUN_STAT_R = crate::R<bool, ADC_OVERRUN_STAT_A>;
impl ADC_OVERRUN_STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_OVERRUN_STAT_A {
        match self.bits {
            false => ADC_OVERRUN_STAT_A::ADC_OVERRUN_FALSE,
            true => ADC_OVERRUN_STAT_A::ADC_OVERRUN_TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_OVERRUN_FALSE`"]
    #[inline(always)]
    pub fn is_adc_overrun_false(&self) -> bool {
        *self == ADC_OVERRUN_STAT_A::ADC_OVERRUN_FALSE
    }
    #[doc = "Checks if the value of the field is `ADC_OVERRUN_TRUE`"]
    #[inline(always)]
    pub fn is_adc_overrun_true(&self) -> bool {
        *self == ADC_OVERRUN_STAT_A::ADC_OVERRUN_TRUE
    }
}
#[doc = "ADC new sample Ready status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_READY_STAT_A {
    #[doc = "0: No new ADC samples available"]
    ADC_READY_FALSE = 0,
    #[doc = "1: New ADC samples are ready"]
    ADC_READY_TRUE = 1,
}
impl From<ADC_READY_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_READY_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_READY_STAT`"]
pub type ADC_READY_STAT_R = crate::R<bool, ADC_READY_STAT_A>;
impl ADC_READY_STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_READY_STAT_A {
        match self.bits {
            false => ADC_READY_STAT_A::ADC_READY_FALSE,
            true => ADC_READY_STAT_A::ADC_READY_TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_READY_FALSE`"]
    #[inline(always)]
    pub fn is_adc_ready_false(&self) -> bool {
        *self == ADC_READY_STAT_A::ADC_READY_FALSE
    }
    #[doc = "Checks if the value of the field is `ADC_READY_TRUE`"]
    #[inline(always)]
    pub fn is_adc_ready_true(&self) -> bool {
        *self == ADC_READY_STAT_A::ADC_READY_TRUE
    }
}
impl R {
    #[doc = "Bit 4 - Battery monitoring alarm status bit"]
    #[inline(always)]
    pub fn batmon_alarm_stat(&self) -> BATMON_ALARM_STAT_R {
        BATMON_ALARM_STAT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC Overrun condition"]
    #[inline(always)]
    pub fn adc_overrun_stat(&self) -> ADC_OVERRUN_STAT_R {
        ADC_OVERRUN_STAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ADC new sample Ready status bit"]
    #[inline(always)]
    pub fn adc_ready_stat(&self) -> ADC_READY_STAT_R {
        ADC_READY_STAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Battery monitoring alarm status bit"]
    #[inline(always)]
    pub fn batmon_alarm_clear(&mut self) -> BATMON_ALARM_CLEAR_W {
        BATMON_ALARM_CLEAR_W { w: self }
    }
    #[doc = "Bit 9 - ADC Overrun condition"]
    #[inline(always)]
    pub fn adc_overrun_clear(&mut self) -> ADC_OVERRUN_CLEAR_W {
        ADC_OVERRUN_CLEAR_W { w: self }
    }
    #[doc = "Bit 8 - ADC new sample Ready status bit"]
    #[inline(always)]
    pub fn adc_ready_clear(&mut self) -> ADC_READY_CLEAR_W {
        ADC_READY_CLEAR_W { w: self }
    }
}
