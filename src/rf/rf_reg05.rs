#[doc = "Reader of register RF_REG05"]
pub type R = crate::R<u32, super::RF_REG05>;
#[doc = "Writer for register RF_REG05"]
pub type W = crate::W<u32, super::RF_REG05>;
#[doc = "Register RF_REG05 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG05 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Switch I and Q channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL_SWITCH_IQ_A {
    #[doc = "0: `0`"]
    CHANNEL_SWITCH_IQ_DEFAULT = 0,
}
impl From<CHANNEL_SWITCH_IQ_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL_SWITCH_IQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHANNEL_SWITCH_IQ`"]
pub type CHANNEL_SWITCH_IQ_R = crate::R<bool, CHANNEL_SWITCH_IQ_A>;
impl CHANNEL_SWITCH_IQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CHANNEL_SWITCH_IQ_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CHANNEL_SWITCH_IQ_A::CHANNEL_SWITCH_IQ_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL_SWITCH_IQ_DEFAULT`"]
    #[inline(always)]
    pub fn is_channel_switch_iq_default(&self) -> bool {
        *self == CHANNEL_SWITCH_IQ_A::CHANNEL_SWITCH_IQ_DEFAULT
    }
}
#[doc = "Write proxy for field `CHANNEL_SWITCH_IQ`"]
pub struct CHANNEL_SWITCH_IQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL_SWITCH_IQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL_SWITCH_IQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn channel_switch_iq_default(self) -> &'a mut W {
        self.variant(CHANNEL_SWITCH_IQ_A::CHANNEL_SWITCH_IQ_DEFAULT)
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
#[doc = "Channel number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHANNEL_CHANNEL_A {
    #[doc = "0: `0`"]
    CHANNEL_CHANNEL_DEFAULT = 0,
}
impl From<CHANNEL_CHANNEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CHANNEL_CHANNEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHANNEL_CHANNEL`"]
pub type CHANNEL_CHANNEL_R = crate::R<u8, CHANNEL_CHANNEL_A>;
impl CHANNEL_CHANNEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CHANNEL_CHANNEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CHANNEL_CHANNEL_A::CHANNEL_CHANNEL_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL_CHANNEL_DEFAULT`"]
    #[inline(always)]
    pub fn is_channel_channel_default(&self) -> bool {
        *self == CHANNEL_CHANNEL_A::CHANNEL_CHANNEL_DEFAULT
    }
}
#[doc = "Write proxy for field `CHANNEL_CHANNEL`"]
pub struct CHANNEL_CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL_CHANNEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL_CHANNEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn channel_channel_default(self) -> &'a mut W {
        self.variant(CHANNEL_CHANNEL_A::CHANNEL_CHANNEL_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Select the data-rate register: 0-> Rx data-rate, 1-> Tx data-rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK_DATARATE_TX_NRX_A {
    #[doc = "0: `0`"]
    BANK_DATARATE_TX_NRX_DEFAULT = 0,
}
impl From<BANK_DATARATE_TX_NRX_A> for bool {
    #[inline(always)]
    fn from(variant: BANK_DATARATE_TX_NRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BANK_DATARATE_TX_NRX`"]
pub type BANK_DATARATE_TX_NRX_R = crate::R<bool, BANK_DATARATE_TX_NRX_A>;
impl BANK_DATARATE_TX_NRX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, BANK_DATARATE_TX_NRX_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(BANK_DATARATE_TX_NRX_A::BANK_DATARATE_TX_NRX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BANK_DATARATE_TX_NRX_DEFAULT`"]
    #[inline(always)]
    pub fn is_bank_datarate_tx_nrx_default(&self) -> bool {
        *self == BANK_DATARATE_TX_NRX_A::BANK_DATARATE_TX_NRX_DEFAULT
    }
}
#[doc = "Write proxy for field `BANK_DATARATE_TX_NRX`"]
pub struct BANK_DATARATE_TX_NRX_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK_DATARATE_TX_NRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BANK_DATARATE_TX_NRX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bank_datarate_tx_nrx_default(self) -> &'a mut W {
        self.variant(BANK_DATARATE_TX_NRX_A::BANK_DATARATE_TX_NRX_DEFAULT)
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
#[doc = "Select the used bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BANK_BANK_A {
    #[doc = "0: `0`"]
    BANK_BANK_DEFAULT = 0,
}
impl From<BANK_BANK_A> for u8 {
    #[inline(always)]
    fn from(variant: BANK_BANK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BANK_BANK`"]
pub type BANK_BANK_R = crate::R<u8, BANK_BANK_A>;
impl BANK_BANK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BANK_BANK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BANK_BANK_A::BANK_BANK_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BANK_BANK_DEFAULT`"]
    #[inline(always)]
    pub fn is_bank_bank_default(&self) -> bool {
        *self == BANK_BANK_A::BANK_BANK_DEFAULT
    }
}
#[doc = "Write proxy for field `BANK_BANK`"]
pub struct BANK_BANK_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK_BANK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BANK_BANK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bank_bank_default(self) -> &'a mut W {
        self.variant(BANK_BANK_A::BANK_BANK_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Time to wait after the Tx mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_MAC_TIMER_TX_MAC_TIMER_A {
    #[doc = "0: `0`"]
    TX_MAC_TIMER_TX_MAC_TIMER_DEFAULT = 0,
}
impl From<TX_MAC_TIMER_TX_MAC_TIMER_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_MAC_TIMER_TX_MAC_TIMER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_MAC_TIMER_TX_MAC_TIMER`"]
pub type TX_MAC_TIMER_TX_MAC_TIMER_R = crate::R<u8, TX_MAC_TIMER_TX_MAC_TIMER_A>;
impl TX_MAC_TIMER_TX_MAC_TIMER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_MAC_TIMER_TX_MAC_TIMER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_MAC_TIMER_TX_MAC_TIMER_A::TX_MAC_TIMER_TX_MAC_TIMER_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_MAC_TIMER_TX_MAC_TIMER_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_mac_timer_tx_mac_timer_default(&self) -> bool {
        *self == TX_MAC_TIMER_TX_MAC_TIMER_A::TX_MAC_TIMER_TX_MAC_TIMER_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_MAC_TIMER_TX_MAC_TIMER`"]
pub struct TX_MAC_TIMER_TX_MAC_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MAC_TIMER_TX_MAC_TIMER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_MAC_TIMER_TX_MAC_TIMER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_mac_timer_tx_mac_timer_default(self) -> &'a mut W {
        self.variant(TX_MAC_TIMER_TX_MAC_TIMER_A::TX_MAC_TIMER_TX_MAC_TIMER_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Time to wait after the Rx mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_MAC_TIMER_RX_MAC_TIMER_A {
    #[doc = "0: `0`"]
    RX_MAC_TIMER_RX_MAC_TIMER_DEFAULT = 0,
}
impl From<RX_MAC_TIMER_RX_MAC_TIMER_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_MAC_TIMER_RX_MAC_TIMER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RX_MAC_TIMER_RX_MAC_TIMER`"]
pub type RX_MAC_TIMER_RX_MAC_TIMER_R = crate::R<u8, RX_MAC_TIMER_RX_MAC_TIMER_A>;
impl RX_MAC_TIMER_RX_MAC_TIMER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RX_MAC_TIMER_RX_MAC_TIMER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RX_MAC_TIMER_RX_MAC_TIMER_A::RX_MAC_TIMER_RX_MAC_TIMER_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX_MAC_TIMER_RX_MAC_TIMER_DEFAULT`"]
    #[inline(always)]
    pub fn is_rx_mac_timer_rx_mac_timer_default(&self) -> bool {
        *self == RX_MAC_TIMER_RX_MAC_TIMER_A::RX_MAC_TIMER_RX_MAC_TIMER_DEFAULT
    }
}
#[doc = "Write proxy for field `RX_MAC_TIMER_RX_MAC_TIMER`"]
pub struct RX_MAC_TIMER_RX_MAC_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MAC_TIMER_RX_MAC_TIMER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_MAC_TIMER_RX_MAC_TIMER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rx_mac_timer_rx_mac_timer_default(self) -> &'a mut W {
        self.variant(RX_MAC_TIMER_RX_MAC_TIMER_A::RX_MAC_TIMER_RX_MAC_TIMER_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Switch I and Q channels"]
    #[inline(always)]
    pub fn channel_switch_iq(&self) -> CHANNEL_SWITCH_IQ_R {
        CHANNEL_SWITCH_IQ_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 24:29 - Channel number"]
    #[inline(always)]
    pub fn channel_channel(&self) -> CHANNEL_CHANNEL_R {
        CHANNEL_CHANNEL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 18 - Select the data-rate register: 0-> Rx data-rate, 1-> Tx data-rate"]
    #[inline(always)]
    pub fn bank_datarate_tx_nrx(&self) -> BANK_DATARATE_TX_NRX_R {
        BANK_DATARATE_TX_NRX_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Select the used bank"]
    #[inline(always)]
    pub fn bank_bank(&self) -> BANK_BANK_R {
        BANK_BANK_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - Time to wait after the Tx mode."]
    #[inline(always)]
    pub fn tx_mac_timer_tx_mac_timer(&self) -> TX_MAC_TIMER_TX_MAC_TIMER_R {
        TX_MAC_TIMER_TX_MAC_TIMER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Time to wait after the Rx mode."]
    #[inline(always)]
    pub fn rx_mac_timer_rx_mac_timer(&self) -> RX_MAC_TIMER_RX_MAC_TIMER_R {
        RX_MAC_TIMER_RX_MAC_TIMER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 30 - Switch I and Q channels"]
    #[inline(always)]
    pub fn channel_switch_iq(&mut self) -> CHANNEL_SWITCH_IQ_W {
        CHANNEL_SWITCH_IQ_W { w: self }
    }
    #[doc = "Bits 24:29 - Channel number"]
    #[inline(always)]
    pub fn channel_channel(&mut self) -> CHANNEL_CHANNEL_W {
        CHANNEL_CHANNEL_W { w: self }
    }
    #[doc = "Bit 18 - Select the data-rate register: 0-> Rx data-rate, 1-> Tx data-rate"]
    #[inline(always)]
    pub fn bank_datarate_tx_nrx(&mut self) -> BANK_DATARATE_TX_NRX_W {
        BANK_DATARATE_TX_NRX_W { w: self }
    }
    #[doc = "Bits 16:17 - Select the used bank"]
    #[inline(always)]
    pub fn bank_bank(&mut self) -> BANK_BANK_W {
        BANK_BANK_W { w: self }
    }
    #[doc = "Bits 8:15 - Time to wait after the Tx mode."]
    #[inline(always)]
    pub fn tx_mac_timer_tx_mac_timer(&mut self) -> TX_MAC_TIMER_TX_MAC_TIMER_W {
        TX_MAC_TIMER_TX_MAC_TIMER_W { w: self }
    }
    #[doc = "Bits 0:7 - Time to wait after the Rx mode."]
    #[inline(always)]
    pub fn rx_mac_timer_rx_mac_timer(&mut self) -> RX_MAC_TIMER_RX_MAC_TIMER_W {
        RX_MAC_TIMER_RX_MAC_TIMER_W { w: self }
    }
}
