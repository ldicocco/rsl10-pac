#[doc = "Reader of register ACS_CLK_DET_CTRL"]
pub type R = crate::R<u32, super::ACS_CLK_DET_CTRL>;
#[doc = "Writer for register ACS_CLK_DET_CTRL"]
pub type W = crate::W<u32, super::ACS_CLK_DET_CTRL>;
#[doc = "Register ACS_CLK_DET_CTRL `reset()`'s with value 0x0101"]
impl crate::ResetValue for super::ACS_CLK_DET_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0101
    }
}
#[doc = "Clock present flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLOCK_PRESENT_A {
    #[doc = "0: No clock detected"]
    ACS_CLK_DET_NO_CLOCK = 0,
    #[doc = "1: Clock detected"]
    ACS_CLK_DET_CLOCK_PRESENT = 1,
}
impl From<CLOCK_PRESENT_A> for bool {
    #[inline(always)]
    fn from(variant: CLOCK_PRESENT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLOCK_PRESENT`"]
pub type CLOCK_PRESENT_R = crate::R<bool, CLOCK_PRESENT_A>;
impl CLOCK_PRESENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLOCK_PRESENT_A {
        match self.bits {
            false => CLOCK_PRESENT_A::ACS_CLK_DET_NO_CLOCK,
            true => CLOCK_PRESENT_A::ACS_CLK_DET_CLOCK_PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `ACS_CLK_DET_NO_CLOCK`"]
    #[inline(always)]
    pub fn is_acs_clk_det_no_clock(&self) -> bool {
        *self == CLOCK_PRESENT_A::ACS_CLK_DET_NO_CLOCK
    }
    #[doc = "Checks if the value of the field is `ACS_CLK_DET_CLOCK_PRESENT`"]
    #[inline(always)]
    pub fn is_acs_clk_det_clock_present(&self) -> bool {
        *self == CLOCK_PRESENT_A::ACS_CLK_DET_CLOCK_PRESENT
    }
}
#[doc = "Clock detector reset condition ignore\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_IGNORE_A {
    #[doc = "1: Clock detector reset condition ignore"]
    ACS_CLK_DET_RESET_DISABLE = 1,
    #[doc = "0: Clock detector reset condition accept"]
    ACS_CLK_DET_RESET_ENABLE = 0,
}
impl From<RESET_IGNORE_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_IGNORE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESET_IGNORE`"]
pub type RESET_IGNORE_R = crate::R<bool, RESET_IGNORE_A>;
impl RESET_IGNORE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_IGNORE_A {
        match self.bits {
            true => RESET_IGNORE_A::ACS_CLK_DET_RESET_DISABLE,
            false => RESET_IGNORE_A::ACS_CLK_DET_RESET_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ACS_CLK_DET_RESET_DISABLE`"]
    #[inline(always)]
    pub fn is_acs_clk_det_reset_disable(&self) -> bool {
        *self == RESET_IGNORE_A::ACS_CLK_DET_RESET_DISABLE
    }
    #[doc = "Checks if the value of the field is `ACS_CLK_DET_RESET_ENABLE`"]
    #[inline(always)]
    pub fn is_acs_clk_det_reset_enable(&self) -> bool {
        *self == RESET_IGNORE_A::ACS_CLK_DET_RESET_ENABLE
    }
}
#[doc = "Write proxy for field `RESET_IGNORE`"]
pub struct RESET_IGNORE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_IGNORE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESET_IGNORE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock detector reset condition ignore"]
    #[inline(always)]
    pub fn acs_clk_det_reset_disable(self) -> &'a mut W {
        self.variant(RESET_IGNORE_A::ACS_CLK_DET_RESET_DISABLE)
    }
    #[doc = "Clock detector reset condition accept"]
    #[inline(always)]
    pub fn acs_clk_det_reset_enable(self) -> &'a mut W {
        self.variant(RESET_IGNORE_A::ACS_CLK_DET_RESET_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Clock detector enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "1: Clock detector enable"]
    ACS_CLK_DET_ENABLE = 1,
    #[doc = "0: Clock detector disable"]
    ACS_CLK_DET_DISABLE = 0,
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
            true => ENABLE_A::ACS_CLK_DET_ENABLE,
            false => ENABLE_A::ACS_CLK_DET_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ACS_CLK_DET_ENABLE`"]
    #[inline(always)]
    pub fn is_acs_clk_det_enable(&self) -> bool {
        *self == ENABLE_A::ACS_CLK_DET_ENABLE
    }
    #[doc = "Checks if the value of the field is `ACS_CLK_DET_DISABLE`"]
    #[inline(always)]
    pub fn is_acs_clk_det_disable(&self) -> bool {
        *self == ENABLE_A::ACS_CLK_DET_DISABLE
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
    #[doc = "Clock detector enable"]
    #[inline(always)]
    pub fn acs_clk_det_enable(self) -> &'a mut W {
        self.variant(ENABLE_A::ACS_CLK_DET_ENABLE)
    }
    #[doc = "Clock detector disable"]
    #[inline(always)]
    pub fn acs_clk_det_disable(self) -> &'a mut W {
        self.variant(ENABLE_A::ACS_CLK_DET_DISABLE)
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
    #[doc = "Bit 8 - Clock present flag"]
    #[inline(always)]
    pub fn clock_present(&self) -> CLOCK_PRESENT_R {
        CLOCK_PRESENT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock detector reset condition ignore"]
    #[inline(always)]
    pub fn reset_ignore(&self) -> RESET_IGNORE_R {
        RESET_IGNORE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Clock detector enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clock detector reset condition ignore"]
    #[inline(always)]
    pub fn reset_ignore(&mut self) -> RESET_IGNORE_W {
        RESET_IGNORE_W { w: self }
    }
    #[doc = "Bit 0 - Clock detector enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
