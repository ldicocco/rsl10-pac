#[doc = "Reader of register RF_PLL_CTRL"]
pub type R = crate::R<u32, super::RF_PLL_CTRL>;
#[doc = "Writer for register RF_PLL_CTRL"]
pub type W = crate::W<u32, super::RF_PLL_CTRL>;
#[doc = "Register RF_PLL_CTRL `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RF_PLL_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "trimming of the xtal: 5MSB thermometric, 3LSB direct\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XTAL_TRIM_XTAL_TRIM_A {
    #[doc = "128: `10000000`"]
    XTAL_TRIM_XTAL_TRIM_DEFAULT = 128,
}
impl From<XTAL_TRIM_XTAL_TRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: XTAL_TRIM_XTAL_TRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XTAL_TRIM_XTAL_TRIM`"]
pub type XTAL_TRIM_XTAL_TRIM_R = crate::R<u8, XTAL_TRIM_XTAL_TRIM_A>;
impl XTAL_TRIM_XTAL_TRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, XTAL_TRIM_XTAL_TRIM_A> {
        use crate::Variant::*;
        match self.bits {
            128 => Val(XTAL_TRIM_XTAL_TRIM_A::XTAL_TRIM_XTAL_TRIM_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_TRIM_XTAL_TRIM_DEFAULT`"]
    #[inline(always)]
    pub fn is_xtal_trim_xtal_trim_default(&self) -> bool {
        *self == XTAL_TRIM_XTAL_TRIM_A::XTAL_TRIM_XTAL_TRIM_DEFAULT
    }
}
#[doc = "Write proxy for field `XTAL_TRIM_XTAL_TRIM`"]
pub struct XTAL_TRIM_XTAL_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_TRIM_XTAL_TRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_TRIM_XTAL_TRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`10000000`"]
    #[inline(always)]
    pub fn xtal_trim_xtal_trim_default(self) -> &'a mut W {
        self.variant(XTAL_TRIM_XTAL_TRIM_A::XTAL_TRIM_XTAL_TRIM_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "If set to 1 the PLL is set to 48MHz in Rx instead of 24MHz (need also to change ck_sel)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CTRL_2_PLL_RX_48MEG_A {
    #[doc = "0: `0`"]
    PLL_CTRL_2_PLL_RX_48MEG_DEFAULT = 0,
}
impl From<PLL_CTRL_2_PLL_RX_48MEG_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CTRL_2_PLL_RX_48MEG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_CTRL_2_PLL_RX_48MEG`"]
pub type PLL_CTRL_2_PLL_RX_48MEG_R = crate::R<bool, PLL_CTRL_2_PLL_RX_48MEG_A>;
impl PLL_CTRL_2_PLL_RX_48MEG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PLL_CTRL_2_PLL_RX_48MEG_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PLL_CTRL_2_PLL_RX_48MEG_A::PLL_CTRL_2_PLL_RX_48MEG_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CTRL_2_PLL_RX_48MEG_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_ctrl_2_pll_rx_48meg_default(&self) -> bool {
        *self == PLL_CTRL_2_PLL_RX_48MEG_A::PLL_CTRL_2_PLL_RX_48MEG_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CTRL_2_PLL_RX_48MEG`"]
pub struct PLL_CTRL_2_PLL_RX_48MEG_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CTRL_2_PLL_RX_48MEG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CTRL_2_PLL_RX_48MEG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ctrl_2_pll_rx_48meg_default(self) -> &'a mut W {
        self.variant(PLL_CTRL_2_PLL_RX_48MEG_A::PLL_CTRL_2_PLL_RX_48MEG_DEFAULT)
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
#[doc = "If set to 1, in case of swcap_fsm=1, the register for Rx and Tx swcap is the same\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CTRL_2_SWCAP_TX_SAME_RX_A {
    #[doc = "0: `0`"]
    PLL_CTRL_2_SWCAP_TX_SAME_RX_DEFAULT = 0,
}
impl From<PLL_CTRL_2_SWCAP_TX_SAME_RX_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CTRL_2_SWCAP_TX_SAME_RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_CTRL_2_SWCAP_TX_SAME_RX`"]
pub type PLL_CTRL_2_SWCAP_TX_SAME_RX_R = crate::R<bool, PLL_CTRL_2_SWCAP_TX_SAME_RX_A>;
impl PLL_CTRL_2_SWCAP_TX_SAME_RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PLL_CTRL_2_SWCAP_TX_SAME_RX_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PLL_CTRL_2_SWCAP_TX_SAME_RX_A::PLL_CTRL_2_SWCAP_TX_SAME_RX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CTRL_2_SWCAP_TX_SAME_RX_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_ctrl_2_swcap_tx_same_rx_default(&self) -> bool {
        *self == PLL_CTRL_2_SWCAP_TX_SAME_RX_A::PLL_CTRL_2_SWCAP_TX_SAME_RX_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CTRL_2_SWCAP_TX_SAME_RX`"]
