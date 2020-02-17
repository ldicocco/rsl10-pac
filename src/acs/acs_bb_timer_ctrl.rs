#[doc = "Reader of register ACS_BB_TIMER_CTRL"]
pub type R = crate::R<u32, super::ACS_BB_TIMER_CTRL>;
#[doc = "Writer for register ACS_BB_TIMER_CTRL"]
pub type W = crate::W<u32, super::ACS_BB_TIMER_CTRL>;
#[doc = "Register ACS_BB_TIMER_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ACS_BB_TIMER_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Prescale value for the baseband timer clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BB_CLK_PRESCALE_A {
    #[doc = "0: Use 32 kHz clock from RTC"]
    BB_CLK_PRESCALE_1 = 0,
    #[doc = "1: Use 16 kHz clock from RTC (counter bit0)"]
    BB_CLK_PRESCALE_2 = 1,
    #[doc = "2: Use 8 kHz clock from RTC (counter bit1)"]
    BB_CLK_PRESCALE_4 = 2,
    #[doc = "3: Use 4 kHz clock from RTC (counter bit2)"]
    BB_CLK_PRESCALE_8 = 3,
}
impl From<BB_CLK_PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: BB_CLK_PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BB_CLK_PRESCALE`"]
pub type BB_CLK_PRESCALE_R = crate::R<u8, BB_CLK_PRESCALE_A>;
impl BB_CLK_PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_CLK_PRESCALE_A {
        match self.bits {
            0 => BB_CLK_PRESCALE_A::BB_CLK_PRESCALE_1,
            1 => BB_CLK_PRESCALE_A::BB_CLK_PRESCALE_2,
            2 => BB_CLK_PRESCALE_A::BB_CLK_PRESCALE_4,
            3 => BB_CLK_PRESCALE_A::BB_CLK_PRESCALE_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BB_CLK_PRESCALE_1`"]
    #[inline(always)]
    pub fn is_bb_clk_prescale_1(&self) -> bool {
        *self == BB_CLK_PRESCALE_A::BB_CLK_PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `BB_CLK_PRESCALE_2`"]
    #[inline(always)]
    pub fn is_bb_clk_prescale_2(&self) -> bool {
        *self == BB_CLK_PRESCALE_A::BB_CLK_PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `BB_CLK_PRESCALE_4`"]
    #[inline(always)]
    pub fn is_bb_clk_prescale_4(&self) -> bool {
        *self == BB_CLK_PRESCALE_A::BB_CLK_PRESCALE_4
    }
    #[doc = "Checks if the value of the field is `BB_CLK_PRESCALE_8`"]
    #[inline(always)]
    pub fn is_bb_clk_prescale_8(&self) -> bool {
        *self == BB_CLK_PRESCALE_A::BB_CLK_PRESCALE_8
    }
}
#[doc = "Write proxy for field `BB_CLK_PRESCALE`"]
pub struct BB_CLK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_CLK_PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_CLK_PRESCALE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Use 32 kHz clock from RTC"]
    #[inline(always)]
    pub fn bb_clk_prescale_1(self) -> &'a mut W {
        self.variant(BB_CLK_PRESCALE_A::BB_CLK_PRESCALE_1)
    }
    #[doc = "Use 16 kHz clock from RTC (counter bit0)"]
    #[inline(always)]
    pub fn bb_clk_prescale_2(self) -> &'a mut W {
        self.variant(BB_CLK_PRESCALE_A::BB_CLK_PRESCALE_2)
    }
    #[doc = "Use 8 kHz clock from RTC (counter bit1)"]
    #[inline(always)]
    pub fn bb_clk_prescale_4(self) -> &'a mut W {
        self.variant(BB_CLK_PRESCALE_A::BB_CLK_PRESCALE_4)
    }
    #[doc = "Use 4 kHz clock from RTC (counter bit2)"]
    #[inline(always)]
    pub fn bb_clk_prescale_8(self) -> &'a mut W {
        self.variant(BB_CLK_PRESCALE_A::BB_CLK_PRESCALE_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "nReset signal for the baseband timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_TIMER_NRESET_A {
    #[doc = "0: Baseband timer is in reset state"]
    BB_TIMER_RESET = 0,
    #[doc = "1: Baseband timer reset is released"]
    BB_TIMER_NRESET = 1,
}
impl From<BB_TIMER_NRESET_A> for bool {
    #[inline(always)]
    fn from(variant: BB_TIMER_NRESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BB_TIMER_NRESET`"]
pub type BB_TIMER_NRESET_R = crate::R<bool, BB_TIMER_NRESET_A>;
impl BB_TIMER_NRESET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_TIMER_NRESET_A {
        match self.bits {
            false => BB_TIMER_NRESET_A::BB_TIMER_RESET,
            true => BB_TIMER_NRESET_A::BB_TIMER_NRESET,
        }
    }
    #[doc = "Checks if the value of the field is `BB_TIMER_RESET`"]
    #[inline(always)]
    pub fn is_bb_timer_reset(&self) -> bool {
        *self == BB_TIMER_NRESET_A::BB_TIMER_RESET
    }
    #[doc = "Checks if the value of the field is `BB_TIMER_NRESET`"]
    #[inline(always)]
    pub fn is_bb_timer_nreset(&self) -> bool {
        *self == BB_TIMER_NRESET_A::BB_TIMER_NRESET
    }
}
#[doc = "Write proxy for field `BB_TIMER_NRESET`"]
pub struct BB_TIMER_NRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_TIMER_NRESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_TIMER_NRESET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Baseband timer is in reset state"]
    #[inline(always)]
    pub fn bb_timer_reset(self) -> &'a mut W {
        self.variant(BB_TIMER_NRESET_A::BB_TIMER_RESET)
    }
    #[doc = "Baseband timer reset is released"]
    #[inline(always)]
    pub fn bb_timer_nreset(self) -> &'a mut W {
        self.variant(BB_TIMER_NRESET_A::BB_TIMER_NRESET)
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
    #[doc = "Bits 8:9 - Prescale value for the baseband timer clock"]
    #[inline(always)]
    pub fn bb_clk_prescale(&self) -> BB_CLK_PRESCALE_R {
        BB_CLK_PRESCALE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 0 - nReset signal for the baseband timer"]
    #[inline(always)]
    pub fn bb_timer_nreset(&self) -> BB_TIMER_NRESET_R {
        BB_TIMER_NRESET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9 - Prescale value for the baseband timer clock"]
    #[inline(always)]
    pub fn bb_clk_prescale(&mut self) -> BB_CLK_PRESCALE_W {
        BB_CLK_PRESCALE_W { w: self }
    }
    #[doc = "Bit 0 - nReset signal for the baseband timer"]
    #[inline(always)]
    pub fn bb_timer_nreset(&mut self) -> BB_TIMER_NRESET_W {
        BB_TIMER_NRESET_W { w: self }
    }
}
