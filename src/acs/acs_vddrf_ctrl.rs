#[doc = "Reader of register ACS_VDDRF_CTRL"]
pub type R = crate::R<u32, super::ACS_VDDRF_CTRL>;
#[doc = "Writer for register ACS_VDDRF_CTRL"]
pub type W = crate::W<u32, super::ACS_VDDRF_CTRL>;
#[doc = "Register ACS_VDDRF_CTRL `reset()`'s with value 0x23"]
impl crate::ResetValue for super::ACS_VDDRF_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x23
    }
}
#[doc = "Supply ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_A {
    #[doc = "0: Supply voltage not ready"]
    VDDRF_NOT_READY = 0,
    #[doc = "1: Supply voltage ready"]
    VDDRF_READY = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READY`"]
pub type READY_R = crate::R<bool, READY_A>;
impl READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::VDDRF_NOT_READY,
            true => READY_A::VDDRF_READY,
        }
    }
    #[doc = "Checks if the value of the field is `VDDRF_NOT_READY`"]
    #[inline(always)]
    pub fn is_vddrf_not_ready(&self) -> bool {
        *self == READY_A::VDDRF_NOT_READY
    }
    #[doc = "Checks if the value of the field is `VDDRF_READY`"]
    #[inline(always)]
    pub fn is_vddrf_ready(&self) -> bool {
        *self == READY_A::VDDRF_READY
    }
}
#[doc = "Disable mode clamp control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLAMP_A {
    #[doc = "0: Set the output HIZ (floating) in disable mode"]
    VDDRF_DISABLE_HIZ = 0,
    #[doc = "1: Clamp output to ground in disable mode"]
    VDDRF_DISABLE_GND = 1,
}
impl From<CLAMP_A> for bool {
    #[inline(always)]
    fn from(variant: CLAMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLAMP`"]
