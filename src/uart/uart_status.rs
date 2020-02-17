#[doc = "Reader of register UART_STATUS"]
pub type R = crate::R<u32, super::UART_STATUS>;
#[doc = "Writer for register UART_STATUS"]
pub type W = crate::W<u32, super::UART_STATUS>;
#[doc = "Register UART_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Indicate that an overrun has occurred when receiving data on the UART interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_RX_OVERRUN_STATUS_A {
    #[doc = "0: No UART RX overrun detected"]
    UART_RX_OVERRUN_FALSE = 0,
    #[doc = "1: UART RX overrun detected"]
    UART_RX_OVERRUN_TRUE = 1,
}
impl From<UART_RX_OVERRUN_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: UART_RX_OVERRUN_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART_RX_OVERRUN_STATUS`"]
pub type UART_RX_OVERRUN_STATUS_R = crate::R<bool, UART_RX_OVERRUN_STATUS_A>;
impl UART_RX_OVERRUN_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART_RX_OVERRUN_STATUS_A {
        match self.bits {
            false => UART_RX_OVERRUN_STATUS_A::UART_RX_OVERRUN_FALSE,
            true => UART_RX_OVERRUN_STATUS_A::UART_RX_OVERRUN_TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `UART_RX_OVERRUN_FALSE`"]
    #[inline(always)]
    pub fn is_uart_rx_overrun_false(&self) -> bool {
        *self == UART_RX_OVERRUN_STATUS_A::UART_RX_OVERRUN_FALSE
    }
    #[doc = "Checks if the value of the field is `UART_RX_OVERRUN_TRUE`"]
    #[inline(always)]
    pub fn is_uart_rx_overrun_true(&self) -> bool {
        *self == UART_RX_OVERRUN_STATUS_A::UART_RX_OVERRUN_TRUE
    }
}
#[doc = "Write proxy for field `UART_RX_OVERRUN_STATUS`"]
pub struct UART_RX_OVERRUN_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RX_OVERRUN_STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_RX_OVERRUN_STATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No UART RX overrun detected"]
    #[inline(always)]
    pub fn uart_rx_overrun_false(self) -> &'a mut W {
        self.variant(UART_RX_OVERRUN_STATUS_A::UART_RX_OVERRUN_FALSE)
    }
    #[doc = "UART RX overrun detected"]
    #[inline(always)]
    pub fn uart_rx_overrun_true(self) -> &'a mut W {
        self.variant(UART_RX_OVERRUN_STATUS_A::UART_RX_OVERRUN_TRUE)
    }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Indicate that an overrun has occurred when receiving data on the UART interface"]
    #[inline(always)]
    pub fn uart_rx_overrun_status(&self) -> UART_RX_OVERRUN_STATUS_R {
        UART_RX_OVERRUN_STATUS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicate that an overrun has occurred when receiving data on the UART interface"]
    #[inline(always)]
    pub fn uart_rx_overrun_status(&mut self) -> UART_RX_OVERRUN_STATUS_W {
        UART_RX_OVERRUN_STATUS_W { w: self }
    }
}
