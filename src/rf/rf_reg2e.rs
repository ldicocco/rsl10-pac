#[doc = "Reader of register RF_REG2E"]
pub type R = crate::R<u32, super::RF_REG2E>;
#[doc = "Writer for register RF_REG2E"]
pub type W = crate::W<u32, super::RF_REG2E>;
#[doc = "Register RF_REG2E `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG2E {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Threshold used for absolute RSSI detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_A {
    #[doc = "0: `0`"]
    RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_DEFAULT = 0,
}
impl From<RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_A> for u8 {
    #[inline(always)]
    fn from(variant: RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR`"]
pub type RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_R =
    crate::R<u8, RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_A>;
impl RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_A> {
        use crate::Variant::*;
        match self . bits { 0 => Val ( RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_A :: RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_DEFAULT ) , i => Res ( i ) }
    }
    #[doc = "Checks if the value of the field is `RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_detect_abs_thr_rssi_det_abs_thr_default(&self) -> bool {
        *self
            == RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_A::RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR`"]
pub struct RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssi_detect_abs_thr_rssi_det_abs_thr_default(self) -> &'a mut W {
        self.variant(
            RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_A::RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_DEFAULT,
        )
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Threshold used for differential RSSI detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_A {
    #[doc = "0: `0`"]
    RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_DEFAULT = 0,
}
impl From<RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_A> for u8 {
    #[inline(always)]
    fn from(variant: RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR`"]
pub type RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_R =
    crate::R<u8, RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_A>;
impl RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_A> {
        use crate::Variant::*;
        match self . bits { 0 => Val ( RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_A :: RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_DEFAULT ) , i => Res ( i ) }
    }
    #[doc = "Checks if the value of the field is `RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_detect_diff_thr_rssi_det_diff_thr_default(&self) -> bool {
        * self == RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_A :: RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR`"]
pub struct RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssi_detect_diff_thr_rssi_det_diff_thr_default(self) -> &'a mut W {
        self . variant ( RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_A :: RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_DEFAULT )
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "If set to 1 enable the sync word detection in the delay line. This implies that nc_sel_out = 0x7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEMOD_CTRL_EN_DELLINE_SYNC_DET_A {
    #[doc = "0: `0`"]
    DEMOD_CTRL_EN_DELLINE_SYNC_DET_DEFAULT = 0,
}
impl From<DEMOD_CTRL_EN_DELLINE_SYNC_DET_A> for bool {
    #[inline(always)]
    fn from(variant: DEMOD_CTRL_EN_DELLINE_SYNC_DET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEMOD_CTRL_EN_DELLINE_SYNC_DET`"]
pub type DEMOD_CTRL_EN_DELLINE_SYNC_DET_R = crate::R<bool, DEMOD_CTRL_EN_DELLINE_SYNC_DET_A>;
impl DEMOD_CTRL_EN_DELLINE_SYNC_DET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DEMOD_CTRL_EN_DELLINE_SYNC_DET_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DEMOD_CTRL_EN_DELLINE_SYNC_DET_A::DEMOD_CTRL_EN_DELLINE_SYNC_DET_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEMOD_CTRL_EN_DELLINE_SYNC_DET_DEFAULT`"]
    #[inline(always)]
    pub fn is_demod_ctrl_en_delline_sync_det_default(&self) -> bool {
        *self == DEMOD_CTRL_EN_DELLINE_SYNC_DET_A::DEMOD_CTRL_EN_DELLINE_SYNC_DET_DEFAULT
    }
}
#[doc = "Write proxy for field `DEMOD_CTRL_EN_DELLINE_SYNC_DET`"]
pub struct DEMOD_CTRL_EN_DELLINE_SYNC_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> DEMOD_CTRL_EN_DELLINE_SYNC_DET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEMOD_CTRL_EN_DELLINE_SYNC_DET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn demod_ctrl_en_delline_sync_det_default(self) -> &'a mut W {
        self.variant(DEMOD_CTRL_EN_DELLINE_SYNC_DET_A::DEMOD_CTRL_EN_DELLINE_SYNC_DET_DEFAULT)
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
#[doc = "Add an additional filtering on the RSSI value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEMOD_CTRL_RSSI_DET_FILT_A {
    #[doc = "0: `0`"]
    DEMOD_CTRL_RSSI_DET_FILT_DEFAULT = 0,
}
impl From<DEMOD_CTRL_RSSI_DET_FILT_A> for bool {
    #[inline(always)]
    fn from(variant: DEMOD_CTRL_RSSI_DET_FILT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEMOD_CTRL_RSSI_DET_FILT`"]
pub type DEMOD_CTRL_RSSI_DET_FILT_R = crate::R<bool, DEMOD_CTRL_RSSI_DET_FILT_A>;
impl DEMOD_CTRL_RSSI_DET_FILT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DEMOD_CTRL_RSSI_DET_FILT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DEMOD_CTRL_RSSI_DET_FILT_A::DEMOD_CTRL_RSSI_DET_FILT_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEMOD_CTRL_RSSI_DET_FILT_DEFAULT`"]
    #[inline(always)]
    pub fn is_demod_ctrl_rssi_det_filt_default(&self) -> bool {
        *self == DEMOD_CTRL_RSSI_DET_FILT_A::DEMOD_CTRL_RSSI_DET_FILT_DEFAULT
    }
}
#[doc = "Write proxy for field `DEMOD_CTRL_RSSI_DET_FILT`"]
pub struct DEMOD_CTRL_RSSI_DET_FILT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEMOD_CTRL_RSSI_DET_FILT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEMOD_CTRL_RSSI_DET_FILT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn demod_ctrl_rssi_det_filt_default(self) -> &'a mut W {
        self.variant(DEMOD_CTRL_RSSI_DET_FILT_A::DEMOD_CTRL_RSSI_DET_FILT_DEFAULT)
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
#[doc = "If set to 1 speed up the clock recovery during the resto of the preamble\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEMOD_CTRL_EN_FAST_CLK_RECOV_A {
    #[doc = "0: `0`"]
    DEMOD_CTRL_EN_FAST_CLK_RECOV_DEFAULT = 0,
}
impl From<DEMOD_CTRL_EN_FAST_CLK_RECOV_A> for bool {
    #[inline(always)]
    fn from(variant: DEMOD_CTRL_EN_FAST_CLK_RECOV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEMOD_CTRL_EN_FAST_CLK_RECOV`"]
pub type DEMOD_CTRL_EN_FAST_CLK_RECOV_R = crate::R<bool, DEMOD_CTRL_EN_FAST_CLK_RECOV_A>;
impl DEMOD_CTRL_EN_FAST_CLK_RECOV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DEMOD_CTRL_EN_FAST_CLK_RECOV_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DEMOD_CTRL_EN_FAST_CLK_RECOV_A::DEMOD_CTRL_EN_FAST_CLK_RECOV_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEMOD_CTRL_EN_FAST_CLK_RECOV_DEFAULT`"]
    #[inline(always)]
    pub fn is_demod_ctrl_en_fast_clk_recov_default(&self) -> bool {
        *self == DEMOD_CTRL_EN_FAST_CLK_RECOV_A::DEMOD_CTRL_EN_FAST_CLK_RECOV_DEFAULT
    }
}
#[doc = "Write proxy for field `DEMOD_CTRL_EN_FAST_CLK_RECOV`"]
pub struct DEMOD_CTRL_EN_FAST_CLK_RECOV_W<'a> {
    w: &'a mut W,
}
impl<'a> DEMOD_CTRL_EN_FAST_CLK_RECOV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEMOD_CTRL_EN_FAST_CLK_RECOV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn demod_ctrl_en_fast_clk_recov_default(self) -> &'a mut W {
        self.variant(DEMOD_CTRL_EN_FAST_CLK_RECOV_A::DEMOD_CTRL_EN_FAST_CLK_RECOV_DEFAULT)
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
#[doc = "If set to 1 enables the min max algo after the matched filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEMOD_CTRL_EN_MIN_MAX_MF_A {
    #[doc = "0: `0`"]
    DEMOD_CTRL_EN_MIN_MAX_MF_DEFAULT = 0,
}
impl From<DEMOD_CTRL_EN_MIN_MAX_MF_A> for bool {
    #[inline(always)]
    fn from(variant: DEMOD_CTRL_EN_MIN_MAX_MF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEMOD_CTRL_EN_MIN_MAX_MF`"]
pub type DEMOD_CTRL_EN_MIN_MAX_MF_R = crate::R<bool, DEMOD_CTRL_EN_MIN_MAX_MF_A>;
impl DEMOD_CTRL_EN_MIN_MAX_MF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DEMOD_CTRL_EN_MIN_MAX_MF_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DEMOD_CTRL_EN_MIN_MAX_MF_A::DEMOD_CTRL_EN_MIN_MAX_MF_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEMOD_CTRL_EN_MIN_MAX_MF_DEFAULT`"]
    #[inline(always)]
    pub fn is_demod_ctrl_en_min_max_mf_default(&self) -> bool {
        *self == DEMOD_CTRL_EN_MIN_MAX_MF_A::DEMOD_CTRL_EN_MIN_MAX_MF_DEFAULT
    }
}
#[doc = "Write proxy for field `DEMOD_CTRL_EN_MIN_MAX_MF`"]
pub struct DEMOD_CTRL_EN_MIN_MAX_MF_W<'a> {
    w: &'a mut W,
}
impl<'a> DEMOD_CTRL_EN_MIN_MAX_MF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEMOD_CTRL_EN_MIN_MAX_MF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn demod_ctrl_en_min_max_mf_default(self) -> &'a mut W {
        self.variant(DEMOD_CTRL_EN_MIN_MAX_MF_A::DEMOD_CTRL_EN_MIN_MAX_MF_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "If set to 1 enables the sync detection on the non-delayed path; not working in 4FSK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEMOD_CTRL_EN_PRE_SYNC_A {
    #[doc = "0: `0`"]
    DEMOD_CTRL_EN_PRE_SYNC_DEFAULT = 0,
}
impl From<DEMOD_CTRL_EN_PRE_SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: DEMOD_CTRL_EN_PRE_SYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEMOD_CTRL_EN_PRE_SYNC`"]
pub type DEMOD_CTRL_EN_PRE_SYNC_R = crate::R<bool, DEMOD_CTRL_EN_PRE_SYNC_A>;
impl DEMOD_CTRL_EN_PRE_SYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DEMOD_CTRL_EN_PRE_SYNC_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DEMOD_CTRL_EN_PRE_SYNC_A::DEMOD_CTRL_EN_PRE_SYNC_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEMOD_CTRL_EN_PRE_SYNC_DEFAULT`"]
    #[inline(always)]
    pub fn is_demod_ctrl_en_pre_sync_default(&self) -> bool {
        *self == DEMOD_CTRL_EN_PRE_SYNC_A::DEMOD_CTRL_EN_PRE_SYNC_DEFAULT
    }
}
#[doc = "Write proxy for field `DEMOD_CTRL_EN_PRE_SYNC`"]
pub struct DEMOD_CTRL_EN_PRE_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> DEMOD_CTRL_EN_PRE_SYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEMOD_CTRL_EN_PRE_SYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn demod_ctrl_en_pre_sync_default(self) -> &'a mut W {
        self.variant(DEMOD_CTRL_EN_PRE_SYNC_A::DEMOD_CTRL_EN_PRE_SYNC_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "If set to 1 blocks the rssi detection during the slow-down period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEMOD_CTRL_BLOCK_RSSI_DET_A {
    #[doc = "0: `0`"]
    DEMOD_CTRL_BLOCK_RSSI_DET_DEFAULT = 0,
}
impl From<DEMOD_CTRL_BLOCK_RSSI_DET_A> for bool {
    #[inline(always)]
    fn from(variant: DEMOD_CTRL_BLOCK_RSSI_DET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEMOD_CTRL_BLOCK_RSSI_DET`"]
pub type DEMOD_CTRL_BLOCK_RSSI_DET_R = crate::R<bool, DEMOD_CTRL_BLOCK_RSSI_DET_A>;
impl DEMOD_CTRL_BLOCK_RSSI_DET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DEMOD_CTRL_BLOCK_RSSI_DET_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DEMOD_CTRL_BLOCK_RSSI_DET_A::DEMOD_CTRL_BLOCK_RSSI_DET_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEMOD_CTRL_BLOCK_RSSI_DET_DEFAULT`"]
    #[inline(always)]
    pub fn is_demod_ctrl_block_rssi_det_default(&self) -> bool {
        *self == DEMOD_CTRL_BLOCK_RSSI_DET_A::DEMOD_CTRL_BLOCK_RSSI_DET_DEFAULT
    }
}
#[doc = "Write proxy for field `DEMOD_CTRL_BLOCK_RSSI_DET`"]
pub struct DEMOD_CTRL_BLOCK_RSSI_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> DEMOD_CTRL_BLOCK_RSSI_DET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEMOD_CTRL_BLOCK_RSSI_DET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn demod_ctrl_block_rssi_det_default(self) -> &'a mut W {
        self.variant(DEMOD_CTRL_BLOCK_RSSI_DET_A::DEMOD_CTRL_BLOCK_RSSI_DET_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "If set to 1 enables the early fine recovery after the packet detection or pre-sync\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEMOD_CTRL_EARLY_FINE_RECOV_A {
    #[doc = "0: `0`"]
    DEMOD_CTRL_EARLY_FINE_RECOV_DEFAULT = 0,
}
impl From<DEMOD_CTRL_EARLY_FINE_RECOV_A> for bool {
    #[inline(always)]
    fn from(variant: DEMOD_CTRL_EARLY_FINE_RECOV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEMOD_CTRL_EARLY_FINE_RECOV`"]
pub type DEMOD_CTRL_EARLY_FINE_RECOV_R = crate::R<bool, DEMOD_CTRL_EARLY_FINE_RECOV_A>;
impl DEMOD_CTRL_EARLY_FINE_RECOV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DEMOD_CTRL_EARLY_FINE_RECOV_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DEMOD_CTRL_EARLY_FINE_RECOV_A::DEMOD_CTRL_EARLY_FINE_RECOV_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEMOD_CTRL_EARLY_FINE_RECOV_DEFAULT`"]
    #[inline(always)]
    pub fn is_demod_ctrl_early_fine_recov_default(&self) -> bool {
        *self == DEMOD_CTRL_EARLY_FINE_RECOV_A::DEMOD_CTRL_EARLY_FINE_RECOV_DEFAULT
    }
}
#[doc = "Write proxy for field `DEMOD_CTRL_EARLY_FINE_RECOV`"]
pub struct DEMOD_CTRL_EARLY_FINE_RECOV_W<'a> {
    w: &'a mut W,
}
impl<'a> DEMOD_CTRL_EARLY_FINE_RECOV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEMOD_CTRL_EARLY_FINE_RECOV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn demod_ctrl_early_fine_recov_default(self) -> &'a mut W {
        self.variant(DEMOD_CTRL_EARLY_FINE_RECOV_A::DEMOD_CTRL_EARLY_FINE_RECOV_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Number of samples to estimate the carrier offset: 0 -> 32, 1 -> 64, 2 -> 128, 3->256\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSSI_DETECT_RSSI_DET_CR_LEN_A {
    #[doc = "0: `0`"]
    RSSI_DETECT_RSSI_DET_CR_LEN_DEFAULT = 0,
}
impl From<RSSI_DETECT_RSSI_DET_CR_LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: RSSI_DETECT_RSSI_DET_CR_LEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSSI_DETECT_RSSI_DET_CR_LEN`"]
pub type RSSI_DETECT_RSSI_DET_CR_LEN_R = crate::R<u8, RSSI_DETECT_RSSI_DET_CR_LEN_A>;
impl RSSI_DETECT_RSSI_DET_CR_LEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSSI_DETECT_RSSI_DET_CR_LEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RSSI_DETECT_RSSI_DET_CR_LEN_A::RSSI_DETECT_RSSI_DET_CR_LEN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_DETECT_RSSI_DET_CR_LEN_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_detect_rssi_det_cr_len_default(&self) -> bool {
        *self == RSSI_DETECT_RSSI_DET_CR_LEN_A::RSSI_DETECT_RSSI_DET_CR_LEN_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_DETECT_RSSI_DET_CR_LEN`"]
pub struct RSSI_DETECT_RSSI_DET_CR_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_DETECT_RSSI_DET_CR_LEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_DETECT_RSSI_DET_CR_LEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssi_detect_rssi_det_cr_len_default(self) -> &'a mut W {
        self.variant(RSSI_DETECT_RSSI_DET_CR_LEN_A::RSSI_DETECT_RSSI_DET_CR_LEN_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Symbols to wait after the RSSI detection: 00 -> 0, 01 -> 1, 10 -> 2, 11 -> 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSSI_DETECT_RSSI_DET_WAIT_A {
    #[doc = "0: `0`"]
    RSSI_DETECT_RSSI_DET_WAIT_DEFAULT = 0,
}
impl From<RSSI_DETECT_RSSI_DET_WAIT_A> for u8 {
    #[inline(always)]
    fn from(variant: RSSI_DETECT_RSSI_DET_WAIT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSSI_DETECT_RSSI_DET_WAIT`"]
pub type RSSI_DETECT_RSSI_DET_WAIT_R = crate::R<u8, RSSI_DETECT_RSSI_DET_WAIT_A>;
impl RSSI_DETECT_RSSI_DET_WAIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSSI_DETECT_RSSI_DET_WAIT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RSSI_DETECT_RSSI_DET_WAIT_A::RSSI_DETECT_RSSI_DET_WAIT_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_DETECT_RSSI_DET_WAIT_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_detect_rssi_det_wait_default(&self) -> bool {
        *self == RSSI_DETECT_RSSI_DET_WAIT_A::RSSI_DETECT_RSSI_DET_WAIT_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_DETECT_RSSI_DET_WAIT`"]
pub struct RSSI_DETECT_RSSI_DET_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_DETECT_RSSI_DET_WAIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_DETECT_RSSI_DET_WAIT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssi_detect_rssi_det_wait_default(self) -> &'a mut W {
        self.variant(RSSI_DETECT_RSSI_DET_WAIT_A::RSSI_DETECT_RSSI_DET_WAIT_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Set the distance between the actual value and the subtracted one (0->1 sample,1->2 samples,etc)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSSI_DETECT_RSSI_DET_DIFF_LL_A {
    #[doc = "0: `0`"]
    RSSI_DETECT_RSSI_DET_DIFF_LL_DEFAULT = 0,
}
impl From<RSSI_DETECT_RSSI_DET_DIFF_LL_A> for u8 {
    #[inline(always)]
    fn from(variant: RSSI_DETECT_RSSI_DET_DIFF_LL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSSI_DETECT_RSSI_DET_DIFF_LL`"]
pub type RSSI_DETECT_RSSI_DET_DIFF_LL_R = crate::R<u8, RSSI_DETECT_RSSI_DET_DIFF_LL_A>;
impl RSSI_DETECT_RSSI_DET_DIFF_LL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSSI_DETECT_RSSI_DET_DIFF_LL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RSSI_DETECT_RSSI_DET_DIFF_LL_A::RSSI_DETECT_RSSI_DET_DIFF_LL_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_DETECT_RSSI_DET_DIFF_LL_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_detect_rssi_det_diff_ll_default(&self) -> bool {
        *self == RSSI_DETECT_RSSI_DET_DIFF_LL_A::RSSI_DETECT_RSSI_DET_DIFF_LL_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_DETECT_RSSI_DET_DIFF_LL`"]
pub struct RSSI_DETECT_RSSI_DET_DIFF_LL_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_DETECT_RSSI_DET_DIFF_LL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_DETECT_RSSI_DET_DIFF_LL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssi_detect_rssi_det_diff_ll_default(self) -> &'a mut W {
        self.variant(RSSI_DETECT_RSSI_DET_DIFF_LL_A::RSSI_DETECT_RSSI_DET_DIFF_LL_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "If set to 1 enables the absolute RSSI detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSI_DETECT_RSSI_DET_EN_ABS_A {
    #[doc = "0: `0`"]
    RSSI_DETECT_RSSI_DET_EN_ABS_DEFAULT = 0,
}
impl From<RSSI_DETECT_RSSI_DET_EN_ABS_A> for bool {
    #[inline(always)]
    fn from(variant: RSSI_DETECT_RSSI_DET_EN_ABS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSSI_DETECT_RSSI_DET_EN_ABS`"]
pub type RSSI_DETECT_RSSI_DET_EN_ABS_R = crate::R<bool, RSSI_DETECT_RSSI_DET_EN_ABS_A>;
impl RSSI_DETECT_RSSI_DET_EN_ABS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RSSI_DETECT_RSSI_DET_EN_ABS_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(RSSI_DETECT_RSSI_DET_EN_ABS_A::RSSI_DETECT_RSSI_DET_EN_ABS_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_DETECT_RSSI_DET_EN_ABS_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_detect_rssi_det_en_abs_default(&self) -> bool {
        *self == RSSI_DETECT_RSSI_DET_EN_ABS_A::RSSI_DETECT_RSSI_DET_EN_ABS_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_DETECT_RSSI_DET_EN_ABS`"]
pub struct RSSI_DETECT_RSSI_DET_EN_ABS_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_DETECT_RSSI_DET_EN_ABS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_DETECT_RSSI_DET_EN_ABS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssi_detect_rssi_det_en_abs_default(self) -> &'a mut W {
        self.variant(RSSI_DETECT_RSSI_DET_EN_ABS_A::RSSI_DETECT_RSSI_DET_EN_ABS_DEFAULT)
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
#[doc = "If set to 1 enables the differential RSSI detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSI_DETECT_RSSI_DET_EN_DIFF_A {
    #[doc = "0: `0`"]
    RSSI_DETECT_RSSI_DET_EN_DIFF_DEFAULT = 0,
}
impl From<RSSI_DETECT_RSSI_DET_EN_DIFF_A> for bool {
    #[inline(always)]
    fn from(variant: RSSI_DETECT_RSSI_DET_EN_DIFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSSI_DETECT_RSSI_DET_EN_DIFF`"]
pub type RSSI_DETECT_RSSI_DET_EN_DIFF_R = crate::R<bool, RSSI_DETECT_RSSI_DET_EN_DIFF_A>;
impl RSSI_DETECT_RSSI_DET_EN_DIFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RSSI_DETECT_RSSI_DET_EN_DIFF_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(RSSI_DETECT_RSSI_DET_EN_DIFF_A::RSSI_DETECT_RSSI_DET_EN_DIFF_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_DETECT_RSSI_DET_EN_DIFF_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_detect_rssi_det_en_diff_default(&self) -> bool {
        *self == RSSI_DETECT_RSSI_DET_EN_DIFF_A::RSSI_DETECT_RSSI_DET_EN_DIFF_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_DETECT_RSSI_DET_EN_DIFF`"]
pub struct RSSI_DETECT_RSSI_DET_EN_DIFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_DETECT_RSSI_DET_EN_DIFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_DETECT_RSSI_DET_EN_DIFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssi_detect_rssi_det_en_diff_default(self) -> &'a mut W {
        self.variant(RSSI_DETECT_RSSI_DET_EN_DIFF_A::RSSI_DETECT_RSSI_DET_EN_DIFF_DEFAULT)
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
    #[doc = "Bits 24:31 - Threshold used for absolute RSSI detection"]
    #[inline(always)]
    pub fn rssi_detect_abs_thr_rssi_det_abs_thr(&self) -> RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_R {
        RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Threshold used for differential RSSI detection"]
    #[inline(always)]
    pub fn rssi_detect_diff_thr_rssi_det_diff_thr(
        &self,
    ) -> RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_R {
        RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 14 - If set to 1 enable the sync word detection in the delay line. This implies that nc_sel_out = 0x7"]
    #[inline(always)]
    pub fn demod_ctrl_en_delline_sync_det(&self) -> DEMOD_CTRL_EN_DELLINE_SYNC_DET_R {
        DEMOD_CTRL_EN_DELLINE_SYNC_DET_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Add an additional filtering on the RSSI value"]
    #[inline(always)]
    pub fn demod_ctrl_rssi_det_filt(&self) -> DEMOD_CTRL_RSSI_DET_FILT_R {
        DEMOD_CTRL_RSSI_DET_FILT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - If set to 1 speed up the clock recovery during the resto of the preamble"]
    #[inline(always)]
    pub fn demod_ctrl_en_fast_clk_recov(&self) -> DEMOD_CTRL_EN_FAST_CLK_RECOV_R {
        DEMOD_CTRL_EN_FAST_CLK_RECOV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - If set to 1 enables the min max algo after the matched filter"]
    #[inline(always)]
    pub fn demod_ctrl_en_min_max_mf(&self) -> DEMOD_CTRL_EN_MIN_MAX_MF_R {
        DEMOD_CTRL_EN_MIN_MAX_MF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - If set to 1 enables the sync detection on the non-delayed path; not working in 4FSK"]
    #[inline(always)]
    pub fn demod_ctrl_en_pre_sync(&self) -> DEMOD_CTRL_EN_PRE_SYNC_R {
        DEMOD_CTRL_EN_PRE_SYNC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - If set to 1 blocks the rssi detection during the slow-down period"]
    #[inline(always)]
    pub fn demod_ctrl_block_rssi_det(&self) -> DEMOD_CTRL_BLOCK_RSSI_DET_R {
        DEMOD_CTRL_BLOCK_RSSI_DET_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - If set to 1 enables the early fine recovery after the packet detection or pre-sync"]
    #[inline(always)]
    pub fn demod_ctrl_early_fine_recov(&self) -> DEMOD_CTRL_EARLY_FINE_RECOV_R {
        DEMOD_CTRL_EARLY_FINE_RECOV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Number of samples to estimate the carrier offset: 0 -> 32, 1 -> 64, 2 -> 128, 3->256"]
    #[inline(always)]
    pub fn rssi_detect_rssi_det_cr_len(&self) -> RSSI_DETECT_RSSI_DET_CR_LEN_R {
        RSSI_DETECT_RSSI_DET_CR_LEN_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Symbols to wait after the RSSI detection: 00 -> 0, 01 -> 1, 10 -> 2, 11 -> 4"]
    #[inline(always)]
    pub fn rssi_detect_rssi_det_wait(&self) -> RSSI_DETECT_RSSI_DET_WAIT_R {
        RSSI_DETECT_RSSI_DET_WAIT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Set the distance between the actual value and the subtracted one (0->1 sample,1->2 samples,etc)"]
    #[inline(always)]
    pub fn rssi_detect_rssi_det_diff_ll(&self) -> RSSI_DETECT_RSSI_DET_DIFF_LL_R {
        RSSI_DETECT_RSSI_DET_DIFF_LL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - If set to 1 enables the absolute RSSI detection"]
    #[inline(always)]
    pub fn rssi_detect_rssi_det_en_abs(&self) -> RSSI_DETECT_RSSI_DET_EN_ABS_R {
        RSSI_DETECT_RSSI_DET_EN_ABS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - If set to 1 enables the differential RSSI detection"]
    #[inline(always)]
    pub fn rssi_detect_rssi_det_en_diff(&self) -> RSSI_DETECT_RSSI_DET_EN_DIFF_R {
        RSSI_DETECT_RSSI_DET_EN_DIFF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - Threshold used for absolute RSSI detection"]
    #[inline(always)]
    pub fn rssi_detect_abs_thr_rssi_det_abs_thr(
        &mut self,
    ) -> RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_W {
        RSSI_DETECT_ABS_THR_RSSI_DET_ABS_THR_W { w: self }
    }
    #[doc = "Bits 16:23 - Threshold used for differential RSSI detection"]
    #[inline(always)]
    pub fn rssi_detect_diff_thr_rssi_det_diff_thr(
        &mut self,
    ) -> RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_W {
        RSSI_DETECT_DIFF_THR_RSSI_DET_DIFF_THR_W { w: self }
    }
    #[doc = "Bit 14 - If set to 1 enable the sync word detection in the delay line. This implies that nc_sel_out = 0x7"]
    #[inline(always)]
    pub fn demod_ctrl_en_delline_sync_det(&mut self) -> DEMOD_CTRL_EN_DELLINE_SYNC_DET_W {
        DEMOD_CTRL_EN_DELLINE_SYNC_DET_W { w: self }
    }
    #[doc = "Bit 13 - Add an additional filtering on the RSSI value"]
    #[inline(always)]
    pub fn demod_ctrl_rssi_det_filt(&mut self) -> DEMOD_CTRL_RSSI_DET_FILT_W {
        DEMOD_CTRL_RSSI_DET_FILT_W { w: self }
    }
    #[doc = "Bit 12 - If set to 1 speed up the clock recovery during the resto of the preamble"]
    #[inline(always)]
    pub fn demod_ctrl_en_fast_clk_recov(&mut self) -> DEMOD_CTRL_EN_FAST_CLK_RECOV_W {
        DEMOD_CTRL_EN_FAST_CLK_RECOV_W { w: self }
    }
    #[doc = "Bit 11 - If set to 1 enables the min max algo after the matched filter"]
    #[inline(always)]
    pub fn demod_ctrl_en_min_max_mf(&mut self) -> DEMOD_CTRL_EN_MIN_MAX_MF_W {
        DEMOD_CTRL_EN_MIN_MAX_MF_W { w: self }
    }
    #[doc = "Bit 10 - If set to 1 enables the sync detection on the non-delayed path; not working in 4FSK"]
    #[inline(always)]
    pub fn demod_ctrl_en_pre_sync(&mut self) -> DEMOD_CTRL_EN_PRE_SYNC_W {
        DEMOD_CTRL_EN_PRE_SYNC_W { w: self }
    }
    #[doc = "Bit 9 - If set to 1 blocks the rssi detection during the slow-down period"]
    #[inline(always)]
    pub fn demod_ctrl_block_rssi_det(&mut self) -> DEMOD_CTRL_BLOCK_RSSI_DET_W {
        DEMOD_CTRL_BLOCK_RSSI_DET_W { w: self }
    }
    #[doc = "Bit 8 - If set to 1 enables the early fine recovery after the packet detection or pre-sync"]
    #[inline(always)]
    pub fn demod_ctrl_early_fine_recov(&mut self) -> DEMOD_CTRL_EARLY_FINE_RECOV_W {
        DEMOD_CTRL_EARLY_FINE_RECOV_W { w: self }
    }
    #[doc = "Bits 6:7 - Number of samples to estimate the carrier offset: 0 -> 32, 1 -> 64, 2 -> 128, 3->256"]
    #[inline(always)]
    pub fn rssi_detect_rssi_det_cr_len(&mut self) -> RSSI_DETECT_RSSI_DET_CR_LEN_W {
        RSSI_DETECT_RSSI_DET_CR_LEN_W { w: self }
    }
    #[doc = "Bits 4:5 - Symbols to wait after the RSSI detection: 00 -> 0, 01 -> 1, 10 -> 2, 11 -> 4"]
    #[inline(always)]
    pub fn rssi_detect_rssi_det_wait(&mut self) -> RSSI_DETECT_RSSI_DET_WAIT_W {
        RSSI_DETECT_RSSI_DET_WAIT_W { w: self }
    }
    #[doc = "Bits 2:3 - Set the distance between the actual value and the subtracted one (0->1 sample,1->2 samples,etc)"]
    #[inline(always)]
    pub fn rssi_detect_rssi_det_diff_ll(&mut self) -> RSSI_DETECT_RSSI_DET_DIFF_LL_W {
        RSSI_DETECT_RSSI_DET_DIFF_LL_W { w: self }
    }
    #[doc = "Bit 1 - If set to 1 enables the absolute RSSI detection"]
    #[inline(always)]
    pub fn rssi_detect_rssi_det_en_abs(&mut self) -> RSSI_DETECT_RSSI_DET_EN_ABS_W {
        RSSI_DETECT_RSSI_DET_EN_ABS_W { w: self }
    }
    #[doc = "Bit 0 - If set to 1 enables the differential RSSI detection"]
    #[inline(always)]
    pub fn rssi_detect_rssi_det_en_diff(&mut self) -> RSSI_DETECT_RSSI_DET_EN_DIFF_W {
        RSSI_DETECT_RSSI_DET_EN_DIFF_W { w: self }
    }
}
