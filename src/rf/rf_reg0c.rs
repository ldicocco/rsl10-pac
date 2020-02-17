#[doc = "Reader of register RF_REG0C"]
pub type R = crate::R<u32, super::RF_REG0C>;
#[doc = "Writer for register RF_REG0C"]
pub type W = crate::W<u32, super::RF_REG0C>;
#[doc = "Register RF_REG0C `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG0C {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "polynom of the third convolutional code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CONV_CODES_POLY_CC_POLY_2_A {
    #[doc = "0: `0`"]
    CONV_CODES_POLY_CC_POLY_2_DEFAULT = 0,
}
impl From<CONV_CODES_POLY_CC_POLY_2_A> for u8 {
    #[inline(always)]
    fn from(variant: CONV_CODES_POLY_CC_POLY_2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CONV_CODES_POLY_CC_POLY_2`"]
pub type CONV_CODES_POLY_CC_POLY_2_R = crate::R<u8, CONV_CODES_POLY_CC_POLY_2_A>;
impl CONV_CODES_POLY_CC_POLY_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CONV_CODES_POLY_CC_POLY_2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CONV_CODES_POLY_CC_POLY_2_A::CONV_CODES_POLY_CC_POLY_2_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONV_CODES_POLY_CC_POLY_2_DEFAULT`"]
    #[inline(always)]
    pub fn is_conv_codes_poly_cc_poly_2_default(&self) -> bool {
        *self == CONV_CODES_POLY_CC_POLY_2_A::CONV_CODES_POLY_CC_POLY_2_DEFAULT
    }
}
#[doc = "Write proxy for field `CONV_CODES_POLY_CC_POLY_2`"]
pub struct CONV_CODES_POLY_CC_POLY_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CONV_CODES_POLY_CC_POLY_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONV_CODES_POLY_CC_POLY_2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn conv_codes_poly_cc_poly_2_default(self) -> &'a mut W {
        self.variant(CONV_CODES_POLY_CC_POLY_2_A::CONV_CODES_POLY_CC_POLY_2_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | (((value as u32) & 0x1f) << 26);
        self.w
    }
}
#[doc = "polynom of the second convolutional code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CONV_CODES_POLY_CC_POLY_1_A {
    #[doc = "0: `0`"]
    CONV_CODES_POLY_CC_POLY_1_DEFAULT = 0,
}
impl From<CONV_CODES_POLY_CC_POLY_1_A> for u8 {
    #[inline(always)]
    fn from(variant: CONV_CODES_POLY_CC_POLY_1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CONV_CODES_POLY_CC_POLY_1`"]
pub type CONV_CODES_POLY_CC_POLY_1_R = crate::R<u8, CONV_CODES_POLY_CC_POLY_1_A>;
impl CONV_CODES_POLY_CC_POLY_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CONV_CODES_POLY_CC_POLY_1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CONV_CODES_POLY_CC_POLY_1_A::CONV_CODES_POLY_CC_POLY_1_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONV_CODES_POLY_CC_POLY_1_DEFAULT`"]
    #[inline(always)]
    pub fn is_conv_codes_poly_cc_poly_1_default(&self) -> bool {
        *self == CONV_CODES_POLY_CC_POLY_1_A::CONV_CODES_POLY_CC_POLY_1_DEFAULT
    }
}
#[doc = "Write proxy for field `CONV_CODES_POLY_CC_POLY_1`"]
pub struct CONV_CODES_POLY_CC_POLY_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CONV_CODES_POLY_CC_POLY_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONV_CODES_POLY_CC_POLY_1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn conv_codes_poly_cc_poly_1_default(self) -> &'a mut W {
        self.variant(CONV_CODES_POLY_CC_POLY_1_A::CONV_CODES_POLY_CC_POLY_1_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | (((value as u32) & 0x1f) << 21);
        self.w
    }
}
#[doc = "polynom of the first convolutional code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CONV_CODES_POLY_CC_POLY_0_A {
    #[doc = "0: `0`"]
    CONV_CODES_POLY_CC_POLY_0_DEFAULT = 0,
}
impl From<CONV_CODES_POLY_CC_POLY_0_A> for u8 {
    #[inline(always)]
    fn from(variant: CONV_CODES_POLY_CC_POLY_0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CONV_CODES_POLY_CC_POLY_0`"]
pub type CONV_CODES_POLY_CC_POLY_0_R = crate::R<u8, CONV_CODES_POLY_CC_POLY_0_A>;
impl CONV_CODES_POLY_CC_POLY_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CONV_CODES_POLY_CC_POLY_0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CONV_CODES_POLY_CC_POLY_0_A::CONV_CODES_POLY_CC_POLY_0_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONV_CODES_POLY_CC_POLY_0_DEFAULT`"]
    #[inline(always)]
    pub fn is_conv_codes_poly_cc_poly_0_default(&self) -> bool {
        *self == CONV_CODES_POLY_CC_POLY_0_A::CONV_CODES_POLY_CC_POLY_0_DEFAULT
    }
}
#[doc = "Write proxy for field `CONV_CODES_POLY_CC_POLY_0`"]
pub struct CONV_CODES_POLY_CC_POLY_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CONV_CODES_POLY_CC_POLY_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONV_CODES_POLY_CC_POLY_0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn conv_codes_poly_cc_poly_0_default(self) -> &'a mut W {
        self.variant(CONV_CODES_POLY_CC_POLY_0_A::CONV_CODES_POLY_CC_POLY_0_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Set the memory length of the viterbi decoder: 00 => 5, 01 => 10, 10 => 20, 11 => 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CONV_CODES_CONF_CC_VITERBI_LEN_A {
    #[doc = "0: `0`"]
    CONV_CODES_CONF_CC_VITERBI_LEN_DEFAULT = 0,
}
impl From<CONV_CODES_CONF_CC_VITERBI_LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CONV_CODES_CONF_CC_VITERBI_LEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CONV_CODES_CONF_CC_VITERBI_LEN`"]
pub type CONV_CODES_CONF_CC_VITERBI_LEN_R = crate::R<u8, CONV_CODES_CONF_CC_VITERBI_LEN_A>;
impl CONV_CODES_CONF_CC_VITERBI_LEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CONV_CODES_CONF_CC_VITERBI_LEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CONV_CODES_CONF_CC_VITERBI_LEN_A::CONV_CODES_CONF_CC_VITERBI_LEN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONV_CODES_CONF_CC_VITERBI_LEN_DEFAULT`"]
    #[inline(always)]
    pub fn is_conv_codes_conf_cc_viterbi_len_default(&self) -> bool {
        *self == CONV_CODES_CONF_CC_VITERBI_LEN_A::CONV_CODES_CONF_CC_VITERBI_LEN_DEFAULT
    }
}
#[doc = "Write proxy for field `CONV_CODES_CONF_CC_VITERBI_LEN`"]
pub struct CONV_CODES_CONF_CC_VITERBI_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CONV_CODES_CONF_CC_VITERBI_LEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONV_CODES_CONF_CC_VITERBI_LEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn conv_codes_conf_cc_viterbi_len_default(self) -> &'a mut W {
        self.variant(CONV_CODES_CONF_CC_VITERBI_LEN_A::CONV_CODES_CONF_CC_VITERBI_LEN_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "if set to 1 enables the stop word at the end of the transmission. Necessary in order to keep a stream coherent with the convolutional coding\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONV_CODES_CONF_CC_EN_TX_STOP_A {
    #[doc = "0: `0`"]
    CONV_CODES_CONF_CC_EN_TX_STOP_DEFAULT = 0,
}
impl From<CONV_CODES_CONF_CC_EN_TX_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: CONV_CODES_CONF_CC_EN_TX_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CONV_CODES_CONF_CC_EN_TX_STOP`"]
pub type CONV_CODES_CONF_CC_EN_TX_STOP_R = crate::R<bool, CONV_CODES_CONF_CC_EN_TX_STOP_A>;
impl CONV_CODES_CONF_CC_EN_TX_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CONV_CODES_CONF_CC_EN_TX_STOP_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CONV_CODES_CONF_CC_EN_TX_STOP_A::CONV_CODES_CONF_CC_EN_TX_STOP_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONV_CODES_CONF_CC_EN_TX_STOP_DEFAULT`"]
    #[inline(always)]
    pub fn is_conv_codes_conf_cc_en_tx_stop_default(&self) -> bool {
        *self == CONV_CODES_CONF_CC_EN_TX_STOP_A::CONV_CODES_CONF_CC_EN_TX_STOP_DEFAULT
    }
}
#[doc = "Write proxy for field `CONV_CODES_CONF_CC_EN_TX_STOP`"]
pub struct CONV_CODES_CONF_CC_EN_TX_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CONV_CODES_CONF_CC_EN_TX_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONV_CODES_CONF_CC_EN_TX_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn conv_codes_conf_cc_en_tx_stop_default(self) -> &'a mut W {
        self.variant(CONV_CODES_CONF_CC_EN_TX_STOP_A::CONV_CODES_CONF_CC_EN_TX_STOP_DEFAULT)
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
#[doc = "If set to 1 enablse the convolutional codes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONV_CODES_CONF_EN_CONV_CODE_A {
    #[doc = "0: `0`"]
    CONV_CODES_CONF_EN_CONV_CODE_DEFAULT = 0,
}
impl From<CONV_CODES_CONF_EN_CONV_CODE_A> for bool {
    #[inline(always)]
    fn from(variant: CONV_CODES_CONF_EN_CONV_CODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CONV_CODES_CONF_EN_CONV_CODE`"]
pub type CONV_CODES_CONF_EN_CONV_CODE_R = crate::R<bool, CONV_CODES_CONF_EN_CONV_CODE_A>;
impl CONV_CODES_CONF_EN_CONV_CODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CONV_CODES_CONF_EN_CONV_CODE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CONV_CODES_CONF_EN_CONV_CODE_A::CONV_CODES_CONF_EN_CONV_CODE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONV_CODES_CONF_EN_CONV_CODE_DEFAULT`"]
    #[inline(always)]
    pub fn is_conv_codes_conf_en_conv_code_default(&self) -> bool {
        *self == CONV_CODES_CONF_EN_CONV_CODE_A::CONV_CODES_CONF_EN_CONV_CODE_DEFAULT
    }
}
#[doc = "Write proxy for field `CONV_CODES_CONF_EN_CONV_CODE`"]
pub struct CONV_CODES_CONF_EN_CONV_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CONV_CODES_CONF_EN_CONV_CODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONV_CODES_CONF_EN_CONV_CODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn conv_codes_conf_en_conv_code_default(self) -> &'a mut W {
        self.variant(CONV_CODES_CONF_EN_CONV_CODE_A::CONV_CODES_CONF_EN_CONV_CODE_DEFAULT)
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
#[doc = "length of the stop word, same as the pattern word length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PACKET_EXTRA_STOP_WORD_LEN_A {
    #[doc = "0: `0`"]
    PACKET_EXTRA_STOP_WORD_LEN_DEFAULT = 0,
}
impl From<PACKET_EXTRA_STOP_WORD_LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: PACKET_EXTRA_STOP_WORD_LEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PACKET_EXTRA_STOP_WORD_LEN`"]
pub type PACKET_EXTRA_STOP_WORD_LEN_R = crate::R<u8, PACKET_EXTRA_STOP_WORD_LEN_A>;
impl PACKET_EXTRA_STOP_WORD_LEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PACKET_EXTRA_STOP_WORD_LEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PACKET_EXTRA_STOP_WORD_LEN_A::PACKET_EXTRA_STOP_WORD_LEN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PACKET_EXTRA_STOP_WORD_LEN_DEFAULT`"]
    #[inline(always)]
    pub fn is_packet_extra_stop_word_len_default(&self) -> bool {
        *self == PACKET_EXTRA_STOP_WORD_LEN_A::PACKET_EXTRA_STOP_WORD_LEN_DEFAULT
    }
}
#[doc = "Write proxy for field `PACKET_EXTRA_STOP_WORD_LEN`"]
pub struct PACKET_EXTRA_STOP_WORD_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_EXTRA_STOP_WORD_LEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACKET_EXTRA_STOP_WORD_LEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn packet_extra_stop_word_len_default(self) -> &'a mut W {
        self.variant(PACKET_EXTRA_STOP_WORD_LEN_A::PACKET_EXTRA_STOP_WORD_LEN_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "If set to 1 adds the stop word (0x00) after the CRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACKET_EXTRA_EN_STOP_WORD_A {
    #[doc = "0: `0`"]
    PACKET_EXTRA_EN_STOP_WORD_DEFAULT = 0,
}
impl From<PACKET_EXTRA_EN_STOP_WORD_A> for bool {
    #[inline(always)]
    fn from(variant: PACKET_EXTRA_EN_STOP_WORD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PACKET_EXTRA_EN_STOP_WORD`"]
pub type PACKET_EXTRA_EN_STOP_WORD_R = crate::R<bool, PACKET_EXTRA_EN_STOP_WORD_A>;
impl PACKET_EXTRA_EN_STOP_WORD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PACKET_EXTRA_EN_STOP_WORD_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PACKET_EXTRA_EN_STOP_WORD_A::PACKET_EXTRA_EN_STOP_WORD_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PACKET_EXTRA_EN_STOP_WORD_DEFAULT`"]
    #[inline(always)]
    pub fn is_packet_extra_en_stop_word_default(&self) -> bool {
        *self == PACKET_EXTRA_EN_STOP_WORD_A::PACKET_EXTRA_EN_STOP_WORD_DEFAULT
    }
}
#[doc = "Write proxy for field `PACKET_EXTRA_EN_STOP_WORD`"]
pub struct PACKET_EXTRA_EN_STOP_WORD_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_EXTRA_EN_STOP_WORD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACKET_EXTRA_EN_STOP_WORD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn packet_extra_en_stop_word_default(self) -> &'a mut W {
        self.variant(PACKET_EXTRA_EN_STOP_WORD_A::PACKET_EXTRA_EN_STOP_WORD_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "If set to 1 the packet information are sampled at the end of the packet instead of the sync word detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACKET_EXTRA_PKT_INFO_PRE_NPOST_A {
    #[doc = "0: `0`"]
    PACKET_EXTRA_PKT_INFO_PRE_NPOST_DEFAULT = 0,
}
impl From<PACKET_EXTRA_PKT_INFO_PRE_NPOST_A> for bool {
    #[inline(always)]
    fn from(variant: PACKET_EXTRA_PKT_INFO_PRE_NPOST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PACKET_EXTRA_PKT_INFO_PRE_NPOST`"]
pub type PACKET_EXTRA_PKT_INFO_PRE_NPOST_R = crate::R<bool, PACKET_EXTRA_PKT_INFO_PRE_NPOST_A>;
impl PACKET_EXTRA_PKT_INFO_PRE_NPOST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PACKET_EXTRA_PKT_INFO_PRE_NPOST_A> {
        use crate::Variant::*;
        match self.bits {
            false => {
                Val(PACKET_EXTRA_PKT_INFO_PRE_NPOST_A::PACKET_EXTRA_PKT_INFO_PRE_NPOST_DEFAULT)
            }
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PACKET_EXTRA_PKT_INFO_PRE_NPOST_DEFAULT`"]
    #[inline(always)]
    pub fn is_packet_extra_pkt_info_pre_npost_default(&self) -> bool {
        *self == PACKET_EXTRA_PKT_INFO_PRE_NPOST_A::PACKET_EXTRA_PKT_INFO_PRE_NPOST_DEFAULT
    }
}
#[doc = "Write proxy for field `PACKET_EXTRA_PKT_INFO_PRE_NPOST`"]
pub struct PACKET_EXTRA_PKT_INFO_PRE_NPOST_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_EXTRA_PKT_INFO_PRE_NPOST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACKET_EXTRA_PKT_INFO_PRE_NPOST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn packet_extra_pkt_info_pre_npost_default(self) -> &'a mut W {
        self.variant(PACKET_EXTRA_PKT_INFO_PRE_NPOST_A::PACKET_EXTRA_PKT_INFO_PRE_NPOST_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "unsigned value that specifies the maximum number of errors in the pattern recognition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PACKET_EXTRA_PATTERN_MAX_ERR_A {
    #[doc = "0: `0`"]
    PACKET_EXTRA_PATTERN_MAX_ERR_DEFAULT = 0,
}
impl From<PACKET_EXTRA_PATTERN_MAX_ERR_A> for u8 {
    #[inline(always)]
    fn from(variant: PACKET_EXTRA_PATTERN_MAX_ERR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PACKET_EXTRA_PATTERN_MAX_ERR`"]
pub type PACKET_EXTRA_PATTERN_MAX_ERR_R = crate::R<u8, PACKET_EXTRA_PATTERN_MAX_ERR_A>;
impl PACKET_EXTRA_PATTERN_MAX_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PACKET_EXTRA_PATTERN_MAX_ERR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PACKET_EXTRA_PATTERN_MAX_ERR_A::PACKET_EXTRA_PATTERN_MAX_ERR_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PACKET_EXTRA_PATTERN_MAX_ERR_DEFAULT`"]
    #[inline(always)]
    pub fn is_packet_extra_pattern_max_err_default(&self) -> bool {
        *self == PACKET_EXTRA_PATTERN_MAX_ERR_A::PACKET_EXTRA_PATTERN_MAX_ERR_DEFAULT
    }
}
#[doc = "Write proxy for field `PACKET_EXTRA_PATTERN_MAX_ERR`"]
pub struct PACKET_EXTRA_PATTERN_MAX_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_EXTRA_PATTERN_MAX_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACKET_EXTRA_PATTERN_MAX_ERR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn packet_extra_pattern_max_err_default(self) -> &'a mut W {
        self.variant(PACKET_EXTRA_PATTERN_MAX_ERR_A::PACKET_EXTRA_PATTERN_MAX_ERR_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Pattern word length: 00 => 8bits, 01 => 16 bits, 10 => 24 bits, 11 => 32 bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PACKET_EXTRA_PATTERN_WORD_LEN_A {
    #[doc = "0: `0`"]
    PACKET_EXTRA_PATTERN_WORD_LEN_DEFAULT = 0,
}
impl From<PACKET_EXTRA_PATTERN_WORD_LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: PACKET_EXTRA_PATTERN_WORD_LEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PACKET_EXTRA_PATTERN_WORD_LEN`"]
pub type PACKET_EXTRA_PATTERN_WORD_LEN_R = crate::R<u8, PACKET_EXTRA_PATTERN_WORD_LEN_A>;
impl PACKET_EXTRA_PATTERN_WORD_LEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PACKET_EXTRA_PATTERN_WORD_LEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PACKET_EXTRA_PATTERN_WORD_LEN_A::PACKET_EXTRA_PATTERN_WORD_LEN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PACKET_EXTRA_PATTERN_WORD_LEN_DEFAULT`"]
    #[inline(always)]
    pub fn is_packet_extra_pattern_word_len_default(&self) -> bool {
        *self == PACKET_EXTRA_PATTERN_WORD_LEN_A::PACKET_EXTRA_PATTERN_WORD_LEN_DEFAULT
    }
}
#[doc = "Write proxy for field `PACKET_EXTRA_PATTERN_WORD_LEN`"]
pub struct PACKET_EXTRA_PATTERN_WORD_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_EXTRA_PATTERN_WORD_LEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACKET_EXTRA_PATTERN_WORD_LEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn packet_extra_pattern_word_len_default(self) -> &'a mut W {
        self.variant(PACKET_EXTRA_PATTERN_WORD_LEN_A::PACKET_EXTRA_PATTERN_WORD_LEN_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:30 - polynom of the third convolutional code"]
    #[inline(always)]
    pub fn conv_codes_poly_cc_poly_2(&self) -> CONV_CODES_POLY_CC_POLY_2_R {
        CONV_CODES_POLY_CC_POLY_2_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - polynom of the second convolutional code"]
    #[inline(always)]
    pub fn conv_codes_poly_cc_poly_1(&self) -> CONV_CODES_POLY_CC_POLY_1_R {
        CONV_CODES_POLY_CC_POLY_1_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - polynom of the first convolutional code"]
    #[inline(always)]
    pub fn conv_codes_poly_cc_poly_0(&self) -> CONV_CODES_POLY_CC_POLY_0_R {
        CONV_CODES_POLY_CC_POLY_0_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 10:11 - Set the memory length of the viterbi decoder: 00 => 5, 01 => 10, 10 => 20, 11 => 30"]
    #[inline(always)]
    pub fn conv_codes_conf_cc_viterbi_len(&self) -> CONV_CODES_CONF_CC_VITERBI_LEN_R {
        CONV_CODES_CONF_CC_VITERBI_LEN_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - if set to 1 enables the stop word at the end of the transmission. Necessary in order to keep a stream coherent with the convolutional coding"]
    #[inline(always)]
    pub fn conv_codes_conf_cc_en_tx_stop(&self) -> CONV_CODES_CONF_CC_EN_TX_STOP_R {
        CONV_CODES_CONF_CC_EN_TX_STOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - If set to 1 enablse the convolutional codes"]
    #[inline(always)]
    pub fn conv_codes_conf_en_conv_code(&self) -> CONV_CODES_CONF_EN_CONV_CODE_R {
        CONV_CODES_CONF_EN_CONV_CODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - length of the stop word, same as the pattern word length"]
    #[inline(always)]
    pub fn packet_extra_stop_word_len(&self) -> PACKET_EXTRA_STOP_WORD_LEN_R {
        PACKET_EXTRA_STOP_WORD_LEN_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - If set to 1 adds the stop word (0x00) after the CRC"]
    #[inline(always)]
    pub fn packet_extra_en_stop_word(&self) -> PACKET_EXTRA_EN_STOP_WORD_R {
        PACKET_EXTRA_EN_STOP_WORD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - If set to 1 the packet information are sampled at the end of the packet instead of the sync word detection."]
    #[inline(always)]
    pub fn packet_extra_pkt_info_pre_npost(&self) -> PACKET_EXTRA_PKT_INFO_PRE_NPOST_R {
        PACKET_EXTRA_PKT_INFO_PRE_NPOST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - unsigned value that specifies the maximum number of errors in the pattern recognition"]
    #[inline(always)]
    pub fn packet_extra_pattern_max_err(&self) -> PACKET_EXTRA_PATTERN_MAX_ERR_R {
        PACKET_EXTRA_PATTERN_MAX_ERR_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Pattern word length: 00 => 8bits, 01 => 16 bits, 10 => 24 bits, 11 => 32 bits"]
    #[inline(always)]
    pub fn packet_extra_pattern_word_len(&self) -> PACKET_EXTRA_PATTERN_WORD_LEN_R {
        PACKET_EXTRA_PATTERN_WORD_LEN_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 26:30 - polynom of the third convolutional code"]
    #[inline(always)]
    pub fn conv_codes_poly_cc_poly_2(&mut self) -> CONV_CODES_POLY_CC_POLY_2_W {
        CONV_CODES_POLY_CC_POLY_2_W { w: self }
    }
    #[doc = "Bits 21:25 - polynom of the second convolutional code"]
    #[inline(always)]
    pub fn conv_codes_poly_cc_poly_1(&mut self) -> CONV_CODES_POLY_CC_POLY_1_W {
        CONV_CODES_POLY_CC_POLY_1_W { w: self }
    }
    #[doc = "Bits 16:20 - polynom of the first convolutional code"]
    #[inline(always)]
    pub fn conv_codes_poly_cc_poly_0(&mut self) -> CONV_CODES_POLY_CC_POLY_0_W {
        CONV_CODES_POLY_CC_POLY_0_W { w: self }
    }
    #[doc = "Bits 10:11 - Set the memory length of the viterbi decoder: 00 => 5, 01 => 10, 10 => 20, 11 => 30"]
    #[inline(always)]
    pub fn conv_codes_conf_cc_viterbi_len(&mut self) -> CONV_CODES_CONF_CC_VITERBI_LEN_W {
        CONV_CODES_CONF_CC_VITERBI_LEN_W { w: self }
    }
    #[doc = "Bit 9 - if set to 1 enables the stop word at the end of the transmission. Necessary in order to keep a stream coherent with the convolutional coding"]
    #[inline(always)]
    pub fn conv_codes_conf_cc_en_tx_stop(&mut self) -> CONV_CODES_CONF_CC_EN_TX_STOP_W {
        CONV_CODES_CONF_CC_EN_TX_STOP_W { w: self }
    }
    #[doc = "Bit 8 - If set to 1 enablse the convolutional codes"]
    #[inline(always)]
    pub fn conv_codes_conf_en_conv_code(&mut self) -> CONV_CODES_CONF_EN_CONV_CODE_W {
        CONV_CODES_CONF_EN_CONV_CODE_W { w: self }
    }
    #[doc = "Bits 6:7 - length of the stop word, same as the pattern word length"]
    #[inline(always)]
    pub fn packet_extra_stop_word_len(&mut self) -> PACKET_EXTRA_STOP_WORD_LEN_W {
        PACKET_EXTRA_STOP_WORD_LEN_W { w: self }
    }
    #[doc = "Bit 5 - If set to 1 adds the stop word (0x00) after the CRC"]
    #[inline(always)]
    pub fn packet_extra_en_stop_word(&mut self) -> PACKET_EXTRA_EN_STOP_WORD_W {
        PACKET_EXTRA_EN_STOP_WORD_W { w: self }
    }
    #[doc = "Bit 4 - If set to 1 the packet information are sampled at the end of the packet instead of the sync word detection."]
    #[inline(always)]
    pub fn packet_extra_pkt_info_pre_npost(&mut self) -> PACKET_EXTRA_PKT_INFO_PRE_NPOST_W {
        PACKET_EXTRA_PKT_INFO_PRE_NPOST_W { w: self }
    }
    #[doc = "Bits 2:3 - unsigned value that specifies the maximum number of errors in the pattern recognition"]
    #[inline(always)]
    pub fn packet_extra_pattern_max_err(&mut self) -> PACKET_EXTRA_PATTERN_MAX_ERR_W {
        PACKET_EXTRA_PATTERN_MAX_ERR_W { w: self }
    }
    #[doc = "Bits 0:1 - Pattern word length: 00 => 8bits, 01 => 16 bits, 10 => 24 bits, 11 => 32 bits"]
    #[inline(always)]
    pub fn packet_extra_pattern_word_len(&mut self) -> PACKET_EXTRA_PATTERN_WORD_LEN_W {
        PACKET_EXTRA_PATTERN_WORD_LEN_W { w: self }
    }
}
