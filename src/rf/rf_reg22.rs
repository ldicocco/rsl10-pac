#[doc = "Reader of register RF_REG22"]
pub type R = crate::R<u32, super::RF_REG22>;
#[doc = "Writer for register RF_REG22"]
pub type W = crate::W<u32, super::RF_REG22>;
#[doc = "Register RF_REG22 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG22 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "If set to 1 enables filter Tx configuration for the fast Rx PLL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMING_FAST_RX_EN_FAST_RX_TXFILT_A {
    #[doc = "0: `0`"]
    TIMING_FAST_RX_EN_FAST_RX_TXFILT_DEFAULT = 0,
}
impl From<TIMING_FAST_RX_EN_FAST_RX_TXFILT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMING_FAST_RX_EN_FAST_RX_TXFILT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMING_FAST_RX_EN_FAST_RX_TXFILT`"]
pub type TIMING_FAST_RX_EN_FAST_RX_TXFILT_R = crate::R<bool, TIMING_FAST_RX_EN_FAST_RX_TXFILT_A>;
impl TIMING_FAST_RX_EN_FAST_RX_TXFILT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TIMING_FAST_RX_EN_FAST_RX_TXFILT_A> {
        use crate::Variant::*;
        match self.bits {
            false => {
                Val(TIMING_FAST_RX_EN_FAST_RX_TXFILT_A::TIMING_FAST_RX_EN_FAST_RX_TXFILT_DEFAULT)
            }
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMING_FAST_RX_EN_FAST_RX_TXFILT_DEFAULT`"]
    #[inline(always)]
    pub fn is_timing_fast_rx_en_fast_rx_txfilt_default(&self) -> bool {
        *self == TIMING_FAST_RX_EN_FAST_RX_TXFILT_A::TIMING_FAST_RX_EN_FAST_RX_TXFILT_DEFAULT
    }
}
#[doc = "Write proxy for field `TIMING_FAST_RX_EN_FAST_RX_TXFILT`"]
pub struct TIMING_FAST_RX_EN_FAST_RX_TXFILT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMING_FAST_RX_EN_FAST_RX_TXFILT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMING_FAST_RX_EN_FAST_RX_TXFILT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn timing_fast_rx_en_fast_rx_txfilt_default(self) -> &'a mut W {
        self.variant(TIMING_FAST_RX_EN_FAST_RX_TXFILT_A::TIMING_FAST_RX_EN_FAST_RX_TXFILT_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "If set to 1 enables the fast Rx PLL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMING_FAST_RX_EN_FAST_RX_A {
    #[doc = "0: `0`"]
    TIMING_FAST_RX_EN_FAST_RX_DEFAULT = 0,
}
impl From<TIMING_FAST_RX_EN_FAST_RX_A> for bool {
    #[inline(always)]
    fn from(variant: TIMING_FAST_RX_EN_FAST_RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMING_FAST_RX_EN_FAST_RX`"]
pub type TIMING_FAST_RX_EN_FAST_RX_R = crate::R<bool, TIMING_FAST_RX_EN_FAST_RX_A>;
impl TIMING_FAST_RX_EN_FAST_RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TIMING_FAST_RX_EN_FAST_RX_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(TIMING_FAST_RX_EN_FAST_RX_A::TIMING_FAST_RX_EN_FAST_RX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMING_FAST_RX_EN_FAST_RX_DEFAULT`"]
    #[inline(always)]
    pub fn is_timing_fast_rx_en_fast_rx_default(&self) -> bool {
        *self == TIMING_FAST_RX_EN_FAST_RX_A::TIMING_FAST_RX_EN_FAST_RX_DEFAULT
    }
}
#[doc = "Write proxy for field `TIMING_FAST_RX_EN_FAST_RX`"]
pub struct TIMING_FAST_RX_EN_FAST_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMING_FAST_RX_EN_FAST_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMING_FAST_RX_EN_FAST_RX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn timing_fast_rx_en_fast_rx_default(self) -> &'a mut W {
        self.variant(TIMING_FAST_RX_EN_FAST_RX_A::TIMING_FAST_RX_EN_FAST_RX_DEFAULT)
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
#[doc = "Time to switch off the fast CHP in Rx mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMING_FAST_RX_T_RX_FAST_CHP_A {
    #[doc = "0: `0`"]
    TIMING_FAST_RX_T_RX_FAST_CHP_DEFAULT = 0,
}
impl From<TIMING_FAST_RX_T_RX_FAST_CHP_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMING_FAST_RX_T_RX_FAST_CHP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMING_FAST_RX_T_RX_FAST_CHP`"]
pub type TIMING_FAST_RX_T_RX_FAST_CHP_R = crate::R<u8, TIMING_FAST_RX_T_RX_FAST_CHP_A>;
impl TIMING_FAST_RX_T_RX_FAST_CHP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMING_FAST_RX_T_RX_FAST_CHP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIMING_FAST_RX_T_RX_FAST_CHP_A::TIMING_FAST_RX_T_RX_FAST_CHP_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMING_FAST_RX_T_RX_FAST_CHP_DEFAULT`"]
    #[inline(always)]
    pub fn is_timing_fast_rx_t_rx_fast_chp_default(&self) -> bool {
        *self == TIMING_FAST_RX_T_RX_FAST_CHP_A::TIMING_FAST_RX_T_RX_FAST_CHP_DEFAULT
    }
}
#[doc = "Write proxy for field `TIMING_FAST_RX_T_RX_FAST_CHP`"]
pub struct TIMING_FAST_RX_T_RX_FAST_CHP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMING_FAST_RX_T_RX_FAST_CHP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMING_FAST_RX_T_RX_FAST_CHP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn timing_fast_rx_t_rx_fast_chp_default(self) -> &'a mut W {
        self.variant(TIMING_FAST_RX_T_RX_FAST_CHP_A::TIMING_FAST_RX_T_RX_FAST_CHP_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Time needed by the Rx RF blocks to switch on.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMINGS_5_T_RX_RF_A {
    #[doc = "0: `0`"]
    TIMINGS_5_T_RX_RF_DEFAULT = 0,
}
impl From<TIMINGS_5_T_RX_RF_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMINGS_5_T_RX_RF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMINGS_5_T_RX_RF`"]
pub type TIMINGS_5_T_RX_RF_R = crate::R<u8, TIMINGS_5_T_RX_RF_A>;
impl TIMINGS_5_T_RX_RF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMINGS_5_T_RX_RF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIMINGS_5_T_RX_RF_A::TIMINGS_5_T_RX_RF_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMINGS_5_T_RX_RF_DEFAULT`"]
    #[inline(always)]
    pub fn is_timings_5_t_rx_rf_default(&self) -> bool {
        *self == TIMINGS_5_T_RX_RF_A::TIMINGS_5_T_RX_RF_DEFAULT
    }
}
#[doc = "Write proxy for field `TIMINGS_5_T_RX_RF`"]
pub struct TIMINGS_5_T_RX_RF_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMINGS_5_T_RX_RF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMINGS_5_T_RX_RF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn timings_5_t_rx_rf_default(self) -> &'a mut W {
        self.variant(TIMINGS_5_T_RX_RF_A::TIMINGS_5_T_RX_RF_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Time needed by the Rx BB blocks to switch on.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMINGS_5_T_RX_BB_A {
    #[doc = "0: `0`"]
    TIMINGS_5_T_RX_BB_DEFAULT = 0,
}
impl From<TIMINGS_5_T_RX_BB_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMINGS_5_T_RX_BB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMINGS_5_T_RX_BB`"]
pub type TIMINGS_5_T_RX_BB_R = crate::R<u8, TIMINGS_5_T_RX_BB_A>;
impl TIMINGS_5_T_RX_BB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMINGS_5_T_RX_BB_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIMINGS_5_T_RX_BB_A::TIMINGS_5_T_RX_BB_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMINGS_5_T_RX_BB_DEFAULT`"]
    #[inline(always)]
    pub fn is_timings_5_t_rx_bb_default(&self) -> bool {
        *self == TIMINGS_5_T_RX_BB_A::TIMINGS_5_T_RX_BB_DEFAULT
    }
}
#[doc = "Write proxy for field `TIMINGS_5_T_RX_BB`"]
pub struct TIMINGS_5_T_RX_BB_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMINGS_5_T_RX_BB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMINGS_5_T_RX_BB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn timings_5_t_rx_bb_default(self) -> &'a mut W {
        self.variant(TIMINGS_5_T_RX_BB_A::TIMINGS_5_T_RX_BB_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Time needed by the subband algorithm to calibrate in Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMINGS_4_T_SUBBAND_RX_A {
    #[doc = "0: `0`"]
    TIMINGS_4_T_SUBBAND_RX_DEFAULT = 0,
}
impl From<TIMINGS_4_T_SUBBAND_RX_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMINGS_4_T_SUBBAND_RX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMINGS_4_T_SUBBAND_RX`"]
pub type TIMINGS_4_T_SUBBAND_RX_R = crate::R<u8, TIMINGS_4_T_SUBBAND_RX_A>;
impl TIMINGS_4_T_SUBBAND_RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMINGS_4_T_SUBBAND_RX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIMINGS_4_T_SUBBAND_RX_A::TIMINGS_4_T_SUBBAND_RX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMINGS_4_T_SUBBAND_RX_DEFAULT`"]
    #[inline(always)]
    pub fn is_timings_4_t_subband_rx_default(&self) -> bool {
        *self == TIMINGS_4_T_SUBBAND_RX_A::TIMINGS_4_T_SUBBAND_RX_DEFAULT
    }
}
#[doc = "Write proxy for field `TIMINGS_4_T_SUBBAND_RX`"]
pub struct TIMINGS_4_T_SUBBAND_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMINGS_4_T_SUBBAND_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMINGS_4_T_SUBBAND_RX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn timings_4_t_subband_rx_default(self) -> &'a mut W {
        self.variant(TIMINGS_4_T_SUBBAND_RX_A::TIMINGS_4_T_SUBBAND_RX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Time needed by the PLL blocks in Rx mode to switch on.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMINGS_4_T_PLL_RX_A {
    #[doc = "0: `0`"]
    TIMINGS_4_T_PLL_RX_DEFAULT = 0,
}
impl From<TIMINGS_4_T_PLL_RX_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMINGS_4_T_PLL_RX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMINGS_4_T_PLL_RX`"]
pub type TIMINGS_4_T_PLL_RX_R = crate::R<u8, TIMINGS_4_T_PLL_RX_A>;
impl TIMINGS_4_T_PLL_RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMINGS_4_T_PLL_RX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIMINGS_4_T_PLL_RX_A::TIMINGS_4_T_PLL_RX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMINGS_4_T_PLL_RX_DEFAULT`"]
    #[inline(always)]
    pub fn is_timings_4_t_pll_rx_default(&self) -> bool {
        *self == TIMINGS_4_T_PLL_RX_A::TIMINGS_4_T_PLL_RX_DEFAULT
    }
}
#[doc = "Write proxy for field `TIMINGS_4_T_PLL_RX`"]
pub struct TIMINGS_4_T_PLL_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMINGS_4_T_PLL_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMINGS_4_T_PLL_RX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn timings_4_t_pll_rx_default(self) -> &'a mut W {
        self.variant(TIMINGS_4_T_PLL_RX_A::TIMINGS_4_T_PLL_RX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "If set to 1 the attenuation are specified by 1dB steps from 4dB to 11dB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AGC_ATT_2_AGC_ATT_1DB_A {
    #[doc = "0: `0`"]
    AGC_ATT_2_AGC_ATT_1DB_DEFAULT = 0,
}
impl From<AGC_ATT_2_AGC_ATT_1DB_A> for bool {
    #[inline(always)]
    fn from(variant: AGC_ATT_2_AGC_ATT_1DB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AGC_ATT_2_AGC_ATT_1DB`"]
pub type AGC_ATT_2_AGC_ATT_1DB_R = crate::R<bool, AGC_ATT_2_AGC_ATT_1DB_A>;
impl AGC_ATT_2_AGC_ATT_1DB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, AGC_ATT_2_AGC_ATT_1DB_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(AGC_ATT_2_AGC_ATT_1DB_A::AGC_ATT_2_AGC_ATT_1DB_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_ATT_2_AGC_ATT_1DB_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_att_2_agc_att_1db_default(&self) -> bool {
        *self == AGC_ATT_2_AGC_ATT_1DB_A::AGC_ATT_2_AGC_ATT_1DB_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_ATT_2_AGC_ATT_1DB`"]
pub struct AGC_ATT_2_AGC_ATT_1DB_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_ATT_2_AGC_ATT_1DB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_ATT_2_AGC_ATT_1DB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_att_2_agc_att_1db_default(self) -> &'a mut W {
        self.variant(AGC_ATT_2_AGC_ATT_1DB_A::AGC_ATT_2_AGC_ATT_1DB_DEFAULT)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AGC_ATT_2_AGC_ATT_AB_HI_A {
    #[doc = "0: `0`"]
    AGC_ATT_2_AGC_ATT_AB_HI_DEFAULT = 0,
}
impl From<AGC_ATT_2_AGC_ATT_AB_HI_A> for bool {
    #[inline(always)]
    fn from(variant: AGC_ATT_2_AGC_ATT_AB_HI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AGC_ATT_2_AGC_ATT_AB_HI`"]
pub type AGC_ATT_2_AGC_ATT_AB_HI_R = crate::R<bool, AGC_ATT_2_AGC_ATT_AB_HI_A>;
impl AGC_ATT_2_AGC_ATT_AB_HI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, AGC_ATT_2_AGC_ATT_AB_HI_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(AGC_ATT_2_AGC_ATT_AB_HI_A::AGC_ATT_2_AGC_ATT_AB_HI_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_ATT_2_AGC_ATT_AB_HI_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_att_2_agc_att_ab_hi_default(&self) -> bool {
        *self == AGC_ATT_2_AGC_ATT_AB_HI_A::AGC_ATT_2_AGC_ATT_AB_HI_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_ATT_2_AGC_ATT_AB_HI`"]
pub struct AGC_ATT_2_AGC_ATT_AB_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_ATT_2_AGC_ATT_AB_HI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_ATT_2_AGC_ATT_AB_HI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_att_2_agc_att_ab_hi_default(self) -> &'a mut W {
        self.variant(AGC_ATT_2_AGC_ATT_AB_HI_A::AGC_ATT_2_AGC_ATT_AB_HI_DEFAULT)
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
    #[doc = "Bit 29 - If set to 1 enables filter Tx configuration for the fast Rx PLL"]
    #[inline(always)]
    pub fn timing_fast_rx_en_fast_rx_txfilt(&self) -> TIMING_FAST_RX_EN_FAST_RX_TXFILT_R {
        TIMING_FAST_RX_EN_FAST_RX_TXFILT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - If set to 1 enables the fast Rx PLL"]
    #[inline(always)]
    pub fn timing_fast_rx_en_fast_rx(&self) -> TIMING_FAST_RX_EN_FAST_RX_R {
        TIMING_FAST_RX_EN_FAST_RX_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Time to switch off the fast CHP in Rx mode"]
    #[inline(always)]
    pub fn timing_fast_rx_t_rx_fast_chp(&self) -> TIMING_FAST_RX_T_RX_FAST_CHP_R {
        TIMING_FAST_RX_T_RX_FAST_CHP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Time needed by the Rx RF blocks to switch on."]
    #[inline(always)]
    pub fn timings_5_t_rx_rf(&self) -> TIMINGS_5_T_RX_RF_R {
        TIMINGS_5_T_RX_RF_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Time needed by the Rx BB blocks to switch on."]
    #[inline(always)]
    pub fn timings_5_t_rx_bb(&self) -> TIMINGS_5_T_RX_BB_R {
        TIMINGS_5_T_RX_BB_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Time needed by the subband algorithm to calibrate in Rx"]
    #[inline(always)]
    pub fn timings_4_t_subband_rx(&self) -> TIMINGS_4_T_SUBBAND_RX_R {
        TIMINGS_4_T_SUBBAND_RX_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Time needed by the PLL blocks in Rx mode to switch on."]
    #[inline(always)]
    pub fn timings_4_t_pll_rx(&self) -> TIMINGS_4_T_PLL_RX_R {
        TIMINGS_4_T_PLL_RX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - If set to 1 the attenuation are specified by 1dB steps from 4dB to 11dB"]
    #[inline(always)]
    pub fn agc_att_2_agc_att_1db(&self) -> AGC_ATT_2_AGC_ATT_1DB_R {
        AGC_ATT_2_AGC_ATT_1DB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn agc_att_2_agc_att_ab_hi(&self) -> AGC_ATT_2_AGC_ATT_AB_HI_R {
        AGC_ATT_2_AGC_ATT_AB_HI_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - If set to 1 enables filter Tx configuration for the fast Rx PLL"]
    #[inline(always)]
    pub fn timing_fast_rx_en_fast_rx_txfilt(&mut self) -> TIMING_FAST_RX_EN_FAST_RX_TXFILT_W {
        TIMING_FAST_RX_EN_FAST_RX_TXFILT_W { w: self }
    }
    #[doc = "Bit 28 - If set to 1 enables the fast Rx PLL"]
    #[inline(always)]
    pub fn timing_fast_rx_en_fast_rx(&mut self) -> TIMING_FAST_RX_EN_FAST_RX_W {
        TIMING_FAST_RX_EN_FAST_RX_W { w: self }
    }
    #[doc = "Bits 24:27 - Time to switch off the fast CHP in Rx mode"]
    #[inline(always)]
    pub fn timing_fast_rx_t_rx_fast_chp(&mut self) -> TIMING_FAST_RX_T_RX_FAST_CHP_W {
        TIMING_FAST_RX_T_RX_FAST_CHP_W { w: self }
    }
    #[doc = "Bits 20:23 - Time needed by the Rx RF blocks to switch on."]
    #[inline(always)]
    pub fn timings_5_t_rx_rf(&mut self) -> TIMINGS_5_T_RX_RF_W {
        TIMINGS_5_T_RX_RF_W { w: self }
    }
    #[doc = "Bits 16:19 - Time needed by the Rx BB blocks to switch on."]
    #[inline(always)]
    pub fn timings_5_t_rx_bb(&mut self) -> TIMINGS_5_T_RX_BB_W {
        TIMINGS_5_T_RX_BB_W { w: self }
    }
    #[doc = "Bits 12:15 - Time needed by the subband algorithm to calibrate in Rx"]
    #[inline(always)]
    pub fn timings_4_t_subband_rx(&mut self) -> TIMINGS_4_T_SUBBAND_RX_W {
        TIMINGS_4_T_SUBBAND_RX_W { w: self }
    }
    #[doc = "Bits 8:11 - Time needed by the PLL blocks in Rx mode to switch on."]
    #[inline(always)]
    pub fn timings_4_t_pll_rx(&mut self) -> TIMINGS_4_T_PLL_RX_W {
        TIMINGS_4_T_PLL_RX_W { w: self }
    }
    #[doc = "Bit 1 - If set to 1 the attenuation are specified by 1dB steps from 4dB to 11dB"]
    #[inline(always)]
    pub fn agc_att_2_agc_att_1db(&mut self) -> AGC_ATT_2_AGC_ATT_1DB_W {
        AGC_ATT_2_AGC_ATT_1DB_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn agc_att_2_agc_att_ab_hi(&mut self) -> AGC_ATT_2_AGC_ATT_AB_HI_W {
        AGC_ATT_2_AGC_ATT_AB_HI_W { w: self }
    }
}
