#[doc = "Reader of register RF_XTAL_CTRL"]
pub type R = crate::R<u32, super::RF_XTAL_CTRL>;
#[doc = "Writer for register RF_XTAL_CTRL"]
pub type W = crate::W<u32, super::RF_XTAL_CTRL>;
#[doc = "Register RF_XTAL_CTRL `reset()`'s with value 0xc381_0500"]
impl crate::ResetValue for super::RF_XTAL_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc381_0500
    }
}
#[doc = "High threshold for xtal trimming\n\nValue on reset: 12"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XTAL_CTRL_XO_THR_HIGH_A {
    #[doc = "12: `1100`"]
    XTAL_CTRL_XO_THR_HIGH_DEFAULT = 12,
}
impl From<XTAL_CTRL_XO_THR_HIGH_A> for u8 {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_XO_THR_HIGH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XTAL_CTRL_XO_THR_HIGH`"]
pub type XTAL_CTRL_XO_THR_HIGH_R = crate::R<u8, XTAL_CTRL_XO_THR_HIGH_A>;
impl XTAL_CTRL_XO_THR_HIGH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, XTAL_CTRL_XO_THR_HIGH_A> {
        use crate::Variant::*;
        match self.bits {
            12 => Val(XTAL_CTRL_XO_THR_HIGH_A::XTAL_CTRL_XO_THR_HIGH_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_XO_THR_HIGH_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_xo_thr_high_default(&self) -> bool {
        *self == XTAL_CTRL_XO_THR_HIGH_A::XTAL_CTRL_XO_THR_HIGH_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_XO_THR_HIGH`"]
pub struct XTAL_CTRL_XO_THR_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_XO_THR_HIGH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_XO_THR_HIGH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn xtal_ctrl_xo_thr_high_default(self) -> &'a mut W {
        self.variant(XTAL_CTRL_XO_THR_HIGH_A::XTAL_CTRL_XO_THR_HIGH_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Low threshold for xtal trimming\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XTAL_CTRL_XO_THR_LOW_A {
    #[doc = "3: `11`"]
    XTAL_CTRL_XO_THR_LOW_DEFAULT = 3,
}
impl From<XTAL_CTRL_XO_THR_LOW_A> for u8 {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_XO_THR_LOW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XTAL_CTRL_XO_THR_LOW`"]
pub type XTAL_CTRL_XO_THR_LOW_R = crate::R<u8, XTAL_CTRL_XO_THR_LOW_A>;
impl XTAL_CTRL_XO_THR_LOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, XTAL_CTRL_XO_THR_LOW_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(XTAL_CTRL_XO_THR_LOW_A::XTAL_CTRL_XO_THR_LOW_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_XO_THR_LOW_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_xo_thr_low_default(&self) -> bool {
        *self == XTAL_CTRL_XO_THR_LOW_A::XTAL_CTRL_XO_THR_LOW_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_XO_THR_LOW`"]
pub struct XTAL_CTRL_XO_THR_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_XO_THR_LOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_XO_THR_LOW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn xtal_ctrl_xo_thr_low_default(self) -> &'a mut W {
        self.variant(XTAL_CTRL_XO_THR_LOW_A::XTAL_CTRL_XO_THR_LOW_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Value of after_startup_curr_sel when level is higher than xo_thr_high\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_A {
    #[doc = "2: `10`"]
    XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_DEFAULT = 2,
}
impl From<XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_A> for u8 {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XTAL_CTRL_XO_A_S_CURR_SEL_HIGH`"]
pub type XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_R = crate::R<u8, XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_A>;
impl XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_A::XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_xo_a_s_curr_sel_high_default(&self) -> bool {
        *self == XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_A::XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_XO_A_S_CURR_SEL_HIGH`"]
pub struct XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn xtal_ctrl_xo_a_s_curr_sel_high_default(self) -> &'a mut W {
        self.variant(XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_A::XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Value of after_startup_curr_sel when level is lower than xo_thr_low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XTAL_CTRL_XO_A_S_CURR_SEL_LOW_A {
    #[doc = "0: `0`"]
    XTAL_CTRL_XO_A_S_CURR_SEL_LOW_DEFAULT = 0,
}
impl From<XTAL_CTRL_XO_A_S_CURR_SEL_LOW_A> for u8 {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_XO_A_S_CURR_SEL_LOW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XTAL_CTRL_XO_A_S_CURR_SEL_LOW`"]
pub type XTAL_CTRL_XO_A_S_CURR_SEL_LOW_R = crate::R<u8, XTAL_CTRL_XO_A_S_CURR_SEL_LOW_A>;
impl XTAL_CTRL_XO_A_S_CURR_SEL_LOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, XTAL_CTRL_XO_A_S_CURR_SEL_LOW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(XTAL_CTRL_XO_A_S_CURR_SEL_LOW_A::XTAL_CTRL_XO_A_S_CURR_SEL_LOW_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_XO_A_S_CURR_SEL_LOW_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_xo_a_s_curr_sel_low_default(&self) -> bool {
        *self == XTAL_CTRL_XO_A_S_CURR_SEL_LOW_A::XTAL_CTRL_XO_A_S_CURR_SEL_LOW_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_XO_A_S_CURR_SEL_LOW`"]
pub struct XTAL_CTRL_XO_A_S_CURR_SEL_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_XO_A_S_CURR_SEL_LOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_XO_A_S_CURR_SEL_LOW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn xtal_ctrl_xo_a_s_curr_sel_low_default(self) -> &'a mut W {
        self.variant(XTAL_CTRL_XO_A_S_CURR_SEL_LOW_A::XTAL_CTRL_XO_A_S_CURR_SEL_LOW_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Bypass the Xtal control algorithm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL_CTRL_XTAL_CTRL_BYPASS_A {
    #[doc = "0: `0`"]
    XTAL_CTRL_XTAL_CTRL_BYPASS_DEFAULT = 0,
}
impl From<XTAL_CTRL_XTAL_CTRL_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_XTAL_CTRL_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XTAL_CTRL_XTAL_CTRL_BYPASS`"]
pub type XTAL_CTRL_XTAL_CTRL_BYPASS_R = crate::R<bool, XTAL_CTRL_XTAL_CTRL_BYPASS_A>;
impl XTAL_CTRL_XTAL_CTRL_BYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, XTAL_CTRL_XTAL_CTRL_BYPASS_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(XTAL_CTRL_XTAL_CTRL_BYPASS_A::XTAL_CTRL_XTAL_CTRL_BYPASS_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_XTAL_CTRL_BYPASS_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_xtal_ctrl_bypass_default(&self) -> bool {
        *self == XTAL_CTRL_XTAL_CTRL_BYPASS_A::XTAL_CTRL_XTAL_CTRL_BYPASS_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_XTAL_CTRL_BYPASS`"]
pub struct XTAL_CTRL_XTAL_CTRL_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_XTAL_CTRL_BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_XTAL_CTRL_BYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn xtal_ctrl_xtal_ctrl_bypass_default(self) -> &'a mut W {
        self.variant(XTAL_CTRL_XTAL_CTRL_BYPASS_A::XTAL_CTRL_XTAL_CTRL_BYPASS_DEFAULT)
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
#[doc = "If set to 1 selects the clk_in_dig signal for the digital block, otherwise the internal xtal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL_CTRL_DIG_CLK_IN_SEL_A {
    #[doc = "0: `0`"]
    XTAL_CTRL_DIG_CLK_IN_SEL_DEFAULT = 0,
}
impl From<XTAL_CTRL_DIG_CLK_IN_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_DIG_CLK_IN_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XTAL_CTRL_DIG_CLK_IN_SEL`"]
pub type XTAL_CTRL_DIG_CLK_IN_SEL_R = crate::R<bool, XTAL_CTRL_DIG_CLK_IN_SEL_A>;
impl XTAL_CTRL_DIG_CLK_IN_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, XTAL_CTRL_DIG_CLK_IN_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(XTAL_CTRL_DIG_CLK_IN_SEL_A::XTAL_CTRL_DIG_CLK_IN_SEL_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_DIG_CLK_IN_SEL_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_dig_clk_in_sel_default(&self) -> bool {
        *self == XTAL_CTRL_DIG_CLK_IN_SEL_A::XTAL_CTRL_DIG_CLK_IN_SEL_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_DIG_CLK_IN_SEL`"]
pub struct XTAL_CTRL_DIG_CLK_IN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_DIG_CLK_IN_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_DIG_CLK_IN_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn xtal_ctrl_dig_clk_in_sel_default(self) -> &'a mut W {
        self.variant(XTAL_CTRL_DIG_CLK_IN_SEL_A::XTAL_CTRL_DIG_CLK_IN_SEL_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Xtal oscillator enable (active low)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL_CTRL_XO_EN_B_REG_A {
    #[doc = "0: `0`"]
    XTAL_CTRL_ENABLE_OSCILLATOR = 0,
    #[doc = "1: `1`"]
    XTAL_CTRL_DISABLE_OSCILLATOR = 1,
}
impl From<XTAL_CTRL_XO_EN_B_REG_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_XO_EN_B_REG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XTAL_CTRL_XO_EN_B_REG`"]
pub type XTAL_CTRL_XO_EN_B_REG_R = crate::R<bool, XTAL_CTRL_XO_EN_B_REG_A>;
impl XTAL_CTRL_XO_EN_B_REG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTAL_CTRL_XO_EN_B_REG_A {
        match self.bits {
            false => XTAL_CTRL_XO_EN_B_REG_A::XTAL_CTRL_ENABLE_OSCILLATOR,
            true => XTAL_CTRL_XO_EN_B_REG_A::XTAL_CTRL_DISABLE_OSCILLATOR,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_ENABLE_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_enable_oscillator(&self) -> bool {
        *self == XTAL_CTRL_XO_EN_B_REG_A::XTAL_CTRL_ENABLE_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_DISABLE_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_disable_oscillator(&self) -> bool {
        *self == XTAL_CTRL_XO_EN_B_REG_A::XTAL_CTRL_DISABLE_OSCILLATOR
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_XO_EN_B_REG`"]
pub struct XTAL_CTRL_XO_EN_B_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_XO_EN_B_REG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_XO_EN_B_REG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn xtal_ctrl_enable_oscillator(self) -> &'a mut W {
        self.variant(XTAL_CTRL_XO_EN_B_REG_A::XTAL_CTRL_ENABLE_OSCILLATOR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn xtal_ctrl_disable_oscillator(self) -> &'a mut W {
        self.variant(XTAL_CTRL_XO_EN_B_REG_A::XTAL_CTRL_DISABLE_OSCILLATOR)
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
#[doc = "Xtal trimming speed: 00 => 43us, 01 => 85us, 10 => 171us, 11 => 341us\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XTAL_CTRL_XTAL_CKDIV_A {
    #[doc = "0: `0`"]
    XTAL_CTRL_XTAL_CKDIV_DEFAULT = 0,
}
impl From<XTAL_CTRL_XTAL_CKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_XTAL_CKDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XTAL_CTRL_XTAL_CKDIV`"]
pub type XTAL_CTRL_XTAL_CKDIV_R = crate::R<u8, XTAL_CTRL_XTAL_CKDIV_A>;
impl XTAL_CTRL_XTAL_CKDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, XTAL_CTRL_XTAL_CKDIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(XTAL_CTRL_XTAL_CKDIV_A::XTAL_CTRL_XTAL_CKDIV_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_XTAL_CKDIV_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_xtal_ckdiv_default(&self) -> bool {
        *self == XTAL_CTRL_XTAL_CKDIV_A::XTAL_CTRL_XTAL_CKDIV_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_XTAL_CKDIV`"]
pub struct XTAL_CTRL_XTAL_CKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_XTAL_CKDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_XTAL_CKDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn xtal_ctrl_xtal_ckdiv_default(self) -> &'a mut W {
        self.variant(XTAL_CTRL_XTAL_CKDIV_A::XTAL_CTRL_XTAL_CKDIV_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "When high, disable the output clock to go to main IP (clk_out output stay low).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL_CTRL_CLK_OUT_EN_B_A {
    #[doc = "0: `0`"]
    XTAL_CTRL_CLK_OUT_EN_B_DEFAULT = 0,
}
impl From<XTAL_CTRL_CLK_OUT_EN_B_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_CLK_OUT_EN_B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XTAL_CTRL_CLK_OUT_EN_B`"]
pub type XTAL_CTRL_CLK_OUT_EN_B_R = crate::R<bool, XTAL_CTRL_CLK_OUT_EN_B_A>;
impl XTAL_CTRL_CLK_OUT_EN_B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, XTAL_CTRL_CLK_OUT_EN_B_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(XTAL_CTRL_CLK_OUT_EN_B_A::XTAL_CTRL_CLK_OUT_EN_B_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_CLK_OUT_EN_B_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_clk_out_en_b_default(&self) -> bool {
        *self == XTAL_CTRL_CLK_OUT_EN_B_A::XTAL_CTRL_CLK_OUT_EN_B_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_CLK_OUT_EN_B`"]
pub struct XTAL_CTRL_CLK_OUT_EN_B_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_CLK_OUT_EN_B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_CLK_OUT_EN_B_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn xtal_ctrl_clk_out_en_b_default(self) -> &'a mut W {
        self.variant(XTAL_CTRL_CLK_OUT_EN_B_A::XTAL_CTRL_CLK_OUT_EN_B_DEFAULT)
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
#[doc = "When low, all main ctrl signals are used instead of corresponding ctrl signal or some control bits of xtal_reg. They are: xo_en_b, ext_clk_mode and lp_mode. When high, corresponding ctrl signal and some control bits of xtal_reg are used instead of main ctrl signals. They are: xo_en_b_reg, ext_clk_mode (bit 0) and lp_mode (bit 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL_CTRL_REG_VALUE_SEL_A {
    #[doc = "0: `0`"]
    XTAL_CTRL_REG_VALUE_SEL_EXTERNAL = 0,
    #[doc = "1: `1`"]
    XTAL_CTRL_REG_VALUE_SEL_INTERNAL = 1,
}
impl From<XTAL_CTRL_REG_VALUE_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_REG_VALUE_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XTAL_CTRL_REG_VALUE_SEL`"]
pub type XTAL_CTRL_REG_VALUE_SEL_R = crate::R<bool, XTAL_CTRL_REG_VALUE_SEL_A>;
impl XTAL_CTRL_REG_VALUE_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTAL_CTRL_REG_VALUE_SEL_A {
        match self.bits {
            false => XTAL_CTRL_REG_VALUE_SEL_A::XTAL_CTRL_REG_VALUE_SEL_EXTERNAL,
            true => XTAL_CTRL_REG_VALUE_SEL_A::XTAL_CTRL_REG_VALUE_SEL_INTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_REG_VALUE_SEL_EXTERNAL`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_reg_value_sel_external(&self) -> bool {
        *self == XTAL_CTRL_REG_VALUE_SEL_A::XTAL_CTRL_REG_VALUE_SEL_EXTERNAL
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_REG_VALUE_SEL_INTERNAL`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_reg_value_sel_internal(&self) -> bool {
        *self == XTAL_CTRL_REG_VALUE_SEL_A::XTAL_CTRL_REG_VALUE_SEL_INTERNAL
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_REG_VALUE_SEL`"]
pub struct XTAL_CTRL_REG_VALUE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_REG_VALUE_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_REG_VALUE_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn xtal_ctrl_reg_value_sel_external(self) -> &'a mut W {
        self.variant(XTAL_CTRL_REG_VALUE_SEL_A::XTAL_CTRL_REG_VALUE_SEL_EXTERNAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn xtal_ctrl_reg_value_sel_internal(self) -> &'a mut W {
        self.variant(XTAL_CTRL_REG_VALUE_SEL_A::XTAL_CTRL_REG_VALUE_SEL_INTERNAL)
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
#[doc = "Selection of the current before amplitude stabilization but after starting-up in active transistors of the core oscillator: '00': typ. 0.15 mA, '01': typ. 0.24 mA, '10': typ. 0.40 mA, '11': typ. 0.61 mA\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XTAL_CTRL_AFTERSTARTUP_CURR_SEL_A {
    #[doc = "1: `1`"]
    XTAL_CTRL_AFTERSTARTUP_CURR_SEL_DEFAULT = 1,
}
impl From<XTAL_CTRL_AFTERSTARTUP_CURR_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_AFTERSTARTUP_CURR_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XTAL_CTRL_AFTERSTARTUP_CURR_SEL`"]
pub type XTAL_CTRL_AFTERSTARTUP_CURR_SEL_R = crate::R<u8, XTAL_CTRL_AFTERSTARTUP_CURR_SEL_A>;
impl XTAL_CTRL_AFTERSTARTUP_CURR_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, XTAL_CTRL_AFTERSTARTUP_CURR_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(XTAL_CTRL_AFTERSTARTUP_CURR_SEL_A::XTAL_CTRL_AFTERSTARTUP_CURR_SEL_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_AFTERSTARTUP_CURR_SEL_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_afterstartup_curr_sel_default(&self) -> bool {
        *self == XTAL_CTRL_AFTERSTARTUP_CURR_SEL_A::XTAL_CTRL_AFTERSTARTUP_CURR_SEL_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_AFTERSTARTUP_CURR_SEL`"]
pub struct XTAL_CTRL_AFTERSTARTUP_CURR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_AFTERSTARTUP_CURR_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_AFTERSTARTUP_CURR_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn xtal_ctrl_afterstartup_curr_sel_default(self) -> &'a mut W {
        self.variant(XTAL_CTRL_AFTERSTARTUP_CURR_SEL_A::XTAL_CTRL_AFTERSTARTUP_CURR_SEL_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Selection of the starting-up current in active transistors of the core oscillator: '00': typ. 0.41 mA, '01': typ. 0.59 mA, '10': typ. 0.88 mA, '11': typ. 1.24 mA\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XTAL_CTRL_STARTUP_CURR_SEL_A {
    #[doc = "1: `1`"]
    XTAL_CTRL_STARTUP_CURR_SEL_DEFAULT = 1,
}
impl From<XTAL_CTRL_STARTUP_CURR_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_STARTUP_CURR_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XTAL_CTRL_STARTUP_CURR_SEL`"]
pub type XTAL_CTRL_STARTUP_CURR_SEL_R = crate::R<u8, XTAL_CTRL_STARTUP_CURR_SEL_A>;
impl XTAL_CTRL_STARTUP_CURR_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, XTAL_CTRL_STARTUP_CURR_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(XTAL_CTRL_STARTUP_CURR_SEL_A::XTAL_CTRL_STARTUP_CURR_SEL_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_STARTUP_CURR_SEL_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_startup_curr_sel_default(&self) -> bool {
        *self == XTAL_CTRL_STARTUP_CURR_SEL_A::XTAL_CTRL_STARTUP_CURR_SEL_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_STARTUP_CURR_SEL`"]
pub struct XTAL_CTRL_STARTUP_CURR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_STARTUP_CURR_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_STARTUP_CURR_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn xtal_ctrl_startup_curr_sel_default(self) -> &'a mut W {
        self.variant(XTAL_CTRL_STARTUP_CURR_SEL_A::XTAL_CTRL_STARTUP_CURR_SEL_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Invert clock on clk_dig output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL_CTRL_INV_CLK_DIG_A {
    #[doc = "0: `0`"]
    XTAL_CTRL_INV_CLK_DIG_DEFAULT = 0,
}
impl From<XTAL_CTRL_INV_CLK_DIG_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_INV_CLK_DIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XTAL_CTRL_INV_CLK_DIG`"]
pub type XTAL_CTRL_INV_CLK_DIG_R = crate::R<bool, XTAL_CTRL_INV_CLK_DIG_A>;
impl XTAL_CTRL_INV_CLK_DIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, XTAL_CTRL_INV_CLK_DIG_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(XTAL_CTRL_INV_CLK_DIG_A::XTAL_CTRL_INV_CLK_DIG_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_INV_CLK_DIG_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_inv_clk_dig_default(&self) -> bool {
        *self == XTAL_CTRL_INV_CLK_DIG_A::XTAL_CTRL_INV_CLK_DIG_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_INV_CLK_DIG`"]
pub struct XTAL_CTRL_INV_CLK_DIG_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_INV_CLK_DIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_INV_CLK_DIG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn xtal_ctrl_inv_clk_dig_default(self) -> &'a mut W {
        self.variant(XTAL_CTRL_INV_CLK_DIG_A::XTAL_CTRL_INV_CLK_DIG_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Invert clock on clk_pll output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL_CTRL_INV_CLK_PLL_A {
    #[doc = "0: `0`"]
    XTAL_CTRL_INV_CLK_PLL_DEFAULT = 0,
}
impl From<XTAL_CTRL_INV_CLK_PLL_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_INV_CLK_PLL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XTAL_CTRL_INV_CLK_PLL`"]
pub type XTAL_CTRL_INV_CLK_PLL_R = crate::R<bool, XTAL_CTRL_INV_CLK_PLL_A>;
impl XTAL_CTRL_INV_CLK_PLL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, XTAL_CTRL_INV_CLK_PLL_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(XTAL_CTRL_INV_CLK_PLL_A::XTAL_CTRL_INV_CLK_PLL_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_INV_CLK_PLL_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_inv_clk_pll_default(&self) -> bool {
        *self == XTAL_CTRL_INV_CLK_PLL_A::XTAL_CTRL_INV_CLK_PLL_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_INV_CLK_PLL`"]
pub struct XTAL_CTRL_INV_CLK_PLL_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_INV_CLK_PLL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_INV_CLK_PLL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn xtal_ctrl_inv_clk_pll_default(self) -> &'a mut W {
        self.variant(XTAL_CTRL_INV_CLK_PLL_A::XTAL_CTRL_INV_CLK_PLL_DEFAULT)
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
#[doc = "Debug: allow to force output clocks on clk_pll, clk_dig and clk_out (if these outputs are enabled) and bypass the xtal internal clock detector that gates these clock outputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL_CTRL_FORCE_CLK_READY_A {
    #[doc = "0: `0`"]
    XTAL_CTRL_FORCE_CLK_READY_DEFAULT = 0,
}
impl From<XTAL_CTRL_FORCE_CLK_READY_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_FORCE_CLK_READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XTAL_CTRL_FORCE_CLK_READY`"]
pub type XTAL_CTRL_FORCE_CLK_READY_R = crate::R<bool, XTAL_CTRL_FORCE_CLK_READY_A>;
impl XTAL_CTRL_FORCE_CLK_READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, XTAL_CTRL_FORCE_CLK_READY_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(XTAL_CTRL_FORCE_CLK_READY_A::XTAL_CTRL_FORCE_CLK_READY_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_FORCE_CLK_READY_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_force_clk_ready_default(&self) -> bool {
        *self == XTAL_CTRL_FORCE_CLK_READY_A::XTAL_CTRL_FORCE_CLK_READY_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_FORCE_CLK_READY`"]
pub struct XTAL_CTRL_FORCE_CLK_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_FORCE_CLK_READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_FORCE_CLK_READY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn xtal_ctrl_force_clk_ready_default(self) -> &'a mut W {
        self.variant(XTAL_CTRL_FORCE_CLK_READY_A::XTAL_CTRL_FORCE_CLK_READY_DEFAULT)
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
#[doc = "When high, disable the output clock to go to digital (clk_dig output stay low).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL_CTRL_CLK_DIG_EN_B_A {
    #[doc = "0: `0`"]
    XTAL_CTRL_CLK_DIG_EN_B_DEFAULT = 0,
}
impl From<XTAL_CTRL_CLK_DIG_EN_B_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_CLK_DIG_EN_B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XTAL_CTRL_CLK_DIG_EN_B`"]
pub type XTAL_CTRL_CLK_DIG_EN_B_R = crate::R<bool, XTAL_CTRL_CLK_DIG_EN_B_A>;
impl XTAL_CTRL_CLK_DIG_EN_B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, XTAL_CTRL_CLK_DIG_EN_B_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(XTAL_CTRL_CLK_DIG_EN_B_A::XTAL_CTRL_CLK_DIG_EN_B_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_CLK_DIG_EN_B_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_clk_dig_en_b_default(&self) -> bool {
        *self == XTAL_CTRL_CLK_DIG_EN_B_A::XTAL_CTRL_CLK_DIG_EN_B_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_CLK_DIG_EN_B`"]
pub struct XTAL_CTRL_CLK_DIG_EN_B_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_CLK_DIG_EN_B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_CLK_DIG_EN_B_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn xtal_ctrl_clk_dig_en_b_default(self) -> &'a mut W {
        self.variant(XTAL_CTRL_CLK_DIG_EN_B_A::XTAL_CTRL_CLK_DIG_EN_B_DEFAULT)
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
#[doc = "When low (and if xtal_en_b(_reg) is low), the xtal buffer is enabled otherwise it is disabled. Could be used to decrease the power consumption of the xtal while maintaining oscillation in the xtal oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL_CTRL_BUFF_EN_B_A {
    #[doc = "0: `0`"]
    XTAL_CTRL_BUFF_EN_B_DEFAULT = 0,
}
impl From<XTAL_CTRL_BUFF_EN_B_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_BUFF_EN_B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XTAL_CTRL_BUFF_EN_B`"]
pub type XTAL_CTRL_BUFF_EN_B_R = crate::R<bool, XTAL_CTRL_BUFF_EN_B_A>;
impl XTAL_CTRL_BUFF_EN_B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, XTAL_CTRL_BUFF_EN_B_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(XTAL_CTRL_BUFF_EN_B_A::XTAL_CTRL_BUFF_EN_B_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_BUFF_EN_B_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_buff_en_b_default(&self) -> bool {
        *self == XTAL_CTRL_BUFF_EN_B_A::XTAL_CTRL_BUFF_EN_B_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_BUFF_EN_B`"]
pub struct XTAL_CTRL_BUFF_EN_B_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_BUFF_EN_B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_BUFF_EN_B_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn xtal_ctrl_buff_en_b_default(self) -> &'a mut W {
        self.variant(XTAL_CTRL_BUFF_EN_B_A::XTAL_CTRL_BUFF_EN_B_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "When high, bias current in the clock buffer is increased compared to normal operation (high bandwidth mode in 132 MHz clock input buffer).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL_CTRL_HP_MODE_A {
    #[doc = "0: `0`"]
    XTAL_CTRL_HP_MODE_DEFAULT = 0,
}
impl From<XTAL_CTRL_HP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_HP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XTAL_CTRL_HP_MODE`"]
pub type XTAL_CTRL_HP_MODE_R = crate::R<bool, XTAL_CTRL_HP_MODE_A>;
impl XTAL_CTRL_HP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, XTAL_CTRL_HP_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(XTAL_CTRL_HP_MODE_A::XTAL_CTRL_HP_MODE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_HP_MODE_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_hp_mode_default(&self) -> bool {
        *self == XTAL_CTRL_HP_MODE_A::XTAL_CTRL_HP_MODE_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_HP_MODE`"]
pub struct XTAL_CTRL_HP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_HP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_HP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn xtal_ctrl_hp_mode_default(self) -> &'a mut W {
        self.variant(XTAL_CTRL_HP_MODE_A::XTAL_CTRL_HP_MODE_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "When high, bias current in the clock buffer is reduced compared to normal operation (low power mode). Usable only if bit 12 is high (see below) otherwise it is bypassed by lp_mode pin input on main interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL_CTRL_LP_MODE_A {
    #[doc = "0: `0`"]
    XTAL_CTRL_LP_MODE_DEFAULT = 0,
}
impl From<XTAL_CTRL_LP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_LP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XTAL_CTRL_LP_MODE`"]
pub type XTAL_CTRL_LP_MODE_R = crate::R<bool, XTAL_CTRL_LP_MODE_A>;
impl XTAL_CTRL_LP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, XTAL_CTRL_LP_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(XTAL_CTRL_LP_MODE_A::XTAL_CTRL_LP_MODE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_LP_MODE_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_lp_mode_default(&self) -> bool {
        *self == XTAL_CTRL_LP_MODE_A::XTAL_CTRL_LP_MODE_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_LP_MODE`"]
pub struct XTAL_CTRL_LP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_LP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_LP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn xtal_ctrl_lp_mode_default(self) -> &'a mut W {
        self.variant(XTAL_CTRL_LP_MODE_A::XTAL_CTRL_LP_MODE_DEFAULT)
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
#[doc = "When high, allow to uses xtal_p (and eventually xtal_n) has external clock input(s). The XTAL oscillator core is disabled. Usable only if bit 12 is high (see below) otherwise it is bypassed by ext_clk_mode pin input on main interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL_CTRL_EXT_CLK_MODE_A {
    #[doc = "0: `0`"]
    XTAL_CTRL_EXT_CLK_MODE_DEFAULT = 0,
}
impl From<XTAL_CTRL_EXT_CLK_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL_CTRL_EXT_CLK_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XTAL_CTRL_EXT_CLK_MODE`"]
pub type XTAL_CTRL_EXT_CLK_MODE_R = crate::R<bool, XTAL_CTRL_EXT_CLK_MODE_A>;
impl XTAL_CTRL_EXT_CLK_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, XTAL_CTRL_EXT_CLK_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(XTAL_CTRL_EXT_CLK_MODE_A::XTAL_CTRL_EXT_CLK_MODE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CTRL_EXT_CLK_MODE_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_ctrl_ext_clk_mode_default(&self) -> bool {
        *self == XTAL_CTRL_EXT_CLK_MODE_A::XTAL_CTRL_EXT_CLK_MODE_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_CTRL_EXT_CLK_MODE`"]
pub struct XTAL_CTRL_EXT_CLK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CTRL_EXT_CLK_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_CTRL_EXT_CLK_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn xtal_ctrl_ext_clk_mode_default(self) -> &'a mut W {
        self.variant(XTAL_CTRL_EXT_CLK_MODE_A::XTAL_CTRL_EXT_CLK_MODE_DEFAULT)
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
    #[doc = "Bits 28:31 - High threshold for xtal trimming"]
    #[inline(always)]
    pub fn xtal_ctrl_xo_thr_high(&self) -> XTAL_CTRL_XO_THR_HIGH_R {
        XTAL_CTRL_XO_THR_HIGH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Low threshold for xtal trimming"]
    #[inline(always)]
    pub fn xtal_ctrl_xo_thr_low(&self) -> XTAL_CTRL_XO_THR_LOW_R {
        XTAL_CTRL_XO_THR_LOW_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - Value of after_startup_curr_sel when level is higher than xo_thr_high"]
    #[inline(always)]
    pub fn xtal_ctrl_xo_a_s_curr_sel_high(&self) -> XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_R {
        XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Value of after_startup_curr_sel when level is lower than xo_thr_low"]
    #[inline(always)]
    pub fn xtal_ctrl_xo_a_s_curr_sel_low(&self) -> XTAL_CTRL_XO_A_S_CURR_SEL_LOW_R {
        XTAL_CTRL_XO_A_S_CURR_SEL_LOW_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Bypass the Xtal control algorithm"]
    #[inline(always)]
    pub fn xtal_ctrl_xtal_ctrl_bypass(&self) -> XTAL_CTRL_XTAL_CTRL_BYPASS_R {
        XTAL_CTRL_XTAL_CTRL_BYPASS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - If set to 1 selects the clk_in_dig signal for the digital block, otherwise the internal xtal"]
    #[inline(always)]
    pub fn xtal_ctrl_dig_clk_in_sel(&self) -> XTAL_CTRL_DIG_CLK_IN_SEL_R {
        XTAL_CTRL_DIG_CLK_IN_SEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Xtal oscillator enable (active low)"]
    #[inline(always)]
    pub fn xtal_ctrl_xo_en_b_reg(&self) -> XTAL_CTRL_XO_EN_B_REG_R {
        XTAL_CTRL_XO_EN_B_REG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Xtal trimming speed: 00 => 43us, 01 => 85us, 10 => 171us, 11 => 341us"]
    #[inline(always)]
    pub fn xtal_ctrl_xtal_ckdiv(&self) -> XTAL_CTRL_XTAL_CKDIV_R {
        XTAL_CTRL_XTAL_CKDIV_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 13 - When high, disable the output clock to go to main IP (clk_out output stay low)."]
    #[inline(always)]
    pub fn xtal_ctrl_clk_out_en_b(&self) -> XTAL_CTRL_CLK_OUT_EN_B_R {
        XTAL_CTRL_CLK_OUT_EN_B_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - When low, all main ctrl signals are used instead of corresponding ctrl signal or some control bits of xtal_reg. They are: xo_en_b, ext_clk_mode and lp_mode. When high, corresponding ctrl signal and some control bits of xtal_reg are used instead of main ctrl signals. They are: xo_en_b_reg, ext_clk_mode (bit 0) and lp_mode (bit 1)."]
    #[inline(always)]
    pub fn xtal_ctrl_reg_value_sel(&self) -> XTAL_CTRL_REG_VALUE_SEL_R {
        XTAL_CTRL_REG_VALUE_SEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Selection of the current before amplitude stabilization but after starting-up in active transistors of the core oscillator: '00': typ. 0.15 mA, '01': typ. 0.24 mA, '10': typ. 0.40 mA, '11': typ. 0.61 mA"]
    #[inline(always)]
    pub fn xtal_ctrl_afterstartup_curr_sel(&self) -> XTAL_CTRL_AFTERSTARTUP_CURR_SEL_R {
        XTAL_CTRL_AFTERSTARTUP_CURR_SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Selection of the starting-up current in active transistors of the core oscillator: '00': typ. 0.41 mA, '01': typ. 0.59 mA, '10': typ. 0.88 mA, '11': typ. 1.24 mA"]
    #[inline(always)]
    pub fn xtal_ctrl_startup_curr_sel(&self) -> XTAL_CTRL_STARTUP_CURR_SEL_R {
        XTAL_CTRL_STARTUP_CURR_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Invert clock on clk_dig output"]
    #[inline(always)]
    pub fn xtal_ctrl_inv_clk_dig(&self) -> XTAL_CTRL_INV_CLK_DIG_R {
        XTAL_CTRL_INV_CLK_DIG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Invert clock on clk_pll output"]
    #[inline(always)]
    pub fn xtal_ctrl_inv_clk_pll(&self) -> XTAL_CTRL_INV_CLK_PLL_R {
        XTAL_CTRL_INV_CLK_PLL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Debug: allow to force output clocks on clk_pll, clk_dig and clk_out (if these outputs are enabled) and bypass the xtal internal clock detector that gates these clock outputs."]
    #[inline(always)]
    pub fn xtal_ctrl_force_clk_ready(&self) -> XTAL_CTRL_FORCE_CLK_READY_R {
        XTAL_CTRL_FORCE_CLK_READY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When high, disable the output clock to go to digital (clk_dig output stay low)."]
    #[inline(always)]
    pub fn xtal_ctrl_clk_dig_en_b(&self) -> XTAL_CTRL_CLK_DIG_EN_B_R {
        XTAL_CTRL_CLK_DIG_EN_B_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When low (and if xtal_en_b(_reg) is low), the xtal buffer is enabled otherwise it is disabled. Could be used to decrease the power consumption of the xtal while maintaining oscillation in the xtal oscillator"]
    #[inline(always)]
    pub fn xtal_ctrl_buff_en_b(&self) -> XTAL_CTRL_BUFF_EN_B_R {
        XTAL_CTRL_BUFF_EN_B_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When high, bias current in the clock buffer is increased compared to normal operation (high bandwidth mode in 132 MHz clock input buffer)."]
    #[inline(always)]
    pub fn xtal_ctrl_hp_mode(&self) -> XTAL_CTRL_HP_MODE_R {
        XTAL_CTRL_HP_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - When high, bias current in the clock buffer is reduced compared to normal operation (low power mode). Usable only if bit 12 is high (see below) otherwise it is bypassed by lp_mode pin input on main interface"]
    #[inline(always)]
    pub fn xtal_ctrl_lp_mode(&self) -> XTAL_CTRL_LP_MODE_R {
        XTAL_CTRL_LP_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - When high, allow to uses xtal_p (and eventually xtal_n) has external clock input(s). The XTAL oscillator core is disabled. Usable only if bit 12 is high (see below) otherwise it is bypassed by ext_clk_mode pin input on main interface"]
    #[inline(always)]
    pub fn xtal_ctrl_ext_clk_mode(&self) -> XTAL_CTRL_EXT_CLK_MODE_R {
        XTAL_CTRL_EXT_CLK_MODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31 - High threshold for xtal trimming"]
    #[inline(always)]
    pub fn xtal_ctrl_xo_thr_high(&mut self) -> XTAL_CTRL_XO_THR_HIGH_W {
        XTAL_CTRL_XO_THR_HIGH_W { w: self }
    }
    #[doc = "Bits 24:27 - Low threshold for xtal trimming"]
    #[inline(always)]
    pub fn xtal_ctrl_xo_thr_low(&mut self) -> XTAL_CTRL_XO_THR_LOW_W {
        XTAL_CTRL_XO_THR_LOW_W { w: self }
    }
    #[doc = "Bits 22:23 - Value of after_startup_curr_sel when level is higher than xo_thr_high"]
    #[inline(always)]
    pub fn xtal_ctrl_xo_a_s_curr_sel_high(&mut self) -> XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_W {
        XTAL_CTRL_XO_A_S_CURR_SEL_HIGH_W { w: self }
    }
    #[doc = "Bits 20:21 - Value of after_startup_curr_sel when level is lower than xo_thr_low"]
    #[inline(always)]
    pub fn xtal_ctrl_xo_a_s_curr_sel_low(&mut self) -> XTAL_CTRL_XO_A_S_CURR_SEL_LOW_W {
        XTAL_CTRL_XO_A_S_CURR_SEL_LOW_W { w: self }
    }
    #[doc = "Bit 18 - Bypass the Xtal control algorithm"]
    #[inline(always)]
    pub fn xtal_ctrl_xtal_ctrl_bypass(&mut self) -> XTAL_CTRL_XTAL_CTRL_BYPASS_W {
        XTAL_CTRL_XTAL_CTRL_BYPASS_W { w: self }
    }
    #[doc = "Bit 17 - If set to 1 selects the clk_in_dig signal for the digital block, otherwise the internal xtal"]
    #[inline(always)]
    pub fn xtal_ctrl_dig_clk_in_sel(&mut self) -> XTAL_CTRL_DIG_CLK_IN_SEL_W {
        XTAL_CTRL_DIG_CLK_IN_SEL_W { w: self }
    }
    #[doc = "Bit 16 - Xtal oscillator enable (active low)"]
    #[inline(always)]
    pub fn xtal_ctrl_xo_en_b_reg(&mut self) -> XTAL_CTRL_XO_EN_B_REG_W {
        XTAL_CTRL_XO_EN_B_REG_W { w: self }
    }
    #[doc = "Bits 14:15 - Xtal trimming speed: 00 => 43us, 01 => 85us, 10 => 171us, 11 => 341us"]
    #[inline(always)]
    pub fn xtal_ctrl_xtal_ckdiv(&mut self) -> XTAL_CTRL_XTAL_CKDIV_W {
        XTAL_CTRL_XTAL_CKDIV_W { w: self }
    }
    #[doc = "Bit 13 - When high, disable the output clock to go to main IP (clk_out output stay low)."]
    #[inline(always)]
    pub fn xtal_ctrl_clk_out_en_b(&mut self) -> XTAL_CTRL_CLK_OUT_EN_B_W {
        XTAL_CTRL_CLK_OUT_EN_B_W { w: self }
    }
    #[doc = "Bit 12 - When low, all main ctrl signals are used instead of corresponding ctrl signal or some control bits of xtal_reg. They are: xo_en_b, ext_clk_mode and lp_mode. When high, corresponding ctrl signal and some control bits of xtal_reg are used instead of main ctrl signals. They are: xo_en_b_reg, ext_clk_mode (bit 0) and lp_mode (bit 1)."]
    #[inline(always)]
    pub fn xtal_ctrl_reg_value_sel(&mut self) -> XTAL_CTRL_REG_VALUE_SEL_W {
        XTAL_CTRL_REG_VALUE_SEL_W { w: self }
    }
    #[doc = "Bits 10:11 - Selection of the current before amplitude stabilization but after starting-up in active transistors of the core oscillator: '00': typ. 0.15 mA, '01': typ. 0.24 mA, '10': typ. 0.40 mA, '11': typ. 0.61 mA"]
    #[inline(always)]
    pub fn xtal_ctrl_afterstartup_curr_sel(&mut self) -> XTAL_CTRL_AFTERSTARTUP_CURR_SEL_W {
        XTAL_CTRL_AFTERSTARTUP_CURR_SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Selection of the starting-up current in active transistors of the core oscillator: '00': typ. 0.41 mA, '01': typ. 0.59 mA, '10': typ. 0.88 mA, '11': typ. 1.24 mA"]
    #[inline(always)]
    pub fn xtal_ctrl_startup_curr_sel(&mut self) -> XTAL_CTRL_STARTUP_CURR_SEL_W {
        XTAL_CTRL_STARTUP_CURR_SEL_W { w: self }
    }
    #[doc = "Bit 7 - Invert clock on clk_dig output"]
    #[inline(always)]
    pub fn xtal_ctrl_inv_clk_dig(&mut self) -> XTAL_CTRL_INV_CLK_DIG_W {
        XTAL_CTRL_INV_CLK_DIG_W { w: self }
    }
    #[doc = "Bit 6 - Invert clock on clk_pll output"]
    #[inline(always)]
    pub fn xtal_ctrl_inv_clk_pll(&mut self) -> XTAL_CTRL_INV_CLK_PLL_W {
        XTAL_CTRL_INV_CLK_PLL_W { w: self }
    }
    #[doc = "Bit 5 - Debug: allow to force output clocks on clk_pll, clk_dig and clk_out (if these outputs are enabled) and bypass the xtal internal clock detector that gates these clock outputs."]
    #[inline(always)]
    pub fn xtal_ctrl_force_clk_ready(&mut self) -> XTAL_CTRL_FORCE_CLK_READY_W {
        XTAL_CTRL_FORCE_CLK_READY_W { w: self }
    }
    #[doc = "Bit 4 - When high, disable the output clock to go to digital (clk_dig output stay low)."]
    #[inline(always)]
    pub fn xtal_ctrl_clk_dig_en_b(&mut self) -> XTAL_CTRL_CLK_DIG_EN_B_W {
        XTAL_CTRL_CLK_DIG_EN_B_W { w: self }
    }
    #[doc = "Bit 3 - When low (and if xtal_en_b(_reg) is low), the xtal buffer is enabled otherwise it is disabled. Could be used to decrease the power consumption of the xtal while maintaining oscillation in the xtal oscillator"]
    #[inline(always)]
    pub fn xtal_ctrl_buff_en_b(&mut self) -> XTAL_CTRL_BUFF_EN_B_W {
        XTAL_CTRL_BUFF_EN_B_W { w: self }
    }
    #[doc = "Bit 2 - When high, bias current in the clock buffer is increased compared to normal operation (high bandwidth mode in 132 MHz clock input buffer)."]
    #[inline(always)]
    pub fn xtal_ctrl_hp_mode(&mut self) -> XTAL_CTRL_HP_MODE_W {
        XTAL_CTRL_HP_MODE_W { w: self }
    }
    #[doc = "Bit 1 - When high, bias current in the clock buffer is reduced compared to normal operation (low power mode). Usable only if bit 12 is high (see below) otherwise it is bypassed by lp_mode pin input on main interface"]
    #[inline(always)]
    pub fn xtal_ctrl_lp_mode(&mut self) -> XTAL_CTRL_LP_MODE_W {
        XTAL_CTRL_LP_MODE_W { w: self }
    }
    #[doc = "Bit 0 - When high, allow to uses xtal_p (and eventually xtal_n) has external clock input(s). The XTAL oscillator core is disabled. Usable only if bit 12 is high (see below) otherwise it is bypassed by ext_clk_mode pin input on main interface"]
    #[inline(always)]
    pub fn xtal_ctrl_ext_clk_mode(&mut self) -> XTAL_CTRL_EXT_CLK_MODE_W {
        XTAL_CTRL_EXT_CLK_MODE_W { w: self }
    }
}