pub type CLAMP_R = crate::R<bool, CLAMP_A>;
impl CLAMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLAMP_A {
        match self.bits {
            false => CLAMP_A::VDDRF_DISABLE_HIZ,
            true => CLAMP_A::VDDRF_DISABLE_GND,
        }
    }
    #[doc = "Checks if the value of the field is `VDDRF_DISABLE_HIZ`"]
    #[inline(always)]
    pub fn is_vddrf_disable_hiz(&self) -> bool {
        *self == CLAMP_A::VDDRF_DISABLE_HIZ
    }
    #[doc = "Checks if the value of the field is `VDDRF_DISABLE_GND`"]
    #[inline(always)]
    pub fn is_vddrf_disable_gnd(&self) -> bool {
        *self == CLAMP_A::VDDRF_DISABLE_GND
    }
}
#[doc = "Write proxy for field `CLAMP`"]
pub struct CLAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CLAMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLAMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set the output HIZ (floating) in disable mode"]
    #[inline(always)]
    pub fn vddrf_disable_hiz(self) -> &'a mut W {
        self.variant(CLAMP_A::VDDRF_DISABLE_HIZ)
    }
    #[doc = "Clamp output to ground in disable mode"]
    #[inline(always)]
    pub fn vddrf_disable_gnd(self) -> &'a mut W {
        self.variant(CLAMP_A::VDDRF_DISABLE_GND)
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
#[doc = "Enable control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Disable the VDDRF regulator"]
    VDDRF_DISABLE = 0,
    #[doc = "1: Enable the VDDRF regulator"]
    VDDRF_ENABLE = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::VDDRF_DISABLE,
            true => ENABLE_A::VDDRF_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `VDDRF_DISABLE`"]
    #[inline(always)]
    pub fn is_vddrf_disable(&self) -> bool {
        *self == ENABLE_A::VDDRF_DISABLE
    }
    #[doc = "Checks if the value of the field is `VDDRF_ENABLE`"]
    #[inline(always)]
    pub fn is_vddrf_enable(&self) -> bool {
        *self == ENABLE_A::VDDRF_ENABLE
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the VDDRF regulator"]
    #[inline(always)]
    pub fn vddrf_disable(self) -> &'a mut W {
        self.variant(ENABLE_A::VDDRF_DISABLE)
    }
    #[doc = "Enable the VDDRF regulator"]
    #[inline(always)]
    pub fn vddrf_enable(self) -> &'a mut W {
        self.variant(ENABLE_A::VDDRF_ENABLE)
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
#[doc = "Output voltage trimming configuration in 10 mV steps\n\nValue on reset: 35"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VTRIM_A {
    #[doc = "0: 0.75 V"]
    VDDRF_TRIM_0P75V = 0,
    #[doc = "1: 0.76 V"]
    VDDRF_TRIM_0P76V = 1,
    #[doc = "25: 1.0 V"]
    VDDRF_TRIM_1P00V = 25,
    #[doc = "33: 1.08 V"]
    VDDRF_TRIM_1P08V = 33,
    #[doc = "35: 1.1 V"]
    VDDRF_TRIM_1P10V = 35,
    #[doc = "40: 1.15 V"]
    VDDRF_TRIM_1P15V = 40,
    #[doc = "45: 1.2 V"]
    VDDRF_TRIM_1P20V = 45,
    #[doc = "50: 1.25 V"]
    VDDRF_TRIM_1P25V = 50,
    #[doc = "57: 1.32 V"]
    VDDRF_TRIM_1P32V = 57,
    #[doc = "63: 1.38 V"]
    VDDRF_TRIM_1P38V = 63,
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
            0 => Val(VTRIM_A::VDDRF_TRIM_0P75V),
            1 => Val(VTRIM_A::VDDRF_TRIM_0P76V),
            25 => Val(VTRIM_A::VDDRF_TRIM_1P00V),
            33 => Val(VTRIM_A::VDDRF_TRIM_1P08V),
            35 => Val(VTRIM_A::VDDRF_TRIM_1P10V),
            40 => Val(VTRIM_A::VDDRF_TRIM_1P15V),
            45 => Val(VTRIM_A::VDDRF_TRIM_1P20V),
            50 => Val(VTRIM_A::VDDRF_TRIM_1P25V),
            57 => Val(VTRIM_A::VDDRF_TRIM_1P32V),
            63 => Val(VTRIM_A::VDDRF_TRIM_1P38V),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VDDRF_TRIM_0P75V`"]
    #[inline(always)]
    pub fn is_vddrf_trim_0p75v(&self) -> bool {
        *self == VTRIM_A::VDDRF_TRIM_0P75V
    }
    #[doc = "Checks if the value of the field is `VDDRF_TRIM_0P76V`"]
    #[inline(always)]
    pub fn is_vddrf_trim_0p76v(&self) -> bool {
        *self == VTRIM_A::VDDRF_TRIM_0P76V
    }
    #[doc = "Checks if the value of the field is `VDDRF_TRIM_1P00V`"]
    #[inline(always)]
    pub fn is_vddrf_trim_1p00v(&self) -> bool {
        *self == VTRIM_A::VDDRF_TRIM_1P00V
    }
    #[doc = "Checks if the value of the field is `VDDRF_TRIM_1P08V`"]
    #[inline(always)]
    pub fn is_vddrf_trim_1p08v(&self) -> bool {
        *self == VTRIM_A::VDDRF_TRIM_1P08V
    }
    #[doc = "Checks if the value of the field is `VDDRF_TRIM_1P10V`"]
    #[inline(always)]
    pub fn is_vddrf_trim_1p10v(&self) -> bool {
        *self == VTRIM_A::VDDRF_TRIM_1P10V
    }
    #[doc = "Checks if the value of the field is `VDDRF_TRIM_1P15V`"]
    #[inline(always)]
    pub fn is_vddrf_trim_1p15v(&self) -> bool {
        *self == VTRIM_A::VDDRF_TRIM_1P15V
    }
    #[doc = "Checks if the value of the field is `VDDRF_TRIM_1P20V`"]
    #[inline(always)]
    pub fn is_vddrf_trim_1p20v(&self) -> bool {
        *self == VTRIM_A::VDDRF_TRIM_1P20V
    }
    #[doc = "Checks if the value of the field is `VDDRF_TRIM_1P25V`"]
    #[inline(always)]
    pub fn is_vddrf_trim_1p25v(&self) -> bool {
        *self == VTRIM_A::VDDRF_TRIM_1P25V
    }
    #[doc = "Checks if the value of the field is `VDDRF_TRIM_1P32V`"]
    #[inline(always)]
    pub fn is_vddrf_trim_1p32v(&self) -> bool {
        *self == VTRIM_A::VDDRF_TRIM_1P32V
    }
    #[doc = "Checks if the value of the field is `VDDRF_TRIM_1P38V`"]
    #[inline(always)]
    pub fn is_vddrf_trim_1p38v(&self) -> bool {
        *self == VTRIM_A::VDDRF_TRIM_1P38V
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
    pub fn vddrf_trim_0p75v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDRF_TRIM_0P75V)
    }
    #[doc = "0.76 V"]
    #[inline(always)]
    pub fn vddrf_trim_0p76v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDRF_TRIM_0P76V)
    }
    #[doc = "1.0 V"]
    #[inline(always)]
    pub fn vddrf_trim_1p00v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDRF_TRIM_1P00V)
    }
    #[doc = "1.08 V"]
    #[inline(always)]
    pub fn vddrf_trim_1p08v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDRF_TRIM_1P08V)
    }
    #[doc = "1.1 V"]
    #[inline(always)]
    pub fn vddrf_trim_1p10v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDRF_TRIM_1P10V)
    }
    #[doc = "1.15 V"]
    #[inline(always)]
    pub fn vddrf_trim_1p15v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDRF_TRIM_1P15V)
    }
    #[doc = "1.2 V"]
    #[inline(always)]
    pub fn vddrf_trim_1p20v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDRF_TRIM_1P20V)
    }
    #[doc = "1.25 V"]
    #[inline(always)]
    pub fn vddrf_trim_1p25v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDRF_TRIM_1P25V)
    }
    #[doc = "1.32 V"]
    #[inline(always)]
    pub fn vddrf_trim_1p32v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDRF_TRIM_1P32V)
    }
    #[doc = "1.38 V"]
    #[inline(always)]
    pub fn vddrf_trim_1p38v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDRF_TRIM_1P38V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - Supply ready"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Disable mode clamp control"]
    #[inline(always)]
    pub fn clamp(&self) -> CLAMP_R {
        CLAMP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable control"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - Output voltage trimming configuration in 10 mV steps"]
    #[inline(always)]
    pub fn vtrim(&self) -> VTRIM_R {
        VTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - Disable mode clamp control"]
    #[inline(always)]
    pub fn clamp(&mut self) -> CLAMP_W {
        CLAMP_W { w: self }
    }
    #[doc = "Bit 8 - Enable control"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 0:5 - Output voltage trimming configuration in 10 mV steps"]
    #[inline(always)]
    pub fn vtrim(&mut self) -> VTRIM_W {
        VTRIM_W { w: self }
    }
}
