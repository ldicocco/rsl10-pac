#[doc = "Reader of register UART_RX_DATA"]
pub type R = crate::R<u32, super::UART_RX_DATA>;
#[doc = "Reader of field `UART_RX_DATA`"]
pub type UART_RX_DATA_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - UART Received data"]
    #[inline(always)]
    pub fn uart_rx_data(&self) -> UART_RX_DATA_R {
        UART_RX_DATA_R::new((self.bits & 0xff) as u8)
    }
}
