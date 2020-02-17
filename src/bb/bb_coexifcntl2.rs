#[doc = "Reader of register BB_COEXIFCNTL2"]
pub type R = crate::R<u32, super::BB_COEXIFCNTL2>;
#[doc = "Writer for register BB_COEXIFCNTL2"]
pub type W = crate::W<u32, super::BB_COEXIFCNTL2>;
#[doc = "Register BB_COEXIFCNTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_COEXIFCNTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Time (in us) by which is anticipated bt_rx to be provided before effective Radio receipt operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_ANT_DELAY_A {
    #[doc = "0: `0`"]
    RX_ANT_DELAY_0 = 0,
}
impl From<RX_ANT_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_ANT_DELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RX_ANT_DELAY`"]
pub type RX_ANT_DELAY_R = crate::R<u8, RX_ANT_DELAY_A>;
impl RX_ANT_DELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RX_ANT_DELAY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RX_ANT_DELAY_A::RX_ANT_DELAY_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX_ANT_DELAY_0`"]
    #[inline(always)]
    pub fn is_rx_ant_delay_0(&self) -> bool {
        *self == RX_ANT_DELAY_A::RX_ANT_DELAY_0
    }
}
#[doc = "Write proxy for field `RX_ANT_DELAY`"]
pub struct RX_ANT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ANT_DELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_ANT_DELAY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rx_ant_delay_0(self) -> &'a mut W {
        self.variant(RX_ANT_DELAY_A::RX_ANT_DELAY_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Time (in us) by which is anticipated bt_tx to be provided before effective Radio transmit operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_ANT_DELAY_A {
    #[doc = "0: `0`"]
    TX_ANT_DELAY_0 = 0,
}
impl From<TX_ANT_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_ANT_DELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_ANT_DELAY`"]
pub type TX_ANT_DELAY_R = crate::R<u8, TX_ANT_DELAY_A>;
impl TX_ANT_DELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_ANT_DELAY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_ANT_DELAY_A::TX_ANT_DELAY_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_ANT_DELAY_0`"]
    #[inline(always)]
    pub fn is_tx_ant_delay_0(&self) -> bool {
        *self == TX_ANT_DELAY_A::TX_ANT_DELAY_0
    }
}
#[doc = "Write proxy for field `TX_ANT_DELAY`"]
pub struct TX_ANT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ANT_DELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_ANT_DELAY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_ant_delay_0(self) -> &'a mut W {
        self.variant(TX_ANT_DELAY_A::TX_ANT_DELAY_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11 - Time (in us) by which is anticipated bt_rx to be provided before effective Radio receipt operation"]
    #[inline(always)]
    pub fn rx_ant_delay(&self) -> RX_ANT_DELAY_R {
        RX_ANT_DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Time (in us) by which is anticipated bt_tx to be provided before effective Radio transmit operation"]
    #[inline(always)]
    pub fn tx_ant_delay(&self) -> TX_ANT_DELAY_R {
        TX_ANT_DELAY_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - Time (in us) by which is anticipated bt_rx to be provided before effective Radio receipt operation"]
    #[inline(always)]
    pub fn rx_ant_delay(&mut self) -> RX_ANT_DELAY_W {
        RX_ANT_DELAY_W { w: self }
    }
    #[doc = "Bits 0:3 - Time (in us) by which is anticipated bt_tx to be provided before effective Radio transmit operation"]
    #[inline(always)]
    pub fn tx_ant_delay(&mut self) -> TX_ANT_DELAY_W {
        TX_ANT_DELAY_W { w: self }
    }
}
