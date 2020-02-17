#[doc = "Reader of register RF_REG19"]
pub type R = crate::R<u32, super::RF_REG19>;
#[doc = "Writer for register RF_REG19"]
pub type W = crate::W<u32, super::RF_REG19>;
#[doc = "Register RF_REG19 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG19 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Same as pll_filter_res_trim but for Tx case. Real value in Tx is pll_filter_res_trim xor pll_filter_res_trim_tx. If set to 0, Tx and Rx have the same value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLL_BANK_PLL_FILTER_RES_TRIM_TX_A {
    #[doc = "0: `0`"]
    PLL_BANK_PLL_FILTER_RES_TRIM_TX_DEFAULT = 0,
}
impl From<PLL_BANK_PLL_FILTER_RES_TRIM_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL_BANK_PLL_FILTER_RES_TRIM_TX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLL_BANK_PLL_FILTER_RES_TRIM_TX`"]
pub type PLL_BANK_PLL_FILTER_RES_TRIM_TX_R = crate::R<u8, PLL_BANK_PLL_FILTER_RES_TRIM_TX_A>;
impl PLL_BANK_PLL_FILTER_RES_TRIM_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLL_BANK_PLL_FILTER_RES_TRIM_TX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PLL_BANK_PLL_FILTER_RES_TRIM_TX_A::PLL_BANK_PLL_FILTER_RES_TRIM_TX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_BANK_PLL_FILTER_RES_TRIM_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_bank_pll_filter_res_trim_tx_default(&self) -> bool {
        *self == PLL_BANK_PLL_FILTER_RES_TRIM_TX_A::PLL_BANK_PLL_FILTER_RES_TRIM_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_BANK_PLL_FILTER_RES_TRIM_TX`"]
pub struct PLL_BANK_PLL_FILTER_RES_TRIM_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_BANK_PLL_FILTER_RES_TRIM_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_BANK_PLL_FILTER_RES_TRIM_TX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_bank_pll_filter_res_trim_tx_default(self) -> &'a mut W {
        self.variant(PLL_BANK_PLL_FILTER_RES_TRIM_TX_A::PLL_BANK_PLL_FILTER_RES_TRIM_TX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Charge pump bias for Tx case. Real value in Tx is iq_pll_0 xor iq_pll_0_tx. If set to 0, Tx and Rx have the same value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLL_BANK_IQ_PLL_0_TX_A {
    #[doc = "0: `0`"]
    PLL_BANK_IQ_PLL_0_TX_DEFAULT = 0,
}
impl From<PLL_BANK_IQ_PLL_0_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL_BANK_IQ_PLL_0_TX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLL_BANK_IQ_PLL_0_TX`"]
pub type PLL_BANK_IQ_PLL_0_TX_R = crate::R<u8, PLL_BANK_IQ_PLL_0_TX_A>;
impl PLL_BANK_IQ_PLL_0_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLL_BANK_IQ_PLL_0_TX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PLL_BANK_IQ_PLL_0_TX_A::PLL_BANK_IQ_PLL_0_TX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_BANK_IQ_PLL_0_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_bank_iq_pll_0_tx_default(&self) -> bool {
        *self == PLL_BANK_IQ_PLL_0_TX_A::PLL_BANK_IQ_PLL_0_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_BANK_IQ_PLL_0_TX`"]
pub struct PLL_BANK_IQ_PLL_0_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_BANK_IQ_PLL_0_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_BANK_IQ_PLL_0_TX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_bank_iq_pll_0_tx_default(self) -> &'a mut W {
        self.variant(PLL_BANK_IQ_PLL_0_TX_A::PLL_BANK_IQ_PLL_0_TX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "If set to 1 the Tx will work in low data-rate mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_BANK_LOW_DR_TX_A {
    #[doc = "0: `0`"]
    PLL_BANK_LOW_DR_TX_DEFAULT = 0,
}
impl From<PLL_BANK_LOW_DR_TX_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_BANK_LOW_DR_TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL_BANK_LOW_DR_TX`"]
pub type PLL_BANK_LOW_DR_TX_R = crate::R<bool, PLL_BANK_LOW_DR_TX_A>;
impl PLL_BANK_LOW_DR_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PLL_BANK_LOW_DR_TX_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PLL_BANK_LOW_DR_TX_A::PLL_BANK_LOW_DR_TX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_BANK_LOW_DR_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_bank_low_dr_tx_default(&self) -> bool {
        *self == PLL_BANK_LOW_DR_TX_A::PLL_BANK_LOW_DR_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_BANK_LOW_DR_TX`"]
pub struct PLL_BANK_LOW_DR_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_BANK_LOW_DR_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_BANK_LOW_DR_TX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_bank_low_dr_tx_default(self) -> &'a mut W {
        self.variant(PLL_BANK_LOW_DR_TX_A::PLL_BANK_LOW_DR_TX_DEFAULT)
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
#[doc = "Allow to modify the value of the loop filter resistor R2 when bit 5 is high (TX mode): 00 => normal resistor (R_2_typ), 01 => 123 percent, 10 => 130 percent 11 => 170 percent\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLL_BANK_PLL_FILTER_RES_TRIM_A {
    #[doc = "0: `0`"]
    PLL_BANK_PLL_FILTER_RES_TRIM_DEFAULT = 0,
}
impl From<PLL_BANK_PLL_FILTER_RES_TRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL_BANK_PLL_FILTER_RES_TRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLL_BANK_PLL_FILTER_RES_TRIM`"]
pub type PLL_BANK_PLL_FILTER_RES_TRIM_R = crate::R<u8, PLL_BANK_PLL_FILTER_RES_TRIM_A>;
impl PLL_BANK_PLL_FILTER_RES_TRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLL_BANK_PLL_FILTER_RES_TRIM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PLL_BANK_PLL_FILTER_RES_TRIM_A::PLL_BANK_PLL_FILTER_RES_TRIM_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_BANK_PLL_FILTER_RES_TRIM_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_bank_pll_filter_res_trim_default(&self) -> bool {
        *self == PLL_BANK_PLL_FILTER_RES_TRIM_A::PLL_BANK_PLL_FILTER_RES_TRIM_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_BANK_PLL_FILTER_RES_TRIM`"]
