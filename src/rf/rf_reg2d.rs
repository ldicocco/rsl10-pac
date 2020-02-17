#[doc = "Reader of register RF_REG2D"]
pub type R = crate::R<u32, super::RF_REG2D>;
#[doc = "Writer for register RF_REG2D"]
pub type W = crate::W<u32, super::RF_REG2D>;
#[doc = "Register RF_REG2D `reset()`'s with value 0x0030_0000"]
impl crate::ResetValue for super::RF_REG2D {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0030_0000
    }
}
#[doc = "Enable the subband correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUBBAND_CORR_SUBBAND_CORR_EN_A {
    #[doc = "0: `0`"]
    SUBBAND_CORR_SUBBAND_CORR_EN_DEFAULT = 0,
}
impl From<SUBBAND_CORR_SUBBAND_CORR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SUBBAND_CORR_SUBBAND_CORR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SUBBAND_CORR_SUBBAND_CORR_EN`"]
pub type SUBBAND_CORR_SUBBAND_CORR_EN_R = crate::R<bool, SUBBAND_CORR_SUBBAND_CORR_EN_A>;
impl SUBBAND_CORR_SUBBAND_CORR_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SUBBAND_CORR_SUBBAND_CORR_EN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(SUBBAND_CORR_SUBBAND_CORR_EN_A::SUBBAND_CORR_SUBBAND_CORR_EN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUBBAND_CORR_SUBBAND_CORR_EN_DEFAULT`"]
    #[inline(always)]
    pub fn is_subband_corr_subband_corr_en_default(&self) -> bool {
        *self == SUBBAND_CORR_SUBBAND_CORR_EN_A::SUBBAND_CORR_SUBBAND_CORR_EN_DEFAULT
    }
}
#[doc = "Write proxy for field `SUBBAND_CORR_SUBBAND_CORR_EN`"]
pub struct SUBBAND_CORR_SUBBAND_CORR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBBAND_CORR_SUBBAND_CORR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUBBAND_CORR_SUBBAND_CORR_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn subband_corr_subband_corr_en_default(self) -> &'a mut W {
        self.variant(SUBBAND_CORR_SUBBAND_CORR_EN_A::SUBBAND_CORR_SUBBAND_CORR_EN_DEFAULT)
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
#[doc = "Subband correction in Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBBAND_CORR_SUBBAND_CORR_RX_A {
    #[doc = "0: `0`"]
    SUBBAND_CORR_SUBBAND_CORR_RX_DEFAULT = 0,
}
impl From<SUBBAND_CORR_SUBBAND_CORR_RX_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBBAND_CORR_SUBBAND_CORR_RX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SUBBAND_CORR_SUBBAND_CORR_RX`"]
pub type SUBBAND_CORR_SUBBAND_CORR_RX_R = crate::R<u8, SUBBAND_CORR_SUBBAND_CORR_RX_A>;
impl SUBBAND_CORR_SUBBAND_CORR_RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUBBAND_CORR_SUBBAND_CORR_RX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SUBBAND_CORR_SUBBAND_CORR_RX_A::SUBBAND_CORR_SUBBAND_CORR_RX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUBBAND_CORR_SUBBAND_CORR_RX_DEFAULT`"]
    #[inline(always)]
    pub fn is_subband_corr_subband_corr_rx_default(&self) -> bool {
        *self == SUBBAND_CORR_SUBBAND_CORR_RX_A::SUBBAND_CORR_SUBBAND_CORR_RX_DEFAULT
    }
}
#[doc = "Write proxy for field `SUBBAND_CORR_SUBBAND_CORR_RX`"]
pub struct SUBBAND_CORR_SUBBAND_CORR_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBBAND_CORR_SUBBAND_CORR_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUBBAND_CORR_SUBBAND_CORR_RX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn subband_corr_subband_corr_rx_default(self) -> &'a mut W {
        self.variant(SUBBAND_CORR_SUBBAND_CORR_RX_A::SUBBAND_CORR_SUBBAND_CORR_RX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Subband correction in Tx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBBAND_CORR_SUBBAND_CORR_TX_A {
    #[doc = "0: `0`"]
    SUBBAND_CORR_SUBBAND_CORR_TX_DEFAULT = 0,
}
impl From<SUBBAND_CORR_SUBBAND_CORR_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBBAND_CORR_SUBBAND_CORR_TX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SUBBAND_CORR_SUBBAND_CORR_TX`"]
pub type SUBBAND_CORR_SUBBAND_CORR_TX_R = crate::R<u8, SUBBAND_CORR_SUBBAND_CORR_TX_A>;
impl SUBBAND_CORR_SUBBAND_CORR_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUBBAND_CORR_SUBBAND_CORR_TX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SUBBAND_CORR_SUBBAND_CORR_TX_A::SUBBAND_CORR_SUBBAND_CORR_TX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUBBAND_CORR_SUBBAND_CORR_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_subband_corr_subband_corr_tx_default(&self) -> bool {
        *self == SUBBAND_CORR_SUBBAND_CORR_TX_A::SUBBAND_CORR_SUBBAND_CORR_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `SUBBAND_CORR_SUBBAND_CORR_TX`"]
pub struct SUBBAND_CORR_SUBBAND_CORR_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBBAND_CORR_SUBBAND_CORR_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUBBAND_CORR_SUBBAND_CORR_TX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn subband_corr_subband_corr_tx_default(self) -> &'a mut W {
        self.variant(SUBBAND_CORR_SUBBAND_CORR_TX_A::SUBBAND_CORR_SUBBAND_CORR_TX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CONF_TX_NRX_INV_CLK_PLL_TX_A {
    #[doc = "0: `0`"]
    PLL_CONF_TX_NRX_INV_CLK_PLL_TX_DEFAULT = 0,
}
impl From<PLL_CONF_TX_NRX_INV_CLK_PLL_TX_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CONF_TX_NRX_INV_CLK_PLL_TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_CONF_TX_NRX_INV_CLK_PLL_TX`"]
pub type PLL_CONF_TX_NRX_INV_CLK_PLL_TX_R = crate::R<bool, PLL_CONF_TX_NRX_INV_CLK_PLL_TX_A>;
impl PLL_CONF_TX_NRX_INV_CLK_PLL_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PLL_CONF_TX_NRX_INV_CLK_PLL_TX_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PLL_CONF_TX_NRX_INV_CLK_PLL_TX_A::PLL_CONF_TX_NRX_INV_CLK_PLL_TX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CONF_TX_NRX_INV_CLK_PLL_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_conf_tx_nrx_inv_clk_pll_tx_default(&self) -> bool {
        *self == PLL_CONF_TX_NRX_INV_CLK_PLL_TX_A::PLL_CONF_TX_NRX_INV_CLK_PLL_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CONF_TX_NRX_INV_CLK_PLL_TX`"]
pub struct PLL_CONF_TX_NRX_INV_CLK_PLL_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CONF_TX_NRX_INV_CLK_PLL_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CONF_TX_NRX_INV_CLK_PLL_TX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_conf_tx_nrx_inv_clk_pll_tx_default(self) -> &'a mut W {
        self.variant(PLL_CONF_TX_NRX_INV_CLK_PLL_TX_A::PLL_CONF_TX_NRX_INV_CLK_PLL_TX_DEFAULT)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CONF_TX_NRX_INV_CLK_DIG_TX_A {
    #[doc = "0: `0`"]
    PLL_CONF_TX_NRX_INV_CLK_DIG_TX_DEFAULT = 0,
}
impl From<PLL_CONF_TX_NRX_INV_CLK_DIG_TX_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CONF_TX_NRX_INV_CLK_DIG_TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_CONF_TX_NRX_INV_CLK_DIG_TX`"]
pub type PLL_CONF_TX_NRX_INV_CLK_DIG_TX_R = crate::R<bool, PLL_CONF_TX_NRX_INV_CLK_DIG_TX_A>;
impl PLL_CONF_TX_NRX_INV_CLK_DIG_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PLL_CONF_TX_NRX_INV_CLK_DIG_TX_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PLL_CONF_TX_NRX_INV_CLK_DIG_TX_A::PLL_CONF_TX_NRX_INV_CLK_DIG_TX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CONF_TX_NRX_INV_CLK_DIG_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_conf_tx_nrx_inv_clk_dig_tx_default(&self) -> bool {
        *self == PLL_CONF_TX_NRX_INV_CLK_DIG_TX_A::PLL_CONF_TX_NRX_INV_CLK_DIG_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CONF_TX_NRX_INV_CLK_DIG_TX`"]
pub struct PLL_CONF_TX_NRX_INV_CLK_DIG_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CONF_TX_NRX_INV_CLK_DIG_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CONF_TX_NRX_INV_CLK_DIG_TX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_conf_tx_nrx_inv_clk_dig_tx_default(self) -> &'a mut W {
        self.variant(PLL_CONF_TX_NRX_INV_CLK_DIG_TX_A::PLL_CONF_TX_NRX_INV_CLK_DIG_TX_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Xor value between Tx and Rx for the ck_sel field of register DLL_CTRL\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLL_CONF_TX_NRX_CK_SEL_TX_A {
    #[doc = "3: `11`"]
    PLL_CONF_TX_NRX_CK_SEL_TX_DEFAULT = 3,
}
impl From<PLL_CONF_TX_NRX_CK_SEL_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL_CONF_TX_NRX_CK_SEL_TX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLL_CONF_TX_NRX_CK_SEL_TX`"]
pub type PLL_CONF_TX_NRX_CK_SEL_TX_R = crate::R<u8, PLL_CONF_TX_NRX_CK_SEL_TX_A>;
impl PLL_CONF_TX_NRX_CK_SEL_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLL_CONF_TX_NRX_CK_SEL_TX_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(PLL_CONF_TX_NRX_CK_SEL_TX_A::PLL_CONF_TX_NRX_CK_SEL_TX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CONF_TX_NRX_CK_SEL_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_conf_tx_nrx_ck_sel_tx_default(&self) -> bool {
        *self == PLL_CONF_TX_NRX_CK_SEL_TX_A::PLL_CONF_TX_NRX_CK_SEL_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CONF_TX_NRX_CK_SEL_TX`"]
pub struct PLL_CONF_TX_NRX_CK_SEL_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CONF_TX_NRX_CK_SEL_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CONF_TX_NRX_CK_SEL_TX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pll_conf_tx_nrx_ck_sel_tx_default(self) -> &'a mut W {
        self.variant(PLL_CONF_TX_NRX_CK_SEL_TX_A::PLL_CONF_TX_NRX_CK_SEL_TX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_A {
    #[doc = "0: `0`"]
    PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_DEFAULT = 0,
}
impl From<PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX`"]
pub type PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_R =
    crate::R<u8, PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_A>;
impl PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_A> {
        use crate::Variant::*;
        match self . bits { 0 => Val ( PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_A :: PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_DEFAULT ) , i => Res ( i ) }
    }
    #[doc = "Checks if the value of the field is `PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_conf_tx_nrx_chp_curr_off_trim_tx_default(&self) -> bool {
        *self
            == PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_A::PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX`"]
pub struct PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_conf_tx_nrx_chp_curr_off_trim_tx_default(self) -> &'a mut W {
        self.variant(
            PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_A::PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_DEFAULT,
        )
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_A {
    #[doc = "0: `0`"]
    PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_DEFAULT = 0,
}
impl From<PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX`"]
pub type PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_R =
    crate::R<bool, PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_A>;
impl PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(
                PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_A::PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_DEFAULT,
            ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_conf_tx_nrx_chp_curr_off_en_tx_default(&self) -> bool {
        *self == PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_A::PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX`"]
pub struct PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_conf_tx_nrx_chp_curr_off_en_tx_default(self) -> &'a mut W {
        self.variant(
            PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_A::PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_DEFAULT,
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "If set to 1, the PA rampup uses the PA backoff enable bit (from -40 dBm)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA_RAMPUP_FULL_PA_RAMPUP_A {
    #[doc = "0: `0`"]
    PA_RAMPUP_FULL_PA_RAMPUP_DEFAULT = 0,
}
impl From<PA_RAMPUP_FULL_PA_RAMPUP_A> for bool {
    #[inline(always)]
    fn from(variant: PA_RAMPUP_FULL_PA_RAMPUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PA_RAMPUP_FULL_PA_RAMPUP`"]
pub type PA_RAMPUP_FULL_PA_RAMPUP_R = crate::R<bool, PA_RAMPUP_FULL_PA_RAMPUP_A>;
impl PA_RAMPUP_FULL_PA_RAMPUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PA_RAMPUP_FULL_PA_RAMPUP_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PA_RAMPUP_FULL_PA_RAMPUP_A::PA_RAMPUP_FULL_PA_RAMPUP_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA_RAMPUP_FULL_PA_RAMPUP_DEFAULT`"]
    #[inline(always)]
    pub fn is_pa_rampup_full_pa_rampup_default(&self) -> bool {
        *self == PA_RAMPUP_FULL_PA_RAMPUP_A::PA_RAMPUP_FULL_PA_RAMPUP_DEFAULT
    }
}
#[doc = "Write proxy for field `PA_RAMPUP_FULL_PA_RAMPUP`"]
pub struct PA_RAMPUP_FULL_PA_RAMPUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_RAMPUP_FULL_PA_RAMPUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PA_RAMPUP_FULL_PA_RAMPUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pa_rampup_full_pa_rampup_default(self) -> &'a mut W {
        self.variant(PA_RAMPUP_FULL_PA_RAMPUP_A::PA_RAMPUP_FULL_PA_RAMPUP_DEFAULT)
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
#[doc = "time to wait to start the ramp-up after the PA enable is detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PA_RAMPUP_DEL_PA_RAMPUP_A {
    #[doc = "0: `0`"]
    PA_RAMPUP_DEL_PA_RAMPUP_DEFAULT = 0,
}
impl From<PA_RAMPUP_DEL_PA_RAMPUP_A> for u8 {
    #[inline(always)]
    fn from(variant: PA_RAMPUP_DEL_PA_RAMPUP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PA_RAMPUP_DEL_PA_RAMPUP`"]
pub type PA_RAMPUP_DEL_PA_RAMPUP_R = crate::R<u8, PA_RAMPUP_DEL_PA_RAMPUP_A>;
impl PA_RAMPUP_DEL_PA_RAMPUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PA_RAMPUP_DEL_PA_RAMPUP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PA_RAMPUP_DEL_PA_RAMPUP_A::PA_RAMPUP_DEL_PA_RAMPUP_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA_RAMPUP_DEL_PA_RAMPUP_DEFAULT`"]
    #[inline(always)]
    pub fn is_pa_rampup_del_pa_rampup_default(&self) -> bool {
        *self == PA_RAMPUP_DEL_PA_RAMPUP_A::PA_RAMPUP_DEL_PA_RAMPUP_DEFAULT
    }
}
#[doc = "Write proxy for field `PA_RAMPUP_DEL_PA_RAMPUP`"]
pub struct PA_RAMPUP_DEL_PA_RAMPUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_RAMPUP_DEL_PA_RAMPUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PA_RAMPUP_DEL_PA_RAMPUP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pa_rampup_del_pa_rampup_default(self) -> &'a mut W {
        self.variant(PA_RAMPUP_DEL_PA_RAMPUP_A::PA_RAMPUP_DEL_PA_RAMPUP_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "time constant of the Ramp-up/Ramp-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PA_RAMPUP_TAU_PA_RAMPUP_A {
    #[doc = "0: `0`"]
    PA_RAMPUP_TAU_PA_RAMPUP_DEFAULT = 0,
}
impl From<PA_RAMPUP_TAU_PA_RAMPUP_A> for u8 {
    #[inline(always)]
    fn from(variant: PA_RAMPUP_TAU_PA_RAMPUP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PA_RAMPUP_TAU_PA_RAMPUP`"]
pub type PA_RAMPUP_TAU_PA_RAMPUP_R = crate::R<u8, PA_RAMPUP_TAU_PA_RAMPUP_A>;
impl PA_RAMPUP_TAU_PA_RAMPUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PA_RAMPUP_TAU_PA_RAMPUP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PA_RAMPUP_TAU_PA_RAMPUP_A::PA_RAMPUP_TAU_PA_RAMPUP_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA_RAMPUP_TAU_PA_RAMPUP_DEFAULT`"]
    #[inline(always)]
    pub fn is_pa_rampup_tau_pa_rampup_default(&self) -> bool {
        *self == PA_RAMPUP_TAU_PA_RAMPUP_A::PA_RAMPUP_TAU_PA_RAMPUP_DEFAULT
    }
}
#[doc = "Write proxy for field `PA_RAMPUP_TAU_PA_RAMPUP`"]
pub struct PA_RAMPUP_TAU_PA_RAMPUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_RAMPUP_TAU_PA_RAMPUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PA_RAMPUP_TAU_PA_RAMPUP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pa_rampup_tau_pa_rampup_default(self) -> &'a mut W {
        self.variant(PA_RAMPUP_TAU_PA_RAMPUP_A::PA_RAMPUP_TAU_PA_RAMPUP_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "if set to 1 enables the PA ramp-down. Only valid in case of ramp-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA_RAMPUP_EN_PA_RAMPDOWN_A {
    #[doc = "0: `0`"]
    PA_RAMPUP_EN_PA_RAMPDOWN_DEFAULT = 0,
}
impl From<PA_RAMPUP_EN_PA_RAMPDOWN_A> for bool {
    #[inline(always)]
    fn from(variant: PA_RAMPUP_EN_PA_RAMPDOWN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PA_RAMPUP_EN_PA_RAMPDOWN`"]
pub type PA_RAMPUP_EN_PA_RAMPDOWN_R = crate::R<bool, PA_RAMPUP_EN_PA_RAMPDOWN_A>;
impl PA_RAMPUP_EN_PA_RAMPDOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PA_RAMPUP_EN_PA_RAMPDOWN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PA_RAMPUP_EN_PA_RAMPDOWN_A::PA_RAMPUP_EN_PA_RAMPDOWN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA_RAMPUP_EN_PA_RAMPDOWN_DEFAULT`"]
    #[inline(always)]
    pub fn is_pa_rampup_en_pa_rampdown_default(&self) -> bool {
        *self == PA_RAMPUP_EN_PA_RAMPDOWN_A::PA_RAMPUP_EN_PA_RAMPDOWN_DEFAULT
    }
}
#[doc = "Write proxy for field `PA_RAMPUP_EN_PA_RAMPDOWN`"]
pub struct PA_RAMPUP_EN_PA_RAMPDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_RAMPUP_EN_PA_RAMPDOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PA_RAMPUP_EN_PA_RAMPDOWN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pa_rampup_en_pa_rampdown_default(self) -> &'a mut W {
        self.variant(PA_RAMPUP_EN_PA_RAMPDOWN_A::PA_RAMPUP_EN_PA_RAMPDOWN_DEFAULT)
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
#[doc = "if set to 1 enables the PA ramp-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA_RAMPUP_EN_PA_RAMPUP_A {
    #[doc = "0: `0`"]
    PA_RAMPUP_EN_PA_RAMPUP_DEFAULT = 0,
}
impl From<PA_RAMPUP_EN_PA_RAMPUP_A> for bool {
    #[inline(always)]
    fn from(variant: PA_RAMPUP_EN_PA_RAMPUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PA_RAMPUP_EN_PA_RAMPUP`"]
pub type PA_RAMPUP_EN_PA_RAMPUP_R = crate::R<bool, PA_RAMPUP_EN_PA_RAMPUP_A>;
impl PA_RAMPUP_EN_PA_RAMPUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PA_RAMPUP_EN_PA_RAMPUP_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PA_RAMPUP_EN_PA_RAMPUP_A::PA_RAMPUP_EN_PA_RAMPUP_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA_RAMPUP_EN_PA_RAMPUP_DEFAULT`"]
    #[inline(always)]
    pub fn is_pa_rampup_en_pa_rampup_default(&self) -> bool {
        *self == PA_RAMPUP_EN_PA_RAMPUP_A::PA_RAMPUP_EN_PA_RAMPUP_DEFAULT
    }
}
#[doc = "Write proxy for field `PA_RAMPUP_EN_PA_RAMPUP`"]
pub struct PA_RAMPUP_EN_PA_RAMPUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_RAMPUP_EN_PA_RAMPUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PA_RAMPUP_EN_PA_RAMPUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pa_rampup_en_pa_rampup_default(self) -> &'a mut W {
        self.variant(PA_RAMPUP_EN_PA_RAMPUP_A::PA_RAMPUP_EN_PA_RAMPUP_DEFAULT)
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
#[doc = "Unused bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MISC_SPARES_A {
    #[doc = "0: `0`"]
    MISC_SPARES_DEFAULT = 0,
}
impl From<MISC_SPARES_A> for u8 {
    #[inline(always)]
    fn from(variant: MISC_SPARES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MISC_SPARES`"]
pub type MISC_SPARES_R = crate::R<u8, MISC_SPARES_A>;
impl MISC_SPARES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MISC_SPARES_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MISC_SPARES_A::MISC_SPARES_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MISC_SPARES_DEFAULT`"]
    #[inline(always)]
    pub fn is_misc_spares_default(&self) -> bool {
        *self == MISC_SPARES_A::MISC_SPARES_DEFAULT
    }
}
#[doc = "Write proxy for field `MISC_SPARES`"]
pub struct MISC_SPARES_W<'a> {
    w: &'a mut W,
}
impl<'a> MISC_SPARES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MISC_SPARES_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn misc_spares_default(self) -> &'a mut W {
        self.variant(MISC_SPARES_A::MISC_SPARES_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "RSSI pre-attenuator: 00 => 0dB, 01 => 4dB, 10 => 8dB, 11 => 12dB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MISC_RSSI_PRE_ATT_A {
    #[doc = "0: `0`"]
    MISC_RSSI_PRE_ATT_DEFAULT = 0,
}
impl From<MISC_RSSI_PRE_ATT_A> for u8 {
    #[inline(always)]
    fn from(variant: MISC_RSSI_PRE_ATT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MISC_RSSI_PRE_ATT`"]
pub type MISC_RSSI_PRE_ATT_R = crate::R<u8, MISC_RSSI_PRE_ATT_A>;
impl MISC_RSSI_PRE_ATT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MISC_RSSI_PRE_ATT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MISC_RSSI_PRE_ATT_A::MISC_RSSI_PRE_ATT_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MISC_RSSI_PRE_ATT_DEFAULT`"]
    #[inline(always)]
    pub fn is_misc_rssi_pre_att_default(&self) -> bool {
        *self == MISC_RSSI_PRE_ATT_A::MISC_RSSI_PRE_ATT_DEFAULT
    }
}
#[doc = "Write proxy for field `MISC_RSSI_PRE_ATT`"]
pub struct MISC_RSSI_PRE_ATT_W<'a> {
    w: &'a mut W,
}
impl<'a> MISC_RSSI_PRE_ATT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MISC_RSSI_PRE_ATT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn misc_rssi_pre_att_default(self) -> &'a mut W {
        self.variant(MISC_RSSI_PRE_ATT_A::MISC_RSSI_PRE_ATT_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "XTAL: if set to 1, the clk_ready threshold is set to a lower value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MISC_XTAL_LOW_CLK_READY_TH_EN_A {
    #[doc = "0: `0`"]
    MISC_XTAL_LOW_CLK_READY_TH_EN_DEFAULT = 0,
}
impl From<MISC_XTAL_LOW_CLK_READY_TH_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MISC_XTAL_LOW_CLK_READY_TH_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MISC_XTAL_LOW_CLK_READY_TH_EN`"]
pub type MISC_XTAL_LOW_CLK_READY_TH_EN_R = crate::R<bool, MISC_XTAL_LOW_CLK_READY_TH_EN_A>;
impl MISC_XTAL_LOW_CLK_READY_TH_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MISC_XTAL_LOW_CLK_READY_TH_EN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MISC_XTAL_LOW_CLK_READY_TH_EN_A::MISC_XTAL_LOW_CLK_READY_TH_EN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MISC_XTAL_LOW_CLK_READY_TH_EN_DEFAULT`"]
    #[inline(always)]
    pub fn is_misc_xtal_low_clk_ready_th_en_default(&self) -> bool {
        *self == MISC_XTAL_LOW_CLK_READY_TH_EN_A::MISC_XTAL_LOW_CLK_READY_TH_EN_DEFAULT
    }
}
#[doc = "Write proxy for field `MISC_XTAL_LOW_CLK_READY_TH_EN`"]
pub struct MISC_XTAL_LOW_CLK_READY_TH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MISC_XTAL_LOW_CLK_READY_TH_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MISC_XTAL_LOW_CLK_READY_TH_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn misc_xtal_low_clk_ready_th_en_default(self) -> &'a mut W {
        self.variant(MISC_XTAL_LOW_CLK_READY_TH_EN_A::MISC_XTAL_LOW_CLK_READY_TH_EN_DEFAULT)
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
    #[doc = "Bit 31 - Enable the subband correction"]
    #[inline(always)]
    pub fn subband_corr_subband_corr_en(&self) -> SUBBAND_CORR_SUBBAND_CORR_EN_R {
        SUBBAND_CORR_SUBBAND_CORR_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - Subband correction in Rx"]
    #[inline(always)]
    pub fn subband_corr_subband_corr_rx(&self) -> SUBBAND_CORR_SUBBAND_CORR_RX_R {
        SUBBAND_CORR_SUBBAND_CORR_RX_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Subband correction in Tx"]
    #[inline(always)]
    pub fn subband_corr_subband_corr_tx(&self) -> SUBBAND_CORR_SUBBAND_CORR_TX_R {
        SUBBAND_CORR_SUBBAND_CORR_TX_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pll_conf_tx_nrx_inv_clk_pll_tx(&self) -> PLL_CONF_TX_NRX_INV_CLK_PLL_TX_R {
        PLL_CONF_TX_NRX_INV_CLK_PLL_TX_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pll_conf_tx_nrx_inv_clk_dig_tx(&self) -> PLL_CONF_TX_NRX_INV_CLK_DIG_TX_R {
        PLL_CONF_TX_NRX_INV_CLK_DIG_TX_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Xor value between Tx and Rx for the ck_sel field of register DLL_CTRL"]
    #[inline(always)]
    pub fn pll_conf_tx_nrx_ck_sel_tx(&self) -> PLL_CONF_TX_NRX_CK_SEL_TX_R {
        PLL_CONF_TX_NRX_CK_SEL_TX_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn pll_conf_tx_nrx_chp_curr_off_trim_tx(&self) -> PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_R {
        PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pll_conf_tx_nrx_chp_curr_off_en_tx(&self) -> PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_R {
        PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - If set to 1, the PA rampup uses the PA backoff enable bit (from -40 dBm)"]
    #[inline(always)]
    pub fn pa_rampup_full_pa_rampup(&self) -> PA_RAMPUP_FULL_PA_RAMPUP_R {
        PA_RAMPUP_FULL_PA_RAMPUP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - time to wait to start the ramp-up after the PA enable is detected"]
    #[inline(always)]
    pub fn pa_rampup_del_pa_rampup(&self) -> PA_RAMPUP_DEL_PA_RAMPUP_R {
        PA_RAMPUP_DEL_PA_RAMPUP_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 10:11 - time constant of the Ramp-up/Ramp-down"]
    #[inline(always)]
    pub fn pa_rampup_tau_pa_rampup(&self) -> PA_RAMPUP_TAU_PA_RAMPUP_R {
        PA_RAMPUP_TAU_PA_RAMPUP_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - if set to 1 enables the PA ramp-down. Only valid in case of ramp-up"]
    #[inline(always)]
    pub fn pa_rampup_en_pa_rampdown(&self) -> PA_RAMPUP_EN_PA_RAMPDOWN_R {
        PA_RAMPUP_EN_PA_RAMPDOWN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - if set to 1 enables the PA ramp-up"]
    #[inline(always)]
    pub fn pa_rampup_en_pa_rampup(&self) -> PA_RAMPUP_EN_PA_RAMPUP_R {
        PA_RAMPUP_EN_PA_RAMPUP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:7 - Unused bits"]
    #[inline(always)]
    pub fn misc_spares(&self) -> MISC_SPARES_R {
        MISC_SPARES_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 1:2 - RSSI pre-attenuator: 00 => 0dB, 01 => 4dB, 10 => 8dB, 11 => 12dB"]
    #[inline(always)]
    pub fn misc_rssi_pre_att(&self) -> MISC_RSSI_PRE_ATT_R {
        MISC_RSSI_PRE_ATT_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - XTAL: if set to 1, the clk_ready threshold is set to a lower value"]
    #[inline(always)]
    pub fn misc_xtal_low_clk_ready_th_en(&self) -> MISC_XTAL_LOW_CLK_READY_TH_EN_R {
        MISC_XTAL_LOW_CLK_READY_TH_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Enable the subband correction"]
    #[inline(always)]
    pub fn subband_corr_subband_corr_en(&mut self) -> SUBBAND_CORR_SUBBAND_CORR_EN_W {
        SUBBAND_CORR_SUBBAND_CORR_EN_W { w: self }
    }
    #[doc = "Bits 28:30 - Subband correction in Rx"]
    #[inline(always)]
    pub fn subband_corr_subband_corr_rx(&mut self) -> SUBBAND_CORR_SUBBAND_CORR_RX_W {
        SUBBAND_CORR_SUBBAND_CORR_RX_W { w: self }
    }
    #[doc = "Bits 24:26 - Subband correction in Tx"]
    #[inline(always)]
    pub fn subband_corr_subband_corr_tx(&mut self) -> SUBBAND_CORR_SUBBAND_CORR_TX_W {
        SUBBAND_CORR_SUBBAND_CORR_TX_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pll_conf_tx_nrx_inv_clk_pll_tx(&mut self) -> PLL_CONF_TX_NRX_INV_CLK_PLL_TX_W {
        PLL_CONF_TX_NRX_INV_CLK_PLL_TX_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pll_conf_tx_nrx_inv_clk_dig_tx(&mut self) -> PLL_CONF_TX_NRX_INV_CLK_DIG_TX_W {
        PLL_CONF_TX_NRX_INV_CLK_DIG_TX_W { w: self }
    }
    #[doc = "Bits 20:21 - Xor value between Tx and Rx for the ck_sel field of register DLL_CTRL"]
    #[inline(always)]
    pub fn pll_conf_tx_nrx_ck_sel_tx(&mut self) -> PLL_CONF_TX_NRX_CK_SEL_TX_W {
        PLL_CONF_TX_NRX_CK_SEL_TX_W { w: self }
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn pll_conf_tx_nrx_chp_curr_off_trim_tx(
        &mut self,
    ) -> PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_W {
        PLL_CONF_TX_NRX_CHP_CURR_OFF_TRIM_TX_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pll_conf_tx_nrx_chp_curr_off_en_tx(&mut self) -> PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_W {
        PLL_CONF_TX_NRX_CHP_CURR_OFF_EN_TX_W { w: self }
    }
    #[doc = "Bit 15 - If set to 1, the PA rampup uses the PA backoff enable bit (from -40 dBm)"]
    #[inline(always)]
    pub fn pa_rampup_full_pa_rampup(&mut self) -> PA_RAMPUP_FULL_PA_RAMPUP_W {
        PA_RAMPUP_FULL_PA_RAMPUP_W { w: self }
    }
    #[doc = "Bits 12:14 - time to wait to start the ramp-up after the PA enable is detected"]
    #[inline(always)]
    pub fn pa_rampup_del_pa_rampup(&mut self) -> PA_RAMPUP_DEL_PA_RAMPUP_W {
        PA_RAMPUP_DEL_PA_RAMPUP_W { w: self }
    }
    #[doc = "Bits 10:11 - time constant of the Ramp-up/Ramp-down"]
    #[inline(always)]
    pub fn pa_rampup_tau_pa_rampup(&mut self) -> PA_RAMPUP_TAU_PA_RAMPUP_W {
        PA_RAMPUP_TAU_PA_RAMPUP_W { w: self }
    }
    #[doc = "Bit 9 - if set to 1 enables the PA ramp-down. Only valid in case of ramp-up"]
    #[inline(always)]
    pub fn pa_rampup_en_pa_rampdown(&mut self) -> PA_RAMPUP_EN_PA_RAMPDOWN_W {
        PA_RAMPUP_EN_PA_RAMPDOWN_W { w: self }
    }
    #[doc = "Bit 8 - if set to 1 enables the PA ramp-up"]
    #[inline(always)]
    pub fn pa_rampup_en_pa_rampup(&mut self) -> PA_RAMPUP_EN_PA_RAMPUP_W {
        PA_RAMPUP_EN_PA_RAMPUP_W { w: self }
    }
    #[doc = "Bits 3:7 - Unused bits"]
    #[inline(always)]
    pub fn misc_spares(&mut self) -> MISC_SPARES_W {
        MISC_SPARES_W { w: self }
    }
    #[doc = "Bits 1:2 - RSSI pre-attenuator: 00 => 0dB, 01 => 4dB, 10 => 8dB, 11 => 12dB"]
    #[inline(always)]
    pub fn misc_rssi_pre_att(&mut self) -> MISC_RSSI_PRE_ATT_W {
        MISC_RSSI_PRE_ATT_W { w: self }
    }
    #[doc = "Bit 0 - XTAL: if set to 1, the clk_ready threshold is set to a lower value"]
    #[inline(always)]
    pub fn misc_xtal_low_clk_ready_th_en(&mut self) -> MISC_XTAL_LOW_CLK_READY_TH_EN_W {
        MISC_XTAL_LOW_CLK_READY_TH_EN_W { w: self }
    }
}
