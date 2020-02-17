#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Configuration Register"]
    pub uart_cfg: UART_CFG,
    #[doc = "0x04 - UART Transmit Data Register"]
    pub uart_tx_data: UART_TX_DATA,
    #[doc = "0x08 - UART Receive Data Register"]
    pub uart_rx_data: UART_RX_DATA,
    #[doc = "0x0c - UART Status Register"]
    pub uart_status: UART_STATUS,
}
#[doc = "UART Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_cfg](uart_cfg) module"]
pub type UART_CFG = crate::Reg<u32, _UART_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_CFG;
#[doc = "`read()` method returns [uart_cfg::R](uart_cfg::R) reader structure"]
impl crate::Readable for UART_CFG {}
#[doc = "`write(|w| ..)` method takes [uart_cfg::W](uart_cfg::W) writer structure"]
impl crate::Writable for UART_CFG {}
#[doc = "UART Configuration Register"]
pub mod uart_cfg;
#[doc = "UART Transmit Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_tx_data](uart_tx_data) module"]
pub type UART_TX_DATA = crate::Reg<u32, _UART_TX_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_TX_DATA;
#[doc = "`read()` method returns [uart_tx_data::R](uart_tx_data::R) reader structure"]
impl crate::Readable for UART_TX_DATA {}
#[doc = "`write(|w| ..)` method takes [uart_tx_data::W](uart_tx_data::W) writer structure"]
impl crate::Writable for UART_TX_DATA {}
#[doc = "UART Transmit Data Register"]
pub mod uart_tx_data;
#[doc = "UART Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_rx_data](uart_rx_data) module"]
pub type UART_RX_DATA = crate::Reg<u32, _UART_RX_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_RX_DATA;
#[doc = "`read()` method returns [uart_rx_data::R](uart_rx_data::R) reader structure"]
impl crate::Readable for UART_RX_DATA {}
#[doc = "UART Receive Data Register"]
pub mod uart_rx_data;
#[doc = "UART Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_status](uart_status) module"]
pub type UART_STATUS = crate::Reg<u32, _UART_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_STATUS;
#[doc = "`read()` method returns [uart_status::R](uart_status::R) reader structure"]
impl crate::Readable for UART_STATUS {}
#[doc = "`write(|w| ..)` method takes [uart_status::W](uart_status::W) writer structure"]
impl crate::Writable for UART_STATUS {}
#[doc = "UART Status Register"]
pub mod uart_status;