pub struct PLL_BANK_PLL_FILTER_RES_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_BANK_PLL_FILTER_RES_TRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_BANK_PLL_FILTER_RES_TRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_bank_pll_filter_res_trim_default(self) -> &'a mut W {
        self.variant(PLL_BANK_PLL_FILTER_RES_TRIM_A::PLL_BANK_PLL_FILTER_RES_TRIM_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Charge pump bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLL_BANK_IQ_PLL_0_A {
    #[doc = "0: `0`"]
    PLL_BANK_IQ_PLL_0_DEFAULT = 0,
}
impl From<PLL_BANK_IQ_PLL_0_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL_BANK_IQ_PLL_0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLL_BANK_IQ_PLL_0`"]
pub type PLL_BANK_IQ_PLL_0_R = crate::R<u8, PLL_BANK_IQ_PLL_0_A>;
impl PLL_BANK_IQ_PLL_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLL_BANK_IQ_PLL_0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PLL_BANK_IQ_PLL_0_A::PLL_BANK_IQ_PLL_0_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_BANK_IQ_PLL_0_DEFAULT`"]
    #[inline(always)]
    pub fn is_pll_bank_iq_pll_0_default(&self) -> bool {
        *self == PLL_BANK_IQ_PLL_0_A::PLL_BANK_IQ_PLL_0_DEFAULT
    }
}
#[doc = "Write proxy for field `PLL_BANK_IQ_PLL_0`"]
pub struct PLL_BANK_IQ_PLL_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_BANK_IQ_PLL_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_BANK_IQ_PLL_0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_bank_iq_pll_0_default(self) -> &'a mut W {
        self.variant(PLL_BANK_IQ_PLL_0_A::PLL_BANK_IQ_PLL_0_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Sets the minimum power during the PA ramp-up: if 0 the ramp-up starts at -3, if 1 the ramp-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA_PWR_MIN_PA_PWR_A {
    #[doc = "0: `0`"]
    PA_PWR_MIN_PA_PWR_DEFAULT = 0,
}
impl From<PA_PWR_MIN_PA_PWR_A> for bool {
    #[inline(always)]
    fn from(variant: PA_PWR_MIN_PA_PWR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PA_PWR_MIN_PA_PWR`"]
pub type PA_PWR_MIN_PA_PWR_R = crate::R<bool, PA_PWR_MIN_PA_PWR_A>;
impl PA_PWR_MIN_PA_PWR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PA_PWR_MIN_PA_PWR_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PA_PWR_MIN_PA_PWR_A::PA_PWR_MIN_PA_PWR_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA_PWR_MIN_PA_PWR_DEFAULT`"]
    #[inline(always)]
    pub fn is_pa_pwr_min_pa_pwr_default(&self) -> bool {
        *self == PA_PWR_MIN_PA_PWR_A::PA_PWR_MIN_PA_PWR_DEFAULT
    }
}
#[doc = "Write proxy for field `PA_PWR_MIN_PA_PWR`"]
pub struct PA_PWR_MIN_PA_PWR_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_PWR_MIN_PA_PWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PA_PWR_MIN_PA_PWR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pa_pwr_min_pa_pwr_default(self) -> &'a mut W {
        self.variant(PA_PWR_MIN_PA_PWR_A::PA_PWR_MIN_PA_PWR_DEFAULT)
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
#[doc = "Signed value that sets the PA power: minimum value is -3 (-40dBm), max value is 12 (3.3dBm).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PA_PWR_PA_PWR_A {
    #[doc = "0: `0`"]
    PA_PWR_PA_PWR_DEFAULT = 0,
}
impl From<PA_PWR_PA_PWR_A> for u8 {
    #[inline(always)]
    fn from(variant: PA_PWR_PA_PWR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PA_PWR_PA_PWR`"]
pub type PA_PWR_PA_PWR_R = crate::R<u8, PA_PWR_PA_PWR_A>;
impl PA_PWR_PA_PWR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PA_PWR_PA_PWR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PA_PWR_PA_PWR_A::PA_PWR_PA_PWR_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA_PWR_PA_PWR_DEFAULT`"]
    #[inline(always)]
    pub fn is_pa_pwr_pa_pwr_default(&self) -> bool {
        *self == PA_PWR_PA_PWR_A::PA_PWR_PA_PWR_DEFAULT
    }
}
#[doc = "Write proxy for field `PA_PWR_PA_PWR`"]
pub struct PA_PWR_PA_PWR_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_PWR_PA_PWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PA_PWR_PA_PWR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pa_pwr_pa_pwr_default(self) -> &'a mut W {
        self.variant(PA_PWR_PA_PWR_A::PA_PWR_PA_PWR_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Unsigned value that specifies the division factor for the clock controlling the RSSI.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_CH_FILTER_DIV_RSSI_A {
    #[doc = "0: `0`"]
    CLK_CH_FILTER_DIV_RSSI_DEFAULT = 0,
}
impl From<CLK_CH_FILTER_DIV_RSSI_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_CH_FILTER_DIV_RSSI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLK_CH_FILTER_DIV_RSSI`"]
pub type CLK_CH_FILTER_DIV_RSSI_R = crate::R<u8, CLK_CH_FILTER_DIV_RSSI_A>;
impl CLK_CH_FILTER_DIV_RSSI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLK_CH_FILTER_DIV_RSSI_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLK_CH_FILTER_DIV_RSSI_A::CLK_CH_FILTER_DIV_RSSI_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_CH_FILTER_DIV_RSSI_DEFAULT`"]
    #[inline(always)]
    pub fn is_clk_ch_filter_div_rssi_default(&self) -> bool {
        *self == CLK_CH_FILTER_DIV_RSSI_A::CLK_CH_FILTER_DIV_RSSI_DEFAULT
    }
}
#[doc = "Write proxy for field `CLK_CH_FILTER_DIV_RSSI`"]
pub struct CLK_CH_FILTER_DIV_RSSI_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_CH_FILTER_DIV_RSSI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_CH_FILTER_DIV_RSSI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn clk_ch_filter_div_rssi_default(self) -> &'a mut W {
        self.variant(CLK_CH_FILTER_DIV_RSSI_A::CLK_CH_FILTER_DIV_RSSI_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Unsigned value that specifies the division factor for the clock controlling the channel filter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_CH_FILTER_DIV_FILT_A {
    #[doc = "0: `0`"]
    CLK_CH_FILTER_DIV_FILT_DEFAULT = 0,
}
impl From<CLK_CH_FILTER_DIV_FILT_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_CH_FILTER_DIV_FILT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLK_CH_FILTER_DIV_FILT`"]
pub type CLK_CH_FILTER_DIV_FILT_R = crate::R<u8, CLK_CH_FILTER_DIV_FILT_A>;
impl CLK_CH_FILTER_DIV_FILT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLK_CH_FILTER_DIV_FILT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLK_CH_FILTER_DIV_FILT_A::CLK_CH_FILTER_DIV_FILT_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_CH_FILTER_DIV_FILT_DEFAULT`"]
    #[inline(always)]
    pub fn is_clk_ch_filter_div_filt_default(&self) -> bool {
        *self == CLK_CH_FILTER_DIV_FILT_A::CLK_CH_FILTER_DIV_FILT_DEFAULT
    }
}
#[doc = "Write proxy for field `CLK_CH_FILTER_DIV_FILT`"]
pub struct CLK_CH_FILTER_DIV_FILT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_CH_FILTER_DIV_FILT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_CH_FILTER_DIV_FILT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn clk_ch_filter_div_filt_default(self) -> &'a mut W {
        self.variant(CLK_CH_FILTER_DIV_FILT_A::CLK_CH_FILTER_DIV_FILT_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:29 - Same as pll_filter_res_trim but for Tx case. Real value in Tx is pll_filter_res_trim xor pll_filter_res_trim_tx. If set to 0, Tx and Rx have the same value."]
    #[inline(always)]
    pub fn pll_bank_pll_filter_res_trim_tx(&self) -> PLL_BANK_PLL_FILTER_RES_TRIM_TX_R {
        PLL_BANK_PLL_FILTER_RES_TRIM_TX_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - Charge pump bias for Tx case. Real value in Tx is iq_pll_0 xor iq_pll_0_tx. If set to 0, Tx and Rx have the same value."]
    #[inline(always)]
    pub fn pll_bank_iq_pll_0_tx(&self) -> PLL_BANK_IQ_PLL_0_TX_R {
        PLL_BANK_IQ_PLL_0_TX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - If set to 1 the Tx will work in low data-rate mode"]
    #[inline(always)]
    pub fn pll_bank_low_dr_tx(&self) -> PLL_BANK_LOW_DR_TX_R {
        PLL_BANK_LOW_DR_TX_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Allow to modify the value of the loop filter resistor R2 when bit 5 is high (TX mode): 00 => normal resistor (R_2_typ), 01 => 123 percent, 10 => 130 percent 11 => 170 percent"]
    #[inline(always)]
    pub fn pll_bank_pll_filter_res_trim(&self) -> PLL_BANK_PLL_FILTER_RES_TRIM_R {
        PLL_BANK_PLL_FILTER_RES_TRIM_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Charge pump bias"]
    #[inline(always)]
    pub fn pll_bank_iq_pll_0(&self) -> PLL_BANK_IQ_PLL_0_R {
        PLL_BANK_IQ_PLL_0_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Sets the minimum power during the PA ramp-up: if 0 the ramp-up starts at -3, if 1 the ramp-up"]
    #[inline(always)]
    pub fn pa_pwr_min_pa_pwr(&self) -> PA_PWR_MIN_PA_PWR_R {
        PA_PWR_MIN_PA_PWR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Signed value that sets the PA power: minimum value is -3 (-40dBm), max value is 12 (3.3dBm)."]
    #[inline(always)]
    pub fn pa_pwr_pa_pwr(&self) -> PA_PWR_PA_PWR_R {
        PA_PWR_PA_PWR_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 4:7 - Unsigned value that specifies the division factor for the clock controlling the RSSI."]
    #[inline(always)]
    pub fn clk_ch_filter_div_rssi(&self) -> CLK_CH_FILTER_DIV_RSSI_R {
        CLK_CH_FILTER_DIV_RSSI_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Unsigned value that specifies the division factor for the clock controlling the channel filter."]
    #[inline(always)]
    pub fn clk_ch_filter_div_filt(&self) -> CLK_CH_FILTER_DIV_FILT_R {
        CLK_CH_FILTER_DIV_FILT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29 - Same as pll_filter_res_trim but for Tx case. Real value in Tx is pll_filter_res_trim xor pll_filter_res_trim_tx. If set to 0, Tx and Rx have the same value."]
    #[inline(always)]
    pub fn pll_bank_pll_filter_res_trim_tx(&mut self) -> PLL_BANK_PLL_FILTER_RES_TRIM_TX_W {
        PLL_BANK_PLL_FILTER_RES_TRIM_TX_W { w: self }
    }
    #[doc = "Bits 24:27 - Charge pump bias for Tx case. Real value in Tx is iq_pll_0 xor iq_pll_0_tx. If set to 0, Tx and Rx have the same value."]
    #[inline(always)]
    pub fn pll_bank_iq_pll_0_tx(&mut self) -> PLL_BANK_IQ_PLL_0_TX_W {
        PLL_BANK_IQ_PLL_0_TX_W { w: self }
    }
    #[doc = "Bit 22 - If set to 1 the Tx will work in low data-rate mode"]
    #[inline(always)]
    pub fn pll_bank_low_dr_tx(&mut self) -> PLL_BANK_LOW_DR_TX_W {
        PLL_BANK_LOW_DR_TX_W { w: self }
    }
    #[doc = "Bits 20:21 - Allow to modify the value of the loop filter resistor R2 when bit 5 is high (TX mode): 00 => normal resistor (R_2_typ), 01 => 123 percent, 10 => 130 percent 11 => 170 percent"]
    #[inline(always)]
    pub fn pll_bank_pll_filter_res_trim(&mut self) -> PLL_BANK_PLL_FILTER_RES_TRIM_W {
        PLL_BANK_PLL_FILTER_RES_TRIM_W { w: self }
    }
    #[doc = "Bits 16:19 - Charge pump bias"]
    #[inline(always)]
    pub fn pll_bank_iq_pll_0(&mut self) -> PLL_BANK_IQ_PLL_0_W {
        PLL_BANK_IQ_PLL_0_W { w: self }
    }
    #[doc = "Bit 13 - Sets the minimum power during the PA ramp-up: if 0 the ramp-up starts at -3, if 1 the ramp-up"]
    #[inline(always)]
    pub fn pa_pwr_min_pa_pwr(&mut self) -> PA_PWR_MIN_PA_PWR_W {
        PA_PWR_MIN_PA_PWR_W { w: self }
    }
    #[doc = "Bits 8:12 - Signed value that sets the PA power: minimum value is -3 (-40dBm), max value is 12 (3.3dBm)."]
    #[inline(always)]
    pub fn pa_pwr_pa_pwr(&mut self) -> PA_PWR_PA_PWR_W {
        PA_PWR_PA_PWR_W { w: self }
    }
    #[doc = "Bits 4:7 - Unsigned value that specifies the division factor for the clock controlling the RSSI."]
    #[inline(always)]
    pub fn clk_ch_filter_div_rssi(&mut self) -> CLK_CH_FILTER_DIV_RSSI_W {
        CLK_CH_FILTER_DIV_RSSI_W { w: self }
    }
    #[doc = "Bits 0:3 - Unsigned value that specifies the division factor for the clock controlling the channel filter."]
    #[inline(always)]
    pub fn clk_ch_filter_div_filt(&mut self) -> CLK_CH_FILTER_DIV_FILT_W {
        CLK_CH_FILTER_DIV_FILT_W { w: self }
    }
}
