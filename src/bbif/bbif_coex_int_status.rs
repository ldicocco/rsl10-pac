#[doc = "Reader of register BBIF_COEX_INT_STATUS"]
pub type R = crate::R<u32, super::BBIF_COEX_INT_STATUS>;
#[doc = "Indicates if a BLE_TX_EVENT interrupt has been generated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLE_TX_EVENT_FLAG_A {
    #[doc = "0: No BLE_TX_EVENT interrupt has been generated"]
    BLE_TX_EVENT_NO_INT = 0,
    #[doc = "1: A BLE_TX_EVENT interrupt has been generated"]
    BLE_TX_EVENT_INT = 1,
}
impl From<BLE_TX_EVENT_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: BLE_TX_EVENT_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLE_TX_EVENT_FLAG`"]
pub type BLE_TX_EVENT_FLAG_R = crate::R<bool, BLE_TX_EVENT_FLAG_A>;
impl BLE_TX_EVENT_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLE_TX_EVENT_FLAG_A {
        match self.bits {
            false => BLE_TX_EVENT_FLAG_A::BLE_TX_EVENT_NO_INT,
            true => BLE_TX_EVENT_FLAG_A::BLE_TX_EVENT_INT,
        }
    }
    #[doc = "Checks if the value of the field is `BLE_TX_EVENT_NO_INT`"]
    #[inline(always)]
    pub fn is_ble_tx_event_no_int(&self) -> bool {
        *self == BLE_TX_EVENT_FLAG_A::BLE_TX_EVENT_NO_INT
    }
    #[doc = "Checks if the value of the field is `BLE_TX_EVENT_INT`"]
    #[inline(always)]
    pub fn is_ble_tx_event_int(&self) -> bool {
        *self == BLE_TX_EVENT_FLAG_A::BLE_TX_EVENT_INT
    }
}
#[doc = "Indicates if a BLE_RX_EVENT interrupt has been generated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLE_RX_EVENT_FLAG_A {
    #[doc = "0: No BLE_RX_EVENT interrupt has been generated"]
    BLE_RX_EVENT_NO_INT = 0,
    #[doc = "1: A BLE_RX_EVENT interrupt has been generated"]
    BLE_RX_EVENT_INT = 1,
}
impl From<BLE_RX_EVENT_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: BLE_RX_EVENT_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLE_RX_EVENT_FLAG`"]
pub type BLE_RX_EVENT_FLAG_R = crate::R<bool, BLE_RX_EVENT_FLAG_A>;
impl BLE_RX_EVENT_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLE_RX_EVENT_FLAG_A {
        match self.bits {
            false => BLE_RX_EVENT_FLAG_A::BLE_RX_EVENT_NO_INT,
            true => BLE_RX_EVENT_FLAG_A::BLE_RX_EVENT_INT,
        }
    }
    #[doc = "Checks if the value of the field is `BLE_RX_EVENT_NO_INT`"]
    #[inline(always)]
    pub fn is_ble_rx_event_no_int(&self) -> bool {
        *self == BLE_RX_EVENT_FLAG_A::BLE_RX_EVENT_NO_INT
    }
    #[doc = "Checks if the value of the field is `BLE_RX_EVENT_INT`"]
    #[inline(always)]
    pub fn is_ble_rx_event_int(&self) -> bool {
        *self == BLE_RX_EVENT_FLAG_A::BLE_RX_EVENT_INT
    }
}
impl R {
    #[doc = "Bit 4 - Indicates if a BLE_TX_EVENT interrupt has been generated"]
    #[inline(always)]
    pub fn ble_tx_event_flag(&self) -> BLE_TX_EVENT_FLAG_R {
        BLE_TX_EVENT_FLAG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indicates if a BLE_RX_EVENT interrupt has been generated"]
    #[inline(always)]
    pub fn ble_rx_event_flag(&self) -> BLE_RX_EVENT_FLAG_R {
        BLE_RX_EVENT_FLAG_R::new((self.bits & 0x01) != 0)
    }
}
