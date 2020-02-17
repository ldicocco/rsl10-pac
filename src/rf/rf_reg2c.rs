#[doc = "Reader of register RF_REG2C"]
pub type R = crate::R<u32, super::RF_REG2C>;
#[doc = "Writer for register RF_REG2C"]
pub type W = crate::W<u32, super::RF_REG2C>;
#[doc = "Register RF_REG2C `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG2C {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Offset to add in frequency count in order to compensate the offset of the varicap.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBBAND_OFFSET_SB_OFFSET_A {
    #[doc = "0: `0`"]
    SUBBAND_OFFSET_SB_OFFSET_DEFAULT = 0,
}
impl From<SUBBAND_OFFSET_SB_OFFSET_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBBAND_OFFSET_SB_OFFSET_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SUBBAND_OFFSET_SB_OFFSET`"]
pub type SUBBAND_OFFSET_SB_OFFSET_R = crate::R<u8, SUBBAND_OFFSET_SB_OFFSET_A>;
impl SUBBAND_OFFSET_SB_OFFSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUBBAND_OFFSET_SB_OFFSET_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SUBBAND_OFFSET_SB_OFFSET_A::SUBBAND_OFFSET_SB_OFFSET_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUBBAND_OFFSET_SB_OFFSET_DEFAULT`"]
    #[inline(always)]
    pub fn is_subband_offset_sb_offset_default(&self) -> bool {
        *self == SUBBAND_OFFSET_SB_OFFSET_A::SUBBAND_OFFSET_SB_OFFSET_DEFAULT
    }
}
#[doc = "Write proxy for field `SUBBAND_OFFSET_SB_OFFSET`"]
pub struct SUBBAND_OFFSET_SB_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBBAND_OFFSET_SB_OFFSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUBBAND_OFFSET_SB_OFFSET_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn subband_offset_sb_offset_default(self) -> &'a mut W {
        self.variant(SUBBAND_OFFSET_SB_OFFSET_A::SUBBAND_OFFSET_SB_OFFSET_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "maximum subband value in linear search subband (freq and comp)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWCAP_LIM_SB_MAX_VAL_A {
    #[doc = "0: `0`"]
    SWCAP_LIM_SB_MAX_VAL_DEFAULT = 0,
}
impl From<SWCAP_LIM_SB_MAX_VAL_A> for u8 {
    #[inline(always)]
    fn from(variant: SWCAP_LIM_SB_MAX_VAL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SWCAP_LIM_SB_MAX_VAL`"]
pub type SWCAP_LIM_SB_MAX_VAL_R = crate::R<u8, SWCAP_LIM_SB_MAX_VAL_A>;
impl SWCAP_LIM_SB_MAX_VAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SWCAP_LIM_SB_MAX_VAL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SWCAP_LIM_SB_MAX_VAL_A::SWCAP_LIM_SB_MAX_VAL_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWCAP_LIM_SB_MAX_VAL_DEFAULT`"]
    #[inline(always)]
    pub fn is_swcap_lim_sb_max_val_default(&self) -> bool {
        *self == SWCAP_LIM_SB_MAX_VAL_A::SWCAP_LIM_SB_MAX_VAL_DEFAULT
    }
}
#[doc = "Write proxy for field `SWCAP_LIM_SB_MAX_VAL`"]
pub struct SWCAP_LIM_SB_MAX_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWCAP_LIM_SB_MAX_VAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWCAP_LIM_SB_MAX_VAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn swcap_lim_sb_max_val_default(self) -> &'a mut W {
        self.variant(SWCAP_LIM_SB_MAX_VAL_A::SWCAP_LIM_SB_MAX_VAL_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "minimum subband value in linear search subband (freq and comp)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWCAP_LIM_SB_MIN_VAL_A {
    #[doc = "0: `0`"]
    SWCAP_LIM_SB_MIN_VAL_DEFAULT = 0,
}
impl From<SWCAP_LIM_SB_MIN_VAL_A> for u8 {
    #[inline(always)]
    fn from(variant: SWCAP_LIM_SB_MIN_VAL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SWCAP_LIM_SB_MIN_VAL`"]
pub type SWCAP_LIM_SB_MIN_VAL_R = crate::R<u8, SWCAP_LIM_SB_MIN_VAL_A>;
impl SWCAP_LIM_SB_MIN_VAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SWCAP_LIM_SB_MIN_VAL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SWCAP_LIM_SB_MIN_VAL_A::SWCAP_LIM_SB_MIN_VAL_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWCAP_LIM_SB_MIN_VAL_DEFAULT`"]
    #[inline(always)]
    pub fn is_swcap_lim_sb_min_val_default(&self) -> bool {
        *self == SWCAP_LIM_SB_MIN_VAL_A::SWCAP_LIM_SB_MIN_VAL_DEFAULT
    }
}
#[doc = "Write proxy for field `SWCAP_LIM_SB_MIN_VAL`"]
pub struct SWCAP_LIM_SB_MIN_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWCAP_LIM_SB_MIN_VAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWCAP_LIM_SB_MIN_VAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn swcap_lim_sb_min_val_default(self) -> &'a mut W {
        self.variant(SWCAP_LIM_SB_MIN_VAL_A::SWCAP_LIM_SB_MIN_VAL_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Enables the FLL mode for the subband selection (overrides other settings)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUBBAND_CONF_SB_FLL_MODE_A {
    #[doc = "0: `0`"]
    SUBBAND_CONF_SB_FLL_MODE_DEFAULT = 0,
}
impl From<SUBBAND_CONF_SB_FLL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: SUBBAND_CONF_SB_FLL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SUBBAND_CONF_SB_FLL_MODE`"]
pub type SUBBAND_CONF_SB_FLL_MODE_R = crate::R<bool, SUBBAND_CONF_SB_FLL_MODE_A>;
impl SUBBAND_CONF_SB_FLL_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SUBBAND_CONF_SB_FLL_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(SUBBAND_CONF_SB_FLL_MODE_A::SUBBAND_CONF_SB_FLL_MODE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUBBAND_CONF_SB_FLL_MODE_DEFAULT`"]
    #[inline(always)]
    pub fn is_subband_conf_sb_fll_mode_default(&self) -> bool {
        *self == SUBBAND_CONF_SB_FLL_MODE_A::SUBBAND_CONF_SB_FLL_MODE_DEFAULT
    }
}
#[doc = "Write proxy for field `SUBBAND_CONF_SB_FLL_MODE`"]
pub struct SUBBAND_CONF_SB_FLL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBBAND_CONF_SB_FLL_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUBBAND_CONF_SB_FLL_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn subband_conf_sb_fll_mode_default(self) -> &'a mut W {
        self.variant(SUBBAND_CONF_SB_FLL_MODE_A::SUBBAND_CONF_SB_FLL_MODE_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "invert the meaning of sb_high and sb_low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUBBAND_CONF_SB_INV_BAND_A {
    #[doc = "0: `0`"]
    SUBBAND_CONF_SB_INV_BAND_DEFAULT = 0,
}
impl From<SUBBAND_CONF_SB_INV_BAND_A> for bool {
    #[inline(always)]
    fn from(variant: SUBBAND_CONF_SB_INV_BAND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SUBBAND_CONF_SB_INV_BAND`"]
pub type SUBBAND_CONF_SB_INV_BAND_R = crate::R<bool, SUBBAND_CONF_SB_INV_BAND_A>;
impl SUBBAND_CONF_SB_INV_BAND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SUBBAND_CONF_SB_INV_BAND_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(SUBBAND_CONF_SB_INV_BAND_A::SUBBAND_CONF_SB_INV_BAND_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUBBAND_CONF_SB_INV_BAND_DEFAULT`"]
    #[inline(always)]
    pub fn is_subband_conf_sb_inv_band_default(&self) -> bool {
        *self == SUBBAND_CONF_SB_INV_BAND_A::SUBBAND_CONF_SB_INV_BAND_DEFAULT
    }
}
#[doc = "Write proxy for field `SUBBAND_CONF_SB_INV_BAND`"]
pub struct SUBBAND_CONF_SB_INV_BAND_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBBAND_CONF_SB_INV_BAND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUBBAND_CONF_SB_INV_BAND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn subband_conf_sb_inv_band_default(self) -> &'a mut W {
        self.variant(SUBBAND_CONF_SB_INV_BAND_A::SUBBAND_CONF_SB_INV_BAND_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "The length to count in frequency mode: 00 => 256 (Rx: 10.7us, Tx: 2.13us),01 => 512 (Rx: 21.3us, Tx: 4.26us),11 => 1024 (Rx: 42.7us, Tx: 8.53us),01 => 4096 (Rx: 171us, Tx: 34.1us)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBBAND_CONF_SB_FREQ_CNT_A {
    #[doc = "0: `0`"]
    SUBBAND_CONF_SB_FREQ_CNT_DEFAULT = 0,
}
impl From<SUBBAND_CONF_SB_FREQ_CNT_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBBAND_CONF_SB_FREQ_CNT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SUBBAND_CONF_SB_FREQ_CNT`"]
pub type SUBBAND_CONF_SB_FREQ_CNT_R = crate::R<u8, SUBBAND_CONF_SB_FREQ_CNT_A>;
impl SUBBAND_CONF_SB_FREQ_CNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUBBAND_CONF_SB_FREQ_CNT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SUBBAND_CONF_SB_FREQ_CNT_A::SUBBAND_CONF_SB_FREQ_CNT_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUBBAND_CONF_SB_FREQ_CNT_DEFAULT`"]
    #[inline(always)]
    pub fn is_subband_conf_sb_freq_cnt_default(&self) -> bool {
        *self == SUBBAND_CONF_SB_FREQ_CNT_A::SUBBAND_CONF_SB_FREQ_CNT_DEFAULT
    }
}
#[doc = "Write proxy for field `SUBBAND_CONF_SB_FREQ_CNT`"]
pub struct SUBBAND_CONF_SB_FREQ_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBBAND_CONF_SB_FREQ_CNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUBBAND_CONF_SB_FREQ_CNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn subband_conf_sb_freq_cnt_default(self) -> &'a mut W {
        self.variant(SUBBAND_CONF_SB_FREQ_CNT_A::SUBBAND_CONF_SB_FREQ_CNT_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "time to wait to the PLL to settle: 00 => Rx 8us, Tx 2us, 01 => Rx 12us, Tx 3us, 10 => Rx 16us, Tx 4us, 11 => Rx 24us, Tx 6u\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBBAND_CONF_SB_WAIT_T_A {
    #[doc = "0: `0`"]
    SUBBAND_CONF_SB_WAIT_T_DEFAULT = 0,
}
impl From<SUBBAND_CONF_SB_WAIT_T_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBBAND_CONF_SB_WAIT_T_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SUBBAND_CONF_SB_WAIT_T`"]
pub type SUBBAND_CONF_SB_WAIT_T_R = crate::R<u8, SUBBAND_CONF_SB_WAIT_T_A>;
impl SUBBAND_CONF_SB_WAIT_T_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUBBAND_CONF_SB_WAIT_T_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SUBBAND_CONF_SB_WAIT_T_A::SUBBAND_CONF_SB_WAIT_T_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUBBAND_CONF_SB_WAIT_T_DEFAULT`"]
    #[inline(always)]
    pub fn is_subband_conf_sb_wait_t_default(&self) -> bool {
        *self == SUBBAND_CONF_SB_WAIT_T_A::SUBBAND_CONF_SB_WAIT_T_DEFAULT
    }
}
#[doc = "Write proxy for field `SUBBAND_CONF_SB_WAIT_T`"]
pub struct SUBBAND_CONF_SB_WAIT_T_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBBAND_CONF_SB_WAIT_T_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUBBAND_CONF_SB_WAIT_T_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn subband_conf_sb_wait_t_default(self) -> &'a mut W {
        self.variant(SUBBAND_CONF_SB_WAIT_T_A::SUBBAND_CONF_SB_WAIT_T_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "sub-band algorithm mode: 00 => SAR w/ comparators, 01 => linear w/ comparators, 00 => SAR w/ frequency ratios, 01 => linear w/ frequency ratios\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBBAND_CONF_SB_MODE_A {
    #[doc = "0: `0`"]
    SUBBAND_CONF_SB_MODE_DEFAULT = 0,
}
impl From<SUBBAND_CONF_SB_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBBAND_CONF_SB_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SUBBAND_CONF_SB_MODE`"]
pub type SUBBAND_CONF_SB_MODE_R = crate::R<u8, SUBBAND_CONF_SB_MODE_A>;
impl SUBBAND_CONF_SB_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUBBAND_CONF_SB_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SUBBAND_CONF_SB_MODE_A::SUBBAND_CONF_SB_MODE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUBBAND_CONF_SB_MODE_DEFAULT`"]
    #[inline(always)]
    pub fn is_subband_conf_sb_mode_default(&self) -> bool {
        *self == SUBBAND_CONF_SB_MODE_A::SUBBAND_CONF_SB_MODE_DEFAULT
    }
}
#[doc = "Write proxy for field `SUBBAND_CONF_SB_MODE`"]
pub struct SUBBAND_CONF_SB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBBAND_CONF_SB_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUBBAND_CONF_SB_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn subband_conf_sb_mode_default(self) -> &'a mut W {
        self.variant(SUBBAND_CONF_SB_MODE_A::SUBBAND_CONF_SB_MODE_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Harmonic 2 notch tuning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PA_CONF_SW_CN_A {
    #[doc = "0: `0`"]
    PA_CONF_SW_CN_DEFAULT = 0,
}
impl From<PA_CONF_SW_CN_A> for u8 {
    #[inline(always)]
    fn from(variant: PA_CONF_SW_CN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PA_CONF_SW_CN`"]
pub type PA_CONF_SW_CN_R = crate::R<u8, PA_CONF_SW_CN_A>;
impl PA_CONF_SW_CN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PA_CONF_SW_CN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PA_CONF_SW_CN_A::PA_CONF_SW_CN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA_CONF_SW_CN_DEFAULT`"]
    #[inline(always)]
    pub fn is_pa_conf_sw_cn_default(&self) -> bool {
        *self == PA_CONF_SW_CN_A::PA_CONF_SW_CN_DEFAULT
    }
}
#[doc = "Write proxy for field `PA_CONF_SW_CN`"]
pub struct PA_CONF_SW_CN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_CONF_SW_CN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PA_CONF_SW_CN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pa_conf_sw_cn_default(self) -> &'a mut W {
        self.variant(PA_CONF_SW_CN_A::PA_CONF_SW_CN_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "If set to 1, enables the PA only with the digital block, otherwise it's the RF Tx timing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA_CONF_TX_SWITCHPA_A {
    #[doc = "0: `0`"]
    PA_CONF_TX_SWITCHPA_DEFAULT = 0,
}
impl From<PA_CONF_TX_SWITCHPA_A> for bool {
    #[inline(always)]
    fn from(variant: PA_CONF_TX_SWITCHPA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PA_CONF_TX_SWITCHPA`"]
pub type PA_CONF_TX_SWITCHPA_R = crate::R<bool, PA_CONF_TX_SWITCHPA_A>;
impl PA_CONF_TX_SWITCHPA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PA_CONF_TX_SWITCHPA_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PA_CONF_TX_SWITCHPA_A::PA_CONF_TX_SWITCHPA_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA_CONF_TX_SWITCHPA_DEFAULT`"]
    #[inline(always)]
    pub fn is_pa_conf_tx_switchpa_default(&self) -> bool {
        *self == PA_CONF_TX_SWITCHPA_A::PA_CONF_TX_SWITCHPA_DEFAULT
    }
}
#[doc = "Write proxy for field `PA_CONF_TX_SWITCHPA`"]
pub struct PA_CONF_TX_SWITCHPA_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_CONF_TX_SWITCHPA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PA_CONF_TX_SWITCHPA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pa_conf_tx_switchpa_default(self) -> &'a mut W {
        self.variant(PA_CONF_TX_SWITCHPA_A::PA_CONF_TX_SWITCHPA_DEFAULT)
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
#[doc = "If set to 1 enables the PA, otherwise only the PPA is used (-20dBm)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA_CONF_TX_0DBM_A {
    #[doc = "0: `0`"]
    PA_CONF_TX_0DBM_DEFAULT = 0,
}
impl From<PA_CONF_TX_0DBM_A> for bool {
    #[inline(always)]
    fn from(variant: PA_CONF_TX_0DBM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PA_CONF_TX_0DBM`"]
pub type PA_CONF_TX_0DBM_R = crate::R<bool, PA_CONF_TX_0DBM_A>;
impl PA_CONF_TX_0DBM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PA_CONF_TX_0DBM_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PA_CONF_TX_0DBM_A::PA_CONF_TX_0DBM_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA_CONF_TX_0DBM_DEFAULT`"]
    #[inline(always)]
    pub fn is_pa_conf_tx_0dbm_default(&self) -> bool {
        *self == PA_CONF_TX_0DBM_A::PA_CONF_TX_0DBM_DEFAULT
    }
}
#[doc = "Write proxy for field `PA_CONF_TX_0DBM`"]
pub struct PA_CONF_TX_0DBM_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_CONF_TX_0DBM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PA_CONF_TX_0DBM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pa_conf_tx_0dbm_default(self) -> &'a mut W {
        self.variant(PA_CONF_TX_0DBM_A::PA_CONF_TX_0DBM_DEFAULT)
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
#[doc = "N.U.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PA_CONF_CTRL_PA_A {
    #[doc = "0: `0`"]
    PA_CONF_CTRL_PA_DEFAULT = 0,
}
impl From<PA_CONF_CTRL_PA_A> for u8 {
    #[inline(always)]
    fn from(variant: PA_CONF_CTRL_PA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PA_CONF_CTRL_PA`"]
pub type PA_CONF_CTRL_PA_R = crate::R<u8, PA_CONF_CTRL_PA_A>;
impl PA_CONF_CTRL_PA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PA_CONF_CTRL_PA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PA_CONF_CTRL_PA_A::PA_CONF_CTRL_PA_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA_CONF_CTRL_PA_DEFAULT`"]
    #[inline(always)]
    pub fn is_pa_conf_ctrl_pa_default(&self) -> bool {
        *self == PA_CONF_CTRL_PA_A::PA_CONF_CTRL_PA_DEFAULT
    }
}
#[doc = "Write proxy for field `PA_CONF_CTRL_PA`"]
pub struct PA_CONF_CTRL_PA_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_CONF_CTRL_PA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PA_CONF_CTRL_PA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pa_conf_ctrl_pa_default(self) -> &'a mut W {
        self.variant(PA_CONF_CTRL_PA_A::PA_CONF_CTRL_PA_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Offset to add in frequency count in order to compensate the offset of the varicap."]
    #[inline(always)]
    pub fn subband_offset_sb_offset(&self) -> SUBBAND_OFFSET_SB_OFFSET_R {
        SUBBAND_OFFSET_SB_OFFSET_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - maximum subband value in linear search subband (freq and comp)"]
    #[inline(always)]
    pub fn swcap_lim_sb_max_val(&self) -> SWCAP_LIM_SB_MAX_VAL_R {
        SWCAP_LIM_SB_MAX_VAL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - minimum subband value in linear search subband (freq and comp)"]
    #[inline(always)]
    pub fn swcap_lim_sb_min_val(&self) -> SWCAP_LIM_SB_MIN_VAL_R {
        SWCAP_LIM_SB_MIN_VAL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Enables the FLL mode for the subband selection (overrides other settings)"]
    #[inline(always)]
    pub fn subband_conf_sb_fll_mode(&self) -> SUBBAND_CONF_SB_FLL_MODE_R {
        SUBBAND_CONF_SB_FLL_MODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - invert the meaning of sb_high and sb_low"]
    #[inline(always)]
    pub fn subband_conf_sb_inv_band(&self) -> SUBBAND_CONF_SB_INV_BAND_R {
        SUBBAND_CONF_SB_INV_BAND_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - The length to count in frequency mode: 00 => 256 (Rx: 10.7us, Tx: 2.13us),01 => 512 (Rx: 21.3us, Tx: 4.26us),11 => 1024 (Rx: 42.7us, Tx: 8.53us),01 => 4096 (Rx: 171us, Tx: 34.1us)"]
    #[inline(always)]
    pub fn subband_conf_sb_freq_cnt(&self) -> SUBBAND_CONF_SB_FREQ_CNT_R {
        SUBBAND_CONF_SB_FREQ_CNT_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - time to wait to the PLL to settle: 00 => Rx 8us, Tx 2us, 01 => Rx 12us, Tx 3us, 10 => Rx 16us, Tx 4us, 11 => Rx 24us, Tx 6u"]
    #[inline(always)]
    pub fn subband_conf_sb_wait_t(&self) -> SUBBAND_CONF_SB_WAIT_T_R {
        SUBBAND_CONF_SB_WAIT_T_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - sub-band algorithm mode: 00 => SAR w/ comparators, 01 => linear w/ comparators, 00 => SAR w/ frequency ratios, 01 => linear w/ frequency ratios"]
    #[inline(always)]
    pub fn subband_conf_sb_mode(&self) -> SUBBAND_CONF_SB_MODE_R {
        SUBBAND_CONF_SB_MODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Harmonic 2 notch tuning"]
    #[inline(always)]
    pub fn pa_conf_sw_cn(&self) -> PA_CONF_SW_CN_R {
        PA_CONF_SW_CN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3 - If set to 1, enables the PA only with the digital block, otherwise it's the RF Tx timing"]
    #[inline(always)]
    pub fn pa_conf_tx_switchpa(&self) -> PA_CONF_TX_SWITCHPA_R {
        PA_CONF_TX_SWITCHPA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If set to 1 enables the PA, otherwise only the PPA is used (-20dBm)"]
    #[inline(always)]
    pub fn pa_conf_tx_0dbm(&self) -> PA_CONF_TX_0DBM_R {
        PA_CONF_TX_0DBM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - N.U."]
    #[inline(always)]
    pub fn pa_conf_ctrl_pa(&self) -> PA_CONF_CTRL_PA_R {
        PA_CONF_CTRL_PA_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Offset to add in frequency count in order to compensate the offset of the varicap."]
    #[inline(always)]
    pub fn subband_offset_sb_offset(&mut self) -> SUBBAND_OFFSET_SB_OFFSET_W {
        SUBBAND_OFFSET_SB_OFFSET_W { w: self }
    }
    #[doc = "Bits 20:23 - maximum subband value in linear search subband (freq and comp)"]
    #[inline(always)]
    pub fn swcap_lim_sb_max_val(&mut self) -> SWCAP_LIM_SB_MAX_VAL_W {
        SWCAP_LIM_SB_MAX_VAL_W { w: self }
    }
    #[doc = "Bits 16:19 - minimum subband value in linear search subband (freq and comp)"]
    #[inline(always)]
    pub fn swcap_lim_sb_min_val(&mut self) -> SWCAP_LIM_SB_MIN_VAL_W {
        SWCAP_LIM_SB_MIN_VAL_W { w: self }
    }
    #[doc = "Bit 15 - Enables the FLL mode for the subband selection (overrides other settings)"]
    #[inline(always)]
    pub fn subband_conf_sb_fll_mode(&mut self) -> SUBBAND_CONF_SB_FLL_MODE_W {
        SUBBAND_CONF_SB_FLL_MODE_W { w: self }
    }
    #[doc = "Bit 14 - invert the meaning of sb_high and sb_low"]
    #[inline(always)]
    pub fn subband_conf_sb_inv_band(&mut self) -> SUBBAND_CONF_SB_INV_BAND_W {
        SUBBAND_CONF_SB_INV_BAND_W { w: self }
    }
    #[doc = "Bits 12:13 - The length to count in frequency mode: 00 => 256 (Rx: 10.7us, Tx: 2.13us),01 => 512 (Rx: 21.3us, Tx: 4.26us),11 => 1024 (Rx: 42.7us, Tx: 8.53us),01 => 4096 (Rx: 171us, Tx: 34.1us)"]
    #[inline(always)]
    pub fn subband_conf_sb_freq_cnt(&mut self) -> SUBBAND_CONF_SB_FREQ_CNT_W {
        SUBBAND_CONF_SB_FREQ_CNT_W { w: self }
    }
    #[doc = "Bits 10:11 - time to wait to the PLL to settle: 00 => Rx 8us, Tx 2us, 01 => Rx 12us, Tx 3us, 10 => Rx 16us, Tx 4us, 11 => Rx 24us, Tx 6u"]
    #[inline(always)]
    pub fn subband_conf_sb_wait_t(&mut self) -> SUBBAND_CONF_SB_WAIT_T_W {
        SUBBAND_CONF_SB_WAIT_T_W { w: self }
    }
    #[doc = "Bits 8:9 - sub-band algorithm mode: 00 => SAR w/ comparators, 01 => linear w/ comparators, 00 => SAR w/ frequency ratios, 01 => linear w/ frequency ratios"]
    #[inline(always)]
    pub fn subband_conf_sb_mode(&mut self) -> SUBBAND_CONF_SB_MODE_W {
        SUBBAND_CONF_SB_MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - Harmonic 2 notch tuning"]
    #[inline(always)]
    pub fn pa_conf_sw_cn(&mut self) -> PA_CONF_SW_CN_W {
        PA_CONF_SW_CN_W { w: self }
    }
    #[doc = "Bit 3 - If set to 1, enables the PA only with the digital block, otherwise it's the RF Tx timing"]
    #[inline(always)]
    pub fn pa_conf_tx_switchpa(&mut self) -> PA_CONF_TX_SWITCHPA_W {
        PA_CONF_TX_SWITCHPA_W { w: self }
    }
    #[doc = "Bit 2 - If set to 1 enables the PA, otherwise only the PPA is used (-20dBm)"]
    #[inline(always)]
    pub fn pa_conf_tx_0dbm(&mut self) -> PA_CONF_TX_0DBM_W {
        PA_CONF_TX_0DBM_W { w: self }
    }
    #[doc = "Bits 0:1 - N.U."]
    #[inline(always)]
    pub fn pa_conf_ctrl_pa(&mut self) -> PA_CONF_CTRL_PA_W {
        PA_CONF_CTRL_PA_W { w: self }
    }
}
