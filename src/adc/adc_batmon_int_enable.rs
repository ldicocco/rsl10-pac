#[doc = "Reader of register ADC_BATMON_INT_ENABLE"]
pub type R = crate::R<u32, super::ADC_BATMON_INT_ENABLE>;
#[doc = "Writer for register ADC_BATMON_INT_ENABLE"]
pub type W = crate::W<u32, super::ADC_BATMON_INT_ENABLE>;
#[doc = "Register ADC_BATMON_INT_ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_BATMON_INT_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "The BATMON Alarm interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BATMON_ALARM_INT_ENABLE_A {
    #[doc = "0: This source cannot set an interrupt"]
    INT_DIS_BATMON_ALARM = 0,
    #[doc = "1: This source can set the ADC interrupt line"]
    INT_EBL_BATMON_ALARM = 1,
}
impl From<BATMON_ALARM_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BATMON_ALARM_INT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BATMON_ALARM_INT_ENABLE`"]
pub type BATMON_ALARM_INT_ENABLE_R = crate::R<bool, BATMON_ALARM_INT_ENABLE_A>;
impl BATMON_ALARM_INT_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BATMON_ALARM_INT_ENABLE_A {
        match self.bits {
            false => BATMON_ALARM_INT_ENABLE_A::INT_DIS_BATMON_ALARM,
            true => BATMON_ALARM_INT_ENABLE_A::INT_EBL_BATMON_ALARM,
        }
    }
    #[doc = "Checks if the value of the field is `INT_DIS_BATMON_ALARM`"]
    #[inline(always)]
    pub fn is_int_dis_batmon_alarm(&self) -> bool {
        *self == BATMON_ALARM_INT_ENABLE_A::INT_DIS_BATMON_ALARM
    }
    #[doc = "Checks if the value of the field is `INT_EBL_BATMON_ALARM`"]
    #[inline(always)]
    pub fn is_int_ebl_batmon_alarm(&self) -> bool {
        *self == BATMON_ALARM_INT_ENABLE_A::INT_EBL_BATMON_ALARM
    }
}
#[doc = "Write proxy for field `BATMON_ALARM_INT_ENABLE`"]
pub struct BATMON_ALARM_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BATMON_ALARM_INT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BATMON_ALARM_INT_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This source cannot set an interrupt"]
    #[inline(always)]
    pub fn int_dis_batmon_alarm(self) -> &'a mut W {
        self.variant(BATMON_ALARM_INT_ENABLE_A::INT_DIS_BATMON_ALARM)
    }
    #[doc = "This source can set the ADC interrupt line"]
    #[inline(always)]
    pub fn int_ebl_batmon_alarm(self) -> &'a mut W {
        self.variant(BATMON_ALARM_INT_ENABLE_A::INT_EBL_BATMON_ALARM)
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
#[doc = "Channel number triggering the ADC interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_INT_CH_NUM_A {
    #[doc = "0: The ADC interrupt is triggered when the ADC_DATA_CH0 register is updated"]
    ADC_INT_CH0 = 0,
    #[doc = "1: The ADC interrupt is triggered when the ADC_DATA_CH1 register is updated"]
    ADC_INT_CH1 = 1,
    #[doc = "2: The ADC interrupt is triggered when the ADC_DATA_CH2 register is updated"]
    ADC_INT_CH2 = 2,
    #[doc = "3: The ADC interrupt is triggered when the ADC_DATA_CH3 register is updated"]
    ADC_INT_CH3 = 3,
    #[doc = "4: The ADC interrupt is triggered when the ADC_DATA_CH4 register is updated"]
    ADC_INT_CH4 = 4,
    #[doc = "5: The ADC interrupt is triggered when the ADC_DATA_CH5 register is updated"]
    ADC_INT_CH5 = 5,
    #[doc = "6: The ADC interrupt is triggered when the ADC_DATA_CH6 register is updated"]
    ADC_INT_CH6 = 6,
    #[doc = "7: The ADC interrupt is triggered when the ADC_DATA_CH7 register is updated"]
    ADC_INT_CH7 = 7,
}
impl From<ADC_INT_CH_NUM_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_INT_CH_NUM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC_INT_CH_NUM`"]
pub type ADC_INT_CH_NUM_R = crate::R<u8, ADC_INT_CH_NUM_A>;
impl ADC_INT_CH_NUM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_INT_CH_NUM_A {
        match self.bits {
            0 => ADC_INT_CH_NUM_A::ADC_INT_CH0,
            1 => ADC_INT_CH_NUM_A::ADC_INT_CH1,
            2 => ADC_INT_CH_NUM_A::ADC_INT_CH2,
            3 => ADC_INT_CH_NUM_A::ADC_INT_CH3,
            4 => ADC_INT_CH_NUM_A::ADC_INT_CH4,
            5 => ADC_INT_CH_NUM_A::ADC_INT_CH5,
            6 => ADC_INT_CH_NUM_A::ADC_INT_CH6,
            7 => ADC_INT_CH_NUM_A::ADC_INT_CH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_INT_CH0`"]
    #[inline(always)]
    pub fn is_adc_int_ch0(&self) -> bool {
        *self == ADC_INT_CH_NUM_A::ADC_INT_CH0
    }
    #[doc = "Checks if the value of the field is `ADC_INT_CH1`"]
    #[inline(always)]
    pub fn is_adc_int_ch1(&self) -> bool {
        *self == ADC_INT_CH_NUM_A::ADC_INT_CH1
    }
    #[doc = "Checks if the value of the field is `ADC_INT_CH2`"]
    #[inline(always)]
    pub fn is_adc_int_ch2(&self) -> bool {
        *self == ADC_INT_CH_NUM_A::ADC_INT_CH2
    }
    #[doc = "Checks if the value of the field is `ADC_INT_CH3`"]
    #[inline(always)]
    pub fn is_adc_int_ch3(&self) -> bool {
        *self == ADC_INT_CH_NUM_A::ADC_INT_CH3
    }
    #[doc = "Checks if the value of the field is `ADC_INT_CH4`"]
    #[inline(always)]
    pub fn is_adc_int_ch4(&self) -> bool {
        *self == ADC_INT_CH_NUM_A::ADC_INT_CH4
    }
    #[doc = "Checks if the value of the field is `ADC_INT_CH5`"]
    #[inline(always)]
    pub fn is_adc_int_ch5(&self) -> bool {
        *self == ADC_INT_CH_NUM_A::ADC_INT_CH5
    }
    #[doc = "Checks if the value of the field is `ADC_INT_CH6`"]
    #[inline(always)]
    pub fn is_adc_int_ch6(&self) -> bool {
        *self == ADC_INT_CH_NUM_A::ADC_INT_CH6
    }
    #[doc = "Checks if the value of the field is `ADC_INT_CH7`"]
    #[inline(always)]
    pub fn is_adc_int_ch7(&self) -> bool {
        *self == ADC_INT_CH_NUM_A::ADC_INT_CH7
    }
}
#[doc = "Write proxy for field `ADC_INT_CH_NUM`"]
pub struct ADC_INT_CH_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_INT_CH_NUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_INT_CH_NUM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The ADC interrupt is triggered when the ADC_DATA_CH0 register is updated"]
    #[inline(always)]
    pub fn adc_int_ch0(self) -> &'a mut W {
        self.variant(ADC_INT_CH_NUM_A::ADC_INT_CH0)
    }
    #[doc = "The ADC interrupt is triggered when the ADC_DATA_CH1 register is updated"]
    #[inline(always)]
    pub fn adc_int_ch1(self) -> &'a mut W {
        self.variant(ADC_INT_CH_NUM_A::ADC_INT_CH1)
    }
    #[doc = "The ADC interrupt is triggered when the ADC_DATA_CH2 register is updated"]
    #[inline(always)]
    pub fn adc_int_ch2(self) -> &'a mut W {
        self.variant(ADC_INT_CH_NUM_A::ADC_INT_CH2)
    }
    #[doc = "The ADC interrupt is triggered when the ADC_DATA_CH3 register is updated"]
    #[inline(always)]
    pub fn adc_int_ch3(self) -> &'a mut W {
        self.variant(ADC_INT_CH_NUM_A::ADC_INT_CH3)
    }
    #[doc = "The ADC interrupt is triggered when the ADC_DATA_CH4 register is updated"]
    #[inline(always)]
    pub fn adc_int_ch4(self) -> &'a mut W {
        self.variant(ADC_INT_CH_NUM_A::ADC_INT_CH4)
    }
    #[doc = "The ADC interrupt is triggered when the ADC_DATA_CH5 register is updated"]
    #[inline(always)]
    pub fn adc_int_ch5(self) -> &'a mut W {
        self.variant(ADC_INT_CH_NUM_A::ADC_INT_CH5)
    }
    #[doc = "The ADC interrupt is triggered when the ADC_DATA_CH6 register is updated"]
    #[inline(always)]
    pub fn adc_int_ch6(self) -> &'a mut W {
        self.variant(ADC_INT_CH_NUM_A::ADC_INT_CH6)
    }
    #[doc = "The ADC interrupt is triggered when the ADC_DATA_CH7 register is updated"]
    #[inline(always)]
    pub fn adc_int_ch7(self) -> &'a mut W {
        self.variant(ADC_INT_CH_NUM_A::ADC_INT_CH7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "The ADC new sample ready interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_INT_ENABLE_A {
    #[doc = "0: This source cannot set an interrupt"]
    INT_DIS_ADC = 0,
    #[doc = "1: This source can set the ADC interrupt line"]
    INT_EBL_ADC = 1,
}
impl From<ADC_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_INT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_INT_ENABLE`"]
pub type ADC_INT_ENABLE_R = crate::R<bool, ADC_INT_ENABLE_A>;
impl ADC_INT_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_INT_ENABLE_A {
        match self.bits {
            false => ADC_INT_ENABLE_A::INT_DIS_ADC,
            true => ADC_INT_ENABLE_A::INT_EBL_ADC,
        }
    }
    #[doc = "Checks if the value of the field is `INT_DIS_ADC`"]
    #[inline(always)]
    pub fn is_int_dis_adc(&self) -> bool {
        *self == ADC_INT_ENABLE_A::INT_DIS_ADC
    }
    #[doc = "Checks if the value of the field is `INT_EBL_ADC`"]
    #[inline(always)]
    pub fn is_int_ebl_adc(&self) -> bool {
        *self == ADC_INT_ENABLE_A::INT_EBL_ADC
    }
}
#[doc = "Write proxy for field `ADC_INT_ENABLE`"]
pub struct ADC_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_INT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_INT_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This source cannot set an interrupt"]
    #[inline(always)]
    pub fn int_dis_adc(self) -> &'a mut W {
        self.variant(ADC_INT_ENABLE_A::INT_DIS_ADC)
    }
    #[doc = "This source can set the ADC interrupt line"]
    #[inline(always)]
    pub fn int_ebl_adc(self) -> &'a mut W {
        self.variant(ADC_INT_ENABLE_A::INT_EBL_ADC)
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
    #[doc = "Bit 4 - The BATMON Alarm interrupt mask"]
    #[inline(always)]
    pub fn batmon_alarm_int_enable(&self) -> BATMON_ALARM_INT_ENABLE_R {
        BATMON_ALARM_INT_ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Channel number triggering the ADC interrupt"]
    #[inline(always)]
    pub fn adc_int_ch_num(&self) -> ADC_INT_CH_NUM_R {
        ADC_INT_CH_NUM_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - The ADC new sample ready interrupt mask"]
    #[inline(always)]
    pub fn adc_int_enable(&self) -> ADC_INT_ENABLE_R {
        ADC_INT_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - The BATMON Alarm interrupt mask"]
    #[inline(always)]
    pub fn batmon_alarm_int_enable(&mut self) -> BATMON_ALARM_INT_ENABLE_W {
        BATMON_ALARM_INT_ENABLE_W { w: self }
    }
    #[doc = "Bits 1:3 - Channel number triggering the ADC interrupt"]
    #[inline(always)]
    pub fn adc_int_ch_num(&mut self) -> ADC_INT_CH_NUM_W {
        ADC_INT_CH_NUM_W { w: self }
    }
    #[doc = "Bit 0 - The ADC new sample ready interrupt mask"]
    #[inline(always)]
    pub fn adc_int_enable(&mut self) -> ADC_INT_ENABLE_W {
        ADC_INT_ENABLE_W { w: self }
    }
}
