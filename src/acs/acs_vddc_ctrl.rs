#[doc = "Reader of register ACS_VDDC_CTRL"]
pub type R = crate::R<u32, super::ACS_VDDC_CTRL>;
#[doc = "Writer for register ACS_VDDC_CTRL"]
pub type W = crate::W<u32, super::ACS_VDDC_CTRL>;
#[doc = "Register ACS_VDDC_CTRL `reset()`'s with value 0x0023_0023"]
impl crate::ResetValue for super::ACS_VDDC_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0023_0023
    }
}
#[doc = "VDDC standby voltage trimming (10 mV steps)\n\nValue on reset: 35"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STANDBY_VTRIM_A {
    #[doc = "0: 0.75 V"]
    VDDC_STANDBY_TRIM_0P75V = 0,
    #[doc = "1: 0.76 V"]
    VDDC_STANDBY_TRIM_0P76V = 1,
    #[doc = "25: 1.0 V"]
    VDDC_STANDBY_TRIM_1P00V = 25,
    #[doc = "33: 1.08 V"]
    VDDC_STANDBY_TRIM_1P08V = 33,
    #[doc = "35: 1.1 V"]
    VDDC_STANDBY_TRIM_1P10V = 35,
    #[doc = "40: 1.15 V"]
    VDDC_STANDBY_TRIM_1P15V = 40,
    #[doc = "45: 1.2 V"]
    VDDC_STANDBY_TRIM_1P20V = 45,
    #[doc = "50: 1.25 V"]
    VDDC_STANDBY_TRIM_1P25V = 50,
    #[doc = "57: 1.32 V"]
    VDDC_STANDBY_TRIM_1P32V = 57,
    #[doc = "63: 1.38 V"]
    VDDC_STANDBY_TRIM_1P38V = 63,
}
impl From<STANDBY_VTRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: STANDBY_VTRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STANDBY_VTRIM`"]
pub type STANDBY_VTRIM_R = crate::R<u8, STANDBY_VTRIM_A>;
impl STANDBY_VTRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STANDBY_VTRIM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_0P75V),
            1 => Val(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_0P76V),
            25 => Val(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P00V),
            33 => Val(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P08V),
            35 => Val(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P10V),
            40 => Val(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P15V),
            45 => Val(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P20V),
            50 => Val(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P25V),
            57 => Val(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P32V),
            63 => Val(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P38V),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VDDC_STANDBY_TRIM_0P75V`"]
    #[inline(always)]
    pub fn is_vddc_standby_trim_0p75v(&self) -> bool {
        *self == STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_0P75V
    }
    #[doc = "Checks if the value of the field is `VDDC_STANDBY_TRIM_0P76V`"]
    #[inline(always)]
    pub fn is_vddc_standby_trim_0p76v(&self) -> bool {
        *self == STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_0P76V
    }
    #[doc = "Checks if the value of the field is `VDDC_STANDBY_TRIM_1P00V`"]
    #[inline(always)]
    pub fn is_vddc_standby_trim_1p00v(&self) -> bool {
        *self == STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P00V
    }
    #[doc = "Checks if the value of the field is `VDDC_STANDBY_TRIM_1P08V`"]
    #[inline(always)]
    pub fn is_vddc_standby_trim_1p08v(&self) -> bool {
        *self == STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P08V
    }
    #[doc = "Checks if the value of the field is `VDDC_STANDBY_TRIM_1P10V`"]
    #[inline(always)]
    pub fn is_vddc_standby_trim_1p10v(&self) -> bool {
        *self == STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P10V
    }
    #[doc = "Checks if the value of the field is `VDDC_STANDBY_TRIM_1P15V`"]
    #[inline(always)]
    pub fn is_vddc_standby_trim_1p15v(&self) -> bool {
        *self == STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P15V
    }
    #[doc = "Checks if the value of the field is `VDDC_STANDBY_TRIM_1P20V`"]
    #[inline(always)]
    pub fn is_vddc_standby_trim_1p20v(&self) -> bool {
        *self == STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P20V
    }
    #[doc = "Checks if the value of the field is `VDDC_STANDBY_TRIM_1P25V`"]
    #[inline(always)]
    pub fn is_vddc_standby_trim_1p25v(&self) -> bool {
        *self == STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P25V
    }
    #[doc = "Checks if the value of the field is `VDDC_STANDBY_TRIM_1P32V`"]
    #[inline(always)]
    pub fn is_vddc_standby_trim_1p32v(&self) -> bool {
        *self == STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P32V
    }
    #[doc = "Checks if the value of the field is `VDDC_STANDBY_TRIM_1P38V`"]
    #[inline(always)]
    pub fn is_vddc_standby_trim_1p38v(&self) -> bool {
        *self == STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P38V
    }
}
#[doc = "Write proxy for field `STANDBY_VTRIM`"]
pub struct STANDBY_VTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_VTRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STANDBY_VTRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0.75 V"]
    #[inline(always)]
    pub fn vddc_standby_trim_0p75v(self) -> &'a mut W {
        self.variant(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_0P75V)
    }
    #[doc = "0.76 V"]
    #[inline(always)]
    pub fn vddc_standby_trim_0p76v(self) -> &'a mut W {
        self.variant(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_0P76V)
    }
    #[doc = "1.0 V"]
    #[inline(always)]
    pub fn vddc_standby_trim_1p00v(self) -> &'a mut W {
        self.variant(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P00V)
    }
    #[doc = "1.08 V"]
    #[inline(always)]
    pub fn vddc_standby_trim_1p08v(self) -> &'a mut W {
        self.variant(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P08V)
    }
    #[doc = "1.1 V"]
    #[inline(always)]
    pub fn vddc_standby_trim_1p10v(self) -> &'a mut W {
        self.variant(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P10V)
    }
    #[doc = "1.15 V"]
    #[inline(always)]
    pub fn vddc_standby_trim_1p15v(self) -> &'a mut W {
        self.variant(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P15V)
    }
    #[doc = "1.2 V"]
    #[inline(always)]
    pub fn vddc_standby_trim_1p20v(self) -> &'a mut W {
        self.variant(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P20V)
    }
    #[doc = "1.25 V"]
    #[inline(always)]
    pub fn vddc_standby_trim_1p25v(self) -> &'a mut W {
        self.variant(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P25V)
    }
    #[doc = "1.32 V"]
    #[inline(always)]
    pub fn vddc_standby_trim_1p32v(self) -> &'a mut W {
        self.variant(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P32V)
    }
    #[doc = "1.38 V"]
    #[inline(always)]
    pub fn vddc_standby_trim_1p38v(self) -> &'a mut W {
        self.variant(STANDBY_VTRIM_A::VDDC_STANDBY_TRIM_1P38V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Low power mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_LOW_BIAS_A {
    #[doc = "0: Nominal regulator biasing"]
    VDDC_NOMINAL_BIAS = 0,
    #[doc = "1: Low regulator biasing"]
    VDDC_LOW_BIAS = 1,
}
impl From<ENABLE_LOW_BIAS_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_LOW_BIAS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLE_LOW_BIAS`"]
pub type ENABLE_LOW_BIAS_R = crate::R<bool, ENABLE_LOW_BIAS_A>;
impl ENABLE_LOW_BIAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_LOW_BIAS_A {
        match self.bits {
            false => ENABLE_LOW_BIAS_A::VDDC_NOMINAL_BIAS,
            true => ENABLE_LOW_BIAS_A::VDDC_LOW_BIAS,
        }
    }
    #[doc = "Checks if the value of the field is `VDDC_NOMINAL_BIAS`"]
    #[inline(always)]
    pub fn is_vddc_nominal_bias(&self) -> bool {
        *self == ENABLE_LOW_BIAS_A::VDDC_NOMINAL_BIAS
    }
    #[doc = "Checks if the value of the field is `VDDC_LOW_BIAS`"]
    #[inline(always)]
    pub fn is_vddc_low_bias(&self) -> bool {
        *self == ENABLE_LOW_BIAS_A::VDDC_LOW_BIAS
    }
}
#[doc = "Write proxy for field `ENABLE_LOW_BIAS`"]
pub struct ENABLE_LOW_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_LOW_BIAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_LOW_BIAS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Nominal regulator biasing"]
    #[inline(always)]
    pub fn vddc_nominal_bias(self) -> &'a mut W {
        self.variant(ENABLE_LOW_BIAS_A::VDDC_NOMINAL_BIAS)
    }
    #[doc = "Low regulator biasing"]
    #[inline(always)]
    pub fn vddc_low_bias(self) -> &'a mut W {
        self.variant(ENABLE_LOW_BIAS_A::VDDC_LOW_BIAS)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Sleep mode clamp control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEP_CLAMP_A {
    #[doc = "0: Set the output HIZ (floating) in sleep mode"]
    VDDC_SLEEP_HIZ = 0,
    #[doc = "1: Clamp output to ground in sleep mode"]
    VDDC_SLEEP_GND = 1,
}
impl From<SLEEP_CLAMP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_CLAMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLEEP_CLAMP`"]
pub type SLEEP_CLAMP_R = crate::R<bool, SLEEP_CLAMP_A>;
impl SLEEP_CLAMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_CLAMP_A {
        match self.bits {
            false => SLEEP_CLAMP_A::VDDC_SLEEP_HIZ,
            true => SLEEP_CLAMP_A::VDDC_SLEEP_GND,
        }
    }
    #[doc = "Checks if the value of the field is `VDDC_SLEEP_HIZ`"]
    #[inline(always)]
    pub fn is_vddc_sleep_hiz(&self) -> bool {
        *self == SLEEP_CLAMP_A::VDDC_SLEEP_HIZ
    }
    #[doc = "Checks if the value of the field is `VDDC_SLEEP_GND`"]
    #[inline(always)]
    pub fn is_vddc_sleep_gnd(&self) -> bool {
        *self == SLEEP_CLAMP_A::VDDC_SLEEP_GND
    }
}
#[doc = "Write proxy for field `SLEEP_CLAMP`"]
pub struct SLEEP_CLAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_CLAMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEP_CLAMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set the output HIZ (floating) in sleep mode"]
    #[inline(always)]
    pub fn vddc_sleep_hiz(self) -> &'a mut W {
        self.variant(SLEEP_CLAMP_A::VDDC_SLEEP_HIZ)
    }
    #[doc = "Clamp output to ground in sleep mode"]
    #[inline(always)]
    pub fn vddc_sleep_gnd(self) -> &'a mut W {
        self.variant(SLEEP_CLAMP_A::VDDC_SLEEP_GND)
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
#[doc = "Output voltage trimming configuration in 10 mV steps\n\nValue on reset: 35"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VTRIM_A {
    #[doc = "0: 0.75 V"]
    VDDC_TRIM_0P75V = 0,
    #[doc = "1: 0.76 V"]
    VDDC_TRIM_0P76V = 1,
    #[doc = "25: 1.0 V"]
    VDDC_TRIM_1P00V = 25,
    #[doc = "33: 1.08 V"]
    VDDC_TRIM_1P08V = 33,
    #[doc = "35: 1.1 V"]
    VDDC_TRIM_1P10V = 35,
    #[doc = "40: 1.15 V"]
    VDDC_TRIM_1P15V = 40,
    #[doc = "45: 1.2 V"]
    VDDC_TRIM_1P20V = 45,
    #[doc = "50: 1.25 V"]
    VDDC_TRIM_1P25V = 50,
    #[doc = "57: 1.32 V"]
    VDDC_TRIM_1P32V = 57,
    #[doc = "63: 1.38 V"]
    VDDC_TRIM_1P38V = 63,
}
impl From<VTRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: VTRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VTRIM`"]
pub type VTRIM_R = crate::R<u8, VTRIM_A>;
impl VTRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VTRIM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VTRIM_A::VDDC_TRIM_0P75V),
            1 => Val(VTRIM_A::VDDC_TRIM_0P76V),
            25 => Val(VTRIM_A::VDDC_TRIM_1P00V),
            33 => Val(VTRIM_A::VDDC_TRIM_1P08V),
            35 => Val(VTRIM_A::VDDC_TRIM_1P10V),
            40 => Val(VTRIM_A::VDDC_TRIM_1P15V),
            45 => Val(VTRIM_A::VDDC_TRIM_1P20V),
            50 => Val(VTRIM_A::VDDC_TRIM_1P25V),
            57 => Val(VTRIM_A::VDDC_TRIM_1P32V),
            63 => Val(VTRIM_A::VDDC_TRIM_1P38V),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VDDC_TRIM_0P75V`"]
    #[inline(always)]
    pub fn is_vddc_trim_0p75v(&self) -> bool {
        *self == VTRIM_A::VDDC_TRIM_0P75V
    }
    #[doc = "Checks if the value of the field is `VDDC_TRIM_0P76V`"]
    #[inline(always)]
    pub fn is_vddc_trim_0p76v(&self) -> bool {
        *self == VTRIM_A::VDDC_TRIM_0P76V
    }
    #[doc = "Checks if the value of the field is `VDDC_TRIM_1P00V`"]
    #[inline(always)]
    pub fn is_vddc_trim_1p00v(&self) -> bool {
        *self == VTRIM_A::VDDC_TRIM_1P00V
    }
    #[doc = "Checks if the value of the field is `VDDC_TRIM_1P08V`"]
    #[inline(always)]
    pub fn is_vddc_trim_1p08v(&self) -> bool {
        *self == VTRIM_A::VDDC_TRIM_1P08V
    }
    #[doc = "Checks if the value of the field is `VDDC_TRIM_1P10V`"]
    #[inline(always)]
    pub fn is_vddc_trim_1p10v(&self) -> bool {
        *self == VTRIM_A::VDDC_TRIM_1P10V
    }
    #[doc = "Checks if the value of the field is `VDDC_TRIM_1P15V`"]
    #[inline(always)]
    pub fn is_vddc_trim_1p15v(&self) -> bool {
        *self == VTRIM_A::VDDC_TRIM_1P15V
    }
    #[doc = "Checks if the value of the field is `VDDC_TRIM_1P20V`"]
    #[inline(always)]
    pub fn is_vddc_trim_1p20v(&self) -> bool {
        *self == VTRIM_A::VDDC_TRIM_1P20V
    }
    #[doc = "Checks if the value of the field is `VDDC_TRIM_1P25V`"]
    #[inline(always)]
    pub fn is_vddc_trim_1p25v(&self) -> bool {
        *self == VTRIM_A::VDDC_TRIM_1P25V
    }
    #[doc = "Checks if the value of the field is `VDDC_TRIM_1P32V`"]
    #[inline(always)]
    pub fn is_vddc_trim_1p32v(&self) -> bool {
        *self == VTRIM_A::VDDC_TRIM_1P32V
    }
    #[doc = "Checks if the value of the field is `VDDC_TRIM_1P38V`"]
    #[inline(always)]
    pub fn is_vddc_trim_1p38v(&self) -> bool {
        *self == VTRIM_A::VDDC_TRIM_1P38V
    }
}
#[doc = "Write proxy for field `VTRIM`"]
pub struct VTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VTRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0.75 V"]
    #[inline(always)]
    pub fn vddc_trim_0p75v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDC_TRIM_0P75V)
    }
    #[doc = "0.76 V"]
    #[inline(always)]
    pub fn vddc_trim_0p76v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDC_TRIM_0P76V)
    }
    #[doc = "1.0 V"]
    #[inline(always)]
    pub fn vddc_trim_1p00v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDC_TRIM_1P00V)
    }
    #[doc = "1.08 V"]
    #[inline(always)]
    pub fn vddc_trim_1p08v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDC_TRIM_1P08V)
    }
    #[doc = "1.1 V"]
    #[inline(always)]
    pub fn vddc_trim_1p10v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDC_TRIM_1P10V)
    }
    #[doc = "1.15 V"]
    #[inline(always)]
    pub fn vddc_trim_1p15v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDC_TRIM_1P15V)
    }
    #[doc = "1.2 V"]
    #[inline(always)]
    pub fn vddc_trim_1p20v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDC_TRIM_1P20V)
    }
    #[doc = "1.25 V"]
    #[inline(always)]
    pub fn vddc_trim_1p25v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDC_TRIM_1P25V)
    }
    #[doc = "1.32 V"]
    #[inline(always)]
    pub fn vddc_trim_1p32v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDC_TRIM_1P32V)
    }
    #[doc = "1.38 V"]
    #[inline(always)]
    pub fn vddc_trim_1p38v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDC_TRIM_1P38V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:21 - VDDC standby voltage trimming (10 mV steps)"]
    #[inline(always)]
    pub fn standby_vtrim(&self) -> STANDBY_VTRIM_R {
        STANDBY_VTRIM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 13 - Low power mode control"]
    #[inline(always)]
    pub fn enable_low_bias(&self) -> ENABLE_LOW_BIAS_R {
        ENABLE_LOW_BIAS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Sleep mode clamp control"]
    #[inline(always)]
    pub fn sleep_clamp(&self) -> SLEEP_CLAMP_R {
        SLEEP_CLAMP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - Output voltage trimming configuration in 10 mV steps"]
    #[inline(always)]
    pub fn vtrim(&self) -> VTRIM_R {
        VTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:21 - VDDC standby voltage trimming (10 mV steps)"]
    #[inline(always)]
    pub fn standby_vtrim(&mut self) -> STANDBY_VTRIM_W {
        STANDBY_VTRIM_W { w: self }
    }
    #[doc = "Bit 13 - Low power mode control"]
    #[inline(always)]
    pub fn enable_low_bias(&mut self) -> ENABLE_LOW_BIAS_W {
        ENABLE_LOW_BIAS_W { w: self }
    }
    #[doc = "Bit 12 - Sleep mode clamp control"]
    #[inline(always)]
    pub fn sleep_clamp(&mut self) -> SLEEP_CLAMP_W {
        SLEEP_CLAMP_W { w: self }
    }
    #[doc = "Bits 0:5 - Output voltage trimming configuration in 10 mV steps"]
    #[inline(always)]
    pub fn vtrim(&mut self) -> VTRIM_W {
        VTRIM_W { w: self }
    }
}
