#[doc = "Reader of register RF_REG1A"]
pub type R = crate::R<u32, super::RF_REG1A>;
#[doc = "Writer for register RF_REG1A"]
pub type W = crate::W<u32, super::RF_REG1A>;
#[doc = "Register RF_REG1A `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG1A {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Maximum attenuation level in AGC algorithm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ATT_CTRL_ATT_CTRL_MAX_A {
    #[doc = "0: `0`"]
    ATT_CTRL_ATT_CTRL_MAX_DEFAULT = 0,
}
impl From<ATT_CTRL_ATT_CTRL_MAX_A> for u8 {
    #[inline(always)]
    fn from(variant: ATT_CTRL_ATT_CTRL_MAX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ATT_CTRL_ATT_CTRL_MAX`"]
pub type ATT_CTRL_ATT_CTRL_MAX_R = crate::R<u8, ATT_CTRL_ATT_CTRL_MAX_A>;
impl ATT_CTRL_ATT_CTRL_MAX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ATT_CTRL_ATT_CTRL_MAX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ATT_CTRL_ATT_CTRL_MAX_A::ATT_CTRL_ATT_CTRL_MAX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ATT_CTRL_ATT_CTRL_MAX_DEFAULT`"]
    #[inline(always)]
    pub fn is_att_ctrl_att_ctrl_max_default(&self) -> bool {
        *self == ATT_CTRL_ATT_CTRL_MAX_A::ATT_CTRL_ATT_CTRL_MAX_DEFAULT
    }
}
#[doc = "Write proxy for field `ATT_CTRL_ATT_CTRL_MAX`"]
pub struct ATT_CTRL_ATT_CTRL_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> ATT_CTRL_ATT_CTRL_MAX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATT_CTRL_ATT_CTRL_MAX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn att_ctrl_att_ctrl_max_default(self) -> &'a mut W {
        self.variant(ATT_CTRL_ATT_CTRL_MAX_A::ATT_CTRL_ATT_CTRL_MAX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Attuenuation level if the AGC is bypassed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ATT_CTRL_SET_RX_ATT_CTRL_A {
    #[doc = "0: `0`"]
    ATT_CTRL_SET_RX_ATT_CTRL_DEFAULT = 0,
}
impl From<ATT_CTRL_SET_RX_ATT_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: ATT_CTRL_SET_RX_ATT_CTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ATT_CTRL_SET_RX_ATT_CTRL`"]
pub type ATT_CTRL_SET_RX_ATT_CTRL_R = crate::R<u8, ATT_CTRL_SET_RX_ATT_CTRL_A>;
impl ATT_CTRL_SET_RX_ATT_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ATT_CTRL_SET_RX_ATT_CTRL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ATT_CTRL_SET_RX_ATT_CTRL_A::ATT_CTRL_SET_RX_ATT_CTRL_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ATT_CTRL_SET_RX_ATT_CTRL_DEFAULT`"]
    #[inline(always)]
    pub fn is_att_ctrl_set_rx_att_ctrl_default(&self) -> bool {
        *self == ATT_CTRL_SET_RX_ATT_CTRL_A::ATT_CTRL_SET_RX_ATT_CTRL_DEFAULT
    }
}
#[doc = "Write proxy for field `ATT_CTRL_SET_RX_ATT_CTRL`"]
pub struct ATT_CTRL_SET_RX_ATT_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ATT_CTRL_SET_RX_ATT_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATT_CTRL_SET_RX_ATT_CTRL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn att_ctrl_set_rx_att_ctrl_default(self) -> &'a mut W {
        self.variant(ATT_CTRL_SET_RX_ATT_CTRL_A::ATT_CTRL_SET_RX_ATT_CTRL_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Time constant of the decay speed; high values corresponds to a slow decay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSSI_CTRL_AGC_DECAY_TAU_A {
    #[doc = "0: `0`"]
    RSSI_CTRL_AGC_DECAY_TAU_DEFAULT = 0,
}
impl From<RSSI_CTRL_AGC_DECAY_TAU_A> for u8 {
    #[inline(always)]
    fn from(variant: RSSI_CTRL_AGC_DECAY_TAU_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSSI_CTRL_AGC_DECAY_TAU`"]
pub type RSSI_CTRL_AGC_DECAY_TAU_R = crate::R<u8, RSSI_CTRL_AGC_DECAY_TAU_A>;
impl RSSI_CTRL_AGC_DECAY_TAU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSSI_CTRL_AGC_DECAY_TAU_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RSSI_CTRL_AGC_DECAY_TAU_A::RSSI_CTRL_AGC_DECAY_TAU_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_CTRL_AGC_DECAY_TAU_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_ctrl_agc_decay_tau_default(&self) -> bool {
        *self == RSSI_CTRL_AGC_DECAY_TAU_A::RSSI_CTRL_AGC_DECAY_TAU_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_CTRL_AGC_DECAY_TAU`"]
pub struct RSSI_CTRL_AGC_DECAY_TAU_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_CTRL_AGC_DECAY_TAU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_CTRL_AGC_DECAY_TAU_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssi_ctrl_agc_decay_tau_default(self) -> &'a mut W {
        self.variant(RSSI_CTRL_AGC_DECAY_TAU_A::RSSI_CTRL_AGC_DECAY_TAU_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "If set to 1 the AGC algorithm uses the LNA bias.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSI_CTRL_AGC_USE_LNA_A {
    #[doc = "0: `0`"]
    RSSI_CTRL_AGC_USE_LNA_DEFAULT = 0,
}
impl From<RSSI_CTRL_AGC_USE_LNA_A> for bool {
    #[inline(always)]
    fn from(variant: RSSI_CTRL_AGC_USE_LNA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSSI_CTRL_AGC_USE_LNA`"]
pub type RSSI_CTRL_AGC_USE_LNA_R = crate::R<bool, RSSI_CTRL_AGC_USE_LNA_A>;
impl RSSI_CTRL_AGC_USE_LNA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RSSI_CTRL_AGC_USE_LNA_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(RSSI_CTRL_AGC_USE_LNA_A::RSSI_CTRL_AGC_USE_LNA_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_CTRL_AGC_USE_LNA_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_ctrl_agc_use_lna_default(&self) -> bool {
        *self == RSSI_CTRL_AGC_USE_LNA_A::RSSI_CTRL_AGC_USE_LNA_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_CTRL_AGC_USE_LNA`"]
pub struct RSSI_CTRL_AGC_USE_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_CTRL_AGC_USE_LNA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_CTRL_AGC_USE_LNA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssi_ctrl_agc_use_lna_default(self) -> &'a mut W {
        self.variant(RSSI_CTRL_AGC_USE_LNA_A::RSSI_CTRL_AGC_USE_LNA_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Select the AGC algorithm: 0 -> old algorithm, 1 -> new algorithm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSI_CTRL_AGC_MODE_A {
    #[doc = "0: `0`"]
    RSSI_CTRL_AGC_MODE_DEFAULT = 0,
}
impl From<RSSI_CTRL_AGC_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: RSSI_CTRL_AGC_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSSI_CTRL_AGC_MODE`"]
pub type RSSI_CTRL_AGC_MODE_R = crate::R<bool, RSSI_CTRL_AGC_MODE_A>;
impl RSSI_CTRL_AGC_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RSSI_CTRL_AGC_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(RSSI_CTRL_AGC_MODE_A::RSSI_CTRL_AGC_MODE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_CTRL_AGC_MODE_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_ctrl_agc_mode_default(&self) -> bool {
        *self == RSSI_CTRL_AGC_MODE_A::RSSI_CTRL_AGC_MODE_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_CTRL_AGC_MODE`"]
pub struct RSSI_CTRL_AGC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_CTRL_AGC_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_CTRL_AGC_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssi_ctrl_agc_mode_default(self) -> &'a mut W {
        self.variant(RSSI_CTRL_AGC_MODE_A::RSSI_CTRL_AGC_MODE_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Sets the wait time of the AGC after switching between states: 00 => don't wait, 01 => wait 1x RSSI filtering period, 10 => wait 2x RSSI filtering period, 11 => wait 3x RSSI filtering period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSSI_CTRL_AGC_WAIT_A {
    #[doc = "0: `0`"]
    RSSI_CTRL_AGC_WAIT_DEFAULT = 0,
}
impl From<RSSI_CTRL_AGC_WAIT_A> for u8 {
    #[inline(always)]
    fn from(variant: RSSI_CTRL_AGC_WAIT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSSI_CTRL_AGC_WAIT`"]
pub type RSSI_CTRL_AGC_WAIT_R = crate::R<u8, RSSI_CTRL_AGC_WAIT_A>;
impl RSSI_CTRL_AGC_WAIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSSI_CTRL_AGC_WAIT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RSSI_CTRL_AGC_WAIT_A::RSSI_CTRL_AGC_WAIT_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_CTRL_AGC_WAIT_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_ctrl_agc_wait_default(&self) -> bool {
        *self == RSSI_CTRL_AGC_WAIT_A::RSSI_CTRL_AGC_WAIT_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_CTRL_AGC_WAIT`"]
pub struct RSSI_CTRL_AGC_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_CTRL_AGC_WAIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_CTRL_AGC_WAIT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssi_ctrl_agc_wait_default(self) -> &'a mut W {
        self.variant(RSSI_CTRL_AGC_WAIT_A::RSSI_CTRL_AGC_WAIT_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "If set to 1, the AGC is blocked during the payload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSI_CTRL_PAYLOAD_BLOCKS_AGC_A {
    #[doc = "0: `0`"]
    RSSI_CTRL_PAYLOAD_BLOCKS_AGC_DEFAULT = 0,
}
impl From<RSSI_CTRL_PAYLOAD_BLOCKS_AGC_A> for bool {
    #[inline(always)]
    fn from(variant: RSSI_CTRL_PAYLOAD_BLOCKS_AGC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSSI_CTRL_PAYLOAD_BLOCKS_AGC`"]
pub type RSSI_CTRL_PAYLOAD_BLOCKS_AGC_R = crate::R<bool, RSSI_CTRL_PAYLOAD_BLOCKS_AGC_A>;
impl RSSI_CTRL_PAYLOAD_BLOCKS_AGC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RSSI_CTRL_PAYLOAD_BLOCKS_AGC_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(RSSI_CTRL_PAYLOAD_BLOCKS_AGC_A::RSSI_CTRL_PAYLOAD_BLOCKS_AGC_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_CTRL_PAYLOAD_BLOCKS_AGC_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_ctrl_payload_blocks_agc_default(&self) -> bool {
        *self == RSSI_CTRL_PAYLOAD_BLOCKS_AGC_A::RSSI_CTRL_PAYLOAD_BLOCKS_AGC_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_CTRL_PAYLOAD_BLOCKS_AGC`"]
pub struct RSSI_CTRL_PAYLOAD_BLOCKS_AGC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_CTRL_PAYLOAD_BLOCKS_AGC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_CTRL_PAYLOAD_BLOCKS_AGC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssi_ctrl_payload_blocks_agc_default(self) -> &'a mut W {
        self.variant(RSSI_CTRL_PAYLOAD_BLOCKS_AGC_A::RSSI_CTRL_PAYLOAD_BLOCKS_AGC_DEFAULT)
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
#[doc = "If set to 1, the AGC algorithm is bypassed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSI_CTRL_BYPASS_AGC_A {
    #[doc = "0: `0`"]
    RSSI_CTRL_BYPASS_AGC_DEFAULT = 0,
}
impl From<RSSI_CTRL_BYPASS_AGC_A> for bool {
    #[inline(always)]
    fn from(variant: RSSI_CTRL_BYPASS_AGC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSSI_CTRL_BYPASS_AGC`"]
pub type RSSI_CTRL_BYPASS_AGC_R = crate::R<bool, RSSI_CTRL_BYPASS_AGC_A>;
impl RSSI_CTRL_BYPASS_AGC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RSSI_CTRL_BYPASS_AGC_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(RSSI_CTRL_BYPASS_AGC_A::RSSI_CTRL_BYPASS_AGC_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_CTRL_BYPASS_AGC_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_ctrl_bypass_agc_default(&self) -> bool {
        *self == RSSI_CTRL_BYPASS_AGC_A::RSSI_CTRL_BYPASS_AGC_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_CTRL_BYPASS_AGC`"]
pub struct RSSI_CTRL_BYPASS_AGC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_CTRL_BYPASS_AGC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_CTRL_BYPASS_AGC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssi_ctrl_bypass_agc_default(self) -> &'a mut W {
        self.variant(RSSI_CTRL_BYPASS_AGC_A::RSSI_CTRL_BYPASS_AGC_DEFAULT)
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
#[doc = "Bias for the bandwidth of the channel filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILTER_BIAS_IQ_FI_BW_A {
    #[doc = "0: `0`"]
    FILTER_BIAS_IQ_FI_BW_DEFAULT = 0,
}
impl From<FILTER_BIAS_IQ_FI_BW_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_BIAS_IQ_FI_BW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FILTER_BIAS_IQ_FI_BW`"]
pub type FILTER_BIAS_IQ_FI_BW_R = crate::R<u8, FILTER_BIAS_IQ_FI_BW_A>;
impl FILTER_BIAS_IQ_FI_BW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FILTER_BIAS_IQ_FI_BW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FILTER_BIAS_IQ_FI_BW_A::FILTER_BIAS_IQ_FI_BW_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FILTER_BIAS_IQ_FI_BW_DEFAULT`"]
    #[inline(always)]
    pub fn is_filter_bias_iq_fi_bw_default(&self) -> bool {
        *self == FILTER_BIAS_IQ_FI_BW_A::FILTER_BIAS_IQ_FI_BW_DEFAULT
    }
}
#[doc = "Write proxy for field `FILTER_BIAS_IQ_FI_BW`"]
pub struct FILTER_BIAS_IQ_FI_BW_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_BIAS_IQ_FI_BW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTER_BIAS_IQ_FI_BW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn filter_bias_iq_fi_bw_default(self) -> &'a mut W {
        self.variant(FILTER_BIAS_IQ_FI_BW_A::FILTER_BIAS_IQ_FI_BW_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Bias for the central frequency of the channel filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILTER_BIAS_IQ_FI_FC_A {
    #[doc = "0: `0`"]
    FILTER_BIAS_IQ_FI_FC_DEFAULT = 0,
}
impl From<FILTER_BIAS_IQ_FI_FC_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_BIAS_IQ_FI_FC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FILTER_BIAS_IQ_FI_FC`"]
pub type FILTER_BIAS_IQ_FI_FC_R = crate::R<u8, FILTER_BIAS_IQ_FI_FC_A>;
impl FILTER_BIAS_IQ_FI_FC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FILTER_BIAS_IQ_FI_FC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FILTER_BIAS_IQ_FI_FC_A::FILTER_BIAS_IQ_FI_FC_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FILTER_BIAS_IQ_FI_FC_DEFAULT`"]
    #[inline(always)]
    pub fn is_filter_bias_iq_fi_fc_default(&self) -> bool {
        *self == FILTER_BIAS_IQ_FI_FC_A::FILTER_BIAS_IQ_FI_FC_DEFAULT
    }
}
#[doc = "Write proxy for field `FILTER_BIAS_IQ_FI_FC`"]
pub struct FILTER_BIAS_IQ_FI_FC_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_BIAS_IQ_FI_FC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTER_BIAS_IQ_FI_FC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn filter_bias_iq_fi_fc_default(self) -> &'a mut W {
        self.variant(FILTER_BIAS_IQ_FI_FC_A::FILTER_BIAS_IQ_FI_FC_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - Maximum attenuation level in AGC algorithm"]
    #[inline(always)]
    pub fn att_ctrl_att_ctrl_max(&self) -> ATT_CTRL_ATT_CTRL_MAX_R {
        ATT_CTRL_ATT_CTRL_MAX_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Attuenuation level if the AGC is bypassed"]
    #[inline(always)]
    pub fn att_ctrl_set_rx_att_ctrl(&self) -> ATT_CTRL_SET_RX_ATT_CTRL_R {
        ATT_CTRL_SET_RX_ATT_CTRL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - Time constant of the decay speed; high values corresponds to a slow decay"]
    #[inline(always)]
    pub fn rssi_ctrl_agc_decay_tau(&self) -> RSSI_CTRL_AGC_DECAY_TAU_R {
        RSSI_CTRL_AGC_DECAY_TAU_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 21 - If set to 1 the AGC algorithm uses the LNA bias."]
    #[inline(always)]
    pub fn rssi_ctrl_agc_use_lna(&self) -> RSSI_CTRL_AGC_USE_LNA_R {
        RSSI_CTRL_AGC_USE_LNA_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Select the AGC algorithm: 0 -> old algorithm, 1 -> new algorithm"]
    #[inline(always)]
    pub fn rssi_ctrl_agc_mode(&self) -> RSSI_CTRL_AGC_MODE_R {
        RSSI_CTRL_AGC_MODE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Sets the wait time of the AGC after switching between states: 00 => don't wait, 01 => wait 1x RSSI filtering period, 10 => wait 2x RSSI filtering period, 11 => wait 3x RSSI filtering period"]
    #[inline(always)]
    pub fn rssi_ctrl_agc_wait(&self) -> RSSI_CTRL_AGC_WAIT_R {
        RSSI_CTRL_AGC_WAIT_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 17 - If set to 1, the AGC is blocked during the payload"]
    #[inline(always)]
    pub fn rssi_ctrl_payload_blocks_agc(&self) -> RSSI_CTRL_PAYLOAD_BLOCKS_AGC_R {
        RSSI_CTRL_PAYLOAD_BLOCKS_AGC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - If set to 1, the AGC algorithm is bypassed"]
    #[inline(always)]
    pub fn rssi_ctrl_bypass_agc(&self) -> RSSI_CTRL_BYPASS_AGC_R {
        RSSI_CTRL_BYPASS_AGC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Bias for the bandwidth of the channel filter"]
    #[inline(always)]
    pub fn filter_bias_iq_fi_bw(&self) -> FILTER_BIAS_IQ_FI_BW_R {
        FILTER_BIAS_IQ_FI_BW_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - Bias for the central frequency of the channel filter"]
    #[inline(always)]
    pub fn filter_bias_iq_fi_fc(&self) -> FILTER_BIAS_IQ_FI_FC_R {
        FILTER_BIAS_IQ_FI_FC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Maximum attenuation level in AGC algorithm"]
    #[inline(always)]
    pub fn att_ctrl_att_ctrl_max(&mut self) -> ATT_CTRL_ATT_CTRL_MAX_W {
        ATT_CTRL_ATT_CTRL_MAX_W { w: self }
    }
    #[doc = "Bits 24:27 - Attuenuation level if the AGC is bypassed"]
    #[inline(always)]
    pub fn att_ctrl_set_rx_att_ctrl(&mut self) -> ATT_CTRL_SET_RX_ATT_CTRL_W {
        ATT_CTRL_SET_RX_ATT_CTRL_W { w: self }
    }
    #[doc = "Bits 22:23 - Time constant of the decay speed; high values corresponds to a slow decay"]
    #[inline(always)]
    pub fn rssi_ctrl_agc_decay_tau(&mut self) -> RSSI_CTRL_AGC_DECAY_TAU_W {
        RSSI_CTRL_AGC_DECAY_TAU_W { w: self }
    }
    #[doc = "Bit 21 - If set to 1 the AGC algorithm uses the LNA bias."]
    #[inline(always)]
    pub fn rssi_ctrl_agc_use_lna(&mut self) -> RSSI_CTRL_AGC_USE_LNA_W {
        RSSI_CTRL_AGC_USE_LNA_W { w: self }
    }
    #[doc = "Bit 20 - Select the AGC algorithm: 0 -> old algorithm, 1 -> new algorithm"]
    #[inline(always)]
    pub fn rssi_ctrl_agc_mode(&mut self) -> RSSI_CTRL_AGC_MODE_W {
        RSSI_CTRL_AGC_MODE_W { w: self }
    }
    #[doc = "Bits 18:19 - Sets the wait time of the AGC after switching between states: 00 => don't wait, 01 => wait 1x RSSI filtering period, 10 => wait 2x RSSI filtering period, 11 => wait 3x RSSI filtering period"]
    #[inline(always)]
    pub fn rssi_ctrl_agc_wait(&mut self) -> RSSI_CTRL_AGC_WAIT_W {
        RSSI_CTRL_AGC_WAIT_W { w: self }
    }
    #[doc = "Bit 17 - If set to 1, the AGC is blocked during the payload"]
    #[inline(always)]
    pub fn rssi_ctrl_payload_blocks_agc(&mut self) -> RSSI_CTRL_PAYLOAD_BLOCKS_AGC_W {
        RSSI_CTRL_PAYLOAD_BLOCKS_AGC_W { w: self }
    }
    #[doc = "Bit 16 - If set to 1, the AGC algorithm is bypassed"]
    #[inline(always)]
    pub fn rssi_ctrl_bypass_agc(&mut self) -> RSSI_CTRL_BYPASS_AGC_W {
        RSSI_CTRL_BYPASS_AGC_W { w: self }
    }
    #[doc = "Bits 8:12 - Bias for the bandwidth of the channel filter"]
    #[inline(always)]
    pub fn filter_bias_iq_fi_bw(&mut self) -> FILTER_BIAS_IQ_FI_BW_W {
        FILTER_BIAS_IQ_FI_BW_W { w: self }
    }
    #[doc = "Bits 0:4 - Bias for the central frequency of the channel filter"]
    #[inline(always)]
    pub fn filter_bias_iq_fi_fc(&mut self) -> FILTER_BIAS_IQ_FI_FC_W {
        FILTER_BIAS_IQ_FI_FC_W { w: self }
    }
}
