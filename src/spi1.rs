#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI1 Control and Configuration Register"]
    pub spi1_ctrl0: SPI1_CTRL0,
    #[doc = "0x04 - SPI1 Transaction Control Register"]
    pub spi1_ctrl1: SPI1_CTRL1,
    #[doc = "0x08 - SPI1 Transmit Data"]
    pub spi1_tx_data: SPI1_TX_DATA,
    #[doc = "0x0c - SPI1 Received Data"]
    pub spi1_rx_data: SPI1_RX_DATA,
    #[doc = "0x10 - SPI1 Status"]
    pub spi1_status: SPI1_STATUS,
}
#[doc = "SPI1 Control and Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi1_ctrl0](spi1_ctrl0) module"]
pub type SPI1_CTRL0 = crate::Reg<u32, _SPI1_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI1_CTRL0;
#[doc = "`read()` method returns [spi1_ctrl0::R](spi1_ctrl0::R) reader structure"]
impl crate::Readable for SPI1_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [spi1_ctrl0::W](spi1_ctrl0::W) writer structure"]
impl crate::Writable for SPI1_CTRL0 {}
#[doc = "SPI1 Control and Configuration Register"]
pub mod spi1_ctrl0;
#[doc = "SPI1 Transaction Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi1_ctrl1](spi1_ctrl1) module"]
pub type SPI1_CTRL1 = crate::Reg<u32, _SPI1_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI1_CTRL1;
#[doc = "`read()` method returns [spi1_ctrl1::R](spi1_ctrl1::R) reader structure"]
impl crate::Readable for SPI1_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [spi1_ctrl1::W](spi1_ctrl1::W) writer structure"]
impl crate::Writable for SPI1_CTRL1 {}
#[doc = "SPI1 Transaction Control Register"]
pub mod spi1_ctrl1;
#[doc = "SPI1 Transmit Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi1_tx_data](spi1_tx_data) module"]
pub type SPI1_TX_DATA = crate::Reg<u32, _SPI1_TX_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI1_TX_DATA;
#[doc = "`read()` method returns [spi1_tx_data::R](spi1_tx_data::R) reader structure"]
impl crate::Readable for SPI1_TX_DATA {}
#[doc = "`write(|w| ..)` method takes [spi1_tx_data::W](spi1_tx_data::W) writer structure"]
impl crate::Writable for SPI1_TX_DATA {}
#[doc = "SPI1 Transmit Data"]
pub mod spi1_tx_data;
#[doc = "SPI1 Received Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi1_rx_data](spi1_rx_data) module"]
pub type SPI1_RX_DATA = crate::Reg<u32, _SPI1_RX_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI1_RX_DATA;
#[doc = "`read()` method returns [spi1_rx_data::R](spi1_rx_data::R) reader structure"]
impl crate::Readable for SPI1_RX_DATA {}
#[doc = "SPI1 Received Data"]
pub mod spi1_rx_data;
#[doc = "SPI1 Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi1_status](spi1_status) module"]
pub type SPI1_STATUS = crate::Reg<u32, _SPI1_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI1_STATUS;
#[doc = "`read()` method returns [spi1_status::R](spi1_status::R) reader structure"]
impl crate::Readable for SPI1_STATUS {}
#[doc = "`write(|w| ..)` method takes [spi1_status::W](spi1_status::W) writer structure"]
impl crate::Writable for SPI1_STATUS {}
#[doc = "SPI1 Status"]
pub mod spi1_status;
