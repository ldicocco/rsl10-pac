#[doc = "Reader of register CLK_DET_CFG"]
pub type R = crate::R<u32, super::CLK_DET_CFG>;
#[doc = "Writer for register CLK_DET_CFG"]
pub type W = crate::W<u32, super::CLK_DET_CFG>;
#[doc = "Register CLK_DET_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_DET_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock detector source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_DET_SEL_A {
    #[doc = "0: Select EXTCLK source"]
    CLK_DET_SEL_EXTCLK = 0,
    #[doc = "1: Select JTCK source"]
    CLK_DET_SEL_JTCK = 1,
}
impl From<CLK_DET_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_DET_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLK_DET_SEL`"]
pub type CLK_DET_SEL_R = crate::R<bool, CLK_DET_SEL_A>;
impl CLK_DET_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_DET_SEL_A {
        match self.bits {
            false => CLK_DET_SEL_A::CLK_DET_SEL_EXTCLK,
            true => CLK_DET_SEL_A::CLK_DET_SEL_JTCK,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DET_SEL_EXTCLK`"]
    #[inline(always)]
    pub fn is_clk_det_sel_extclk(&self) -> bool {
        *self == CLK_DET_SEL_A::CLK_DET_SEL_EXTCLK
    }
    #[doc = "Checks if the value of the field is `CLK_DET_SEL_JTCK`"]
    #[inline(always)]
    pub fn is_clk_det_sel_jtck(&self) -> bool {
        *self == CLK_DET_SEL_A::CLK_DET_SEL_JTCK
    }
}
#[doc = "Write proxy for field `CLK_DET_SEL`"]
pub struct CLK_DET_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DET_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_DET_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Select EXTCLK source"]
    #[inline(always)]
    pub fn clk_det_sel_extclk(self) -> &'a mut W {
        self.variant(CLK_DET_SEL_A::CLK_DET_SEL_EXTCLK)
    }
    #[doc = "Select JTCK source"]
    #[inline(always)]
    pub fn clk_det_sel_jtck(self) -> &'a mut W {
        self.variant(CLK_DET_SEL_A::CLK_DET_SEL_JTCK)
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
#[doc = "Clock detector interrupt configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_DET_INT_SEL_A {
    #[doc = "0: Clock detector interrupt disabled"]
    CLK_DET_INT_DISABLE = 0,
    #[doc = "1: If the clock source becomes active an interrupt is created"]
    CLK_DET_INT_ACTIVATED = 1,
    #[doc = "2: If the clock source becomes inactive an interrupt is created"]
    CLK_DET_INT_DEACTIVATED = 2,
    #[doc = "3: Any the clock source activity change will create an interrupt"]
    CLK_DET_INT_ACTIVITY_CHANGE = 3,
}
impl From<CLK_DET_INT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_DET_INT_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLK_DET_INT_SEL`"]
pub type CLK_DET_INT_SEL_R = crate::R<u8, CLK_DET_INT_SEL_A>;
impl CLK_DET_INT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_DET_INT_SEL_A {
        match self.bits {
            0 => CLK_DET_INT_SEL_A::CLK_DET_INT_DISABLE,
            1 => CLK_DET_INT_SEL_A::CLK_DET_INT_ACTIVATED,
            2 => CLK_DET_INT_SEL_A::CLK_DET_INT_DEACTIVATED,
            3 => CLK_DET_INT_SEL_A::CLK_DET_INT_ACTIVITY_CHANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DET_INT_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_det_int_disable(&self) -> bool {
        *self == CLK_DET_INT_SEL_A::CLK_DET_INT_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_DET_INT_ACTIVATED`"]
    #[inline(always)]
    pub fn is_clk_det_int_activated(&self) -> bool {
        *self == CLK_DET_INT_SEL_A::CLK_DET_INT_ACTIVATED
    }
    #[doc = "Checks if the value of the field is `CLK_DET_INT_DEACTIVATED`"]
    #[inline(always)]
    pub fn is_clk_det_int_deactivated(&self) -> bool {
        *self == CLK_DET_INT_SEL_A::CLK_DET_INT_DEACTIVATED
    }
    #[doc = "Checks if the value of the field is `CLK_DET_INT_ACTIVITY_CHANGE`"]
    #[inline(always)]
    pub fn is_clk_det_int_activity_change(&self) -> bool {
        *self == CLK_DET_INT_SEL_A::CLK_DET_INT_ACTIVITY_CHANGE
    }
}
#[doc = "Write proxy for field `CLK_DET_INT_SEL`"]
pub struct CLK_DET_INT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DET_INT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_DET_INT_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock detector interrupt disabled"]
    #[inline(always)]
    pub fn clk_det_int_disable(self) -> &'a mut W {
        self.variant(CLK_DET_INT_SEL_A::CLK_DET_INT_DISABLE)
    }
    #[doc = "If the clock source becomes active an interrupt is created"]
    #[inline(always)]
    pub fn clk_det_int_activated(self) -> &'a mut W {
        self.variant(CLK_DET_INT_SEL_A::CLK_DET_INT_ACTIVATED)
    }
    #[doc = "If the clock source becomes inactive an interrupt is created"]
    #[inline(always)]
    pub fn clk_det_int_deactivated(self) -> &'a mut W {
        self.variant(CLK_DET_INT_SEL_A::CLK_DET_INT_DEACTIVATED)
    }
    #[doc = "Any the clock source activity change will create an interrupt"]
    #[inline(always)]
    pub fn clk_det_int_activity_change(self) -> &'a mut W {
        self.variant(CLK_DET_INT_SEL_A::CLK_DET_INT_ACTIVITY_CHANGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Clock detector configuration - Not used when running on standby clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_DET_DIV_A {
    #[doc = "0: EXTCLK or JTCK detector runs on SLOWCLK divided by 32"]
    CLK_DET_SLOWCLK_DIV32 = 0,
    #[doc = "1: EXTCLK or JTCK detector runs on SLOWCLK divided by 64"]
    CLK_DET_SLOWCLK_DIV64 = 1,
    #[doc = "2: EXTCLK or JTCK detector runs on SLOWCLK divided by 96"]
    CLK_DET_SLOWCLK_DIV96 = 2,
    #[doc = "3: EXTCLK or JTCK detector runs on SLOWCLK divided by 128"]
    CLK_DET_SLOWCLK_DIV128 = 3,
}
impl From<CLK_DET_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_DET_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLK_DET_DIV`"]
pub type CLK_DET_DIV_R = crate::R<u8, CLK_DET_DIV_A>;
impl CLK_DET_DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_DET_DIV_A {
        match self.bits {
            0 => CLK_DET_DIV_A::CLK_DET_SLOWCLK_DIV32,
            1 => CLK_DET_DIV_A::CLK_DET_SLOWCLK_DIV64,
            2 => CLK_DET_DIV_A::CLK_DET_SLOWCLK_DIV96,
            3 => CLK_DET_DIV_A::CLK_DET_SLOWCLK_DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DET_SLOWCLK_DIV32`"]
    #[inline(always)]
    pub fn is_clk_det_slowclk_div32(&self) -> bool {
        *self == CLK_DET_DIV_A::CLK_DET_SLOWCLK_DIV32
    }
    #[doc = "Checks if the value of the field is `CLK_DET_SLOWCLK_DIV64`"]
    #[inline(always)]
    pub fn is_clk_det_slowclk_div64(&self) -> bool {
        *self == CLK_DET_DIV_A::CLK_DET_SLOWCLK_DIV64
    }
    #[doc = "Checks if the value of the field is `CLK_DET_SLOWCLK_DIV96`"]
    #[inline(always)]
    pub fn is_clk_det_slowclk_div96(&self) -> bool {
        *self == CLK_DET_DIV_A::CLK_DET_SLOWCLK_DIV96
    }
    #[doc = "Checks if the value of the field is `CLK_DET_SLOWCLK_DIV128`"]
    #[inline(always)]
    pub fn is_clk_det_slowclk_div128(&self) -> bool {
        *self == CLK_DET_DIV_A::CLK_DET_SLOWCLK_DIV128
    }
}
#[doc = "Write proxy for field `CLK_DET_DIV`"]
pub struct CLK_DET_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DET_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_DET_DIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "EXTCLK or JTCK detector runs on SLOWCLK divided by 32"]
    #[inline(always)]
    pub fn clk_det_slowclk_div32(self) -> &'a mut W {
        self.variant(CLK_DET_DIV_A::CLK_DET_SLOWCLK_DIV32)
    }
    #[doc = "EXTCLK or JTCK detector runs on SLOWCLK divided by 64"]
    #[inline(always)]
    pub fn clk_det_slowclk_div64(self) -> &'a mut W {
        self.variant(CLK_DET_DIV_A::CLK_DET_SLOWCLK_DIV64)
    }
    #[doc = "EXTCLK or JTCK detector runs on SLOWCLK divided by 96"]
    #[inline(always)]
    pub fn clk_det_slowclk_div96(self) -> &'a mut W {
        self.variant(CLK_DET_DIV_A::CLK_DET_SLOWCLK_DIV96)
    }
    #[doc = "EXTCLK or JTCK detector runs on SLOWCLK divided by 128"]
    #[inline(always)]
    pub fn clk_det_slowclk_div128(self) -> &'a mut W {
        self.variant(CLK_DET_DIV_A::CLK_DET_SLOWCLK_DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Clock detector enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_DET_ENABLE_A {
    #[doc = "0: No clock detected"]
    CLK_DET_DISABLE = 0,
    #[doc = "1: Clock detected"]
    CLK_DET_ENABLE = 1,
}
impl From<CLK_DET_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_DET_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLK_DET_ENABLE`"]
pub type CLK_DET_ENABLE_R = crate::R<bool, CLK_DET_ENABLE_A>;
impl CLK_DET_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_DET_ENABLE_A {
        match self.bits {
            false => CLK_DET_ENABLE_A::CLK_DET_DISABLE,
            true => CLK_DET_ENABLE_A::CLK_DET_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DET_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_det_disable(&self) -> bool {
        *self == CLK_DET_ENABLE_A::CLK_DET_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_DET_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_det_enable(&self) -> bool {
        *self == CLK_DET_ENABLE_A::CLK_DET_ENABLE
    }
}
#[doc = "Write proxy for field `CLK_DET_ENABLE`"]
pub struct CLK_DET_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DET_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_DET_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No clock detected"]
    #[inline(always)]
    pub fn clk_det_disable(self) -> &'a mut W {
        self.variant(CLK_DET_ENABLE_A::CLK_DET_DISABLE)
    }
    #[doc = "Clock detected"]
    #[inline(always)]
    pub fn clk_det_enable(self) -> &'a mut W {
        self.variant(CLK_DET_ENABLE_A::CLK_DET_ENABLE)
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
    #[doc = "Bit 5 - Clock detector source selection"]
    #[inline(always)]
    pub fn clk_det_sel(&self) -> CLK_DET_SEL_R {
        CLK_DET_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Clock detector interrupt configuration"]
    #[inline(always)]
    pub fn clk_det_int_sel(&self) -> CLK_DET_INT_SEL_R {
        CLK_DET_INT_SEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 1:2 - Clock detector configuration - Not used when running on standby clock"]
    #[inline(always)]
    pub fn clk_det_div(&self) -> CLK_DET_DIV_R {
        CLK_DET_DIV_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - Clock detector enable/disable"]
    #[inline(always)]
    pub fn clk_det_enable(&self) -> CLK_DET_ENABLE_R {
        CLK_DET_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Clock detector source selection"]
    #[inline(always)]
    pub fn clk_det_sel(&mut self) -> CLK_DET_SEL_W {
        CLK_DET_SEL_W { w: self }
    }
    #[doc = "Bits 3:4 - Clock detector interrupt configuration"]
    #[inline(always)]
    pub fn clk_det_int_sel(&mut self) -> CLK_DET_INT_SEL_W {
        CLK_DET_INT_SEL_W { w: self }
    }
    #[doc = "Bits 1:2 - Clock detector configuration - Not used when running on standby clock"]
    #[inline(always)]
    pub fn clk_det_div(&mut self) -> CLK_DET_DIV_W {
        CLK_DET_DIV_W { w: self }
    }
    #[doc = "Bit 0 - Clock detector enable/disable"]
    #[inline(always)]
    pub fn clk_det_enable(&mut self) -> CLK_DET_ENABLE_W {
        CLK_DET_ENABLE_W { w: self }
    }
}
