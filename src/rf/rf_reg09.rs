#[doc = "Reader of register RF_REG09"]
pub type R = crate::R<u32, super::RF_REG09>;
#[doc = "Writer for register RF_REG09"]
pub type W = crate::W<u32, super::RF_REG09>;
#[doc = "Register RF_REG09 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG09 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "If set to 1 the address length is 16 bits, 8 otherwise.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_CONF_ADDRESS_LEN_A {
    #[doc = "0: `0`"]
    ADDRESS_CONF_ADDRESS_LEN_DEFAULT = 0,
}
impl From<ADDRESS_CONF_ADDRESS_LEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_CONF_ADDRESS_LEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDRESS_CONF_ADDRESS_LEN`"]
pub type ADDRESS_CONF_ADDRESS_LEN_R = crate::R<bool, ADDRESS_CONF_ADDRESS_LEN_A>;
impl ADDRESS_CONF_ADDRESS_LEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ADDRESS_CONF_ADDRESS_LEN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(ADDRESS_CONF_ADDRESS_LEN_A::ADDRESS_CONF_ADDRESS_LEN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADDRESS_CONF_ADDRESS_LEN_DEFAULT`"]
    #[inline(always)]
    pub fn is_address_conf_address_len_default(&self) -> bool {
        *self == ADDRESS_CONF_ADDRESS_LEN_A::ADDRESS_CONF_ADDRESS_LEN_DEFAULT
    }
}
#[doc = "Write proxy for field `ADDRESS_CONF_ADDRESS_LEN`"]
pub struct ADDRESS_CONF_ADDRESS_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_CONF_ADDRESS_LEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRESS_CONF_ADDRESS_LEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn address_conf_address_len_default(self) -> &'a mut W {
        self.variant(ADDRESS_CONF_ADDRESS_LEN_A::ADDRESS_CONF_ADDRESS_LEN_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "If set to 1 enables the broadcast address detection on Rx.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_CONF_EN_ADDRESS_RX_BR_A {
    #[doc = "0: `0`"]
    ADDRESS_CONF_EN_ADDRESS_RX_BR_DEFAULT = 0,
}
impl From<ADDRESS_CONF_EN_ADDRESS_RX_BR_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_CONF_EN_ADDRESS_RX_BR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDRESS_CONF_EN_ADDRESS_RX_BR`"]
pub type ADDRESS_CONF_EN_ADDRESS_RX_BR_R = crate::R<bool, ADDRESS_CONF_EN_ADDRESS_RX_BR_A>;
impl ADDRESS_CONF_EN_ADDRESS_RX_BR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ADDRESS_CONF_EN_ADDRESS_RX_BR_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(ADDRESS_CONF_EN_ADDRESS_RX_BR_A::ADDRESS_CONF_EN_ADDRESS_RX_BR_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADDRESS_CONF_EN_ADDRESS_RX_BR_DEFAULT`"]
    #[inline(always)]
    pub fn is_address_conf_en_address_rx_br_default(&self) -> bool {
        *self == ADDRESS_CONF_EN_ADDRESS_RX_BR_A::ADDRESS_CONF_EN_ADDRESS_RX_BR_DEFAULT
    }
}
#[doc = "Write proxy for field `ADDRESS_CONF_EN_ADDRESS_RX_BR`"]
pub struct ADDRESS_CONF_EN_ADDRESS_RX_BR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_CONF_EN_ADDRESS_RX_BR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRESS_CONF_EN_ADDRESS_RX_BR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn address_conf_en_address_rx_br_default(self) -> &'a mut W {
        self.variant(ADDRESS_CONF_EN_ADDRESS_RX_BR_A::ADDRESS_CONF_EN_ADDRESS_RX_BR_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "If set to 1 enables the address detection on Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_CONF_EN_ADDRESS_RX_A {
    #[doc = "0: `0`"]
    ADDRESS_CONF_EN_ADDRESS_RX_DEFAULT = 0,
}
impl From<ADDRESS_CONF_EN_ADDRESS_RX_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_CONF_EN_ADDRESS_RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDRESS_CONF_EN_ADDRESS_RX`"]
pub type ADDRESS_CONF_EN_ADDRESS_RX_R = crate::R<bool, ADDRESS_CONF_EN_ADDRESS_RX_A>;
impl ADDRESS_CONF_EN_ADDRESS_RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ADDRESS_CONF_EN_ADDRESS_RX_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(ADDRESS_CONF_EN_ADDRESS_RX_A::ADDRESS_CONF_EN_ADDRESS_RX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADDRESS_CONF_EN_ADDRESS_RX_DEFAULT`"]
    #[inline(always)]
    pub fn is_address_conf_en_address_rx_default(&self) -> bool {
        *self == ADDRESS_CONF_EN_ADDRESS_RX_A::ADDRESS_CONF_EN_ADDRESS_RX_DEFAULT
    }
}
#[doc = "Write proxy for field `ADDRESS_CONF_EN_ADDRESS_RX`"]
pub struct ADDRESS_CONF_EN_ADDRESS_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_CONF_EN_ADDRESS_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRESS_CONF_EN_ADDRESS_RX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn address_conf_en_address_rx_default(self) -> &'a mut W {
        self.variant(ADDRESS_CONF_EN_ADDRESS_RX_A::ADDRESS_CONF_EN_ADDRESS_RX_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "If set to 1 enables the address insertion on Tx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_CONF_EN_ADDRESS_TX_A {
    #[doc = "0: `0`"]
    ADDRESS_CONF_EN_ADDRESS_TX_DEFAULT = 0,
}
impl From<ADDRESS_CONF_EN_ADDRESS_TX_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_CONF_EN_ADDRESS_TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDRESS_CONF_EN_ADDRESS_TX`"]
pub type ADDRESS_CONF_EN_ADDRESS_TX_R = crate::R<bool, ADDRESS_CONF_EN_ADDRESS_TX_A>;
impl ADDRESS_CONF_EN_ADDRESS_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ADDRESS_CONF_EN_ADDRESS_TX_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(ADDRESS_CONF_EN_ADDRESS_TX_A::ADDRESS_CONF_EN_ADDRESS_TX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADDRESS_CONF_EN_ADDRESS_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_address_conf_en_address_tx_default(&self) -> bool {
        *self == ADDRESS_CONF_EN_ADDRESS_TX_A::ADDRESS_CONF_EN_ADDRESS_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `ADDRESS_CONF_EN_ADDRESS_TX`"]
pub struct ADDRESS_CONF_EN_ADDRESS_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_CONF_EN_ADDRESS_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRESS_CONF_EN_ADDRESS_TX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn address_conf_en_address_tx_default(self) -> &'a mut W {
        self.variant(ADDRESS_CONF_EN_ADDRESS_TX_A::ADDRESS_CONF_EN_ADDRESS_TX_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Length of the preamble -1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PREAMBLE_LENGTH_PREAMBLE_LEN_A {
    #[doc = "0: `0`"]
    PREAMBLE_LENGTH_PREAMBLE_LEN_DEFAULT = 0,
}
impl From<PREAMBLE_LENGTH_PREAMBLE_LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: PREAMBLE_LENGTH_PREAMBLE_LEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PREAMBLE_LENGTH_PREAMBLE_LEN`"]
pub type PREAMBLE_LENGTH_PREAMBLE_LEN_R = crate::R<u8, PREAMBLE_LENGTH_PREAMBLE_LEN_A>;
impl PREAMBLE_LENGTH_PREAMBLE_LEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PREAMBLE_LENGTH_PREAMBLE_LEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PREAMBLE_LENGTH_PREAMBLE_LEN_A::PREAMBLE_LENGTH_PREAMBLE_LEN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PREAMBLE_LENGTH_PREAMBLE_LEN_DEFAULT`"]
    #[inline(always)]
    pub fn is_preamble_length_preamble_len_default(&self) -> bool {
        *self == PREAMBLE_LENGTH_PREAMBLE_LEN_A::PREAMBLE_LENGTH_PREAMBLE_LEN_DEFAULT
    }
}
#[doc = "Write proxy for field `PREAMBLE_LENGTH_PREAMBLE_LEN`"]
pub struct PREAMBLE_LENGTH_PREAMBLE_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREAMBLE_LENGTH_PREAMBLE_LEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREAMBLE_LENGTH_PREAMBLE_LEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn preamble_length_preamble_len_default(self) -> &'a mut W {
        self.variant(PREAMBLE_LENGTH_PREAMBLE_LEN_A::PREAMBLE_LENGTH_PREAMBLE_LEN_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Preamble to be inserted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PREAMBLE_PREAMBLE_A {
    #[doc = "0: `0`"]
    PREAMBLE_PREAMBLE_DEFAULT = 0,
}
impl From<PREAMBLE_PREAMBLE_A> for u8 {
    #[inline(always)]
    fn from(variant: PREAMBLE_PREAMBLE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PREAMBLE_PREAMBLE`"]
