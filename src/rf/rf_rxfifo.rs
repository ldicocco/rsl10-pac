#[doc = "Reader of register RF_RXFIFO"]
pub type R = crate::R<u32, super::RF_RXFIFO>;
#[doc = "Reader of field `RXFIFO_RX_DATA`"]
pub type RXFIFO_RX_DATA_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Received data"]
    #[inline(always)]
    pub fn rxfifo_rx_data(&self) -> RXFIFO_RX_DATA_R {
        RXFIFO_RX_DATA_R::new((self.bits & 0xff) as u8)
    }
}
