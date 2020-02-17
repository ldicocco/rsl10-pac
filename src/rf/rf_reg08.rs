#[doc = "Reader of register RF_REG08"]
pub type R = crate::R<u32, super::RF_REG08>;
#[doc = "Writer for register RF_REG08"]
pub type W = crate::W<u32, super::RF_REG08>;
#[doc = "Register RF_REG08 `reset()`'s with value 0xff00_0000"]
impl crate::ResetValue for super::RF_REG08 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff00_0000
    }
}
#[doc = "The packet length in the fixed packet length mode. In the variable packet length mode, it specifies the maximal packet length defined by the standard. In case of error a packet_len_err is raised.\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PACKET_LENGTH_PACKET_LEN_A {
    #[doc = "255: `11111111`"]
    PACKET_LENGTH_PACKET_LEN_DEFAULT = 255,
}
impl From<PACKET_LENGTH_PACKET_LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: PACKET_LENGTH_PACKET_LEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PACKET_LENGTH_PACKET_LEN`"]
pub type PACKET_LENGTH_PACKET_LEN_R = crate::R<u8, PACKET_LENGTH_PACKET_LEN_A>;
impl PACKET_LENGTH_PACKET_LEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PACKET_LENGTH_PACKET_LEN_A> {
        use crate::Variant::*;
        match self.bits {
            255 => Val(PACKET_LENGTH_PACKET_LEN_A::PACKET_LENGTH_PACKET_LEN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PACKET_LENGTH_PACKET_LEN_DEFAULT`"]
    #[inline(always)]
    pub fn is_packet_length_packet_len_default(&self) -> bool {
        *self == PACKET_LENGTH_PACKET_LEN_A::PACKET_LENGTH_PACKET_LEN_DEFAULT
    }
}
#[doc = "Write proxy for field `PACKET_LENGTH_PACKET_LEN`"]
pub struct PACKET_LENGTH_PACKET_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_LENGTH_PACKET_LEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACKET_LENGTH_PACKET_LEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`11111111`"]
    #[inline(always)]
    pub fn packet_length_packet_len_default(self) -> &'a mut W {
        self.variant(PACKET_LENGTH_PACKET_LEN_A::PACKET_LENGTH_PACKET_LEN_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "If set to 1, the LSB is the first bit to be sent, the MSB otherwise\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACKET_HANDLING_LSB_FIRST_A {
    #[doc = "0: `0`"]
    PACKET_HANDLING_LSB_FIRST_DEFAULT = 0,
}
impl From<PACKET_HANDLING_LSB_FIRST_A> for bool {
    #[inline(always)]
    fn from(variant: PACKET_HANDLING_LSB_FIRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PACKET_HANDLING_LSB_FIRST`"]
pub type PACKET_HANDLING_LSB_FIRST_R = crate::R<bool, PACKET_HANDLING_LSB_FIRST_A>;
impl PACKET_HANDLING_LSB_FIRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PACKET_HANDLING_LSB_FIRST_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PACKET_HANDLING_LSB_FIRST_A::PACKET_HANDLING_LSB_FIRST_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PACKET_HANDLING_LSB_FIRST_DEFAULT`"]
    #[inline(always)]
    pub fn is_packet_handling_lsb_first_default(&self) -> bool {
        *self == PACKET_HANDLING_LSB_FIRST_A::PACKET_HANDLING_LSB_FIRST_DEFAULT
    }
}
#[doc = "Write proxy for field `PACKET_HANDLING_LSB_FIRST`"]
pub struct PACKET_HANDLING_LSB_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_HANDLING_LSB_FIRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACKET_HANDLING_LSB_FIRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn packet_handling_lsb_first_default(self) -> &'a mut W {
        self.variant(PACKET_HANDLING_LSB_FIRST_A::PACKET_HANDLING_LSB_FIRST_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "If set to 1, enables the automatic CRC evaluation and insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACKET_HANDLING_EN_CRC_A {
    #[doc = "0: `0`"]
    PACKET_HANDLING_EN_CRC_DEFAULT = 0,
}
impl From<PACKET_HANDLING_EN_CRC_A> for bool {
    #[inline(always)]
    fn from(variant: PACKET_HANDLING_EN_CRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PACKET_HANDLING_EN_CRC`"]
pub type PACKET_HANDLING_EN_CRC_R = crate::R<bool, PACKET_HANDLING_EN_CRC_A>;
impl PACKET_HANDLING_EN_CRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PACKET_HANDLING_EN_CRC_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PACKET_HANDLING_EN_CRC_A::PACKET_HANDLING_EN_CRC_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PACKET_HANDLING_EN_CRC_DEFAULT`"]
    #[inline(always)]
    pub fn is_packet_handling_en_crc_default(&self) -> bool {
        *self == PACKET_HANDLING_EN_CRC_A::PACKET_HANDLING_EN_CRC_DEFAULT
    }
}
#[doc = "Write proxy for field `PACKET_HANDLING_EN_CRC`"]
pub struct PACKET_HANDLING_EN_CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_HANDLING_EN_CRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACKET_HANDLING_EN_CRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn packet_handling_en_crc_default(self) -> &'a mut W {
        self.variant(PACKET_HANDLING_EN_CRC_A::PACKET_HANDLING_EN_CRC_DEFAULT)
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
#[doc = "If set to 1, enables the CRC calculation on the packet length part of the packet.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACKET_HANDLING_EN_CRC_ON_PKTLEN_A {
    #[doc = "0: `0`"]
    PACKET_HANDLING_EN_CRC_ON_PKTLEN_DEFAULT = 0,
}
impl From<PACKET_HANDLING_EN_CRC_ON_PKTLEN_A> for bool {
    #[inline(always)]
    fn from(variant: PACKET_HANDLING_EN_CRC_ON_PKTLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PACKET_HANDLING_EN_CRC_ON_PKTLEN`"]
pub type PACKET_HANDLING_EN_CRC_ON_PKTLEN_R = crate::R<bool, PACKET_HANDLING_EN_CRC_ON_PKTLEN_A>;
impl PACKET_HANDLING_EN_CRC_ON_PKTLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PACKET_HANDLING_EN_CRC_ON_PKTLEN_A> {
        use crate::Variant::*;
        match self.bits {
            false => {
                Val(PACKET_HANDLING_EN_CRC_ON_PKTLEN_A::PACKET_HANDLING_EN_CRC_ON_PKTLEN_DEFAULT)
            }
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PACKET_HANDLING_EN_CRC_ON_PKTLEN_DEFAULT`"]
    #[inline(always)]
    pub fn is_packet_handling_en_crc_on_pktlen_default(&self) -> bool {
        *self == PACKET_HANDLING_EN_CRC_ON_PKTLEN_A::PACKET_HANDLING_EN_CRC_ON_PKTLEN_DEFAULT
    }
}
#[doc = "Write proxy for field `PACKET_HANDLING_EN_CRC_ON_PKTLEN`"]
pub struct PACKET_HANDLING_EN_CRC_ON_PKTLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_HANDLING_EN_CRC_ON_PKTLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACKET_HANDLING_EN_CRC_ON_PKTLEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn packet_handling_en_crc_on_pktlen_default(self) -> &'a mut W {
        self.variant(PACKET_HANDLING_EN_CRC_ON_PKTLEN_A::PACKET_HANDLING_EN_CRC_ON_PKTLEN_DEFAULT)
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
#[doc = "If set to 1, enables the automatic preamble insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACKET_HANDLING_EN_PREAMBLE_A {
    #[doc = "0: `0`"]
    PACKET_HANDLING_EN_PREAMBLE_DEFAULT = 0,
}
impl From<PACKET_HANDLING_EN_PREAMBLE_A> for bool {
    #[inline(always)]
    fn from(variant: PACKET_HANDLING_EN_PREAMBLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PACKET_HANDLING_EN_PREAMBLE`"]
pub type PACKET_HANDLING_EN_PREAMBLE_R = crate::R<bool, PACKET_HANDLING_EN_PREAMBLE_A>;
impl PACKET_HANDLING_EN_PREAMBLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PACKET_HANDLING_EN_PREAMBLE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PACKET_HANDLING_EN_PREAMBLE_A::PACKET_HANDLING_EN_PREAMBLE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PACKET_HANDLING_EN_PREAMBLE_DEFAULT`"]
    #[inline(always)]
    pub fn is_packet_handling_en_preamble_default(&self) -> bool {
        *self == PACKET_HANDLING_EN_PREAMBLE_A::PACKET_HANDLING_EN_PREAMBLE_DEFAULT
    }
}
#[doc = "Write proxy for field `PACKET_HANDLING_EN_PREAMBLE`"]
pub struct PACKET_HANDLING_EN_PREAMBLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_HANDLING_EN_PREAMBLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACKET_HANDLING_EN_PREAMBLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn packet_handling_en_preamble_default(self) -> &'a mut W {
        self.variant(PACKET_HANDLING_EN_PREAMBLE_A::PACKET_HANDLING_EN_PREAMBLE_DEFAULT)
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
#[doc = "If set to 1, enables the multi-frame packet (preamble-pattern-data-CRC-data-CRC-...)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACKET_HANDLING_EN_MULTI_FRAME_A {
    #[doc = "0: `0`"]
    PACKET_HANDLING_EN_MULTI_FRAME_DEFAULT = 0,
}
impl From<PACKET_HANDLING_EN_MULTI_FRAME_A> for bool {
    #[inline(always)]
    fn from(variant: PACKET_HANDLING_EN_MULTI_FRAME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PACKET_HANDLING_EN_MULTI_FRAME`"]
pub type PACKET_HANDLING_EN_MULTI_FRAME_R = crate::R<bool, PACKET_HANDLING_EN_MULTI_FRAME_A>;
impl PACKET_HANDLING_EN_MULTI_FRAME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PACKET_HANDLING_EN_MULTI_FRAME_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PACKET_HANDLING_EN_MULTI_FRAME_A::PACKET_HANDLING_EN_MULTI_FRAME_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PACKET_HANDLING_EN_MULTI_FRAME_DEFAULT`"]
    #[inline(always)]
    pub fn is_packet_handling_en_multi_frame_default(&self) -> bool {
        *self == PACKET_HANDLING_EN_MULTI_FRAME_A::PACKET_HANDLING_EN_MULTI_FRAME_DEFAULT
    }
}
#[doc = "Write proxy for field `PACKET_HANDLING_EN_MULTI_FRAME`"]
pub struct PACKET_HANDLING_EN_MULTI_FRAME_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_HANDLING_EN_MULTI_FRAME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACKET_HANDLING_EN_MULTI_FRAME_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn packet_handling_en_multi_frame_default(self) -> &'a mut W {
        self.variant(PACKET_HANDLING_EN_MULTI_FRAME_A::PACKET_HANDLING_EN_MULTI_FRAME_DEFAULT)
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
#[doc = "Enables the data-whitening on the CRC (active low)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACKET_HANDLING_ENB_DW_ON_CRC_A {
    #[doc = "0: `0`"]
    PACKET_HANDLING_ENB_DW_ON_CRC_DEFAULT = 0,
}
impl From<PACKET_HANDLING_ENB_DW_ON_CRC_A> for bool {
    #[inline(always)]
    fn from(variant: PACKET_HANDLING_ENB_DW_ON_CRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PACKET_HANDLING_ENB_DW_ON_CRC`"]
pub type PACKET_HANDLING_ENB_DW_ON_CRC_R = crate::R<bool, PACKET_HANDLING_ENB_DW_ON_CRC_A>;
impl PACKET_HANDLING_ENB_DW_ON_CRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PACKET_HANDLING_ENB_DW_ON_CRC_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PACKET_HANDLING_ENB_DW_ON_CRC_A::PACKET_HANDLING_ENB_DW_ON_CRC_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PACKET_HANDLING_ENB_DW_ON_CRC_DEFAULT`"]
    #[inline(always)]
    pub fn is_packet_handling_enb_dw_on_crc_default(&self) -> bool {
        *self == PACKET_HANDLING_ENB_DW_ON_CRC_A::PACKET_HANDLING_ENB_DW_ON_CRC_DEFAULT
    }
}
#[doc = "Write proxy for field `PACKET_HANDLING_ENB_DW_ON_CRC`"]
pub struct PACKET_HANDLING_ENB_DW_ON_CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_HANDLING_ENB_DW_ON_CRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACKET_HANDLING_ENB_DW_ON_CRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn packet_handling_enb_dw_on_crc_default(self) -> &'a mut W {
        self.variant(PACKET_HANDLING_ENB_DW_ON_CRC_A::PACKET_HANDLING_ENB_DW_ON_CRC_DEFAULT)
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
#[doc = "If set to 1, enables the automatic pattern insertion and recognition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACKET_HANDLING_EN_PATTERN_A {
    #[doc = "0: `0`"]
    PACKET_HANDLING_EN_PATTERN_DEFAULT = 0,
}
impl From<PACKET_HANDLING_EN_PATTERN_A> for bool {
    #[inline(always)]
    fn from(variant: PACKET_HANDLING_EN_PATTERN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PACKET_HANDLING_EN_PATTERN`"]
pub type PACKET_HANDLING_EN_PATTERN_R = crate::R<bool, PACKET_HANDLING_EN_PATTERN_A>;
impl PACKET_HANDLING_EN_PATTERN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PACKET_HANDLING_EN_PATTERN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PACKET_HANDLING_EN_PATTERN_A::PACKET_HANDLING_EN_PATTERN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PACKET_HANDLING_EN_PATTERN_DEFAULT`"]
    #[inline(always)]
    pub fn is_packet_handling_en_pattern_default(&self) -> bool {
        *self == PACKET_HANDLING_EN_PATTERN_A::PACKET_HANDLING_EN_PATTERN_DEFAULT
    }
}
#[doc = "Write proxy for field `PACKET_HANDLING_EN_PATTERN`"]
pub struct PACKET_HANDLING_EN_PATTERN_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_HANDLING_EN_PATTERN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACKET_HANDLING_EN_PATTERN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn packet_handling_en_pattern_default(self) -> &'a mut W {
        self.variant(PACKET_HANDLING_EN_PATTERN_A::PACKET_HANDLING_EN_PATTERN_DEFAULT)
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
#[doc = "If set to 1 enables the packet handler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACKET_HANDLING_EN_PACKET_A {
    #[doc = "0: `0`"]
    PACKET_HANDLING_EN_PACKET_DEFAULT = 0,
}
impl From<PACKET_HANDLING_EN_PACKET_A> for bool {
    #[inline(always)]
    fn from(variant: PACKET_HANDLING_EN_PACKET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PACKET_HANDLING_EN_PACKET`"]
pub type PACKET_HANDLING_EN_PACKET_R = crate::R<bool, PACKET_HANDLING_EN_PACKET_A>;
impl PACKET_HANDLING_EN_PACKET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PACKET_HANDLING_EN_PACKET_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PACKET_HANDLING_EN_PACKET_A::PACKET_HANDLING_EN_PACKET_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PACKET_HANDLING_EN_PACKET_DEFAULT`"]
    #[inline(always)]
    pub fn is_packet_handling_en_packet_default(&self) -> bool {
        *self == PACKET_HANDLING_EN_PACKET_A::PACKET_HANDLING_EN_PACKET_DEFAULT
    }
}
#[doc = "Write proxy for field `PACKET_HANDLING_EN_PACKET`"]
pub struct PACKET_HANDLING_EN_PACKET_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_HANDLING_EN_PACKET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACKET_HANDLING_EN_PACKET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn packet_handling_en_packet_default(self) -> &'a mut W {
        self.variant(PACKET_HANDLING_EN_PACKET_A::PACKET_HANDLING_EN_PACKET_DEFAULT)
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
#[doc = "If set to 1 enables the data-whitening\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CODING_EN_DATAWHITE_A {
    #[doc = "0: `0`"]
    CODING_EN_DATAWHITE_DEFAULT = 0,
}
impl From<CODING_EN_DATAWHITE_A> for bool {
    #[inline(always)]
    fn from(variant: CODING_EN_DATAWHITE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CODING_EN_DATAWHITE`"]
pub type CODING_EN_DATAWHITE_R = crate::R<bool, CODING_EN_DATAWHITE_A>;
impl CODING_EN_DATAWHITE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CODING_EN_DATAWHITE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CODING_EN_DATAWHITE_A::CODING_EN_DATAWHITE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CODING_EN_DATAWHITE_DEFAULT`"]
    #[inline(always)]
    pub fn is_coding_en_datawhite_default(&self) -> bool {
        *self == CODING_EN_DATAWHITE_A::CODING_EN_DATAWHITE_DEFAULT
    }
}
#[doc = "Write proxy for field `CODING_EN_DATAWHITE`"]
pub struct CODING_EN_DATAWHITE_W<'a> {
    w: &'a mut W,
}
impl<'a> CODING_EN_DATAWHITE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CODING_EN_DATAWHITE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn coding_en_datawhite_default(self) -> &'a mut W {
        self.variant(CODING_EN_DATAWHITE_A::CODING_EN_DATAWHITE_DEFAULT)
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
#[doc = "If set to 1, the channel I is considered 'delayed' in case of a 2bit per symbol modulaton\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CODING_I_NQ_DELAYED_A {
    #[doc = "0: `0`"]
    CODING_I_NQ_DELAYED_DEFAULT = 0,
}
impl From<CODING_I_NQ_DELAYED_A> for bool {
    #[inline(always)]
    fn from(variant: CODING_I_NQ_DELAYED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CODING_I_NQ_DELAYED`"]
pub type CODING_I_NQ_DELAYED_R = crate::R<bool, CODING_I_NQ_DELAYED_A>;
impl CODING_I_NQ_DELAYED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CODING_I_NQ_DELAYED_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CODING_I_NQ_DELAYED_A::CODING_I_NQ_DELAYED_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CODING_I_NQ_DELAYED_DEFAULT`"]
    #[inline(always)]
    pub fn is_coding_i_nq_delayed_default(&self) -> bool {
        *self == CODING_I_NQ_DELAYED_A::CODING_I_NQ_DELAYED_DEFAULT
    }
}
#[doc = "Write proxy for field `CODING_I_NQ_DELAYED`"]
pub struct CODING_I_NQ_DELAYED_W<'a> {
    w: &'a mut W,
}
impl<'a> CODING_I_NQ_DELAYED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CODING_I_NQ_DELAYED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn coding_i_nq_delayed_default(self) -> &'a mut W {
        self.variant(CODING_I_NQ_DELAYED_A::CODING_I_NQ_DELAYED_DEFAULT)
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
#[doc = "If set to 1, an offset (delay) is introduced in one of the two channels (2 bits per symbol modulation).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CODING_OFFSET_A {
    #[doc = "0: `0`"]
    CODING_OFFSET_DEFAULT = 0,
}
impl From<CODING_OFFSET_A> for bool {
    #[inline(always)]
    fn from(variant: CODING_OFFSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CODING_OFFSET`"]
pub type CODING_OFFSET_R = crate::R<bool, CODING_OFFSET_A>;
impl CODING_OFFSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CODING_OFFSET_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CODING_OFFSET_A::CODING_OFFSET_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CODING_OFFSET_DEFAULT`"]
    #[inline(always)]
    pub fn is_coding_offset_default(&self) -> bool {
        *self == CODING_OFFSET_A::CODING_OFFSET_DEFAULT
    }
}
#[doc = "Write proxy for field `CODING_OFFSET`"]
pub struct CODING_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> CODING_OFFSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CODING_OFFSET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn coding_offset_default(self) -> &'a mut W {
        self.variant(CODING_OFFSET_A::CODING_OFFSET_DEFAULT)
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
#[doc = "If set to 1, it inverts the bit value (Tx and Rx)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CODING_BIT_INVERT_A {
    #[doc = "0: `0`"]
    CODING_BIT_INVERT_DEFAULT = 0,
}
impl From<CODING_BIT_INVERT_A> for bool {
    #[inline(always)]
    fn from(variant: CODING_BIT_INVERT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CODING_BIT_INVERT`"]
pub type CODING_BIT_INVERT_R = crate::R<bool, CODING_BIT_INVERT_A>;
impl CODING_BIT_INVERT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CODING_BIT_INVERT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CODING_BIT_INVERT_A::CODING_BIT_INVERT_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CODING_BIT_INVERT_DEFAULT`"]
    #[inline(always)]
    pub fn is_coding_bit_invert_default(&self) -> bool {
        *self == CODING_BIT_INVERT_A::CODING_BIT_INVERT_DEFAULT
    }
}
#[doc = "Write proxy for field `CODING_BIT_INVERT`"]
pub struct CODING_BIT_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> CODING_BIT_INVERT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CODING_BIT_INVERT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn coding_bit_invert_default(self) -> &'a mut W {
        self.variant(CODING_BIT_INVERT_A::CODING_BIT_INVERT_DEFAULT)
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
#[doc = "Determines the bit order in case of a 2 bits per symbol modulation: if set to 1 the first bit (bit 0, even) goes to the I path\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CODING_EVEN_BEFORE_ODD_A {
    #[doc = "0: `0`"]
    CODING_EVEN_BEFORE_ODD_DEFAULT = 0,
}
impl From<CODING_EVEN_BEFORE_ODD_A> for bool {
    #[inline(always)]
    fn from(variant: CODING_EVEN_BEFORE_ODD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CODING_EVEN_BEFORE_ODD`"]
pub type CODING_EVEN_BEFORE_ODD_R = crate::R<bool, CODING_EVEN_BEFORE_ODD_A>;
impl CODING_EVEN_BEFORE_ODD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CODING_EVEN_BEFORE_ODD_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CODING_EVEN_BEFORE_ODD_A::CODING_EVEN_BEFORE_ODD_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CODING_EVEN_BEFORE_ODD_DEFAULT`"]
    #[inline(always)]
    pub fn is_coding_even_before_odd_default(&self) -> bool {
        *self == CODING_EVEN_BEFORE_ODD_A::CODING_EVEN_BEFORE_ODD_DEFAULT
    }
}
#[doc = "Write proxy for field `CODING_EVEN_BEFORE_ODD`"]
pub struct CODING_EVEN_BEFORE_ODD_W<'a> {
    w: &'a mut W,
}
impl<'a> CODING_EVEN_BEFORE_ODD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CODING_EVEN_BEFORE_ODD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn coding_even_before_odd_default(self) -> &'a mut W {
        self.variant(CODING_EVEN_BEFORE_ODD_A::CODING_EVEN_BEFORE_ODD_DEFAULT)
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
#[doc = "If set to 1 enables the linear to frequency encoding needed in order to modulate an OQPSK as an MSK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CODING_EN_802154_L2F_A {
    #[doc = "0: `0`"]
    CODING_EN_802154_L2F_DEFAULT = 0,
}
impl From<CODING_EN_802154_L2F_A> for bool {
    #[inline(always)]
    fn from(variant: CODING_EN_802154_L2F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CODING_EN_802154_L2F`"]
pub type CODING_EN_802154_L2F_R = crate::R<bool, CODING_EN_802154_L2F_A>;
impl CODING_EN_802154_L2F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CODING_EN_802154_L2F_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CODING_EN_802154_L2F_A::CODING_EN_802154_L2F_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CODING_EN_802154_L2F_DEFAULT`"]
    #[inline(always)]
    pub fn is_coding_en_802154_l2f_default(&self) -> bool {
        *self == CODING_EN_802154_L2F_A::CODING_EN_802154_L2F_DEFAULT
    }
}
#[doc = "Write proxy for field `CODING_EN_802154_L2F`"]
pub struct CODING_EN_802154_L2F_W<'a> {
    w: &'a mut W,
}
impl<'a> CODING_EN_802154_L2F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CODING_EN_802154_L2F_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn coding_en_802154_l2f_default(self) -> &'a mut W {
        self.variant(CODING_EN_802154_L2F_A::CODING_EN_802154_L2F_DEFAULT)
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
#[doc = "If set to 1 enables the bit to chips encoding used in the IEEE 802.15.4 standard\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CODING_EN_802154_B2C_A {
    #[doc = "0: `0`"]
    CODING_EN_802154_B2C_DEFAULT = 0,
}
impl From<CODING_EN_802154_B2C_A> for bool {
    #[inline(always)]
    fn from(variant: CODING_EN_802154_B2C_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CODING_EN_802154_B2C`"]
pub type CODING_EN_802154_B2C_R = crate::R<bool, CODING_EN_802154_B2C_A>;
impl CODING_EN_802154_B2C_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CODING_EN_802154_B2C_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CODING_EN_802154_B2C_A::CODING_EN_802154_B2C_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CODING_EN_802154_B2C_DEFAULT`"]
    #[inline(always)]
    pub fn is_coding_en_802154_b2c_default(&self) -> bool {
        *self == CODING_EN_802154_B2C_A::CODING_EN_802154_B2C_DEFAULT
    }
}
#[doc = "Write proxy for field `CODING_EN_802154_B2C`"]
pub struct CODING_EN_802154_B2C_W<'a> {
    w: &'a mut W,
}
impl<'a> CODING_EN_802154_B2C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CODING_EN_802154_B2C_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn coding_en_802154_b2c_default(self) -> &'a mut W {
        self.variant(CODING_EN_802154_B2C_A::CODING_EN_802154_B2C_DEFAULT)
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
#[doc = "If set to 1 enables the Manchester encoding\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CODING_EN_MANCHESTER_A {
    #[doc = "0: `0`"]
    CODING_EN_MANCHESTER_DEFAULT = 0,
}
impl From<CODING_EN_MANCHESTER_A> for bool {
    #[inline(always)]
    fn from(variant: CODING_EN_MANCHESTER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CODING_EN_MANCHESTER`"]
pub type CODING_EN_MANCHESTER_R = crate::R<bool, CODING_EN_MANCHESTER_A>;
impl CODING_EN_MANCHESTER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CODING_EN_MANCHESTER_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CODING_EN_MANCHESTER_A::CODING_EN_MANCHESTER_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CODING_EN_MANCHESTER_DEFAULT`"]
    #[inline(always)]
    pub fn is_coding_en_manchester_default(&self) -> bool {
        *self == CODING_EN_MANCHESTER_A::CODING_EN_MANCHESTER_DEFAULT
    }
}
#[doc = "Write proxy for field `CODING_EN_MANCHESTER`"]
pub struct CODING_EN_MANCHESTER_W<'a> {
    w: &'a mut W,
}
impl<'a> CODING_EN_MANCHESTER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CODING_EN_MANCHESTER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn coding_en_manchester_default(self) -> &'a mut W {
        self.variant(CODING_EN_MANCHESTER_A::CODING_EN_MANCHESTER_DEFAULT)
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
#[doc = "If set to 1 enables the definition of channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNELS_2_EN_CHANNEL_SEL_A {
    #[doc = "0: `0`"]
    CHANNELS_2_EN_CHANNEL_SEL_DEFAULT = 0,
}
impl From<CHANNELS_2_EN_CHANNEL_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNELS_2_EN_CHANNEL_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHANNELS_2_EN_CHANNEL_SEL`"]
pub type CHANNELS_2_EN_CHANNEL_SEL_R = crate::R<bool, CHANNELS_2_EN_CHANNEL_SEL_A>;
impl CHANNELS_2_EN_CHANNEL_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CHANNELS_2_EN_CHANNEL_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CHANNELS_2_EN_CHANNEL_SEL_A::CHANNELS_2_EN_CHANNEL_SEL_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CHANNELS_2_EN_CHANNEL_SEL_DEFAULT`"]
    #[inline(always)]
    pub fn is_channels_2_en_channel_sel_default(&self) -> bool {
        *self == CHANNELS_2_EN_CHANNEL_SEL_A::CHANNELS_2_EN_CHANNEL_SEL_DEFAULT
    }
}
#[doc = "Write proxy for field `CHANNELS_2_EN_CHANNEL_SEL`"]
pub struct CHANNELS_2_EN_CHANNEL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNELS_2_EN_CHANNEL_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNELS_2_EN_CHANNEL_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn channels_2_en_channel_sel_default(self) -> &'a mut W {
        self.variant(CHANNELS_2_EN_CHANNEL_SEL_A::CHANNELS_2_EN_CHANNEL_SEL_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "channel spacing: the formula that determines this value is the same as for the central frequency. v=ch_sp/144e6*2^25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHANNELS_2_CHANNEL_SPACING_HI_A {
    #[doc = "0: `0`"]
    CHANNELS_2_CHANNEL_SPACING_HI_DEFAULT = 0,
}
impl From<CHANNELS_2_CHANNEL_SPACING_HI_A> for u8 {
    #[inline(always)]
    fn from(variant: CHANNELS_2_CHANNEL_SPACING_HI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHANNELS_2_CHANNEL_SPACING_HI`"]
pub type CHANNELS_2_CHANNEL_SPACING_HI_R = crate::R<u8, CHANNELS_2_CHANNEL_SPACING_HI_A>;
impl CHANNELS_2_CHANNEL_SPACING_HI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CHANNELS_2_CHANNEL_SPACING_HI_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CHANNELS_2_CHANNEL_SPACING_HI_A::CHANNELS_2_CHANNEL_SPACING_HI_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CHANNELS_2_CHANNEL_SPACING_HI_DEFAULT`"]
    #[inline(always)]
    pub fn is_channels_2_channel_spacing_hi_default(&self) -> bool {
        *self == CHANNELS_2_CHANNEL_SPACING_HI_A::CHANNELS_2_CHANNEL_SPACING_HI_DEFAULT
    }
}
#[doc = "Write proxy for field `CHANNELS_2_CHANNEL_SPACING_HI`"]
pub struct CHANNELS_2_CHANNEL_SPACING_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNELS_2_CHANNEL_SPACING_HI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNELS_2_CHANNEL_SPACING_HI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn channels_2_channel_spacing_hi_default(self) -> &'a mut W {
        self.variant(CHANNELS_2_CHANNEL_SPACING_HI_A::CHANNELS_2_CHANNEL_SPACING_HI_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - The packet length in the fixed packet length mode. In the variable packet length mode, it specifies the maximal packet length defined by the standard. In case of error a packet_len_err is raised."]
    #[inline(always)]
    pub fn packet_length_packet_len(&self) -> PACKET_LENGTH_PACKET_LEN_R {
        PACKET_LENGTH_PACKET_LEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 23 - If set to 1, the LSB is the first bit to be sent, the MSB otherwise"]
    #[inline(always)]
    pub fn packet_handling_lsb_first(&self) -> PACKET_HANDLING_LSB_FIRST_R {
        PACKET_HANDLING_LSB_FIRST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - If set to 1, enables the automatic CRC evaluation and insertion"]
    #[inline(always)]
    pub fn packet_handling_en_crc(&self) -> PACKET_HANDLING_EN_CRC_R {
        PACKET_HANDLING_EN_CRC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - If set to 1, enables the CRC calculation on the packet length part of the packet."]
    #[inline(always)]
    pub fn packet_handling_en_crc_on_pktlen(&self) -> PACKET_HANDLING_EN_CRC_ON_PKTLEN_R {
        PACKET_HANDLING_EN_CRC_ON_PKTLEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - If set to 1, enables the automatic preamble insertion"]
    #[inline(always)]
    pub fn packet_handling_en_preamble(&self) -> PACKET_HANDLING_EN_PREAMBLE_R {
        PACKET_HANDLING_EN_PREAMBLE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - If set to 1, enables the multi-frame packet (preamble-pattern-data-CRC-data-CRC-...)"]
    #[inline(always)]
    pub fn packet_handling_en_multi_frame(&self) -> PACKET_HANDLING_EN_MULTI_FRAME_R {
        PACKET_HANDLING_EN_MULTI_FRAME_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables the data-whitening on the CRC (active low)"]
    #[inline(always)]
    pub fn packet_handling_enb_dw_on_crc(&self) -> PACKET_HANDLING_ENB_DW_ON_CRC_R {
        PACKET_HANDLING_ENB_DW_ON_CRC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - If set to 1, enables the automatic pattern insertion and recognition"]
    #[inline(always)]
    pub fn packet_handling_en_pattern(&self) -> PACKET_HANDLING_EN_PATTERN_R {
        PACKET_HANDLING_EN_PATTERN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - If set to 1 enables the packet handler"]
    #[inline(always)]
    pub fn packet_handling_en_packet(&self) -> PACKET_HANDLING_EN_PACKET_R {
        PACKET_HANDLING_EN_PACKET_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - If set to 1 enables the data-whitening"]
    #[inline(always)]
    pub fn coding_en_datawhite(&self) -> CODING_EN_DATAWHITE_R {
        CODING_EN_DATAWHITE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - If set to 1, the channel I is considered 'delayed' in case of a 2bit per symbol modulaton"]
    #[inline(always)]
    pub fn coding_i_nq_delayed(&self) -> CODING_I_NQ_DELAYED_R {
        CODING_I_NQ_DELAYED_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - If set to 1, an offset (delay) is introduced in one of the two channels (2 bits per symbol modulation)."]
    #[inline(always)]
    pub fn coding_offset(&self) -> CODING_OFFSET_R {
        CODING_OFFSET_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - If set to 1, it inverts the bit value (Tx and Rx)"]
    #[inline(always)]
    pub fn coding_bit_invert(&self) -> CODING_BIT_INVERT_R {
        CODING_BIT_INVERT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Determines the bit order in case of a 2 bits per symbol modulation: if set to 1 the first bit (bit 0, even) goes to the I path"]
    #[inline(always)]
    pub fn coding_even_before_odd(&self) -> CODING_EVEN_BEFORE_ODD_R {
        CODING_EVEN_BEFORE_ODD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - If set to 1 enables the linear to frequency encoding needed in order to modulate an OQPSK as an MSK."]
    #[inline(always)]
    pub fn coding_en_802154_l2f(&self) -> CODING_EN_802154_L2F_R {
        CODING_EN_802154_L2F_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - If set to 1 enables the bit to chips encoding used in the IEEE 802.15.4 standard"]
    #[inline(always)]
    pub fn coding_en_802154_b2c(&self) -> CODING_EN_802154_B2C_R {
        CODING_EN_802154_B2C_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - If set to 1 enables the Manchester encoding"]
    #[inline(always)]
    pub fn coding_en_manchester(&self) -> CODING_EN_MANCHESTER_R {
        CODING_EN_MANCHESTER_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - If set to 1 enables the definition of channels"]
    #[inline(always)]
    pub fn channels_2_en_channel_sel(&self) -> CHANNELS_2_EN_CHANNEL_SEL_R {
        CHANNELS_2_EN_CHANNEL_SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - channel spacing: the formula that determines this value is the same as for the central frequency. v=ch_sp/144e6*2^25"]
    #[inline(always)]
    pub fn channels_2_channel_spacing_hi(&self) -> CHANNELS_2_CHANNEL_SPACING_HI_R {
        CHANNELS_2_CHANNEL_SPACING_HI_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - The packet length in the fixed packet length mode. In the variable packet length mode, it specifies the maximal packet length defined by the standard. In case of error a packet_len_err is raised."]
    #[inline(always)]
    pub fn packet_length_packet_len(&mut self) -> PACKET_LENGTH_PACKET_LEN_W {
        PACKET_LENGTH_PACKET_LEN_W { w: self }
    }
    #[doc = "Bit 23 - If set to 1, the LSB is the first bit to be sent, the MSB otherwise"]
    #[inline(always)]
    pub fn packet_handling_lsb_first(&mut self) -> PACKET_HANDLING_LSB_FIRST_W {
        PACKET_HANDLING_LSB_FIRST_W { w: self }
    }
    #[doc = "Bit 22 - If set to 1, enables the automatic CRC evaluation and insertion"]
    #[inline(always)]
    pub fn packet_handling_en_crc(&mut self) -> PACKET_HANDLING_EN_CRC_W {
        PACKET_HANDLING_EN_CRC_W { w: self }
    }
    #[doc = "Bit 21 - If set to 1, enables the CRC calculation on the packet length part of the packet."]
    #[inline(always)]
    pub fn packet_handling_en_crc_on_pktlen(&mut self) -> PACKET_HANDLING_EN_CRC_ON_PKTLEN_W {
        PACKET_HANDLING_EN_CRC_ON_PKTLEN_W { w: self }
    }
    #[doc = "Bit 20 - If set to 1, enables the automatic preamble insertion"]
    #[inline(always)]
    pub fn packet_handling_en_preamble(&mut self) -> PACKET_HANDLING_EN_PREAMBLE_W {
        PACKET_HANDLING_EN_PREAMBLE_W { w: self }
    }
    #[doc = "Bit 19 - If set to 1, enables the multi-frame packet (preamble-pattern-data-CRC-data-CRC-...)"]
    #[inline(always)]
    pub fn packet_handling_en_multi_frame(&mut self) -> PACKET_HANDLING_EN_MULTI_FRAME_W {
        PACKET_HANDLING_EN_MULTI_FRAME_W { w: self }
    }
    #[doc = "Bit 18 - Enables the data-whitening on the CRC (active low)"]
    #[inline(always)]
    pub fn packet_handling_enb_dw_on_crc(&mut self) -> PACKET_HANDLING_ENB_DW_ON_CRC_W {
        PACKET_HANDLING_ENB_DW_ON_CRC_W { w: self }
    }
    #[doc = "Bit 17 - If set to 1, enables the automatic pattern insertion and recognition"]
    #[inline(always)]
    pub fn packet_handling_en_pattern(&mut self) -> PACKET_HANDLING_EN_PATTERN_W {
        PACKET_HANDLING_EN_PATTERN_W { w: self }
    }
    #[doc = "Bit 16 - If set to 1 enables the packet handler"]
    #[inline(always)]
    pub fn packet_handling_en_packet(&mut self) -> PACKET_HANDLING_EN_PACKET_W {
        PACKET_HANDLING_EN_PACKET_W { w: self }
    }
    #[doc = "Bit 15 - If set to 1 enables the data-whitening"]
    #[inline(always)]
    pub fn coding_en_datawhite(&mut self) -> CODING_EN_DATAWHITE_W {
        CODING_EN_DATAWHITE_W { w: self }
    }
    #[doc = "Bit 14 - If set to 1, the channel I is considered 'delayed' in case of a 2bit per symbol modulaton"]
    #[inline(always)]
    pub fn coding_i_nq_delayed(&mut self) -> CODING_I_NQ_DELAYED_W {
        CODING_I_NQ_DELAYED_W { w: self }
    }
    #[doc = "Bit 13 - If set to 1, an offset (delay) is introduced in one of the two channels (2 bits per symbol modulation)."]
    #[inline(always)]
    pub fn coding_offset(&mut self) -> CODING_OFFSET_W {
        CODING_OFFSET_W { w: self }
    }
    #[doc = "Bit 12 - If set to 1, it inverts the bit value (Tx and Rx)"]
    #[inline(always)]
    pub fn coding_bit_invert(&mut self) -> CODING_BIT_INVERT_W {
        CODING_BIT_INVERT_W { w: self }
    }
    #[doc = "Bit 11 - Determines the bit order in case of a 2 bits per symbol modulation: if set to 1 the first bit (bit 0, even) goes to the I path"]
    #[inline(always)]
    pub fn coding_even_before_odd(&mut self) -> CODING_EVEN_BEFORE_ODD_W {
        CODING_EVEN_BEFORE_ODD_W { w: self }
    }
    #[doc = "Bit 10 - If set to 1 enables the linear to frequency encoding needed in order to modulate an OQPSK as an MSK."]
    #[inline(always)]
    pub fn coding_en_802154_l2f(&mut self) -> CODING_EN_802154_L2F_W {
        CODING_EN_802154_L2F_W { w: self }
    }
    #[doc = "Bit 9 - If set to 1 enables the bit to chips encoding used in the IEEE 802.15.4 standard"]
    #[inline(always)]
    pub fn coding_en_802154_b2c(&mut self) -> CODING_EN_802154_B2C_W {
        CODING_EN_802154_B2C_W { w: self }
    }
    #[doc = "Bit 8 - If set to 1 enables the Manchester encoding"]
    #[inline(always)]
    pub fn coding_en_manchester(&mut self) -> CODING_EN_MANCHESTER_W {
        CODING_EN_MANCHESTER_W { w: self }
    }
    #[doc = "Bit 7 - If set to 1 enables the definition of channels"]
    #[inline(always)]
    pub fn channels_2_en_channel_sel(&mut self) -> CHANNELS_2_EN_CHANNEL_SEL_W {
        CHANNELS_2_EN_CHANNEL_SEL_W { w: self }
    }
    #[doc = "Bits 0:3 - channel spacing: the formula that determines this value is the same as for the central frequency. v=ch_sp/144e6*2^25"]
    #[inline(always)]
    pub fn channels_2_channel_spacing_hi(&mut self) -> CHANNELS_2_CHANNEL_SPACING_HI_W {
        CHANNELS_2_CHANNEL_SPACING_HI_W { w: self }
    }
}
