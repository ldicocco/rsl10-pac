#[doc = "Reader of register RF_REG1B"]
pub type R = crate::R<u32, super::RF_REG1B>;
#[doc = "Writer for register RF_REG1B"]
pub type W = crate::W<u32, super::RF_REG1B>;
#[doc = "Register RF_REG1B `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG1B {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "If set to 1 enables the Tx data-whitening before the convolutional code block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEEE802154_OPTS_EN_DW_TEST_A {
    #[doc = "0: `0`"]
    IEEE802154_OPTS_EN_DW_TEST_DEFAULT = 0,
}
impl From<IEEE802154_OPTS_EN_DW_TEST_A> for bool {
    #[inline(always)]
    fn from(variant: IEEE802154_OPTS_EN_DW_TEST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IEEE802154_OPTS_EN_DW_TEST`"]
pub type IEEE802154_OPTS_EN_DW_TEST_R = crate::R<bool, IEEE802154_OPTS_EN_DW_TEST_A>;
impl IEEE802154_OPTS_EN_DW_TEST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, IEEE802154_OPTS_EN_DW_TEST_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(IEEE802154_OPTS_EN_DW_TEST_A::IEEE802154_OPTS_EN_DW_TEST_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IEEE802154_OPTS_EN_DW_TEST_DEFAULT`"]
    #[inline(always)]
    pub fn is_ieee802154_opts_en_dw_test_default(&self) -> bool {
        *self == IEEE802154_OPTS_EN_DW_TEST_A::IEEE802154_OPTS_EN_DW_TEST_DEFAULT
    }
}
#[doc = "Write proxy for field `IEEE802154_OPTS_EN_DW_TEST`"]
pub struct IEEE802154_OPTS_EN_DW_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> IEEE802154_OPTS_EN_DW_TEST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEEE802154_OPTS_EN_DW_TEST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ieee802154_opts_en_dw_test_default(self) -> &'a mut W {
        self.variant(IEEE802154_OPTS_EN_DW_TEST_A::IEEE802154_OPTS_EN_DW_TEST_DEFAULT)
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
#[doc = "sets the clock output mode for BER mode or RW mode: 00 => data change on falling edge, 01 => data change on rising edge, 10 => clock signal is a toggled signal, 11 => enable signal from clock recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IEEE802154_OPTS_BER_CLK_MODE_A {
    #[doc = "0: `0`"]
    IEEE802154_OPTS_BER_CLK_MODE_DEFAULT = 0,
}
impl From<IEEE802154_OPTS_BER_CLK_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: IEEE802154_OPTS_BER_CLK_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IEEE802154_OPTS_BER_CLK_MODE`"]
pub type IEEE802154_OPTS_BER_CLK_MODE_R = crate::R<u8, IEEE802154_OPTS_BER_CLK_MODE_A>;
impl IEEE802154_OPTS_BER_CLK_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IEEE802154_OPTS_BER_CLK_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IEEE802154_OPTS_BER_CLK_MODE_A::IEEE802154_OPTS_BER_CLK_MODE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IEEE802154_OPTS_BER_CLK_MODE_DEFAULT`"]
    #[inline(always)]
    pub fn is_ieee802154_opts_ber_clk_mode_default(&self) -> bool {
        *self == IEEE802154_OPTS_BER_CLK_MODE_A::IEEE802154_OPTS_BER_CLK_MODE_DEFAULT
    }
}
#[doc = "Write proxy for field `IEEE802154_OPTS_BER_CLK_MODE`"]
pub struct IEEE802154_OPTS_BER_CLK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IEEE802154_OPTS_BER_CLK_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEEE802154_OPTS_BER_CLK_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ieee802154_opts_ber_clk_mode_default(self) -> &'a mut W {
        self.variant(IEEE802154_OPTS_BER_CLK_MODE_A::IEEE802154_OPTS_BER_CLK_MODE_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "If set to 1, the signal rx_data in testmodes is not sampled. Used for debug purposes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_A {
    #[doc = "0: `0`"]
    IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_DEFAULT = 0,
}
impl From<IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_A> for bool {
    #[inline(always)]
    fn from(variant: IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IEEE802154_OPTS_RX_DATA_NOT_SAMPLED`"]
pub type IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_R =
    crate::R<bool, IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_A>;
impl IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(
                IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_A::IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_DEFAULT,
            ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_DEFAULT`"]
    #[inline(always)]
    pub fn is_ieee802154_opts_rx_data_not_sampled_default(&self) -> bool {
        *self == IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_A::IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_DEFAULT
    }
}
#[doc = "Write proxy for field `IEEE802154_OPTS_RX_DATA_NOT_SAMPLED`"]
pub struct IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_W<'a> {
    w: &'a mut W,
}
impl<'a> IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ieee802154_opts_rx_data_not_sampled_default(self) -> &'a mut W {
        self.variant(
            IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_A::IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_DEFAULT,
        )
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "if set to 1 enables the frequency to linear conversion in the Rx side (always controlled by the en_802154_l2f configuration bit).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEEE802154_OPTS_EN_L2F_RX_A {
    #[doc = "0: `0`"]
    IEEE802154_OPTS_EN_L2F_RX_DEFAULT = 0,
}
impl From<IEEE802154_OPTS_EN_L2F_RX_A> for bool {
    #[inline(always)]
    fn from(variant: IEEE802154_OPTS_EN_L2F_RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IEEE802154_OPTS_EN_L2F_RX`"]
pub type IEEE802154_OPTS_EN_L2F_RX_R = crate::R<bool, IEEE802154_OPTS_EN_L2F_RX_A>;
impl IEEE802154_OPTS_EN_L2F_RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, IEEE802154_OPTS_EN_L2F_RX_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(IEEE802154_OPTS_EN_L2F_RX_A::IEEE802154_OPTS_EN_L2F_RX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IEEE802154_OPTS_EN_L2F_RX_DEFAULT`"]
    #[inline(always)]
    pub fn is_ieee802154_opts_en_l2f_rx_default(&self) -> bool {
        *self == IEEE802154_OPTS_EN_L2F_RX_A::IEEE802154_OPTS_EN_L2F_RX_DEFAULT
    }
}
#[doc = "Write proxy for field `IEEE802154_OPTS_EN_L2F_RX`"]
pub struct IEEE802154_OPTS_EN_L2F_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> IEEE802154_OPTS_EN_L2F_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEEE802154_OPTS_EN_L2F_RX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ieee802154_opts_en_l2f_rx_default(self) -> &'a mut W {
        self.variant(IEEE802154_OPTS_EN_L2F_RX_A::IEEE802154_OPTS_EN_L2F_RX_DEFAULT)
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
#[doc = "Threshold of the chip2bit correlator of the IEEE 802.15.4 protocol.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IEEE802154_OPTS_C2B_THR_A {
    #[doc = "0: `0`"]
    IEEE802154_OPTS_C2B_THR_DEFAULT = 0,
}
impl From<IEEE802154_OPTS_C2B_THR_A> for u8 {
    #[inline(always)]
    fn from(variant: IEEE802154_OPTS_C2B_THR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IEEE802154_OPTS_C2B_THR`"]
pub type IEEE802154_OPTS_C2B_THR_R = crate::R<u8, IEEE802154_OPTS_C2B_THR_A>;
impl IEEE802154_OPTS_C2B_THR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IEEE802154_OPTS_C2B_THR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IEEE802154_OPTS_C2B_THR_A::IEEE802154_OPTS_C2B_THR_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IEEE802154_OPTS_C2B_THR_DEFAULT`"]
    #[inline(always)]
    pub fn is_ieee802154_opts_c2b_thr_default(&self) -> bool {
        *self == IEEE802154_OPTS_C2B_THR_A::IEEE802154_OPTS_C2B_THR_DEFAULT
    }
}
#[doc = "Write proxy for field `IEEE802154_OPTS_C2B_THR`"]
pub struct IEEE802154_OPTS_C2B_THR_W<'a> {
    w: &'a mut W,
}
impl<'a> IEEE802154_OPTS_C2B_THR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEEE802154_OPTS_C2B_THR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ieee802154_opts_c2b_thr_default(self) -> &'a mut W {
        self.variant(IEEE802154_OPTS_C2B_THR_A::IEEE802154_OPTS_C2B_THR_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Time constant of the peak detector monostable circuit; if set to 0 the monostable is bypassed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_PEAK_DET_PEAK_DET_TAU_A {
    #[doc = "0: `0`"]
    AGC_PEAK_DET_PEAK_DET_TAU_DEFAULT = 0,
}
impl From<AGC_PEAK_DET_PEAK_DET_TAU_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_PEAK_DET_PEAK_DET_TAU_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_PEAK_DET_PEAK_DET_TAU`"]
pub type AGC_PEAK_DET_PEAK_DET_TAU_R = crate::R<u8, AGC_PEAK_DET_PEAK_DET_TAU_A>;
impl AGC_PEAK_DET_PEAK_DET_TAU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_PEAK_DET_PEAK_DET_TAU_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AGC_PEAK_DET_PEAK_DET_TAU_A::AGC_PEAK_DET_PEAK_DET_TAU_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_PEAK_DET_PEAK_DET_TAU_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_peak_det_peak_det_tau_default(&self) -> bool {
        *self == AGC_PEAK_DET_PEAK_DET_TAU_A::AGC_PEAK_DET_PEAK_DET_TAU_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_PEAK_DET_PEAK_DET_TAU`"]
pub struct AGC_PEAK_DET_PEAK_DET_TAU_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_PEAK_DET_PEAK_DET_TAU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_PEAK_DET_PEAK_DET_TAU_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_peak_det_peak_det_tau_default(self) -> &'a mut W {
        self.variant(AGC_PEAK_DET_PEAK_DET_TAU_A::AGC_PEAK_DET_PEAK_DET_TAU_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Threshold for the low level of the peak detector: 0 => 0, 1 => 1, 2 => 2, 3 => N.A.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_PEAK_DET_PEAK_DET_THR_LOW_A {
    #[doc = "0: `0`"]
    AGC_PEAK_DET_PEAK_DET_THR_LOW_DEFAULT = 0,
}
impl From<AGC_PEAK_DET_PEAK_DET_THR_LOW_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_PEAK_DET_PEAK_DET_THR_LOW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_PEAK_DET_PEAK_DET_THR_LOW`"]
pub type AGC_PEAK_DET_PEAK_DET_THR_LOW_R = crate::R<u8, AGC_PEAK_DET_PEAK_DET_THR_LOW_A>;
impl AGC_PEAK_DET_PEAK_DET_THR_LOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_PEAK_DET_PEAK_DET_THR_LOW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AGC_PEAK_DET_PEAK_DET_THR_LOW_A::AGC_PEAK_DET_PEAK_DET_THR_LOW_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_PEAK_DET_PEAK_DET_THR_LOW_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_peak_det_peak_det_thr_low_default(&self) -> bool {
        *self == AGC_PEAK_DET_PEAK_DET_THR_LOW_A::AGC_PEAK_DET_PEAK_DET_THR_LOW_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_PEAK_DET_PEAK_DET_THR_LOW`"]
pub struct AGC_PEAK_DET_PEAK_DET_THR_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_PEAK_DET_PEAK_DET_THR_LOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_PEAK_DET_PEAK_DET_THR_LOW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_peak_det_peak_det_thr_low_default(self) -> &'a mut W {
        self.variant(AGC_PEAK_DET_PEAK_DET_THR_LOW_A::AGC_PEAK_DET_PEAK_DET_THR_LOW_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Threshold for the high level of the peak detector: 0 => 2, 1 => 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AGC_PEAK_DET_PEAK_DET_THR_HIGH_A {
    #[doc = "0: `0`"]
    AGC_PEAK_DET_PEAK_DET_THR_HIGH_DEFAULT = 0,
}
impl From<AGC_PEAK_DET_PEAK_DET_THR_HIGH_A> for bool {
    #[inline(always)]
    fn from(variant: AGC_PEAK_DET_PEAK_DET_THR_HIGH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AGC_PEAK_DET_PEAK_DET_THR_HIGH`"]
pub type AGC_PEAK_DET_PEAK_DET_THR_HIGH_R = crate::R<bool, AGC_PEAK_DET_PEAK_DET_THR_HIGH_A>;
impl AGC_PEAK_DET_PEAK_DET_THR_HIGH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, AGC_PEAK_DET_PEAK_DET_THR_HIGH_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(AGC_PEAK_DET_PEAK_DET_THR_HIGH_A::AGC_PEAK_DET_PEAK_DET_THR_HIGH_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_PEAK_DET_PEAK_DET_THR_HIGH_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_peak_det_peak_det_thr_high_default(&self) -> bool {
        *self == AGC_PEAK_DET_PEAK_DET_THR_HIGH_A::AGC_PEAK_DET_PEAK_DET_THR_HIGH_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_PEAK_DET_PEAK_DET_THR_HIGH`"]
pub struct AGC_PEAK_DET_PEAK_DET_THR_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_PEAK_DET_PEAK_DET_THR_HIGH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_PEAK_DET_PEAK_DET_THR_HIGH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_peak_det_peak_det_thr_high_default(self) -> &'a mut W {
        self.variant(AGC_PEAK_DET_PEAK_DET_THR_HIGH_A::AGC_PEAK_DET_PEAK_DET_THR_HIGH_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "If set to 1 enables the AGC peak detector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AGC_PEAK_DET_EN_AGC_PEAK_A {
    #[doc = "0: `0`"]
    AGC_PEAK_DET_EN_AGC_PEAK_DEFAULT = 0,
}
impl From<AGC_PEAK_DET_EN_AGC_PEAK_A> for bool {
    #[inline(always)]
    fn from(variant: AGC_PEAK_DET_EN_AGC_PEAK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AGC_PEAK_DET_EN_AGC_PEAK`"]
pub type AGC_PEAK_DET_EN_AGC_PEAK_R = crate::R<bool, AGC_PEAK_DET_EN_AGC_PEAK_A>;
impl AGC_PEAK_DET_EN_AGC_PEAK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, AGC_PEAK_DET_EN_AGC_PEAK_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(AGC_PEAK_DET_EN_AGC_PEAK_A::AGC_PEAK_DET_EN_AGC_PEAK_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_PEAK_DET_EN_AGC_PEAK_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_peak_det_en_agc_peak_default(&self) -> bool {
        *self == AGC_PEAK_DET_EN_AGC_PEAK_A::AGC_PEAK_DET_EN_AGC_PEAK_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_PEAK_DET_EN_AGC_PEAK`"]
pub struct AGC_PEAK_DET_EN_AGC_PEAK_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_PEAK_DET_EN_AGC_PEAK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_PEAK_DET_EN_AGC_PEAK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_peak_det_en_agc_peak_default(self) -> &'a mut W {
        self.variant(AGC_PEAK_DET_EN_AGC_PEAK_A::AGC_PEAK_DET_EN_AGC_PEAK_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "AGC threshold high level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_THR_HIGH_AGC_THR_HIGH_A {
    #[doc = "0: `0`"]
    AGC_THR_HIGH_AGC_THR_HIGH_DEFAULT = 0,
}
impl From<AGC_THR_HIGH_AGC_THR_HIGH_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_THR_HIGH_AGC_THR_HIGH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_THR_HIGH_AGC_THR_HIGH`"]
pub type AGC_THR_HIGH_AGC_THR_HIGH_R = crate::R<u8, AGC_THR_HIGH_AGC_THR_HIGH_A>;
impl AGC_THR_HIGH_AGC_THR_HIGH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_THR_HIGH_AGC_THR_HIGH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AGC_THR_HIGH_AGC_THR_HIGH_A::AGC_THR_HIGH_AGC_THR_HIGH_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_THR_HIGH_AGC_THR_HIGH_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_thr_high_agc_thr_high_default(&self) -> bool {
        *self == AGC_THR_HIGH_AGC_THR_HIGH_A::AGC_THR_HIGH_AGC_THR_HIGH_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_THR_HIGH_AGC_THR_HIGH`"]
pub struct AGC_THR_HIGH_AGC_THR_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_THR_HIGH_AGC_THR_HIGH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_THR_HIGH_AGC_THR_HIGH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_thr_high_agc_thr_high_default(self) -> &'a mut W {
        self.variant(AGC_THR_HIGH_AGC_THR_HIGH_A::AGC_THR_HIGH_AGC_THR_HIGH_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "AGC threshold low level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_THR_LOW_AGC_THR_LOW_A {
    #[doc = "0: `0`"]
    AGC_THR_LOW_AGC_THR_LOW_DEFAULT = 0,
}
impl From<AGC_THR_LOW_AGC_THR_LOW_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_THR_LOW_AGC_THR_LOW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_THR_LOW_AGC_THR_LOW`"]
pub type AGC_THR_LOW_AGC_THR_LOW_R = crate::R<u8, AGC_THR_LOW_AGC_THR_LOW_A>;
impl AGC_THR_LOW_AGC_THR_LOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_THR_LOW_AGC_THR_LOW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AGC_THR_LOW_AGC_THR_LOW_A::AGC_THR_LOW_AGC_THR_LOW_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_THR_LOW_AGC_THR_LOW_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_thr_low_agc_thr_low_default(&self) -> bool {
        *self == AGC_THR_LOW_AGC_THR_LOW_A::AGC_THR_LOW_AGC_THR_LOW_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_THR_LOW_AGC_THR_LOW`"]
pub struct AGC_THR_LOW_AGC_THR_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_THR_LOW_AGC_THR_LOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_THR_LOW_AGC_THR_LOW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_thr_low_agc_thr_low_default(self) -> &'a mut W {
        self.variant(AGC_THR_LOW_AGC_THR_LOW_A::AGC_THR_LOW_AGC_THR_LOW_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - If set to 1 enables the Tx data-whitening before the convolutional code block"]
    #[inline(always)]
    pub fn ieee802154_opts_en_dw_test(&self) -> IEEE802154_OPTS_EN_DW_TEST_R {
        IEEE802154_OPTS_EN_DW_TEST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - sets the clock output mode for BER mode or RW mode: 00 => data change on falling edge, 01 => data change on rising edge, 10 => clock signal is a toggled signal, 11 => enable signal from clock recovery"]
    #[inline(always)]
    pub fn ieee802154_opts_ber_clk_mode(&self) -> IEEE802154_OPTS_BER_CLK_MODE_R {
        IEEE802154_OPTS_BER_CLK_MODE_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - If set to 1, the signal rx_data in testmodes is not sampled. Used for debug purposes"]
    #[inline(always)]
    pub fn ieee802154_opts_rx_data_not_sampled(&self) -> IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_R {
        IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - if set to 1 enables the frequency to linear conversion in the Rx side (always controlled by the en_802154_l2f configuration bit)."]
    #[inline(always)]
    pub fn ieee802154_opts_en_l2f_rx(&self) -> IEEE802154_OPTS_EN_L2F_RX_R {
        IEEE802154_OPTS_EN_L2F_RX_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Threshold of the chip2bit correlator of the IEEE 802.15.4 protocol."]
    #[inline(always)]
    pub fn ieee802154_opts_c2b_thr(&self) -> IEEE802154_OPTS_C2B_THR_R {
        IEEE802154_OPTS_C2B_THR_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 20:23 - Time constant of the peak detector monostable circuit; if set to 0 the monostable is bypassed"]
    #[inline(always)]
    pub fn agc_peak_det_peak_det_tau(&self) -> AGC_PEAK_DET_PEAK_DET_TAU_R {
        AGC_PEAK_DET_PEAK_DET_TAU_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 18:19 - Threshold for the low level of the peak detector: 0 => 0, 1 => 1, 2 => 2, 3 => N.A."]
    #[inline(always)]
    pub fn agc_peak_det_peak_det_thr_low(&self) -> AGC_PEAK_DET_PEAK_DET_THR_LOW_R {
        AGC_PEAK_DET_PEAK_DET_THR_LOW_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 17 - Threshold for the high level of the peak detector: 0 => 2, 1 => 3"]
    #[inline(always)]
    pub fn agc_peak_det_peak_det_thr_high(&self) -> AGC_PEAK_DET_PEAK_DET_THR_HIGH_R {
        AGC_PEAK_DET_PEAK_DET_THR_HIGH_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - If set to 1 enables the AGC peak detector"]
    #[inline(always)]
    pub fn agc_peak_det_en_agc_peak(&self) -> AGC_PEAK_DET_EN_AGC_PEAK_R {
        AGC_PEAK_DET_EN_AGC_PEAK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - AGC threshold high level"]
    #[inline(always)]
    pub fn agc_thr_high_agc_thr_high(&self) -> AGC_THR_HIGH_AGC_THR_HIGH_R {
        AGC_THR_HIGH_AGC_THR_HIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - AGC threshold low level"]
    #[inline(always)]
    pub fn agc_thr_low_agc_thr_low(&self) -> AGC_THR_LOW_AGC_THR_LOW_R {
        AGC_THR_LOW_AGC_THR_LOW_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - If set to 1 enables the Tx data-whitening before the convolutional code block"]
    #[inline(always)]
    pub fn ieee802154_opts_en_dw_test(&mut self) -> IEEE802154_OPTS_EN_DW_TEST_W {
        IEEE802154_OPTS_EN_DW_TEST_W { w: self }
    }
    #[doc = "Bits 29:30 - sets the clock output mode for BER mode or RW mode: 00 => data change on falling edge, 01 => data change on rising edge, 10 => clock signal is a toggled signal, 11 => enable signal from clock recovery"]
    #[inline(always)]
    pub fn ieee802154_opts_ber_clk_mode(&mut self) -> IEEE802154_OPTS_BER_CLK_MODE_W {
        IEEE802154_OPTS_BER_CLK_MODE_W { w: self }
    }
    #[doc = "Bit 28 - If set to 1, the signal rx_data in testmodes is not sampled. Used for debug purposes"]
    #[inline(always)]
    pub fn ieee802154_opts_rx_data_not_sampled(&mut self) -> IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_W {
        IEEE802154_OPTS_RX_DATA_NOT_SAMPLED_W { w: self }
    }
    #[doc = "Bit 27 - if set to 1 enables the frequency to linear conversion in the Rx side (always controlled by the en_802154_l2f configuration bit)."]
    #[inline(always)]
    pub fn ieee802154_opts_en_l2f_rx(&mut self) -> IEEE802154_OPTS_EN_L2F_RX_W {
        IEEE802154_OPTS_EN_L2F_RX_W { w: self }
    }
    #[doc = "Bits 24:26 - Threshold of the chip2bit correlator of the IEEE 802.15.4 protocol."]
    #[inline(always)]
    pub fn ieee802154_opts_c2b_thr(&mut self) -> IEEE802154_OPTS_C2B_THR_W {
        IEEE802154_OPTS_C2B_THR_W { w: self }
    }
    #[doc = "Bits 20:23 - Time constant of the peak detector monostable circuit; if set to 0 the monostable is bypassed"]
    #[inline(always)]
    pub fn agc_peak_det_peak_det_tau(&mut self) -> AGC_PEAK_DET_PEAK_DET_TAU_W {
        AGC_PEAK_DET_PEAK_DET_TAU_W { w: self }
    }
    #[doc = "Bits 18:19 - Threshold for the low level of the peak detector: 0 => 0, 1 => 1, 2 => 2, 3 => N.A."]
    #[inline(always)]
    pub fn agc_peak_det_peak_det_thr_low(&mut self) -> AGC_PEAK_DET_PEAK_DET_THR_LOW_W {
        AGC_PEAK_DET_PEAK_DET_THR_LOW_W { w: self }
    }
    #[doc = "Bit 17 - Threshold for the high level of the peak detector: 0 => 2, 1 => 3"]
    #[inline(always)]
    pub fn agc_peak_det_peak_det_thr_high(&mut self) -> AGC_PEAK_DET_PEAK_DET_THR_HIGH_W {
        AGC_PEAK_DET_PEAK_DET_THR_HIGH_W { w: self }
    }
    #[doc = "Bit 16 - If set to 1 enables the AGC peak detector"]
    #[inline(always)]
    pub fn agc_peak_det_en_agc_peak(&mut self) -> AGC_PEAK_DET_EN_AGC_PEAK_W {
        AGC_PEAK_DET_EN_AGC_PEAK_W { w: self }
    }
    #[doc = "Bits 8:15 - AGC threshold high level"]
    #[inline(always)]
    pub fn agc_thr_high_agc_thr_high(&mut self) -> AGC_THR_HIGH_AGC_THR_HIGH_W {
        AGC_THR_HIGH_AGC_THR_HIGH_W { w: self }
    }
    #[doc = "Bits 0:7 - AGC threshold low level"]
    #[inline(always)]
    pub fn agc_thr_low_agc_thr_low(&mut self) -> AGC_THR_LOW_AGC_THR_LOW_W {
        AGC_THR_LOW_AGC_THR_LOW_W { w: self }
    }
}
