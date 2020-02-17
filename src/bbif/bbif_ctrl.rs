#[doc = "Reader of register BBIF_CTRL"]
pub type R = crate::R<u32, super::BBIF_CTRL>;
#[doc = "Writer for register BBIF_CTRL"]
pub type W = crate::W<u32, super::BBIF_CTRL>;
#[doc = "Register BBIF_CTRL `reset()`'s with value 0x80"]
impl crate::ResetValue for super::BBIF_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "External wake up request used to sort-out sleep modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_REQ_A {
    #[doc = "0: Keep the baseband controller in deep sleep mode"]
    BB_DEEP_SLEEP = 0,
    #[doc = "1: Wake up the baseband controller and keep it active"]
    BB_WAKEUP = 1,
}
impl From<WAKEUP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAKEUP_REQ`"]
pub type WAKEUP_REQ_R = crate::R<bool, WAKEUP_REQ_A>;
impl WAKEUP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP_REQ_A {
        match self.bits {
            false => WAKEUP_REQ_A::BB_DEEP_SLEEP,
            true => WAKEUP_REQ_A::BB_WAKEUP,
        }
    }
    #[doc = "Checks if the value of the field is `BB_DEEP_SLEEP`"]
    #[inline(always)]
    pub fn is_bb_deep_sleep(&self) -> bool {
        *self == WAKEUP_REQ_A::BB_DEEP_SLEEP
    }
    #[doc = "Checks if the value of the field is `BB_WAKEUP`"]
    #[inline(always)]
    pub fn is_bb_wakeup(&self) -> bool {
        *self == WAKEUP_REQ_A::BB_WAKEUP
    }
}
#[doc = "Write proxy for field `WAKEUP_REQ`"]
pub struct WAKEUP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Keep the baseband controller in deep sleep mode"]
    #[inline(always)]
    pub fn bb_deep_sleep(self) -> &'a mut W {
        self.variant(WAKEUP_REQ_A::BB_DEEP_SLEEP)
    }
    #[doc = "Wake up the baseband controller and keep it active"]
    #[inline(always)]
    pub fn bb_wakeup(self) -> &'a mut W {
        self.variant(WAKEUP_REQ_A::BB_WAKEUP)
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
#[doc = "Configure the internal baseband controller clock divider in order to provide a 1MHz reference clock\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_SEL_A {
    #[doc = "6: Divide the BBCLK by 6 (minimum authorized value)"]
    BBCLK_DIVIDER_6 = 6,
    #[doc = "8: Divide the BBCLK by 8"]
    BBCLK_DIVIDER_8 = 8,
    #[doc = "12: Divide the BBCLK by 12"]
    BBCLK_DIVIDER_12 = 12,
    #[doc = "16: Divide the BBCLK by 16"]
    BBCLK_DIVIDER_16 = 16,
    #[doc = "24: Divide the BBCLK by 24 (maximum authorized value)"]
    BBCLK_DIVIDER_24 = 24,
}
impl From<CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLK_SEL`"]
pub type CLK_SEL_R = crate::R<u8, CLK_SEL_A>;
impl CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLK_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            6 => Val(CLK_SEL_A::BBCLK_DIVIDER_6),
            8 => Val(CLK_SEL_A::BBCLK_DIVIDER_8),
            12 => Val(CLK_SEL_A::BBCLK_DIVIDER_12),
            16 => Val(CLK_SEL_A::BBCLK_DIVIDER_16),
            24 => Val(CLK_SEL_A::BBCLK_DIVIDER_24),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BBCLK_DIVIDER_6`"]
    #[inline(always)]
    pub fn is_bbclk_divider_6(&self) -> bool {
        *self == CLK_SEL_A::BBCLK_DIVIDER_6
    }
    #[doc = "Checks if the value of the field is `BBCLK_DIVIDER_8`"]
    #[inline(always)]
    pub fn is_bbclk_divider_8(&self) -> bool {
        *self == CLK_SEL_A::BBCLK_DIVIDER_8
    }
    #[doc = "Checks if the value of the field is `BBCLK_DIVIDER_12`"]
    #[inline(always)]
    pub fn is_bbclk_divider_12(&self) -> bool {
        *self == CLK_SEL_A::BBCLK_DIVIDER_12
    }
    #[doc = "Checks if the value of the field is `BBCLK_DIVIDER_16`"]
    #[inline(always)]
    pub fn is_bbclk_divider_16(&self) -> bool {
        *self == CLK_SEL_A::BBCLK_DIVIDER_16
    }
    #[doc = "Checks if the value of the field is `BBCLK_DIVIDER_24`"]
    #[inline(always)]
    pub fn is_bbclk_divider_24(&self) -> bool {
        *self == CLK_SEL_A::BBCLK_DIVIDER_24
    }
}
#[doc = "Write proxy for field `CLK_SEL`"]
pub struct CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide the BBCLK by 6 (minimum authorized value)"]
    #[inline(always)]
    pub fn bbclk_divider_6(self) -> &'a mut W {
        self.variant(CLK_SEL_A::BBCLK_DIVIDER_6)
    }
    #[doc = "Divide the BBCLK by 8"]
    #[inline(always)]
    pub fn bbclk_divider_8(self) -> &'a mut W {
        self.variant(CLK_SEL_A::BBCLK_DIVIDER_8)
    }
    #[doc = "Divide the BBCLK by 12"]
    #[inline(always)]
    pub fn bbclk_divider_12(self) -> &'a mut W {
        self.variant(CLK_SEL_A::BBCLK_DIVIDER_12)
    }
    #[doc = "Divide the BBCLK by 16"]
    #[inline(always)]
    pub fn bbclk_divider_16(self) -> &'a mut W {
        self.variant(CLK_SEL_A::BBCLK_DIVIDER_16)
    }
    #[doc = "Divide the BBCLK by 24 (maximum authorized value)"]
    #[inline(always)]
    pub fn bbclk_divider_24(self) -> &'a mut W {
        self.variant(CLK_SEL_A::BBCLK_DIVIDER_24)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | (((value as u32) & 0x3f) << 4);
        self.w
    }
}
#[doc = "Enable the baseband controller clocks generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_ENABLE_A {
    #[doc = "0: Baseband controller clocks are gated"]
    BB_CLK_DISABLE = 0,
    #[doc = "1: Baseband controller clocks are generated"]
    BB_CLK_ENABLE = 1,
}
impl From<CLK_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLK_ENABLE`"]
pub type CLK_ENABLE_R = crate::R<bool, CLK_ENABLE_A>;
impl CLK_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_ENABLE_A {
        match self.bits {
            false => CLK_ENABLE_A::BB_CLK_DISABLE,
            true => CLK_ENABLE_A::BB_CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BB_CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_bb_clk_disable(&self) -> bool {
        *self == CLK_ENABLE_A::BB_CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `BB_CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_bb_clk_enable(&self) -> bool {
        *self == CLK_ENABLE_A::BB_CLK_ENABLE
    }
}
#[doc = "Write proxy for field `CLK_ENABLE`"]
pub struct CLK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Baseband controller clocks are gated"]
    #[inline(always)]
    pub fn bb_clk_disable(self) -> &'a mut W {
        self.variant(CLK_ENABLE_A::BB_CLK_DISABLE)
    }
    #[doc = "Baseband controller clocks are generated"]
    #[inline(always)]
    pub fn bb_clk_enable(self) -> &'a mut W {
        self.variant(CLK_ENABLE_A::BB_CLK_ENABLE)
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
    #[doc = "Bit 16 - External wake up request used to sort-out sleep modes"]
    #[inline(always)]
    pub fn wakeup_req(&self) -> WAKEUP_REQ_R {
        WAKEUP_REQ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 4:9 - Configure the internal baseband controller clock divider in order to provide a 1MHz reference clock"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bit 0 - Enable the baseband controller clocks generation"]
    #[inline(always)]
    pub fn clk_enable(&self) -> CLK_ENABLE_R {
        CLK_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - External wake up request used to sort-out sleep modes"]
    #[inline(always)]
    pub fn wakeup_req(&mut self) -> WAKEUP_REQ_W {
        WAKEUP_REQ_W { w: self }
    }
    #[doc = "Bits 4:9 - Configure the internal baseband controller clock divider in order to provide a 1MHz reference clock"]
    #[inline(always)]
    pub fn clk_sel(&mut self) -> CLK_SEL_W {
        CLK_SEL_W { w: self }
    }
    #[doc = "Bit 0 - Enable the baseband controller clocks generation"]
    #[inline(always)]
    pub fn clk_enable(&mut self) -> CLK_ENABLE_W {
        CLK_ENABLE_W { w: self }
    }
}
