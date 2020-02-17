#[doc = "Reader of register SPI0_RX_DATA"]
pub type R = crate::R<u32, super::SPI0_RX_DATA>;
#[doc = "Reader of field `SPI0_RX_DATA`"]
pub type SPI0_RX_DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Single word buffer for data that has been received over the SPI interface"]
    #[inline(always)]
    pub fn spi0_rx_data(&self) -> SPI0_RX_DATA_R {
        SPI0_RX_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
