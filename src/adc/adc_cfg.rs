#[doc = "Reader of register ADC_CFG"]
pub type R = crate::R<u32, super::ADC_CFG>;
#[doc = "Writer for register ADC_CFG"]
pub type W = crate::W<u32, super::ADC_CFG>;
#[doc = "Register ADC_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Duty cycling VDD divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUTY_DIVIDER_A {
    #[doc = "0: Normal mode: VBAT divider is always enabled"]
    ADC_VBAT_DIV2_NORMAL = 0,
    #[doc = "1: Duty cycling VBAT divider (enabled only during ADC conversion)"]
    ADC_VBAT_DIV2_DUTY = 1,
}
impl From<DUTY_DIVIDER_A> for bool {
    #[inline(always)]
    fn from(variant: DUTY_DIVIDER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DUTY_DIVIDER`"]
pub type DUTY_DIVIDER_R = crate::R<bool, DUTY_DIVIDER_A>;
impl DUTY_DIVIDER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUTY_DIVIDER_A {
        match self.bits {
            false => DUTY_DIVIDER_A::ADC_VBAT_DIV2_NORMAL,
            true => DUTY_DIVIDER_A::ADC_VBAT_DIV2_DUTY,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_VBAT_DIV2_NORMAL`"]
    #[inline(always)]
    pub fn is_adc_vbat_div2_normal(&self) -> bool {
        *self == DUTY_DIVIDER_A::ADC_VBAT_DIV2_NORMAL
    }
    #[doc = "Checks if the value of the field is `ADC_VBAT_DIV2_DUTY`"]
    #[inline(always)]
    pub fn is_adc_vbat_div2_duty(&self) -> bool {
        *self == DUTY_DIVIDER_A::ADC_VBAT_DIV2_DUTY
    }
}
#[doc = "Write proxy for field `DUTY_DIVIDER`"]
pub struct DUTY_DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_DIVIDER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DUTY_DIVIDER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal mode: VBAT divider is always enabled"]
    #[inline(always)]
    pub fn adc_vbat_div2_normal(self) -> &'a mut W {
        self.variant(DUTY_DIVIDER_A::ADC_VBAT_DIV2_NORMAL)
    }
    #[doc = "Duty cycling VBAT divider (enabled only during ADC conversion)"]
    #[inline(always)]
    pub fn adc_vbat_div2_duty(self) -> &'a mut W {
        self.variant(DUTY_DIVIDER_A::ADC_VBAT_DIV2_DUTY)
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
#[doc = "ADC continuously sampling the channel selected as interrupt source (ADC_INT_CH_NUM)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTINUOUS_MODE_A {
    #[doc = "0: Normal mode, all 8 channels sampled"]
    ADC_NORMAL = 0,
    #[doc = "1: Continuous mode: only one channel sampled (for test purpose)"]
    ADC_CONTINUOUS = 1,
}
impl From<CONTINUOUS_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: CONTINUOUS_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CONTINUOUS_MODE`"]
pub type CONTINUOUS_MODE_R = crate::R<bool, CONTINUOUS_MODE_A>;
impl CONTINUOUS_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONTINUOUS_MODE_A {
        match self.bits {
            false => CONTINUOUS_MODE_A::ADC_NORMAL,
            true => CONTINUOUS_MODE_A::ADC_CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_NORMAL`"]
    #[inline(always)]
    pub fn is_adc_normal(&self) -> bool {
        *self == CONTINUOUS_MODE_A::ADC_NORMAL
    }
    #[doc = "Checks if the value of the field is `ADC_CONTINUOUS`"]
    #[inline(always)]
    pub fn is_adc_continuous(&self) -> bool {
        *self == CONTINUOUS_MODE_A::ADC_CONTINUOUS
    }
}
#[doc = "Write proxy for field `CONTINUOUS_MODE`"]
pub struct CONTINUOUS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTINUOUS_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONTINUOUS_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal mode, all 8 channels sampled"]
    #[inline(always)]
    pub fn adc_normal(self) -> &'a mut W {
        self.variant(CONTINUOUS_MODE_A::ADC_NORMAL)
    }
    #[doc = "Continuous mode: only one channel sampled (for test purpose)"]
    #[inline(always)]
    pub fn adc_continuous(self) -> &'a mut W {
        self.variant(CONTINUOUS_MODE_A::ADC_CONTINUOUS)
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
#[doc = "Defines the sampling frequency of the ADC channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FREQ_A {
    #[doc = "0: ADC disabled"]
    ADC_DISABLE = 0,
    #[doc = "1: Sample rate is SLOWCLK/200 (low frequency mode)"]
    ADC_PRESCALE_200 = 1,
    #[doc = "2: Sample rate is SLOWCLK/400 (low frequency mode)"]
    ADC_PRESCALE_400 = 2,
    #[doc = "3: Sample rate is SLOWCLK/640 (low frequency mode)"]
    ADC_PRESCALE_640 = 3,
    #[doc = "4: Sample rate is SLOWCLK/800 (low frequency mode)"]
    ADC_PRESCALE_800 = 4,
    #[doc = "5: Sample rate is SLOWCLK/1600 (low frequency mode)"]
    ADC_PRESCALE_1600 = 5,
    #[doc = "6: Sample rate is SLOWCLK/3200 (low frequency mode)"]
    ADC_PRESCALE_3200 = 6,
    #[doc = "7: Sample rate is SLOWCLK/6400 (low frequency mode)"]
    ADC_PRESCALE_6400 = 7,
    #[doc = "8: Sample rate is SLOWCLK/20 (high frequency mode)"]
    ADC_PRESCALE_20H = 8,
    #[doc = "9: Sample rate is SLOWCLK/40 (high frequency mode)"]
    ADC_PRESCALE_40H = 9,
    #[doc = "10: Sample rate is SLOWCLK/80 (high frequency mode)"]
    ADC_PRESCALE_80H = 10,
    #[doc = "11: Sample rate is SLOWCLK/128 (high frequency mode)"]
    ADC_PRESCALE_128H = 11,
    #[doc = "12: Sample rate is SLOWCLK/160 (high frequency mode)"]
    ADC_PRESCALE_160H = 12,
    #[doc = "13: Sample rate is SLOWCLK/320 (high frequency mode)"]
    ADC_PRESCALE_320H = 13,
    #[doc = "14: Sample rate is SLOWCLK/640 (high frequency mode)"]
    ADC_PRESCALE_640H = 14,
    #[doc = "15: Sample rate is SLOWCLK/1280 (high frequency mode)"]
    ADC_PRESCALE_1280H = 15,
}
impl From<FREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: FREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FREQ`"]
pub type FREQ_R = crate::R<u8, FREQ_A>;
impl FREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQ_A {
        match self.bits {
            0 => FREQ_A::ADC_DISABLE,
            1 => FREQ_A::ADC_PRESCALE_200,
            2 => FREQ_A::ADC_PRESCALE_400,
            3 => FREQ_A::ADC_PRESCALE_640,
            4 => FREQ_A::ADC_PRESCALE_800,
            5 => FREQ_A::ADC_PRESCALE_1600,
            6 => FREQ_A::ADC_PRESCALE_3200,
            7 => FREQ_A::ADC_PRESCALE_6400,
            8 => FREQ_A::ADC_PRESCALE_20H,
            9 => FREQ_A::ADC_PRESCALE_40H,
            10 => FREQ_A::ADC_PRESCALE_80H,
            11 => FREQ_A::ADC_PRESCALE_128H,
            12 => FREQ_A::ADC_PRESCALE_160H,
            13 => FREQ_A::ADC_PRESCALE_320H,
            14 => FREQ_A::ADC_PRESCALE_640H,
            15 => FREQ_A::ADC_PRESCALE_1280H,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_DISABLE`"]
    #[inline(always)]
    pub fn is_adc_disable(&self) -> bool {
        *self == FREQ_A::ADC_DISABLE
    }
    #[doc = "Checks if the value of the field is `ADC_PRESCALE_200`"]
    #[inline(always)]
    pub fn is_adc_prescale_200(&self) -> bool {
        *self == FREQ_A::ADC_PRESCALE_200
    }
    #[doc = "Checks if the value of the field is `ADC_PRESCALE_400`"]
    #[inline(always)]
    pub fn is_adc_prescale_400(&self) -> bool {
        *self == FREQ_A::ADC_PRESCALE_400
    }
    #[doc = "Checks if the value of the field is `ADC_PRESCALE_640`"]
    #[inline(always)]
    pub fn is_adc_prescale_640(&self) -> bool {
        *self == FREQ_A::ADC_PRESCALE_640
    }
    #[doc = "Checks if the value of the field is `ADC_PRESCALE_800`"]
    #[inline(always)]
    pub fn is_adc_prescale_800(&self) -> bool {
        *self == FREQ_A::ADC_PRESCALE_800
    }
    #[doc = "Checks if the value of the field is `ADC_PRESCALE_1600`"]
    #[inline(always)]
    pub fn is_adc_prescale_1600(&self) -> bool {
        *self == FREQ_A::ADC_PRESCALE_1600
    }
    #[doc = "Checks if the value of the field is `ADC_PRESCALE_3200`"]
    #[inline(always)]
    pub fn is_adc_prescale_3200(&self) -> bool {
        *self == FREQ_A::ADC_PRESCALE_3200
    }
    #[doc = "Checks if the value of the field is `ADC_PRESCALE_6400`"]
    #[inline(always)]
    pub fn is_adc_prescale_6400(&self) -> bool {
        *self == FREQ_A::ADC_PRESCALE_6400
    }
    #[doc = "Checks if the value of the field is `ADC_PRESCALE_20H`"]
    #[inline(always)]
    pub fn is_adc_prescale_20h(&self) -> bool {
        *self == FREQ_A::ADC_PRESCALE_20H
    }
    #[doc = "Checks if the value of the field is `ADC_PRESCALE_40H`"]
    #[inline(always)]
    pub fn is_adc_prescale_40h(&self) -> bool {
        *self == FREQ_A::ADC_PRESCALE_40H
    }
    #[doc = "Checks if the value of the field is `ADC_PRESCALE_80H`"]
    #[inline(always)]
    pub fn is_adc_prescale_80h(&self) -> bool {
        *self == FREQ_A::ADC_PRESCALE_80H
    }
    #[doc = "Checks if the value of the field is `ADC_PRESCALE_128H`"]
    #[inline(always)]
    pub fn is_adc_prescale_128h(&self) -> bool {
        *self == FREQ_A::ADC_PRESCALE_128H
    }
    #[doc = "Checks if the value of the field is `ADC_PRESCALE_160H`"]
    #[inline(always)]
    pub fn is_adc_prescale_160h(&self) -> bool {
        *self == FREQ_A::ADC_PRESCALE_160H
    }
    #[doc = "Checks if the value of the field is `ADC_PRESCALE_320H`"]
    #[inline(always)]
    pub fn is_adc_prescale_320h(&self) -> bool {
        *self == FREQ_A::ADC_PRESCALE_320H
    }
    #[doc = "Checks if the value of the field is `ADC_PRESCALE_640H`"]
    #[inline(always)]
    pub fn is_adc_prescale_640h(&self) -> bool {
        *self == FREQ_A::ADC_PRESCALE_640H
    }
    #[doc = "Checks if the value of the field is `ADC_PRESCALE_1280H`"]
    #[inline(always)]
    pub fn is_adc_prescale_1280h(&self) -> bool {
        *self == FREQ_A::ADC_PRESCALE_1280H
    }
}
#[doc = "Write proxy for field `FREQ`"]
pub struct FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC disabled"]
    #[inline(always)]
    pub fn adc_disable(self) -> &'a mut W {
        self.variant(FREQ_A::ADC_DISABLE)
    }
    #[doc = "Sample rate is SLOWCLK/200 (low frequency mode)"]
    #[inline(always)]
    pub fn adc_prescale_200(self) -> &'a mut W {
        self.variant(FREQ_A::ADC_PRESCALE_200)
    }
    #[doc = "Sample rate is SLOWCLK/400 (low frequency mode)"]
    #[inline(always)]
    pub fn adc_prescale_400(self) -> &'a mut W {
        self.variant(FREQ_A::ADC_PRESCALE_400)
    }
    #[doc = "Sample rate is SLOWCLK/640 (low frequency mode)"]
    #[inline(always)]
    pub fn adc_prescale_640(self) -> &'a mut W {
        self.variant(FREQ_A::ADC_PRESCALE_640)
    }
    #[doc = "Sample rate is SLOWCLK/800 (low frequency mode)"]
    #[inline(always)]
    pub fn adc_prescale_800(self) -> &'a mut W {
        self.variant(FREQ_A::ADC_PRESCALE_800)
    }
    #[doc = "Sample rate is SLOWCLK/1600 (low frequency mode)"]
    #[inline(always)]
    pub fn adc_prescale_1600(self) -> &'a mut W {
        self.variant(FREQ_A::ADC_PRESCALE_1600)
    }
    #[doc = "Sample rate is SLOWCLK/3200 (low frequency mode)"]
    #[inline(always)]
    pub fn adc_prescale_3200(self) -> &'a mut W {
        self.variant(FREQ_A::ADC_PRESCALE_3200)
    }
    #[doc = "Sample rate is SLOWCLK/6400 (low frequency mode)"]
    #[inline(always)]
    pub fn adc_prescale_6400(self) -> &'a mut W {
        self.variant(FREQ_A::ADC_PRESCALE_6400)
    }
    #[doc = "Sample rate is SLOWCLK/20 (high frequency mode)"]
    #[inline(always)]
    pub fn adc_prescale_20h(self) -> &'a mut W {
        self.variant(FREQ_A::ADC_PRESCALE_20H)
    }
    #[doc = "Sample rate is SLOWCLK/40 (high frequency mode)"]
    #[inline(always)]
    pub fn adc_prescale_40h(self) -> &'a mut W {
        self.variant(FREQ_A::ADC_PRESCALE_40H)
    }
    #[doc = "Sample rate is SLOWCLK/80 (high frequency mode)"]
    #[inline(always)]
    pub fn adc_prescale_80h(self) -> &'a mut W {
        self.variant(FREQ_A::ADC_PRESCALE_80H)
    }
    #[doc = "Sample rate is SLOWCLK/128 (high frequency mode)"]
    #[inline(always)]
    pub fn adc_prescale_128h(self) -> &'a mut W {
        self.variant(FREQ_A::ADC_PRESCALE_128H)
    }
    #[doc = "Sample rate is SLOWCLK/160 (high frequency mode)"]
    #[inline(always)]
    pub fn adc_prescale_160h(self) -> &'a mut W {
        self.variant(FREQ_A::ADC_PRESCALE_160H)
    }
    #[doc = "Sample rate is SLOWCLK/320 (high frequency mode)"]
    #[inline(always)]
    pub fn adc_prescale_320h(self) -> &'a mut W {
        self.variant(FREQ_A::ADC_PRESCALE_320H)
    }
    #[doc = "Sample rate is SLOWCLK/640 (high frequency mode)"]
    #[inline(always)]
    pub fn adc_prescale_640h(self) -> &'a mut W {
        self.variant(FREQ_A::ADC_PRESCALE_640H)
    }
    #[doc = "Sample rate is SLOWCLK/1280 (high frequency mode)"]
    #[inline(always)]
    pub fn adc_prescale_1280h(self) -> &'a mut W {
        self.variant(FREQ_A::ADC_PRESCALE_1280H)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Duty cycling VDD divider"]
    #[inline(always)]
    pub fn duty_divider(&self) -> DUTY_DIVIDER_R {
        DUTY_DIVIDER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC continuously sampling the channel selected as interrupt source (ADC_INT_CH_NUM)"]
    #[inline(always)]
    pub fn continuous_mode(&self) -> CONTINUOUS_MODE_R {
        CONTINUOUS_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - Defines the sampling frequency of the ADC channels"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - Duty cycling VDD divider"]
    #[inline(always)]
    pub fn duty_divider(&mut self) -> DUTY_DIVIDER_W {
        DUTY_DIVIDER_W { w: self }
    }
    #[doc = "Bit 4 - ADC continuously sampling the channel selected as interrupt source (ADC_INT_CH_NUM)"]
    #[inline(always)]
    pub fn continuous_mode(&mut self) -> CONTINUOUS_MODE_W {
        CONTINUOUS_MODE_W { w: self }
    }
    #[doc = "Bits 0:3 - Defines the sampling frequency of the ADC channels"]
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W {
        FREQ_W { w: self }
    }
}