pub struct PLL_CTRL_2_SWCAP_TX_SAME_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CTRL_2_SWCAP_TX_SAME_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CTRL_2_SWCAP_TX_SAME_RX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ctrl_2_swcap_tx_same_rx_default(self) -> &'a mut W {
        self.variant(PLL_CTRL_2_SWCAP_TX_SAME_RX_A::PLL_CTRL_2_SWCAP_TX_SAME_RX_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "If set to 1 use the swcap_fsm register as reference for the sub-band selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CTRL_2_SWCAP_FSM_A {
    #[doc = "0: `0`"]
    PLL_CTRL_2_SWCAP_FSM_DEFAULT = 0,
}
impl From<PLL_CTRL_2_SWCAP_FSM_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CTRL_2_SWCAP_FSM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_CTRL_2_SWCAP_FSM`"]
pub type PLL_CTRL_2_SWCAP_FSM_R = crate::R<bool, PLL_CTRL_2_SWCAP_FSM_A>;
impl PLL_CTRL_2_SWCAP_FSM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PLL_CTRL_2_SWCAP_FSM_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PLL_CTRL_2_SWCAP_FSM_A::PLL_CTRL_2_SWCAP_FSM_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CTRL_2_SWCAP_FSM_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_ctrl_2_swcap_fsm_default(&self) -> bool {
        *self == PLL_CTRL_2_SWCAP_FSM_A::PLL_CTRL_2_SWCAP_FSM_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CTRL_2_SWCAP_FSM`"]
pub struct PLL_CTRL_2_SWCAP_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CTRL_2_SWCAP_FSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CTRL_2_SWCAP_FSM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ctrl_2_swcap_fsm_default(self) -> &'a mut W {
        self.variant(PLL_CTRL_2_SWCAP_FSM_A::PLL_CTRL_2_SWCAP_FSM_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reset signal of the DLL (active low)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CTRL_2_DLL_RSTB_A {
    #[doc = "0: `0`"]
    PLL_CTRL_2_DLL_RSTB_DEFAULT = 0,
}
impl From<PLL_CTRL_2_DLL_RSTB_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CTRL_2_DLL_RSTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_CTRL_2_DLL_RSTB`"]
pub type PLL_CTRL_2_DLL_RSTB_R = crate::R<bool, PLL_CTRL_2_DLL_RSTB_A>;
impl PLL_CTRL_2_DLL_RSTB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PLL_CTRL_2_DLL_RSTB_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PLL_CTRL_2_DLL_RSTB_A::PLL_CTRL_2_DLL_RSTB_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CTRL_2_DLL_RSTB_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_ctrl_2_dll_rstb_default(&self) -> bool {
        *self == PLL_CTRL_2_DLL_RSTB_A::PLL_CTRL_2_DLL_RSTB_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CTRL_2_DLL_RSTB`"]
pub struct PLL_CTRL_2_DLL_RSTB_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CTRL_2_DLL_RSTB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CTRL_2_DLL_RSTB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ctrl_2_dll_rstb_default(self) -> &'a mut W {
        self.variant(PLL_CTRL_2_DLL_RSTB_A::PLL_CTRL_2_DLL_RSTB_DEFAULT)
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
#[doc = "VCO sub-band selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_A {
    #[doc = "0: `0`"]
    PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_DEFAULT = 0,
}
impl From<PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_CTRL_2_VCO_SUBBAND_TRIM_HI`"]
pub type PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_R = crate::R<bool, PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_A>;
impl PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_A::PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_ctrl_2_vco_subband_trim_hi_default(&self) -> bool {
        *self == PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_A::PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CTRL_2_VCO_SUBBAND_TRIM_HI`"]
pub struct PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ctrl_2_vco_subband_trim_hi_default(self) -> &'a mut W {
        self.variant(PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_A::PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_DEFAULT)
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
#[doc = "VCO sub-band selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_A {
    #[doc = "0: `0`"]
    PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_DEFAULT = 0,
}
impl From<PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLL_CTRL_1_VCO_SUBBAND_TRIM_LO`"]
pub type PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_R = crate::R<u8, PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_A>;
impl PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_A::PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_ctrl_1_vco_subband_trim_lo_default(&self) -> bool {
        *self == PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_A::PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CTRL_1_VCO_SUBBAND_TRIM_LO`"]
