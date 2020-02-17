#[doc = "Reader of register RF_REG10"]
pub type R = crate::R<u32, super::RF_REG10>;
#[doc = "Writer for register RF_REG10"]
pub type W = crate::W<u32, super::RF_REG10>;
#[doc = "Register RF_REG10 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Gain of the phase resampling block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FRONTEND2_RESAMPLE_PH_GAIN_A {
    #[doc = "0: `0`"]
    FRONTEND2_RESAMPLE_PH_GAIN_DEFAULT = 0,
}
impl From<FRONTEND2_RESAMPLE_PH_GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: FRONTEND2_RESAMPLE_PH_GAIN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FRONTEND2_RESAMPLE_PH_GAIN`"]
pub type FRONTEND2_RESAMPLE_PH_GAIN_R = crate::R<u8, FRONTEND2_RESAMPLE_PH_GAIN_A>;
impl FRONTEND2_RESAMPLE_PH_GAIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FRONTEND2_RESAMPLE_PH_GAIN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FRONTEND2_RESAMPLE_PH_GAIN_A::FRONTEND2_RESAMPLE_PH_GAIN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FRONTEND2_RESAMPLE_PH_GAIN_DEFAULT`"]
    #[inline(always)]
    pub fn is_frontend2_resample_ph_gain_default(&self) -> bool {
        *self == FRONTEND2_RESAMPLE_PH_GAIN_A::FRONTEND2_RESAMPLE_PH_GAIN_DEFAULT
    }
}
#[doc = "Write proxy for field `FRONTEND2_RESAMPLE_PH_GAIN`"]
pub struct FRONTEND2_RESAMPLE_PH_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRONTEND2_RESAMPLE_PH_GAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRONTEND2_RESAMPLE_PH_GAIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn frontend2_resample_ph_gain_default(self) -> &'a mut W {
        self.variant(FRONTEND2_RESAMPLE_PH_GAIN_A::FRONTEND2_RESAMPLE_PH_GAIN_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
#[doc = "Gain of the decimator in the RSSI resampling block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FRONTEND2_RESAMPLE_RSSI_G2_A {
    #[doc = "0: `0`"]
    FRONTEND2_RESAMPLE_RSSI_G2_DEFAULT = 0,
}
impl From<FRONTEND2_RESAMPLE_RSSI_G2_A> for u8 {
    #[inline(always)]
    fn from(variant: FRONTEND2_RESAMPLE_RSSI_G2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FRONTEND2_RESAMPLE_RSSI_G2`"]
pub type FRONTEND2_RESAMPLE_RSSI_G2_R = crate::R<u8, FRONTEND2_RESAMPLE_RSSI_G2_A>;
impl FRONTEND2_RESAMPLE_RSSI_G2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FRONTEND2_RESAMPLE_RSSI_G2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FRONTEND2_RESAMPLE_RSSI_G2_A::FRONTEND2_RESAMPLE_RSSI_G2_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FRONTEND2_RESAMPLE_RSSI_G2_DEFAULT`"]
    #[inline(always)]
    pub fn is_frontend2_resample_rssi_g2_default(&self) -> bool {
        *self == FRONTEND2_RESAMPLE_RSSI_G2_A::FRONTEND2_RESAMPLE_RSSI_G2_DEFAULT
    }
}
#[doc = "Write proxy for field `FRONTEND2_RESAMPLE_RSSI_G2`"]
pub struct FRONTEND2_RESAMPLE_RSSI_G2_W<'a> {
    w: &'a mut W,
}
impl<'a> FRONTEND2_RESAMPLE_RSSI_G2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRONTEND2_RESAMPLE_RSSI_G2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn frontend2_resample_rssi_g2_default(self) -> &'a mut W {
        self.variant(FRONTEND2_RESAMPLE_RSSI_G2_A::FRONTEND2_RESAMPLE_RSSI_G2_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
#[doc = "Gain of the interpolator in the RSSI resampling block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FRONTEND2_RESAMPLE_RSSI_G1_A {
    #[doc = "0: `0`"]
    FRONTEND2_RESAMPLE_RSSI_G1_DEFAULT = 0,
}
impl From<FRONTEND2_RESAMPLE_RSSI_G1_A> for u8 {
    #[inline(always)]
    fn from(variant: FRONTEND2_RESAMPLE_RSSI_G1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FRONTEND2_RESAMPLE_RSSI_G1`"]
pub type FRONTEND2_RESAMPLE_RSSI_G1_R = crate::R<u8, FRONTEND2_RESAMPLE_RSSI_G1_A>;
impl FRONTEND2_RESAMPLE_RSSI_G1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FRONTEND2_RESAMPLE_RSSI_G1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FRONTEND2_RESAMPLE_RSSI_G1_A::FRONTEND2_RESAMPLE_RSSI_G1_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FRONTEND2_RESAMPLE_RSSI_G1_DEFAULT`"]
    #[inline(always)]
    pub fn is_frontend2_resample_rssi_g1_default(&self) -> bool {
        *self == FRONTEND2_RESAMPLE_RSSI_G1_A::FRONTEND2_RESAMPLE_RSSI_G1_DEFAULT
    }
}
#[doc = "Write proxy for field `FRONTEND2_RESAMPLE_RSSI_G1`"]
pub struct FRONTEND2_RESAMPLE_RSSI_G1_W<'a> {
    w: &'a mut W,
}
impl<'a> FRONTEND2_RESAMPLE_RSSI_G1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRONTEND2_RESAMPLE_RSSI_G1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn frontend2_resample_rssi_g1_default(self) -> &'a mut W {
        self.variant(FRONTEND2_RESAMPLE_RSSI_G1_A::FRONTEND2_RESAMPLE_RSSI_G1_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "If set to 1 enables the phADC deglitcher\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRONTEND_EN_PHADC_DEGLITCH_A {
    #[doc = "0: `0`"]
    FRONTEND_EN_PHADC_DEGLITCH_DEFAULT = 0,
}
impl From<FRONTEND_EN_PHADC_DEGLITCH_A> for bool {
    #[inline(always)]
    fn from(variant: FRONTEND_EN_PHADC_DEGLITCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRONTEND_EN_PHADC_DEGLITCH`"]
pub type FRONTEND_EN_PHADC_DEGLITCH_R = crate::R<bool, FRONTEND_EN_PHADC_DEGLITCH_A>;
impl FRONTEND_EN_PHADC_DEGLITCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FRONTEND_EN_PHADC_DEGLITCH_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(FRONTEND_EN_PHADC_DEGLITCH_A::FRONTEND_EN_PHADC_DEGLITCH_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FRONTEND_EN_PHADC_DEGLITCH_DEFAULT`"]
    #[inline(always)]
    pub fn is_frontend_en_phadc_deglitch_default(&self) -> bool {
        *self == FRONTEND_EN_PHADC_DEGLITCH_A::FRONTEND_EN_PHADC_DEGLITCH_DEFAULT
    }
}
#[doc = "Write proxy for field `FRONTEND_EN_PHADC_DEGLITCH`"]
pub struct FRONTEND_EN_PHADC_DEGLITCH_W<'a> {
    w: &'a mut W,
}
impl<'a> FRONTEND_EN_PHADC_DEGLITCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRONTEND_EN_PHADC_DEGLITCH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn frontend_en_phadc_deglitch_default(self) -> &'a mut W {
        self.variant(FRONTEND_EN_PHADC_DEGLITCH_A::FRONTEND_EN_PHADC_DEGLITCH_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "If set to 1 enables the RSSI resampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRONTEND_EN_RESAMPLE_RSSI_A {
    #[doc = "0: `0`"]
    FRONTEND_EN_RESAMPLE_RSSI_DEFAULT = 0,
}
impl From<FRONTEND_EN_RESAMPLE_RSSI_A> for bool {
    #[inline(always)]
    fn from(variant: FRONTEND_EN_RESAMPLE_RSSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRONTEND_EN_RESAMPLE_RSSI`"]
pub type FRONTEND_EN_RESAMPLE_RSSI_R = crate::R<bool, FRONTEND_EN_RESAMPLE_RSSI_A>;
impl FRONTEND_EN_RESAMPLE_RSSI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FRONTEND_EN_RESAMPLE_RSSI_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(FRONTEND_EN_RESAMPLE_RSSI_A::FRONTEND_EN_RESAMPLE_RSSI_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FRONTEND_EN_RESAMPLE_RSSI_DEFAULT`"]
    #[inline(always)]
    pub fn is_frontend_en_resample_rssi_default(&self) -> bool {
        *self == FRONTEND_EN_RESAMPLE_RSSI_A::FRONTEND_EN_RESAMPLE_RSSI_DEFAULT
    }
}
#[doc = "Write proxy for field `FRONTEND_EN_RESAMPLE_RSSI`"]
pub struct FRONTEND_EN_RESAMPLE_RSSI_W<'a> {
    w: &'a mut W,
}
impl<'a> FRONTEND_EN_RESAMPLE_RSSI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRONTEND_EN_RESAMPLE_RSSI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn frontend_en_resample_rssi_default(self) -> &'a mut W {
        self.variant(FRONTEND_EN_RESAMPLE_RSSI_A::FRONTEND_EN_RESAMPLE_RSSI_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "If set to 1 enables the phase resampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRONTEND_EN_RESAMPLE_PHADC_A {
    #[doc = "0: `0`"]
    FRONTEND_EN_RESAMPLE_PHADC_DEFAULT = 0,
}
impl From<FRONTEND_EN_RESAMPLE_PHADC_A> for bool {
    #[inline(always)]
    fn from(variant: FRONTEND_EN_RESAMPLE_PHADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRONTEND_EN_RESAMPLE_PHADC`"]
pub type FRONTEND_EN_RESAMPLE_PHADC_R = crate::R<bool, FRONTEND_EN_RESAMPLE_PHADC_A>;
impl FRONTEND_EN_RESAMPLE_PHADC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FRONTEND_EN_RESAMPLE_PHADC_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(FRONTEND_EN_RESAMPLE_PHADC_A::FRONTEND_EN_RESAMPLE_PHADC_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FRONTEND_EN_RESAMPLE_PHADC_DEFAULT`"]
    #[inline(always)]
    pub fn is_frontend_en_resample_phadc_default(&self) -> bool {
        *self == FRONTEND_EN_RESAMPLE_PHADC_A::FRONTEND_EN_RESAMPLE_PHADC_DEFAULT
    }
}
#[doc = "Write proxy for field `FRONTEND_EN_RESAMPLE_PHADC`"]
pub struct FRONTEND_EN_RESAMPLE_PHADC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRONTEND_EN_RESAMPLE_PHADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRONTEND_EN_RESAMPLE_PHADC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn frontend_en_resample_phadc_default(self) -> &'a mut W {
        self.variant(FRONTEND_EN_RESAMPLE_PHADC_A::FRONTEND_EN_RESAMPLE_PHADC_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Unsigned value that specifies the divider to obtain the phADC clock (and RSSI).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FRONTEND_DIV_PHADC_A {
    #[doc = "0: `0`"]
    FRONTEND_DIV_PHADC_DEFAULT = 0,
}
impl From<FRONTEND_DIV_PHADC_A> for u8 {
    #[inline(always)]
    fn from(variant: FRONTEND_DIV_PHADC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FRONTEND_DIV_PHADC`"]
pub type FRONTEND_DIV_PHADC_R = crate::R<u8, FRONTEND_DIV_PHADC_A>;
impl FRONTEND_DIV_PHADC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FRONTEND_DIV_PHADC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FRONTEND_DIV_PHADC_A::FRONTEND_DIV_PHADC_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FRONTEND_DIV_PHADC_DEFAULT`"]
    #[inline(always)]
    pub fn is_frontend_div_phadc_default(&self) -> bool {
        *self == FRONTEND_DIV_PHADC_A::FRONTEND_DIV_PHADC_DEFAULT
    }
}
#[doc = "Write proxy for field `FRONTEND_DIV_PHADC`"]
pub struct FRONTEND_DIV_PHADC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRONTEND_DIV_PHADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRONTEND_DIV_PHADC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn frontend_div_phadc_default(self) -> &'a mut W {
        self.variant(FRONTEND_DIV_PHADC_A::FRONTEND_DIV_PHADC_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Exponent of the Tx multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_MULT_TX_MULT_EXP_A {
    #[doc = "0: `0`"]
    TX_MULT_TX_MULT_EXP_DEFAULT = 0,
}
impl From<TX_MULT_TX_MULT_EXP_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_MULT_TX_MULT_EXP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_MULT_TX_MULT_EXP`"]
pub type TX_MULT_TX_MULT_EXP_R = crate::R<u8, TX_MULT_TX_MULT_EXP_A>;
impl TX_MULT_TX_MULT_EXP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_MULT_TX_MULT_EXP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_MULT_TX_MULT_EXP_A::TX_MULT_TX_MULT_EXP_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_MULT_TX_MULT_EXP_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_mult_tx_mult_exp_default(&self) -> bool {
        *self == TX_MULT_TX_MULT_EXP_A::TX_MULT_TX_MULT_EXP_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_MULT_TX_MULT_EXP`"]
pub struct TX_MULT_TX_MULT_EXP_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MULT_TX_MULT_EXP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_MULT_TX_MULT_EXP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_mult_tx_mult_exp_default(self) -> &'a mut W {
        self.variant(TX_MULT_TX_MULT_EXP_A::TX_MULT_TX_MULT_EXP_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Mantissa of the Tx multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_MULT_TX_MULT_MAN_A {
    #[doc = "0: `0`"]
    TX_MULT_TX_MULT_MAN_DEFAULT = 0,
}
impl From<TX_MULT_TX_MULT_MAN_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_MULT_TX_MULT_MAN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_MULT_TX_MULT_MAN`"]
pub type TX_MULT_TX_MULT_MAN_R = crate::R<u8, TX_MULT_TX_MULT_MAN_A>;
impl TX_MULT_TX_MULT_MAN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_MULT_TX_MULT_MAN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_MULT_TX_MULT_MAN_A::TX_MULT_TX_MULT_MAN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_MULT_TX_MULT_MAN_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_mult_tx_mult_man_default(&self) -> bool {
        *self == TX_MULT_TX_MULT_MAN_A::TX_MULT_TX_MULT_MAN_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_MULT_TX_MULT_MAN`"]
pub struct TX_MULT_TX_MULT_MAN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MULT_TX_MULT_MAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_MULT_TX_MULT_MAN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_mult_tx_mult_man_default(self) -> &'a mut W {
        self.variant(TX_MULT_TX_MULT_MAN_A::TX_MULT_TX_MULT_MAN_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_FRAC_CONF_TX_FRAC_DEN_A {
    #[doc = "0: `0`"]
    TX_FRAC_CONF_TX_FRAC_DEN_DEFAULT = 0,
}
impl From<TX_FRAC_CONF_TX_FRAC_DEN_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_FRAC_CONF_TX_FRAC_DEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_FRAC_CONF_TX_FRAC_DEN`"]
pub type TX_FRAC_CONF_TX_FRAC_DEN_R = crate::R<u8, TX_FRAC_CONF_TX_FRAC_DEN_A>;
impl TX_FRAC_CONF_TX_FRAC_DEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_FRAC_CONF_TX_FRAC_DEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_FRAC_CONF_TX_FRAC_DEN_A::TX_FRAC_CONF_TX_FRAC_DEN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_FRAC_CONF_TX_FRAC_DEN_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_frac_conf_tx_frac_den_default(&self) -> bool {
        *self == TX_FRAC_CONF_TX_FRAC_DEN_A::TX_FRAC_CONF_TX_FRAC_DEN_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_FRAC_CONF_TX_FRAC_DEN`"]
pub struct TX_FRAC_CONF_TX_FRAC_DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FRAC_CONF_TX_FRAC_DEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_FRAC_CONF_TX_FRAC_DEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_frac_conf_tx_frac_den_default(self) -> &'a mut W {
        self.variant(TX_FRAC_CONF_TX_FRAC_DEN_A::TX_FRAC_CONF_TX_FRAC_DEN_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_FRAC_CONF_TX_FRAC_NUM_A {
    #[doc = "0: `0`"]
    TX_FRAC_CONF_TX_FRAC_NUM_DEFAULT = 0,
}
impl From<TX_FRAC_CONF_TX_FRAC_NUM_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_FRAC_CONF_TX_FRAC_NUM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_FRAC_CONF_TX_FRAC_NUM`"]
pub type TX_FRAC_CONF_TX_FRAC_NUM_R = crate::R<u8, TX_FRAC_CONF_TX_FRAC_NUM_A>;
impl TX_FRAC_CONF_TX_FRAC_NUM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_FRAC_CONF_TX_FRAC_NUM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_FRAC_CONF_TX_FRAC_NUM_A::TX_FRAC_CONF_TX_FRAC_NUM_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_FRAC_CONF_TX_FRAC_NUM_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_frac_conf_tx_frac_num_default(&self) -> bool {
        *self == TX_FRAC_CONF_TX_FRAC_NUM_A::TX_FRAC_CONF_TX_FRAC_NUM_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_FRAC_CONF_TX_FRAC_NUM`"]
pub struct TX_FRAC_CONF_TX_FRAC_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FRAC_CONF_TX_FRAC_NUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_FRAC_CONF_TX_FRAC_NUM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_frac_conf_tx_frac_num_default(self) -> &'a mut W {
        self.variant(TX_FRAC_CONF_TX_FRAC_NUM_A::TX_FRAC_CONF_TX_FRAC_NUM_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31 - Gain of the phase resampling block"]
    #[inline(always)]
    pub fn frontend2_resample_ph_gain(&self) -> FRONTEND2_RESAMPLE_PH_GAIN_R {
        FRONTEND2_RESAMPLE_PH_GAIN_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bits 26:28 - Gain of the decimator in the RSSI resampling block"]
    #[inline(always)]
    pub fn frontend2_resample_rssi_g2(&self) -> FRONTEND2_RESAMPLE_RSSI_G2_R {
        FRONTEND2_RESAMPLE_RSSI_G2_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - Gain of the interpolator in the RSSI resampling block"]
    #[inline(always)]
    pub fn frontend2_resample_rssi_g1(&self) -> FRONTEND2_RESAMPLE_RSSI_G1_R {
        FRONTEND2_RESAMPLE_RSSI_G1_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 22 - If set to 1 enables the phADC deglitcher"]
    #[inline(always)]
    pub fn frontend_en_phadc_deglitch(&self) -> FRONTEND_EN_PHADC_DEGLITCH_R {
        FRONTEND_EN_PHADC_DEGLITCH_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - If set to 1 enables the RSSI resampling"]
    #[inline(always)]
    pub fn frontend_en_resample_rssi(&self) -> FRONTEND_EN_RESAMPLE_RSSI_R {
        FRONTEND_EN_RESAMPLE_RSSI_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - If set to 1 enables the phase resampling"]
    #[inline(always)]
    pub fn frontend_en_resample_phadc(&self) -> FRONTEND_EN_RESAMPLE_PHADC_R {
        FRONTEND_EN_RESAMPLE_PHADC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Unsigned value that specifies the divider to obtain the phADC clock (and RSSI)."]
    #[inline(always)]
    pub fn frontend_div_phadc(&self) -> FRONTEND_DIV_PHADC_R {
        FRONTEND_DIV_PHADC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Exponent of the Tx multiplier"]
    #[inline(always)]
    pub fn tx_mult_tx_mult_exp(&self) -> TX_MULT_TX_MULT_EXP_R {
        TX_MULT_TX_MULT_EXP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Mantissa of the Tx multiplier"]
    #[inline(always)]
    pub fn tx_mult_tx_mult_man(&self) -> TX_MULT_TX_MULT_MAN_R {
        TX_MULT_TX_MULT_MAN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn tx_frac_conf_tx_frac_den(&self) -> TX_FRAC_CONF_TX_FRAC_DEN_R {
        TX_FRAC_CONF_TX_FRAC_DEN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn tx_frac_conf_tx_frac_num(&self) -> TX_FRAC_CONF_TX_FRAC_NUM_R {
        TX_FRAC_CONF_TX_FRAC_NUM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 29:31 - Gain of the phase resampling block"]
    #[inline(always)]
    pub fn frontend2_resample_ph_gain(&mut self) -> FRONTEND2_RESAMPLE_PH_GAIN_W {
        FRONTEND2_RESAMPLE_PH_GAIN_W { w: self }
    }
    #[doc = "Bits 26:28 - Gain of the decimator in the RSSI resampling block"]
    #[inline(always)]
    pub fn frontend2_resample_rssi_g2(&mut self) -> FRONTEND2_RESAMPLE_RSSI_G2_W {
        FRONTEND2_RESAMPLE_RSSI_G2_W { w: self }
    }
    #[doc = "Bits 24:25 - Gain of the interpolator in the RSSI resampling block"]
    #[inline(always)]
    pub fn frontend2_resample_rssi_g1(&mut self) -> FRONTEND2_RESAMPLE_RSSI_G1_W {
        FRONTEND2_RESAMPLE_RSSI_G1_W { w: self }
    }
    #[doc = "Bit 22 - If set to 1 enables the phADC deglitcher"]
    #[inline(always)]
    pub fn frontend_en_phadc_deglitch(&mut self) -> FRONTEND_EN_PHADC_DEGLITCH_W {
        FRONTEND_EN_PHADC_DEGLITCH_W { w: self }
    }
    #[doc = "Bit 21 - If set to 1 enables the RSSI resampling"]
    #[inline(always)]
    pub fn frontend_en_resample_rssi(&mut self) -> FRONTEND_EN_RESAMPLE_RSSI_W {
        FRONTEND_EN_RESAMPLE_RSSI_W { w: self }
    }
    #[doc = "Bit 20 - If set to 1 enables the phase resampling"]
    #[inline(always)]
    pub fn frontend_en_resample_phadc(&mut self) -> FRONTEND_EN_RESAMPLE_PHADC_W {
        FRONTEND_EN_RESAMPLE_PHADC_W { w: self }
    }
    #[doc = "Bits 16:19 - Unsigned value that specifies the divider to obtain the phADC clock (and RSSI)."]
    #[inline(always)]
    pub fn frontend_div_phadc(&mut self) -> FRONTEND_DIV_PHADC_W {
        FRONTEND_DIV_PHADC_W { w: self }
    }
    #[doc = "Bits 12:15 - Exponent of the Tx multiplier"]
    #[inline(always)]
    pub fn tx_mult_tx_mult_exp(&mut self) -> TX_MULT_TX_MULT_EXP_W {
        TX_MULT_TX_MULT_EXP_W { w: self }
    }
    #[doc = "Bits 8:11 - Mantissa of the Tx multiplier"]
    #[inline(always)]
    pub fn tx_mult_tx_mult_man(&mut self) -> TX_MULT_TX_MULT_MAN_W {
        TX_MULT_TX_MULT_MAN_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn tx_frac_conf_tx_frac_den(&mut self) -> TX_FRAC_CONF_TX_FRAC_DEN_W {
        TX_FRAC_CONF_TX_FRAC_DEN_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn tx_frac_conf_tx_frac_num(&mut self) -> TX_FRAC_CONF_TX_FRAC_NUM_W {
        TX_FRAC_CONF_TX_FRAC_NUM_W { w: self }
    }
}
