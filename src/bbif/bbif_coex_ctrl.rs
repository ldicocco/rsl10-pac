#[doc = "Reader of register BBIF_COEX_CTRL"]
pub type R = crate::R<u32, super::BBIF_COEX_CTRL>;
#[doc = "Writer for register BBIF_COEX_CTRL"]
pub type W = crate::W<u32, super::BBIF_COEX_CTRL>;
#[doc = "Register BBIF_COEX_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::BBIF_COEX_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Indicates if the RF front-end performs a non-BLE Tx activity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_A {
    #[doc = "0: RF front-end has no non-BLE TX activity"]
    COEX_TX_IDLE = 0,
    #[doc = "1: RF front-end performs a non-BLE TX activity"]
    COEX_TX_BUSY = 1,
}
impl From<TX_A> for bool {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX`"]
pub type TX_R = crate::R<bool, TX_A>;
impl TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            false => TX_A::COEX_TX_IDLE,
            true => TX_A::COEX_TX_BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `COEX_TX_IDLE`"]
    #[inline(always)]
    pub fn is_coex_tx_idle(&self) -> bool {
        *self == TX_A::COEX_TX_IDLE
    }
    #[doc = "Checks if the value of the field is `COEX_TX_BUSY`"]
    #[inline(always)]
    pub fn is_coex_tx_busy(&self) -> bool {
        *self == TX_A::COEX_TX_BUSY
    }
}
#[doc = "Write proxy for field `TX`"]
pub struct TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RF front-end has no non-BLE TX activity"]
    #[inline(always)]
    pub fn coex_tx_idle(self) -> &'a mut W {
        self.variant(TX_A::COEX_TX_IDLE)
    }
    #[doc = "RF front-end performs a non-BLE TX activity"]
    #[inline(always)]
    pub fn coex_tx_busy(self) -> &'a mut W {
        self.variant(TX_A::COEX_TX_BUSY)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Indicates if the RF front-end performs a non-BLE Rx activity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_A {
    #[doc = "0: RF front-end has no non-BLE RX activity"]
    COEX_RX_IDLE = 0,
    #[doc = "1: RF front-end performs a non-BLE RX activity"]
    COEX_RX_BUSY = 1,
}
impl From<RX_A> for bool {
    #[inline(always)]
    fn from(variant: RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX`"]
pub type RX_R = crate::R<bool, RX_A>;
impl RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_A {
        match self.bits {
            false => RX_A::COEX_RX_IDLE,
            true => RX_A::COEX_RX_BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `COEX_RX_IDLE`"]
    #[inline(always)]
    pub fn is_coex_rx_idle(&self) -> bool {
        *self == RX_A::COEX_RX_IDLE
    }
    #[doc = "Checks if the value of the field is `COEX_RX_BUSY`"]
    #[inline(always)]
    pub fn is_coex_rx_busy(&self) -> bool {
        *self == RX_A::COEX_RX_BUSY
    }
}
#[doc = "Write proxy for field `RX`"]
pub struct RX_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RF front-end has no non-BLE RX activity"]
    #[inline(always)]
    pub fn coex_rx_idle(self) -> &'a mut W {
        self.variant(RX_A::COEX_RX_IDLE)
    }
    #[doc = "RF front-end performs a non-BLE RX activity"]
    #[inline(always)]
    pub fn coex_rx_busy(self) -> &'a mut W {
        self.variant(RX_A::COEX_RX_BUSY)
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
    #[doc = "Bit 4 - Indicates if the RF front-end performs a non-BLE Tx activity"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indicates if the RF front-end performs a non-BLE Rx activity"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Indicates if the RF front-end performs a non-BLE Tx activity"]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W { w: self }
    }
    #[doc = "Bit 0 - Indicates if the RF front-end performs a non-BLE Rx activity"]
    #[inline(always)]
    pub fn rx(&mut self) -> RX_W {
        RX_W { w: self }
    }
}
