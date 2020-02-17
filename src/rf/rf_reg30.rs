#[doc = "Reader of register RF_REG30"]
pub type R = crate::R<u32, super::RF_REG30>;
#[doc = "Writer for register RF_REG30"]
pub type W = crate::W<u32, super::RF_REG30>;
#[doc = "Register RF_REG30 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG30 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RXFIFO_STATUS_BIST`"]
pub struct RXFIFO_STATUS_BIST_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_STATUS_BIST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Write proxy for field `RXFIFO_STATUS_FLUSH`"]
pub struct RXFIFO_STATUS_FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_STATUS_FLUSH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RXFIFO_STATUS_BIST_ERRORS`"]
pub type RXFIFO_STATUS_BIST_ERRORS_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXFIFO_STATUS_NEAR_UNDERFLOW`"]
pub type RXFIFO_STATUS_NEAR_UNDERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFO_STATUS_NEAR_OVERFLOW`"]
pub type RXFIFO_STATUS_NEAR_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFO_STATUS_UNDERFLOW`"]
pub type RXFIFO_STATUS_UNDERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFO_STATUS_OVERFLOW`"]
pub type RXFIFO_STATUS_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFO_STATUS_FULL`"]
pub type RXFIFO_STATUS_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFO_STATUS_EMPTY`"]
pub type RXFIFO_STATUS_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFIFO_STATUS_BIST`"]
pub struct TXFIFO_STATUS_BIST_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_STATUS_BIST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 17)) | (((value as u32) & 0x7f) << 17);
        self.w
    }
}
#[doc = "Write proxy for field `TXFIFO_STATUS_FLUSH`"]
pub struct TXFIFO_STATUS_FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_STATUS_FLUSH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TXFIFO_STATUS_BIST_ERRORS`"]
pub type TXFIFO_STATUS_BIST_ERRORS_R = crate::R<u8, u8>;
#[doc = "Reader of field `TXFIFO_STATUS_NEAR_UNDERFLOW`"]
pub type TXFIFO_STATUS_NEAR_UNDERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFIFO_STATUS_NEAR_OVERFLOW`"]
pub type TXFIFO_STATUS_NEAR_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFIFO_STATUS_UNDERFLOW`"]
pub type TXFIFO_STATUS_UNDERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFIFO_STATUS_OVERFLOW`"]
pub type TXFIFO_STATUS_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFIFO_STATUS_FULL`"]
pub type TXFIFO_STATUS_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFIFO_STATUS_EMPTY`"]
pub type TXFIFO_STATUS_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `FSM_STATUS_TX_NRX`"]
pub type FSM_STATUS_TX_NRX_R = crate::R<bool, bool>;
#[doc = "Reader of field `FSM_STATUS_STATUS`"]
pub type FSM_STATUS_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSM_MODE_RESET`"]
pub struct FSM_MODE_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_MODE_RESET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `FSM_MODE_TX_NRX`"]
pub struct FSM_MODE_TX_NRX_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_MODE_TX_NRX_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `FSM_MODE_MODE`"]
pub struct FSM_MODE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_MODE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `FSM_MODE_RX_MODE`"]
pub type FSM_MODE_RX_MODE_R = crate::R<bool, bool>;
#[doc = "Reader of field `FSM_MODE_TX_MODE`"]
pub type FSM_MODE_TX_MODE_R = crate::R<bool, bool>;
#[doc = "Reader of field `FSM_MODE_N_IDLE`"]
pub type FSM_MODE_N_IDLE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 30:31 - Indicate the BIST error: 00 => no error, 01 => error in checkboard test, 10 => error in inversed checkboard test, 11 => error in decoder test"]
    #[inline(always)]
    pub fn rxfifo_status_bist_errors(&self) -> RXFIFO_STATUS_BIST_ERRORS_R {
        RXFIFO_STATUS_BIST_ERRORS_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29 - Is set to 1 if the Rx FIFO is close to the underflow"]
    #[inline(always)]
    pub fn rxfifo_status_near_underflow(&self) -> RXFIFO_STATUS_NEAR_UNDERFLOW_R {
        RXFIFO_STATUS_NEAR_UNDERFLOW_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Is set to 1 if the Rx FIFO is close to the overflow"]
    #[inline(always)]
    pub fn rxfifo_status_near_overflow(&self) -> RXFIFO_STATUS_NEAR_OVERFLOW_R {
        RXFIFO_STATUS_NEAR_OVERFLOW_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Is set to 1 if there has been an underflow"]
    #[inline(always)]
    pub fn rxfifo_status_underflow(&self) -> RXFIFO_STATUS_UNDERFLOW_R {
        RXFIFO_STATUS_UNDERFLOW_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Is set to 1 if there has been an overflow"]
    #[inline(always)]
    pub fn rxfifo_status_overflow(&self) -> RXFIFO_STATUS_OVERFLOW_R {
        RXFIFO_STATUS_OVERFLOW_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Is set to 1 if the Rx FIFO is full"]
    #[inline(always)]
    pub fn rxfifo_status_full(&self) -> RXFIFO_STATUS_FULL_R {
        RXFIFO_STATUS_FULL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Is set to 1 if the Rx FIFO is empty"]
    #[inline(always)]
    pub fn rxfifo_status_empty(&self) -> RXFIFO_STATUS_EMPTY_R {
        RXFIFO_STATUS_EMPTY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - Indicate the BIST error: 00 => no error, 01 => error in checkboard test, 10 => error in inversed checkboard test, 11 => error in decoder test"]
    #[inline(always)]
    pub fn txfifo_status_bist_errors(&self) -> TXFIFO_STATUS_BIST_ERRORS_R {
        TXFIFO_STATUS_BIST_ERRORS_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 21 - Is set to 1 if the Tx FIFO is close to the underflow"]
    #[inline(always)]
    pub fn txfifo_status_near_underflow(&self) -> TXFIFO_STATUS_NEAR_UNDERFLOW_R {
        TXFIFO_STATUS_NEAR_UNDERFLOW_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Is set to 1 if the Tx FIFO is close to the overflow"]
    #[inline(always)]
    pub fn txfifo_status_near_overflow(&self) -> TXFIFO_STATUS_NEAR_OVERFLOW_R {
        TXFIFO_STATUS_NEAR_OVERFLOW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Is set to 1 if there has been an underflow"]
    #[inline(always)]
    pub fn txfifo_status_underflow(&self) -> TXFIFO_STATUS_UNDERFLOW_R {
        TXFIFO_STATUS_UNDERFLOW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Is set to 1 if there has been an overflow"]
    #[inline(always)]
    pub fn txfifo_status_overflow(&self) -> TXFIFO_STATUS_OVERFLOW_R {
        TXFIFO_STATUS_OVERFLOW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Is set to 1 if the Tx FIFO is full"]
    #[inline(always)]
    pub fn txfifo_status_full(&self) -> TXFIFO_STATUS_FULL_R {
        TXFIFO_STATUS_FULL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Is set to 1 if the Tx FIFO is empty"]
    #[inline(always)]
    pub fn txfifo_status_empty(&self) -> TXFIFO_STATUS_EMPTY_R {
        TXFIFO_STATUS_EMPTY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Is set to 0 if the radio is in Rx mode, to 1 if in Tx mode"]
    #[inline(always)]
    pub fn fsm_status_tx_nrx(&self) -> FSM_STATUS_TX_NRX_R {
        FSM_STATUS_TX_NRX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Status of the FSM: 00 => Idle, 01 => Tx mode, 10 => Rx mode, 11 => Suspend"]
    #[inline(always)]
    pub fn fsm_status_status(&self) -> FSM_STATUS_STATUS_R {
        FSM_STATUS_STATUS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 2 - The field stay with value 1 as long as the reception isn't over"]
    #[inline(always)]
    pub fn fsm_mode_rx_mode(&self) -> FSM_MODE_RX_MODE_R {
        FSM_MODE_RX_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - The field keep the value 1 as long as the transmission isn't over"]
    #[inline(always)]
    pub fn fsm_mode_tx_mode(&self) -> FSM_MODE_TX_MODE_R {
        FSM_MODE_TX_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The field is set to 1 if the FSM is not in the Idle mode."]
    #[inline(always)]
    pub fn fsm_mode_n_idle(&self) -> FSM_MODE_N_IDLE_R {
        FSM_MODE_N_IDLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 25:31 - Start the bist test on the Rx FIFO (code 0x5d)"]
    #[inline(always)]
    pub fn rxfifo_status_bist(&mut self) -> RXFIFO_STATUS_BIST_W {
        RXFIFO_STATUS_BIST_W { w: self }
    }
    #[doc = "Bit 24 - If set to 1 the Rx FIFO is flushed"]
    #[inline(always)]
    pub fn rxfifo_status_flush(&mut self) -> RXFIFO_STATUS_FLUSH_W {
        RXFIFO_STATUS_FLUSH_W { w: self }
    }
    #[doc = "Bits 17:23 - Start the bist test on the Tx FIFO (code 0x5d)"]
    #[inline(always)]
    pub fn txfifo_status_bist(&mut self) -> TXFIFO_STATUS_BIST_W {
        TXFIFO_STATUS_BIST_W { w: self }
    }
    #[doc = "Bit 16 - If set to 1 the Tx FIFO is flushed"]
    #[inline(always)]
    pub fn txfifo_status_flush(&mut self) -> TXFIFO_STATUS_FLUSH_W {
        TXFIFO_STATUS_FLUSH_W { w: self }
    }
    #[doc = "Bit 3 - If set to 1, the FSM is reset. If mode is set to 0 the FSM is reset abrubtly. If is set to 1 the Tx or Rx (depending on tx_nrx) is stopped gently via the serializer or the deserializer"]
    #[inline(always)]
    pub fn fsm_mode_reset(&mut self) -> FSM_MODE_RESET_W {
        FSM_MODE_RESET_W { w: self }
    }
    #[doc = "Bit 2 - Sets the Radio in Tx (1) or Rx (0) mode"]
    #[inline(always)]
    pub fn fsm_mode_tx_nrx(&mut self) -> FSM_MODE_TX_NRX_W {
        FSM_MODE_TX_NRX_W { w: self }
    }
    #[doc = "Bits 0:1 - Sets the FSM mode: 00: nothing is done, 01: activate, 10: calibrate the PLL, 11: calibrate the PLL then Tx/Rx"]
    #[inline(always)]
    pub fn fsm_mode_mode(&mut self) -> FSM_MODE_MODE_W {
        FSM_MODE_MODE_W { w: self }
    }
}
