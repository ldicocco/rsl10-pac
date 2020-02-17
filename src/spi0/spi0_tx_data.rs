#[doc = "Reader of register SPI0_TX_DATA"]
pub type R = crate::R<u32, super::SPI0_TX_DATA>;
#[doc = "Writer for register SPI0_TX_DATA"]
pub type W = crate::W<u32, super::SPI0_TX_DATA>;
#[doc = "Register SPI0_TX_DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI0_TX_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI0_TX_DATA`"]
pub type SPI0_TX_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI0_TX_DATA`"]
pub struct SPI0_TX_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_TX_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Single word buffer for data to be transmitted over the SPI interface"]
    #[inline(always)]
    pub fn spi0_tx_data(&self) -> SPI0_TX_DATA_R {
        SPI0_TX_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Single word buffer for data to be transmitted over the SPI interface"]
    #[inline(always)]
    pub fn spi0_tx_data(&mut self) -> SPI0_TX_DATA_W {
        SPI0_TX_DATA_W { w: self }
    }
}