pub type PREAMBLE_PREAMBLE_R = crate::R<u8, PREAMBLE_PREAMBLE_A>;
impl PREAMBLE_PREAMBLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PREAMBLE_PREAMBLE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PREAMBLE_PREAMBLE_A::PREAMBLE_PREAMBLE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PREAMBLE_PREAMBLE_DEFAULT`"]
    #[inline(always)]
    pub fn is_preamble_preamble_default(&self) -> bool {
        *self == PREAMBLE_PREAMBLE_A::PREAMBLE_PREAMBLE_DEFAULT
    }
}
#[doc = "Write proxy for field `PREAMBLE_PREAMBLE`"]
pub struct PREAMBLE_PREAMBLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PREAMBLE_PREAMBLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREAMBLE_PREAMBLE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn preamble_preamble_default(self) -> &'a mut W {
        self.variant(PREAMBLE_PREAMBLE_A::PREAMBLE_PREAMBLE_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "If set to 1, the packet length is fixed and specified in the PACKET_LEN register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_A {
    #[doc = "0: `0`"]
    PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_DEFAULT = 0,
}
impl From<PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_A> for bool {
    #[inline(always)]
    fn from(variant: PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX`"]
pub type PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_R =
    crate::R<bool, PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_A>;
impl PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_A> {
        use crate::Variant::*;
        match self . bits { false => Val ( PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_A :: PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_DEFAULT ) , i => Res ( i ) }
    }
    #[doc = "Checks if the value of the field is `PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_DEFAULT`"]
    #[inline(always)]
    pub fn is_packet_length_opts_en_packet_len_fix_default(&self) -> bool {
        *self
            == PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_A::PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_DEFAULT
    }
}
#[doc = "Write proxy for field `PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX`"]
pub struct PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn packet_length_opts_en_packet_len_fix_default(self) -> &'a mut W {
        self.variant(
            PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_A::PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_DEFAULT,
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Signed value that specifies the correction to apply to the specified packet length (due to differences between standards). The packet length here is specified by the byte number after the packet length byte, with the exclusion of the CRC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PACKET_LENGTH_OPTS_PACKET_LEN_CORR_A {
    #[doc = "0: `0`"]
    PACKET_LENGTH_OPTS_PACKET_LEN_CORR_DEFAULT = 0,
}
impl From<PACKET_LENGTH_OPTS_PACKET_LEN_CORR_A> for u8 {
    #[inline(always)]
    fn from(variant: PACKET_LENGTH_OPTS_PACKET_LEN_CORR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PACKET_LENGTH_OPTS_PACKET_LEN_CORR`"]
pub type PACKET_LENGTH_OPTS_PACKET_LEN_CORR_R = crate::R<u8, PACKET_LENGTH_OPTS_PACKET_LEN_CORR_A>;
impl PACKET_LENGTH_OPTS_PACKET_LEN_CORR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PACKET_LENGTH_OPTS_PACKET_LEN_CORR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(
                PACKET_LENGTH_OPTS_PACKET_LEN_CORR_A::PACKET_LENGTH_OPTS_PACKET_LEN_CORR_DEFAULT,
            ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PACKET_LENGTH_OPTS_PACKET_LEN_CORR_DEFAULT`"]
    #[inline(always)]
    pub fn is_packet_length_opts_packet_len_corr_default(&self) -> bool {
        *self == PACKET_LENGTH_OPTS_PACKET_LEN_CORR_A::PACKET_LENGTH_OPTS_PACKET_LEN_CORR_DEFAULT
    }
}
#[doc = "Write proxy for field `PACKET_LENGTH_OPTS_PACKET_LEN_CORR`"]
pub struct PACKET_LENGTH_OPTS_PACKET_LEN_CORR_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_LENGTH_OPTS_PACKET_LEN_CORR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACKET_LENGTH_OPTS_PACKET_LEN_CORR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn packet_length_opts_packet_len_corr_default(self) -> &'a mut W {
        self.variant(
            PACKET_LENGTH_OPTS_PACKET_LEN_CORR_A::PACKET_LENGTH_OPTS_PACKET_LEN_CORR_DEFAULT,
        )
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Unsigned value that specifies the position of the packet length after the pattern\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PACKET_LENGTH_OPTS_PACKET_LEN_POS_A {
    #[doc = "0: `0`"]
    PACKET_LENGTH_OPTS_PACKET_LEN_POS_DEFAULT = 0,
}
impl From<PACKET_LENGTH_OPTS_PACKET_LEN_POS_A> for u8 {
    #[inline(always)]
    fn from(variant: PACKET_LENGTH_OPTS_PACKET_LEN_POS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PACKET_LENGTH_OPTS_PACKET_LEN_POS`"]
pub type PACKET_LENGTH_OPTS_PACKET_LEN_POS_R = crate::R<u8, PACKET_LENGTH_OPTS_PACKET_LEN_POS_A>;
impl PACKET_LENGTH_OPTS_PACKET_LEN_POS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PACKET_LENGTH_OPTS_PACKET_LEN_POS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => {
                Val(PACKET_LENGTH_OPTS_PACKET_LEN_POS_A::PACKET_LENGTH_OPTS_PACKET_LEN_POS_DEFAULT)
            }
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PACKET_LENGTH_OPTS_PACKET_LEN_POS_DEFAULT`"]
    #[inline(always)]
    pub fn is_packet_length_opts_packet_len_pos_default(&self) -> bool {
        *self == PACKET_LENGTH_OPTS_PACKET_LEN_POS_A::PACKET_LENGTH_OPTS_PACKET_LEN_POS_DEFAULT
    }
}
#[doc = "Write proxy for field `PACKET_LENGTH_OPTS_PACKET_LEN_POS`"]
pub struct PACKET_LENGTH_OPTS_PACKET_LEN_POS_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_LENGTH_OPTS_PACKET_LEN_POS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACKET_LENGTH_OPTS_PACKET_LEN_POS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn packet_length_opts_packet_len_pos_default(self) -> &'a mut W {
        self.variant(PACKET_LENGTH_OPTS_PACKET_LEN_POS_A::PACKET_LENGTH_OPTS_PACKET_LEN_POS_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 27 - If set to 1 the address length is 16 bits, 8 otherwise."]
    #[inline(always)]
    pub fn address_conf_address_len(&self) -> ADDRESS_CONF_ADDRESS_LEN_R {
        ADDRESS_CONF_ADDRESS_LEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - If set to 1 enables the broadcast address detection on Rx."]
    #[inline(always)]
    pub fn address_conf_en_address_rx_br(&self) -> ADDRESS_CONF_EN_ADDRESS_RX_BR_R {
        ADDRESS_CONF_EN_ADDRESS_RX_BR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - If set to 1 enables the address detection on Rx"]
    #[inline(always)]
    pub fn address_conf_en_address_rx(&self) -> ADDRESS_CONF_EN_ADDRESS_RX_R {
        ADDRESS_CONF_EN_ADDRESS_RX_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - If set to 1 enables the address insertion on Tx"]
    #[inline(always)]
    pub fn address_conf_en_address_tx(&self) -> ADDRESS_CONF_EN_ADDRESS_TX_R {
        ADDRESS_CONF_EN_ADDRESS_TX_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Length of the preamble -1"]
    #[inline(always)]
    pub fn preamble_length_preamble_len(&self) -> PREAMBLE_LENGTH_PREAMBLE_LEN_R {
        PREAMBLE_LENGTH_PREAMBLE_LEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Preamble to be inserted"]
    #[inline(always)]
    pub fn preamble_preamble(&self) -> PREAMBLE_PREAMBLE_R {
        PREAMBLE_PREAMBLE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 6 - If set to 1, the packet length is fixed and specified in the PACKET_LEN register"]
    #[inline(always)]
    pub fn packet_length_opts_en_packet_len_fix(&self) -> PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_R {
        PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - Signed value that specifies the correction to apply to the specified packet length (due to differences between standards). The packet length here is specified by the byte number after the packet length byte, with the exclusion of the CRC."]
    #[inline(always)]
    pub fn packet_length_opts_packet_len_corr(&self) -> PACKET_LENGTH_OPTS_PACKET_LEN_CORR_R {
        PACKET_LENGTH_OPTS_PACKET_LEN_CORR_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1 - Unsigned value that specifies the position of the packet length after the pattern"]
    #[inline(always)]
    pub fn packet_length_opts_packet_len_pos(&self) -> PACKET_LENGTH_OPTS_PACKET_LEN_POS_R {
        PACKET_LENGTH_OPTS_PACKET_LEN_POS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 27 - If set to 1 the address length is 16 bits, 8 otherwise."]
    #[inline(always)]
    pub fn address_conf_address_len(&mut self) -> ADDRESS_CONF_ADDRESS_LEN_W {
        ADDRESS_CONF_ADDRESS_LEN_W { w: self }
    }
    #[doc = "Bit 26 - If set to 1 enables the broadcast address detection on Rx."]
    #[inline(always)]
    pub fn address_conf_en_address_rx_br(&mut self) -> ADDRESS_CONF_EN_ADDRESS_RX_BR_W {
        ADDRESS_CONF_EN_ADDRESS_RX_BR_W { w: self }
    }
    #[doc = "Bit 25 - If set to 1 enables the address detection on Rx"]
    #[inline(always)]
    pub fn address_conf_en_address_rx(&mut self) -> ADDRESS_CONF_EN_ADDRESS_RX_W {
        ADDRESS_CONF_EN_ADDRESS_RX_W { w: self }
    }
    #[doc = "Bit 24 - If set to 1 enables the address insertion on Tx"]
    #[inline(always)]
    pub fn address_conf_en_address_tx(&mut self) -> ADDRESS_CONF_EN_ADDRESS_TX_W {
        ADDRESS_CONF_EN_ADDRESS_TX_W { w: self }
    }
    #[doc = "Bits 16:23 - Length of the preamble -1"]
    #[inline(always)]
    pub fn preamble_length_preamble_len(&mut self) -> PREAMBLE_LENGTH_PREAMBLE_LEN_W {
        PREAMBLE_LENGTH_PREAMBLE_LEN_W { w: self }
    }
    #[doc = "Bits 8:15 - Preamble to be inserted"]
    #[inline(always)]
    pub fn preamble_preamble(&mut self) -> PREAMBLE_PREAMBLE_W {
        PREAMBLE_PREAMBLE_W { w: self }
    }
    #[doc = "Bit 6 - If set to 1, the packet length is fixed and specified in the PACKET_LEN register"]
    #[inline(always)]
    pub fn packet_length_opts_en_packet_len_fix(
        &mut self,
    ) -> PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_W {
        PACKET_LENGTH_OPTS_EN_PACKET_LEN_FIX_W { w: self }
    }
    #[doc = "Bits 2:5 - Signed value that specifies the correction to apply to the specified packet length (due to differences between standards). The packet length here is specified by the byte number after the packet length byte, with the exclusion of the CRC."]
    #[inline(always)]
    pub fn packet_length_opts_packet_len_corr(&mut self) -> PACKET_LENGTH_OPTS_PACKET_LEN_CORR_W {
        PACKET_LENGTH_OPTS_PACKET_LEN_CORR_W { w: self }
    }
    #[doc = "Bits 0:1 - Unsigned value that specifies the position of the packet length after the pattern"]
    #[inline(always)]
    pub fn packet_length_opts_packet_len_pos(&mut self) -> PACKET_LENGTH_OPTS_PACKET_LEN_POS_W {
        PACKET_LENGTH_OPTS_PACKET_LEN_POS_W { w: self }
    }
}
