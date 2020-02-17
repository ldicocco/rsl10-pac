#[doc = "Reader of register ACS_AOUT_CTRL"]
pub type R = crate::R<u32, super::ACS_AOUT_CTRL>;
#[doc = "Writer for register ACS_AOUT_CTRL"]
pub type W = crate::W<u32, super::ACS_AOUT_CTRL>;
#[doc = "Register ACS_AOUT_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ACS_AOUT_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Stop edge for RTC clock output on AOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_CLOCK_DIO0_STOP_EDGE_A {
    #[doc = "0: Stop to output RTC clock on rising edge"]
    DIO0_RTC_CLK_STOP_RISING = 0,
    #[doc = "1: Stop to output RTC clock on falling edge"]
    DIO0_RTC_CLK_STOP_FALLING = 1,
}
impl From<RTC_CLOCK_DIO0_STOP_EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_CLOCK_DIO0_STOP_EDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTC_CLOCK_DIO0_STOP_EDGE`"]
pub type RTC_CLOCK_DIO0_STOP_EDGE_R = crate::R<bool, RTC_CLOCK_DIO0_STOP_EDGE_A>;
impl RTC_CLOCK_DIO0_STOP_EDGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_CLOCK_DIO0_STOP_EDGE_A {
        match self.bits {
            false => RTC_CLOCK_DIO0_STOP_EDGE_A::DIO0_RTC_CLK_STOP_RISING,
            true => RTC_CLOCK_DIO0_STOP_EDGE_A::DIO0_RTC_CLK_STOP_FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `DIO0_RTC_CLK_STOP_RISING`"]
    #[inline(always)]
    pub fn is_dio0_rtc_clk_stop_rising(&self) -> bool {
        *self == RTC_CLOCK_DIO0_STOP_EDGE_A::DIO0_RTC_CLK_STOP_RISING
    }
    #[doc = "Checks if the value of the field is `DIO0_RTC_CLK_STOP_FALLING`"]
    #[inline(always)]
    pub fn is_dio0_rtc_clk_stop_falling(&self) -> bool {
        *self == RTC_CLOCK_DIO0_STOP_EDGE_A::DIO0_RTC_CLK_STOP_FALLING
    }
}
#[doc = "Write proxy for field `RTC_CLOCK_DIO0_STOP_EDGE`"]
pub struct RTC_CLOCK_DIO0_STOP_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CLOCK_DIO0_STOP_EDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_CLOCK_DIO0_STOP_EDGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop to output RTC clock on rising edge"]
    #[inline(always)]
    pub fn dio0_rtc_clk_stop_rising(self) -> &'a mut W {
        self.variant(RTC_CLOCK_DIO0_STOP_EDGE_A::DIO0_RTC_CLK_STOP_RISING)
    }
    #[doc = "Stop to output RTC clock on falling edge"]
    #[inline(always)]
    pub fn dio0_rtc_clk_stop_falling(self) -> &'a mut W {
        self.variant(RTC_CLOCK_DIO0_STOP_EDGE_A::DIO0_RTC_CLK_STOP_FALLING)
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
#[doc = "Stop source for RTC clock output on AOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTC_CLOCK_DIO0_STOP_SRC_A {
    #[doc = "0: Stop to output RTC clock on DIO0 event"]
    DIO0_RTC_CLK_STOP_DIO0 = 0,
    #[doc = "1: Stop to output RTC clock on DIO1 event"]
    DIO0_RTC_CLK_STOP_DIO1 = 1,
    #[doc = "2: Stop to output RTC clock on DIO2 event"]
    DIO0_RTC_CLK_STOP_DIO2 = 2,
    #[doc = "3: Stop to output RTC clock on DIO3 event"]
    DIO0_RTC_CLK_STOP_DIO3 = 3,
}
impl From<RTC_CLOCK_DIO0_STOP_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: RTC_CLOCK_DIO0_STOP_SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTC_CLOCK_DIO0_STOP_SRC`"]
pub type RTC_CLOCK_DIO0_STOP_SRC_R = crate::R<u8, RTC_CLOCK_DIO0_STOP_SRC_A>;
impl RTC_CLOCK_DIO0_STOP_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_CLOCK_DIO0_STOP_SRC_A {
        match self.bits {
            0 => RTC_CLOCK_DIO0_STOP_SRC_A::DIO0_RTC_CLK_STOP_DIO0,
            1 => RTC_CLOCK_DIO0_STOP_SRC_A::DIO0_RTC_CLK_STOP_DIO1,
            2 => RTC_CLOCK_DIO0_STOP_SRC_A::DIO0_RTC_CLK_STOP_DIO2,
            3 => RTC_CLOCK_DIO0_STOP_SRC_A::DIO0_RTC_CLK_STOP_DIO3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIO0_RTC_CLK_STOP_DIO0`"]
    #[inline(always)]
    pub fn is_dio0_rtc_clk_stop_dio0(&self) -> bool {
        *self == RTC_CLOCK_DIO0_STOP_SRC_A::DIO0_RTC_CLK_STOP_DIO0
    }
    #[doc = "Checks if the value of the field is `DIO0_RTC_CLK_STOP_DIO1`"]
    #[inline(always)]
    pub fn is_dio0_rtc_clk_stop_dio1(&self) -> bool {
        *self == RTC_CLOCK_DIO0_STOP_SRC_A::DIO0_RTC_CLK_STOP_DIO1
    }
    #[doc = "Checks if the value of the field is `DIO0_RTC_CLK_STOP_DIO2`"]
    #[inline(always)]
    pub fn is_dio0_rtc_clk_stop_dio2(&self) -> bool {
        *self == RTC_CLOCK_DIO0_STOP_SRC_A::DIO0_RTC_CLK_STOP_DIO2
    }
    #[doc = "Checks if the value of the field is `DIO0_RTC_CLK_STOP_DIO3`"]
    #[inline(always)]
    pub fn is_dio0_rtc_clk_stop_dio3(&self) -> bool {
        *self == RTC_CLOCK_DIO0_STOP_SRC_A::DIO0_RTC_CLK_STOP_DIO3
    }
}
#[doc = "Write proxy for field `RTC_CLOCK_DIO0_STOP_SRC`"]
pub struct RTC_CLOCK_DIO0_STOP_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CLOCK_DIO0_STOP_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_CLOCK_DIO0_STOP_SRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Stop to output RTC clock on DIO0 event"]
    #[inline(always)]
    pub fn dio0_rtc_clk_stop_dio0(self) -> &'a mut W {
        self.variant(RTC_CLOCK_DIO0_STOP_SRC_A::DIO0_RTC_CLK_STOP_DIO0)
    }
    #[doc = "Stop to output RTC clock on DIO1 event"]
    #[inline(always)]
    pub fn dio0_rtc_clk_stop_dio1(self) -> &'a mut W {
        self.variant(RTC_CLOCK_DIO0_STOP_SRC_A::DIO0_RTC_CLK_STOP_DIO1)
    }
    #[doc = "Stop to output RTC clock on DIO2 event"]
    #[inline(always)]
    pub fn dio0_rtc_clk_stop_dio2(self) -> &'a mut W {
        self.variant(RTC_CLOCK_DIO0_STOP_SRC_A::DIO0_RTC_CLK_STOP_DIO2)
    }
    #[doc = "Stop to output RTC clock on DIO3 event"]
    #[inline(always)]
    pub fn dio0_rtc_clk_stop_dio3(self) -> &'a mut W {
        self.variant(RTC_CLOCK_DIO0_STOP_SRC_A::DIO0_RTC_CLK_STOP_DIO3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Start event for RTC clock output on AOUT (RTC prescaler and counter need to be enabled)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTC_CLOCK_DIO0_START_A {
    #[doc = "0: No start event (DIO0 not driven)"]
    DIO0_RTC_CLK_DISABLE = 0,
    #[doc = "1: Start to output RTC clock every 125 ms"]
    DIO0_RTC_CLK_125MS = 1,
    #[doc = "2: Start to output RTC clock every 250 ms"]
    DIO0_RTC_CLK_250MS = 2,
    #[doc = "3: Start to output RTC clock every 500 ms"]
    DIO0_RTC_CLK_500MS = 3,
    #[doc = "4: Start to output RTC clock every 1 s"]
    DIO0_RTC_CLK_1S = 4,
    #[doc = "5: Start to output RTC clock every 2 s"]
    DIO0_RTC_CLK_2S = 5,
    #[doc = "6: Start to output RTC clock every 4 s"]
    DIO0_RTC_CLK_4S = 6,
    #[doc = "7: Start to output RTC clock every 8 s"]
    DIO0_RTC_CLK_8S = 7,
}
impl From<RTC_CLOCK_DIO0_START_A> for u8 {
    #[inline(always)]
    fn from(variant: RTC_CLOCK_DIO0_START_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTC_CLOCK_DIO0_START`"]
pub type RTC_CLOCK_DIO0_START_R = crate::R<u8, RTC_CLOCK_DIO0_START_A>;
impl RTC_CLOCK_DIO0_START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_CLOCK_DIO0_START_A {
        match self.bits {
            0 => RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_DISABLE,
            1 => RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_125MS,
            2 => RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_250MS,
            3 => RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_500MS,
            4 => RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_1S,
            5 => RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_2S,
            6 => RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_4S,
            7 => RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_8S,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIO0_RTC_CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_dio0_rtc_clk_disable(&self) -> bool {
        *self == RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `DIO0_RTC_CLK_125MS`"]
    #[inline(always)]
    pub fn is_dio0_rtc_clk_125ms(&self) -> bool {
        *self == RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_125MS
    }
    #[doc = "Checks if the value of the field is `DIO0_RTC_CLK_250MS`"]
    #[inline(always)]
    pub fn is_dio0_rtc_clk_250ms(&self) -> bool {
        *self == RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_250MS
    }
    #[doc = "Checks if the value of the field is `DIO0_RTC_CLK_500MS`"]
    #[inline(always)]
    pub fn is_dio0_rtc_clk_500ms(&self) -> bool {
        *self == RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_500MS
    }
    #[doc = "Checks if the value of the field is `DIO0_RTC_CLK_1S`"]
    #[inline(always)]
    pub fn is_dio0_rtc_clk_1s(&self) -> bool {
        *self == RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_1S
    }
    #[doc = "Checks if the value of the field is `DIO0_RTC_CLK_2S`"]
    #[inline(always)]
    pub fn is_dio0_rtc_clk_2s(&self) -> bool {
        *self == RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_2S
    }
    #[doc = "Checks if the value of the field is `DIO0_RTC_CLK_4S`"]
    #[inline(always)]
    pub fn is_dio0_rtc_clk_4s(&self) -> bool {
        *self == RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_4S
    }
    #[doc = "Checks if the value of the field is `DIO0_RTC_CLK_8S`"]
    #[inline(always)]
    pub fn is_dio0_rtc_clk_8s(&self) -> bool {
        *self == RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_8S
    }
}
#[doc = "Write proxy for field `RTC_CLOCK_DIO0_START`"]
pub struct RTC_CLOCK_DIO0_START_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CLOCK_DIO0_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_CLOCK_DIO0_START_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No start event (DIO0 not driven)"]
    #[inline(always)]
    pub fn dio0_rtc_clk_disable(self) -> &'a mut W {
        self.variant(RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_DISABLE)
    }
    #[doc = "Start to output RTC clock every 125 ms"]
    #[inline(always)]
    pub fn dio0_rtc_clk_125ms(self) -> &'a mut W {
        self.variant(RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_125MS)
    }
    #[doc = "Start to output RTC clock every 250 ms"]
    #[inline(always)]
    pub fn dio0_rtc_clk_250ms(self) -> &'a mut W {
        self.variant(RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_250MS)
    }
    #[doc = "Start to output RTC clock every 500 ms"]
    #[inline(always)]
    pub fn dio0_rtc_clk_500ms(self) -> &'a mut W {
        self.variant(RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_500MS)
    }
    #[doc = "Start to output RTC clock every 1 s"]
    #[inline(always)]
    pub fn dio0_rtc_clk_1s(self) -> &'a mut W {
        self.variant(RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_1S)
    }
    #[doc = "Start to output RTC clock every 2 s"]
    #[inline(always)]
    pub fn dio0_rtc_clk_2s(self) -> &'a mut W {
        self.variant(RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_2S)
    }
    #[doc = "Start to output RTC clock every 4 s"]
    #[inline(always)]
    pub fn dio0_rtc_clk_4s(self) -> &'a mut W {
        self.variant(RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_4S)
    }
    #[doc = "Start to output RTC clock every 8 s"]
    #[inline(always)]
    pub fn dio0_rtc_clk_8s(self) -> &'a mut W {
        self.variant(RTC_CLOCK_DIO0_START_A::DIO0_RTC_CLK_8S)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "AOUT test signal selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TEST_AOUT_A {
    #[doc = "0: AOUT grounded"]
    AOUT_VSSA = 0,
    #[doc = "1: AOUT high / VCC connected on AOUT (can be sensed for 4 wires measurement of the load regulation)"]
    AOUT_VCC_SENSE = 1,
    #[doc = "2: Bandgap reference voltage 0p75V connected on AOUT"]
    AOUT_VREF_0P75V_OUTPUT = 2,
    #[doc = "3: Bandgap reference voltage 0p67V connected on AOUT"]
    AOUT_VREF_0P67V_OUTPUT = 3,
    #[doc = "4: Bandgap iref current source connected on AOUT"]
    AOUT_IREF_50N_OUTPUT = 4,
    #[doc = "5: PTAT iref current source connected on AOUT"]
    AOUT_IREF_1N_OUTPUT = 5,
    #[doc = "6: vddacs voltage connected on AOUT"]
    AOUT_VDDACS_OUTPUT = 6,
    #[doc = "7: Bandgap buffered reference voltage 0p75V connected on AOUT"]
    AOUT_VREF_0P75V_BUF_OUTPUT = 7,
    #[doc = "8: Bandgap regulated supply voltage"]
    AOUT_VREG_BG = 8,
    #[doc = "9: vddrf_sw voltage connected on AOUT"]
    AOUT_VDDRF_SW = 9,
    #[doc = "10: VDDRF connected on AOUT (can be sensed for 4 wires measurement of the load regulation)"]
    AOUT_VDDRF_SENSE = 10,
    #[doc = "11: Baseband timer supply voltage"]
    AOUT_VDDT = 11,
    #[doc = "12: VDDC connected on AOUT (can be sensed for 4 wires measurement of the load regulation)"]
    AOUT_VDDC_SENSE = 12,
    #[doc = "13: vdda_sw voltage connected on AOUT"]
    AOUT_VDDA_SW = 13,
    #[doc = "14: VDDA connected on AOUT (can be sensed for 4 wires measurement of the load regulation)"]
    AOUT_VDDA_SENSE = 14,
    #[doc = "15: VDDM connected on AOUT (can be sensed for 4 wires measurement of the load regulation)"]
    AOUT_VDDM_SENSE = 15,
    #[doc = "16: AOUT floating (for pad leakage measurement)"]
    AOUT_NC = 16,
    #[doc = "17: VDDPA connected on AOUT (can be sensed for 4 wires measurement of the load regulation)"]
    AOUT_VDDPA_SENSE = 17,
    #[doc = "18: VDDPA current sensing circuit connected to AOUT"]
    AOUT_VDDPA_ISENSE = 18,
    #[doc = "19: Flash TM0 connected to AOUT"]
    AOUT_TM0 = 19,
    #[doc = "20: Bandgap ready on AOUT (digital signal using VSSA and VCC states)"]
    AOUT_BG_READY = 20,
    #[doc = "21: vcc_ready on AOUT (digital signal using VSSA and VCC states)"]
    AOUT_VCC_READY = 21,
    #[doc = "22: dcdc_overload on AOUT (digital signal using VSSA and VCC states)"]
    AOUT_DCDC_OVERLOAD = 22,
    #[doc = "23: dcdc_activated on AOUT (digital signal using VSSA and VCC states)"]
    AOUT_DCDC_ACTIVATED = 23,
    #[doc = "24: vddrf_ready on AOUT (digital signal using VSSA and VCC states)"]
    AOUT_VDDRF_READY = 24,
    #[doc = "25: vddc_ready on AOUT (digital signal using VSSA and VCC states)"]
    AOUT_VDDC_READY = 25,
    #[doc = "26: vddm_ready on AOUT (digital signal using VSSA and VCC states)"]
    AOUT_VDDM_READY = 26,
    #[doc = "27: vdda_ready on AOUT (digital signal using VSSA and VCC states)"]
    AOUT_VDDA_READY = 27,
    #[doc = "28: Clock present from clock detector on AOUT (digital signal using VSSA and VCC states)"]
    AOUT_CLK_PRESENT = 28,
    #[doc = "29: XTAL ok on AOUT (digital signal using VSSA and VCC states)"]
    AOUT_XTAL_OK = 29,
    #[doc = "30: XTAL clock on AOUT (digital signal using VSSA and VCC states)"]
    AOUT_XTAL_CLK = 30,
    #[doc = "31: 32 kHz RC oscillator clock on AOUT (digital signal using VSSA and VCC states)"]
    AOUT_CLK_32K = 31,
}
impl From<TEST_AOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: TEST_AOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TEST_AOUT`"]
pub type TEST_AOUT_R = crate::R<u8, TEST_AOUT_A>;
impl TEST_AOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEST_AOUT_A {
        match self.bits {
            0 => TEST_AOUT_A::AOUT_VSSA,
            1 => TEST_AOUT_A::AOUT_VCC_SENSE,
            2 => TEST_AOUT_A::AOUT_VREF_0P75V_OUTPUT,
            3 => TEST_AOUT_A::AOUT_VREF_0P67V_OUTPUT,
            4 => TEST_AOUT_A::AOUT_IREF_50N_OUTPUT,
            5 => TEST_AOUT_A::AOUT_IREF_1N_OUTPUT,
            6 => TEST_AOUT_A::AOUT_VDDACS_OUTPUT,
            7 => TEST_AOUT_A::AOUT_VREF_0P75V_BUF_OUTPUT,
            8 => TEST_AOUT_A::AOUT_VREG_BG,
            9 => TEST_AOUT_A::AOUT_VDDRF_SW,
            10 => TEST_AOUT_A::AOUT_VDDRF_SENSE,
            11 => TEST_AOUT_A::AOUT_VDDT,
            12 => TEST_AOUT_A::AOUT_VDDC_SENSE,
            13 => TEST_AOUT_A::AOUT_VDDA_SW,
            14 => TEST_AOUT_A::AOUT_VDDA_SENSE,
            15 => TEST_AOUT_A::AOUT_VDDM_SENSE,
            16 => TEST_AOUT_A::AOUT_NC,
            17 => TEST_AOUT_A::AOUT_VDDPA_SENSE,
            18 => TEST_AOUT_A::AOUT_VDDPA_ISENSE,
            19 => TEST_AOUT_A::AOUT_TM0,
            20 => TEST_AOUT_A::AOUT_BG_READY,
            21 => TEST_AOUT_A::AOUT_VCC_READY,
            22 => TEST_AOUT_A::AOUT_DCDC_OVERLOAD,
            23 => TEST_AOUT_A::AOUT_DCDC_ACTIVATED,
            24 => TEST_AOUT_A::AOUT_VDDRF_READY,
            25 => TEST_AOUT_A::AOUT_VDDC_READY,
            26 => TEST_AOUT_A::AOUT_VDDM_READY,
            27 => TEST_AOUT_A::AOUT_VDDA_READY,
            28 => TEST_AOUT_A::AOUT_CLK_PRESENT,
            29 => TEST_AOUT_A::AOUT_XTAL_OK,
            30 => TEST_AOUT_A::AOUT_XTAL_CLK,
            31 => TEST_AOUT_A::AOUT_CLK_32K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AOUT_VSSA`"]
    #[inline(always)]
    pub fn is_aout_vssa(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VSSA
    }
    #[doc = "Checks if the value of the field is `AOUT_VCC_SENSE`"]
    #[inline(always)]
    pub fn is_aout_vcc_sense(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VCC_SENSE
    }
    #[doc = "Checks if the value of the field is `AOUT_VREF_0P75V_OUTPUT`"]
    #[inline(always)]
    pub fn is_aout_vref_0p75v_output(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VREF_0P75V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AOUT_VREF_0P67V_OUTPUT`"]
    #[inline(always)]
    pub fn is_aout_vref_0p67v_output(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VREF_0P67V_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AOUT_IREF_50N_OUTPUT`"]
    #[inline(always)]
    pub fn is_aout_iref_50n_output(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_IREF_50N_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AOUT_IREF_1N_OUTPUT`"]
    #[inline(always)]
    pub fn is_aout_iref_1n_output(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_IREF_1N_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AOUT_VDDACS_OUTPUT`"]
    #[inline(always)]
    pub fn is_aout_vddacs_output(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VDDACS_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AOUT_VREF_0P75V_BUF_OUTPUT`"]
    #[inline(always)]
    pub fn is_aout_vref_0p75v_buf_output(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VREF_0P75V_BUF_OUTPUT
    }
    #[doc = "Checks if the value of the field is `AOUT_VREG_BG`"]
    #[inline(always)]
    pub fn is_aout_vreg_bg(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VREG_BG
    }
    #[doc = "Checks if the value of the field is `AOUT_VDDRF_SW`"]
    #[inline(always)]
    pub fn is_aout_vddrf_sw(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VDDRF_SW
    }
    #[doc = "Checks if the value of the field is `AOUT_VDDRF_SENSE`"]
    #[inline(always)]
    pub fn is_aout_vddrf_sense(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VDDRF_SENSE
    }
    #[doc = "Checks if the value of the field is `AOUT_VDDT`"]
    #[inline(always)]
    pub fn is_aout_vddt(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VDDT
    }
    #[doc = "Checks if the value of the field is `AOUT_VDDC_SENSE`"]
    #[inline(always)]
    pub fn is_aout_vddc_sense(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VDDC_SENSE
    }
    #[doc = "Checks if the value of the field is `AOUT_VDDA_SW`"]
    #[inline(always)]
    pub fn is_aout_vdda_sw(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VDDA_SW
    }
    #[doc = "Checks if the value of the field is `AOUT_VDDA_SENSE`"]
    #[inline(always)]
    pub fn is_aout_vdda_sense(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VDDA_SENSE
    }
    #[doc = "Checks if the value of the field is `AOUT_VDDM_SENSE`"]
    #[inline(always)]
    pub fn is_aout_vddm_sense(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VDDM_SENSE
    }
    #[doc = "Checks if the value of the field is `AOUT_NC`"]
    #[inline(always)]
    pub fn is_aout_nc(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_NC
    }
    #[doc = "Checks if the value of the field is `AOUT_VDDPA_SENSE`"]
    #[inline(always)]
    pub fn is_aout_vddpa_sense(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VDDPA_SENSE
    }
    #[doc = "Checks if the value of the field is `AOUT_VDDPA_ISENSE`"]
    #[inline(always)]
    pub fn is_aout_vddpa_isense(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VDDPA_ISENSE
    }
    #[doc = "Checks if the value of the field is `AOUT_TM0`"]
    #[inline(always)]
    pub fn is_aout_tm0(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_TM0
    }
    #[doc = "Checks if the value of the field is `AOUT_BG_READY`"]
    #[inline(always)]
    pub fn is_aout_bg_ready(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_BG_READY
    }
    #[doc = "Checks if the value of the field is `AOUT_VCC_READY`"]
    #[inline(always)]
    pub fn is_aout_vcc_ready(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VCC_READY
    }
    #[doc = "Checks if the value of the field is `AOUT_DCDC_OVERLOAD`"]
    #[inline(always)]
    pub fn is_aout_dcdc_overload(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_DCDC_OVERLOAD
    }
    #[doc = "Checks if the value of the field is `AOUT_DCDC_ACTIVATED`"]
    #[inline(always)]
    pub fn is_aout_dcdc_activated(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_DCDC_ACTIVATED
    }
    #[doc = "Checks if the value of the field is `AOUT_VDDRF_READY`"]
    #[inline(always)]
    pub fn is_aout_vddrf_ready(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VDDRF_READY
    }
    #[doc = "Checks if the value of the field is `AOUT_VDDC_READY`"]
    #[inline(always)]
    pub fn is_aout_vddc_ready(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VDDC_READY
    }
    #[doc = "Checks if the value of the field is `AOUT_VDDM_READY`"]
    #[inline(always)]
    pub fn is_aout_vddm_ready(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VDDM_READY
    }
    #[doc = "Checks if the value of the field is `AOUT_VDDA_READY`"]
    #[inline(always)]
    pub fn is_aout_vdda_ready(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_VDDA_READY
    }
    #[doc = "Checks if the value of the field is `AOUT_CLK_PRESENT`"]
    #[inline(always)]
    pub fn is_aout_clk_present(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_CLK_PRESENT
    }
    #[doc = "Checks if the value of the field is `AOUT_XTAL_OK`"]
    #[inline(always)]
    pub fn is_aout_xtal_ok(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_XTAL_OK
    }
    #[doc = "Checks if the value of the field is `AOUT_XTAL_CLK`"]
    #[inline(always)]
    pub fn is_aout_xtal_clk(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_XTAL_CLK
    }
    #[doc = "Checks if the value of the field is `AOUT_CLK_32K`"]
    #[inline(always)]
    pub fn is_aout_clk_32k(&self) -> bool {
        *self == TEST_AOUT_A::AOUT_CLK_32K
    }
}
#[doc = "Write proxy for field `TEST_AOUT`"]
pub struct TEST_AOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_AOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEST_AOUT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "AOUT grounded"]
    #[inline(always)]
    pub fn aout_vssa(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VSSA)
    }
    #[doc = "AOUT high / VCC connected on AOUT (can be sensed for 4 wires measurement of the load regulation)"]
    #[inline(always)]
    pub fn aout_vcc_sense(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VCC_SENSE)
    }
    #[doc = "Bandgap reference voltage 0p75V connected on AOUT"]
    #[inline(always)]
    pub fn aout_vref_0p75v_output(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VREF_0P75V_OUTPUT)
    }
    #[doc = "Bandgap reference voltage 0p67V connected on AOUT"]
    #[inline(always)]
    pub fn aout_vref_0p67v_output(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VREF_0P67V_OUTPUT)
    }
    #[doc = "Bandgap iref current source connected on AOUT"]
    #[inline(always)]
    pub fn aout_iref_50n_output(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_IREF_50N_OUTPUT)
    }
    #[doc = "PTAT iref current source connected on AOUT"]
    #[inline(always)]
    pub fn aout_iref_1n_output(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_IREF_1N_OUTPUT)
    }
    #[doc = "vddacs voltage connected on AOUT"]
    #[inline(always)]
    pub fn aout_vddacs_output(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VDDACS_OUTPUT)
    }
    #[doc = "Bandgap buffered reference voltage 0p75V connected on AOUT"]
    #[inline(always)]
    pub fn aout_vref_0p75v_buf_output(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VREF_0P75V_BUF_OUTPUT)
    }
    #[doc = "Bandgap regulated supply voltage"]
    #[inline(always)]
    pub fn aout_vreg_bg(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VREG_BG)
    }
    #[doc = "vddrf_sw voltage connected on AOUT"]
    #[inline(always)]
    pub fn aout_vddrf_sw(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VDDRF_SW)
    }
    #[doc = "VDDRF connected on AOUT (can be sensed for 4 wires measurement of the load regulation)"]
    #[inline(always)]
    pub fn aout_vddrf_sense(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VDDRF_SENSE)
    }
    #[doc = "Baseband timer supply voltage"]
    #[inline(always)]
    pub fn aout_vddt(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VDDT)
    }
    #[doc = "VDDC connected on AOUT (can be sensed for 4 wires measurement of the load regulation)"]
    #[inline(always)]
    pub fn aout_vddc_sense(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VDDC_SENSE)
    }
    #[doc = "vdda_sw voltage connected on AOUT"]
    #[inline(always)]
    pub fn aout_vdda_sw(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VDDA_SW)
    }
    #[doc = "VDDA connected on AOUT (can be sensed for 4 wires measurement of the load regulation)"]
    #[inline(always)]
    pub fn aout_vdda_sense(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VDDA_SENSE)
    }
    #[doc = "VDDM connected on AOUT (can be sensed for 4 wires measurement of the load regulation)"]
    #[inline(always)]
    pub fn aout_vddm_sense(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VDDM_SENSE)
    }
    #[doc = "AOUT floating (for pad leakage measurement)"]
    #[inline(always)]
    pub fn aout_nc(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_NC)
    }
    #[doc = "VDDPA connected on AOUT (can be sensed for 4 wires measurement of the load regulation)"]
    #[inline(always)]
    pub fn aout_vddpa_sense(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VDDPA_SENSE)
    }
    #[doc = "VDDPA current sensing circuit connected to AOUT"]
    #[inline(always)]
    pub fn aout_vddpa_isense(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VDDPA_ISENSE)
    }
    #[doc = "Flash TM0 connected to AOUT"]
    #[inline(always)]
    pub fn aout_tm0(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_TM0)
    }
    #[doc = "Bandgap ready on AOUT (digital signal using VSSA and VCC states)"]
    #[inline(always)]
    pub fn aout_bg_ready(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_BG_READY)
    }
    #[doc = "vcc_ready on AOUT (digital signal using VSSA and VCC states)"]
    #[inline(always)]
    pub fn aout_vcc_ready(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VCC_READY)
    }
    #[doc = "dcdc_overload on AOUT (digital signal using VSSA and VCC states)"]
    #[inline(always)]
    pub fn aout_dcdc_overload(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_DCDC_OVERLOAD)
    }
    #[doc = "dcdc_activated on AOUT (digital signal using VSSA and VCC states)"]
    #[inline(always)]
    pub fn aout_dcdc_activated(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_DCDC_ACTIVATED)
    }
    #[doc = "vddrf_ready on AOUT (digital signal using VSSA and VCC states)"]
    #[inline(always)]
    pub fn aout_vddrf_ready(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VDDRF_READY)
    }
    #[doc = "vddc_ready on AOUT (digital signal using VSSA and VCC states)"]
    #[inline(always)]
    pub fn aout_vddc_ready(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VDDC_READY)
    }
    #[doc = "vddm_ready on AOUT (digital signal using VSSA and VCC states)"]
    #[inline(always)]
    pub fn aout_vddm_ready(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VDDM_READY)
    }
    #[doc = "vdda_ready on AOUT (digital signal using VSSA and VCC states)"]
    #[inline(always)]
    pub fn aout_vdda_ready(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_VDDA_READY)
    }
    #[doc = "Clock present from clock detector on AOUT (digital signal using VSSA and VCC states)"]
    #[inline(always)]
    pub fn aout_clk_present(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_CLK_PRESENT)
    }
    #[doc = "XTAL ok on AOUT (digital signal using VSSA and VCC states)"]
    #[inline(always)]
    pub fn aout_xtal_ok(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_XTAL_OK)
    }
    #[doc = "XTAL clock on AOUT (digital signal using VSSA and VCC states)"]
    #[inline(always)]
    pub fn aout_xtal_clk(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_XTAL_CLK)
    }
    #[doc = "32 kHz RC oscillator clock on AOUT (digital signal using VSSA and VCC states)"]
    #[inline(always)]
    pub fn aout_clk_32k(self) -> &'a mut W {
        self.variant(TEST_AOUT_A::AOUT_CLK_32K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 13 - Stop edge for RTC clock output on AOUT"]
    #[inline(always)]
    pub fn rtc_clock_dio0_stop_edge(&self) -> RTC_CLOCK_DIO0_STOP_EDGE_R {
        RTC_CLOCK_DIO0_STOP_EDGE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - Stop source for RTC clock output on AOUT"]
    #[inline(always)]
    pub fn rtc_clock_dio0_stop_src(&self) -> RTC_CLOCK_DIO0_STOP_SRC_R {
        RTC_CLOCK_DIO0_STOP_SRC_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - Start event for RTC clock output on AOUT (RTC prescaler and counter need to be enabled)"]
    #[inline(always)]
    pub fn rtc_clock_dio0_start(&self) -> RTC_CLOCK_DIO0_START_R {
        RTC_CLOCK_DIO0_START_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:4 - AOUT test signal selection"]
    #[inline(always)]
    pub fn test_aout(&self) -> TEST_AOUT_R {
        TEST_AOUT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 13 - Stop edge for RTC clock output on AOUT"]
    #[inline(always)]
    pub fn rtc_clock_dio0_stop_edge(&mut self) -> RTC_CLOCK_DIO0_STOP_EDGE_W {
        RTC_CLOCK_DIO0_STOP_EDGE_W { w: self }
    }
    #[doc = "Bits 11:12 - Stop source for RTC clock output on AOUT"]
    #[inline(always)]
    pub fn rtc_clock_dio0_stop_src(&mut self) -> RTC_CLOCK_DIO0_STOP_SRC_W {
        RTC_CLOCK_DIO0_STOP_SRC_W { w: self }
    }
    #[doc = "Bits 8:10 - Start event for RTC clock output on AOUT (RTC prescaler and counter need to be enabled)"]
    #[inline(always)]
    pub fn rtc_clock_dio0_start(&mut self) -> RTC_CLOCK_DIO0_START_W {
        RTC_CLOCK_DIO0_START_W { w: self }
    }
    #[doc = "Bits 0:4 - AOUT test signal selection"]
    #[inline(always)]
    pub fn test_aout(&mut self) -> TEST_AOUT_W {
        TEST_AOUT_W { w: self }
    }
}
