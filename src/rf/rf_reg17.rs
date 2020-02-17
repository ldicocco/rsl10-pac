#[doc = "Reader of register RF_REG17"]
pub type R = crate::R<u32, super::RF_REG17>;
#[doc = "Writer for register RF_REG17"]
pub type W = crate::W<u32, super::RF_REG17>;
#[doc = "Register RF_REG17 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG17 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FSK amplitude 3 (highest): in 4FSK is the high amplitude (+/-3). in FSK w/ ISI it specify the highet amplitude (generally it corresponds to a sequence 1-1-1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FSK_FCR_AMP_3_FSK_FCR_AMP3_A {
    #[doc = "0: `0`"]
    FSK_FCR_AMP_3_FSK_FCR_AMP3_DEFAULT = 0,
}
impl From<FSK_FCR_AMP_3_FSK_FCR_AMP3_A> for u8 {
    #[inline(always)]
    fn from(variant: FSK_FCR_AMP_3_FSK_FCR_AMP3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FSK_FCR_AMP_3_FSK_FCR_AMP3`"]
pub type FSK_FCR_AMP_3_FSK_FCR_AMP3_R = crate::R<u8, FSK_FCR_AMP_3_FSK_FCR_AMP3_A>;
impl FSK_FCR_AMP_3_FSK_FCR_AMP3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FSK_FCR_AMP_3_FSK_FCR_AMP3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FSK_FCR_AMP_3_FSK_FCR_AMP3_A::FSK_FCR_AMP_3_FSK_FCR_AMP3_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FSK_FCR_AMP_3_FSK_FCR_AMP3_DEFAULT`"]
    #[inline(always)]
    pub fn is_fsk_fcr_amp_3_fsk_fcr_amp3_default(&self) -> bool {
        *self == FSK_FCR_AMP_3_FSK_FCR_AMP3_A::FSK_FCR_AMP_3_FSK_FCR_AMP3_DEFAULT
    }
}
#[doc = "Write proxy for field `FSK_FCR_AMP_3_FSK_FCR_AMP3`"]
pub struct FSK_FCR_AMP_3_FSK_FCR_AMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> FSK_FCR_AMP_3_FSK_FCR_AMP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSK_FCR_AMP_3_FSK_FCR_AMP3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fsk_fcr_amp_3_fsk_fcr_amp3_default(self) -> &'a mut W {
        self.variant(FSK_FCR_AMP_3_FSK_FCR_AMP3_A::FSK_FCR_AMP_3_FSK_FCR_AMP3_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "FSK amplitude 2 (mid): in 4FSK is the threshold. in FSK w/ ISI it specify the mid amplitude (generally it corresponds to a sequence 0-1-1 or 1-1-0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FSK_FCR_AMP_2_FSK_FCR_AMP2_A {
    #[doc = "0: `0`"]
    FSK_FCR_AMP_2_FSK_FCR_AMP2_DEFAULT = 0,
}
impl From<FSK_FCR_AMP_2_FSK_FCR_AMP2_A> for u8 {
    #[inline(always)]
    fn from(variant: FSK_FCR_AMP_2_FSK_FCR_AMP2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FSK_FCR_AMP_2_FSK_FCR_AMP2`"]
pub type FSK_FCR_AMP_2_FSK_FCR_AMP2_R = crate::R<u8, FSK_FCR_AMP_2_FSK_FCR_AMP2_A>;
impl FSK_FCR_AMP_2_FSK_FCR_AMP2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FSK_FCR_AMP_2_FSK_FCR_AMP2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FSK_FCR_AMP_2_FSK_FCR_AMP2_A::FSK_FCR_AMP_2_FSK_FCR_AMP2_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FSK_FCR_AMP_2_FSK_FCR_AMP2_DEFAULT`"]
    #[inline(always)]
    pub fn is_fsk_fcr_amp_2_fsk_fcr_amp2_default(&self) -> bool {
        *self == FSK_FCR_AMP_2_FSK_FCR_AMP2_A::FSK_FCR_AMP_2_FSK_FCR_AMP2_DEFAULT
    }
}
#[doc = "Write proxy for field `FSK_FCR_AMP_2_FSK_FCR_AMP2`"]
pub struct FSK_FCR_AMP_2_FSK_FCR_AMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> FSK_FCR_AMP_2_FSK_FCR_AMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSK_FCR_AMP_2_FSK_FCR_AMP2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fsk_fcr_amp_2_fsk_fcr_amp2_default(self) -> &'a mut W {
        self.variant(FSK_FCR_AMP_2_FSK_FCR_AMP2_A::FSK_FCR_AMP_2_FSK_FCR_AMP2_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Set the maximum errors in the delay line sync detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_A {
    #[doc = "0: `0`"]
    CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_DEFAULT = 0,
}
impl From<CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_A> for u8 {
    #[inline(always)]
    fn from(variant: CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC`"]
pub type CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_R =
    crate::R<u8, CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_A>;
impl CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_A> {
        use crate::Variant::*;
        match self . bits { 0 => Val ( CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_A :: CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_DEFAULT ) , i => Res ( i ) }
    }
    #[doc = "Checks if the value of the field is `CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_DEFAULT`"]
    #[inline(always)]
    pub fn is_carrier_recovery_extra_max_err_in_dl_sync_default(&self) -> bool {
        * self == CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_A :: CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_DEFAULT
    }
}
#[doc = "Write proxy for field `CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC`"]
pub struct CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn carrier_recovery_extra_max_err_in_dl_sync_default(self) -> &'a mut W {
        self . variant ( CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_A :: CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_DEFAULT )
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "If set to 1 uses the pattern_ok signal in delay line to synchronize the deserializer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_A {
    #[doc = "0: `0`"]
    CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_DEFAULT = 0,
}
impl From<CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_A> for bool {
    #[inline(always)]
    fn from(variant: CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE`"]
pub type CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_R =
    crate::R<bool, CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_A>;
impl CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_A> {
        use crate::Variant::*;
        match self . bits { false => Val ( CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_A :: CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_DEFAULT ) , i => Res ( i ) }
    }
    #[doc = "Checks if the value of the field is `CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_DEFAULT`"]
    #[inline(always)]
    pub fn is_carrier_recovery_extra_en_sync_ok_delay_line_default(&self) -> bool {
        * self == CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_A :: CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_DEFAULT
    }
}
#[doc = "Write proxy for field `CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE`"]
pub struct CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn carrier_recovery_extra_en_sync_ok_delay_line_default(self) -> &'a mut W {
        self . variant ( CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_A :: CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_DEFAULT )
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
#[doc = "Select the output position for the 'not-causal processing': 000 => 4 symbol, 001 => 6 symbols, 010 => 8 symbols, 011 => 12 symbols, 100 => 16 symbols, 101 => 24 symbols, 110 => 32 symbols, 111 => 40 symbols\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_A {
    #[doc = "0: `0`"]
    CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_DEFAULT = 0,
}
impl From<CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_A> for u8 {
    #[inline(always)]
    fn from(variant: CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CARRIER_RECOVERY_EXTRA_NC_SEL_OUT`"]
pub type CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_R = crate::R<u8, CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_A>;
impl CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => {
                Val(CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_A::CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_DEFAULT)
            }
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_DEFAULT`"]
    #[inline(always)]
    pub fn is_carrier_recovery_extra_nc_sel_out_default(&self) -> bool {
        *self == CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_A::CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_DEFAULT
    }
}
#[doc = "Write proxy for field `CARRIER_RECOVERY_EXTRA_NC_SEL_OUT`"]
pub struct CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn carrier_recovery_extra_nc_sel_out_default(self) -> &'a mut W {
        self.variant(CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_A::CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "if set to 1 enables the not causal processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_A {
    #[doc = "0: `0`"]
    CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_DEFAULT = 0,
}
impl From<CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_A> for bool {
    #[inline(always)]
    fn from(variant: CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL`"]
pub type CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_R =
    crate::R<bool, CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_A>;
impl CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_A> {
        use crate::Variant::*;
        match self . bits { false => Val ( CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_A :: CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_DEFAULT ) , i => Res ( i ) }
    }
    #[doc = "Checks if the value of the field is `CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_DEFAULT`"]
    #[inline(always)]
    pub fn is_carrier_recovery_extra_en_not_causal_default(&self) -> bool {
        *self
            == CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_A::CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_DEFAULT
    }
}
#[doc = "Write proxy for field `CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL`"]
pub struct CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn carrier_recovery_extra_en_not_causal_default(self) -> &'a mut W {
        self.variant(
            CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_A::CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_DEFAULT,
        )
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
#[doc = "Mantissa of the carrier recovery frequency limit (unsigned).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_A {
    #[doc = "0: `0`"]
    CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_DEFAULT = 0,
}
impl From<CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_A> for u8 {
    #[inline(always)]
    fn from(variant: CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN`"]
pub type CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_R =
    crate::R<u8, CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_A>;
impl CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_A> {
        use crate::Variant::*;
        match self . bits { 0 => Val ( CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_A :: CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_DEFAULT ) , i => Res ( i ) }
    }
    #[doc = "Checks if the value of the field is `CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_DEFAULT`"]
    #[inline(always)]
    pub fn is_carrier_recovery_extra_freq_limit_man_default(&self) -> bool {
        * self == CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_A :: CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_DEFAULT
    }
}
#[doc = "Write proxy for field `CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN`"]
pub struct CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn carrier_recovery_extra_freq_limit_man_default(self) -> &'a mut W {
        self.variant(
            CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_A::CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_DEFAULT,
        )
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Exponent of the carrier recovery frequency limit (signed). Formula: carrier_offset_max=(1+m/8)*2^e/4*f_sym\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_A {
    #[doc = "0: `0`"]
    CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_DEFAULT = 0,
}
impl From<CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_A> for u8 {
    #[inline(always)]
    fn from(variant: CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP`"]
pub type CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_R =
    crate::R<u8, CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_A>;
impl CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_A> {
        use crate::Variant::*;
        match self . bits { 0 => Val ( CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_A :: CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_DEFAULT ) , i => Res ( i ) }
    }
    #[doc = "Checks if the value of the field is `CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_DEFAULT`"]
    #[inline(always)]
    pub fn is_carrier_recovery_extra_freq_limit_exp_default(&self) -> bool {
        * self == CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_A :: CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_DEFAULT
    }
}
#[doc = "Write proxy for field `CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP`"]
pub struct CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn carrier_recovery_extra_freq_limit_exp_default(self) -> &'a mut W {
        self.variant(
            CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_A::CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_DEFAULT,
        )
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - FSK amplitude 3 (highest): in 4FSK is the high amplitude (+/-3). in FSK w/ ISI it specify the highet amplitude (generally it corresponds to a sequence 1-1-1."]
    #[inline(always)]
    pub fn fsk_fcr_amp_3_fsk_fcr_amp3(&self) -> FSK_FCR_AMP_3_FSK_FCR_AMP3_R {
        FSK_FCR_AMP_3_FSK_FCR_AMP3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - FSK amplitude 2 (mid): in 4FSK is the threshold. in FSK w/ ISI it specify the mid amplitude (generally it corresponds to a sequence 0-1-1 or 1-1-0."]
    #[inline(always)]
    pub fn fsk_fcr_amp_2_fsk_fcr_amp2(&self) -> FSK_FCR_AMP_2_FSK_FCR_AMP2_R {
        FSK_FCR_AMP_2_FSK_FCR_AMP2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 13:14 - Set the maximum errors in the delay line sync detection"]
    #[inline(always)]
    pub fn carrier_recovery_extra_max_err_in_dl_sync(
        &self,
    ) -> CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_R {
        CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - If set to 1 uses the pattern_ok signal in delay line to synchronize the deserializer"]
    #[inline(always)]
    pub fn carrier_recovery_extra_en_sync_ok_delay_line(
        &self,
    ) -> CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_R {
        CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - Select the output position for the 'not-causal processing': 000 => 4 symbol, 001 => 6 symbols, 010 => 8 symbols, 011 => 12 symbols, 100 => 16 symbols, 101 => 24 symbols, 110 => 32 symbols, 111 => 40 symbols"]
    #[inline(always)]
    pub fn carrier_recovery_extra_nc_sel_out(&self) -> CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_R {
        CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 8 - if set to 1 enables the not causal processing"]
    #[inline(always)]
    pub fn carrier_recovery_extra_en_not_causal(&self) -> CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_R {
        CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Mantissa of the carrier recovery frequency limit (unsigned)."]
    #[inline(always)]
    pub fn carrier_recovery_extra_freq_limit_man(&self) -> CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_R {
        CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Exponent of the carrier recovery frequency limit (signed). Formula: carrier_offset_max=(1+m/8)*2^e/4*f_sym"]
    #[inline(always)]
    pub fn carrier_recovery_extra_freq_limit_exp(&self) -> CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_R {
        CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - FSK amplitude 3 (highest): in 4FSK is the high amplitude (+/-3). in FSK w/ ISI it specify the highet amplitude (generally it corresponds to a sequence 1-1-1."]
    #[inline(always)]
    pub fn fsk_fcr_amp_3_fsk_fcr_amp3(&mut self) -> FSK_FCR_AMP_3_FSK_FCR_AMP3_W {
        FSK_FCR_AMP_3_FSK_FCR_AMP3_W { w: self }
    }
    #[doc = "Bits 16:23 - FSK amplitude 2 (mid): in 4FSK is the threshold. in FSK w/ ISI it specify the mid amplitude (generally it corresponds to a sequence 0-1-1 or 1-1-0."]
    #[inline(always)]
    pub fn fsk_fcr_amp_2_fsk_fcr_amp2(&mut self) -> FSK_FCR_AMP_2_FSK_FCR_AMP2_W {
        FSK_FCR_AMP_2_FSK_FCR_AMP2_W { w: self }
    }
    #[doc = "Bits 13:14 - Set the maximum errors in the delay line sync detection"]
    #[inline(always)]
    pub fn carrier_recovery_extra_max_err_in_dl_sync(
        &mut self,
    ) -> CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_W {
        CARRIER_RECOVERY_EXTRA_MAX_ERR_IN_DL_SYNC_W { w: self }
    }
    #[doc = "Bit 12 - If set to 1 uses the pattern_ok signal in delay line to synchronize the deserializer"]
    #[inline(always)]
    pub fn carrier_recovery_extra_en_sync_ok_delay_line(
        &mut self,
    ) -> CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_W {
        CARRIER_RECOVERY_EXTRA_EN_SYNC_OK_DELAY_LINE_W { w: self }
    }
    #[doc = "Bits 9:11 - Select the output position for the 'not-causal processing': 000 => 4 symbol, 001 => 6 symbols, 010 => 8 symbols, 011 => 12 symbols, 100 => 16 symbols, 101 => 24 symbols, 110 => 32 symbols, 111 => 40 symbols"]
    #[inline(always)]
    pub fn carrier_recovery_extra_nc_sel_out(&mut self) -> CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_W {
        CARRIER_RECOVERY_EXTRA_NC_SEL_OUT_W { w: self }
    }
    #[doc = "Bit 8 - if set to 1 enables the not causal processing"]
    #[inline(always)]
    pub fn carrier_recovery_extra_en_not_causal(
        &mut self,
    ) -> CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_W {
        CARRIER_RECOVERY_EXTRA_EN_NOT_CAUSAL_W { w: self }
    }
    #[doc = "Bits 4:6 - Mantissa of the carrier recovery frequency limit (unsigned)."]
    #[inline(always)]
    pub fn carrier_recovery_extra_freq_limit_man(
        &mut self,
    ) -> CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_W {
        CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_MAN_W { w: self }
    }
    #[doc = "Bits 0:2 - Exponent of the carrier recovery frequency limit (signed). Formula: carrier_offset_max=(1+m/8)*2^e/4*f_sym"]
    #[inline(always)]
    pub fn carrier_recovery_extra_freq_limit_exp(
        &mut self,
    ) -> CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_W {
        CARRIER_RECOVERY_EXTRA_FREQ_LIMIT_EXP_W { w: self }
    }
}
