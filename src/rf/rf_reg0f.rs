#[doc = "Reader of register RF_REG0F"]
pub type R = crate::R<u32, super::RF_REG0F>;
#[doc = "Writer for register RF_REG0F"]
pub type W = crate::W<u32, super::RF_REG0F>;
#[doc = "Register RF_REG0F `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG0F {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_FRAC_CONF_RX_FRAC_DEN_A {
    #[doc = "0: `0`"]
    RX_FRAC_CONF_RX_FRAC_DEN_DEFAULT = 0,
}
impl From<RX_FRAC_CONF_RX_FRAC_DEN_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_FRAC_CONF_RX_FRAC_DEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RX_FRAC_CONF_RX_FRAC_DEN`"]
pub type RX_FRAC_CONF_RX_FRAC_DEN_R = crate::R<u8, RX_FRAC_CONF_RX_FRAC_DEN_A>;
impl RX_FRAC_CONF_RX_FRAC_DEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RX_FRAC_CONF_RX_FRAC_DEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RX_FRAC_CONF_RX_FRAC_DEN_A::RX_FRAC_CONF_RX_FRAC_DEN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX_FRAC_CONF_RX_FRAC_DEN_DEFAULT`"]
    #[inline(always)]
    pub fn is_rx_frac_conf_rx_frac_den_default(&self) -> bool {
        *self == RX_FRAC_CONF_RX_FRAC_DEN_A::RX_FRAC_CONF_RX_FRAC_DEN_DEFAULT
    }
}
#[doc = "Write proxy for field `RX_FRAC_CONF_RX_FRAC_DEN`"]
pub struct RX_FRAC_CONF_RX_FRAC_DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FRAC_CONF_RX_FRAC_DEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FRAC_CONF_RX_FRAC_DEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rx_frac_conf_rx_frac_den_default(self) -> &'a mut W {
        self.variant(RX_FRAC_CONF_RX_FRAC_DEN_A::RX_FRAC_CONF_RX_FRAC_DEN_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_FRAC_CONF_RX_FRAC_NUM_A {
    #[doc = "0: `0`"]
    RX_FRAC_CONF_RX_FRAC_NUM_DEFAULT = 0,
}
impl From<RX_FRAC_CONF_RX_FRAC_NUM_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_FRAC_CONF_RX_FRAC_NUM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RX_FRAC_CONF_RX_FRAC_NUM`"]
pub type RX_FRAC_CONF_RX_FRAC_NUM_R = crate::R<u8, RX_FRAC_CONF_RX_FRAC_NUM_A>;
impl RX_FRAC_CONF_RX_FRAC_NUM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RX_FRAC_CONF_RX_FRAC_NUM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RX_FRAC_CONF_RX_FRAC_NUM_A::RX_FRAC_CONF_RX_FRAC_NUM_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX_FRAC_CONF_RX_FRAC_NUM_DEFAULT`"]
    #[inline(always)]
    pub fn is_rx_frac_conf_rx_frac_num_default(&self) -> bool {
        *self == RX_FRAC_CONF_RX_FRAC_NUM_A::RX_FRAC_CONF_RX_FRAC_NUM_DEFAULT
    }
}
#[doc = "Write proxy for field `RX_FRAC_CONF_RX_FRAC_NUM`"]
pub struct RX_FRAC_CONF_RX_FRAC_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FRAC_CONF_RX_FRAC_NUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FRAC_CONF_RX_FRAC_NUM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rx_frac_conf_rx_frac_num_default(self) -> &'a mut W {
        self.variant(RX_FRAC_CONF_RX_FRAC_NUM_A::RX_FRAC_CONF_RX_FRAC_NUM_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAC_CONF_TX_FRAC_GAIN_A {
    #[doc = "0: `0`"]
    FRAC_CONF_TX_FRAC_GAIN_DEFAULT = 0,
}
impl From<FRAC_CONF_TX_FRAC_GAIN_A> for bool {
    #[inline(always)]
    fn from(variant: FRAC_CONF_TX_FRAC_GAIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRAC_CONF_TX_FRAC_GAIN`"]
pub type FRAC_CONF_TX_FRAC_GAIN_R = crate::R<bool, FRAC_CONF_TX_FRAC_GAIN_A>;
impl FRAC_CONF_TX_FRAC_GAIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FRAC_CONF_TX_FRAC_GAIN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(FRAC_CONF_TX_FRAC_GAIN_A::FRAC_CONF_TX_FRAC_GAIN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FRAC_CONF_TX_FRAC_GAIN_DEFAULT`"]
    #[inline(always)]
    pub fn is_frac_conf_tx_frac_gain_default(&self) -> bool {
        *self == FRAC_CONF_TX_FRAC_GAIN_A::FRAC_CONF_TX_FRAC_GAIN_DEFAULT
    }
}
#[doc = "Write proxy for field `FRAC_CONF_TX_FRAC_GAIN`"]
pub struct FRAC_CONF_TX_FRAC_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC_CONF_TX_FRAC_GAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAC_CONF_TX_FRAC_GAIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn frac_conf_tx_frac_gain_default(self) -> &'a mut W {
        self.variant(FRAC_CONF_TX_FRAC_GAIN_A::FRAC_CONF_TX_FRAC_GAIN_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAC_CONF_RX_FRAC_GAIN_A {
    #[doc = "0: `0`"]
    FRAC_CONF_RX_FRAC_GAIN_DEFAULT = 0,
}
impl From<FRAC_CONF_RX_FRAC_GAIN_A> for bool {
    #[inline(always)]
    fn from(variant: FRAC_CONF_RX_FRAC_GAIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRAC_CONF_RX_FRAC_GAIN`"]
pub type FRAC_CONF_RX_FRAC_GAIN_R = crate::R<bool, FRAC_CONF_RX_FRAC_GAIN_A>;
impl FRAC_CONF_RX_FRAC_GAIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FRAC_CONF_RX_FRAC_GAIN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(FRAC_CONF_RX_FRAC_GAIN_A::FRAC_CONF_RX_FRAC_GAIN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FRAC_CONF_RX_FRAC_GAIN_DEFAULT`"]
    #[inline(always)]
    pub fn is_frac_conf_rx_frac_gain_default(&self) -> bool {
        *self == FRAC_CONF_RX_FRAC_GAIN_A::FRAC_CONF_RX_FRAC_GAIN_DEFAULT
    }
}
#[doc = "Write proxy for field `FRAC_CONF_RX_FRAC_GAIN`"]
pub struct FRAC_CONF_RX_FRAC_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC_CONF_RX_FRAC_GAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAC_CONF_RX_FRAC_GAIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn frac_conf_rx_frac_gain_default(self) -> &'a mut W {
        self.variant(FRAC_CONF_RX_FRAC_GAIN_A::FRAC_CONF_RX_FRAC_GAIN_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAC_CONF_TX_EN_FRAC_A {
    #[doc = "0: `0`"]
    FRAC_CONF_TX_EN_FRAC_DEFAULT = 0,
}
impl From<FRAC_CONF_TX_EN_FRAC_A> for bool {
    #[inline(always)]
    fn from(variant: FRAC_CONF_TX_EN_FRAC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRAC_CONF_TX_EN_FRAC`"]
pub type FRAC_CONF_TX_EN_FRAC_R = crate::R<bool, FRAC_CONF_TX_EN_FRAC_A>;
impl FRAC_CONF_TX_EN_FRAC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FRAC_CONF_TX_EN_FRAC_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(FRAC_CONF_TX_EN_FRAC_A::FRAC_CONF_TX_EN_FRAC_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FRAC_CONF_TX_EN_FRAC_DEFAULT`"]
    #[inline(always)]
    pub fn is_frac_conf_tx_en_frac_default(&self) -> bool {
        *self == FRAC_CONF_TX_EN_FRAC_A::FRAC_CONF_TX_EN_FRAC_DEFAULT
    }
}
#[doc = "Write proxy for field `FRAC_CONF_TX_EN_FRAC`"]
pub struct FRAC_CONF_TX_EN_FRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC_CONF_TX_EN_FRAC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAC_CONF_TX_EN_FRAC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn frac_conf_tx_en_frac_default(self) -> &'a mut W {
        self.variant(FRAC_CONF_TX_EN_FRAC_A::FRAC_CONF_TX_EN_FRAC_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAC_CONF_RX_EN_FRAC_A {
    #[doc = "0: `0`"]
    FRAC_CONF_RX_EN_FRAC_DEFAULT = 0,
}
impl From<FRAC_CONF_RX_EN_FRAC_A> for bool {
    #[inline(always)]
    fn from(variant: FRAC_CONF_RX_EN_FRAC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRAC_CONF_RX_EN_FRAC`"]
pub type FRAC_CONF_RX_EN_FRAC_R = crate::R<bool, FRAC_CONF_RX_EN_FRAC_A>;
impl FRAC_CONF_RX_EN_FRAC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FRAC_CONF_RX_EN_FRAC_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(FRAC_CONF_RX_EN_FRAC_A::FRAC_CONF_RX_EN_FRAC_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FRAC_CONF_RX_EN_FRAC_DEFAULT`"]
    #[inline(always)]
    pub fn is_frac_conf_rx_en_frac_default(&self) -> bool {
        *self == FRAC_CONF_RX_EN_FRAC_A::FRAC_CONF_RX_EN_FRAC_DEFAULT
    }
}
#[doc = "Write proxy for field `FRAC_CONF_RX_EN_FRAC`"]
pub struct FRAC_CONF_RX_EN_FRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC_CONF_RX_EN_FRAC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAC_CONF_RX_EN_FRAC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn frac_conf_rx_en_frac_default(self) -> &'a mut W {
        self.variant(FRAC_CONF_RX_EN_FRAC_A::FRAC_CONF_RX_EN_FRAC_DEFAULT)
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
#[doc = "puncture of the third convolutional code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CONV_CODES_PUNCT_CC_PUNCT_2_A {
    #[doc = "0: `0`"]
    CONV_CODES_PUNCT_CC_PUNCT_2_DEFAULT = 0,
}
impl From<CONV_CODES_PUNCT_CC_PUNCT_2_A> for u8 {
    #[inline(always)]
    fn from(variant: CONV_CODES_PUNCT_CC_PUNCT_2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CONV_CODES_PUNCT_CC_PUNCT_2`"]
pub type CONV_CODES_PUNCT_CC_PUNCT_2_R = crate::R<u8, CONV_CODES_PUNCT_CC_PUNCT_2_A>;
impl CONV_CODES_PUNCT_CC_PUNCT_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CONV_CODES_PUNCT_CC_PUNCT_2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CONV_CODES_PUNCT_CC_PUNCT_2_A::CONV_CODES_PUNCT_CC_PUNCT_2_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONV_CODES_PUNCT_CC_PUNCT_2_DEFAULT`"]
    #[inline(always)]
    pub fn is_conv_codes_punct_cc_punct_2_default(&self) -> bool {
        *self == CONV_CODES_PUNCT_CC_PUNCT_2_A::CONV_CODES_PUNCT_CC_PUNCT_2_DEFAULT
    }
}
#[doc = "Write proxy for field `CONV_CODES_PUNCT_CC_PUNCT_2`"]
pub struct CONV_CODES_PUNCT_CC_PUNCT_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CONV_CODES_PUNCT_CC_PUNCT_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONV_CODES_PUNCT_CC_PUNCT_2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn conv_codes_punct_cc_punct_2_default(self) -> &'a mut W {
        self.variant(CONV_CODES_PUNCT_CC_PUNCT_2_A::CONV_CODES_PUNCT_CC_PUNCT_2_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "puncture of the second convolutional code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CONV_CODES_PUNCT_CC_PUNCT_1_A {
    #[doc = "0: `0`"]
    CONV_CODES_PUNCT_CC_PUNCT_1_DEFAULT = 0,
}
impl From<CONV_CODES_PUNCT_CC_PUNCT_1_A> for u8 {
    #[inline(always)]
    fn from(variant: CONV_CODES_PUNCT_CC_PUNCT_1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CONV_CODES_PUNCT_CC_PUNCT_1`"]
pub type CONV_CODES_PUNCT_CC_PUNCT_1_R = crate::R<u8, CONV_CODES_PUNCT_CC_PUNCT_1_A>;
impl CONV_CODES_PUNCT_CC_PUNCT_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CONV_CODES_PUNCT_CC_PUNCT_1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CONV_CODES_PUNCT_CC_PUNCT_1_A::CONV_CODES_PUNCT_CC_PUNCT_1_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONV_CODES_PUNCT_CC_PUNCT_1_DEFAULT`"]
    #[inline(always)]
    pub fn is_conv_codes_punct_cc_punct_1_default(&self) -> bool {
        *self == CONV_CODES_PUNCT_CC_PUNCT_1_A::CONV_CODES_PUNCT_CC_PUNCT_1_DEFAULT
    }
}
#[doc = "Write proxy for field `CONV_CODES_PUNCT_CC_PUNCT_1`"]
pub struct CONV_CODES_PUNCT_CC_PUNCT_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CONV_CODES_PUNCT_CC_PUNCT_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONV_CODES_PUNCT_CC_PUNCT_1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn conv_codes_punct_cc_punct_1_default(self) -> &'a mut W {
        self.variant(CONV_CODES_PUNCT_CC_PUNCT_1_A::CONV_CODES_PUNCT_CC_PUNCT_1_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "puncture of the first convolutional code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CONV_CODES_PUNCT_CC_PUNCT_0_A {
    #[doc = "0: `0`"]
    CONV_CODES_PUNCT_CC_PUNCT_0_DEFAULT = 0,
}
impl From<CONV_CODES_PUNCT_CC_PUNCT_0_A> for u8 {
    #[inline(always)]
    fn from(variant: CONV_CODES_PUNCT_CC_PUNCT_0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CONV_CODES_PUNCT_CC_PUNCT_0`"]
pub type CONV_CODES_PUNCT_CC_PUNCT_0_R = crate::R<u8, CONV_CODES_PUNCT_CC_PUNCT_0_A>;
impl CONV_CODES_PUNCT_CC_PUNCT_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CONV_CODES_PUNCT_CC_PUNCT_0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CONV_CODES_PUNCT_CC_PUNCT_0_A::CONV_CODES_PUNCT_CC_PUNCT_0_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONV_CODES_PUNCT_CC_PUNCT_0_DEFAULT`"]
    #[inline(always)]
    pub fn is_conv_codes_punct_cc_punct_0_default(&self) -> bool {
        *self == CONV_CODES_PUNCT_CC_PUNCT_0_A::CONV_CODES_PUNCT_CC_PUNCT_0_DEFAULT
    }
}
#[doc = "Write proxy for field `CONV_CODES_PUNCT_CC_PUNCT_0`"]
pub struct CONV_CODES_PUNCT_CC_PUNCT_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CONV_CODES_PUNCT_CC_PUNCT_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONV_CODES_PUNCT_CC_PUNCT_0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn conv_codes_punct_cc_punct_0_default(self) -> &'a mut W {
        self.variant(CONV_CODES_PUNCT_CC_PUNCT_0_A::CONV_CODES_PUNCT_CC_PUNCT_0_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn rx_frac_conf_rx_frac_den(&self) -> RX_FRAC_CONF_RX_FRAC_DEN_R {
        RX_FRAC_CONF_RX_FRAC_DEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn rx_frac_conf_rx_frac_num(&self) -> RX_FRAC_CONF_RX_FRAC_NUM_R {
        RX_FRAC_CONF_RX_FRAC_NUM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn frac_conf_tx_frac_gain(&self) -> FRAC_CONF_TX_FRAC_GAIN_R {
        FRAC_CONF_TX_FRAC_GAIN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn frac_conf_rx_frac_gain(&self) -> FRAC_CONF_RX_FRAC_GAIN_R {
        FRAC_CONF_RX_FRAC_GAIN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn frac_conf_tx_en_frac(&self) -> FRAC_CONF_TX_EN_FRAC_R {
        FRAC_CONF_TX_EN_FRAC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn frac_conf_rx_en_frac(&self) -> FRAC_CONF_RX_EN_FRAC_R {
        FRAC_CONF_RX_EN_FRAC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 10:14 - puncture of the third convolutional code"]
    #[inline(always)]
    pub fn conv_codes_punct_cc_punct_2(&self) -> CONV_CODES_PUNCT_CC_PUNCT_2_R {
        CONV_CODES_PUNCT_CC_PUNCT_2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - puncture of the second convolutional code"]
    #[inline(always)]
    pub fn conv_codes_punct_cc_punct_1(&self) -> CONV_CODES_PUNCT_CC_PUNCT_1_R {
        CONV_CODES_PUNCT_CC_PUNCT_1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - puncture of the first convolutional code"]
    #[inline(always)]
    pub fn conv_codes_punct_cc_punct_0(&self) -> CONV_CODES_PUNCT_CC_PUNCT_0_R {
        CONV_CODES_PUNCT_CC_PUNCT_0_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn rx_frac_conf_rx_frac_den(&mut self) -> RX_FRAC_CONF_RX_FRAC_DEN_W {
        RX_FRAC_CONF_RX_FRAC_DEN_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn rx_frac_conf_rx_frac_num(&mut self) -> RX_FRAC_CONF_RX_FRAC_NUM_W {
        RX_FRAC_CONF_RX_FRAC_NUM_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn frac_conf_tx_frac_gain(&mut self) -> FRAC_CONF_TX_FRAC_GAIN_W {
        FRAC_CONF_TX_FRAC_GAIN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn frac_conf_rx_frac_gain(&mut self) -> FRAC_CONF_RX_FRAC_GAIN_W {
        FRAC_CONF_RX_FRAC_GAIN_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn frac_conf_tx_en_frac(&mut self) -> FRAC_CONF_TX_EN_FRAC_W {
        FRAC_CONF_TX_EN_FRAC_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn frac_conf_rx_en_frac(&mut self) -> FRAC_CONF_RX_EN_FRAC_W {
        FRAC_CONF_RX_EN_FRAC_W { w: self }
    }
    #[doc = "Bits 10:14 - puncture of the third convolutional code"]
    #[inline(always)]
    pub fn conv_codes_punct_cc_punct_2(&mut self) -> CONV_CODES_PUNCT_CC_PUNCT_2_W {
        CONV_CODES_PUNCT_CC_PUNCT_2_W { w: self }
    }
    #[doc = "Bits 5:9 - puncture of the second convolutional code"]
    #[inline(always)]
    pub fn conv_codes_punct_cc_punct_1(&mut self) -> CONV_CODES_PUNCT_CC_PUNCT_1_W {
        CONV_CODES_PUNCT_CC_PUNCT_1_W { w: self }
    }
    #[doc = "Bits 0:4 - puncture of the first convolutional code"]
    #[inline(always)]
    pub fn conv_codes_punct_cc_punct_0(&mut self) -> CONV_CODES_PUNCT_CC_PUNCT_0_W {
        CONV_CODES_PUNCT_CC_PUNCT_0_W { w: self }
    }
}
