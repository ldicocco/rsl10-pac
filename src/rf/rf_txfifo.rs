#[doc = "Reader of register RF_TXFIFO"]
pub type R = crate::R<u32, super::RF_TXFIFO>;
#[doc = "Writer for register RF_TXFIFO"]
pub type W = crate::W<u32, super::RF_TXFIFO>;
#[doc = "Register RF_TXFIFO `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_TXFIFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TXFIFO_TX_DATA`"]
pub struct TXFIFO_TX_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_TX_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bits 0:7 - Data to be sent"]
    #[inline(always)]
    pub fn txfifo_tx_data(&mut self) -> TXFIFO_TX_DATA_W {
        TXFIFO_TX_DATA_W { w: self }
    }
}
