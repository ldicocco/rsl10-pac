#[doc = "Reader of register TIMER_CFG[%s]"]
pub type R = crate::R<u32, super::TIMER_CFG>;
#[doc = "Writer for register TIMER_CFG[%s]"]
pub type W = crate::W<u32, super::TIMER_CFG>;
#[doc = "Register TIMER_CFG[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::TIMER_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Multi-count value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MULTI_COUNT_A {
    #[doc = "0: Stop on 1st Time-out occurrence and issue an interrupt"]
    TIMER_MULTI_COUNT_1 = 0,
    #[doc = "1: Stop on 2nd Time-out occurrence and issue an interrupt"]
    TIMER_MULTI_COUNT_2 = 1,
    #[doc = "2: Stop on 3rd Time-out occurrence and issue an interrupt"]
    TIMER_MULTI_COUNT_3 = 2,
    #[doc = "3: Stop on 4th Time-out occurrence and issue an interrupt"]
    TIMER_MULTI_COUNT_4 = 3,
    #[doc = "4: Stop on 5th Time-out occurrence and issue an interrupt"]
    TIMER_MULTI_COUNT_5 = 4,
    #[doc = "5: Stop on 6th Time-out occurrence and issue an interrupt"]
    TIMER_MULTI_COUNT_6 = 5,
    #[doc = "6: Stop on 7th Time-out occurrence and issue an interrupt"]
    TIMER_MULTI_COUNT_7 = 6,
    #[doc = "7: Stop on 8th Time-out occurrence and issue an interrupt"]
    TIMER_MULTI_COUNT_8 = 7,
}
impl From<MULTI_COUNT_A> for u8 {
    #[inline(always)]
    fn from(variant: MULTI_COUNT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MULTI_COUNT`"]
pub type MULTI_COUNT_R = crate::R<u8, MULTI_COUNT_A>;
impl MULTI_COUNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MULTI_COUNT_A {
        match self.bits {
            0 => MULTI_COUNT_A::TIMER_MULTI_COUNT_1,
            1 => MULTI_COUNT_A::TIMER_MULTI_COUNT_2,
            2 => MULTI_COUNT_A::TIMER_MULTI_COUNT_3,
            3 => MULTI_COUNT_A::TIMER_MULTI_COUNT_4,
            4 => MULTI_COUNT_A::TIMER_MULTI_COUNT_5,
            5 => MULTI_COUNT_A::TIMER_MULTI_COUNT_6,
            6 => MULTI_COUNT_A::TIMER_MULTI_COUNT_7,
            7 => MULTI_COUNT_A::TIMER_MULTI_COUNT_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_MULTI_COUNT_1`"]
    #[inline(always)]
    pub fn is_timer_multi_count_1(&self) -> bool {
        *self == MULTI_COUNT_A::TIMER_MULTI_COUNT_1
    }
    #[doc = "Checks if the value of the field is `TIMER_MULTI_COUNT_2`"]
    #[inline(always)]
    pub fn is_timer_multi_count_2(&self) -> bool {
        *self == MULTI_COUNT_A::TIMER_MULTI_COUNT_2
    }
    #[doc = "Checks if the value of the field is `TIMER_MULTI_COUNT_3`"]
    #[inline(always)]
    pub fn is_timer_multi_count_3(&self) -> bool {
        *self == MULTI_COUNT_A::TIMER_MULTI_COUNT_3
    }
    #[doc = "Checks if the value of the field is `TIMER_MULTI_COUNT_4`"]
    #[inline(always)]
    pub fn is_timer_multi_count_4(&self) -> bool {
        *self == MULTI_COUNT_A::TIMER_MULTI_COUNT_4
    }
    #[doc = "Checks if the value of the field is `TIMER_MULTI_COUNT_5`"]
    #[inline(always)]
    pub fn is_timer_multi_count_5(&self) -> bool {
        *self == MULTI_COUNT_A::TIMER_MULTI_COUNT_5
    }
    #[doc = "Checks if the value of the field is `TIMER_MULTI_COUNT_6`"]
    #[inline(always)]
    pub fn is_timer_multi_count_6(&self) -> bool {
        *self == MULTI_COUNT_A::TIMER_MULTI_COUNT_6
    }
    #[doc = "Checks if the value of the field is `TIMER_MULTI_COUNT_7`"]
    #[inline(always)]
    pub fn is_timer_multi_count_7(&self) -> bool {
        *self == MULTI_COUNT_A::TIMER_MULTI_COUNT_7
    }
    #[doc = "Checks if the value of the field is `TIMER_MULTI_COUNT_8`"]
    #[inline(always)]
    pub fn is_timer_multi_count_8(&self) -> bool {
        *self == MULTI_COUNT_A::TIMER_MULTI_COUNT_8
    }
}
#[doc = "Write proxy for field `MULTI_COUNT`"]
pub struct MULTI_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MULTI_COUNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MULTI_COUNT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Stop on 1st Time-out occurrence and issue an interrupt"]
    #[inline(always)]
    pub fn timer_multi_count_1(self) -> &'a mut W {
        self.variant(MULTI_COUNT_A::TIMER_MULTI_COUNT_1)
    }
    #[doc = "Stop on 2nd Time-out occurrence and issue an interrupt"]
    #[inline(always)]
    pub fn timer_multi_count_2(self) -> &'a mut W {
        self.variant(MULTI_COUNT_A::TIMER_MULTI_COUNT_2)
    }
    #[doc = "Stop on 3rd Time-out occurrence and issue an interrupt"]
    #[inline(always)]
    pub fn timer_multi_count_3(self) -> &'a mut W {
        self.variant(MULTI_COUNT_A::TIMER_MULTI_COUNT_3)
    }
    #[doc = "Stop on 4th Time-out occurrence and issue an interrupt"]
    #[inline(always)]
    pub fn timer_multi_count_4(self) -> &'a mut W {
        self.variant(MULTI_COUNT_A::TIMER_MULTI_COUNT_4)
    }
    #[doc = "Stop on 5th Time-out occurrence and issue an interrupt"]
    #[inline(always)]
    pub fn timer_multi_count_5(self) -> &'a mut W {
        self.variant(MULTI_COUNT_A::TIMER_MULTI_COUNT_5)
    }
    #[doc = "Stop on 6th Time-out occurrence and issue an interrupt"]
    #[inline(always)]
    pub fn timer_multi_count_6(self) -> &'a mut W {
        self.variant(MULTI_COUNT_A::TIMER_MULTI_COUNT_6)
    }
    #[doc = "Stop on 7th Time-out occurrence and issue an interrupt"]
    #[inline(always)]
    pub fn timer_multi_count_7(self) -> &'a mut W {
        self.variant(MULTI_COUNT_A::TIMER_MULTI_COUNT_7)
    }
    #[doc = "Stop on 8th Time-out occurrence and issue an interrupt"]
    #[inline(always)]
    pub fn timer_multi_count_8(self) -> &'a mut W {
        self.variant(MULTI_COUNT_A::TIMER_MULTI_COUNT_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
#[doc = "Timer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Enable the one shot / multi shot mode"]
    TIMER_SHOT_MODE = 0,
    #[doc = "1: Enable the free-run mode"]
    TIMER_FREE_RUN = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::TIMER_SHOT_MODE,
            true => MODE_A::TIMER_FREE_RUN,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SHOT_MODE`"]
    #[inline(always)]
    pub fn is_timer_shot_mode(&self) -> bool {
        *self == MODE_A::TIMER_SHOT_MODE
    }
    #[doc = "Checks if the value of the field is `TIMER_FREE_RUN`"]
    #[inline(always)]
    pub fn is_timer_free_run(&self) -> bool {
        *self == MODE_A::TIMER_FREE_RUN
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the one shot / multi shot mode"]
    #[inline(always)]
    pub fn timer_shot_mode(self) -> &'a mut W {
        self.variant(MODE_A::TIMER_SHOT_MODE)
    }
    #[doc = "Enable the free-run mode"]
    #[inline(always)]
    pub fn timer_free_run(self) -> &'a mut W {
        self.variant(MODE_A::TIMER_FREE_RUN)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_SRC_A {
    #[doc = "0: Timer clock source is SLOWCLK divided by 32"]
    TIMER_SLOWCLK_DIV32 = 0,
    #[doc = "1: Timer clock source is SLOWCLK divided by 2"]
    TIMER_SLOWCLK_DIV2 = 1,
}
impl From<CLK_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLK_SRC`"]
pub type CLK_SRC_R = crate::R<bool, CLK_SRC_A>;
impl CLK_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_SRC_A {
        match self.bits {
            false => CLK_SRC_A::TIMER_SLOWCLK_DIV32,
            true => CLK_SRC_A::TIMER_SLOWCLK_DIV2,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SLOWCLK_DIV32`"]
    #[inline(always)]
    pub fn is_timer_slowclk_div32(&self) -> bool {
        *self == CLK_SRC_A::TIMER_SLOWCLK_DIV32
    }
    #[doc = "Checks if the value of the field is `TIMER_SLOWCLK_DIV2`"]
    #[inline(always)]
    pub fn is_timer_slowclk_div2(&self) -> bool {
        *self == CLK_SRC_A::TIMER_SLOWCLK_DIV2
    }
}
#[doc = "Write proxy for field `CLK_SRC`"]
pub struct CLK_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer clock source is SLOWCLK divided by 32"]
    #[inline(always)]
    pub fn timer_slowclk_div32(self) -> &'a mut W {
        self.variant(CLK_SRC_A::TIMER_SLOWCLK_DIV32)
    }
    #[doc = "Timer clock source is SLOWCLK divided by 2"]
    #[inline(always)]
    pub fn timer_slowclk_div2(self) -> &'a mut W {
        self.variant(CLK_SRC_A::TIMER_SLOWCLK_DIV2)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Prescale value of the timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALE_A {
    #[doc = "0: Divide the input clock frequency by 1"]
    TIMER_PRESCALE_1 = 0,
    #[doc = "1: Divide the input clock frequency by 2"]
    TIMER_PRESCALE_2 = 1,
    #[doc = "2: Divide the input clock frequency by 4"]
    TIMER_PRESCALE_4 = 2,
    #[doc = "3: Divide the input clock frequency by 8"]
    TIMER_PRESCALE_8 = 3,
    #[doc = "4: Divide the input clock frequency by 16"]
    TIMER_PRESCALE_16 = 4,
    #[doc = "5: Divide the input clock frequency by 32"]
    TIMER_PRESCALE_32 = 5,
    #[doc = "6: Divide the input clock frequency by 64"]
    TIMER_PRESCALE_64 = 6,
    #[doc = "7: Divide the input clock frequency by 128"]
    TIMER_PRESCALE_128 = 7,
}
impl From<PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESCALE`"]
pub type PRESCALE_R = crate::R<u8, PRESCALE_A>;
impl PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCALE_A {
        match self.bits {
            0 => PRESCALE_A::TIMER_PRESCALE_1,
            1 => PRESCALE_A::TIMER_PRESCALE_2,
            2 => PRESCALE_A::TIMER_PRESCALE_4,
            3 => PRESCALE_A::TIMER_PRESCALE_8,
            4 => PRESCALE_A::TIMER_PRESCALE_16,
            5 => PRESCALE_A::TIMER_PRESCALE_32,
            6 => PRESCALE_A::TIMER_PRESCALE_64,
            7 => PRESCALE_A::TIMER_PRESCALE_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_PRESCALE_1`"]
    #[inline(always)]
    pub fn is_timer_prescale_1(&self) -> bool {
        *self == PRESCALE_A::TIMER_PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `TIMER_PRESCALE_2`"]
    #[inline(always)]
    pub fn is_timer_prescale_2(&self) -> bool {
        *self == PRESCALE_A::TIMER_PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `TIMER_PRESCALE_4`"]
    #[inline(always)]
    pub fn is_timer_prescale_4(&self) -> bool {
        *self == PRESCALE_A::TIMER_PRESCALE_4
    }
    #[doc = "Checks if the value of the field is `TIMER_PRESCALE_8`"]
    #[inline(always)]
    pub fn is_timer_prescale_8(&self) -> bool {
        *self == PRESCALE_A::TIMER_PRESCALE_8
    }
    #[doc = "Checks if the value of the field is `TIMER_PRESCALE_16`"]
    #[inline(always)]
    pub fn is_timer_prescale_16(&self) -> bool {
        *self == PRESCALE_A::TIMER_PRESCALE_16
    }
    #[doc = "Checks if the value of the field is `TIMER_PRESCALE_32`"]
    #[inline(always)]
    pub fn is_timer_prescale_32(&self) -> bool {
        *self == PRESCALE_A::TIMER_PRESCALE_32
    }
    #[doc = "Checks if the value of the field is `TIMER_PRESCALE_64`"]
    #[inline(always)]
    pub fn is_timer_prescale_64(&self) -> bool {
        *self == PRESCALE_A::TIMER_PRESCALE_64
    }
    #[doc = "Checks if the value of the field is `TIMER_PRESCALE_128`"]
    #[inline(always)]
    pub fn is_timer_prescale_128(&self) -> bool {
        *self == PRESCALE_A::TIMER_PRESCALE_128
    }
}
#[doc = "Write proxy for field `PRESCALE`"]
pub struct PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide the input clock frequency by 1"]
    #[inline(always)]
    pub fn timer_prescale_1(self) -> &'a mut W {
        self.variant(PRESCALE_A::TIMER_PRESCALE_1)
    }
    #[doc = "Divide the input clock frequency by 2"]
    #[inline(always)]
    pub fn timer_prescale_2(self) -> &'a mut W {
        self.variant(PRESCALE_A::TIMER_PRESCALE_2)
    }
    #[doc = "Divide the input clock frequency by 4"]
    #[inline(always)]
    pub fn timer_prescale_4(self) -> &'a mut W {
        self.variant(PRESCALE_A::TIMER_PRESCALE_4)
    }
    #[doc = "Divide the input clock frequency by 8"]
    #[inline(always)]
    pub fn timer_prescale_8(self) -> &'a mut W {
        self.variant(PRESCALE_A::TIMER_PRESCALE_8)
    }
    #[doc = "Divide the input clock frequency by 16"]
    #[inline(always)]
    pub fn timer_prescale_16(self) -> &'a mut W {
        self.variant(PRESCALE_A::TIMER_PRESCALE_16)
    }
    #[doc = "Divide the input clock frequency by 32"]
    #[inline(always)]
    pub fn timer_prescale_32(self) -> &'a mut W {
        self.variant(PRESCALE_A::TIMER_PRESCALE_32)
    }
    #[doc = "Divide the input clock frequency by 64"]
    #[inline(always)]
    pub fn timer_prescale_64(self) -> &'a mut W {
        self.variant(PRESCALE_A::TIMER_PRESCALE_64)
    }
    #[doc = "Divide the input clock frequency by 128"]
    #[inline(always)]
    pub fn timer_prescale_128(self) -> &'a mut W {
        self.variant(PRESCALE_A::TIMER_PRESCALE_128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `TIMEOUT_VALUE`"]
pub type TIMEOUT_VALUE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TIMEOUT_VALUE`"]
pub struct TIMEOUT_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31 - Multi-count value"]
    #[inline(always)]
    pub fn multi_count(&self) -> MULTI_COUNT_R {
        MULTI_COUNT_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bit 28 - Timer mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Clock source"]
    #[inline(always)]
    pub fn clk_src(&self) -> CLK_SRC_R {
        CLK_SRC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Prescale value of the timer"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 0:23 - Number of Timer clock cycles to time-out"]
    #[inline(always)]
    pub fn timeout_value(&self) -> TIMEOUT_VALUE_R {
        TIMEOUT_VALUE_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 29:31 - Multi-count value"]
    #[inline(always)]
    pub fn multi_count(&mut self) -> MULTI_COUNT_W {
        MULTI_COUNT_W { w: self }
    }
    #[doc = "Bit 28 - Timer mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 27 - Clock source"]
    #[inline(always)]
    pub fn clk_src(&mut self) -> CLK_SRC_W {
        CLK_SRC_W { w: self }
    }
    #[doc = "Bits 24:26 - Prescale value of the timer"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W {
        PRESCALE_W { w: self }
    }
    #[doc = "Bits 0:23 - Number of Timer clock cycles to time-out"]
    #[inline(always)]
    pub fn timeout_value(&mut self) -> TIMEOUT_VALUE_W {
        TIMEOUT_VALUE_W { w: self }
    }
}
