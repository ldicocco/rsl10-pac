#[doc = "Reader of register RF_REG04"]
pub type R = crate::R<u32, super::RF_REG04>;
#[doc = "Writer for register RF_REG04"]
pub type W = crate::W<u32, super::RF_REG04>;
#[doc = "Register RF_REG04 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG04 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MAC timer granularity. The granularity is given by (2^(2mac_timer_gr))x1us\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAC_CONF_MAC_TIMER_GR_A {
    #[doc = "0: `0`"]
    MAC_CONF_MAC_TIMER_GR_DEFAULT = 0,
}
impl From<MAC_CONF_MAC_TIMER_GR_A> for u8 {
    #[inline(always)]
    fn from(variant: MAC_CONF_MAC_TIMER_GR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MAC_CONF_MAC_TIMER_GR`"]
pub type MAC_CONF_MAC_TIMER_GR_R = crate::R<u8, MAC_CONF_MAC_TIMER_GR_A>;
impl MAC_CONF_MAC_TIMER_GR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MAC_CONF_MAC_TIMER_GR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MAC_CONF_MAC_TIMER_GR_A::MAC_CONF_MAC_TIMER_GR_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAC_CONF_MAC_TIMER_GR_DEFAULT`"]
    #[inline(always)]
    pub fn is_mac_conf_mac_timer_gr_default(&self) -> bool {
        *self == MAC_CONF_MAC_TIMER_GR_A::MAC_CONF_MAC_TIMER_GR_DEFAULT
    }
}
#[doc = "Write proxy for field `MAC_CONF_MAC_TIMER_GR`"]
pub struct MAC_CONF_MAC_TIMER_GR_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_CONF_MAC_TIMER_GR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAC_CONF_MAC_TIMER_GR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mac_conf_mac_timer_gr_default(self) -> &'a mut W {
        self.variant(MAC_CONF_MAC_TIMER_GR_A::MAC_CONF_MAC_TIMER_GR_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "If set to 1, the FSM will switch to Rx or Tx after an Rx mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAC_CONF_RX_MAC_ACT_A {
    #[doc = "0: `0`"]
    MAC_CONF_RX_MAC_ACT_DEFAULT = 0,
}
impl From<MAC_CONF_RX_MAC_ACT_A> for bool {
    #[inline(always)]
    fn from(variant: MAC_CONF_RX_MAC_ACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAC_CONF_RX_MAC_ACT`"]
pub type MAC_CONF_RX_MAC_ACT_R = crate::R<bool, MAC_CONF_RX_MAC_ACT_A>;
impl MAC_CONF_RX_MAC_ACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MAC_CONF_RX_MAC_ACT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MAC_CONF_RX_MAC_ACT_A::MAC_CONF_RX_MAC_ACT_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAC_CONF_RX_MAC_ACT_DEFAULT`"]
    #[inline(always)]
    pub fn is_mac_conf_rx_mac_act_default(&self) -> bool {
        *self == MAC_CONF_RX_MAC_ACT_A::MAC_CONF_RX_MAC_ACT_DEFAULT
    }
}
#[doc = "Write proxy for field `MAC_CONF_RX_MAC_ACT`"]
pub struct MAC_CONF_RX_MAC_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_CONF_RX_MAC_ACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAC_CONF_RX_MAC_ACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mac_conf_rx_mac_act_default(self) -> &'a mut W {
        self.variant(MAC_CONF_RX_MAC_ACT_A::MAC_CONF_RX_MAC_ACT_DEFAULT)
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
#[doc = "If set to 1, the FSM will switch to Tx after an Rx mode, Rx otherwise.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAC_CONF_RX_MAC_TX_NRX_A {
    #[doc = "0: `0`"]
    MAC_CONF_RX_MAC_TX_NRX_DEFAULT = 0,
}
impl From<MAC_CONF_RX_MAC_TX_NRX_A> for bool {
    #[inline(always)]
    fn from(variant: MAC_CONF_RX_MAC_TX_NRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAC_CONF_RX_MAC_TX_NRX`"]
pub type MAC_CONF_RX_MAC_TX_NRX_R = crate::R<bool, MAC_CONF_RX_MAC_TX_NRX_A>;
impl MAC_CONF_RX_MAC_TX_NRX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MAC_CONF_RX_MAC_TX_NRX_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MAC_CONF_RX_MAC_TX_NRX_A::MAC_CONF_RX_MAC_TX_NRX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAC_CONF_RX_MAC_TX_NRX_DEFAULT`"]
    #[inline(always)]
    pub fn is_mac_conf_rx_mac_tx_nrx_default(&self) -> bool {
        *self == MAC_CONF_RX_MAC_TX_NRX_A::MAC_CONF_RX_MAC_TX_NRX_DEFAULT
    }
}
#[doc = "Write proxy for field `MAC_CONF_RX_MAC_TX_NRX`"]
pub struct MAC_CONF_RX_MAC_TX_NRX_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_CONF_RX_MAC_TX_NRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAC_CONF_RX_MAC_TX_NRX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mac_conf_rx_mac_tx_nrx_default(self) -> &'a mut W {
        self.variant(MAC_CONF_RX_MAC_TX_NRX_A::MAC_CONF_RX_MAC_TX_NRX_DEFAULT)
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
#[doc = "If set to 1, the MAC timer is activated at the reception of the sync word, at the end of the packet otherwise.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAC_CONF_RX_MAC_START_NSTOP_A {
    #[doc = "0: `0`"]
    MAC_CONF_RX_MAC_START_NSTOP_DEFAULT = 0,
}
impl From<MAC_CONF_RX_MAC_START_NSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: MAC_CONF_RX_MAC_START_NSTOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAC_CONF_RX_MAC_START_NSTOP`"]
pub type MAC_CONF_RX_MAC_START_NSTOP_R = crate::R<bool, MAC_CONF_RX_MAC_START_NSTOP_A>;
impl MAC_CONF_RX_MAC_START_NSTOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MAC_CONF_RX_MAC_START_NSTOP_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MAC_CONF_RX_MAC_START_NSTOP_A::MAC_CONF_RX_MAC_START_NSTOP_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAC_CONF_RX_MAC_START_NSTOP_DEFAULT`"]
    #[inline(always)]
    pub fn is_mac_conf_rx_mac_start_nstop_default(&self) -> bool {
        *self == MAC_CONF_RX_MAC_START_NSTOP_A::MAC_CONF_RX_MAC_START_NSTOP_DEFAULT
    }
}
#[doc = "Write proxy for field `MAC_CONF_RX_MAC_START_NSTOP`"]
pub struct MAC_CONF_RX_MAC_START_NSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_CONF_RX_MAC_START_NSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAC_CONF_RX_MAC_START_NSTOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mac_conf_rx_mac_start_nstop_default(self) -> &'a mut W {
        self.variant(MAC_CONF_RX_MAC_START_NSTOP_A::MAC_CONF_RX_MAC_START_NSTOP_DEFAULT)
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
#[doc = "If set to 1, the FSM will switch to Rx or Tx after a Tx mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAC_CONF_TX_MAC_ACT_A {
    #[doc = "0: `0`"]
    MAC_CONF_TX_MAC_ACT_DEFAULT = 0,
}
impl From<MAC_CONF_TX_MAC_ACT_A> for bool {
    #[inline(always)]
    fn from(variant: MAC_CONF_TX_MAC_ACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAC_CONF_TX_MAC_ACT`"]
pub type MAC_CONF_TX_MAC_ACT_R = crate::R<bool, MAC_CONF_TX_MAC_ACT_A>;
impl MAC_CONF_TX_MAC_ACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MAC_CONF_TX_MAC_ACT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MAC_CONF_TX_MAC_ACT_A::MAC_CONF_TX_MAC_ACT_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAC_CONF_TX_MAC_ACT_DEFAULT`"]
    #[inline(always)]
    pub fn is_mac_conf_tx_mac_act_default(&self) -> bool {
        *self == MAC_CONF_TX_MAC_ACT_A::MAC_CONF_TX_MAC_ACT_DEFAULT
    }
}
#[doc = "Write proxy for field `MAC_CONF_TX_MAC_ACT`"]
pub struct MAC_CONF_TX_MAC_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_CONF_TX_MAC_ACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAC_CONF_TX_MAC_ACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mac_conf_tx_mac_act_default(self) -> &'a mut W {
        self.variant(MAC_CONF_TX_MAC_ACT_A::MAC_CONF_TX_MAC_ACT_DEFAULT)
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
#[doc = "If set to 1, the FSM will switch to Tx after an Tx mode, Rx otherwise.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAC_CONF_TX_MAC_TX_NRX_A {
    #[doc = "0: `0`"]
    MAC_CONF_TX_MAC_TX_NRX_DEFAULT = 0,
}
impl From<MAC_CONF_TX_MAC_TX_NRX_A> for bool {
    #[inline(always)]
    fn from(variant: MAC_CONF_TX_MAC_TX_NRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAC_CONF_TX_MAC_TX_NRX`"]
pub type MAC_CONF_TX_MAC_TX_NRX_R = crate::R<bool, MAC_CONF_TX_MAC_TX_NRX_A>;
impl MAC_CONF_TX_MAC_TX_NRX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MAC_CONF_TX_MAC_TX_NRX_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MAC_CONF_TX_MAC_TX_NRX_A::MAC_CONF_TX_MAC_TX_NRX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAC_CONF_TX_MAC_TX_NRX_DEFAULT`"]
    #[inline(always)]
    pub fn is_mac_conf_tx_mac_tx_nrx_default(&self) -> bool {
        *self == MAC_CONF_TX_MAC_TX_NRX_A::MAC_CONF_TX_MAC_TX_NRX_DEFAULT
    }
}
#[doc = "Write proxy for field `MAC_CONF_TX_MAC_TX_NRX`"]
pub struct MAC_CONF_TX_MAC_TX_NRX_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_CONF_TX_MAC_TX_NRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAC_CONF_TX_MAC_TX_NRX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mac_conf_tx_mac_tx_nrx_default(self) -> &'a mut W {
        self.variant(MAC_CONF_TX_MAC_TX_NRX_A::MAC_CONF_TX_MAC_TX_NRX_DEFAULT)
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
#[doc = "If set to 1, the MAC timer is activated at beginning of the packet, otherwise at the end of the packet transmission.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAC_CONF_TX_MAC_START_NSTOP_A {
    #[doc = "0: `0`"]
    MAC_CONF_TX_MAC_START_NSTOP_DEFAULT = 0,
}
impl From<MAC_CONF_TX_MAC_START_NSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: MAC_CONF_TX_MAC_START_NSTOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAC_CONF_TX_MAC_START_NSTOP`"]
pub type MAC_CONF_TX_MAC_START_NSTOP_R = crate::R<bool, MAC_CONF_TX_MAC_START_NSTOP_A>;
impl MAC_CONF_TX_MAC_START_NSTOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MAC_CONF_TX_MAC_START_NSTOP_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MAC_CONF_TX_MAC_START_NSTOP_A::MAC_CONF_TX_MAC_START_NSTOP_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAC_CONF_TX_MAC_START_NSTOP_DEFAULT`"]
    #[inline(always)]
    pub fn is_mac_conf_tx_mac_start_nstop_default(&self) -> bool {
        *self == MAC_CONF_TX_MAC_START_NSTOP_A::MAC_CONF_TX_MAC_START_NSTOP_DEFAULT
    }
}
#[doc = "Write proxy for field `MAC_CONF_TX_MAC_START_NSTOP`"]
pub struct MAC_CONF_TX_MAC_START_NSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_CONF_TX_MAC_START_NSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAC_CONF_TX_MAC_START_NSTOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mac_conf_tx_mac_start_nstop_default(self) -> &'a mut W {
        self.variant(MAC_CONF_TX_MAC_START_NSTOP_A::MAC_CONF_TX_MAC_START_NSTOP_DEFAULT)
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
#[doc = "Configuration of GPIO pad 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD_CONF_5_PAD_9_CONF_A {
    #[doc = "0: `0`"]
    PAD_CONF_5_PAD_9_CONF_DEFAULT = 0,
}
impl From<PAD_CONF_5_PAD_9_CONF_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD_CONF_5_PAD_9_CONF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD_CONF_5_PAD_9_CONF`"]
pub type PAD_CONF_5_PAD_9_CONF_R = crate::R<u8, PAD_CONF_5_PAD_9_CONF_A>;
impl PAD_CONF_5_PAD_9_CONF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD_CONF_5_PAD_9_CONF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD_CONF_5_PAD_9_CONF_A::PAD_CONF_5_PAD_9_CONF_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PAD_CONF_5_PAD_9_CONF_DEFAULT`"]
    #[inline(always)]
    pub fn is_pad_conf_5_pad_9_conf_default(&self) -> bool {
        *self == PAD_CONF_5_PAD_9_CONF_A::PAD_CONF_5_PAD_9_CONF_DEFAULT
    }
}
#[doc = "Write proxy for field `PAD_CONF_5_PAD_9_CONF`"]
pub struct PAD_CONF_5_PAD_9_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CONF_5_PAD_9_CONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD_CONF_5_PAD_9_CONF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pad_conf_5_pad_9_conf_default(self) -> &'a mut W {
        self.variant(PAD_CONF_5_PAD_9_CONF_A::PAD_CONF_5_PAD_9_CONF_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Configuration of GPIO pad 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD_CONF_5_PAD_8_CONF_A {
    #[doc = "0: `0`"]
    PAD_CONF_5_PAD_8_CONF_DEFAULT = 0,
}
impl From<PAD_CONF_5_PAD_8_CONF_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD_CONF_5_PAD_8_CONF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD_CONF_5_PAD_8_CONF`"]
pub type PAD_CONF_5_PAD_8_CONF_R = crate::R<u8, PAD_CONF_5_PAD_8_CONF_A>;
impl PAD_CONF_5_PAD_8_CONF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD_CONF_5_PAD_8_CONF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD_CONF_5_PAD_8_CONF_A::PAD_CONF_5_PAD_8_CONF_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PAD_CONF_5_PAD_8_CONF_DEFAULT`"]
    #[inline(always)]
    pub fn is_pad_conf_5_pad_8_conf_default(&self) -> bool {
        *self == PAD_CONF_5_PAD_8_CONF_A::PAD_CONF_5_PAD_8_CONF_DEFAULT
    }
}
#[doc = "Write proxy for field `PAD_CONF_5_PAD_8_CONF`"]
pub struct PAD_CONF_5_PAD_8_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CONF_5_PAD_8_CONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD_CONF_5_PAD_8_CONF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pad_conf_5_pad_8_conf_default(self) -> &'a mut W {
        self.variant(PAD_CONF_5_PAD_8_CONF_A::PAD_CONF_5_PAD_8_CONF_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Configuration of GPIO pad 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD_CONF_4_PAD_7_CONF_A {
    #[doc = "0: `0`"]
    PAD_CONF_4_PAD_7_CONF_DEFAULT = 0,
}
impl From<PAD_CONF_4_PAD_7_CONF_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD_CONF_4_PAD_7_CONF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD_CONF_4_PAD_7_CONF`"]
pub type PAD_CONF_4_PAD_7_CONF_R = crate::R<u8, PAD_CONF_4_PAD_7_CONF_A>;
impl PAD_CONF_4_PAD_7_CONF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD_CONF_4_PAD_7_CONF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD_CONF_4_PAD_7_CONF_A::PAD_CONF_4_PAD_7_CONF_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PAD_CONF_4_PAD_7_CONF_DEFAULT`"]
    #[inline(always)]
    pub fn is_pad_conf_4_pad_7_conf_default(&self) -> bool {
        *self == PAD_CONF_4_PAD_7_CONF_A::PAD_CONF_4_PAD_7_CONF_DEFAULT
    }
}
#[doc = "Write proxy for field `PAD_CONF_4_PAD_7_CONF`"]
pub struct PAD_CONF_4_PAD_7_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CONF_4_PAD_7_CONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD_CONF_4_PAD_7_CONF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pad_conf_4_pad_7_conf_default(self) -> &'a mut W {
        self.variant(PAD_CONF_4_PAD_7_CONF_A::PAD_CONF_4_PAD_7_CONF_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Configuration of GPIO pad 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD_CONF_4_PAD_6_CONF_A {
    #[doc = "0: `0`"]
    PAD_CONF_4_PAD_6_CONF_DEFAULT = 0,
}
impl From<PAD_CONF_4_PAD_6_CONF_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD_CONF_4_PAD_6_CONF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD_CONF_4_PAD_6_CONF`"]
pub type PAD_CONF_4_PAD_6_CONF_R = crate::R<u8, PAD_CONF_4_PAD_6_CONF_A>;
impl PAD_CONF_4_PAD_6_CONF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD_CONF_4_PAD_6_CONF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD_CONF_4_PAD_6_CONF_A::PAD_CONF_4_PAD_6_CONF_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PAD_CONF_4_PAD_6_CONF_DEFAULT`"]
    #[inline(always)]
    pub fn is_pad_conf_4_pad_6_conf_default(&self) -> bool {
        *self == PAD_CONF_4_PAD_6_CONF_A::PAD_CONF_4_PAD_6_CONF_DEFAULT
    }
}
#[doc = "Write proxy for field `PAD_CONF_4_PAD_6_CONF`"]
pub struct PAD_CONF_4_PAD_6_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CONF_4_PAD_6_CONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD_CONF_4_PAD_6_CONF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pad_conf_4_pad_6_conf_default(self) -> &'a mut W {
        self.variant(PAD_CONF_4_PAD_6_CONF_A::PAD_CONF_4_PAD_6_CONF_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Configuration of GPIO pad 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD_CONF_3_PAD_5_CONF_A {
    #[doc = "0: `0`"]
    PAD_CONF_3_PAD_5_CONF_DEFAULT = 0,
}
impl From<PAD_CONF_3_PAD_5_CONF_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD_CONF_3_PAD_5_CONF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD_CONF_3_PAD_5_CONF`"]
pub type PAD_CONF_3_PAD_5_CONF_R = crate::R<u8, PAD_CONF_3_PAD_5_CONF_A>;
impl PAD_CONF_3_PAD_5_CONF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD_CONF_3_PAD_5_CONF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD_CONF_3_PAD_5_CONF_A::PAD_CONF_3_PAD_5_CONF_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PAD_CONF_3_PAD_5_CONF_DEFAULT`"]
    #[inline(always)]
    pub fn is_pad_conf_3_pad_5_conf_default(&self) -> bool {
        *self == PAD_CONF_3_PAD_5_CONF_A::PAD_CONF_3_PAD_5_CONF_DEFAULT
    }
}
#[doc = "Write proxy for field `PAD_CONF_3_PAD_5_CONF`"]
pub struct PAD_CONF_3_PAD_5_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CONF_3_PAD_5_CONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD_CONF_3_PAD_5_CONF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pad_conf_3_pad_5_conf_default(self) -> &'a mut W {
        self.variant(PAD_CONF_3_PAD_5_CONF_A::PAD_CONF_3_PAD_5_CONF_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Configuration of GPIO pad 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD_CONF_3_PAD_4_CONF_A {
    #[doc = "0: `0`"]
    PAD_CONF_3_PAD_4_CONF_DEFAULT = 0,
}
impl From<PAD_CONF_3_PAD_4_CONF_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD_CONF_3_PAD_4_CONF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD_CONF_3_PAD_4_CONF`"]
pub type PAD_CONF_3_PAD_4_CONF_R = crate::R<u8, PAD_CONF_3_PAD_4_CONF_A>;
impl PAD_CONF_3_PAD_4_CONF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD_CONF_3_PAD_4_CONF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD_CONF_3_PAD_4_CONF_A::PAD_CONF_3_PAD_4_CONF_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PAD_CONF_3_PAD_4_CONF_DEFAULT`"]
    #[inline(always)]
    pub fn is_pad_conf_3_pad_4_conf_default(&self) -> bool {
        *self == PAD_CONF_3_PAD_4_CONF_A::PAD_CONF_3_PAD_4_CONF_DEFAULT
    }
}
#[doc = "Write proxy for field `PAD_CONF_3_PAD_4_CONF`"]
pub struct PAD_CONF_3_PAD_4_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CONF_3_PAD_4_CONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD_CONF_3_PAD_4_CONF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pad_conf_3_pad_4_conf_default(self) -> &'a mut W {
        self.variant(PAD_CONF_3_PAD_4_CONF_A::PAD_CONF_3_PAD_4_CONF_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - MAC timer granularity. The granularity is given by (2^(2mac_timer_gr))x1us"]
    #[inline(always)]
    pub fn mac_conf_mac_timer_gr(&self) -> MAC_CONF_MAC_TIMER_GR_R {
        MAC_CONF_MAC_TIMER_GR_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29 - If set to 1, the FSM will switch to Rx or Tx after an Rx mode."]
    #[inline(always)]
    pub fn mac_conf_rx_mac_act(&self) -> MAC_CONF_RX_MAC_ACT_R {
        MAC_CONF_RX_MAC_ACT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - If set to 1, the FSM will switch to Tx after an Rx mode, Rx otherwise."]
    #[inline(always)]
    pub fn mac_conf_rx_mac_tx_nrx(&self) -> MAC_CONF_RX_MAC_TX_NRX_R {
        MAC_CONF_RX_MAC_TX_NRX_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - If set to 1, the MAC timer is activated at the reception of the sync word, at the end of the packet otherwise."]
    #[inline(always)]
    pub fn mac_conf_rx_mac_start_nstop(&self) -> MAC_CONF_RX_MAC_START_NSTOP_R {
        MAC_CONF_RX_MAC_START_NSTOP_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - If set to 1, the FSM will switch to Rx or Tx after a Tx mode."]
    #[inline(always)]
    pub fn mac_conf_tx_mac_act(&self) -> MAC_CONF_TX_MAC_ACT_R {
        MAC_CONF_TX_MAC_ACT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - If set to 1, the FSM will switch to Tx after an Tx mode, Rx otherwise."]
    #[inline(always)]
    pub fn mac_conf_tx_mac_tx_nrx(&self) -> MAC_CONF_TX_MAC_TX_NRX_R {
        MAC_CONF_TX_MAC_TX_NRX_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - If set to 1, the MAC timer is activated at beginning of the packet, otherwise at the end of the packet transmission."]
    #[inline(always)]
    pub fn mac_conf_tx_mac_start_nstop(&self) -> MAC_CONF_TX_MAC_START_NSTOP_R {
        MAC_CONF_TX_MAC_START_NSTOP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - Configuration of GPIO pad 9"]
    #[inline(always)]
    pub fn pad_conf_5_pad_9_conf(&self) -> PAD_CONF_5_PAD_9_CONF_R {
        PAD_CONF_5_PAD_9_CONF_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Configuration of GPIO pad 8"]
    #[inline(always)]
    pub fn pad_conf_5_pad_8_conf(&self) -> PAD_CONF_5_PAD_8_CONF_R {
        PAD_CONF_5_PAD_8_CONF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Configuration of GPIO pad 7"]
    #[inline(always)]
    pub fn pad_conf_4_pad_7_conf(&self) -> PAD_CONF_4_PAD_7_CONF_R {
        PAD_CONF_4_PAD_7_CONF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Configuration of GPIO pad 6"]
    #[inline(always)]
    pub fn pad_conf_4_pad_6_conf(&self) -> PAD_CONF_4_PAD_6_CONF_R {
        PAD_CONF_4_PAD_6_CONF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configuration of GPIO pad 5"]
    #[inline(always)]
    pub fn pad_conf_3_pad_5_conf(&self) -> PAD_CONF_3_PAD_5_CONF_R {
        PAD_CONF_3_PAD_5_CONF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Configuration of GPIO pad 4"]
    #[inline(always)]
    pub fn pad_conf_3_pad_4_conf(&self) -> PAD_CONF_3_PAD_4_CONF_R {
        PAD_CONF_3_PAD_4_CONF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - MAC timer granularity. The granularity is given by (2^(2mac_timer_gr))x1us"]
    #[inline(always)]
    pub fn mac_conf_mac_timer_gr(&mut self) -> MAC_CONF_MAC_TIMER_GR_W {
        MAC_CONF_MAC_TIMER_GR_W { w: self }
    }
    #[doc = "Bit 29 - If set to 1, the FSM will switch to Rx or Tx after an Rx mode."]
    #[inline(always)]
    pub fn mac_conf_rx_mac_act(&mut self) -> MAC_CONF_RX_MAC_ACT_W {
        MAC_CONF_RX_MAC_ACT_W { w: self }
    }
    #[doc = "Bit 28 - If set to 1, the FSM will switch to Tx after an Rx mode, Rx otherwise."]
    #[inline(always)]
    pub fn mac_conf_rx_mac_tx_nrx(&mut self) -> MAC_CONF_RX_MAC_TX_NRX_W {
        MAC_CONF_RX_MAC_TX_NRX_W { w: self }
    }
    #[doc = "Bit 27 - If set to 1, the MAC timer is activated at the reception of the sync word, at the end of the packet otherwise."]
    #[inline(always)]
    pub fn mac_conf_rx_mac_start_nstop(&mut self) -> MAC_CONF_RX_MAC_START_NSTOP_W {
        MAC_CONF_RX_MAC_START_NSTOP_W { w: self }
    }
    #[doc = "Bit 26 - If set to 1, the FSM will switch to Rx or Tx after a Tx mode."]
    #[inline(always)]
    pub fn mac_conf_tx_mac_act(&mut self) -> MAC_CONF_TX_MAC_ACT_W {
        MAC_CONF_TX_MAC_ACT_W { w: self }
    }
    #[doc = "Bit 25 - If set to 1, the FSM will switch to Tx after an Tx mode, Rx otherwise."]
    #[inline(always)]
    pub fn mac_conf_tx_mac_tx_nrx(&mut self) -> MAC_CONF_TX_MAC_TX_NRX_W {
        MAC_CONF_TX_MAC_TX_NRX_W { w: self }
    }
    #[doc = "Bit 24 - If set to 1, the MAC timer is activated at beginning of the packet, otherwise at the end of the packet transmission."]
    #[inline(always)]
    pub fn mac_conf_tx_mac_start_nstop(&mut self) -> MAC_CONF_TX_MAC_START_NSTOP_W {
        MAC_CONF_TX_MAC_START_NSTOP_W { w: self }
    }
    #[doc = "Bits 20:23 - Configuration of GPIO pad 9"]
    #[inline(always)]
    pub fn pad_conf_5_pad_9_conf(&mut self) -> PAD_CONF_5_PAD_9_CONF_W {
        PAD_CONF_5_PAD_9_CONF_W { w: self }
    }
    #[doc = "Bits 16:19 - Configuration of GPIO pad 8"]
    #[inline(always)]
    pub fn pad_conf_5_pad_8_conf(&mut self) -> PAD_CONF_5_PAD_8_CONF_W {
        PAD_CONF_5_PAD_8_CONF_W { w: self }
    }
    #[doc = "Bits 12:15 - Configuration of GPIO pad 7"]
    #[inline(always)]
    pub fn pad_conf_4_pad_7_conf(&mut self) -> PAD_CONF_4_PAD_7_CONF_W {
        PAD_CONF_4_PAD_7_CONF_W { w: self }
    }
    #[doc = "Bits 8:11 - Configuration of GPIO pad 6"]
    #[inline(always)]
    pub fn pad_conf_4_pad_6_conf(&mut self) -> PAD_CONF_4_PAD_6_CONF_W {
        PAD_CONF_4_PAD_6_CONF_W { w: self }
    }
    #[doc = "Bits 4:7 - Configuration of GPIO pad 5"]
    #[inline(always)]
    pub fn pad_conf_3_pad_5_conf(&mut self) -> PAD_CONF_3_PAD_5_CONF_W {
        PAD_CONF_3_PAD_5_CONF_W { w: self }
    }
    #[doc = "Bits 0:3 - Configuration of GPIO pad 4"]
    #[inline(always)]
    pub fn pad_conf_3_pad_4_conf(&mut self) -> PAD_CONF_3_PAD_4_CONF_W {
        PAD_CONF_3_PAD_4_CONF_W { w: self }
    }
}
