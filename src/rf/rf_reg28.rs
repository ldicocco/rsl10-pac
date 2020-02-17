#[doc = "Reader of register RF_REG28"]
pub type R = crate::R<u32, super::RF_REG28>;
#[doc = "Writer for register RF_REG28"]
pub type W = crate::W<u32, super::RF_REG28>;
#[doc = "Register RF_REG28 `reset()`'s with value 0x02"]
impl crate::ResetValue for super::RF_REG28 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "If set to 1 switch the low-pass filter in the Rx chain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRL_RX_SWITCH_LP_A {
    #[doc = "0: `0`"]
    CTRL_RX_SWITCH_LP_DEFAULT = 0,
}
impl From<CTRL_RX_SWITCH_LP_A> for bool {
    #[inline(always)]
    fn from(variant: CTRL_RX_SWITCH_LP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTRL_RX_SWITCH_LP`"]
pub type CTRL_RX_SWITCH_LP_R = crate::R<bool, CTRL_RX_SWITCH_LP_A>;
impl CTRL_RX_SWITCH_LP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CTRL_RX_SWITCH_LP_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CTRL_RX_SWITCH_LP_A::CTRL_RX_SWITCH_LP_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CTRL_RX_SWITCH_LP_DEFAULT`"]
    #[inline(always)]
    pub fn is_ctrl_rx_switch_lp_default(&self) -> bool {
        *self == CTRL_RX_SWITCH_LP_A::CTRL_RX_SWITCH_LP_DEFAULT
    }
}
#[doc = "Write proxy for field `CTRL_RX_SWITCH_LP`"]
pub struct CTRL_RX_SWITCH_LP_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_RX_SWITCH_LP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_RX_SWITCH_LP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ctrl_rx_switch_lp_default(self) -> &'a mut W {
        self.variant(CTRL_RX_SWITCH_LP_A::CTRL_RX_SWITCH_LP_DEFAULT)
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
#[doc = "If set to 1, the peak detector is powered on during the Rx by the FSM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRL_RX_USE_PEAK_DETECTOR_A {
    #[doc = "0: `0`"]
    CTRL_RX_USE_PEAK_DETECTOR_DEFAULT = 0,
}
impl From<CTRL_RX_USE_PEAK_DETECTOR_A> for bool {
    #[inline(always)]
    fn from(variant: CTRL_RX_USE_PEAK_DETECTOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTRL_RX_USE_PEAK_DETECTOR`"]
pub type CTRL_RX_USE_PEAK_DETECTOR_R = crate::R<bool, CTRL_RX_USE_PEAK_DETECTOR_A>;
impl CTRL_RX_USE_PEAK_DETECTOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CTRL_RX_USE_PEAK_DETECTOR_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CTRL_RX_USE_PEAK_DETECTOR_A::CTRL_RX_USE_PEAK_DETECTOR_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CTRL_RX_USE_PEAK_DETECTOR_DEFAULT`"]
    #[inline(always)]
    pub fn is_ctrl_rx_use_peak_detector_default(&self) -> bool {
        *self == CTRL_RX_USE_PEAK_DETECTOR_A::CTRL_RX_USE_PEAK_DETECTOR_DEFAULT
    }
}
#[doc = "Write proxy for field `CTRL_RX_USE_PEAK_DETECTOR`"]
pub struct CTRL_RX_USE_PEAK_DETECTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_RX_USE_PEAK_DETECTOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_RX_USE_PEAK_DETECTOR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ctrl_rx_use_peak_detector_default(self) -> &'a mut W {
        self.variant(CTRL_RX_USE_PEAK_DETECTOR_A::CTRL_RX_USE_PEAK_DETECTOR_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "If set to 1, the mixer is enabled during the sub-band selection phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRL_RX_START_MIX_ON_CAL_A {
    #[doc = "0: `0`"]
    CTRL_RX_START_MIX_ON_CAL_DEFAULT = 0,
}
impl From<CTRL_RX_START_MIX_ON_CAL_A> for bool {
    #[inline(always)]
    fn from(variant: CTRL_RX_START_MIX_ON_CAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTRL_RX_START_MIX_ON_CAL`"]
pub type CTRL_RX_START_MIX_ON_CAL_R = crate::R<bool, CTRL_RX_START_MIX_ON_CAL_A>;
impl CTRL_RX_START_MIX_ON_CAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CTRL_RX_START_MIX_ON_CAL_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CTRL_RX_START_MIX_ON_CAL_A::CTRL_RX_START_MIX_ON_CAL_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CTRL_RX_START_MIX_ON_CAL_DEFAULT`"]
    #[inline(always)]
    pub fn is_ctrl_rx_start_mix_on_cal_default(&self) -> bool {
        *self == CTRL_RX_START_MIX_ON_CAL_A::CTRL_RX_START_MIX_ON_CAL_DEFAULT
    }
}
#[doc = "Write proxy for field `CTRL_RX_START_MIX_ON_CAL`"]
pub struct CTRL_RX_START_MIX_ON_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_RX_START_MIX_ON_CAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_RX_START_MIX_ON_CAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ctrl_rx_start_mix_on_cal_default(self) -> &'a mut W {
        self.variant(CTRL_RX_START_MIX_ON_CAL_A::CTRL_RX_START_MIX_ON_CAL_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "bits(1:0) => resonance 1 LNA, bits(3:2) => resonance 2 LNA, bit(4) => IFA PTAT-R only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTRL_RX_CTRL_RX_A {
    #[doc = "0: `0`"]
    CTRL_RX_CTRL_RX_DEFAULT = 0,
}
impl From<CTRL_RX_CTRL_RX_A> for u8 {
    #[inline(always)]
    fn from(variant: CTRL_RX_CTRL_RX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CTRL_RX_CTRL_RX`"]
pub type CTRL_RX_CTRL_RX_R = crate::R<u8, CTRL_RX_CTRL_RX_A>;
impl CTRL_RX_CTRL_RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CTRL_RX_CTRL_RX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CTRL_RX_CTRL_RX_A::CTRL_RX_CTRL_RX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CTRL_RX_CTRL_RX_DEFAULT`"]
    #[inline(always)]
    pub fn is_ctrl_rx_ctrl_rx_default(&self) -> bool {
        *self == CTRL_RX_CTRL_RX_A::CTRL_RX_CTRL_RX_DEFAULT
    }
}
#[doc = "Write proxy for field `CTRL_RX_CTRL_RX`"]
pub struct CTRL_RX_CTRL_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_RX_CTRL_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_RX_CTRL_RX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ctrl_rx_ctrl_rx_default(self) -> &'a mut W {
        self.variant(CTRL_RX_CTRL_RX_A::CTRL_RX_CTRL_RX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "VCO subband selection (Rx in FSM mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWCAP_FSM_SB_CAP_RX_A {
    #[doc = "0: `0`"]
    SWCAP_FSM_SB_CAP_RX_DEFAULT = 0,
}
impl From<SWCAP_FSM_SB_CAP_RX_A> for u8 {
    #[inline(always)]
    fn from(variant: SWCAP_FSM_SB_CAP_RX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SWCAP_FSM_SB_CAP_RX`"]
pub type SWCAP_FSM_SB_CAP_RX_R = crate::R<u8, SWCAP_FSM_SB_CAP_RX_A>;
impl SWCAP_FSM_SB_CAP_RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SWCAP_FSM_SB_CAP_RX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SWCAP_FSM_SB_CAP_RX_A::SWCAP_FSM_SB_CAP_RX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWCAP_FSM_SB_CAP_RX_DEFAULT`"]
    #[inline(always)]
    pub fn is_swcap_fsm_sb_cap_rx_default(&self) -> bool {
        *self == SWCAP_FSM_SB_CAP_RX_A::SWCAP_FSM_SB_CAP_RX_DEFAULT
    }
}
#[doc = "Write proxy for field `SWCAP_FSM_SB_CAP_RX`"]
pub struct SWCAP_FSM_SB_CAP_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> SWCAP_FSM_SB_CAP_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWCAP_FSM_SB_CAP_RX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn swcap_fsm_sb_cap_rx_default(self) -> &'a mut W {
        self.variant(SWCAP_FSM_SB_CAP_RX_A::SWCAP_FSM_SB_CAP_RX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "VCO subband selection (Tx in FSM mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWCAP_FSM_SB_CAP_TX_A {
    #[doc = "0: `0`"]
    SWCAP_FSM_SB_CAP_TX_DEFAULT = 0,
}
impl From<SWCAP_FSM_SB_CAP_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: SWCAP_FSM_SB_CAP_TX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SWCAP_FSM_SB_CAP_TX`"]
pub type SWCAP_FSM_SB_CAP_TX_R = crate::R<u8, SWCAP_FSM_SB_CAP_TX_A>;
impl SWCAP_FSM_SB_CAP_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SWCAP_FSM_SB_CAP_TX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SWCAP_FSM_SB_CAP_TX_A::SWCAP_FSM_SB_CAP_TX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWCAP_FSM_SB_CAP_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_swcap_fsm_sb_cap_tx_default(&self) -> bool {
        *self == SWCAP_FSM_SB_CAP_TX_A::SWCAP_FSM_SB_CAP_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `SWCAP_FSM_SB_CAP_TX`"]
pub struct SWCAP_FSM_SB_CAP_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> SWCAP_FSM_SB_CAP_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWCAP_FSM_SB_CAP_TX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn swcap_fsm_sb_cap_tx_default(self) -> &'a mut W {
        self.variant(SWCAP_FSM_SB_CAP_TX_A::SWCAP_FSM_SB_CAP_TX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLL_CTRL_CK_LAST_SEL_DELAY_A {
    #[doc = "0: `0`"]
    DLL_CTRL_CK_LAST_SEL_DELAY_DEFAULT = 0,
}
impl From<DLL_CTRL_CK_LAST_SEL_DELAY_A> for bool {
    #[inline(always)]
    fn from(variant: DLL_CTRL_CK_LAST_SEL_DELAY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLL_CTRL_CK_LAST_SEL_DELAY`"]
pub type DLL_CTRL_CK_LAST_SEL_DELAY_R = crate::R<bool, DLL_CTRL_CK_LAST_SEL_DELAY_A>;
impl DLL_CTRL_CK_LAST_SEL_DELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DLL_CTRL_CK_LAST_SEL_DELAY_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DLL_CTRL_CK_LAST_SEL_DELAY_A::DLL_CTRL_CK_LAST_SEL_DELAY_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DLL_CTRL_CK_LAST_SEL_DELAY_DEFAULT`"]
    #[inline(always)]
    pub fn is_dll_ctrl_ck_last_sel_delay_default(&self) -> bool {
        *self == DLL_CTRL_CK_LAST_SEL_DELAY_A::DLL_CTRL_CK_LAST_SEL_DELAY_DEFAULT
    }
}
#[doc = "Write proxy for field `DLL_CTRL_CK_LAST_SEL_DELAY`"]
pub struct DLL_CTRL_CK_LAST_SEL_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_CK_LAST_SEL_DELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLL_CTRL_CK_LAST_SEL_DELAY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dll_ctrl_ck_last_sel_delay_default(self) -> &'a mut W {
        self.variant(DLL_CTRL_CK_LAST_SEL_DELAY_A::DLL_CTRL_CK_LAST_SEL_DELAY_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLL_CTRL_CK_FIRST_SEL_DELAY_A {
    #[doc = "0: `0`"]
    DLL_CTRL_CK_FIRST_SEL_DELAY_DEFAULT = 0,
}
impl From<DLL_CTRL_CK_FIRST_SEL_DELAY_A> for bool {
    #[inline(always)]
    fn from(variant: DLL_CTRL_CK_FIRST_SEL_DELAY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLL_CTRL_CK_FIRST_SEL_DELAY`"]
pub type DLL_CTRL_CK_FIRST_SEL_DELAY_R = crate::R<bool, DLL_CTRL_CK_FIRST_SEL_DELAY_A>;
impl DLL_CTRL_CK_FIRST_SEL_DELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DLL_CTRL_CK_FIRST_SEL_DELAY_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DLL_CTRL_CK_FIRST_SEL_DELAY_A::DLL_CTRL_CK_FIRST_SEL_DELAY_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DLL_CTRL_CK_FIRST_SEL_DELAY_DEFAULT`"]
    #[inline(always)]
    pub fn is_dll_ctrl_ck_first_sel_delay_default(&self) -> bool {
        *self == DLL_CTRL_CK_FIRST_SEL_DELAY_A::DLL_CTRL_CK_FIRST_SEL_DELAY_DEFAULT
    }
}
#[doc = "Write proxy for field `DLL_CTRL_CK_FIRST_SEL_DELAY`"]
pub struct DLL_CTRL_CK_FIRST_SEL_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_CK_FIRST_SEL_DELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLL_CTRL_CK_FIRST_SEL_DELAY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dll_ctrl_ck_first_sel_delay_default(self) -> &'a mut W {
        self.variant(DLL_CTRL_CK_FIRST_SEL_DELAY_A::DLL_CTRL_CK_FIRST_SEL_DELAY_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Low: input clock comes from ck_xtal pin (default). High: input clock comes from ck_ext pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLL_CTRL_CK_EXT_SEL_A {
    #[doc = "0: `0`"]
    DLL_CTRL_CK_EXT_SEL_DEFAULT = 0,
}
impl From<DLL_CTRL_CK_EXT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DLL_CTRL_CK_EXT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLL_CTRL_CK_EXT_SEL`"]
pub type DLL_CTRL_CK_EXT_SEL_R = crate::R<bool, DLL_CTRL_CK_EXT_SEL_A>;
impl DLL_CTRL_CK_EXT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DLL_CTRL_CK_EXT_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DLL_CTRL_CK_EXT_SEL_A::DLL_CTRL_CK_EXT_SEL_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DLL_CTRL_CK_EXT_SEL_DEFAULT`"]
    #[inline(always)]
    pub fn is_dll_ctrl_ck_ext_sel_default(&self) -> bool {
        *self == DLL_CTRL_CK_EXT_SEL_A::DLL_CTRL_CK_EXT_SEL_DEFAULT
    }
}
#[doc = "Write proxy for field `DLL_CTRL_CK_EXT_SEL`"]
pub struct DLL_CTRL_CK_EXT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_CK_EXT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLL_CTRL_CK_EXT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dll_ctrl_ck_ext_sel_default(self) -> &'a mut W {
        self.variant(DLL_CTRL_CK_EXT_SEL_A::DLL_CTRL_CK_EXT_SEL_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Debug: enable to use the alternate ck_dig pin to output the PLL reference clock signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLL_CTRL_CK_DIG_EN_A {
    #[doc = "0: `0`"]
    DLL_CTRL_CK_DIG_EN_DEFAULT = 0,
}
impl From<DLL_CTRL_CK_DIG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DLL_CTRL_CK_DIG_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLL_CTRL_CK_DIG_EN`"]
pub type DLL_CTRL_CK_DIG_EN_R = crate::R<bool, DLL_CTRL_CK_DIG_EN_A>;
impl DLL_CTRL_CK_DIG_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DLL_CTRL_CK_DIG_EN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DLL_CTRL_CK_DIG_EN_A::DLL_CTRL_CK_DIG_EN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DLL_CTRL_CK_DIG_EN_DEFAULT`"]
    #[inline(always)]
    pub fn is_dll_ctrl_ck_dig_en_default(&self) -> bool {
        *self == DLL_CTRL_CK_DIG_EN_A::DLL_CTRL_CK_DIG_EN_DEFAULT
    }
}
#[doc = "Write proxy for field `DLL_CTRL_CK_DIG_EN`"]
pub struct DLL_CTRL_CK_DIG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_CK_DIG_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLL_CTRL_CK_DIG_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dll_ctrl_ck_dig_en_default(self) -> &'a mut W {
        self.variant(DLL_CTRL_CK_DIG_EN_A::DLL_CTRL_CK_DIG_EN_DEFAULT)
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
#[doc = "Debug: enable to output on GPIO the PLL reference clock signal via ck_test pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLL_CTRL_CK_TEST_EN_A {
    #[doc = "0: `0`"]
    DLL_CTRL_CK_TEST_EN_DEFAULT = 0,
}
impl From<DLL_CTRL_CK_TEST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DLL_CTRL_CK_TEST_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLL_CTRL_CK_TEST_EN`"]
pub type DLL_CTRL_CK_TEST_EN_R = crate::R<bool, DLL_CTRL_CK_TEST_EN_A>;
impl DLL_CTRL_CK_TEST_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DLL_CTRL_CK_TEST_EN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DLL_CTRL_CK_TEST_EN_A::DLL_CTRL_CK_TEST_EN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DLL_CTRL_CK_TEST_EN_DEFAULT`"]
    #[inline(always)]
    pub fn is_dll_ctrl_ck_test_en_default(&self) -> bool {
        *self == DLL_CTRL_CK_TEST_EN_A::DLL_CTRL_CK_TEST_EN_DEFAULT
    }
}
#[doc = "Write proxy for field `DLL_CTRL_CK_TEST_EN`"]
pub struct DLL_CTRL_CK_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_CK_TEST_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLL_CTRL_CK_TEST_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dll_ctrl_ck_test_en_default(self) -> &'a mut W {
        self.variant(DLL_CTRL_CK_TEST_EN_A::DLL_CTRL_CK_TEST_EN_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "When low, enable auxiliary wide lock range phase detector when fast mode locking is enabled (fast_enb = 0). When high, only the narrow lock range phase detector is enabled and bit 2 (fast_enb) must be high to avoid false frequency lock (slow mode locking)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLL_CTRL_TOO_FAST_ENB_A {
    #[doc = "0: `0`"]
    DLL_CTRL_TOO_FAST_ENB_DEFAULT = 0,
}
impl From<DLL_CTRL_TOO_FAST_ENB_A> for bool {
    #[inline(always)]
    fn from(variant: DLL_CTRL_TOO_FAST_ENB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLL_CTRL_TOO_FAST_ENB`"]
pub type DLL_CTRL_TOO_FAST_ENB_R = crate::R<bool, DLL_CTRL_TOO_FAST_ENB_A>;
impl DLL_CTRL_TOO_FAST_ENB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DLL_CTRL_TOO_FAST_ENB_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DLL_CTRL_TOO_FAST_ENB_A::DLL_CTRL_TOO_FAST_ENB_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DLL_CTRL_TOO_FAST_ENB_DEFAULT`"]
    #[inline(always)]
    pub fn is_dll_ctrl_too_fast_enb_default(&self) -> bool {
        *self == DLL_CTRL_TOO_FAST_ENB_A::DLL_CTRL_TOO_FAST_ENB_DEFAULT
    }
}
#[doc = "Write proxy for field `DLL_CTRL_TOO_FAST_ENB`"]
pub struct DLL_CTRL_TOO_FAST_ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_TOO_FAST_ENB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLL_CTRL_TOO_FAST_ENB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dll_ctrl_too_fast_enb_default(self) -> &'a mut W {
        self.variant(DLL_CTRL_TOO_FAST_ENB_A::DLL_CTRL_TOO_FAST_ENB_DEFAULT)
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
#[doc = "Enable reference frequency multiplier locked detector. When this signal is high, the dll_locked output goes high when the output multiplied clock is nearly about three times the frequency of the input clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLL_CTRL_LOCKED_DET_EN_A {
    #[doc = "0: `0`"]
    DLL_CTRL_LOCKED_DET_EN_DEFAULT = 0,
}
impl From<DLL_CTRL_LOCKED_DET_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DLL_CTRL_LOCKED_DET_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLL_CTRL_LOCKED_DET_EN`"]
pub type DLL_CTRL_LOCKED_DET_EN_R = crate::R<bool, DLL_CTRL_LOCKED_DET_EN_A>;
impl DLL_CTRL_LOCKED_DET_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DLL_CTRL_LOCKED_DET_EN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DLL_CTRL_LOCKED_DET_EN_A::DLL_CTRL_LOCKED_DET_EN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DLL_CTRL_LOCKED_DET_EN_DEFAULT`"]
    #[inline(always)]
    pub fn is_dll_ctrl_locked_det_en_default(&self) -> bool {
        *self == DLL_CTRL_LOCKED_DET_EN_A::DLL_CTRL_LOCKED_DET_EN_DEFAULT
    }
}
#[doc = "Write proxy for field `DLL_CTRL_LOCKED_DET_EN`"]
pub struct DLL_CTRL_LOCKED_DET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_LOCKED_DET_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLL_CTRL_LOCKED_DET_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dll_ctrl_locked_det_en_default(self) -> &'a mut W {
        self.variant(DLL_CTRL_LOCKED_DET_EN_A::DLL_CTRL_LOCKED_DET_EN_DEFAULT)
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
#[doc = "If for some reason the reference frequency multiplier is out of lock (usually because some input clocks from ck_xtal or ck_ext are missing) and this signal is high, the frequency multiplier will try to lock again automatically. Otherwise, a manual reset should be performed via dll_rstb input(see Table 3) to relock the frequency multiplier. This mode only works if bit 4 is also high (locked detector enabled, see below)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLL_CTRL_LOCKED_AUTO_CHECK_EN_A {
    #[doc = "0: `0`"]
    DLL_CTRL_LOCKED_AUTO_CHECK_EN_DEFAULT = 0,
}
impl From<DLL_CTRL_LOCKED_AUTO_CHECK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DLL_CTRL_LOCKED_AUTO_CHECK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLL_CTRL_LOCKED_AUTO_CHECK_EN`"]
pub type DLL_CTRL_LOCKED_AUTO_CHECK_EN_R = crate::R<bool, DLL_CTRL_LOCKED_AUTO_CHECK_EN_A>;
impl DLL_CTRL_LOCKED_AUTO_CHECK_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DLL_CTRL_LOCKED_AUTO_CHECK_EN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DLL_CTRL_LOCKED_AUTO_CHECK_EN_A::DLL_CTRL_LOCKED_AUTO_CHECK_EN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DLL_CTRL_LOCKED_AUTO_CHECK_EN_DEFAULT`"]
    #[inline(always)]
    pub fn is_dll_ctrl_locked_auto_check_en_default(&self) -> bool {
        *self == DLL_CTRL_LOCKED_AUTO_CHECK_EN_A::DLL_CTRL_LOCKED_AUTO_CHECK_EN_DEFAULT
    }
}
#[doc = "Write proxy for field `DLL_CTRL_LOCKED_AUTO_CHECK_EN`"]
pub struct DLL_CTRL_LOCKED_AUTO_CHECK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_LOCKED_AUTO_CHECK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLL_CTRL_LOCKED_AUTO_CHECK_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dll_ctrl_locked_auto_check_en_default(self) -> &'a mut W {
        self.variant(DLL_CTRL_LOCKED_AUTO_CHECK_EN_A::DLL_CTRL_LOCKED_AUTO_CHECK_EN_DEFAULT)
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
#[doc = "Enable, when low, fast mode locking of the reference frequency multiplier (default). Bit 5 must also be set low in this mode of operation (see below)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLL_CTRL_FAST_ENB_A {
    #[doc = "0: `0`"]
    DLL_CTRL_FAST_ENB_DEFAULT = 0,
}
impl From<DLL_CTRL_FAST_ENB_A> for bool {
    #[inline(always)]
    fn from(variant: DLL_CTRL_FAST_ENB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLL_CTRL_FAST_ENB`"]
pub type DLL_CTRL_FAST_ENB_R = crate::R<bool, DLL_CTRL_FAST_ENB_A>;
impl DLL_CTRL_FAST_ENB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DLL_CTRL_FAST_ENB_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DLL_CTRL_FAST_ENB_A::DLL_CTRL_FAST_ENB_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DLL_CTRL_FAST_ENB_DEFAULT`"]
    #[inline(always)]
    pub fn is_dll_ctrl_fast_enb_default(&self) -> bool {
        *self == DLL_CTRL_FAST_ENB_A::DLL_CTRL_FAST_ENB_DEFAULT
    }
}
#[doc = "Write proxy for field `DLL_CTRL_FAST_ENB`"]
pub struct DLL_CTRL_FAST_ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_FAST_ENB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLL_CTRL_FAST_ENB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dll_ctrl_fast_enb_default(self) -> &'a mut W {
        self.variant(DLL_CTRL_FAST_ENB_A::DLL_CTRL_FAST_ENB_DEFAULT)
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
#[doc = "Selection of the clock used as frequency reference of the PLL (also to ck_test and ck_dig outputs): 00 => ref = ck_xtal ot ck_ext (if bit 8 is high), 01 => ref = same as ck_sel = 00 if dll_en = 0, otherwise frequency(ref) = 3x frequency(ck_xtal) or 3x frequency(ck_ext) (if bit 8 is high), 10 => ref = same as ck_sel = 01 but output frequency divided by 2 (used in normal RX mode when dll_en = 0), 11 => ref = same as ck_sel = 01 but output frequency divided by 5 (used for RX mode with external signal at 132 MHz when dll_en = 0)\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DLL_CTRL_CK_SEL_A {
    #[doc = "2: `10`"]
    DLL_CTRL_CK_SEL_DEFAULT = 2,
}
impl From<DLL_CTRL_CK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DLL_CTRL_CK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DLL_CTRL_CK_SEL`"]
pub type DLL_CTRL_CK_SEL_R = crate::R<u8, DLL_CTRL_CK_SEL_A>;
impl DLL_CTRL_CK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DLL_CTRL_CK_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(DLL_CTRL_CK_SEL_A::DLL_CTRL_CK_SEL_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DLL_CTRL_CK_SEL_DEFAULT`"]
    #[inline(always)]
    pub fn is_dll_ctrl_ck_sel_default(&self) -> bool {
        *self == DLL_CTRL_CK_SEL_A::DLL_CTRL_CK_SEL_DEFAULT
    }
}
#[doc = "Write proxy for field `DLL_CTRL_CK_SEL`"]
pub struct DLL_CTRL_CK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_CK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLL_CTRL_CK_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn dll_ctrl_ck_sel_default(self) -> &'a mut W {
        self.variant(DLL_CTRL_CK_SEL_A::DLL_CTRL_CK_SEL_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - If set to 1 switch the low-pass filter in the Rx chain"]
    #[inline(always)]
    pub fn ctrl_rx_switch_lp(&self) -> CTRL_RX_SWITCH_LP_R {
        CTRL_RX_SWITCH_LP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - If set to 1, the peak detector is powered on during the Rx by the FSM"]
    #[inline(always)]
    pub fn ctrl_rx_use_peak_detector(&self) -> CTRL_RX_USE_PEAK_DETECTOR_R {
        CTRL_RX_USE_PEAK_DETECTOR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - If set to 1, the mixer is enabled during the sub-band selection phase"]
    #[inline(always)]
    pub fn ctrl_rx_start_mix_on_cal(&self) -> CTRL_RX_START_MIX_ON_CAL_R {
        CTRL_RX_START_MIX_ON_CAL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - bits(1:0) => resonance 1 LNA, bits(3:2) => resonance 2 LNA, bit(4) => IFA PTAT-R only"]
    #[inline(always)]
    pub fn ctrl_rx_ctrl_rx(&self) -> CTRL_RX_CTRL_RX_R {
        CTRL_RX_CTRL_RX_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 20:23 - VCO subband selection (Rx in FSM mode)"]
    #[inline(always)]
    pub fn swcap_fsm_sb_cap_rx(&self) -> SWCAP_FSM_SB_CAP_RX_R {
        SWCAP_FSM_SB_CAP_RX_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - VCO subband selection (Tx in FSM mode)"]
    #[inline(always)]
    pub fn swcap_fsm_sb_cap_tx(&self) -> SWCAP_FSM_SB_CAP_TX_R {
        SWCAP_FSM_SB_CAP_TX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dll_ctrl_ck_last_sel_delay(&self) -> DLL_CTRL_CK_LAST_SEL_DELAY_R {
        DLL_CTRL_CK_LAST_SEL_DELAY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dll_ctrl_ck_first_sel_delay(&self) -> DLL_CTRL_CK_FIRST_SEL_DELAY_R {
        DLL_CTRL_CK_FIRST_SEL_DELAY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Low: input clock comes from ck_xtal pin (default). High: input clock comes from ck_ext pin"]
    #[inline(always)]
    pub fn dll_ctrl_ck_ext_sel(&self) -> DLL_CTRL_CK_EXT_SEL_R {
        DLL_CTRL_CK_EXT_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Debug: enable to use the alternate ck_dig pin to output the PLL reference clock signal"]
    #[inline(always)]
    pub fn dll_ctrl_ck_dig_en(&self) -> DLL_CTRL_CK_DIG_EN_R {
        DLL_CTRL_CK_DIG_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Debug: enable to output on GPIO the PLL reference clock signal via ck_test pin"]
    #[inline(always)]
    pub fn dll_ctrl_ck_test_en(&self) -> DLL_CTRL_CK_TEST_EN_R {
        DLL_CTRL_CK_TEST_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When low, enable auxiliary wide lock range phase detector when fast mode locking is enabled (fast_enb = 0). When high, only the narrow lock range phase detector is enabled and bit 2 (fast_enb) must be high to avoid false frequency lock (slow mode locking)"]
    #[inline(always)]
    pub fn dll_ctrl_too_fast_enb(&self) -> DLL_CTRL_TOO_FAST_ENB_R {
        DLL_CTRL_TOO_FAST_ENB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable reference frequency multiplier locked detector. When this signal is high, the dll_locked output goes high when the output multiplied clock is nearly about three times the frequency of the input clock."]
    #[inline(always)]
    pub fn dll_ctrl_locked_det_en(&self) -> DLL_CTRL_LOCKED_DET_EN_R {
        DLL_CTRL_LOCKED_DET_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If for some reason the reference frequency multiplier is out of lock (usually because some input clocks from ck_xtal or ck_ext are missing) and this signal is high, the frequency multiplier will try to lock again automatically. Otherwise, a manual reset should be performed via dll_rstb input(see Table 3) to relock the frequency multiplier. This mode only works if bit 4 is also high (locked detector enabled, see below)"]
    #[inline(always)]
    pub fn dll_ctrl_locked_auto_check_en(&self) -> DLL_CTRL_LOCKED_AUTO_CHECK_EN_R {
        DLL_CTRL_LOCKED_AUTO_CHECK_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable, when low, fast mode locking of the reference frequency multiplier (default). Bit 5 must also be set low in this mode of operation (see below)"]
    #[inline(always)]
    pub fn dll_ctrl_fast_enb(&self) -> DLL_CTRL_FAST_ENB_R {
        DLL_CTRL_FAST_ENB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - Selection of the clock used as frequency reference of the PLL (also to ck_test and ck_dig outputs): 00 => ref = ck_xtal ot ck_ext (if bit 8 is high), 01 => ref = same as ck_sel = 00 if dll_en = 0, otherwise frequency(ref) = 3x frequency(ck_xtal) or 3x frequency(ck_ext) (if bit 8 is high), 10 => ref = same as ck_sel = 01 but output frequency divided by 2 (used in normal RX mode when dll_en = 0), 11 => ref = same as ck_sel = 01 but output frequency divided by 5 (used for RX mode with external signal at 132 MHz when dll_en = 0)"]
    #[inline(always)]
    pub fn dll_ctrl_ck_sel(&self) -> DLL_CTRL_CK_SEL_R {
        DLL_CTRL_CK_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - If set to 1 switch the low-pass filter in the Rx chain"]
    #[inline(always)]
    pub fn ctrl_rx_switch_lp(&mut self) -> CTRL_RX_SWITCH_LP_W {
        CTRL_RX_SWITCH_LP_W { w: self }
    }
    #[doc = "Bit 30 - If set to 1, the peak detector is powered on during the Rx by the FSM"]
    #[inline(always)]
    pub fn ctrl_rx_use_peak_detector(&mut self) -> CTRL_RX_USE_PEAK_DETECTOR_W {
        CTRL_RX_USE_PEAK_DETECTOR_W { w: self }
    }
    #[doc = "Bit 29 - If set to 1, the mixer is enabled during the sub-band selection phase"]
    #[inline(always)]
    pub fn ctrl_rx_start_mix_on_cal(&mut self) -> CTRL_RX_START_MIX_ON_CAL_W {
        CTRL_RX_START_MIX_ON_CAL_W { w: self }
    }
    #[doc = "Bits 24:28 - bits(1:0) => resonance 1 LNA, bits(3:2) => resonance 2 LNA, bit(4) => IFA PTAT-R only"]
    #[inline(always)]
    pub fn ctrl_rx_ctrl_rx(&mut self) -> CTRL_RX_CTRL_RX_W {
        CTRL_RX_CTRL_RX_W { w: self }
    }
    #[doc = "Bits 20:23 - VCO subband selection (Rx in FSM mode)"]
    #[inline(always)]
    pub fn swcap_fsm_sb_cap_rx(&mut self) -> SWCAP_FSM_SB_CAP_RX_W {
        SWCAP_FSM_SB_CAP_RX_W { w: self }
    }
    #[doc = "Bits 16:19 - VCO subband selection (Tx in FSM mode)"]
    #[inline(always)]
    pub fn swcap_fsm_sb_cap_tx(&mut self) -> SWCAP_FSM_SB_CAP_TX_W {
        SWCAP_FSM_SB_CAP_TX_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dll_ctrl_ck_last_sel_delay(&mut self) -> DLL_CTRL_CK_LAST_SEL_DELAY_W {
        DLL_CTRL_CK_LAST_SEL_DELAY_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dll_ctrl_ck_first_sel_delay(&mut self) -> DLL_CTRL_CK_FIRST_SEL_DELAY_W {
        DLL_CTRL_CK_FIRST_SEL_DELAY_W { w: self }
    }
    #[doc = "Bit 8 - Low: input clock comes from ck_xtal pin (default). High: input clock comes from ck_ext pin"]
    #[inline(always)]
    pub fn dll_ctrl_ck_ext_sel(&mut self) -> DLL_CTRL_CK_EXT_SEL_W {
        DLL_CTRL_CK_EXT_SEL_W { w: self }
    }
    #[doc = "Bit 7 - Debug: enable to use the alternate ck_dig pin to output the PLL reference clock signal"]
    #[inline(always)]
    pub fn dll_ctrl_ck_dig_en(&mut self) -> DLL_CTRL_CK_DIG_EN_W {
        DLL_CTRL_CK_DIG_EN_W { w: self }
    }
    #[doc = "Bit 6 - Debug: enable to output on GPIO the PLL reference clock signal via ck_test pin"]
    #[inline(always)]
    pub fn dll_ctrl_ck_test_en(&mut self) -> DLL_CTRL_CK_TEST_EN_W {
        DLL_CTRL_CK_TEST_EN_W { w: self }
    }
    #[doc = "Bit 5 - When low, enable auxiliary wide lock range phase detector when fast mode locking is enabled (fast_enb = 0). When high, only the narrow lock range phase detector is enabled and bit 2 (fast_enb) must be high to avoid false frequency lock (slow mode locking)"]
    #[inline(always)]
    pub fn dll_ctrl_too_fast_enb(&mut self) -> DLL_CTRL_TOO_FAST_ENB_W {
        DLL_CTRL_TOO_FAST_ENB_W { w: self }
    }
    #[doc = "Bit 4 - Enable reference frequency multiplier locked detector. When this signal is high, the dll_locked output goes high when the output multiplied clock is nearly about three times the frequency of the input clock."]
    #[inline(always)]
    pub fn dll_ctrl_locked_det_en(&mut self) -> DLL_CTRL_LOCKED_DET_EN_W {
        DLL_CTRL_LOCKED_DET_EN_W { w: self }
    }
    #[doc = "Bit 3 - If for some reason the reference frequency multiplier is out of lock (usually because some input clocks from ck_xtal or ck_ext are missing) and this signal is high, the frequency multiplier will try to lock again automatically. Otherwise, a manual reset should be performed via dll_rstb input(see Table 3) to relock the frequency multiplier. This mode only works if bit 4 is also high (locked detector enabled, see below)"]
    #[inline(always)]
    pub fn dll_ctrl_locked_auto_check_en(&mut self) -> DLL_CTRL_LOCKED_AUTO_CHECK_EN_W {
        DLL_CTRL_LOCKED_AUTO_CHECK_EN_W { w: self }
    }
    #[doc = "Bit 2 - Enable, when low, fast mode locking of the reference frequency multiplier (default). Bit 5 must also be set low in this mode of operation (see below)"]
    #[inline(always)]
    pub fn dll_ctrl_fast_enb(&mut self) -> DLL_CTRL_FAST_ENB_W {
        DLL_CTRL_FAST_ENB_W { w: self }
    }
    #[doc = "Bits 0:1 - Selection of the clock used as frequency reference of the PLL (also to ck_test and ck_dig outputs): 00 => ref = ck_xtal ot ck_ext (if bit 8 is high), 01 => ref = same as ck_sel = 00 if dll_en = 0, otherwise frequency(ref) = 3x frequency(ck_xtal) or 3x frequency(ck_ext) (if bit 8 is high), 10 => ref = same as ck_sel = 01 but output frequency divided by 2 (used in normal RX mode when dll_en = 0), 11 => ref = same as ck_sel = 01 but output frequency divided by 5 (used for RX mode with external signal at 132 MHz when dll_en = 0)"]
    #[inline(always)]
    pub fn dll_ctrl_ck_sel(&mut self) -> DLL_CTRL_CK_SEL_W {
        DLL_CTRL_CK_SEL_W { w: self }
    }
}
