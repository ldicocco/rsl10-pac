#[doc = "Reader of register RF_IRQ_STATUS"]
pub type R = crate::R<u32, super::RF_IRQ_STATUS>;
#[doc = "Reader of field `IRQ_STATUS_FLAG_RXFIFO`"]
pub type IRQ_STATUS_FLAG_RXFIFO_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRQ_STATUS_FLAG_TXFIFO`"]
pub type IRQ_STATUS_FLAG_TXFIFO_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRQ_STATUS_FLAG_SYNC`"]
pub type IRQ_STATUS_FLAG_SYNC_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRQ_STATUS_FLAG_RECEIVED`"]
pub type IRQ_STATUS_FLAG_RECEIVED_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRQ_STATUS_FLAG_RXSTOP`"]
pub type IRQ_STATUS_FLAG_RXSTOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRQ_STATUS_FLAG_TX`"]
pub type IRQ_STATUS_FLAG_TX_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 5 - Is set to 1 when the IRQ RXFIFO is active"]
    #[inline(always)]
    pub fn irq_status_flag_rxfifo(&self) -> IRQ_STATUS_FLAG_RXFIFO_R {
        IRQ_STATUS_FLAG_RXFIFO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Is set to 1 when the IRQ TXFIFO is active"]
    #[inline(always)]
    pub fn irq_status_flag_txfifo(&self) -> IRQ_STATUS_FLAG_TXFIFO_R {
        IRQ_STATUS_FLAG_TXFIFO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Is set to 1 when the IRQ SYNC is active"]
    #[inline(always)]
    pub fn irq_status_flag_sync(&self) -> IRQ_STATUS_FLAG_SYNC_R {
        IRQ_STATUS_FLAG_SYNC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Is set to 1 when the IRQ RECEIVED is active"]
    #[inline(always)]
    pub fn irq_status_flag_received(&self) -> IRQ_STATUS_FLAG_RECEIVED_R {
        IRQ_STATUS_FLAG_RECEIVED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Is set to 1 when the IRQ RXSTOP is active"]
    #[inline(always)]
    pub fn irq_status_flag_rxstop(&self) -> IRQ_STATUS_FLAG_RXSTOP_R {
        IRQ_STATUS_FLAG_RXSTOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Is set to 1 when the IRQ TX is active"]
    #[inline(always)]
    pub fn irq_status_flag_tx(&self) -> IRQ_STATUS_FLAG_TX_R {
        IRQ_STATUS_FLAG_TX_R::new((self.bits & 0x01) != 0)
    }
}