pub struct PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ctrl_1_vco_subband_trim_lo_default(self) -> &'a mut W {
        self.variant(PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_A::PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Add offset to sub-band selection comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CTRL_1_SUB_SEL_OFFS_EN_A {
    #[doc = "0: `0`"]
    PLL_CTRL_1_SUB_SEL_OFFS_EN_DEFAULT = 0,
}
impl From<PLL_CTRL_1_SUB_SEL_OFFS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CTRL_1_SUB_SEL_OFFS_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_CTRL_1_SUB_SEL_OFFS_EN`"]
pub type PLL_CTRL_1_SUB_SEL_OFFS_EN_R = crate::R<bool, PLL_CTRL_1_SUB_SEL_OFFS_EN_A>;
impl PLL_CTRL_1_SUB_SEL_OFFS_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PLL_CTRL_1_SUB_SEL_OFFS_EN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PLL_CTRL_1_SUB_SEL_OFFS_EN_A::PLL_CTRL_1_SUB_SEL_OFFS_EN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CTRL_1_SUB_SEL_OFFS_EN_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_ctrl_1_sub_sel_offs_en_default(&self) -> bool {
        *self == PLL_CTRL_1_SUB_SEL_OFFS_EN_A::PLL_CTRL_1_SUB_SEL_OFFS_EN_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CTRL_1_SUB_SEL_OFFS_EN`"]
pub struct PLL_CTRL_1_SUB_SEL_OFFS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CTRL_1_SUB_SEL_OFFS_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CTRL_1_SUB_SEL_OFFS_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ctrl_1_sub_sel_offs_en_default(self) -> &'a mut W {
        self.variant(PLL_CTRL_1_SUB_SEL_OFFS_EN_A::PLL_CTRL_1_SUB_SEL_OFFS_EN_DEFAULT)
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
#[doc = "Debug: VCO signal divided by the programmable divider is divided by a: 0 => division ratio set to 1, 1 => division ratio set to 2; before to be outputted to ck_div_test\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_A {
    #[doc = "0: `0`"]
    PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_DEFAULT = 0,
}
impl From<PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_CTRL_1_DIV2_CLKVCO_TEST_EN`"]
pub type PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_R = crate::R<bool, PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_A>;
impl PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_A::PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_ctrl_1_div2_clkvco_test_en_default(&self) -> bool {
        *self == PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_A::PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CTRL_1_DIV2_CLKVCO_TEST_EN`"]
pub struct PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ctrl_1_div2_clkvco_test_en_default(self) -> &'a mut W {
        self.variant(PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_A::PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_DEFAULT)
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
#[doc = "Debug: enable to output on GPIO the VCO signal divided by the programmable divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CTRL_1_VCODIV_CLK_TEST_EN_A {
    #[doc = "0: `0`"]
    PLL_CTRL_1_VCODIV_CLK_TEST_EN_DEFAULT = 0,
}
impl From<PLL_CTRL_1_VCODIV_CLK_TEST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CTRL_1_VCODIV_CLK_TEST_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_CTRL_1_VCODIV_CLK_TEST_EN`"]
pub type PLL_CTRL_1_VCODIV_CLK_TEST_EN_R = crate::R<bool, PLL_CTRL_1_VCODIV_CLK_TEST_EN_A>;
impl PLL_CTRL_1_VCODIV_CLK_TEST_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PLL_CTRL_1_VCODIV_CLK_TEST_EN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PLL_CTRL_1_VCODIV_CLK_TEST_EN_A::PLL_CTRL_1_VCODIV_CLK_TEST_EN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CTRL_1_VCODIV_CLK_TEST_EN_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_ctrl_1_vcodiv_clk_test_en_default(&self) -> bool {
        *self == PLL_CTRL_1_VCODIV_CLK_TEST_EN_A::PLL_CTRL_1_VCODIV_CLK_TEST_EN_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CTRL_1_VCODIV_CLK_TEST_EN`"]
pub struct PLL_CTRL_1_VCODIV_CLK_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CTRL_1_VCODIV_CLK_TEST_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CTRL_1_VCODIV_CLK_TEST_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ctrl_1_vcodiv_clk_test_en_default(self) -> &'a mut W {
        self.variant(PLL_CTRL_1_VCODIV_CLK_TEST_EN_A::PLL_CTRL_1_VCODIV_CLK_TEST_EN_DEFAULT)
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
#[doc = "When high, allow to decrease half time the bias current for the same output pumping current. Should be always high in IcyTRX.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CTRL_1_EN_LOW_CHP_BIAS_A {
    #[doc = "0: `0`"]
    PLL_CTRL_1_EN_LOW_CHP_BIAS_DEFAULT = 0,
}
impl From<PLL_CTRL_1_EN_LOW_CHP_BIAS_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CTRL_1_EN_LOW_CHP_BIAS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_CTRL_1_EN_LOW_CHP_BIAS`"]
pub type PLL_CTRL_1_EN_LOW_CHP_BIAS_R = crate::R<bool, PLL_CTRL_1_EN_LOW_CHP_BIAS_A>;
impl PLL_CTRL_1_EN_LOW_CHP_BIAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PLL_CTRL_1_EN_LOW_CHP_BIAS_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PLL_CTRL_1_EN_LOW_CHP_BIAS_A::PLL_CTRL_1_EN_LOW_CHP_BIAS_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CTRL_1_EN_LOW_CHP_BIAS_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_ctrl_1_en_low_chp_bias_default(&self) -> bool {
        *self == PLL_CTRL_1_EN_LOW_CHP_BIAS_A::PLL_CTRL_1_EN_LOW_CHP_BIAS_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CTRL_1_EN_LOW_CHP_BIAS`"]
