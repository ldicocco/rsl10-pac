#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI0 Control and Configuration Register"]
    pub spi0_ctrl0: SPI0_CTRL0,
    #[doc = "0x04 - SPI0 Transaction Control Register"]
    pub spi0_ctrl1: SPI0_CTRL1,
    #[doc = "0x08 - SPI0 Transmit Data"]
    pub spi0_tx_data: SPI0_TX_DATA,
    #[doc = "0x0c - SPI0 Received Data"]
    pub spi0_rx_data: SPI0_RX_DATA,
    #[doc = "0x10 - SPI0 Status"]
    pub spi0_status: SPI0_STATUS,
}
#[doc = "SPI0 Control and Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi0_ctrl0](spi0_ctrl0) module"]
pub type SPI0_CTRL0 = crate::Reg<u32, _SPI0_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI0_CTRL0;
#[doc = "`read()` method returns [spi0_ctrl0::R](spi0_ctrl0::R) reader structure"]
impl crate::Readable for SPI0_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [spi0_ctrl0::W](spi0_ctrl0::W) writer structure"]
impl crate::Writable for SPI0_CTRL0 {}
#[doc = "SPI0 Control and Configuration Register"]
pub mod spi0_ctrl0;
#[doc = "SPI0 Transaction Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi0_ctrl1](spi0_ctrl1) module"]
pub type SPI0_CTRL1 = crate::Reg<u32, _SPI0_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI0_CTRL1;
#[doc = "`read()` method returns [spi0_ctrl1::R](spi0_ctrl1::R) reader structure"]
impl crate::Readable for SPI0_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [spi0_ctrl1::W](spi0_ctrl1::W) writer structure"]
impl crate::Writable for SPI0_CTRL1 {}
#[doc = "SPI0 Transaction Control Register"]
pub mod spi0_ctrl1;
#[doc = "SPI0 Transmit Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi0_tx_data](spi0_tx_data) module"]
pub type SPI0_TX_DATA = crate::Reg<u32, _SPI0_TX_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI0_TX_DATA;
#[doc = "`read()` method returns [spi0_tx_data::R](spi0_tx_data::R) reader structure"]
impl crate::Readable for SPI0_TX_DATA {}
#[doc = "`write(|w| ..)` method takes [spi0_tx_data::W](spi0_tx_data::W) writer structure"]
impl crate::Writable for SPI0_TX_DATA {}
#[doc = "SPI0 Transmit Data"]
pub mod spi0_tx_data;
#[doc = "SPI0 Received Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi0_rx_data](spi0_rx_data) module"]
pub type SPI0_RX_DATA = crate::Reg<u32, _SPI0_RX_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI0_RX_DATA;
#[doc = "`read()` method returns [spi0_rx_data::R](spi0_rx_data::R) reader structure"]
impl crate::Readable for SPI0_RX_DATA {}
#[doc = "SPI0 Received Data"]
pub mod spi0_rx_data;
#[doc = "SPI0 Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi0_status](spi0_status) module"]
pub type SPI0_STATUS = crate::Reg<u32, _SPI0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI0_STATUS;
#[doc = "`read()` method returns [spi0_status::R](spi0_status::R) reader structure"]
impl crate::Readable for SPI0_STATUS {}
#[doc = "`write(|w| ..)` method takes [spi0_status::W](spi0_status::W) writer structure"]
impl crate::Writable for SPI0_STATUS {}
#[doc = "SPI0 Status"]
pub mod spi0_status;
