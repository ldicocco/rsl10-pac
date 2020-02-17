#[doc = "Reader of register RF_REG20"]
pub type R = crate::R<u32, super::RF_REG20>;
#[doc = "Writer for register RF_REG20"]
pub type W = crate::W<u32, super::RF_REG20>;
#[doc = "Register RF_REG20 `reset()`'s with value 0x01ff_010f"]
impl crate::ResetValue for super::RF_REG20 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01ff_010f
    }
}
#[doc = "Time needed by the DLL blocks to switch on.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMINGS_3_T_DLL_A {
    #[doc = "0: `0`"]
    TIMINGS_3_T_DLL_DEFAULT = 0,
}
impl From<TIMINGS_3_T_DLL_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMINGS_3_T_DLL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMINGS_3_T_DLL`"]
pub type TIMINGS_3_T_DLL_R = crate::R<u8, TIMINGS_3_T_DLL_A>;
impl TIMINGS_3_T_DLL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMINGS_3_T_DLL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIMINGS_3_T_DLL_A::TIMINGS_3_T_DLL_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMINGS_3_T_DLL_DEFAULT`"]
    #[inline(always)]
    pub fn is_timings_3_t_dll_default(&self) -> bool {
        *self == TIMINGS_3_T_DLL_A::TIMINGS_3_T_DLL_DEFAULT
    }
}
#[doc = "Write proxy for field `TIMINGS_3_T_DLL`"]
pub struct TIMINGS_3_T_DLL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMINGS_3_T_DLL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMINGS_3_T_DLL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn timings_3_t_dll_default(self) -> &'a mut W {
        self.variant(TIMINGS_3_T_DLL_A::TIMINGS_3_T_DLL_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Time needed by the PLL blocks in Tx mode to switch on.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMINGS_3_T_PLL_TX_A {
    #[doc = "1: `1`"]
    TIMINGS_3_T_PLL_TX_DEFAULT = 1,
}
impl From<TIMINGS_3_T_PLL_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMINGS_3_T_PLL_TX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMINGS_3_T_PLL_TX`"]
pub type TIMINGS_3_T_PLL_TX_R = crate::R<u8, TIMINGS_3_T_PLL_TX_A>;
impl TIMINGS_3_T_PLL_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMINGS_3_T_PLL_TX_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(TIMINGS_3_T_PLL_TX_A::TIMINGS_3_T_PLL_TX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMINGS_3_T_PLL_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_timings_3_t_pll_tx_default(&self) -> bool {
        *self == TIMINGS_3_T_PLL_TX_A::TIMINGS_3_T_PLL_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `TIMINGS_3_T_PLL_TX`"]
pub struct TIMINGS_3_T_PLL_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMINGS_3_T_PLL_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMINGS_3_T_PLL_TX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn timings_3_t_pll_tx_default(self) -> &'a mut W {
        self.variant(TIMINGS_3_T_PLL_TX_A::TIMINGS_3_T_PLL_TX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Time needed by the subband algorithm to calibrate in Tx.\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMINGS_2_T_SUBBAND_TX_A {
    #[doc = "15: `1111`"]
    TIMINGS_2_T_SUBBAND_TX_DEFAULT = 15,
}
impl From<TIMINGS_2_T_SUBBAND_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMINGS_2_T_SUBBAND_TX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMINGS_2_T_SUBBAND_TX`"]
pub type TIMINGS_2_T_SUBBAND_TX_R = crate::R<u8, TIMINGS_2_T_SUBBAND_TX_A>;
impl TIMINGS_2_T_SUBBAND_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMINGS_2_T_SUBBAND_TX_A> {
        use crate::Variant::*;
        match self.bits {
            15 => Val(TIMINGS_2_T_SUBBAND_TX_A::TIMINGS_2_T_SUBBAND_TX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMINGS_2_T_SUBBAND_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_timings_2_t_subband_tx_default(&self) -> bool {
        *self == TIMINGS_2_T_SUBBAND_TX_A::TIMINGS_2_T_SUBBAND_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `TIMINGS_2_T_SUBBAND_TX`"]
pub struct TIMINGS_2_T_SUBBAND_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMINGS_2_T_SUBBAND_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMINGS_2_T_SUBBAND_TX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn timings_2_t_subband_tx_default(self) -> &'a mut W {
        self.variant(TIMINGS_2_T_SUBBAND_TX_A::TIMINGS_2_T_SUBBAND_TX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Time needed by the Tx RF blocks to switch on.\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMINGS_2_T_TX_RF_A {
    #[doc = "255: `11111111`"]
    TIMINGS_2_T_TX_RF_DEFAULT = 255,
}
impl From<TIMINGS_2_T_TX_RF_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMINGS_2_T_TX_RF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMINGS_2_T_TX_RF`"]
pub type TIMINGS_2_T_TX_RF_R = crate::R<u8, TIMINGS_2_T_TX_RF_A>;
impl TIMINGS_2_T_TX_RF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMINGS_2_T_TX_RF_A> {
        use crate::Variant::*;
        match self.bits {
            255 => Val(TIMINGS_2_T_TX_RF_A::TIMINGS_2_T_TX_RF_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMINGS_2_T_TX_RF_DEFAULT`"]
    #[inline(always)]
    pub fn is_timings_2_t_tx_rf_default(&self) -> bool {
        *self == TIMINGS_2_T_TX_RF_A::TIMINGS_2_T_TX_RF_DEFAULT
    }
}
#[doc = "Write proxy for field `TIMINGS_2_T_TX_RF`"]
pub struct TIMINGS_2_T_TX_RF_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMINGS_2_T_TX_RF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMINGS_2_T_TX_RF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`11111111`"]
    #[inline(always)]
    pub fn timings_2_t_tx_rf_default(self) -> &'a mut W {
        self.variant(TIMINGS_2_T_TX_RF_A::TIMINGS_2_T_TX_RF_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Fixes the granularity of the timer in Tx mode. The granularity is given by (2^(t_granularity-2))x1us\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMINGS_1_T_GRANULARITY_TX_A {
    #[doc = "0: `0`"]
    TIMINGS_1_T_GRANULARITY_TX_DEFAULT = 0,
}
impl From<TIMINGS_1_T_GRANULARITY_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMINGS_1_T_GRANULARITY_TX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMINGS_1_T_GRANULARITY_TX`"]
pub type TIMINGS_1_T_GRANULARITY_TX_R = crate::R<u8, TIMINGS_1_T_GRANULARITY_TX_A>;
impl TIMINGS_1_T_GRANULARITY_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMINGS_1_T_GRANULARITY_TX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIMINGS_1_T_GRANULARITY_TX_A::TIMINGS_1_T_GRANULARITY_TX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMINGS_1_T_GRANULARITY_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_timings_1_t_granularity_tx_default(&self) -> bool {
        *self == TIMINGS_1_T_GRANULARITY_TX_A::TIMINGS_1_T_GRANULARITY_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `TIMINGS_1_T_GRANULARITY_TX`"]
pub struct TIMINGS_1_T_GRANULARITY_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMINGS_1_T_GRANULARITY_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMINGS_1_T_GRANULARITY_TX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn timings_1_t_granularity_tx_default(self) -> &'a mut W {
        self.variant(TIMINGS_1_T_GRANULARITY_TX_A::TIMINGS_1_T_GRANULARITY_TX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Fixes the granularity of the timer in Rx mode. The granularity is given by (2^(t_granularity))x1us\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMINGS_1_T_GRANULARITY_RX_A {
    #[doc = "1: `1`"]
    TIMINGS_1_T_GRANULARITY_RX_DEFAULT = 1,
}
impl From<TIMINGS_1_T_GRANULARITY_RX_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMINGS_1_T_GRANULARITY_RX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMINGS_1_T_GRANULARITY_RX`"]
pub type TIMINGS_1_T_GRANULARITY_RX_R = crate::R<u8, TIMINGS_1_T_GRANULARITY_RX_A>;
impl TIMINGS_1_T_GRANULARITY_RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMINGS_1_T_GRANULARITY_RX_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(TIMINGS_1_T_GRANULARITY_RX_A::TIMINGS_1_T_GRANULARITY_RX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMINGS_1_T_GRANULARITY_RX_DEFAULT`"]
    #[inline(always)]
    pub fn is_timings_1_t_granularity_rx_default(&self) -> bool {
        *self == TIMINGS_1_T_GRANULARITY_RX_A::TIMINGS_1_T_GRANULARITY_RX_DEFAULT
    }
}
#[doc = "Write proxy for field `TIMINGS_1_T_GRANULARITY_RX`"]
pub struct TIMINGS_1_T_GRANULARITY_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMINGS_1_T_GRANULARITY_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMINGS_1_T_GRANULARITY_RX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn timings_1_t_granularity_rx_default(self) -> &'a mut W {
        self.variant(TIMINGS_1_T_GRANULARITY_RX_A::TIMINGS_1_T_GRANULARITY_RX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation.\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_LUT_5_AGC_LEVEL_11_HI_A {
    #[doc = "15: `1111`"]
    AGC_LUT_5_AGC_LEVEL_11_HI_DEFAULT = 15,
}
impl From<AGC_LUT_5_AGC_LEVEL_11_HI_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_LUT_5_AGC_LEVEL_11_HI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_LUT_5_AGC_LEVEL_11_HI`"]
pub type AGC_LUT_5_AGC_LEVEL_11_HI_R = crate::R<u8, AGC_LUT_5_AGC_LEVEL_11_HI_A>;
impl AGC_LUT_5_AGC_LEVEL_11_HI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_LUT_5_AGC_LEVEL_11_HI_A> {
        use crate::Variant::*;
        match self.bits {
            15 => Val(AGC_LUT_5_AGC_LEVEL_11_HI_A::AGC_LUT_5_AGC_LEVEL_11_HI_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_LUT_5_AGC_LEVEL_11_HI_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_lut_5_agc_level_11_hi_default(&self) -> bool {
        *self == AGC_LUT_5_AGC_LEVEL_11_HI_A::AGC_LUT_5_AGC_LEVEL_11_HI_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_LUT_5_AGC_LEVEL_11_HI`"]
pub struct AGC_LUT_5_AGC_LEVEL_11_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_LUT_5_AGC_LEVEL_11_HI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_LUT_5_AGC_LEVEL_11_HI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn agc_lut_5_agc_level_11_hi_default(self) -> &'a mut W {
        self.variant(AGC_LUT_5_AGC_LEVEL_11_HI_A::AGC_LUT_5_AGC_LEVEL_11_HI_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - Time needed by the DLL blocks to switch on."]
    #[inline(always)]
    pub fn timings_3_t_dll(&self) -> TIMINGS_3_T_DLL_R {
        TIMINGS_3_T_DLL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Time needed by the PLL blocks in Tx mode to switch on."]
    #[inline(always)]
    pub fn timings_3_t_pll_tx(&self) -> TIMINGS_3_T_PLL_TX_R {
        TIMINGS_3_T_PLL_TX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Time needed by the subband algorithm to calibrate in Tx."]
    #[inline(always)]
    pub fn timings_2_t_subband_tx(&self) -> TIMINGS_2_T_SUBBAND_TX_R {
        TIMINGS_2_T_SUBBAND_TX_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Time needed by the Tx RF blocks to switch on."]
    #[inline(always)]
    pub fn timings_2_t_tx_rf(&self) -> TIMINGS_2_T_TX_RF_R {
        TIMINGS_2_T_TX_RF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Fixes the granularity of the timer in Tx mode. The granularity is given by (2^(t_granularity-2))x1us"]
    #[inline(always)]
    pub fn timings_1_t_granularity_tx(&self) -> TIMINGS_1_T_GRANULARITY_TX_R {
        TIMINGS_1_T_GRANULARITY_TX_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Fixes the granularity of the timer in Rx mode. The granularity is given by (2^(t_granularity))x1us"]
    #[inline(always)]
    pub fn timings_1_t_granularity_rx(&self) -> TIMINGS_1_T_GRANULARITY_RX_R {
        TIMINGS_1_T_GRANULARITY_RX_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:3 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_5_agc_level_11_hi(&self) -> AGC_LUT_5_AGC_LEVEL_11_HI_R {
        AGC_LUT_5_AGC_LEVEL_11_HI_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Time needed by the DLL blocks to switch on."]
    #[inline(always)]
    pub fn timings_3_t_dll(&mut self) -> TIMINGS_3_T_DLL_W {
        TIMINGS_3_T_DLL_W { w: self }
    }
    #[doc = "Bits 24:27 - Time needed by the PLL blocks in Tx mode to switch on."]
    #[inline(always)]
    pub fn timings_3_t_pll_tx(&mut self) -> TIMINGS_3_T_PLL_TX_W {
        TIMINGS_3_T_PLL_TX_W { w: self }
    }
    #[doc = "Bits 20:23 - Time needed by the subband algorithm to calibrate in Tx."]
    #[inline(always)]
    pub fn timings_2_t_subband_tx(&mut self) -> TIMINGS_2_T_SUBBAND_TX_W {
        TIMINGS_2_T_SUBBAND_TX_W { w: self }
    }
    #[doc = "Bits 16:19 - Time needed by the Tx RF blocks to switch on."]
    #[inline(always)]
    pub fn timings_2_t_tx_rf(&mut self) -> TIMINGS_2_T_TX_RF_W {
        TIMINGS_2_T_TX_RF_W { w: self }
    }
    #[doc = "Bits 12:14 - Fixes the granularity of the timer in Tx mode. The granularity is given by (2^(t_granularity-2))x1us"]
    #[inline(always)]
    pub fn timings_1_t_granularity_tx(&mut self) -> TIMINGS_1_T_GRANULARITY_TX_W {
        TIMINGS_1_T_GRANULARITY_TX_W { w: self }
    }
    #[doc = "Bits 8:10 - Fixes the granularity of the timer in Rx mode. The granularity is given by (2^(t_granularity))x1us"]
    #[inline(always)]
    pub fn timings_1_t_granularity_rx(&mut self) -> TIMINGS_1_T_GRANULARITY_RX_W {
        TIMINGS_1_T_GRANULARITY_RX_W { w: self }
    }
    #[doc = "Bits 0:3 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_5_agc_level_11_hi(&mut self) -> AGC_LUT_5_AGC_LEVEL_11_HI_W {
        AGC_LUT_5_AGC_LEVEL_11_HI_W { w: self }
    }
}