pub struct PLL_CTRL_1_EN_LOW_CHP_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CTRL_1_EN_LOW_CHP_BIAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CTRL_1_EN_LOW_CHP_BIAS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ctrl_1_en_low_chp_bias_default(self) -> &'a mut W {
        self.variant(PLL_CTRL_1_EN_LOW_CHP_BIAS_A::PLL_CTRL_1_EN_LOW_CHP_BIAS_DEFAULT)
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
#[doc = "Debug: enable charge-pump dead zone (degraded PLL characteristics for test)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CTRL_1_CHP_DEAD_ZONE_EN_A {
    #[doc = "0: `0`"]
    PLL_CTRL_1_CHP_DEAD_ZONE_EN_DEFAULT = 0,
}
impl From<PLL_CTRL_1_CHP_DEAD_ZONE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CTRL_1_CHP_DEAD_ZONE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_CTRL_1_CHP_DEAD_ZONE_EN`"]
pub type PLL_CTRL_1_CHP_DEAD_ZONE_EN_R = crate::R<bool, PLL_CTRL_1_CHP_DEAD_ZONE_EN_A>;
impl PLL_CTRL_1_CHP_DEAD_ZONE_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PLL_CTRL_1_CHP_DEAD_ZONE_EN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PLL_CTRL_1_CHP_DEAD_ZONE_EN_A::PLL_CTRL_1_CHP_DEAD_ZONE_EN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CTRL_1_CHP_DEAD_ZONE_EN_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_ctrl_1_chp_dead_zone_en_default(&self) -> bool {
        *self == PLL_CTRL_1_CHP_DEAD_ZONE_EN_A::PLL_CTRL_1_CHP_DEAD_ZONE_EN_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CTRL_1_CHP_DEAD_ZONE_EN`"]
pub struct PLL_CTRL_1_CHP_DEAD_ZONE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CTRL_1_CHP_DEAD_ZONE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CTRL_1_CHP_DEAD_ZONE_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ctrl_1_chp_dead_zone_en_default(self) -> &'a mut W {
        self.variant(PLL_CTRL_1_CHP_DEAD_ZONE_EN_A::PLL_CTRL_1_CHP_DEAD_ZONE_EN_DEFAULT)
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
#[doc = "Debug: charge-pump offset current values selection bits (see bit 6 to enable this mode): 00 => d_phi = 15, 01 => d_phi=22.5, 10 => d_phi = 30, 11 => d_phi = 60. Also sets the bias current of the common mode control block of the charge-pump. Must be sets to 01 to ensure a proper operation of the VCO tuning voltage comparator for sub-band selection, if used\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_A {
    #[doc = "0: `0`"]
    PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_DEFAULT = 0,
}
impl From<PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLL_CTRL_1_CHP_CURR_OFFSET_TRIM`"]
pub type PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_R = crate::R<u8, PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_A>;
impl PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_A::PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_ctrl_1_chp_curr_offset_trim_default(&self) -> bool {
        *self == PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_A::PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CTRL_1_CHP_CURR_OFFSET_TRIM`"]
pub struct PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ctrl_1_chp_curr_offset_trim_default(self) -> &'a mut W {
        self.variant(PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_A::PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Enable the PLL filter high bandwidth needed in TX (must be high together with bit 4 in TX, low in RX)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CTRL_1_HIGH_BW_FILTER_EN_A {
    #[doc = "0: `0`"]
    PLL_CTRL_1_HIGH_BW_FILTER_EN_DEFAULT = 0,
}
impl From<PLL_CTRL_1_HIGH_BW_FILTER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CTRL_1_HIGH_BW_FILTER_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_CTRL_1_HIGH_BW_FILTER_EN`"]
pub type PLL_CTRL_1_HIGH_BW_FILTER_EN_R = crate::R<bool, PLL_CTRL_1_HIGH_BW_FILTER_EN_A>;
impl PLL_CTRL_1_HIGH_BW_FILTER_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PLL_CTRL_1_HIGH_BW_FILTER_EN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PLL_CTRL_1_HIGH_BW_FILTER_EN_A::PLL_CTRL_1_HIGH_BW_FILTER_EN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CTRL_1_HIGH_BW_FILTER_EN_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_ctrl_1_high_bw_filter_en_default(&self) -> bool {
        *self == PLL_CTRL_1_HIGH_BW_FILTER_EN_A::PLL_CTRL_1_HIGH_BW_FILTER_EN_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CTRL_1_HIGH_BW_FILTER_EN`"]
pub struct PLL_CTRL_1_HIGH_BW_FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CTRL_1_HIGH_BW_FILTER_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CTRL_1_HIGH_BW_FILTER_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ctrl_1_high_bw_filter_en_default(self) -> &'a mut W {
        self.variant(PLL_CTRL_1_HIGH_BW_FILTER_EN_A::PLL_CTRL_1_HIGH_BW_FILTER_EN_DEFAULT)
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
#[doc = "Enable the high current output of the charge-pump for PLL TX high bandwidth mode (must be high together with bit 5 in TX, low in RX)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CTRL_1_FAST_CHP_EN_A {
    #[doc = "0: `0`"]
    PLL_CTRL_1_FAST_CHP_EN_DEFAULT = 0,
}
impl From<PLL_CTRL_1_FAST_CHP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CTRL_1_FAST_CHP_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_CTRL_1_FAST_CHP_EN`"]
pub type PLL_CTRL_1_FAST_CHP_EN_R = crate::R<bool, PLL_CTRL_1_FAST_CHP_EN_A>;
impl PLL_CTRL_1_FAST_CHP_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PLL_CTRL_1_FAST_CHP_EN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PLL_CTRL_1_FAST_CHP_EN_A::PLL_CTRL_1_FAST_CHP_EN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CTRL_1_FAST_CHP_EN_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_ctrl_1_fast_chp_en_default(&self) -> bool {
        *self == PLL_CTRL_1_FAST_CHP_EN_A::PLL_CTRL_1_FAST_CHP_EN_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CTRL_1_FAST_CHP_EN`"]
