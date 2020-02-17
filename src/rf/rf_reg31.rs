#[doc = "Reader of register RF_REG31"]
pub type R = crate::R<u32, super::RF_REG31>;
#[doc = "Reader of field `RSSI_MAX_RSSI_MAX`"]
pub type RSSI_MAX_RSSI_MAX_R = crate::R<u8, u8>;
#[doc = "Reader of field `RSSI_MIN_RSSI_MIN`"]
pub type RSSI_MIN_RSSI_MIN_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXFIFO_COUNT_RX_COUNT`"]
pub type RXFIFO_COUNT_RX_COUNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `TXFIFO_COUNT_TX_COUNT`"]
pub type TXFIFO_COUNT_TX_COUNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - Maximum RSSI value over a filtering period"]
    #[inline(always)]
    pub fn rssi_max_rssi_max(&self) -> RSSI_MAX_RSSI_MAX_R {
        RSSI_MAX_RSSI_MAX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Minimum RSSI value over a filtering period"]
    #[inline(always)]
    pub fn rssi_min_rssi_min(&self) -> RSSI_MIN_RSSI_MIN_R {
        RSSI_MIN_RSSI_MIN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of bytes in the Rx FIFO"]
    #[inline(always)]
    pub fn rxfifo_count_rx_count(&self) -> RXFIFO_COUNT_RX_COUNT_R {
        RXFIFO_COUNT_RX_COUNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Number of bytes in the Tx FIFO"]
    #[inline(always)]
    pub fn txfifo_count_tx_count(&self) -> TXFIFO_COUNT_TX_COUNT_R {
        TXFIFO_COUNT_TX_COUNT_R::new((self.bits & 0xff) as u8)
    }
}
