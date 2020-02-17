#[doc = "Reader of register ADC_BATMON_CFG"]
pub type R = crate::R<u32, super::ADC_BATMON_CFG>;
#[doc = "Writer for register ADC_BATMON_CFG"]
pub type W = crate::W<u32, super::ADC_BATMON_CFG>;
#[doc = "Register ADC_BATMON_CFG `reset()`'s with value 0x8000"]
impl crate::ResetValue for super::ADC_BATMON_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000
    }
}
#[doc = "An Alarm Status bit gets set when SUPPLY_COUNT_VALUE= ALARM_COUNT_VALUE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ALARM_COUNT_VALUE_A {
    #[doc = "0: No Alarm is triggered"]
    BATMON_ALARM_NONE = 0,
    #[doc = "1: Alarm count value is 1"]
    BATMON_ALARM_COUNT1 = 1,
    #[doc = "255: Alarm count value is 255"]
    BATMON_ALARM_COUNT255 = 255,
}
impl From<ALARM_COUNT_VALUE_A> for u8 {
    #[inline(always)]
    fn from(variant: ALARM_COUNT_VALUE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ALARM_COUNT_VALUE`"]
pub type ALARM_COUNT_VALUE_R = crate::R<u8, ALARM_COUNT_VALUE_A>;
impl ALARM_COUNT_VALUE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ALARM_COUNT_VALUE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ALARM_COUNT_VALUE_A::BATMON_ALARM_NONE),
            1 => Val(ALARM_COUNT_VALUE_A::BATMON_ALARM_COUNT1),
            255 => Val(ALARM_COUNT_VALUE_A::BATMON_ALARM_COUNT255),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BATMON_ALARM_NONE`"]
    #[inline(always)]
    pub fn is_batmon_alarm_none(&self) -> bool {
        *self == ALARM_COUNT_VALUE_A::BATMON_ALARM_NONE
    }
    #[doc = "Checks if the value of the field is `BATMON_ALARM_COUNT1`"]
    #[inline(always)]
    pub fn is_batmon_alarm_count1(&self) -> bool {
        *self == ALARM_COUNT_VALUE_A::BATMON_ALARM_COUNT1
    }
    #[doc = "Checks if the value of the field is `BATMON_ALARM_COUNT255`"]
    #[inline(always)]
    pub fn is_batmon_alarm_count255(&self) -> bool {
        *self == ALARM_COUNT_VALUE_A::BATMON_ALARM_COUNT255
    }
}
#[doc = "Write proxy for field `ALARM_COUNT_VALUE`"]
pub struct ALARM_COUNT_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARM_COUNT_VALUE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALARM_COUNT_VALUE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No Alarm is triggered"]
    #[inline(always)]
    pub fn batmon_alarm_none(self) -> &'a mut W {
        self.variant(ALARM_COUNT_VALUE_A::BATMON_ALARM_NONE)
    }
    #[doc = "Alarm count value is 1"]
    #[inline(always)]
    pub fn batmon_alarm_count1(self) -> &'a mut W {
        self.variant(ALARM_COUNT_VALUE_A::BATMON_ALARM_COUNT1)
    }
    #[doc = "Alarm count value is 255"]
    #[inline(always)]
    pub fn batmon_alarm_count255(self) -> &'a mut W {
        self.variant(ALARM_COUNT_VALUE_A::BATMON_ALARM_COUNT255)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Low voltage detection threshold (7.8 mV steps)\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUPPLY_THRESHOLD_A {
    #[doc = "0: Lowest voltage threshold: 7.8 mV"]
    SUPPLY_THRESHOLD_LOW = 0,
    #[doc = "128: Mid voltage threshold: 1 V"]
    SUPPLY_THRESHOLD_MID = 128,
    #[doc = "255: Highest voltage threshold: ~2.0 V"]
    SUPPLY_THRESHOLD_HIGH = 255,
}
impl From<SUPPLY_THRESHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: SUPPLY_THRESHOLD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SUPPLY_THRESHOLD`"]
pub type SUPPLY_THRESHOLD_R = crate::R<u8, SUPPLY_THRESHOLD_A>;
impl SUPPLY_THRESHOLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUPPLY_THRESHOLD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SUPPLY_THRESHOLD_A::SUPPLY_THRESHOLD_LOW),
            128 => Val(SUPPLY_THRESHOLD_A::SUPPLY_THRESHOLD_MID),
            255 => Val(SUPPLY_THRESHOLD_A::SUPPLY_THRESHOLD_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUPPLY_THRESHOLD_LOW`"]
    #[inline(always)]
    pub fn is_supply_threshold_low(&self) -> bool {
        *self == SUPPLY_THRESHOLD_A::SUPPLY_THRESHOLD_LOW
    }
    #[doc = "Checks if the value of the field is `SUPPLY_THRESHOLD_MID`"]
    #[inline(always)]
    pub fn is_supply_threshold_mid(&self) -> bool {
        *self == SUPPLY_THRESHOLD_A::SUPPLY_THRESHOLD_MID
    }
    #[doc = "Checks if the value of the field is `SUPPLY_THRESHOLD_HIGH`"]
    #[inline(always)]
    pub fn is_supply_threshold_high(&self) -> bool {
        *self == SUPPLY_THRESHOLD_A::SUPPLY_THRESHOLD_HIGH
    }
}
#[doc = "Write proxy for field `SUPPLY_THRESHOLD`"]
pub struct SUPPLY_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SUPPLY_THRESHOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUPPLY_THRESHOLD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Lowest voltage threshold: 7.8 mV"]
    #[inline(always)]
    pub fn supply_threshold_low(self) -> &'a mut W {
        self.variant(SUPPLY_THRESHOLD_A::SUPPLY_THRESHOLD_LOW)
    }
    #[doc = "Mid voltage threshold: 1 V"]
    #[inline(always)]
    pub fn supply_threshold_mid(self) -> &'a mut W {
        self.variant(SUPPLY_THRESHOLD_A::SUPPLY_THRESHOLD_MID)
    }
    #[doc = "Highest voltage threshold: ~2.0 V"]
    #[inline(always)]
    pub fn supply_threshold_high(self) -> &'a mut W {
        self.variant(SUPPLY_THRESHOLD_A::SUPPLY_THRESHOLD_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Selects the power supply voltage source to be monitored\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUPPLY_SRC_A {
    #[doc = "0: Channel 6 (typically VBAT divided by 2) is monitored"]
    BATMON_CH6 = 0,
    #[doc = "1: Channel 7 (typically VDDC) is monitored"]
    BATMON_CH7 = 1,
}
impl From<SUPPLY_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: SUPPLY_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SUPPLY_SRC`"]
pub type SUPPLY_SRC_R = crate::R<bool, SUPPLY_SRC_A>;
impl SUPPLY_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUPPLY_SRC_A {
        match self.bits {
            false => SUPPLY_SRC_A::BATMON_CH6,
            true => SUPPLY_SRC_A::BATMON_CH7,
        }
    }
    #[doc = "Checks if the value of the field is `BATMON_CH6`"]
    #[inline(always)]
    pub fn is_batmon_ch6(&self) -> bool {
        *self == SUPPLY_SRC_A::BATMON_CH6
    }
    #[doc = "Checks if the value of the field is `BATMON_CH7`"]
    #[inline(always)]
    pub fn is_batmon_ch7(&self) -> bool {
        *self == SUPPLY_SRC_A::BATMON_CH7
    }
}
#[doc = "Write proxy for field `SUPPLY_SRC`"]
pub struct SUPPLY_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SUPPLY_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUPPLY_SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel 6 (typically VBAT divided by 2) is monitored"]
    #[inline(always)]
    pub fn batmon_ch6(self) -> &'a mut W {
        self.variant(SUPPLY_SRC_A::BATMON_CH6)
    }
    #[doc = "Channel 7 (typically VDDC) is monitored"]
    #[inline(always)]
    pub fn batmon_ch7(self) -> &'a mut W {
        self.variant(SUPPLY_SRC_A::BATMON_CH7)
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
    #[doc = "Bits 16:23 - An Alarm Status bit gets set when SUPPLY_COUNT_VALUE= ALARM_COUNT_VALUE"]
    #[inline(always)]
    pub fn alarm_count_value(&self) -> ALARM_COUNT_VALUE_R {
        ALARM_COUNT_VALUE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Low voltage detection threshold (7.8 mV steps)"]
    #[inline(always)]
    pub fn supply_threshold(&self) -> SUPPLY_THRESHOLD_R {
        SUPPLY_THRESHOLD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 0 - Selects the power supply voltage source to be monitored"]
    #[inline(always)]
    pub fn supply_src(&self) -> SUPPLY_SRC_R {
        SUPPLY_SRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:23 - An Alarm Status bit gets set when SUPPLY_COUNT_VALUE= ALARM_COUNT_VALUE"]
    #[inline(always)]
    pub fn alarm_count_value(&mut self) -> ALARM_COUNT_VALUE_W {
        ALARM_COUNT_VALUE_W { w: self }
    }
    #[doc = "Bits 8:15 - Low voltage detection threshold (7.8 mV steps)"]
    #[inline(always)]
    pub fn supply_threshold(&mut self) -> SUPPLY_THRESHOLD_W {
        SUPPLY_THRESHOLD_W { w: self }
    }
    #[doc = "Bit 0 - Selects the power supply voltage source to be monitored"]
    #[inline(always)]
    pub fn supply_src(&mut self) -> SUPPLY_SRC_W {
        SUPPLY_SRC_W { w: self }
    }
}