pub struct PLL_CTRL_1_FAST_CHP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CTRL_1_FAST_CHP_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CTRL_1_FAST_CHP_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ctrl_1_fast_chp_en_default(self) -> &'a mut W {
        self.variant(PLL_CTRL_1_FAST_CHP_EN_A::PLL_CTRL_1_FAST_CHP_EN_DEFAULT)
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
#[doc = "Charge-pump active if 00 else this allow to open the PLL and force the VCO tune voltage to reach: 01 => minimum frequency inside sub-band selection, 10 => medium frequency inside sub-band selection, 11 => maximum frequency inside sub-band selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLL_CTRL_1_CHP_MODE_TRIM_A {
    #[doc = "0: `0`"]
    PLL_CTRL_1_CHP_MODE_TRIM_DEFAULT = 0,
}
impl From<PLL_CTRL_1_CHP_MODE_TRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL_CTRL_1_CHP_MODE_TRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLL_CTRL_1_CHP_MODE_TRIM`"]
pub type PLL_CTRL_1_CHP_MODE_TRIM_R = crate::R<u8, PLL_CTRL_1_CHP_MODE_TRIM_A>;
impl PLL_CTRL_1_CHP_MODE_TRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLL_CTRL_1_CHP_MODE_TRIM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PLL_CTRL_1_CHP_MODE_TRIM_A::PLL_CTRL_1_CHP_MODE_TRIM_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CTRL_1_CHP_MODE_TRIM_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_ctrl_1_chp_mode_trim_default(&self) -> bool {
        *self == PLL_CTRL_1_CHP_MODE_TRIM_A::PLL_CTRL_1_CHP_MODE_TRIM_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CTRL_1_CHP_MODE_TRIM`"]
