#[doc = "Reader of register ACS_RTC_CTRL"]
pub type R = crate::R<u32, super::ACS_RTC_CTRL>;
#[doc = "Writer for register ACS_RTC_CTRL"]
pub type W = crate::W<u32, super::ACS_RTC_CTRL>;
#[doc = "Register ACS_RTC_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ACS_RTC_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Force a clock on RTC timer (Test Purpose)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCE_CLOCK_AW {
    #[doc = "1: Clock the RTC timer (has an effect only if the source clock is low)"]
    RTC_FORCE_CLOCK = 1,
}
impl From<FORCE_CLOCK_AW> for bool {
    #[inline(always)]
    fn from(variant: FORCE_CLOCK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FORCE_CLOCK`"]
pub struct FORCE_CLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_CLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCE_CLOCK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock the RTC timer (has an effect only if the source clock is low)"]
    #[inline(always)]
    pub fn rtc_force_clock(self) -> &'a mut W {
        self.variant(FORCE_CLOCK_AW::RTC_FORCE_CLOCK)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reset the RTC timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_AW {
    #[doc = "1: The RTC timer is reset"]
    RTC_RESET = 1,
}
impl From<RESET_AW> for bool {
    #[inline(always)]
    fn from(variant: RESET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RESET`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESET_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The RTC timer is reset"]
    #[inline(always)]
    pub fn rtc_reset(self) -> &'a mut W {
        self.variant(RESET_AW::RTC_RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Configure RTC timer alarm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ALARM_CFG_A {
    #[doc = "0: RTC alarm is disabled"]
    RTC_ALARM_DISABLE = 0,
    #[doc = "1: RTC alarm on counter bit 7 rising edge (7.8125 ms)"]
    RTC_ALARM_7P8125MS = 1,
    #[doc = "2: RTC alarm on counter bit 8 rising edge (15.625 ms)"]
    RTC_ALARM_15P625MS = 2,
    #[doc = "3: RTC alarm on counter bit 9 rising edge (31.25 ms)"]
    RTC_ALARM_31P25MS = 3,
    #[doc = "4: RTC alarm on counter bit 10 rising edge (62.5 ms)"]
    RTC_ALARM_62P5MS = 4,
    #[doc = "5: RTC alarm on counter bit 11 rising edge (125 ms)"]
    RTC_ALARM_125MS = 5,
    #[doc = "6: RTC alarm on counter bit 12 rising edge (250 ms)"]
    RTC_ALARM_250MS = 6,
    #[doc = "7: RTC alarm on counter bit 13 rising edge (500 ms)"]
    RTC_ALARM_500MS = 7,
    #[doc = "8: RTC alarm on counter bit 14 rising edge (1 s)"]
    RTC_ALARM_1S = 8,
    #[doc = "9: RTC alarm on counter bit 15 rising edge (2 s)"]
    RTC_ALARM_2S = 9,
    #[doc = "10: RTC alarm on counter bit 16 rising edge (4 s)"]
    RTC_ALARM_4S = 10,
    #[doc = "11: RTC alarm on counter bit 17 rising edge (8 s)"]
    RTC_ALARM_8S = 11,
    #[doc = "12: RTC alarm on counter bit 18 rising edge (16 s)"]
    RTC_ALARM_16S = 12,
    #[doc = "13: RTC alarm on counter bit 19 rising edge (32 s)"]
    RTC_ALARM_32S = 13,
    #[doc = "14: RTC alarm on counter bit 20 rising edge (64 s)"]
    RTC_ALARM_64S = 14,
    #[doc = "15: RTC alarm on (down) counter reaching zero (up to 36.4 hours)"]
    RTC_ALARM_ZERO = 15,
}
impl From<ALARM_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: ALARM_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ALARM_CFG`"]
pub type ALARM_CFG_R = crate::R<u8, ALARM_CFG_A>;
impl ALARM_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARM_CFG_A {
        match self.bits {
            0 => ALARM_CFG_A::RTC_ALARM_DISABLE,
            1 => ALARM_CFG_A::RTC_ALARM_7P8125MS,
            2 => ALARM_CFG_A::RTC_ALARM_15P625MS,
            3 => ALARM_CFG_A::RTC_ALARM_31P25MS,
            4 => ALARM_CFG_A::RTC_ALARM_62P5MS,
            5 => ALARM_CFG_A::RTC_ALARM_125MS,
            6 => ALARM_CFG_A::RTC_ALARM_250MS,
            7 => ALARM_CFG_A::RTC_ALARM_500MS,
            8 => ALARM_CFG_A::RTC_ALARM_1S,
            9 => ALARM_CFG_A::RTC_ALARM_2S,
            10 => ALARM_CFG_A::RTC_ALARM_4S,
            11 => ALARM_CFG_A::RTC_ALARM_8S,
            12 => ALARM_CFG_A::RTC_ALARM_16S,
            13 => ALARM_CFG_A::RTC_ALARM_32S,
            14 => ALARM_CFG_A::RTC_ALARM_64S,
            15 => ALARM_CFG_A::RTC_ALARM_ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTC_ALARM_DISABLE`"]
    #[inline(always)]
    pub fn is_rtc_alarm_disable(&self) -> bool {
        *self == ALARM_CFG_A::RTC_ALARM_DISABLE
    }
    #[doc = "Checks if the value of the field is `RTC_ALARM_7P8125MS`"]
    #[inline(always)]
    pub fn is_rtc_alarm_7p8125ms(&self) -> bool {
        *self == ALARM_CFG_A::RTC_ALARM_7P8125MS
    }
    #[doc = "Checks if the value of the field is `RTC_ALARM_15P625MS`"]
    #[inline(always)]
    pub fn is_rtc_alarm_15p625ms(&self) -> bool {
        *self == ALARM_CFG_A::RTC_ALARM_15P625MS
    }
    #[doc = "Checks if the value of the field is `RTC_ALARM_31P25MS`"]
    #[inline(always)]
    pub fn is_rtc_alarm_31p25ms(&self) -> bool {
        *self == ALARM_CFG_A::RTC_ALARM_31P25MS
    }
    #[doc = "Checks if the value of the field is `RTC_ALARM_62P5MS`"]
    #[inline(always)]
    pub fn is_rtc_alarm_62p5ms(&self) -> bool {
        *self == ALARM_CFG_A::RTC_ALARM_62P5MS
    }
    #[doc = "Checks if the value of the field is `RTC_ALARM_125MS`"]
    #[inline(always)]
    pub fn is_rtc_alarm_125ms(&self) -> bool {
        *self == ALARM_CFG_A::RTC_ALARM_125MS
    }
    #[doc = "Checks if the value of the field is `RTC_ALARM_250MS`"]
    #[inline(always)]
    pub fn is_rtc_alarm_250ms(&self) -> bool {
        *self == ALARM_CFG_A::RTC_ALARM_250MS
    }
    #[doc = "Checks if the value of the field is `RTC_ALARM_500MS`"]
    #[inline(always)]
    pub fn is_rtc_alarm_500ms(&self) -> bool {
        *self == ALARM_CFG_A::RTC_ALARM_500MS
    }
    #[doc = "Checks if the value of the field is `RTC_ALARM_1S`"]
    #[inline(always)]
    pub fn is_rtc_alarm_1s(&self) -> bool {
        *self == ALARM_CFG_A::RTC_ALARM_1S
    }
    #[doc = "Checks if the value of the field is `RTC_ALARM_2S`"]
    #[inline(always)]
    pub fn is_rtc_alarm_2s(&self) -> bool {
        *self == ALARM_CFG_A::RTC_ALARM_2S
    }
    #[doc = "Checks if the value of the field is `RTC_ALARM_4S`"]
    #[inline(always)]
    pub fn is_rtc_alarm_4s(&self) -> bool {
        *self == ALARM_CFG_A::RTC_ALARM_4S
    }
    #[doc = "Checks if the value of the field is `RTC_ALARM_8S`"]
    #[inline(always)]
    pub fn is_rtc_alarm_8s(&self) -> bool {
        *self == ALARM_CFG_A::RTC_ALARM_8S
    }
    #[doc = "Checks if the value of the field is `RTC_ALARM_16S`"]
    #[inline(always)]
    pub fn is_rtc_alarm_16s(&self) -> bool {
        *self == ALARM_CFG_A::RTC_ALARM_16S
    }
    #[doc = "Checks if the value of the field is `RTC_ALARM_32S`"]
    #[inline(always)]
    pub fn is_rtc_alarm_32s(&self) -> bool {
        *self == ALARM_CFG_A::RTC_ALARM_32S
    }
    #[doc = "Checks if the value of the field is `RTC_ALARM_64S`"]
    #[inline(always)]
    pub fn is_rtc_alarm_64s(&self) -> bool {
        *self == ALARM_CFG_A::RTC_ALARM_64S
    }
    #[doc = "Checks if the value of the field is `RTC_ALARM_ZERO`"]
    #[inline(always)]
    pub fn is_rtc_alarm_zero(&self) -> bool {
        *self == ALARM_CFG_A::RTC_ALARM_ZERO
    }
}
#[doc = "Write proxy for field `ALARM_CFG`"]
pub struct ALARM_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARM_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALARM_CFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "RTC alarm is disabled"]
    #[inline(always)]
    pub fn rtc_alarm_disable(self) -> &'a mut W {
        self.variant(ALARM_CFG_A::RTC_ALARM_DISABLE)
    }
    #[doc = "RTC alarm on counter bit 7 rising edge (7.8125 ms)"]
    #[inline(always)]
    pub fn rtc_alarm_7p8125ms(self) -> &'a mut W {
        self.variant(ALARM_CFG_A::RTC_ALARM_7P8125MS)
    }
    #[doc = "RTC alarm on counter bit 8 rising edge (15.625 ms)"]
    #[inline(always)]
    pub fn rtc_alarm_15p625ms(self) -> &'a mut W {
        self.variant(ALARM_CFG_A::RTC_ALARM_15P625MS)
    }
    #[doc = "RTC alarm on counter bit 9 rising edge (31.25 ms)"]
    #[inline(always)]
    pub fn rtc_alarm_31p25ms(self) -> &'a mut W {
        self.variant(ALARM_CFG_A::RTC_ALARM_31P25MS)
    }
    #[doc = "RTC alarm on counter bit 10 rising edge (62.5 ms)"]
    #[inline(always)]
    pub fn rtc_alarm_62p5ms(self) -> &'a mut W {
        self.variant(ALARM_CFG_A::RTC_ALARM_62P5MS)
    }
    #[doc = "RTC alarm on counter bit 11 rising edge (125 ms)"]
    #[inline(always)]
    pub fn rtc_alarm_125ms(self) -> &'a mut W {
        self.variant(ALARM_CFG_A::RTC_ALARM_125MS)
    }
    #[doc = "RTC alarm on counter bit 12 rising edge (250 ms)"]
    #[inline(always)]
    pub fn rtc_alarm_250ms(self) -> &'a mut W {
        self.variant(ALARM_CFG_A::RTC_ALARM_250MS)
    }
    #[doc = "RTC alarm on counter bit 13 rising edge (500 ms)"]
    #[inline(always)]
    pub fn rtc_alarm_500ms(self) -> &'a mut W {
        self.variant(ALARM_CFG_A::RTC_ALARM_500MS)
    }
    #[doc = "RTC alarm on counter bit 14 rising edge (1 s)"]
    #[inline(always)]
    pub fn rtc_alarm_1s(self) -> &'a mut W {
        self.variant(ALARM_CFG_A::RTC_ALARM_1S)
    }
    #[doc = "RTC alarm on counter bit 15 rising edge (2 s)"]
    #[inline(always)]
    pub fn rtc_alarm_2s(self) -> &'a mut W {
        self.variant(ALARM_CFG_A::RTC_ALARM_2S)
    }
    #[doc = "RTC alarm on counter bit 16 rising edge (4 s)"]
    #[inline(always)]
    pub fn rtc_alarm_4s(self) -> &'a mut W {
        self.variant(ALARM_CFG_A::RTC_ALARM_4S)
    }
    #[doc = "RTC alarm on counter bit 17 rising edge (8 s)"]
    #[inline(always)]
    pub fn rtc_alarm_8s(self) -> &'a mut W {
        self.variant(ALARM_CFG_A::RTC_ALARM_8S)
    }
    #[doc = "RTC alarm on counter bit 18 rising edge (16 s)"]
    #[inline(always)]
    pub fn rtc_alarm_16s(self) -> &'a mut W {
        self.variant(ALARM_CFG_A::RTC_ALARM_16S)
    }
    #[doc = "RTC alarm on counter bit 19 rising edge (32 s)"]
    #[inline(always)]
    pub fn rtc_alarm_32s(self) -> &'a mut W {
        self.variant(ALARM_CFG_A::RTC_ALARM_32S)
    }
    #[doc = "RTC alarm on counter bit 20 rising edge (64 s)"]
    #[inline(always)]
    pub fn rtc_alarm_64s(self) -> &'a mut W {
        self.variant(ALARM_CFG_A::RTC_ALARM_64S)
    }
    #[doc = "RTC alarm on (down) counter reaching zero (up to 36.4 hours)"]
    #[inline(always)]
    pub fn rtc_alarm_zero(self) -> &'a mut W {
        self.variant(ALARM_CFG_A::RTC_ALARM_ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Select the RTC, standby and bb timer clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_SRC_SEL_A {
    #[doc = "0: Select the internal RC Oscillator clock"]
    RTC_CLK_SRC_RC_OSC = 0,
    #[doc = "1: Select the internal XTAL 32 kHz clock"]
    RTC_CLK_SRC_XTAL32K = 1,
    #[doc = "4: Select DIO0 as a clock source"]
    RTC_CLK_SRC_DIO0 = 4,
    #[doc = "5: Select DIO1 as a clock source"]
    RTC_CLK_SRC_DIO1 = 5,
    #[doc = "6: Select DIO2 as a clock source"]
    RTC_CLK_SRC_DIO2 = 6,
    #[doc = "7: Select DIO3 as a clock source"]
    RTC_CLK_SRC_DIO3 = 7,
}
impl From<CLK_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SRC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLK_SRC_SEL`"]
pub type CLK_SRC_SEL_R = crate::R<u8, CLK_SRC_SEL_A>;
impl CLK_SRC_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLK_SRC_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLK_SRC_SEL_A::RTC_CLK_SRC_RC_OSC),
            1 => Val(CLK_SRC_SEL_A::RTC_CLK_SRC_XTAL32K),
            4 => Val(CLK_SRC_SEL_A::RTC_CLK_SRC_DIO0),
            5 => Val(CLK_SRC_SEL_A::RTC_CLK_SRC_DIO1),
            6 => Val(CLK_SRC_SEL_A::RTC_CLK_SRC_DIO2),
            7 => Val(CLK_SRC_SEL_A::RTC_CLK_SRC_DIO3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RTC_CLK_SRC_RC_OSC`"]
    #[inline(always)]
    pub fn is_rtc_clk_src_rc_osc(&self) -> bool {
        *self == CLK_SRC_SEL_A::RTC_CLK_SRC_RC_OSC
    }
    #[doc = "Checks if the value of the field is `RTC_CLK_SRC_XTAL32K`"]
    #[inline(always)]
    pub fn is_rtc_clk_src_xtal32k(&self) -> bool {
        *self == CLK_SRC_SEL_A::RTC_CLK_SRC_XTAL32K
    }
    #[doc = "Checks if the value of the field is `RTC_CLK_SRC_DIO0`"]
    #[inline(always)]
    pub fn is_rtc_clk_src_dio0(&self) -> bool {
        *self == CLK_SRC_SEL_A::RTC_CLK_SRC_DIO0
    }
    #[doc = "Checks if the value of the field is `RTC_CLK_SRC_DIO1`"]
    #[inline(always)]
    pub fn is_rtc_clk_src_dio1(&self) -> bool {
        *self == CLK_SRC_SEL_A::RTC_CLK_SRC_DIO1
    }
    #[doc = "Checks if the value of the field is `RTC_CLK_SRC_DIO2`"]
    #[inline(always)]
    pub fn is_rtc_clk_src_dio2(&self) -> bool {
        *self == CLK_SRC_SEL_A::RTC_CLK_SRC_DIO2
    }
    #[doc = "Checks if the value of the field is `RTC_CLK_SRC_DIO3`"]
    #[inline(always)]
    pub fn is_rtc_clk_src_dio3(&self) -> bool {
        *self == CLK_SRC_SEL_A::RTC_CLK_SRC_DIO3
    }
}
#[doc = "Write proxy for field `CLK_SRC_SEL`"]
pub struct CLK_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SRC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_SRC_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select the internal RC Oscillator clock"]
    #[inline(always)]
    pub fn rtc_clk_src_rc_osc(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::RTC_CLK_SRC_RC_OSC)
    }
    #[doc = "Select the internal XTAL 32 kHz clock"]
    #[inline(always)]
    pub fn rtc_clk_src_xtal32k(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::RTC_CLK_SRC_XTAL32K)
    }
    #[doc = "Select DIO0 as a clock source"]
    #[inline(always)]
    pub fn rtc_clk_src_dio0(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::RTC_CLK_SRC_DIO0)
    }
    #[doc = "Select DIO1 as a clock source"]
    #[inline(always)]
    pub fn rtc_clk_src_dio1(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::RTC_CLK_SRC_DIO1)
    }
    #[doc = "Select DIO2 as a clock source"]
    #[inline(always)]
    pub fn rtc_clk_src_dio2(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::RTC_CLK_SRC_DIO2)
    }
    #[doc = "Select DIO3 as a clock source"]
    #[inline(always)]
    pub fn rtc_clk_src_dio3(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::RTC_CLK_SRC_DIO3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Enable counter and RTC interrupt every 1s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: The RTC is disabled"]
    RTC_DISABLE = 0,
    #[doc = "1: The RTC is enabled"]
    RTC_ENABLE = 1,
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
            false => ENABLE_A::RTC_DISABLE,
            true => ENABLE_A::RTC_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `RTC_DISABLE`"]
    #[inline(always)]
    pub fn is_rtc_disable(&self) -> bool {
        *self == ENABLE_A::RTC_DISABLE
    }
    #[doc = "Checks if the value of the field is `RTC_ENABLE`"]
    #[inline(always)]
    pub fn is_rtc_enable(&self) -> bool {
        *self == ENABLE_A::RTC_ENABLE
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
    #[doc = "The RTC is disabled"]
    #[inline(always)]
    pub fn rtc_disable(self) -> &'a mut W {
        self.variant(ENABLE_A::RTC_DISABLE)
    }
    #[doc = "The RTC is enabled"]
    #[inline(always)]
    pub fn rtc_enable(self) -> &'a mut W {
        self.variant(ENABLE_A::RTC_ENABLE)
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
    #[doc = "Bits 4:7 - Configure RTC timer alarm"]
    #[inline(always)]
    pub fn alarm_cfg(&self) -> ALARM_CFG_R {
        ALARM_CFG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 1:3 - Select the RTC, standby and bb timer clock source"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - Enable counter and RTC interrupt every 1s"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - Force a clock on RTC timer (Test Purpose)"]
    #[inline(always)]
    pub fn force_clock(&mut self) -> FORCE_CLOCK_W {
        FORCE_CLOCK_W { w: self }
    }
    #[doc = "Bit 24 - Reset the RTC timer"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bits 4:7 - Configure RTC timer alarm"]
    #[inline(always)]
    pub fn alarm_cfg(&mut self) -> ALARM_CFG_W {
        ALARM_CFG_W { w: self }
    }
    #[doc = "Bits 1:3 - Select the RTC, standby and bb timer clock source"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W {
        CLK_SRC_SEL_W { w: self }
    }
    #[doc = "Bit 0 - Enable counter and RTC interrupt every 1s"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
