#[doc = "Reader of register RF_REG02"]
pub type R = crate::R<u32, super::RF_REG02>;
#[doc = "Writer for register RF_REG02"]
pub type W = crate::W<u32, super::RF_REG02>;
#[doc = "Register RF_REG02 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG02 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "If set to 1, stops the Rx and flushes the FIFO in case of overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_FIFO_FLUSH_ON_OVFLW_A {
    #[doc = "0: `0`"]
    FIFO_FIFO_FLUSH_ON_OVFLW_DEFAULT = 0,
}
impl From<FIFO_FIFO_FLUSH_ON_OVFLW_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_FIFO_FLUSH_ON_OVFLW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIFO_FIFO_FLUSH_ON_OVFLW`"]
pub type FIFO_FIFO_FLUSH_ON_OVFLW_R = crate::R<bool, FIFO_FIFO_FLUSH_ON_OVFLW_A>;
impl FIFO_FIFO_FLUSH_ON_OVFLW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FIFO_FIFO_FLUSH_ON_OVFLW_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(FIFO_FIFO_FLUSH_ON_OVFLW_A::FIFO_FIFO_FLUSH_ON_OVFLW_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_FIFO_FLUSH_ON_OVFLW_DEFAULT`"]
    #[inline(always)]
    pub fn is_fifo_fifo_flush_on_ovflw_default(&self) -> bool {
        *self == FIFO_FIFO_FLUSH_ON_OVFLW_A::FIFO_FIFO_FLUSH_ON_OVFLW_DEFAULT
    }
}
#[doc = "Write proxy for field `FIFO_FIFO_FLUSH_ON_OVFLW`"]
pub struct FIFO_FIFO_FLUSH_ON_OVFLW_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_FIFO_FLUSH_ON_OVFLW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_FIFO_FLUSH_ON_OVFLW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fifo_fifo_flush_on_ovflw_default(self) -> &'a mut W {
        self.variant(FIFO_FIFO_FLUSH_ON_OVFLW_A::FIFO_FIFO_FLUSH_ON_OVFLW_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "If set to 1, stops the Rx and flushes the FIFO in case of address error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_FIFO_FLUSH_ON_ADDR_ERR_A {
    #[doc = "0: `0`"]
    FIFO_FIFO_FLUSH_ON_ADDR_ERR_DEFAULT = 0,
}
impl From<FIFO_FIFO_FLUSH_ON_ADDR_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_FIFO_FLUSH_ON_ADDR_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIFO_FIFO_FLUSH_ON_ADDR_ERR`"]
pub type FIFO_FIFO_FLUSH_ON_ADDR_ERR_R = crate::R<bool, FIFO_FIFO_FLUSH_ON_ADDR_ERR_A>;
impl FIFO_FIFO_FLUSH_ON_ADDR_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FIFO_FIFO_FLUSH_ON_ADDR_ERR_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(FIFO_FIFO_FLUSH_ON_ADDR_ERR_A::FIFO_FIFO_FLUSH_ON_ADDR_ERR_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_FIFO_FLUSH_ON_ADDR_ERR_DEFAULT`"]
    #[inline(always)]
    pub fn is_fifo_fifo_flush_on_addr_err_default(&self) -> bool {
        *self == FIFO_FIFO_FLUSH_ON_ADDR_ERR_A::FIFO_FIFO_FLUSH_ON_ADDR_ERR_DEFAULT
    }
}
#[doc = "Write proxy for field `FIFO_FIFO_FLUSH_ON_ADDR_ERR`"]
pub struct FIFO_FIFO_FLUSH_ON_ADDR_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_FIFO_FLUSH_ON_ADDR_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_FIFO_FLUSH_ON_ADDR_ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fifo_fifo_flush_on_addr_err_default(self) -> &'a mut W {
        self.variant(FIFO_FIFO_FLUSH_ON_ADDR_ERR_A::FIFO_FIFO_FLUSH_ON_ADDR_ERR_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "If set to 1, stops the Rx and flushes the FIFO in case of packet length error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_FIFO_FLUSH_ON_PL_ERR_A {
    #[doc = "0: `0`"]
    FIFO_FIFO_FLUSH_ON_PL_ERR_DEFAULT = 0,
}
impl From<FIFO_FIFO_FLUSH_ON_PL_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_FIFO_FLUSH_ON_PL_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIFO_FIFO_FLUSH_ON_PL_ERR`"]
pub type FIFO_FIFO_FLUSH_ON_PL_ERR_R = crate::R<bool, FIFO_FIFO_FLUSH_ON_PL_ERR_A>;
impl FIFO_FIFO_FLUSH_ON_PL_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FIFO_FIFO_FLUSH_ON_PL_ERR_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(FIFO_FIFO_FLUSH_ON_PL_ERR_A::FIFO_FIFO_FLUSH_ON_PL_ERR_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_FIFO_FLUSH_ON_PL_ERR_DEFAULT`"]
    #[inline(always)]
    pub fn is_fifo_fifo_flush_on_pl_err_default(&self) -> bool {
        *self == FIFO_FIFO_FLUSH_ON_PL_ERR_A::FIFO_FIFO_FLUSH_ON_PL_ERR_DEFAULT
    }
}
#[doc = "Write proxy for field `FIFO_FIFO_FLUSH_ON_PL_ERR`"]
pub struct FIFO_FIFO_FLUSH_ON_PL_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_FIFO_FLUSH_ON_PL_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_FIFO_FLUSH_ON_PL_ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fifo_fifo_flush_on_pl_err_default(self) -> &'a mut W {
        self.variant(FIFO_FIFO_FLUSH_ON_PL_ERR_A::FIFO_FIFO_FLUSH_ON_PL_ERR_DEFAULT)
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
#[doc = "If set to 1, stops the Rx and flushes the FIFO in case of CRC error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_FIFO_FLUSH_ON_CRC_ERR_A {
    #[doc = "0: `0`"]
    FIFO_FIFO_FLUSH_ON_CRC_ERR_DEFAULT = 0,
}
impl From<FIFO_FIFO_FLUSH_ON_CRC_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_FIFO_FLUSH_ON_CRC_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIFO_FIFO_FLUSH_ON_CRC_ERR`"]
pub type FIFO_FIFO_FLUSH_ON_CRC_ERR_R = crate::R<bool, FIFO_FIFO_FLUSH_ON_CRC_ERR_A>;
impl FIFO_FIFO_FLUSH_ON_CRC_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FIFO_FIFO_FLUSH_ON_CRC_ERR_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(FIFO_FIFO_FLUSH_ON_CRC_ERR_A::FIFO_FIFO_FLUSH_ON_CRC_ERR_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_FIFO_FLUSH_ON_CRC_ERR_DEFAULT`"]
    #[inline(always)]
    pub fn is_fifo_fifo_flush_on_crc_err_default(&self) -> bool {
        *self == FIFO_FIFO_FLUSH_ON_CRC_ERR_A::FIFO_FIFO_FLUSH_ON_CRC_ERR_DEFAULT
    }
}
#[doc = "Write proxy for field `FIFO_FIFO_FLUSH_ON_CRC_ERR`"]
pub struct FIFO_FIFO_FLUSH_ON_CRC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_FIFO_FLUSH_ON_CRC_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_FIFO_FLUSH_ON_CRC_ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fifo_fifo_flush_on_crc_err_default(self) -> &'a mut W {
        self.variant(FIFO_FIFO_FLUSH_ON_CRC_ERR_A::FIFO_FIFO_FLUSH_ON_CRC_ERR_DEFAULT)
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
#[doc = "If set to 1, the Rx FIFO needs an acknowledgement (packet received correctly) to change its state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_RX_FIFO_ACK_A {
    #[doc = "0: `0`"]
    FIFO_RX_FIFO_ACK_DEFAULT = 0,
}
impl From<FIFO_RX_FIFO_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_RX_FIFO_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIFO_RX_FIFO_ACK`"]
pub type FIFO_RX_FIFO_ACK_R = crate::R<bool, FIFO_RX_FIFO_ACK_A>;
impl FIFO_RX_FIFO_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FIFO_RX_FIFO_ACK_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(FIFO_RX_FIFO_ACK_A::FIFO_RX_FIFO_ACK_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_RX_FIFO_ACK_DEFAULT`"]
    #[inline(always)]
    pub fn is_fifo_rx_fifo_ack_default(&self) -> bool {
        *self == FIFO_RX_FIFO_ACK_A::FIFO_RX_FIFO_ACK_DEFAULT
    }
}
#[doc = "Write proxy for field `FIFO_RX_FIFO_ACK`"]
pub struct FIFO_RX_FIFO_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_RX_FIFO_ACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_RX_FIFO_ACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fifo_rx_fifo_ack_default(self) -> &'a mut W {
        self.variant(FIFO_RX_FIFO_ACK_A::FIFO_RX_FIFO_ACK_DEFAULT)
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
#[doc = "Threshold indicating the 'almost full' state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FIFO_FIFO_THR_A {
    #[doc = "0: `0`"]
    FIFO_FIFO_THR_DEFAULT = 0,
}
impl From<FIFO_FIFO_THR_A> for u8 {
    #[inline(always)]
    fn from(variant: FIFO_FIFO_THR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FIFO_FIFO_THR`"]
pub type FIFO_FIFO_THR_R = crate::R<u8, FIFO_FIFO_THR_A>;
impl FIFO_FIFO_THR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FIFO_FIFO_THR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FIFO_FIFO_THR_A::FIFO_FIFO_THR_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_FIFO_THR_DEFAULT`"]
    #[inline(always)]
    pub fn is_fifo_fifo_thr_default(&self) -> bool {
        *self == FIFO_FIFO_THR_A::FIFO_FIFO_THR_DEFAULT
    }
}
#[doc = "Write proxy for field `FIFO_FIFO_THR`"]
pub struct FIFO_FIFO_THR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_FIFO_THR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_FIFO_THR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fifo_fifo_thr_default(self) -> &'a mut W {
        self.variant(FIFO_FIFO_THR_A::FIFO_FIFO_THR_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Data-rate offset. Is a signed value and the full scale (0x7f) corresponds to a data-rate offset of 12.5 percent.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATARATE_OFFSET_DATARATE_OFFSET_A {
    #[doc = "0: `0`"]
    DATARATE_OFFSET_DATARATE_OFFSET_DEFAULT = 0,
}
impl From<DATARATE_OFFSET_DATARATE_OFFSET_A> for u8 {
    #[inline(always)]
    fn from(variant: DATARATE_OFFSET_DATARATE_OFFSET_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DATARATE_OFFSET_DATARATE_OFFSET`"]
pub type DATARATE_OFFSET_DATARATE_OFFSET_R = crate::R<u8, DATARATE_OFFSET_DATARATE_OFFSET_A>;
impl DATARATE_OFFSET_DATARATE_OFFSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DATARATE_OFFSET_DATARATE_OFFSET_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DATARATE_OFFSET_DATARATE_OFFSET_A::DATARATE_OFFSET_DATARATE_OFFSET_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DATARATE_OFFSET_DATARATE_OFFSET_DEFAULT`"]
    #[inline(always)]
    pub fn is_datarate_offset_datarate_offset_default(&self) -> bool {
        *self == DATARATE_OFFSET_DATARATE_OFFSET_A::DATARATE_OFFSET_DATARATE_OFFSET_DEFAULT
    }
}
#[doc = "Write proxy for field `DATARATE_OFFSET_DATARATE_OFFSET`"]
pub struct DATARATE_OFFSET_DATARATE_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> DATARATE_OFFSET_DATARATE_OFFSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATARATE_OFFSET_DATARATE_OFFSET_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn datarate_offset_datarate_offset_default(self) -> &'a mut W {
        self.variant(DATARATE_OFFSET_DATARATE_OFFSET_A::DATARATE_OFFSET_DATARATE_OFFSET_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Time constant of the data-rate recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_A {
    #[doc = "0: `0`"]
    TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_DEFAULT = 0,
}
impl From<TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_A> for u8 {
    #[inline(always)]
    fn from(variant: TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TAU_DATARATE_RECOV_TAU_DATARATE_RECOV`"]
pub type TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_R =
    crate::R<u8, TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_A>;
impl TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_A> {
        use crate::Variant::*;
        match self . bits { 0 => Val ( TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_A :: TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_DEFAULT ) , i => Res ( i ) }
    }
    #[doc = "Checks if the value of the field is `TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_DEFAULT`"]
    #[inline(always)]
    pub fn is_tau_datarate_recov_tau_datarate_recov_default(&self) -> bool {
        * self == TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_A :: TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_DEFAULT
    }
}
#[doc = "Write proxy for field `TAU_DATARATE_RECOV_TAU_DATARATE_RECOV`"]
pub struct TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_W<'a> {
    w: &'a mut W,
}
impl<'a> TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tau_datarate_recov_tau_datarate_recov_default(self) -> &'a mut W {
        self.variant(
            TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_A::TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_DEFAULT,
        )
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Time constant of the clock recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAU_CLK_RECOV_TAU_CLK_RECOV_A {
    #[doc = "0: `0`"]
    TAU_CLK_RECOV_TAU_CLK_RECOV_DEFAULT = 0,
}
impl From<TAU_CLK_RECOV_TAU_CLK_RECOV_A> for u8 {
    #[inline(always)]
    fn from(variant: TAU_CLK_RECOV_TAU_CLK_RECOV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TAU_CLK_RECOV_TAU_CLK_RECOV`"]
pub type TAU_CLK_RECOV_TAU_CLK_RECOV_R = crate::R<u8, TAU_CLK_RECOV_TAU_CLK_RECOV_A>;
impl TAU_CLK_RECOV_TAU_CLK_RECOV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TAU_CLK_RECOV_TAU_CLK_RECOV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TAU_CLK_RECOV_TAU_CLK_RECOV_A::TAU_CLK_RECOV_TAU_CLK_RECOV_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TAU_CLK_RECOV_TAU_CLK_RECOV_DEFAULT`"]
    #[inline(always)]
    pub fn is_tau_clk_recov_tau_clk_recov_default(&self) -> bool {
        *self == TAU_CLK_RECOV_TAU_CLK_RECOV_A::TAU_CLK_RECOV_TAU_CLK_RECOV_DEFAULT
    }
}
#[doc = "Write proxy for field `TAU_CLK_RECOV_TAU_CLK_RECOV`"]
pub struct TAU_CLK_RECOV_TAU_CLK_RECOV_W<'a> {
    w: &'a mut W,
}
impl<'a> TAU_CLK_RECOV_TAU_CLK_RECOV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAU_CLK_RECOV_TAU_CLK_RECOV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tau_clk_recov_tau_clk_recov_default(self) -> &'a mut W {
        self.variant(TAU_CLK_RECOV_TAU_CLK_RECOV_A::TAU_CLK_RECOV_TAU_CLK_RECOV_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - If set to 1, stops the Rx and flushes the FIFO in case of overflow"]
    #[inline(always)]
    pub fn fifo_fifo_flush_on_ovflw(&self) -> FIFO_FIFO_FLUSH_ON_OVFLW_R {
        FIFO_FIFO_FLUSH_ON_OVFLW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - If set to 1, stops the Rx and flushes the FIFO in case of address error"]
    #[inline(always)]
    pub fn fifo_fifo_flush_on_addr_err(&self) -> FIFO_FIFO_FLUSH_ON_ADDR_ERR_R {
        FIFO_FIFO_FLUSH_ON_ADDR_ERR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - If set to 1, stops the Rx and flushes the FIFO in case of packet length error"]
    #[inline(always)]
    pub fn fifo_fifo_flush_on_pl_err(&self) -> FIFO_FIFO_FLUSH_ON_PL_ERR_R {
        FIFO_FIFO_FLUSH_ON_PL_ERR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - If set to 1, stops the Rx and flushes the FIFO in case of CRC error"]
    #[inline(always)]
    pub fn fifo_fifo_flush_on_crc_err(&self) -> FIFO_FIFO_FLUSH_ON_CRC_ERR_R {
        FIFO_FIFO_FLUSH_ON_CRC_ERR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - If set to 1, the Rx FIFO needs an acknowledgement (packet received correctly) to change its state."]
    #[inline(always)]
    pub fn fifo_rx_fifo_ack(&self) -> FIFO_RX_FIFO_ACK_R {
        FIFO_RX_FIFO_ACK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Threshold indicating the 'almost full' state"]
    #[inline(always)]
    pub fn fifo_fifo_thr(&self) -> FIFO_FIFO_THR_R {
        FIFO_FIFO_THR_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:23 - Data-rate offset. Is a signed value and the full scale (0x7f) corresponds to a data-rate offset of 12.5 percent."]
    #[inline(always)]
    pub fn datarate_offset_datarate_offset(&self) -> DATARATE_OFFSET_DATARATE_OFFSET_R {
        DATARATE_OFFSET_DATARATE_OFFSET_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Time constant of the data-rate recovery"]
    #[inline(always)]
    pub fn tau_datarate_recov_tau_datarate_recov(&self) -> TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_R {
        TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Time constant of the clock recovery"]
    #[inline(always)]
    pub fn tau_clk_recov_tau_clk_recov(&self) -> TAU_CLK_RECOV_TAU_CLK_RECOV_R {
        TAU_CLK_RECOV_TAU_CLK_RECOV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - If set to 1, stops the Rx and flushes the FIFO in case of overflow"]
    #[inline(always)]
    pub fn fifo_fifo_flush_on_ovflw(&mut self) -> FIFO_FIFO_FLUSH_ON_OVFLW_W {
        FIFO_FIFO_FLUSH_ON_OVFLW_W { w: self }
    }
    #[doc = "Bit 30 - If set to 1, stops the Rx and flushes the FIFO in case of address error"]
    #[inline(always)]
    pub fn fifo_fifo_flush_on_addr_err(&mut self) -> FIFO_FIFO_FLUSH_ON_ADDR_ERR_W {
        FIFO_FIFO_FLUSH_ON_ADDR_ERR_W { w: self }
    }
    #[doc = "Bit 29 - If set to 1, stops the Rx and flushes the FIFO in case of packet length error"]
    #[inline(always)]
    pub fn fifo_fifo_flush_on_pl_err(&mut self) -> FIFO_FIFO_FLUSH_ON_PL_ERR_W {
        FIFO_FIFO_FLUSH_ON_PL_ERR_W { w: self }
    }
    #[doc = "Bit 28 - If set to 1, stops the Rx and flushes the FIFO in case of CRC error"]
    #[inline(always)]
    pub fn fifo_fifo_flush_on_crc_err(&mut self) -> FIFO_FIFO_FLUSH_ON_CRC_ERR_W {
        FIFO_FIFO_FLUSH_ON_CRC_ERR_W { w: self }
    }
    #[doc = "Bit 27 - If set to 1, the Rx FIFO needs an acknowledgement (packet received correctly) to change its state."]
    #[inline(always)]
    pub fn fifo_rx_fifo_ack(&mut self) -> FIFO_RX_FIFO_ACK_W {
        FIFO_RX_FIFO_ACK_W { w: self }
    }
    #[doc = "Bits 24:26 - Threshold indicating the 'almost full' state"]
    #[inline(always)]
    pub fn fifo_fifo_thr(&mut self) -> FIFO_FIFO_THR_W {
        FIFO_FIFO_THR_W { w: self }
    }
    #[doc = "Bits 16:23 - Data-rate offset. Is a signed value and the full scale (0x7f) corresponds to a data-rate offset of 12.5 percent."]
    #[inline(always)]
    pub fn datarate_offset_datarate_offset(&mut self) -> DATARATE_OFFSET_DATARATE_OFFSET_W {
        DATARATE_OFFSET_DATARATE_OFFSET_W { w: self }
    }
    #[doc = "Bits 8:15 - Time constant of the data-rate recovery"]
    #[inline(always)]
    pub fn tau_datarate_recov_tau_datarate_recov(
        &mut self,
    ) -> TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_W {
        TAU_DATARATE_RECOV_TAU_DATARATE_RECOV_W { w: self }
    }
    #[doc = "Bits 0:7 - Time constant of the clock recovery"]
    #[inline(always)]
    pub fn tau_clk_recov_tau_clk_recov(&mut self) -> TAU_CLK_RECOV_TAU_CLK_RECOV_W {
        TAU_CLK_RECOV_TAU_CLK_RECOV_W { w: self }
    }
}
