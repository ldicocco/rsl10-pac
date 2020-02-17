#[doc = "Reader of register RF_REG00"]
pub type R = crate::R<u32, super::RF_REG00>;
#[doc = "Writer for register RF_REG00"]
pub type W = crate::W<u32, super::RF_REG00>;
#[doc = "Register RF_REG00 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG00 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "If set to 1, the data whitening specified in the Bluetooth LE standard is used. Note that the en_datawhite field of the CODING register has also to be set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAWHITE_BTLE_DW_BTLE_A {
    #[doc = "0: `0`"]
    DATAWHITE_BTLE_DW_BTLE_DEFAULT = 0,
}
impl From<DATAWHITE_BTLE_DW_BTLE_A> for bool {
    #[inline(always)]
    fn from(variant: DATAWHITE_BTLE_DW_BTLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DATAWHITE_BTLE_DW_BTLE`"]
pub type DATAWHITE_BTLE_DW_BTLE_R = crate::R<bool, DATAWHITE_BTLE_DW_BTLE_A>;
impl DATAWHITE_BTLE_DW_BTLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DATAWHITE_BTLE_DW_BTLE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DATAWHITE_BTLE_DW_BTLE_A::DATAWHITE_BTLE_DW_BTLE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DATAWHITE_BTLE_DW_BTLE_DEFAULT`"]
    #[inline(always)]
    pub fn is_datawhite_btle_dw_btle_default(&self) -> bool {
        *self == DATAWHITE_BTLE_DW_BTLE_A::DATAWHITE_BTLE_DW_BTLE_DEFAULT
    }
}
#[doc = "Write proxy for field `DATAWHITE_BTLE_DW_BTLE`"]
pub struct DATAWHITE_BTLE_DW_BTLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAWHITE_BTLE_DW_BTLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATAWHITE_BTLE_DW_BTLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn datawhite_btle_dw_btle_default(self) -> &'a mut W {
        self.variant(DATAWHITE_BTLE_DW_BTLE_A::DATAWHITE_BTLE_DW_BTLE_DEFAULT)
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
#[doc = "Reset value to put on the Bluetooth LE data whitening shift register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATAWHITE_BTLE_DW_BTLE_RST_A {
    #[doc = "0: `0`"]
    DATAWHITE_BTLE_DW_BTLE_RST_DEFAULT = 0,
}
impl From<DATAWHITE_BTLE_DW_BTLE_RST_A> for u8 {
    #[inline(always)]
    fn from(variant: DATAWHITE_BTLE_DW_BTLE_RST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DATAWHITE_BTLE_DW_BTLE_RST`"]
pub type DATAWHITE_BTLE_DW_BTLE_RST_R = crate::R<u8, DATAWHITE_BTLE_DW_BTLE_RST_A>;
impl DATAWHITE_BTLE_DW_BTLE_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DATAWHITE_BTLE_DW_BTLE_RST_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DATAWHITE_BTLE_DW_BTLE_RST_A::DATAWHITE_BTLE_DW_BTLE_RST_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DATAWHITE_BTLE_DW_BTLE_RST_DEFAULT`"]
    #[inline(always)]
    pub fn is_datawhite_btle_dw_btle_rst_default(&self) -> bool {
        *self == DATAWHITE_BTLE_DW_BTLE_RST_A::DATAWHITE_BTLE_DW_BTLE_RST_DEFAULT
    }
}
#[doc = "Write proxy for field `DATAWHITE_BTLE_DW_BTLE_RST`"]
pub struct DATAWHITE_BTLE_DW_BTLE_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAWHITE_BTLE_DW_BTLE_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATAWHITE_BTLE_DW_BTLE_RST_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn datawhite_btle_dw_btle_rst_default(self) -> &'a mut W {
        self.variant(DATAWHITE_BTLE_DW_BTLE_RST_A::DATAWHITE_BTLE_DW_BTLE_RST_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
#[doc = "If set to 1 the 4FSK coding is activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOURFSK_CODING_EN_FOURFSK_CODING_A {
    #[doc = "0: `0`"]
    FOURFSK_CODING_EN_FOURFSK_CODING_DEFAULT = 0,
}
impl From<FOURFSK_CODING_EN_FOURFSK_CODING_A> for bool {
    #[inline(always)]
    fn from(variant: FOURFSK_CODING_EN_FOURFSK_CODING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FOURFSK_CODING_EN_FOURFSK_CODING`"]
pub type FOURFSK_CODING_EN_FOURFSK_CODING_R = crate::R<bool, FOURFSK_CODING_EN_FOURFSK_CODING_A>;
impl FOURFSK_CODING_EN_FOURFSK_CODING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FOURFSK_CODING_EN_FOURFSK_CODING_A> {
        use crate::Variant::*;
        match self.bits {
            false => {
                Val(FOURFSK_CODING_EN_FOURFSK_CODING_A::FOURFSK_CODING_EN_FOURFSK_CODING_DEFAULT)
            }
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FOURFSK_CODING_EN_FOURFSK_CODING_DEFAULT`"]
    #[inline(always)]
    pub fn is_fourfsk_coding_en_fourfsk_coding_default(&self) -> bool {
        *self == FOURFSK_CODING_EN_FOURFSK_CODING_A::FOURFSK_CODING_EN_FOURFSK_CODING_DEFAULT
    }
}
#[doc = "Write proxy for field `FOURFSK_CODING_EN_FOURFSK_CODING`"]
pub struct FOURFSK_CODING_EN_FOURFSK_CODING_W<'a> {
    w: &'a mut W,
}
impl<'a> FOURFSK_CODING_EN_FOURFSK_CODING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOURFSK_CODING_EN_FOURFSK_CODING_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fourfsk_coding_en_fourfsk_coding_default(self) -> &'a mut W {
        self.variant(FOURFSK_CODING_EN_FOURFSK_CODING_A::FOURFSK_CODING_EN_FOURFSK_CODING_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Set the 4FSK coding (Tx): bit 0 determine if the sign is given by the Q signal (0) or I signal (1), bit 1 select if the signal is inverted for the sign, it 2 select if the signal is inverted for the abs amplitude\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FOURFSK_CODING_TX_FOURFSK_CODING_A {
    #[doc = "0: `0`"]
    FOURFSK_CODING_TX_FOURFSK_CODING_DEFAULT = 0,
}
impl From<FOURFSK_CODING_TX_FOURFSK_CODING_A> for u8 {
    #[inline(always)]
    fn from(variant: FOURFSK_CODING_TX_FOURFSK_CODING_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FOURFSK_CODING_TX_FOURFSK_CODING`"]
pub type FOURFSK_CODING_TX_FOURFSK_CODING_R = crate::R<u8, FOURFSK_CODING_TX_FOURFSK_CODING_A>;
impl FOURFSK_CODING_TX_FOURFSK_CODING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FOURFSK_CODING_TX_FOURFSK_CODING_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FOURFSK_CODING_TX_FOURFSK_CODING_A::FOURFSK_CODING_TX_FOURFSK_CODING_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FOURFSK_CODING_TX_FOURFSK_CODING_DEFAULT`"]
    #[inline(always)]
    pub fn is_fourfsk_coding_tx_fourfsk_coding_default(&self) -> bool {
        *self == FOURFSK_CODING_TX_FOURFSK_CODING_A::FOURFSK_CODING_TX_FOURFSK_CODING_DEFAULT
    }
}
#[doc = "Write proxy for field `FOURFSK_CODING_TX_FOURFSK_CODING`"]
pub struct FOURFSK_CODING_TX_FOURFSK_CODING_W<'a> {
    w: &'a mut W,
}
impl<'a> FOURFSK_CODING_TX_FOURFSK_CODING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOURFSK_CODING_TX_FOURFSK_CODING_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fourfsk_coding_tx_fourfsk_coding_default(self) -> &'a mut W {
        self.variant(FOURFSK_CODING_TX_FOURFSK_CODING_A::FOURFSK_CODING_TX_FOURFSK_CODING_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Set the 4FSK decoding (Rx): bit 0 determine if the sign is given by the Q signal (0) or I signal (1), bit 1 select if the signal is inverted for the sign, it 2 select if the signal is inverted for the abs amplitude\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FOURFSK_CODING_RX_FOURFSK_CODING_A {
    #[doc = "0: `0`"]
    FOURFSK_CODING_RX_FOURFSK_CODING_DEFAULT = 0,
}
impl From<FOURFSK_CODING_RX_FOURFSK_CODING_A> for u8 {
    #[inline(always)]
    fn from(variant: FOURFSK_CODING_RX_FOURFSK_CODING_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FOURFSK_CODING_RX_FOURFSK_CODING`"]
pub type FOURFSK_CODING_RX_FOURFSK_CODING_R = crate::R<u8, FOURFSK_CODING_RX_FOURFSK_CODING_A>;
impl FOURFSK_CODING_RX_FOURFSK_CODING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FOURFSK_CODING_RX_FOURFSK_CODING_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FOURFSK_CODING_RX_FOURFSK_CODING_A::FOURFSK_CODING_RX_FOURFSK_CODING_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FOURFSK_CODING_RX_FOURFSK_CODING_DEFAULT`"]
    #[inline(always)]
    pub fn is_fourfsk_coding_rx_fourfsk_coding_default(&self) -> bool {
        *self == FOURFSK_CODING_RX_FOURFSK_CODING_A::FOURFSK_CODING_RX_FOURFSK_CODING_DEFAULT
    }
}
#[doc = "Write proxy for field `FOURFSK_CODING_RX_FOURFSK_CODING`"]
pub struct FOURFSK_CODING_RX_FOURFSK_CODING_W<'a> {
    w: &'a mut W,
}
impl<'a> FOURFSK_CODING_RX_FOURFSK_CODING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOURFSK_CODING_RX_FOURFSK_CODING_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fourfsk_coding_rx_fourfsk_coding_default(self) -> &'a mut W {
        self.variant(FOURFSK_CODING_RX_FOURFSK_CODING_A::FOURFSK_CODING_RX_FOURFSK_CODING_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "If set to 1 enables the differential coding/decoding\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE2_DIFF_CODING_A {
    #[doc = "0: `0`"]
    MODE2_DIFF_CODING_DEFAULT = 0,
}
impl From<MODE2_DIFF_CODING_A> for bool {
    #[inline(always)]
    fn from(variant: MODE2_DIFF_CODING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE2_DIFF_CODING`"]
pub type MODE2_DIFF_CODING_R = crate::R<bool, MODE2_DIFF_CODING_A>;
impl MODE2_DIFF_CODING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MODE2_DIFF_CODING_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MODE2_DIFF_CODING_A::MODE2_DIFF_CODING_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MODE2_DIFF_CODING_DEFAULT`"]
    #[inline(always)]
    pub fn is_mode2_diff_coding_default(&self) -> bool {
        *self == MODE2_DIFF_CODING_A::MODE2_DIFF_CODING_DEFAULT
    }
}
#[doc = "Write proxy for field `MODE2_DIFF_CODING`"]
pub struct MODE2_DIFF_CODING_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE2_DIFF_CODING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE2_DIFF_CODING_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mode2_diff_coding_default(self) -> &'a mut W {
        self.variant(MODE2_DIFF_CODING_A::MODE2_DIFF_CODING_DEFAULT)
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
#[doc = "If set to 1, the PSK mode is selected, FSK otherwise.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE2_PSK_NFSK_A {
    #[doc = "0: `0`"]
    MODE2_PSK_NFSK_DEFAULT = 0,
}
impl From<MODE2_PSK_NFSK_A> for bool {
    #[inline(always)]
    fn from(variant: MODE2_PSK_NFSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE2_PSK_NFSK`"]
pub type MODE2_PSK_NFSK_R = crate::R<bool, MODE2_PSK_NFSK_A>;
impl MODE2_PSK_NFSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MODE2_PSK_NFSK_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MODE2_PSK_NFSK_A::MODE2_PSK_NFSK_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MODE2_PSK_NFSK_DEFAULT`"]
    #[inline(always)]
    pub fn is_mode2_psk_nfsk_default(&self) -> bool {
        *self == MODE2_PSK_NFSK_A::MODE2_PSK_NFSK_DEFAULT
    }
}
#[doc = "Write proxy for field `MODE2_PSK_NFSK`"]
pub struct MODE2_PSK_NFSK_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE2_PSK_NFSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE2_PSK_NFSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mode2_psk_nfsk_default(self) -> &'a mut W {
        self.variant(MODE2_PSK_NFSK_A::MODE2_PSK_NFSK_DEFAULT)
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
#[doc = "set the output testmode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE2_TESTMODE_A {
    #[doc = "0: `0`"]
    MODE2_TESTMODE_DEFAULT = 0,
}
impl From<MODE2_TESTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE2_TESTMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE2_TESTMODE`"]
pub type MODE2_TESTMODE_R = crate::R<u8, MODE2_TESTMODE_A>;
impl MODE2_TESTMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE2_TESTMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE2_TESTMODE_A::MODE2_TESTMODE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MODE2_TESTMODE_DEFAULT`"]
    #[inline(always)]
    pub fn is_mode2_testmode_default(&self) -> bool {
        *self == MODE2_TESTMODE_A::MODE2_TESTMODE_DEFAULT
    }
}
#[doc = "Write proxy for field `MODE2_TESTMODE`"]
pub struct MODE2_TESTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE2_TESTMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE2_TESTMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mode2_testmode_default(self) -> &'a mut W {
        self.variant(MODE2_TESTMODE_A::MODE2_TESTMODE_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "In FSM mode, if set to 1 indicates to the FSM to go in suspend mode after a Tx or Rx packet\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_NOT_TO_IDLE_A {
    #[doc = "0: `0`"]
    MODE_NOT_TO_IDLE_DEFAULT = 0,
}
impl From<MODE_NOT_TO_IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_NOT_TO_IDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE_NOT_TO_IDLE`"]
pub type MODE_NOT_TO_IDLE_R = crate::R<bool, MODE_NOT_TO_IDLE_A>;
impl MODE_NOT_TO_IDLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MODE_NOT_TO_IDLE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MODE_NOT_TO_IDLE_A::MODE_NOT_TO_IDLE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MODE_NOT_TO_IDLE_DEFAULT`"]
    #[inline(always)]
    pub fn is_mode_not_to_idle_default(&self) -> bool {
        *self == MODE_NOT_TO_IDLE_A::MODE_NOT_TO_IDLE_DEFAULT
    }
}
#[doc = "Write proxy for field `MODE_NOT_TO_IDLE`"]
pub struct MODE_NOT_TO_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_NOT_TO_IDLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_NOT_TO_IDLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mode_not_to_idle_default(self) -> &'a mut W {
        self.variant(MODE_NOT_TO_IDLE_A::MODE_NOT_TO_IDLE_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "If set to 1 enables the radio FSM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_EN_FSM_A {
    #[doc = "0: `0`"]
    MODE_EN_FSM_DEFAULT = 0,
}
impl From<MODE_EN_FSM_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_EN_FSM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE_EN_FSM`"]
pub type MODE_EN_FSM_R = crate::R<bool, MODE_EN_FSM_A>;
impl MODE_EN_FSM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MODE_EN_FSM_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MODE_EN_FSM_A::MODE_EN_FSM_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MODE_EN_FSM_DEFAULT`"]
    #[inline(always)]
    pub fn is_mode_en_fsm_default(&self) -> bool {
        *self == MODE_EN_FSM_A::MODE_EN_FSM_DEFAULT
    }
}
#[doc = "Write proxy for field `MODE_EN_FSM`"]
pub struct MODE_EN_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_EN_FSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_EN_FSM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mode_en_fsm_default(self) -> &'a mut W {
        self.variant(MODE_EN_FSM_A::MODE_EN_FSM_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "If set to 1 enables the deserializer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_EN_DESERIALIZER_A {
    #[doc = "0: `0`"]
    MODE_EN_DESERIALIZER_DEFAULT = 0,
}
impl From<MODE_EN_DESERIALIZER_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_EN_DESERIALIZER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE_EN_DESERIALIZER`"]
pub type MODE_EN_DESERIALIZER_R = crate::R<bool, MODE_EN_DESERIALIZER_A>;
impl MODE_EN_DESERIALIZER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MODE_EN_DESERIALIZER_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MODE_EN_DESERIALIZER_A::MODE_EN_DESERIALIZER_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MODE_EN_DESERIALIZER_DEFAULT`"]
    #[inline(always)]
    pub fn is_mode_en_deserializer_default(&self) -> bool {
        *self == MODE_EN_DESERIALIZER_A::MODE_EN_DESERIALIZER_DEFAULT
    }
}
#[doc = "Write proxy for field `MODE_EN_DESERIALIZER`"]
pub struct MODE_EN_DESERIALIZER_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_EN_DESERIALIZER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_EN_DESERIALIZER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mode_en_deserializer_default(self) -> &'a mut W {
        self.variant(MODE_EN_DESERIALIZER_A::MODE_EN_DESERIALIZER_DEFAULT)
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
#[doc = "If set to 1 enables the serializer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_EN_SERIALIZER_A {
    #[doc = "0: `0`"]
    MODE_EN_SERIALIZER_DEFAULT = 0,
}
impl From<MODE_EN_SERIALIZER_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_EN_SERIALIZER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE_EN_SERIALIZER`"]
pub type MODE_EN_SERIALIZER_R = crate::R<bool, MODE_EN_SERIALIZER_A>;
impl MODE_EN_SERIALIZER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MODE_EN_SERIALIZER_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MODE_EN_SERIALIZER_A::MODE_EN_SERIALIZER_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MODE_EN_SERIALIZER_DEFAULT`"]
    #[inline(always)]
    pub fn is_mode_en_serializer_default(&self) -> bool {
        *self == MODE_EN_SERIALIZER_A::MODE_EN_SERIALIZER_DEFAULT
    }
}
#[doc = "Write proxy for field `MODE_EN_SERIALIZER`"]
pub struct MODE_EN_SERIALIZER_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_EN_SERIALIZER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_EN_SERIALIZER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mode_en_serializer_default(self) -> &'a mut W {
        self.variant(MODE_EN_SERIALIZER_A::MODE_EN_SERIALIZER_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "if set to 1 use the Tx, otherwise the Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_TX_NRX_A {
    #[doc = "0: `0`"]
    MODE_TX_NRX_DEFAULT = 0,
}
impl From<MODE_TX_NRX_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_TX_NRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE_TX_NRX`"]
pub type MODE_TX_NRX_R = crate::R<bool, MODE_TX_NRX_A>;
impl MODE_TX_NRX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MODE_TX_NRX_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MODE_TX_NRX_A::MODE_TX_NRX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MODE_TX_NRX_DEFAULT`"]
    #[inline(always)]
    pub fn is_mode_tx_nrx_default(&self) -> bool {
        *self == MODE_TX_NRX_A::MODE_TX_NRX_DEFAULT
    }
}
#[doc = "Write proxy for field `MODE_TX_NRX`"]
pub struct MODE_TX_NRX_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_TX_NRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_TX_NRX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mode_tx_nrx_default(self) -> &'a mut W {
        self.variant(MODE_TX_NRX_A::MODE_TX_NRX_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Select the working mode of the digital baseband: 00) the digital baseband is off (no ck) 01) the clock is generated but the blocks are reset (Tx,Rx,FIFOs and FSM) 10) 10: the digital baseband is freezed 11) working\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_MODE_A {
    #[doc = "0: `0`"]
    MODE_MODE_DEFAULT = 0,
}
impl From<MODE_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE_MODE`"]
pub type MODE_MODE_R = crate::R<u8, MODE_MODE_A>;
impl MODE_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_MODE_A::MODE_MODE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MODE_MODE_DEFAULT`"]
    #[inline(always)]
    pub fn is_mode_mode_default(&self) -> bool {
        *self == MODE_MODE_A::MODE_MODE_DEFAULT
    }
}
#[doc = "Write proxy for field `MODE_MODE`"]
pub struct MODE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mode_mode_default(self) -> &'a mut W {
        self.variant(MODE_MODE_A::MODE_MODE_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - If set to 1, the data whitening specified in the Bluetooth LE standard is used. Note that the en_datawhite field of the CODING register has also to be set to 1"]
    #[inline(always)]
    pub fn datawhite_btle_dw_btle(&self) -> DATAWHITE_BTLE_DW_BTLE_R {
        DATAWHITE_BTLE_DW_BTLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 24:30 - Reset value to put on the Bluetooth LE data whitening shift register"]
    #[inline(always)]
    pub fn datawhite_btle_dw_btle_rst(&self) -> DATAWHITE_BTLE_DW_BTLE_RST_R {
        DATAWHITE_BTLE_DW_BTLE_RST_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - If set to 1 the 4FSK coding is activated"]
    #[inline(always)]
    pub fn fourfsk_coding_en_fourfsk_coding(&self) -> FOURFSK_CODING_EN_FOURFSK_CODING_R {
        FOURFSK_CODING_EN_FOURFSK_CODING_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - Set the 4FSK coding (Tx): bit 0 determine if the sign is given by the Q signal (0) or I signal (1), bit 1 select if the signal is inverted for the sign, it 2 select if the signal is inverted for the abs amplitude"]
    #[inline(always)]
    pub fn fourfsk_coding_tx_fourfsk_coding(&self) -> FOURFSK_CODING_TX_FOURFSK_CODING_R {
        FOURFSK_CODING_TX_FOURFSK_CODING_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Set the 4FSK decoding (Rx): bit 0 determine if the sign is given by the Q signal (0) or I signal (1), bit 1 select if the signal is inverted for the sign, it 2 select if the signal is inverted for the abs amplitude"]
    #[inline(always)]
    pub fn fourfsk_coding_rx_fourfsk_coding(&self) -> FOURFSK_CODING_RX_FOURFSK_CODING_R {
        FOURFSK_CODING_RX_FOURFSK_CODING_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 14 - If set to 1 enables the differential coding/decoding"]
    #[inline(always)]
    pub fn mode2_diff_coding(&self) -> MODE2_DIFF_CODING_R {
        MODE2_DIFF_CODING_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - If set to 1, the PSK mode is selected, FSK otherwise."]
    #[inline(always)]
    pub fn mode2_psk_nfsk(&self) -> MODE2_PSK_NFSK_R {
        MODE2_PSK_NFSK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - set the output testmode"]
    #[inline(always)]
    pub fn mode2_testmode(&self) -> MODE2_TESTMODE_R {
        MODE2_TESTMODE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - In FSM mode, if set to 1 indicates to the FSM to go in suspend mode after a Tx or Rx packet"]
    #[inline(always)]
    pub fn mode_not_to_idle(&self) -> MODE_NOT_TO_IDLE_R {
        MODE_NOT_TO_IDLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If set to 1 enables the radio FSM"]
    #[inline(always)]
    pub fn mode_en_fsm(&self) -> MODE_EN_FSM_R {
        MODE_EN_FSM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - If set to 1 enables the deserializer"]
    #[inline(always)]
    pub fn mode_en_deserializer(&self) -> MODE_EN_DESERIALIZER_R {
        MODE_EN_DESERIALIZER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If set to 1 enables the serializer"]
    #[inline(always)]
    pub fn mode_en_serializer(&self) -> MODE_EN_SERIALIZER_R {
        MODE_EN_SERIALIZER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - if set to 1 use the Tx, otherwise the Rx"]
    #[inline(always)]
    pub fn mode_tx_nrx(&self) -> MODE_TX_NRX_R {
        MODE_TX_NRX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - Select the working mode of the digital baseband: 00) the digital baseband is off (no ck) 01) the clock is generated but the blocks are reset (Tx,Rx,FIFOs and FSM) 10) 10: the digital baseband is freezed 11) working"]
    #[inline(always)]
    pub fn mode_mode(&self) -> MODE_MODE_R {
        MODE_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - If set to 1, the data whitening specified in the Bluetooth LE standard is used. Note that the en_datawhite field of the CODING register has also to be set to 1"]
    #[inline(always)]
    pub fn datawhite_btle_dw_btle(&mut self) -> DATAWHITE_BTLE_DW_BTLE_W {
        DATAWHITE_BTLE_DW_BTLE_W { w: self }
    }
    #[doc = "Bits 24:30 - Reset value to put on the Bluetooth LE data whitening shift register"]
    #[inline(always)]
    pub fn datawhite_btle_dw_btle_rst(&mut self) -> DATAWHITE_BTLE_DW_BTLE_RST_W {
        DATAWHITE_BTLE_DW_BTLE_RST_W { w: self }
    }
    #[doc = "Bit 23 - If set to 1 the 4FSK coding is activated"]
    #[inline(always)]
    pub fn fourfsk_coding_en_fourfsk_coding(&mut self) -> FOURFSK_CODING_EN_FOURFSK_CODING_W {
        FOURFSK_CODING_EN_FOURFSK_CODING_W { w: self }
    }
    #[doc = "Bits 20:22 - Set the 4FSK coding (Tx): bit 0 determine if the sign is given by the Q signal (0) or I signal (1), bit 1 select if the signal is inverted for the sign, it 2 select if the signal is inverted for the abs amplitude"]
    #[inline(always)]
    pub fn fourfsk_coding_tx_fourfsk_coding(&mut self) -> FOURFSK_CODING_TX_FOURFSK_CODING_W {
        FOURFSK_CODING_TX_FOURFSK_CODING_W { w: self }
    }
    #[doc = "Bits 16:18 - Set the 4FSK decoding (Rx): bit 0 determine if the sign is given by the Q signal (0) or I signal (1), bit 1 select if the signal is inverted for the sign, it 2 select if the signal is inverted for the abs amplitude"]
    #[inline(always)]
    pub fn fourfsk_coding_rx_fourfsk_coding(&mut self) -> FOURFSK_CODING_RX_FOURFSK_CODING_W {
        FOURFSK_CODING_RX_FOURFSK_CODING_W { w: self }
    }
    #[doc = "Bit 14 - If set to 1 enables the differential coding/decoding"]
    #[inline(always)]
    pub fn mode2_diff_coding(&mut self) -> MODE2_DIFF_CODING_W {
        MODE2_DIFF_CODING_W { w: self }
    }
    #[doc = "Bit 13 - If set to 1, the PSK mode is selected, FSK otherwise."]
    #[inline(always)]
    pub fn mode2_psk_nfsk(&mut self) -> MODE2_PSK_NFSK_W {
        MODE2_PSK_NFSK_W { w: self }
    }
    #[doc = "Bits 8:12 - set the output testmode"]
    #[inline(always)]
    pub fn mode2_testmode(&mut self) -> MODE2_TESTMODE_W {
        MODE2_TESTMODE_W { w: self }
    }
    #[doc = "Bit 7 - In FSM mode, if set to 1 indicates to the FSM to go in suspend mode after a Tx or Rx packet"]
    #[inline(always)]
    pub fn mode_not_to_idle(&mut self) -> MODE_NOT_TO_IDLE_W {
        MODE_NOT_TO_IDLE_W { w: self }
    }
    #[doc = "Bit 5 - If set to 1 enables the radio FSM"]
    #[inline(always)]
    pub fn mode_en_fsm(&mut self) -> MODE_EN_FSM_W {
        MODE_EN_FSM_W { w: self }
    }
    #[doc = "Bit 4 - If set to 1 enables the deserializer"]
    #[inline(always)]
    pub fn mode_en_deserializer(&mut self) -> MODE_EN_DESERIALIZER_W {
        MODE_EN_DESERIALIZER_W { w: self }
    }
    #[doc = "Bit 3 - If set to 1 enables the serializer"]
    #[inline(always)]
    pub fn mode_en_serializer(&mut self) -> MODE_EN_SERIALIZER_W {
        MODE_EN_SERIALIZER_W { w: self }
    }
    #[doc = "Bit 2 - if set to 1 use the Tx, otherwise the Rx"]
    #[inline(always)]
    pub fn mode_tx_nrx(&mut self) -> MODE_TX_NRX_W {
        MODE_TX_NRX_W { w: self }
    }
    #[doc = "Bits 0:1 - Select the working mode of the digital baseband: 00) the digital baseband is off (no ck) 01) the clock is generated but the blocks are reset (Tx,Rx,FIFOs and FSM) 10) 10: the digital baseband is freezed 11) working"]
    #[inline(always)]
    pub fn mode_mode(&mut self) -> MODE_MODE_W {
        MODE_MODE_W { w: self }
    }
}
