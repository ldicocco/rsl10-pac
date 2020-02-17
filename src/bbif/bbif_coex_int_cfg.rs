#[doc = "Reader of register BBIF_COEX_INT_CFG"]
pub type R = crate::R<u32, super::BBIF_COEX_INT_CFG>;
#[doc = "Writer for register BBIF_COEX_INT_CFG"]
pub type W = crate::W<u32, super::BBIF_COEX_INT_CFG>;
#[doc = "Register BBIF_COEX_INT_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::BBIF_COEX_INT_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "BLE_IN_PROCESS event interrupt configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLE_IN_PROCESS_EVENT_A {
    #[doc = "0: Interrupt not triggered"]
    BLE_IN_PROCESS_EVENT_NONE = 0,
    #[doc = "1: Interrupt triggered on rising edge"]
    BLE_IN_PROCESS_EVENT_RISING_EDGE = 1,
    #[doc = "2: Interrupt triggered on falling edge"]
    BLE_IN_PROCESS_EVENT_FALLING_EDGE = 2,
    #[doc = "3: Interrupt triggered on any edge"]
    BLE_IN_PROCESS_EVENT_TRANSITION = 3,
}
impl From<BLE_IN_PROCESS_EVENT_A> for u8 {
    #[inline(always)]
    fn from(variant: BLE_IN_PROCESS_EVENT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLE_IN_PROCESS_EVENT`"]
pub type BLE_IN_PROCESS_EVENT_R = crate::R<u8, BLE_IN_PROCESS_EVENT_A>;
impl BLE_IN_PROCESS_EVENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLE_IN_PROCESS_EVENT_A {
        match self.bits {
            0 => BLE_IN_PROCESS_EVENT_A::BLE_IN_PROCESS_EVENT_NONE,
            1 => BLE_IN_PROCESS_EVENT_A::BLE_IN_PROCESS_EVENT_RISING_EDGE,
            2 => BLE_IN_PROCESS_EVENT_A::BLE_IN_PROCESS_EVENT_FALLING_EDGE,
            3 => BLE_IN_PROCESS_EVENT_A::BLE_IN_PROCESS_EVENT_TRANSITION,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BLE_IN_PROCESS_EVENT_NONE`"]
    #[inline(always)]
    pub fn is_ble_in_process_event_none(&self) -> bool {
        *self == BLE_IN_PROCESS_EVENT_A::BLE_IN_PROCESS_EVENT_NONE
    }
    #[doc = "Checks if the value of the field is `BLE_IN_PROCESS_EVENT_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_ble_in_process_event_rising_edge(&self) -> bool {
        *self == BLE_IN_PROCESS_EVENT_A::BLE_IN_PROCESS_EVENT_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `BLE_IN_PROCESS_EVENT_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_ble_in_process_event_falling_edge(&self) -> bool {
        *self == BLE_IN_PROCESS_EVENT_A::BLE_IN_PROCESS_EVENT_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `BLE_IN_PROCESS_EVENT_TRANSITION`"]
    #[inline(always)]
    pub fn is_ble_in_process_event_transition(&self) -> bool {
        *self == BLE_IN_PROCESS_EVENT_A::BLE_IN_PROCESS_EVENT_TRANSITION
    }
}
#[doc = "Write proxy for field `BLE_IN_PROCESS_EVENT`"]
pub struct BLE_IN_PROCESS_EVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_IN_PROCESS_EVENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLE_IN_PROCESS_EVENT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt not triggered"]
    #[inline(always)]
    pub fn ble_in_process_event_none(self) -> &'a mut W {
        self.variant(BLE_IN_PROCESS_EVENT_A::BLE_IN_PROCESS_EVENT_NONE)
    }
    #[doc = "Interrupt triggered on rising edge"]
    #[inline(always)]
    pub fn ble_in_process_event_rising_edge(self) -> &'a mut W {
        self.variant(BLE_IN_PROCESS_EVENT_A::BLE_IN_PROCESS_EVENT_RISING_EDGE)
    }
    #[doc = "Interrupt triggered on falling edge"]
    #[inline(always)]
    pub fn ble_in_process_event_falling_edge(self) -> &'a mut W {
        self.variant(BLE_IN_PROCESS_EVENT_A::BLE_IN_PROCESS_EVENT_FALLING_EDGE)
    }
    #[doc = "Interrupt triggered on any edge"]
    #[inline(always)]
    pub fn ble_in_process_event_transition(self) -> &'a mut W {
        self.variant(BLE_IN_PROCESS_EVENT_A::BLE_IN_PROCESS_EVENT_TRANSITION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "BLE_TX event interrupt configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLE_TX_EVENT_A {
    #[doc = "0: Interrupt not triggered"]
    BLE_TX_EVENT_NONE = 0,
    #[doc = "1: Interrupt triggered on rising edge"]
    BLE_TX_EVENT_RISING_EDGE = 1,
    #[doc = "2: Interrupt triggered on falling edge"]
    BLE_TX_EVENT_FALLING_EDGE = 2,
    #[doc = "3: Interrupt triggered on any edge"]
    BLE_TX_EVENT_TRANSITION = 3,
}
impl From<BLE_TX_EVENT_A> for u8 {
    #[inline(always)]
    fn from(variant: BLE_TX_EVENT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLE_TX_EVENT`"]
pub type BLE_TX_EVENT_R = crate::R<u8, BLE_TX_EVENT_A>;
impl BLE_TX_EVENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLE_TX_EVENT_A {
        match self.bits {
            0 => BLE_TX_EVENT_A::BLE_TX_EVENT_NONE,
            1 => BLE_TX_EVENT_A::BLE_TX_EVENT_RISING_EDGE,
            2 => BLE_TX_EVENT_A::BLE_TX_EVENT_FALLING_EDGE,
            3 => BLE_TX_EVENT_A::BLE_TX_EVENT_TRANSITION,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BLE_TX_EVENT_NONE`"]
    #[inline(always)]
    pub fn is_ble_tx_event_none(&self) -> bool {
        *self == BLE_TX_EVENT_A::BLE_TX_EVENT_NONE
    }
    #[doc = "Checks if the value of the field is `BLE_TX_EVENT_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_ble_tx_event_rising_edge(&self) -> bool {
        *self == BLE_TX_EVENT_A::BLE_TX_EVENT_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `BLE_TX_EVENT_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_ble_tx_event_falling_edge(&self) -> bool {
        *self == BLE_TX_EVENT_A::BLE_TX_EVENT_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `BLE_TX_EVENT_TRANSITION`"]
    #[inline(always)]
    pub fn is_ble_tx_event_transition(&self) -> bool {
        *self == BLE_TX_EVENT_A::BLE_TX_EVENT_TRANSITION
    }
}
#[doc = "Write proxy for field `BLE_TX_EVENT`"]
pub struct BLE_TX_EVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_TX_EVENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLE_TX_EVENT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt not triggered"]
    #[inline(always)]
    pub fn ble_tx_event_none(self) -> &'a mut W {
        self.variant(BLE_TX_EVENT_A::BLE_TX_EVENT_NONE)
    }
    #[doc = "Interrupt triggered on rising edge"]
    #[inline(always)]
    pub fn ble_tx_event_rising_edge(self) -> &'a mut W {
        self.variant(BLE_TX_EVENT_A::BLE_TX_EVENT_RISING_EDGE)
    }
    #[doc = "Interrupt triggered on falling edge"]
    #[inline(always)]
    pub fn ble_tx_event_falling_edge(self) -> &'a mut W {
        self.variant(BLE_TX_EVENT_A::BLE_TX_EVENT_FALLING_EDGE)
    }
    #[doc = "Interrupt triggered on any edge"]
    #[inline(always)]
    pub fn ble_tx_event_transition(self) -> &'a mut W {
        self.variant(BLE_TX_EVENT_A::BLE_TX_EVENT_TRANSITION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "BLE_RX event interrupt configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLE_RX_EVENT_A {
    #[doc = "0: Interrupt not triggered"]
    BLE_RX_EVENT_NONE = 0,
    #[doc = "1: Interrupt triggered on rising edge"]
    BLE_RX_EVENT_RISING_EDGE = 1,
    #[doc = "2: Interrupt triggered on falling edge"]
    BLE_RX_EVENT_FALLING_EDGE = 2,
    #[doc = "3: Interrupt triggered on any edge"]
    BLE_RX_EVENT_TRANSITION = 3,
}
impl From<BLE_RX_EVENT_A> for u8 {
    #[inline(always)]
    fn from(variant: BLE_RX_EVENT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLE_RX_EVENT`"]
pub type BLE_RX_EVENT_R = crate::R<u8, BLE_RX_EVENT_A>;
impl BLE_RX_EVENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLE_RX_EVENT_A {
        match self.bits {
            0 => BLE_RX_EVENT_A::BLE_RX_EVENT_NONE,
            1 => BLE_RX_EVENT_A::BLE_RX_EVENT_RISING_EDGE,
            2 => BLE_RX_EVENT_A::BLE_RX_EVENT_FALLING_EDGE,
            3 => BLE_RX_EVENT_A::BLE_RX_EVENT_TRANSITION,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BLE_RX_EVENT_NONE`"]
    #[inline(always)]
    pub fn is_ble_rx_event_none(&self) -> bool {
        *self == BLE_RX_EVENT_A::BLE_RX_EVENT_NONE
    }
    #[doc = "Checks if the value of the field is `BLE_RX_EVENT_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_ble_rx_event_rising_edge(&self) -> bool {
        *self == BLE_RX_EVENT_A::BLE_RX_EVENT_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `BLE_RX_EVENT_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_ble_rx_event_falling_edge(&self) -> bool {
        *self == BLE_RX_EVENT_A::BLE_RX_EVENT_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `BLE_RX_EVENT_TRANSITION`"]
    #[inline(always)]
    pub fn is_ble_rx_event_transition(&self) -> bool {
        *self == BLE_RX_EVENT_A::BLE_RX_EVENT_TRANSITION
    }
}
#[doc = "Write proxy for field `BLE_RX_EVENT`"]
pub struct BLE_RX_EVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_RX_EVENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLE_RX_EVENT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt not triggered"]
    #[inline(always)]
    pub fn ble_rx_event_none(self) -> &'a mut W {
        self.variant(BLE_RX_EVENT_A::BLE_RX_EVENT_NONE)
    }
    #[doc = "Interrupt triggered on rising edge"]
    #[inline(always)]
    pub fn ble_rx_event_rising_edge(self) -> &'a mut W {
        self.variant(BLE_RX_EVENT_A::BLE_RX_EVENT_RISING_EDGE)
    }
    #[doc = "Interrupt triggered on falling edge"]
    #[inline(always)]
    pub fn ble_rx_event_falling_edge(self) -> &'a mut W {
        self.variant(BLE_RX_EVENT_A::BLE_RX_EVENT_FALLING_EDGE)
    }
    #[doc = "Interrupt triggered on any edge"]
    #[inline(always)]
    pub fn ble_rx_event_transition(self) -> &'a mut W {
        self.variant(BLE_RX_EVENT_A::BLE_RX_EVENT_TRANSITION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - BLE_IN_PROCESS event interrupt configuration"]
    #[inline(always)]
    pub fn ble_in_process_event(&self) -> BLE_IN_PROCESS_EVENT_R {
        BLE_IN_PROCESS_EVENT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - BLE_TX event interrupt configuration"]
    #[inline(always)]
    pub fn ble_tx_event(&self) -> BLE_TX_EVENT_R {
        BLE_TX_EVENT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - BLE_RX event interrupt configuration"]
    #[inline(always)]
    pub fn ble_rx_event(&self) -> BLE_RX_EVENT_R {
        BLE_RX_EVENT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - BLE_IN_PROCESS event interrupt configuration"]
    #[inline(always)]
    pub fn ble_in_process_event(&mut self) -> BLE_IN_PROCESS_EVENT_W {
        BLE_IN_PROCESS_EVENT_W { w: self }
    }
    #[doc = "Bits 4:5 - BLE_TX event interrupt configuration"]
    #[inline(always)]
    pub fn ble_tx_event(&mut self) -> BLE_TX_EVENT_W {
        BLE_TX_EVENT_W { w: self }
    }
    #[doc = "Bits 0:1 - BLE_RX event interrupt configuration"]
    #[inline(always)]
    pub fn ble_rx_event(&mut self) -> BLE_RX_EVENT_W {
        BLE_RX_EVENT_W { w: self }
    }
}
