#[doc = "Reader of register ACS_RCOSC_CTRL"]
pub type R = crate::R<u32, super::ACS_RCOSC_CTRL>;
#[doc = "Writer for register ACS_RCOSC_CTRL"]
pub type W = crate::W<u32, super::ACS_RCOSC_CTRL>;
#[doc = "Register ACS_RCOSC_CTRL `reset()`'s with value 0x2020"]
impl crate::ResetValue for super::ACS_RCOSC_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2020
    }
}
#[doc = "Enable 12 MHz mode of startup oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLOCK_MULT_A {
    #[doc = "0: The startup RC Oscillator is at 3 MHz"]
    RC_START_OSC_3MHZ = 0,
    #[doc = "1: The startup RC Oscillator is at 12 MHz"]
    RC_START_OSC_12MHZ = 1,
}
impl From<CLOCK_MULT_A> for bool {
    #[inline(always)]
    fn from(variant: CLOCK_MULT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLOCK_MULT`"]
pub type CLOCK_MULT_R = crate::R<bool, CLOCK_MULT_A>;
impl CLOCK_MULT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLOCK_MULT_A {
        match self.bits {
            false => CLOCK_MULT_A::RC_START_OSC_3MHZ,
            true => CLOCK_MULT_A::RC_START_OSC_12MHZ,
        }
    }
    #[doc = "Checks if the value of the field is `RC_START_OSC_3MHZ`"]
    #[inline(always)]
    pub fn is_rc_start_osc_3mhz(&self) -> bool {
        *self == CLOCK_MULT_A::RC_START_OSC_3MHZ
    }
    #[doc = "Checks if the value of the field is `RC_START_OSC_12MHZ`"]
    #[inline(always)]
    pub fn is_rc_start_osc_12mhz(&self) -> bool {
        *self == CLOCK_MULT_A::RC_START_OSC_12MHZ
    }
}
#[doc = "Write proxy for field `CLOCK_MULT`"]
pub struct CLOCK_MULT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_MULT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLOCK_MULT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The startup RC Oscillator is at 3 MHz"]
    #[inline(always)]
    pub fn rc_start_osc_3mhz(self) -> &'a mut W {
        self.variant(CLOCK_MULT_A::RC_START_OSC_3MHZ)
    }
    #[doc = "The startup RC Oscillator is at 12 MHz"]
    #[inline(always)]
    pub fn rc_start_osc_12mhz(self) -> &'a mut W {
        self.variant(CLOCK_MULT_A::RC_START_OSC_12MHZ)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Enable/Disable the 32 kHz RC Oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RC_OSC_EN_A {
    #[doc = "0: The 32kHz RC Oscillator is disabled"]
    RC_OSC_DISABLE = 0,
    #[doc = "1: The 32kHz RC Oscillator is enabled"]
    RC_OSC_ENABLE = 1,
}
impl From<RC_OSC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RC_OSC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RC_OSC_EN`"]
pub type RC_OSC_EN_R = crate::R<bool, RC_OSC_EN_A>;
impl RC_OSC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RC_OSC_EN_A {
        match self.bits {
            false => RC_OSC_EN_A::RC_OSC_DISABLE,
            true => RC_OSC_EN_A::RC_OSC_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `RC_OSC_DISABLE`"]
    #[inline(always)]
    pub fn is_rc_osc_disable(&self) -> bool {
        *self == RC_OSC_EN_A::RC_OSC_DISABLE
    }
    #[doc = "Checks if the value of the field is `RC_OSC_ENABLE`"]
    #[inline(always)]
    pub fn is_rc_osc_enable(&self) -> bool {
        *self == RC_OSC_EN_A::RC_OSC_ENABLE
    }
}
#[doc = "Write proxy for field `RC_OSC_EN`"]
pub struct RC_OSC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RC_OSC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RC_OSC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The 32kHz RC Oscillator is disabled"]
    #[inline(always)]
    pub fn rc_osc_disable(self) -> &'a mut W {
        self.variant(RC_OSC_EN_A::RC_OSC_DISABLE)
    }
    #[doc = "The 32kHz RC Oscillator is enabled"]
    #[inline(always)]
    pub fn rc_osc_enable(self) -> &'a mut W {
        self.variant(RC_OSC_EN_A::RC_OSC_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Trimming flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTRIM_FLAG_A {
    #[doc = "0: The oscillators are not calibrated"]
    RC_OSC_UNCALIBRATED = 0,
    #[doc = "1: The oscillators are calibrated"]
    RC_OSC_CALIBRATED = 1,
}
impl From<FTRIM_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: FTRIM_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTRIM_FLAG`"]
pub type FTRIM_FLAG_R = crate::R<bool, FTRIM_FLAG_A>;
impl FTRIM_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTRIM_FLAG_A {
        match self.bits {
            false => FTRIM_FLAG_A::RC_OSC_UNCALIBRATED,
            true => FTRIM_FLAG_A::RC_OSC_CALIBRATED,
        }
    }
    #[doc = "Checks if the value of the field is `RC_OSC_UNCALIBRATED`"]
    #[inline(always)]
    pub fn is_rc_osc_uncalibrated(&self) -> bool {
        *self == FTRIM_FLAG_A::RC_OSC_UNCALIBRATED
    }
    #[doc = "Checks if the value of the field is `RC_OSC_CALIBRATED`"]
    #[inline(always)]
    pub fn is_rc_osc_calibrated(&self) -> bool {
        *self == FTRIM_FLAG_A::RC_OSC_CALIBRATED
    }
}
#[doc = "Write proxy for field `FTRIM_FLAG`"]
pub struct FTRIM_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> FTRIM_FLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTRIM_FLAG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The oscillators are not calibrated"]
    #[inline(always)]
    pub fn rc_osc_uncalibrated(self) -> &'a mut W {
        self.variant(FTRIM_FLAG_A::RC_OSC_UNCALIBRATED)
    }
    #[doc = "The oscillators are calibrated"]
    #[inline(always)]
    pub fn rc_osc_calibrated(self) -> &'a mut W {
        self.variant(FTRIM_FLAG_A::RC_OSC_CALIBRATED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Start RC oscillator frequency trimming\n\nValue on reset: 32"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTRIM_START_A {
    #[doc = "0: -48  percent trimming"]
    RC_START_OSC_M48 = 0,
    #[doc = "1: -46.5 percent trimming"]
    RC_START_OSC_M46P5 = 1,
    #[doc = "32: Nominal frequency"]
    RC_START_OSC_NOM = 32,
    #[doc = "63: +46.5 percent trimming"]
    RC_START_OSC_P46P5 = 63,
}
impl From<FTRIM_START_A> for u8 {
    #[inline(always)]
    fn from(variant: FTRIM_START_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTRIM_START`"]
pub type FTRIM_START_R = crate::R<u8, FTRIM_START_A>;
impl FTRIM_START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FTRIM_START_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FTRIM_START_A::RC_START_OSC_M48),
            1 => Val(FTRIM_START_A::RC_START_OSC_M46P5),
            32 => Val(FTRIM_START_A::RC_START_OSC_NOM),
            63 => Val(FTRIM_START_A::RC_START_OSC_P46P5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RC_START_OSC_M48`"]
    #[inline(always)]
    pub fn is_rc_start_osc_m48(&self) -> bool {
        *self == FTRIM_START_A::RC_START_OSC_M48
    }
    #[doc = "Checks if the value of the field is `RC_START_OSC_M46P5`"]
    #[inline(always)]
    pub fn is_rc_start_osc_m46p5(&self) -> bool {
        *self == FTRIM_START_A::RC_START_OSC_M46P5
    }
    #[doc = "Checks if the value of the field is `RC_START_OSC_NOM`"]
    #[inline(always)]
    pub fn is_rc_start_osc_nom(&self) -> bool {
        *self == FTRIM_START_A::RC_START_OSC_NOM
    }
    #[doc = "Checks if the value of the field is `RC_START_OSC_P46P5`"]
    #[inline(always)]
    pub fn is_rc_start_osc_p46p5(&self) -> bool {
        *self == FTRIM_START_A::RC_START_OSC_P46P5
    }
}
#[doc = "Write proxy for field `FTRIM_START`"]
pub struct FTRIM_START_W<'a> {
    w: &'a mut W,
}
impl<'a> FTRIM_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTRIM_START_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "-48 percent trimming"]
    #[inline(always)]
    pub fn rc_start_osc_m48(self) -> &'a mut W {
        self.variant(FTRIM_START_A::RC_START_OSC_M48)
    }
    #[doc = "-46.5 percent trimming"]
    #[inline(always)]
    pub fn rc_start_osc_m46p5(self) -> &'a mut W {
        self.variant(FTRIM_START_A::RC_START_OSC_M46P5)
    }
    #[doc = "Nominal frequency"]
    #[inline(always)]
    pub fn rc_start_osc_nom(self) -> &'a mut W {
        self.variant(FTRIM_START_A::RC_START_OSC_NOM)
    }
    #[doc = "+46.5 percent trimming"]
    #[inline(always)]
    pub fn rc_start_osc_p46p5(self) -> &'a mut W {
        self.variant(FTRIM_START_A::RC_START_OSC_P46P5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Adjust 32 kHz oscillator frequency range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTRIM_32K_ADJ_A {
    #[doc = "0: The 32 kHz RC Oscillator frequency range is nominal"]
    RC_OSC_RANGE_NOM = 0,
    #[doc = "1: The 32 kHz RC Oscillator frequency range is lowered by 25 percent"]
    RC_OSC_RANGE_M25 = 1,
}
impl From<FTRIM_32K_ADJ_A> for bool {
    #[inline(always)]
    fn from(variant: FTRIM_32K_ADJ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTRIM_32K_ADJ`"]
pub type FTRIM_32K_ADJ_R = crate::R<bool, FTRIM_32K_ADJ_A>;
impl FTRIM_32K_ADJ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTRIM_32K_ADJ_A {
        match self.bits {
            false => FTRIM_32K_ADJ_A::RC_OSC_RANGE_NOM,
            true => FTRIM_32K_ADJ_A::RC_OSC_RANGE_M25,
        }
    }
    #[doc = "Checks if the value of the field is `RC_OSC_RANGE_NOM`"]
    #[inline(always)]
    pub fn is_rc_osc_range_nom(&self) -> bool {
        *self == FTRIM_32K_ADJ_A::RC_OSC_RANGE_NOM
    }
    #[doc = "Checks if the value of the field is `RC_OSC_RANGE_M25`"]
    #[inline(always)]
    pub fn is_rc_osc_range_m25(&self) -> bool {
        *self == FTRIM_32K_ADJ_A::RC_OSC_RANGE_M25
    }
}
#[doc = "Write proxy for field `FTRIM_32K_ADJ`"]
pub struct FTRIM_32K_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> FTRIM_32K_ADJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTRIM_32K_ADJ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The 32 kHz RC Oscillator frequency range is nominal"]
    #[inline(always)]
    pub fn rc_osc_range_nom(self) -> &'a mut W {
        self.variant(FTRIM_32K_ADJ_A::RC_OSC_RANGE_NOM)
    }
    #[doc = "The 32 kHz RC Oscillator frequency range is lowered by 25 percent"]
    #[inline(always)]
    pub fn rc_osc_range_m25(self) -> &'a mut W {
        self.variant(FTRIM_32K_ADJ_A::RC_OSC_RANGE_M25)
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
#[doc = "32 kHz RC oscillator frequency trimming\n\nValue on reset: 32"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTRIM_32K_A {
    #[doc = "0: -48  percent trimming"]
    RC_OSC_M48 = 0,
    #[doc = "1: -46.5 percent trimming"]
    RC_OSC_M46P5 = 1,
    #[doc = "32: Nominal frequency"]
    RC_OSC_NOM = 32,
    #[doc = "63: +46.5 percent trimming"]
    RC_OSC_P46P5 = 63,
}
impl From<FTRIM_32K_A> for u8 {
    #[inline(always)]
    fn from(variant: FTRIM_32K_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTRIM_32K`"]
pub type FTRIM_32K_R = crate::R<u8, FTRIM_32K_A>;
impl FTRIM_32K_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FTRIM_32K_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FTRIM_32K_A::RC_OSC_M48),
            1 => Val(FTRIM_32K_A::RC_OSC_M46P5),
            32 => Val(FTRIM_32K_A::RC_OSC_NOM),
            63 => Val(FTRIM_32K_A::RC_OSC_P46P5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RC_OSC_M48`"]
    #[inline(always)]
    pub fn is_rc_osc_m48(&self) -> bool {
        *self == FTRIM_32K_A::RC_OSC_M48
    }
    #[doc = "Checks if the value of the field is `RC_OSC_M46P5`"]
    #[inline(always)]
    pub fn is_rc_osc_m46p5(&self) -> bool {
        *self == FTRIM_32K_A::RC_OSC_M46P5
    }
    #[doc = "Checks if the value of the field is `RC_OSC_NOM`"]
    #[inline(always)]
    pub fn is_rc_osc_nom(&self) -> bool {
        *self == FTRIM_32K_A::RC_OSC_NOM
    }
    #[doc = "Checks if the value of the field is `RC_OSC_P46P5`"]
    #[inline(always)]
    pub fn is_rc_osc_p46p5(&self) -> bool {
        *self == FTRIM_32K_A::RC_OSC_P46P5
    }
}
#[doc = "Write proxy for field `FTRIM_32K`"]
pub struct FTRIM_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> FTRIM_32K_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTRIM_32K_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "-48 percent trimming"]
    #[inline(always)]
    pub fn rc_osc_m48(self) -> &'a mut W {
        self.variant(FTRIM_32K_A::RC_OSC_M48)
    }
    #[doc = "-46.5 percent trimming"]
    #[inline(always)]
    pub fn rc_osc_m46p5(self) -> &'a mut W {
        self.variant(FTRIM_32K_A::RC_OSC_M46P5)
    }
    #[doc = "Nominal frequency"]
    #[inline(always)]
    pub fn rc_osc_nom(self) -> &'a mut W {
        self.variant(FTRIM_32K_A::RC_OSC_NOM)
    }
    #[doc = "+46.5 percent trimming"]
    #[inline(always)]
    pub fn rc_osc_p46p5(self) -> &'a mut W {
        self.variant(FTRIM_32K_A::RC_OSC_P46P5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - Enable 12 MHz mode of startup oscillator"]
    #[inline(always)]
    pub fn clock_mult(&self) -> CLOCK_MULT_R {
        CLOCK_MULT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable/Disable the 32 kHz RC Oscillator"]
    #[inline(always)]
    pub fn rc_osc_en(&self) -> RC_OSC_EN_R {
        RC_OSC_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Trimming flag"]
    #[inline(always)]
    pub fn ftrim_flag(&self) -> FTRIM_FLAG_R {
        FTRIM_FLAG_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Start RC oscillator frequency trimming"]
    #[inline(always)]
    pub fn ftrim_start(&self) -> FTRIM_START_R {
        FTRIM_START_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Adjust 32 kHz oscillator frequency range"]
    #[inline(always)]
    pub fn ftrim_32k_adj(&self) -> FTRIM_32K_ADJ_R {
        FTRIM_32K_ADJ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - 32 kHz RC oscillator frequency trimming"]
    #[inline(always)]
    pub fn ftrim_32k(&self) -> FTRIM_32K_R {
        FTRIM_32K_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 18 - Enable 12 MHz mode of startup oscillator"]
    #[inline(always)]
    pub fn clock_mult(&mut self) -> CLOCK_MULT_W {
        CLOCK_MULT_W { w: self }
    }
    #[doc = "Bit 16 - Enable/Disable the 32 kHz RC Oscillator"]
    #[inline(always)]
    pub fn rc_osc_en(&mut self) -> RC_OSC_EN_W {
        RC_OSC_EN_W { w: self }
    }
    #[doc = "Bit 15 - Trimming flag"]
    #[inline(always)]
    pub fn ftrim_flag(&mut self) -> FTRIM_FLAG_W {
        FTRIM_FLAG_W { w: self }
    }
    #[doc = "Bits 8:13 - Start RC oscillator frequency trimming"]
    #[inline(always)]
    pub fn ftrim_start(&mut self) -> FTRIM_START_W {
        FTRIM_START_W { w: self }
    }
    #[doc = "Bit 6 - Adjust 32 kHz oscillator frequency range"]
    #[inline(always)]
    pub fn ftrim_32k_adj(&mut self) -> FTRIM_32K_ADJ_W {
        FTRIM_32K_ADJ_W { w: self }
    }
    #[doc = "Bits 0:5 - 32 kHz RC oscillator frequency trimming"]
    #[inline(always)]
    pub fn ftrim_32k(&mut self) -> FTRIM_32K_W {
        FTRIM_32K_W { w: self }
    }
}
