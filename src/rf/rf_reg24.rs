#[doc = "Reader of register RF_REG24"]
pub type R = crate::R<u32, super::RF_REG24>;
#[doc = "Writer for register RF_REG24"]
pub type W = crate::W<u32, super::RF_REG24>;
#[doc = "Register RF_REG24 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG24 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "VCO bias for Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_5_IQ_PLL_4_RX_A {
    #[doc = "0: `0`"]
    BIAS_5_IQ_PLL_4_RX_DEFAULT = 0,
}
impl From<BIAS_5_IQ_PLL_4_RX_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_5_IQ_PLL_4_RX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_5_IQ_PLL_4_RX`"]
pub type BIAS_5_IQ_PLL_4_RX_R = crate::R<u8, BIAS_5_IQ_PLL_4_RX_A>;
impl BIAS_5_IQ_PLL_4_RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_5_IQ_PLL_4_RX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_5_IQ_PLL_4_RX_A::BIAS_5_IQ_PLL_4_RX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_5_IQ_PLL_4_RX_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_5_iq_pll_4_rx_default(&self) -> bool {
        *self == BIAS_5_IQ_PLL_4_RX_A::BIAS_5_IQ_PLL_4_RX_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_5_IQ_PLL_4_RX`"]
pub struct BIAS_5_IQ_PLL_4_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_5_IQ_PLL_4_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_5_IQ_PLL_4_RX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_5_iq_pll_4_rx_default(self) -> &'a mut W {
        self.variant(BIAS_5_IQ_PLL_4_RX_A::BIAS_5_IQ_PLL_4_RX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "VCO bias for Tx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_5_IQ_PLL_4_TX_A {
    #[doc = "0: `0`"]
    BIAS_5_IQ_PLL_4_TX_DEFAULT = 0,
}
impl From<BIAS_5_IQ_PLL_4_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_5_IQ_PLL_4_TX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_5_IQ_PLL_4_TX`"]
pub type BIAS_5_IQ_PLL_4_TX_R = crate::R<u8, BIAS_5_IQ_PLL_4_TX_A>;
impl BIAS_5_IQ_PLL_4_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_5_IQ_PLL_4_TX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_5_IQ_PLL_4_TX_A::BIAS_5_IQ_PLL_4_TX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_5_IQ_PLL_4_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_5_iq_pll_4_tx_default(&self) -> bool {
        *self == BIAS_5_IQ_PLL_4_TX_A::BIAS_5_IQ_PLL_4_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_5_IQ_PLL_4_TX`"]
