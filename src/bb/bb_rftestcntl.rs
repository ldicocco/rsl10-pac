#[doc = "Reader of register BB_RFTESTCNTL"]
pub type R = crate::R<u32, super::BB_RFTESTCNTL>;
#[doc = "Writer for register BB_RFTESTCNTL"]
pub type W = crate::W<u32, super::BB_RFTESTCNTL>;
#[doc = "Register BB_RFTESTCNTL `reset()`'s with value 0x0800"]
impl crate::ResetValue for super::BB_RFTESTCNTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0800
    }
}
#[doc = "Applicable in RF test mode only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INFINITERX_A {
    #[doc = "0: Normal mode of operation"]
    INFINITERX_0 = 0,
    #[doc = "1: Infinite Rx window"]
    INFINITERX_1 = 1,
}
impl From<INFINITERX_A> for bool {
    #[inline(always)]
    fn from(variant: INFINITERX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INFINITERX`"]
pub type INFINITERX_R = crate::R<bool, INFINITERX_A>;
impl INFINITERX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INFINITERX_A {
        match self.bits {
            false => INFINITERX_A::INFINITERX_0,
            true => INFINITERX_A::INFINITERX_1,
        }
    }
    #[doc = "Checks if the value of the field is `INFINITERX_0`"]
    #[inline(always)]
    pub fn is_infiniterx_0(&self) -> bool {
        *self == INFINITERX_A::INFINITERX_0
    }
    #[doc = "Checks if the value of the field is `INFINITERX_1`"]
    #[inline(always)]
    pub fn is_infiniterx_1(&self) -> bool {
        *self == INFINITERX_A::INFINITERX_1
    }
}
#[doc = "Write proxy for field `INFINITERX`"]
pub struct INFINITERX_W<'a> {
    w: &'a mut W,
}
impl<'a> INFINITERX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INFINITERX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal mode of operation"]
    #[inline(always)]
    pub fn infiniterx_0(self) -> &'a mut W {
        self.variant(INFINITERX_A::INFINITERX_0)
    }
    #[doc = "Infinite Rx window"]
    #[inline(always)]
    pub fn infiniterx_1(self) -> &'a mut W {
        self.variant(INFINITERX_A::INFINITERX_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Applicable in RF test mode only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPKTCNTEN_A {
    #[doc = "0: Rx packet count disabled"]
    RXPKTCNTEN_0 = 0,
    #[doc = "1: Rx packet count enabled, and reported in CS-RXCCMPKTCNT and RFTESTRXSTAT-RXPKTCNT on RF abort command"]
    RXPKTCNTEN_1 = 1,
}
impl From<RXPKTCNTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXPKTCNTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXPKTCNTEN`"]
pub type RXPKTCNTEN_R = crate::R<bool, RXPKTCNTEN_A>;
impl RXPKTCNTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPKTCNTEN_A {
        match self.bits {
            false => RXPKTCNTEN_A::RXPKTCNTEN_0,
            true => RXPKTCNTEN_A::RXPKTCNTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXPKTCNTEN_0`"]
    #[inline(always)]
    pub fn is_rxpktcnten_0(&self) -> bool {
        *self == RXPKTCNTEN_A::RXPKTCNTEN_0
    }
    #[doc = "Checks if the value of the field is `RXPKTCNTEN_1`"]
    #[inline(always)]
    pub fn is_rxpktcnten_1(&self) -> bool {
        *self == RXPKTCNTEN_A::RXPKTCNTEN_1
    }
}
#[doc = "Write proxy for field `RXPKTCNTEN`"]
pub struct RXPKTCNTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPKTCNTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPKTCNTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rx packet count disabled"]
    #[inline(always)]
    pub fn rxpktcnten_0(self) -> &'a mut W {
        self.variant(RXPKTCNTEN_A::RXPKTCNTEN_0)
    }
    #[doc = "Rx packet count enabled, and reported in CS-RXCCMPKTCNT and RFTESTRXSTAT-RXPKTCNT on RF abort command"]
    #[inline(always)]
    pub fn rxpktcnten_1(self) -> &'a mut W {
        self.variant(RXPKTCNTEN_A::RXPKTCNTEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Applicable in RF test mode only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INFINITETX_A {
    #[doc = "0: Normal mode of operation"]
    INFINITETX_0 = 0,
    #[doc = "1: Infinite Tx packet / Normal start of a packet but endless payload"]
    INFINITETX_1 = 1,
}
impl From<INFINITETX_A> for bool {
    #[inline(always)]
    fn from(variant: INFINITETX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INFINITETX`"]
pub type INFINITETX_R = crate::R<bool, INFINITETX_A>;
impl INFINITETX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INFINITETX_A {
        match self.bits {
            false => INFINITETX_A::INFINITETX_0,
            true => INFINITETX_A::INFINITETX_1,
        }
    }
    #[doc = "Checks if the value of the field is `INFINITETX_0`"]
    #[inline(always)]
    pub fn is_infinitetx_0(&self) -> bool {
        *self == INFINITETX_A::INFINITETX_0
    }
    #[doc = "Checks if the value of the field is `INFINITETX_1`"]
    #[inline(always)]
    pub fn is_infinitetx_1(&self) -> bool {
        *self == INFINITETX_A::INFINITETX_1
    }
}
#[doc = "Write proxy for field `INFINITETX`"]
pub struct INFINITETX_W<'a> {
    w: &'a mut W,
}
impl<'a> INFINITETX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INFINITETX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal mode of operation"]
    #[inline(always)]
    pub fn infinitetx_0(self) -> &'a mut W {
        self.variant(INFINITETX_A::INFINITETX_0)
    }
    #[doc = "Infinite Tx packet / Normal start of a packet but endless payload"]
    #[inline(always)]
    pub fn infinitetx_1(self) -> &'a mut W {
        self.variant(INFINITETX_A::INFINITETX_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Applicable only in Tx/Rx RF test mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXLENGTHSRC_A {
    #[doc = "0: Normal mode of operation: TxDESC-TXADVLEN controls the Tx packet payload size"]
    TXLENGTHSRC_0 = 0,
    #[doc = "1: Uses RFTESTCTRL-TXLENGTH packet length (can support up to 512 bytes transmit)"]
    TXLENGTHSRC_1 = 1,
}
impl From<TXLENGTHSRC_A> for bool {
    #[inline(always)]
    fn from(variant: TXLENGTHSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXLENGTHSRC`"]
pub type TXLENGTHSRC_R = crate::R<bool, TXLENGTHSRC_A>;
impl TXLENGTHSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXLENGTHSRC_A {
        match self.bits {
            false => TXLENGTHSRC_A::TXLENGTHSRC_0,
            true => TXLENGTHSRC_A::TXLENGTHSRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXLENGTHSRC_0`"]
    #[inline(always)]
    pub fn is_txlengthsrc_0(&self) -> bool {
        *self == TXLENGTHSRC_A::TXLENGTHSRC_0
    }
    #[doc = "Checks if the value of the field is `TXLENGTHSRC_1`"]
    #[inline(always)]
    pub fn is_txlengthsrc_1(&self) -> bool {
        *self == TXLENGTHSRC_A::TXLENGTHSRC_1
    }
}
#[doc = "Write proxy for field `TXLENGTHSRC`"]
pub struct TXLENGTHSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLENGTHSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXLENGTHSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal mode of operation: TxDESC-TXADVLEN controls the Tx packet payload size"]
    #[inline(always)]
    pub fn txlengthsrc_0(self) -> &'a mut W {
        self.variant(TXLENGTHSRC_A::TXLENGTHSRC_0)
    }
    #[doc = "Uses RFTESTCTRL-TXLENGTH packet length (can support up to 512 bytes transmit)"]
    #[inline(always)]
    pub fn txlengthsrc_1(self) -> &'a mut W {
        self.variant(TXLENGTHSRC_A::TXLENGTHSRC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Applicable only in Tx/Rx RF test mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRBSTYPE_A {
    #[doc = "0: Tx packet payload are PRBS9 type"]
    PRBSTYPE_0 = 0,
    #[doc = "1: Tx packet payload are PRBS15 type"]
    PRBSTYPE_1 = 1,
}
impl From<PRBSTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: PRBSTYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRBSTYPE`"]
pub type PRBSTYPE_R = crate::R<bool, PRBSTYPE_A>;
impl PRBSTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRBSTYPE_A {
        match self.bits {
            false => PRBSTYPE_A::PRBSTYPE_0,
            true => PRBSTYPE_A::PRBSTYPE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PRBSTYPE_0`"]
    #[inline(always)]
    pub fn is_prbstype_0(&self) -> bool {
        *self == PRBSTYPE_A::PRBSTYPE_0
    }
    #[doc = "Checks if the value of the field is `PRBSTYPE_1`"]
    #[inline(always)]
    pub fn is_prbstype_1(&self) -> bool {
        *self == PRBSTYPE_A::PRBSTYPE_1
    }
}
#[doc = "Write proxy for field `PRBSTYPE`"]
pub struct PRBSTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRBSTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRBSTYPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx packet payload are PRBS9 type"]
    #[inline(always)]
    pub fn prbstype_0(self) -> &'a mut W {
        self.variant(PRBSTYPE_A::PRBSTYPE_0)
    }
    #[doc = "Tx packet payload are PRBS15 type"]
    #[inline(always)]
    pub fn prbstype_1(self) -> &'a mut W {
        self.variant(PRBSTYPE_A::PRBSTYPE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Applicable only in Tx/Rx RF test mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPLDSRC_A {
    #[doc = "0: Tx packet payload source is the control structure"]
    TXPLDSRC_0 = 0,
    #[doc = "1: Tx packet payload are PRBS generator"]
    TXPLDSRC_1 = 1,
}
impl From<TXPLDSRC_A> for bool {
    #[inline(always)]
    fn from(variant: TXPLDSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXPLDSRC`"]
pub type TXPLDSRC_R = crate::R<bool, TXPLDSRC_A>;
impl TXPLDSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPLDSRC_A {
        match self.bits {
            false => TXPLDSRC_A::TXPLDSRC_0,
            true => TXPLDSRC_A::TXPLDSRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXPLDSRC_0`"]
    #[inline(always)]
    pub fn is_txpldsrc_0(&self) -> bool {
        *self == TXPLDSRC_A::TXPLDSRC_0
    }
    #[doc = "Checks if the value of the field is `TXPLDSRC_1`"]
    #[inline(always)]
    pub fn is_txpldsrc_1(&self) -> bool {
        *self == TXPLDSRC_A::TXPLDSRC_1
    }
}
#[doc = "Write proxy for field `TXPLDSRC`"]
pub struct TXPLDSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPLDSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPLDSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx packet payload source is the control structure"]
    #[inline(always)]
    pub fn txpldsrc_0(self) -> &'a mut W {
        self.variant(TXPLDSRC_A::TXPLDSRC_0)
    }
    #[doc = "Tx packet payload are PRBS generator"]
    #[inline(always)]
    pub fn txpldsrc_1(self) -> &'a mut W {
        self.variant(TXPLDSRC_A::TXPLDSRC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Applicable in RF test mode only\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPKTCNTEN_A {
    #[doc = "0: Tx packet count disabled"]
    TXPKTCNTEN_0 = 0,
    #[doc = "1: Tx packet count enabled, and reported in CS-TXCCMPKTCNT and RFTESTTXSTAT-TXPKTCNT on RF abort command"]
    TXPKTCNTEN_1 = 1,
}
impl From<TXPKTCNTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXPKTCNTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXPKTCNTEN`"]
pub type TXPKTCNTEN_R = crate::R<bool, TXPKTCNTEN_A>;
impl TXPKTCNTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPKTCNTEN_A {
        match self.bits {
            false => TXPKTCNTEN_A::TXPKTCNTEN_0,
            true => TXPKTCNTEN_A::TXPKTCNTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXPKTCNTEN_0`"]
    #[inline(always)]
    pub fn is_txpktcnten_0(&self) -> bool {
        *self == TXPKTCNTEN_A::TXPKTCNTEN_0
    }
    #[doc = "Checks if the value of the field is `TXPKTCNTEN_1`"]
    #[inline(always)]
    pub fn is_txpktcnten_1(&self) -> bool {
        *self == TXPKTCNTEN_A::TXPKTCNTEN_1
    }
}
#[doc = "Write proxy for field `TXPKTCNTEN`"]
pub struct TXPKTCNTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPKTCNTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPKTCNTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx packet count disabled"]
    #[inline(always)]
    pub fn txpktcnten_0(self) -> &'a mut W {
        self.variant(TXPKTCNTEN_A::TXPKTCNTEN_0)
    }
    #[doc = "Tx packet count enabled, and reported in CS-TXCCMPKTCNT and RFTESTTXSTAT-TXPKTCNT on RF abort command"]
    #[inline(always)]
    pub fn txpktcnten_1(self) -> &'a mut W {
        self.variant(TXPKTCNTEN_A::TXPKTCNTEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Tx packet length in number of byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum TXLENGTH_A {
    #[doc = "0: `0`"]
    TXLENGTH_0 = 0,
}
impl From<TXLENGTH_A> for u16 {
    #[inline(always)]
    fn from(variant: TXLENGTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXLENGTH`"]
pub type TXLENGTH_R = crate::R<u16, TXLENGTH_A>;
impl TXLENGTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, TXLENGTH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXLENGTH_A::TXLENGTH_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXLENGTH_0`"]
    #[inline(always)]
    pub fn is_txlength_0(&self) -> bool {
        *self == TXLENGTH_A::TXLENGTH_0
    }
}
#[doc = "Write proxy for field `TXLENGTH`"]
pub struct TXLENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLENGTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXLENGTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn txlength_0(self) -> &'a mut W {
        self.variant(TXLENGTH_A::TXLENGTH_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Applicable in RF test mode only"]
    #[inline(always)]
    pub fn infiniterx(&self) -> INFINITERX_R {
        INFINITERX_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Applicable in RF test mode only"]
    #[inline(always)]
    pub fn rxpktcnten(&self) -> RXPKTCNTEN_R {
        RXPKTCNTEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Applicable in RF test mode only"]
    #[inline(always)]
    pub fn infinitetx(&self) -> INFINITETX_R {
        INFINITETX_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Applicable only in Tx/Rx RF test mode"]
    #[inline(always)]
    pub fn txlengthsrc(&self) -> TXLENGTHSRC_R {
        TXLENGTHSRC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Applicable only in Tx/Rx RF test mode"]
    #[inline(always)]
    pub fn prbstype(&self) -> PRBSTYPE_R {
        PRBSTYPE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Applicable only in Tx/Rx RF test mode"]
    #[inline(always)]
    pub fn txpldsrc(&self) -> TXPLDSRC_R {
        TXPLDSRC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Applicable in RF test mode only"]
    #[inline(always)]
    pub fn txpktcnten(&self) -> TXPKTCNTEN_R {
        TXPKTCNTEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 0:8 - Tx packet length in number of byte"]
    #[inline(always)]
    pub fn txlength(&self) -> TXLENGTH_R {
        TXLENGTH_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Applicable in RF test mode only"]
    #[inline(always)]
    pub fn infiniterx(&mut self) -> INFINITERX_W {
        INFINITERX_W { w: self }
    }
    #[doc = "Bit 27 - Applicable in RF test mode only"]
    #[inline(always)]
    pub fn rxpktcnten(&mut self) -> RXPKTCNTEN_W {
        RXPKTCNTEN_W { w: self }
    }
    #[doc = "Bit 15 - Applicable in RF test mode only"]
    #[inline(always)]
    pub fn infinitetx(&mut self) -> INFINITETX_W {
        INFINITETX_W { w: self }
    }
    #[doc = "Bit 14 - Applicable only in Tx/Rx RF test mode"]
    #[inline(always)]
    pub fn txlengthsrc(&mut self) -> TXLENGTHSRC_W {
        TXLENGTHSRC_W { w: self }
    }
    #[doc = "Bit 13 - Applicable only in Tx/Rx RF test mode"]
    #[inline(always)]
    pub fn prbstype(&mut self) -> PRBSTYPE_W {
        PRBSTYPE_W { w: self }
    }
    #[doc = "Bit 12 - Applicable only in Tx/Rx RF test mode"]
    #[inline(always)]
    pub fn txpldsrc(&mut self) -> TXPLDSRC_W {
        TXPLDSRC_W { w: self }
    }
    #[doc = "Bit 11 - Applicable in RF test mode only"]
    #[inline(always)]
    pub fn txpktcnten(&mut self) -> TXPKTCNTEN_W {
        TXPKTCNTEN_W { w: self }
    }
    #[doc = "Bits 0:8 - Tx packet length in number of byte"]
    #[inline(always)]
    pub fn txlength(&mut self) -> TXLENGTH_W {
        TXLENGTH_W { w: self }
    }
}
