#[doc = "Reader of register CLK_SYS_CFG"]
pub type R = crate::R<u32, super::CLK_SYS_CFG>;
#[doc = "Writer for register CLK_SYS_CFG"]
pub type W = crate::W<u32, super::CLK_SYS_CFG>;
#[doc = "Register CLK_SYS_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_SYS_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Prescale value for the input clock from pad JTCK (1 to 16 in steps of 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum JTCK_PRESCALE_A {
    #[doc = "0: Divide by 1"]
    JTCK_PRESCALE_1 = 0,
    #[doc = "1: Divide by 2"]
    JTCK_PRESCALE_2 = 1,
    #[doc = "14: Divide by 15"]
    JTCK_PRESCALE_15 = 14,
    #[doc = "15: Divide by 16"]
    JTCK_PRESCALE_16 = 15,
}
impl From<JTCK_PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: JTCK_PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `JTCK_PRESCALE`"]
pub type JTCK_PRESCALE_R = crate::R<u8, JTCK_PRESCALE_A>;
impl JTCK_PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, JTCK_PRESCALE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(JTCK_PRESCALE_A::JTCK_PRESCALE_1),
            1 => Val(JTCK_PRESCALE_A::JTCK_PRESCALE_2),
            14 => Val(JTCK_PRESCALE_A::JTCK_PRESCALE_15),
            15 => Val(JTCK_PRESCALE_A::JTCK_PRESCALE_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `JTCK_PRESCALE_1`"]
    #[inline(always)]
    pub fn is_jtck_prescale_1(&self) -> bool {
        *self == JTCK_PRESCALE_A::JTCK_PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `JTCK_PRESCALE_2`"]
    #[inline(always)]
    pub fn is_jtck_prescale_2(&self) -> bool {
        *self == JTCK_PRESCALE_A::JTCK_PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `JTCK_PRESCALE_15`"]
    #[inline(always)]
    pub fn is_jtck_prescale_15(&self) -> bool {
        *self == JTCK_PRESCALE_A::JTCK_PRESCALE_15
    }
    #[doc = "Checks if the value of the field is `JTCK_PRESCALE_16`"]
    #[inline(always)]
    pub fn is_jtck_prescale_16(&self) -> bool {
        *self == JTCK_PRESCALE_A::JTCK_PRESCALE_16
    }
}
#[doc = "Write proxy for field `JTCK_PRESCALE`"]
pub struct JTCK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> JTCK_PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JTCK_PRESCALE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn jtck_prescale_1(self) -> &'a mut W {
        self.variant(JTCK_PRESCALE_A::JTCK_PRESCALE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn jtck_prescale_2(self) -> &'a mut W {
        self.variant(JTCK_PRESCALE_A::JTCK_PRESCALE_2)
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn jtck_prescale_15(self) -> &'a mut W {
        self.variant(JTCK_PRESCALE_A::JTCK_PRESCALE_15)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn jtck_prescale_16(self) -> &'a mut W {
        self.variant(JTCK_PRESCALE_A::JTCK_PRESCALE_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Prescale value for the input clock from pad EXTCLK (1 to 16 in steps of 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTCLK_PRESCALE_A {
    #[doc = "0: Divide by 1"]
    EXTCLK_PRESCALE_1 = 0,
    #[doc = "1: Divide by 2"]
    EXTCLK_PRESCALE_2 = 1,
    #[doc = "14: Divide by 15"]
    EXTCLK_PRESCALE_15 = 14,
    #[doc = "15: Divide by 16"]
    EXTCLK_PRESCALE_16 = 15,
}
impl From<EXTCLK_PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTCLK_PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTCLK_PRESCALE`"]
pub type EXTCLK_PRESCALE_R = crate::R<u8, EXTCLK_PRESCALE_A>;
impl EXTCLK_PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTCLK_PRESCALE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTCLK_PRESCALE_A::EXTCLK_PRESCALE_1),
            1 => Val(EXTCLK_PRESCALE_A::EXTCLK_PRESCALE_2),
            14 => Val(EXTCLK_PRESCALE_A::EXTCLK_PRESCALE_15),
            15 => Val(EXTCLK_PRESCALE_A::EXTCLK_PRESCALE_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EXTCLK_PRESCALE_1`"]
    #[inline(always)]
    pub fn is_extclk_prescale_1(&self) -> bool {
        *self == EXTCLK_PRESCALE_A::EXTCLK_PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `EXTCLK_PRESCALE_2`"]
    #[inline(always)]
    pub fn is_extclk_prescale_2(&self) -> bool {
        *self == EXTCLK_PRESCALE_A::EXTCLK_PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `EXTCLK_PRESCALE_15`"]
    #[inline(always)]
    pub fn is_extclk_prescale_15(&self) -> bool {
        *self == EXTCLK_PRESCALE_A::EXTCLK_PRESCALE_15
    }
    #[doc = "Checks if the value of the field is `EXTCLK_PRESCALE_16`"]
    #[inline(always)]
    pub fn is_extclk_prescale_16(&self) -> bool {
        *self == EXTCLK_PRESCALE_A::EXTCLK_PRESCALE_16
    }
}
#[doc = "Write proxy for field `EXTCLK_PRESCALE`"]
pub struct EXTCLK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTCLK_PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTCLK_PRESCALE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn extclk_prescale_1(self) -> &'a mut W {
        self.variant(EXTCLK_PRESCALE_A::EXTCLK_PRESCALE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn extclk_prescale_2(self) -> &'a mut W {
        self.variant(EXTCLK_PRESCALE_A::EXTCLK_PRESCALE_2)
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn extclk_prescale_15(self) -> &'a mut W {
        self.variant(EXTCLK_PRESCALE_A::EXTCLK_PRESCALE_15)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn extclk_prescale_16(self) -> &'a mut W {
        self.variant(EXTCLK_PRESCALE_A::EXTCLK_PRESCALE_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Controls the source of the system clock : JTCK, RFCLK, RCCLK, EXTCLK or STANDBYCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCLK_SRC_SEL_A {
    #[doc = "0: Select the RCCLK clock as SYSCLK clock source"]
    SYSCLK_CLKSRC_RCCLK = 0,
    #[doc = "1: Select the STANDBYCLK clock as SYSCLK clock source"]
    SYSCLK_CLKSRC_STANDBYCLK = 1,
    #[doc = "2: Select the RFCLK clock as SYSCLK clock source"]
    SYSCLK_CLKSRC_RFCLK = 2,
    #[doc = "3: Select the EXTCLK clock as SYSCLK clock source"]
    SYSCLK_CLKSRC_EXTCLK = 3,
    #[doc = "4: Select the JTCK clock as SYSCLK clock source"]
    SYSCLK_CLKSRC_JTCK = 4,
}
impl From<SYSCLK_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCLK_SRC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCLK_SRC_SEL`"]
pub type SYSCLK_SRC_SEL_R = crate::R<u8, SYSCLK_SRC_SEL_A>;
impl SYSCLK_SRC_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCLK_SRC_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSCLK_SRC_SEL_A::SYSCLK_CLKSRC_RCCLK),
            1 => Val(SYSCLK_SRC_SEL_A::SYSCLK_CLKSRC_STANDBYCLK),
            2 => Val(SYSCLK_SRC_SEL_A::SYSCLK_CLKSRC_RFCLK),
            3 => Val(SYSCLK_SRC_SEL_A::SYSCLK_CLKSRC_EXTCLK),
            4 => Val(SYSCLK_SRC_SEL_A::SYSCLK_CLKSRC_JTCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK_CLKSRC_RCCLK`"]
    #[inline(always)]
    pub fn is_sysclk_clksrc_rcclk(&self) -> bool {
        *self == SYSCLK_SRC_SEL_A::SYSCLK_CLKSRC_RCCLK
    }
    #[doc = "Checks if the value of the field is `SYSCLK_CLKSRC_STANDBYCLK`"]
    #[inline(always)]
    pub fn is_sysclk_clksrc_standbyclk(&self) -> bool {
        *self == SYSCLK_SRC_SEL_A::SYSCLK_CLKSRC_STANDBYCLK
    }
    #[doc = "Checks if the value of the field is `SYSCLK_CLKSRC_RFCLK`"]
    #[inline(always)]
    pub fn is_sysclk_clksrc_rfclk(&self) -> bool {
        *self == SYSCLK_SRC_SEL_A::SYSCLK_CLKSRC_RFCLK
    }
    #[doc = "Checks if the value of the field is `SYSCLK_CLKSRC_EXTCLK`"]
    #[inline(always)]
    pub fn is_sysclk_clksrc_extclk(&self) -> bool {
        *self == SYSCLK_SRC_SEL_A::SYSCLK_CLKSRC_EXTCLK
    }
    #[doc = "Checks if the value of the field is `SYSCLK_CLKSRC_JTCK`"]
    #[inline(always)]
    pub fn is_sysclk_clksrc_jtck(&self) -> bool {
        *self == SYSCLK_SRC_SEL_A::SYSCLK_CLKSRC_JTCK
    }
}
#[doc = "Write proxy for field `SYSCLK_SRC_SEL`"]
pub struct SYSCLK_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCLK_SRC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCLK_SRC_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select the RCCLK clock as SYSCLK clock source"]
    #[inline(always)]
    pub fn sysclk_clksrc_rcclk(self) -> &'a mut W {
        self.variant(SYSCLK_SRC_SEL_A::SYSCLK_CLKSRC_RCCLK)
    }
    #[doc = "Select the STANDBYCLK clock as SYSCLK clock source"]
    #[inline(always)]
    pub fn sysclk_clksrc_standbyclk(self) -> &'a mut W {
        self.variant(SYSCLK_SRC_SEL_A::SYSCLK_CLKSRC_STANDBYCLK)
    }
    #[doc = "Select the RFCLK clock as SYSCLK clock source"]
    #[inline(always)]
    pub fn sysclk_clksrc_rfclk(self) -> &'a mut W {
        self.variant(SYSCLK_SRC_SEL_A::SYSCLK_CLKSRC_RFCLK)
    }
    #[doc = "Select the EXTCLK clock as SYSCLK clock source"]
    #[inline(always)]
    pub fn sysclk_clksrc_extclk(self) -> &'a mut W {
        self.variant(SYSCLK_SRC_SEL_A::SYSCLK_CLKSRC_EXTCLK)
    }
    #[doc = "Select the JTCK clock as SYSCLK clock source"]
    #[inline(always)]
    pub fn sysclk_clksrc_jtck(self) -> &'a mut W {
        self.variant(SYSCLK_SRC_SEL_A::SYSCLK_CLKSRC_JTCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19 - Prescale value for the input clock from pad JTCK (1 to 16 in steps of 1)"]
    #[inline(always)]
    pub fn jtck_prescale(&self) -> JTCK_PRESCALE_R {
        JTCK_PRESCALE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Prescale value for the input clock from pad EXTCLK (1 to 16 in steps of 1)"]
    #[inline(always)]
    pub fn extclk_prescale(&self) -> EXTCLK_PRESCALE_R {
        EXTCLK_PRESCALE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:2 - Controls the source of the system clock : JTCK, RFCLK, RCCLK, EXTCLK or STANDBYCLK"]
    #[inline(always)]
    pub fn sysclk_src_sel(&self) -> SYSCLK_SRC_SEL_R {
        SYSCLK_SRC_SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Prescale value for the input clock from pad JTCK (1 to 16 in steps of 1)"]
    #[inline(always)]
    pub fn jtck_prescale(&mut self) -> JTCK_PRESCALE_W {
        JTCK_PRESCALE_W { w: self }
    }
    #[doc = "Bits 8:11 - Prescale value for the input clock from pad EXTCLK (1 to 16 in steps of 1)"]
    #[inline(always)]
    pub fn extclk_prescale(&mut self) -> EXTCLK_PRESCALE_W {
        EXTCLK_PRESCALE_W { w: self }
    }
    #[doc = "Bits 0:2 - Controls the source of the system clock : JTCK, RFCLK, RCCLK, EXTCLK or STANDBYCLK"]
    #[inline(always)]
    pub fn sysclk_src_sel(&mut self) -> SYSCLK_SRC_SEL_W {
        SYSCLK_SRC_SEL_W { w: self }
    }
}