pub struct BIAS_5_IQ_PLL_4_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_5_IQ_PLL_4_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_5_IQ_PLL_4_TX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_5_iq_pll_4_tx_default(self) -> &'a mut W {
        self.variant(BIAS_5_IQ_PLL_4_TX_A::BIAS_5_IQ_PLL_4_TX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Sub-band comparator bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_4_IQ_PLL_2_A {
    #[doc = "0: `0`"]
    BIAS_4_IQ_PLL_2_DEFAULT = 0,
}
impl From<BIAS_4_IQ_PLL_2_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_4_IQ_PLL_2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_4_IQ_PLL_2`"]
pub type BIAS_4_IQ_PLL_2_R = crate::R<u8, BIAS_4_IQ_PLL_2_A>;
impl BIAS_4_IQ_PLL_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_4_IQ_PLL_2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_4_IQ_PLL_2_A::BIAS_4_IQ_PLL_2_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_4_IQ_PLL_2_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_4_iq_pll_2_default(&self) -> bool {
        *self == BIAS_4_IQ_PLL_2_A::BIAS_4_IQ_PLL_2_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_4_IQ_PLL_2`"]
pub struct BIAS_4_IQ_PLL_2_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_4_IQ_PLL_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_4_IQ_PLL_2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_4_iq_pll_2_default(self) -> &'a mut W {
        self.variant(BIAS_4_IQ_PLL_2_A::BIAS_4_IQ_PLL_2_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Dynamic divider bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_4_IQ_PLL_1_A {
    #[doc = "0: `0`"]
    BIAS_4_IQ_PLL_1_DEFAULT = 0,
}
impl From<BIAS_4_IQ_PLL_1_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_4_IQ_PLL_1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_4_IQ_PLL_1`"]
pub type BIAS_4_IQ_PLL_1_R = crate::R<u8, BIAS_4_IQ_PLL_1_A>;
impl BIAS_4_IQ_PLL_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_4_IQ_PLL_1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_4_IQ_PLL_1_A::BIAS_4_IQ_PLL_1_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_4_IQ_PLL_1_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_4_iq_pll_1_default(&self) -> bool {
        *self == BIAS_4_IQ_PLL_1_A::BIAS_4_IQ_PLL_1_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_4_IQ_PLL_1`"]
pub struct BIAS_4_IQ_PLL_1_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_4_IQ_PLL_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_4_IQ_PLL_1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_4_iq_pll_1_default(self) -> &'a mut W {
        self.variant(BIAS_4_IQ_PLL_1_A::BIAS_4_IQ_PLL_1_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "IFA ctrl_c bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_3_IQ_RXTX_8_A {
    #[doc = "0: `0`"]
    BIAS_3_IQ_RXTX_8_DEFAULT = 0,
}
impl From<BIAS_3_IQ_RXTX_8_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_3_IQ_RXTX_8_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_3_IQ_RXTX_8`"]
pub type BIAS_3_IQ_RXTX_8_R = crate::R<u8, BIAS_3_IQ_RXTX_8_A>;
impl BIAS_3_IQ_RXTX_8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_3_IQ_RXTX_8_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_3_IQ_RXTX_8_A::BIAS_3_IQ_RXTX_8_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_3_IQ_RXTX_8_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_3_iq_rxtx_8_default(&self) -> bool {
        *self == BIAS_3_IQ_RXTX_8_A::BIAS_3_IQ_RXTX_8_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_3_IQ_RXTX_8`"]
pub struct BIAS_3_IQ_RXTX_8_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_3_IQ_RXTX_8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_3_IQ_RXTX_8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_3_iq_rxtx_8_default(self) -> &'a mut W {
        self.variant(BIAS_3_IQ_RXTX_8_A::BIAS_3_IQ_RXTX_8_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "IFA ctrl_r bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_3_IQ_RXTX_7_A {
    #[doc = "0: `0`"]
    BIAS_3_IQ_RXTX_7_DEFAULT = 0,
}
impl From<BIAS_3_IQ_RXTX_7_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_3_IQ_RXTX_7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_3_IQ_RXTX_7`"]
pub type BIAS_3_IQ_RXTX_7_R = crate::R<u8, BIAS_3_IQ_RXTX_7_A>;
impl BIAS_3_IQ_RXTX_7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_3_IQ_RXTX_7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_3_IQ_RXTX_7_A::BIAS_3_IQ_RXTX_7_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_3_IQ_RXTX_7_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_3_iq_rxtx_7_default(&self) -> bool {
        *self == BIAS_3_IQ_RXTX_7_A::BIAS_3_IQ_RXTX_7_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_3_IQ_RXTX_7`"]
pub struct BIAS_3_IQ_RXTX_7_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_3_IQ_RXTX_7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_3_IQ_RXTX_7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_3_iq_rxtx_7_default(self) -> &'a mut W {
        self.variant(BIAS_3_IQ_RXTX_7_A::BIAS_3_IQ_RXTX_7_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "VCOM_MX bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_2_IQ_RXTX_6_A {
    #[doc = "0: `0`"]
    BIAS_2_IQ_RXTX_6_DEFAULT = 0,
}
impl From<BIAS_2_IQ_RXTX_6_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_2_IQ_RXTX_6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_2_IQ_RXTX_6`"]
pub type BIAS_2_IQ_RXTX_6_R = crate::R<u8, BIAS_2_IQ_RXTX_6_A>;
impl BIAS_2_IQ_RXTX_6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_2_IQ_RXTX_6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_2_IQ_RXTX_6_A::BIAS_2_IQ_RXTX_6_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_2_IQ_RXTX_6_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_2_iq_rxtx_6_default(&self) -> bool {
        *self == BIAS_2_IQ_RXTX_6_A::BIAS_2_IQ_RXTX_6_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_2_IQ_RXTX_6`"]
pub struct BIAS_2_IQ_RXTX_6_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_2_IQ_RXTX_6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_2_IQ_RXTX_6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_2_iq_rxtx_6_default(self) -> &'a mut W {
        self.variant(BIAS_2_IQ_RXTX_6_A::BIAS_2_IQ_RXTX_6_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "VCOM_LO bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_2_IQ_RXTX_5_A {
    #[doc = "0: `0`"]
    BIAS_2_IQ_RXTX_5_DEFAULT = 0,
}
impl From<BIAS_2_IQ_RXTX_5_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_2_IQ_RXTX_5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_2_IQ_RXTX_5`"]
pub type BIAS_2_IQ_RXTX_5_R = crate::R<u8, BIAS_2_IQ_RXTX_5_A>;
impl BIAS_2_IQ_RXTX_5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_2_IQ_RXTX_5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_2_IQ_RXTX_5_A::BIAS_2_IQ_RXTX_5_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_2_IQ_RXTX_5_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_2_iq_rxtx_5_default(&self) -> bool {
        *self == BIAS_2_IQ_RXTX_5_A::BIAS_2_IQ_RXTX_5_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_2_IQ_RXTX_5`"]
pub struct BIAS_2_IQ_RXTX_5_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_2_IQ_RXTX_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_2_IQ_RXTX_5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_2_iq_rxtx_5_default(self) -> &'a mut W {
        self.variant(BIAS_2_IQ_RXTX_5_A::BIAS_2_IQ_RXTX_5_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - VCO bias for Rx"]
    #[inline(always)]
    pub fn bias_5_iq_pll_4_rx(&self) -> BIAS_5_IQ_PLL_4_RX_R {
        BIAS_5_IQ_PLL_4_RX_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - VCO bias for Tx"]
    #[inline(always)]
    pub fn bias_5_iq_pll_4_tx(&self) -> BIAS_5_IQ_PLL_4_TX_R {
        BIAS_5_IQ_PLL_4_TX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Sub-band comparator bias"]
    #[inline(always)]
    pub fn bias_4_iq_pll_2(&self) -> BIAS_4_IQ_PLL_2_R {
        BIAS_4_IQ_PLL_2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Dynamic divider bias"]
    #[inline(always)]
    pub fn bias_4_iq_pll_1(&self) -> BIAS_4_IQ_PLL_1_R {
        BIAS_4_IQ_PLL_1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - IFA ctrl_c bias"]
    #[inline(always)]
    pub fn bias_3_iq_rxtx_8(&self) -> BIAS_3_IQ_RXTX_8_R {
        BIAS_3_IQ_RXTX_8_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - IFA ctrl_r bias"]
    #[inline(always)]
    pub fn bias_3_iq_rxtx_7(&self) -> BIAS_3_IQ_RXTX_7_R {
        BIAS_3_IQ_RXTX_7_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - VCOM_MX bias"]
    #[inline(always)]
    pub fn bias_2_iq_rxtx_6(&self) -> BIAS_2_IQ_RXTX_6_R {
        BIAS_2_IQ_RXTX_6_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - VCOM_LO bias"]
    #[inline(always)]
    pub fn bias_2_iq_rxtx_5(&self) -> BIAS_2_IQ_RXTX_5_R {
        BIAS_2_IQ_RXTX_5_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - VCO bias for Rx"]
    #[inline(always)]
    pub fn bias_5_iq_pll_4_rx(&mut self) -> BIAS_5_IQ_PLL_4_RX_W {
        BIAS_5_IQ_PLL_4_RX_W { w: self }
    }
    #[doc = "Bits 24:27 - VCO bias for Tx"]
    #[inline(always)]
    pub fn bias_5_iq_pll_4_tx(&mut self) -> BIAS_5_IQ_PLL_4_TX_W {
        BIAS_5_IQ_PLL_4_TX_W { w: self }
    }
    #[doc = "Bits 20:23 - Sub-band comparator bias"]
    #[inline(always)]
    pub fn bias_4_iq_pll_2(&mut self) -> BIAS_4_IQ_PLL_2_W {
        BIAS_4_IQ_PLL_2_W { w: self }
    }
    #[doc = "Bits 16:19 - Dynamic divider bias"]
    #[inline(always)]
    pub fn bias_4_iq_pll_1(&mut self) -> BIAS_4_IQ_PLL_1_W {
        BIAS_4_IQ_PLL_1_W { w: self }
    }
    #[doc = "Bits 12:15 - IFA ctrl_c bias"]
    #[inline(always)]
    pub fn bias_3_iq_rxtx_8(&mut self) -> BIAS_3_IQ_RXTX_8_W {
        BIAS_3_IQ_RXTX_8_W { w: self }
    }
    #[doc = "Bits 8:11 - IFA ctrl_r bias"]
    #[inline(always)]
    pub fn bias_3_iq_rxtx_7(&mut self) -> BIAS_3_IQ_RXTX_7_W {
        BIAS_3_IQ_RXTX_7_W { w: self }
    }
    #[doc = "Bits 4:7 - VCOM_MX bias"]
    #[inline(always)]
    pub fn bias_2_iq_rxtx_6(&mut self) -> BIAS_2_IQ_RXTX_6_W {
        BIAS_2_IQ_RXTX_6_W { w: self }
    }
    #[doc = "Bits 0:3 - VCOM_LO bias"]
    #[inline(always)]
    pub fn bias_2_iq_rxtx_5(&mut self) -> BIAS_2_IQ_RXTX_5_W {
        BIAS_2_IQ_RXTX_5_W { w: self }
    }
}