pub struct PLL_CTRL_1_CHP_MODE_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CTRL_1_CHP_MODE_TRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CTRL_1_CHP_MODE_TRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ctrl_1_chp_mode_trim_default(self) -> &'a mut W {
        self.variant(PLL_CTRL_1_CHP_MODE_TRIM_A::PLL_CTRL_1_CHP_MODE_TRIM_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Enable the common mode control block of the charge-pump. Must be high to ensure proper operation of the VCO tuning voltage comparator for sub-band selection, if used\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CTRL_1_CHP_CMC_EN_A {
    #[doc = "0: `0`"]
    PLL_CTRL_1_CHP_CMC_EN_DEFAULT = 0,
}
impl From<PLL_CTRL_1_CHP_CMC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CTRL_1_CHP_CMC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_CTRL_1_CHP_CMC_EN`"]
pub type PLL_CTRL_1_CHP_CMC_EN_R = crate::R<bool, PLL_CTRL_1_CHP_CMC_EN_A>;
impl PLL_CTRL_1_CHP_CMC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PLL_CTRL_1_CHP_CMC_EN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PLL_CTRL_1_CHP_CMC_EN_A::PLL_CTRL_1_CHP_CMC_EN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CTRL_1_CHP_CMC_EN_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_ctrl_1_chp_cmc_en_default(&self) -> bool {
        *self == PLL_CTRL_1_CHP_CMC_EN_A::PLL_CTRL_1_CHP_CMC_EN_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CTRL_1_CHP_CMC_EN`"]
pub struct PLL_CTRL_1_CHP_CMC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CTRL_1_CHP_CMC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CTRL_1_CHP_CMC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ctrl_1_chp_cmc_en_default(self) -> &'a mut W {
        self.variant(PLL_CTRL_1_CHP_CMC_EN_A::PLL_CTRL_1_CHP_CMC_EN_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Debug: enable the charge-pump offset current (see bits 7:6 for offset current value)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CTRL_1_CHP_CURR_OFFSET_EN_A {
    #[doc = "0: `0`"]
    PLL_CTRL_1_CHP_CURR_OFFSET_EN_DEFAULT = 0,
}
impl From<PLL_CTRL_1_CHP_CURR_OFFSET_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CTRL_1_CHP_CURR_OFFSET_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_CTRL_1_CHP_CURR_OFFSET_EN`"]
pub type PLL_CTRL_1_CHP_CURR_OFFSET_EN_R = crate::R<bool, PLL_CTRL_1_CHP_CURR_OFFSET_EN_A>;
impl PLL_CTRL_1_CHP_CURR_OFFSET_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PLL_CTRL_1_CHP_CURR_OFFSET_EN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PLL_CTRL_1_CHP_CURR_OFFSET_EN_A::PLL_CTRL_1_CHP_CURR_OFFSET_EN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CTRL_1_CHP_CURR_OFFSET_EN_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_ctrl_1_chp_curr_offset_en_default(&self) -> bool {
        *self == PLL_CTRL_1_CHP_CURR_OFFSET_EN_A::PLL_CTRL_1_CHP_CURR_OFFSET_EN_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CTRL_1_CHP_CURR_OFFSET_EN`"]
pub struct PLL_CTRL_1_CHP_CURR_OFFSET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CTRL_1_CHP_CURR_OFFSET_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CTRL_1_CHP_CURR_OFFSET_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ctrl_1_chp_curr_offset_en_default(self) -> &'a mut W {
        self.variant(PLL_CTRL_1_CHP_CURR_OFFSET_EN_A::PLL_CTRL_1_CHP_CURR_OFFSET_EN_DEFAULT)
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
    #[doc = "Bits 24:31 - trimming of the xtal: 5MSB thermometric, 3LSB direct"]
    #[inline(always)]
    pub fn xtal_trim_xtal_trim(&self) -> XTAL_TRIM_XTAL_TRIM_R {
        XTAL_TRIM_XTAL_TRIM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 20 - If set to 1 the PLL is set to 48MHz in Rx instead of 24MHz (need also to change ck_sel)"]
    #[inline(always)]
    pub fn pll_ctrl_2_pll_rx_48meg(&self) -> PLL_CTRL_2_PLL_RX_48MEG_R {
        PLL_CTRL_2_PLL_RX_48MEG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - If set to 1, in case of swcap_fsm=1, the register for Rx and Tx swcap is the same"]
    #[inline(always)]
    pub fn pll_ctrl_2_swcap_tx_same_rx(&self) -> PLL_CTRL_2_SWCAP_TX_SAME_RX_R {
        PLL_CTRL_2_SWCAP_TX_SAME_RX_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - If set to 1 use the swcap_fsm register as reference for the sub-band selection"]
    #[inline(always)]
    pub fn pll_ctrl_2_swcap_fsm(&self) -> PLL_CTRL_2_SWCAP_FSM_R {
        PLL_CTRL_2_SWCAP_FSM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Reset signal of the DLL (active low)"]
    #[inline(always)]
    pub fn pll_ctrl_2_dll_rstb(&self) -> PLL_CTRL_2_DLL_RSTB_R {
        PLL_CTRL_2_DLL_RSTB_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - VCO sub-band selection bits"]
    #[inline(always)]
    pub fn pll_ctrl_2_vco_subband_trim_hi(&self) -> PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_R {
        PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - VCO sub-band selection bits"]
    #[inline(always)]
    pub fn pll_ctrl_1_vco_subband_trim_lo(&self) -> PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_R {
        PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Add offset to sub-band selection comparator"]
    #[inline(always)]
    pub fn pll_ctrl_1_sub_sel_offs_en(&self) -> PLL_CTRL_1_SUB_SEL_OFFS_EN_R {
        PLL_CTRL_1_SUB_SEL_OFFS_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Debug: VCO signal divided by the programmable divider is divided by a: 0 => division ratio set to 1, 1 => division ratio set to 2; before to be outputted to ck_div_test"]
    #[inline(always)]
    pub fn pll_ctrl_1_div2_clkvco_test_en(&self) -> PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_R {
        PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Debug: enable to output on GPIO the VCO signal divided by the programmable divider"]
    #[inline(always)]
    pub fn pll_ctrl_1_vcodiv_clk_test_en(&self) -> PLL_CTRL_1_VCODIV_CLK_TEST_EN_R {
        PLL_CTRL_1_VCODIV_CLK_TEST_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When high, allow to decrease half time the bias current for the same output pumping current. Should be always high in IcyTRX."]
    #[inline(always)]
    pub fn pll_ctrl_1_en_low_chp_bias(&self) -> PLL_CTRL_1_EN_LOW_CHP_BIAS_R {
        PLL_CTRL_1_EN_LOW_CHP_BIAS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Debug: enable charge-pump dead zone (degraded PLL characteristics for test)"]
    #[inline(always)]
    pub fn pll_ctrl_1_chp_dead_zone_en(&self) -> PLL_CTRL_1_CHP_DEAD_ZONE_EN_R {
        PLL_CTRL_1_CHP_DEAD_ZONE_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Debug: charge-pump offset current values selection bits (see bit 6 to enable this mode): 00 => d_phi = 15, 01 => d_phi=22.5, 10 => d_phi = 30, 11 => d_phi = 60. Also sets the bias current of the common mode control block of the charge-pump. Must be sets to 01 to ensure a proper operation of the VCO tuning voltage comparator for sub-band selection, if used"]
    #[inline(always)]
    pub fn pll_ctrl_1_chp_curr_offset_trim(&self) -> PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_R {
        PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Enable the PLL filter high bandwidth needed in TX (must be high together with bit 4 in TX, low in RX)"]
    #[inline(always)]
    pub fn pll_ctrl_1_high_bw_filter_en(&self) -> PLL_CTRL_1_HIGH_BW_FILTER_EN_R {
        PLL_CTRL_1_HIGH_BW_FILTER_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable the high current output of the charge-pump for PLL TX high bandwidth mode (must be high together with bit 5 in TX, low in RX)"]
    #[inline(always)]
    pub fn pll_ctrl_1_fast_chp_en(&self) -> PLL_CTRL_1_FAST_CHP_EN_R {
        PLL_CTRL_1_FAST_CHP_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Charge-pump active if 00 else this allow to open the PLL and force the VCO tune voltage to reach: 01 => minimum frequency inside sub-band selection, 10 => medium frequency inside sub-band selection, 11 => maximum frequency inside sub-band selection."]
    #[inline(always)]
    pub fn pll_ctrl_1_chp_mode_trim(&self) -> PLL_CTRL_1_CHP_MODE_TRIM_R {
        PLL_CTRL_1_CHP_MODE_TRIM_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - Enable the common mode control block of the charge-pump. Must be high to ensure proper operation of the VCO tuning voltage comparator for sub-band selection, if used"]
    #[inline(always)]
    pub fn pll_ctrl_1_chp_cmc_en(&self) -> PLL_CTRL_1_CHP_CMC_EN_R {
        PLL_CTRL_1_CHP_CMC_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Debug: enable the charge-pump offset current (see bits 7:6 for offset current value)"]
    #[inline(always)]
    pub fn pll_ctrl_1_chp_curr_offset_en(&self) -> PLL_CTRL_1_CHP_CURR_OFFSET_EN_R {
        PLL_CTRL_1_CHP_CURR_OFFSET_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - trimming of the xtal: 5MSB thermometric, 3LSB direct"]
    #[inline(always)]
    pub fn xtal_trim_xtal_trim(&mut self) -> XTAL_TRIM_XTAL_TRIM_W {
        XTAL_TRIM_XTAL_TRIM_W { w: self }
    }
    #[doc = "Bit 20 - If set to 1 the PLL is set to 48MHz in Rx instead of 24MHz (need also to change ck_sel)"]
    #[inline(always)]
    pub fn pll_ctrl_2_pll_rx_48meg(&mut self) -> PLL_CTRL_2_PLL_RX_48MEG_W {
        PLL_CTRL_2_PLL_RX_48MEG_W { w: self }
    }
    #[doc = "Bit 19 - If set to 1, in case of swcap_fsm=1, the register for Rx and Tx swcap is the same"]
    #[inline(always)]
    pub fn pll_ctrl_2_swcap_tx_same_rx(&mut self) -> PLL_CTRL_2_SWCAP_TX_SAME_RX_W {
        PLL_CTRL_2_SWCAP_TX_SAME_RX_W { w: self }
    }
    #[doc = "Bit 18 - If set to 1 use the swcap_fsm register as reference for the sub-band selection"]
    #[inline(always)]
    pub fn pll_ctrl_2_swcap_fsm(&mut self) -> PLL_CTRL_2_SWCAP_FSM_W {
        PLL_CTRL_2_SWCAP_FSM_W { w: self }
    }
    #[doc = "Bit 17 - Reset signal of the DLL (active low)"]
    #[inline(always)]
    pub fn pll_ctrl_2_dll_rstb(&mut self) -> PLL_CTRL_2_DLL_RSTB_W {
        PLL_CTRL_2_DLL_RSTB_W { w: self }
    }
    #[doc = "Bit 16 - VCO sub-band selection bits"]
    #[inline(always)]
    pub fn pll_ctrl_2_vco_subband_trim_hi(&mut self) -> PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_W {
        PLL_CTRL_2_VCO_SUBBAND_TRIM_HI_W { w: self }
    }
    #[doc = "Bits 13:15 - VCO sub-band selection bits"]
    #[inline(always)]
    pub fn pll_ctrl_1_vco_subband_trim_lo(&mut self) -> PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_W {
        PLL_CTRL_1_VCO_SUBBAND_TRIM_LO_W { w: self }
    }
    #[doc = "Bit 12 - Add offset to sub-band selection comparator"]
    #[inline(always)]
    pub fn pll_ctrl_1_sub_sel_offs_en(&mut self) -> PLL_CTRL_1_SUB_SEL_OFFS_EN_W {
        PLL_CTRL_1_SUB_SEL_OFFS_EN_W { w: self }
    }
    #[doc = "Bit 11 - Debug: VCO signal divided by the programmable divider is divided by a: 0 => division ratio set to 1, 1 => division ratio set to 2; before to be outputted to ck_div_test"]
    #[inline(always)]
    pub fn pll_ctrl_1_div2_clkvco_test_en(&mut self) -> PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_W {
        PLL_CTRL_1_DIV2_CLKVCO_TEST_EN_W { w: self }
    }
    #[doc = "Bit 10 - Debug: enable to output on GPIO the VCO signal divided by the programmable divider"]
    #[inline(always)]
    pub fn pll_ctrl_1_vcodiv_clk_test_en(&mut self) -> PLL_CTRL_1_VCODIV_CLK_TEST_EN_W {
        PLL_CTRL_1_VCODIV_CLK_TEST_EN_W { w: self }
    }
    #[doc = "Bit 9 - When high, allow to decrease half time the bias current for the same output pumping current. Should be always high in IcyTRX."]
    #[inline(always)]
    pub fn pll_ctrl_1_en_low_chp_bias(&mut self) -> PLL_CTRL_1_EN_LOW_CHP_BIAS_W {
        PLL_CTRL_1_EN_LOW_CHP_BIAS_W { w: self }
    }
    #[doc = "Bit 8 - Debug: enable charge-pump dead zone (degraded PLL characteristics for test)"]
    #[inline(always)]
    pub fn pll_ctrl_1_chp_dead_zone_en(&mut self) -> PLL_CTRL_1_CHP_DEAD_ZONE_EN_W {
        PLL_CTRL_1_CHP_DEAD_ZONE_EN_W { w: self }
    }
    #[doc = "Bits 6:7 - Debug: charge-pump offset current values selection bits (see bit 6 to enable this mode): 00 => d_phi = 15, 01 => d_phi=22.5, 10 => d_phi = 30, 11 => d_phi = 60. Also sets the bias current of the common mode control block of the charge-pump. Must be sets to 01 to ensure a proper operation of the VCO tuning voltage comparator for sub-band selection, if used"]
    #[inline(always)]
    pub fn pll_ctrl_1_chp_curr_offset_trim(&mut self) -> PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_W {
        PLL_CTRL_1_CHP_CURR_OFFSET_TRIM_W { w: self }
    }
    #[doc = "Bit 5 - Enable the PLL filter high bandwidth needed in TX (must be high together with bit 4 in TX, low in RX)"]
    #[inline(always)]
    pub fn pll_ctrl_1_high_bw_filter_en(&mut self) -> PLL_CTRL_1_HIGH_BW_FILTER_EN_W {
        PLL_CTRL_1_HIGH_BW_FILTER_EN_W { w: self }
    }
    #[doc = "Bit 4 - Enable the high current output of the charge-pump for PLL TX high bandwidth mode (must be high together with bit 5 in TX, low in RX)"]
    #[inline(always)]
    pub fn pll_ctrl_1_fast_chp_en(&mut self) -> PLL_CTRL_1_FAST_CHP_EN_W {
        PLL_CTRL_1_FAST_CHP_EN_W { w: self }
    }
    #[doc = "Bits 2:3 - Charge-pump active if 00 else this allow to open the PLL and force the VCO tune voltage to reach: 01 => minimum frequency inside sub-band selection, 10 => medium frequency inside sub-band selection, 11 => maximum frequency inside sub-band selection."]
    #[inline(always)]
    pub fn pll_ctrl_1_chp_mode_trim(&mut self) -> PLL_CTRL_1_CHP_MODE_TRIM_W {
        PLL_CTRL_1_CHP_MODE_TRIM_W { w: self }
    }
    #[doc = "Bit 1 - Enable the common mode control block of the charge-pump. Must be high to ensure proper operation of the VCO tuning voltage comparator for sub-band selection, if used"]
    #[inline(always)]
    pub fn pll_ctrl_1_chp_cmc_en(&mut self) -> PLL_CTRL_1_CHP_CMC_EN_W {
        PLL_CTRL_1_CHP_CMC_EN_W { w: self }
    }
    #[doc = "Bit 0 - Debug: enable the charge-pump offset current (see bits 7:6 for offset current value)"]
    #[inline(always)]
    pub fn pll_ctrl_1_chp_curr_offset_en(&mut self) -> PLL_CTRL_1_CHP_CURR_OFFSET_EN_W {
        PLL_CTRL_1_CHP_CURR_OFFSET_EN_W { w: self }
    }
}
