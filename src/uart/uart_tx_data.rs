#[doc = "Reader of register UART_TX_DATA"]
pub type R = crate::R<u32, super::UART_TX_DATA>;
#[doc = "Writer for register UART_TX_DATA"]
pub type W = crate::W<u32, super::UART_TX_DATA>;
#[doc = "Register UART_TX_DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_TX_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_TX_DATA`"]
pub type UART_TX_DATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_TX_DATA`"]
pub struct UART_TX_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TX_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - UART Transmit data"]
    #[inline(always)]
    pub fn uart_tx_data(&self) -> UART_TX_DATA_R {
        UART_TX_DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART Transmit data"]
    #[inline(always)]
    pub fn uart_tx_data(&mut self) -> UART_TX_DATA_W {
        UART_TX_DATA_W { w: self }
    }
}
