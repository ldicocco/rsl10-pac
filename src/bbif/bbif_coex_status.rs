#[doc = "Reader of register BBIF_COEX_STATUS"]
pub type R = crate::R<u32, super::BBIF_COEX_STATUS>;
#[doc = "Indicates the priority level of the current RW-BLE core activity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLE_PTI_A {
    #[doc = "0: BLE_PTI lowest priority"]
    BLE_PTI_PRIORITY_0 = 0,
    #[doc = "15: BLE_PTI highest priority"]
    BLE_PTI_PRIORITY_15 = 15,
}
impl From<BLE_PTI_A> for u8 {
    #[inline(always)]
    fn from(variant: BLE_PTI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLE_PTI`"]
pub type BLE_PTI_R = crate::R<u8, BLE_PTI_A>;
impl BLE_PTI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BLE_PTI_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BLE_PTI_A::BLE_PTI_PRIORITY_0),
            15 => Val(BLE_PTI_A::BLE_PTI_PRIORITY_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLE_PTI_PRIORITY_0`"]
    #[inline(always)]
    pub fn is_ble_pti_priority_0(&self) -> bool {
        *self == BLE_PTI_A::BLE_PTI_PRIORITY_0
    }
    #[doc = "Checks if the value of the field is `BLE_PTI_PRIORITY_15`"]
    #[inline(always)]
    pub fn is_ble_pti_priority_15(&self) -> bool {
        *self == BLE_PTI_A::BLE_PTI_PRIORITY_15
    }
}
#[doc = "Indicate if the RW-BLE core has an event in process, active high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLE_IN_PROCESS_A {
    #[doc = "0: RW-BLE processes no event"]
    BLE_IDLE = 0,
    #[doc = "1: RW-BLE processes an event"]
    BLE_IN_PROCESS = 1,
}
impl From<BLE_IN_PROCESS_A> for bool {
    #[inline(always)]
    fn from(variant: BLE_IN_PROCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLE_IN_PROCESS`"]
pub type BLE_IN_PROCESS_R = crate::R<bool, BLE_IN_PROCESS_A>;
impl BLE_IN_PROCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLE_IN_PROCESS_A {
        match self.bits {
            false => BLE_IN_PROCESS_A::BLE_IDLE,
            true => BLE_IN_PROCESS_A::BLE_IN_PROCESS,
        }
    }
    #[doc = "Checks if the value of the field is `BLE_IDLE`"]
    #[inline(always)]
    pub fn is_ble_idle(&self) -> bool {
        *self == BLE_IN_PROCESS_A::BLE_IDLE
    }
    #[doc = "Checks if the value of the field is `BLE_IN_PROCESS`"]
    #[inline(always)]
    pub fn is_ble_in_process(&self) -> bool {
        *self == BLE_IN_PROCESS_A::BLE_IN_PROCESS
    }
}
#[doc = "Indicates if the RW-BLE core is busy and performs Tx activity, active high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLE_TX_A {
    #[doc = "0: RW-BLE core performs no Tx activity"]
    BLE_TX_IDLE = 0,
    #[doc = "1: RW-BLE core performs Tx activity"]
    BLE_TX_BUSY = 1,
}
impl From<BLE_TX_A> for bool {
    #[inline(always)]
    fn from(variant: BLE_TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLE_TX`"]
pub type BLE_TX_R = crate::R<bool, BLE_TX_A>;
impl BLE_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLE_TX_A {
        match self.bits {
            false => BLE_TX_A::BLE_TX_IDLE,
            true => BLE_TX_A::BLE_TX_BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `BLE_TX_IDLE`"]
    #[inline(always)]
    pub fn is_ble_tx_idle(&self) -> bool {
        *self == BLE_TX_A::BLE_TX_IDLE
    }
    #[doc = "Checks if the value of the field is `BLE_TX_BUSY`"]
    #[inline(always)]
    pub fn is_ble_tx_busy(&self) -> bool {
        *self == BLE_TX_A::BLE_TX_BUSY
    }
}
#[doc = "Indicates if the RW-BLE core is busy and performs Rx activity, active high\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLE_RX_A {
    #[doc = "0: RW-BLE core performs no Rx activity"]
    BLE_RX_IDLE = 0,
    #[doc = "1: RW-BLE core performs Rx activity"]
    BLE_RX_BUSY = 1,
}
impl From<BLE_RX_A> for bool {
    #[inline(always)]
    fn from(variant: BLE_RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLE_RX`"]
pub type BLE_RX_R = crate::R<bool, BLE_RX_A>;
impl BLE_RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLE_RX_A {
        match self.bits {
            false => BLE_RX_A::BLE_RX_IDLE,
            true => BLE_RX_A::BLE_RX_BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `BLE_RX_IDLE`"]
    #[inline(always)]
    pub fn is_ble_rx_idle(&self) -> bool {
        *self == BLE_RX_A::BLE_RX_IDLE
    }
    #[doc = "Checks if the value of the field is `BLE_RX_BUSY`"]
    #[inline(always)]
    pub fn is_ble_rx_busy(&self) -> bool {
        *self == BLE_RX_A::BLE_RX_BUSY
    }
}
impl R {
    #[doc = "Bits 12:15 - Indicates the priority level of the current RW-BLE core activity"]
    #[inline(always)]
    pub fn ble_pti(&self) -> BLE_PTI_R {
        BLE_PTI_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Indicate if the RW-BLE core has an event in process, active high."]
    #[inline(always)]
    pub fn ble_in_process(&self) -> BLE_IN_PROCESS_R {
        BLE_IN_PROCESS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates if the RW-BLE core is busy and performs Tx activity, active high."]
    #[inline(always)]
    pub fn ble_tx(&self) -> BLE_TX_R {
        BLE_TX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indicates if the RW-BLE core is busy and performs Rx activity, active high"]
    #[inline(always)]
    pub fn ble_rx(&self) -> BLE_RX_R {
        BLE_RX_R::new((self.bits & 0x01) != 0)
    }
}
