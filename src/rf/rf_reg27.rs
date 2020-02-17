#[doc = "Reader of register RF_REG27"]
pub type R = crate::R<u32, super::RF_REG27>;
#[doc = "Writer for register RF_REG27"]
pub type W = crate::W<u32, super::RF_REG27>;
#[doc = "Register RF_REG27 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG27 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "If set to 1, the RSSI and the phADC share the same clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRL_ADC_ONE_CK_RSSI_PHADC_A {
    #[doc = "0: `0`"]
    CTRL_ADC_ONE_CK_RSSI_PHADC_DEFAULT = 0,
}
impl From<CTRL_ADC_ONE_CK_RSSI_PHADC_A> for bool {
    #[inline(always)]
    fn from(variant: CTRL_ADC_ONE_CK_RSSI_PHADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTRL_ADC_ONE_CK_RSSI_PHADC`"]
pub type CTRL_ADC_ONE_CK_RSSI_PHADC_R = crate::R<bool, CTRL_ADC_ONE_CK_RSSI_PHADC_A>;
impl CTRL_ADC_ONE_CK_RSSI_PHADC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CTRL_ADC_ONE_CK_RSSI_PHADC_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CTRL_ADC_ONE_CK_RSSI_PHADC_A::CTRL_ADC_ONE_CK_RSSI_PHADC_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CTRL_ADC_ONE_CK_RSSI_PHADC_DEFAULT`"]
    #[inline(always)]
    pub fn is_ctrl_adc_one_ck_rssi_phadc_default(&self) -> bool {
        *self == CTRL_ADC_ONE_CK_RSSI_PHADC_A::CTRL_ADC_ONE_CK_RSSI_PHADC_DEFAULT
    }
}
#[doc = "Write proxy for field `CTRL_ADC_ONE_CK_RSSI_PHADC`"]
pub struct CTRL_ADC_ONE_CK_RSSI_PHADC_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_ADC_ONE_CK_RSSI_PHADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_ADC_ONE_CK_RSSI_PHADC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ctrl_adc_one_ck_rssi_phadc_default(self) -> &'a mut W {
        self.variant(CTRL_ADC_ONE_CK_RSSI_PHADC_A::CTRL_ADC_ONE_CK_RSSI_PHADC_DEFAULT)
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
#[doc = "phADC delay latch trimming\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTRL_ADC_PHADC_DELLATCH_A {
    #[doc = "0: `0`"]
    CTRL_ADC_PHADC_DELLATCH_DEFAULT = 0,
}
impl From<CTRL_ADC_PHADC_DELLATCH_A> for u8 {
    #[inline(always)]
    fn from(variant: CTRL_ADC_PHADC_DELLATCH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CTRL_ADC_PHADC_DELLATCH`"]
pub type CTRL_ADC_PHADC_DELLATCH_R = crate::R<u8, CTRL_ADC_PHADC_DELLATCH_A>;
impl CTRL_ADC_PHADC_DELLATCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CTRL_ADC_PHADC_DELLATCH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CTRL_ADC_PHADC_DELLATCH_A::CTRL_ADC_PHADC_DELLATCH_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CTRL_ADC_PHADC_DELLATCH_DEFAULT`"]
    #[inline(always)]
    pub fn is_ctrl_adc_phadc_dellatch_default(&self) -> bool {
        *self == CTRL_ADC_PHADC_DELLATCH_A::CTRL_ADC_PHADC_DELLATCH_DEFAULT
    }
}
#[doc = "Write proxy for field `CTRL_ADC_PHADC_DELLATCH`"]
pub struct CTRL_ADC_PHADC_DELLATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_ADC_PHADC_DELLATCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_ADC_PHADC_DELLATCH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ctrl_adc_phadc_dellatch_default(self) -> &'a mut W {
        self.variant(CTRL_ADC_PHADC_DELLATCH_A::CTRL_ADC_PHADC_DELLATCH_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "bits(1:0) => phADC reset delay, bits(3:2) phADC clock delay, bit(4) phADC latch idle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTRL_ADC_CTRL_ADC_A {
    #[doc = "0: `0`"]
    CTRL_ADC_CTRL_ADC_DEFAULT = 0,
}
impl From<CTRL_ADC_CTRL_ADC_A> for u8 {
    #[inline(always)]
    fn from(variant: CTRL_ADC_CTRL_ADC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CTRL_ADC_CTRL_ADC`"]
pub type CTRL_ADC_CTRL_ADC_R = crate::R<u8, CTRL_ADC_CTRL_ADC_A>;
impl CTRL_ADC_CTRL_ADC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CTRL_ADC_CTRL_ADC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CTRL_ADC_CTRL_ADC_A::CTRL_ADC_CTRL_ADC_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CTRL_ADC_CTRL_ADC_DEFAULT`"]
    #[inline(always)]
    pub fn is_ctrl_adc_ctrl_adc_default(&self) -> bool {
        *self == CTRL_ADC_CTRL_ADC_A::CTRL_ADC_CTRL_ADC_DEFAULT
    }
}
#[doc = "Write proxy for field `CTRL_ADC_CTRL_ADC`"]
pub struct CTRL_ADC_CTRL_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_ADC_CTRL_ADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_ADC_CTRL_ADC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ctrl_adc_ctrl_adc_default(self) -> &'a mut W {
        self.variant(CTRL_ADC_CTRL_ADC_A::CTRL_ADC_CTRL_ADC_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Enable PTAT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIAS_EN_2_EN_PTAT_A {
    #[doc = "0: `0`"]
    BIAS_EN_2_EN_PTAT_DEFAULT = 0,
}
impl From<BIAS_EN_2_EN_PTAT_A> for bool {
    #[inline(always)]
    fn from(variant: BIAS_EN_2_EN_PTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BIAS_EN_2_EN_PTAT`"]
pub type BIAS_EN_2_EN_PTAT_R = crate::R<bool, BIAS_EN_2_EN_PTAT_A>;
impl BIAS_EN_2_EN_PTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, BIAS_EN_2_EN_PTAT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(BIAS_EN_2_EN_PTAT_A::BIAS_EN_2_EN_PTAT_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_EN_2_EN_PTAT_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_en_2_en_ptat_default(&self) -> bool {
        *self == BIAS_EN_2_EN_PTAT_A::BIAS_EN_2_EN_PTAT_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_EN_2_EN_PTAT`"]
pub struct BIAS_EN_2_EN_PTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_EN_2_EN_PTAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_EN_2_EN_PTAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_en_2_en_ptat_default(self) -> &'a mut W {
        self.variant(BIAS_EN_2_EN_PTAT_A::BIAS_EN_2_EN_PTAT_DEFAULT)
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
#[doc = "Bias enable for BB (same order as biases)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_EN_2_EN_BIAS_BB_HI_A {
    #[doc = "0: `0`"]
    BIAS_EN_2_EN_BIAS_BB_HI_DEFAULT = 0,
}
impl From<BIAS_EN_2_EN_BIAS_BB_HI_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_EN_2_EN_BIAS_BB_HI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_EN_2_EN_BIAS_BB_HI`"]
pub type BIAS_EN_2_EN_BIAS_BB_HI_R = crate::R<u8, BIAS_EN_2_EN_BIAS_BB_HI_A>;
impl BIAS_EN_2_EN_BIAS_BB_HI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_EN_2_EN_BIAS_BB_HI_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_EN_2_EN_BIAS_BB_HI_A::BIAS_EN_2_EN_BIAS_BB_HI_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_EN_2_EN_BIAS_BB_HI_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_en_2_en_bias_bb_hi_default(&self) -> bool {
        *self == BIAS_EN_2_EN_BIAS_BB_HI_A::BIAS_EN_2_EN_BIAS_BB_HI_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_EN_2_EN_BIAS_BB_HI`"]
pub struct BIAS_EN_2_EN_BIAS_BB_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_EN_2_EN_BIAS_BB_HI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_EN_2_EN_BIAS_BB_HI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_en_2_en_bias_bb_hi_default(self) -> &'a mut W {
        self.variant(BIAS_EN_2_EN_BIAS_BB_HI_A::BIAS_EN_2_EN_BIAS_BB_HI_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Bias enable for BB (same order as biases)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_EN_1_EN_BIAS_BB_LO_A {
    #[doc = "0: `0`"]
    BIAS_EN_1_EN_BIAS_BB_LO_DEFAULT = 0,
}
impl From<BIAS_EN_1_EN_BIAS_BB_LO_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_EN_1_EN_BIAS_BB_LO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_EN_1_EN_BIAS_BB_LO`"]
pub type BIAS_EN_1_EN_BIAS_BB_LO_R = crate::R<u8, BIAS_EN_1_EN_BIAS_BB_LO_A>;
impl BIAS_EN_1_EN_BIAS_BB_LO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_EN_1_EN_BIAS_BB_LO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_EN_1_EN_BIAS_BB_LO_A::BIAS_EN_1_EN_BIAS_BB_LO_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_EN_1_EN_BIAS_BB_LO_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_en_1_en_bias_bb_lo_default(&self) -> bool {
        *self == BIAS_EN_1_EN_BIAS_BB_LO_A::BIAS_EN_1_EN_BIAS_BB_LO_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_EN_1_EN_BIAS_BB_LO`"]
pub struct BIAS_EN_1_EN_BIAS_BB_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_EN_1_EN_BIAS_BB_LO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_EN_1_EN_BIAS_BB_LO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_en_1_en_bias_bb_lo_default(self) -> &'a mut W {
        self.variant(BIAS_EN_1_EN_BIAS_BB_LO_A::BIAS_EN_1_EN_BIAS_BB_LO_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Bias enable for PLL (same order as biases)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_EN_1_EN_BIAS_PLL_A {
    #[doc = "0: `0`"]
    BIAS_EN_1_EN_BIAS_PLL_DEFAULT = 0,
}
impl From<BIAS_EN_1_EN_BIAS_PLL_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_EN_1_EN_BIAS_PLL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_EN_1_EN_BIAS_PLL`"]
pub type BIAS_EN_1_EN_BIAS_PLL_R = crate::R<u8, BIAS_EN_1_EN_BIAS_PLL_A>;
impl BIAS_EN_1_EN_BIAS_PLL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_EN_1_EN_BIAS_PLL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_EN_1_EN_BIAS_PLL_A::BIAS_EN_1_EN_BIAS_PLL_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_EN_1_EN_BIAS_PLL_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_en_1_en_bias_pll_default(&self) -> bool {
        *self == BIAS_EN_1_EN_BIAS_PLL_A::BIAS_EN_1_EN_BIAS_PLL_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_EN_1_EN_BIAS_PLL`"]
pub struct BIAS_EN_1_EN_BIAS_PLL_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_EN_1_EN_BIAS_PLL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_EN_1_EN_BIAS_PLL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_en_1_en_bias_pll_default(self) -> &'a mut W {
        self.variant(BIAS_EN_1_EN_BIAS_PLL_A::BIAS_EN_1_EN_BIAS_PLL_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 7)) | (((value as u32) & 0x1f) << 7);
        self.w
    }
}
#[doc = "Bias enable for RxTx (same order as biases)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_EN_1_EN_BIAS_RXTX_A {
    #[doc = "0: `0`"]
    BIAS_EN_1_EN_BIAS_RXTX_DEFAULT = 0,
}
impl From<BIAS_EN_1_EN_BIAS_RXTX_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_EN_1_EN_BIAS_RXTX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_EN_1_EN_BIAS_RXTX`"]
pub type BIAS_EN_1_EN_BIAS_RXTX_R = crate::R<u8, BIAS_EN_1_EN_BIAS_RXTX_A>;
impl BIAS_EN_1_EN_BIAS_RXTX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_EN_1_EN_BIAS_RXTX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_EN_1_EN_BIAS_RXTX_A::BIAS_EN_1_EN_BIAS_RXTX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_EN_1_EN_BIAS_RXTX_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_en_1_en_bias_rxtx_default(&self) -> bool {
        *self == BIAS_EN_1_EN_BIAS_RXTX_A::BIAS_EN_1_EN_BIAS_RXTX_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_EN_1_EN_BIAS_RXTX`"]
pub struct BIAS_EN_1_EN_BIAS_RXTX_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_EN_1_EN_BIAS_RXTX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_EN_1_EN_BIAS_RXTX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_en_1_en_bias_rxtx_default(self) -> &'a mut W {
        self.variant(BIAS_EN_1_EN_BIAS_RXTX_A::BIAS_EN_1_EN_BIAS_RXTX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - If set to 1, the RSSI and the phADC share the same clock"]
    #[inline(always)]
    pub fn ctrl_adc_one_ck_rssi_phadc(&self) -> CTRL_ADC_ONE_CK_RSSI_PHADC_R {
        CTRL_ADC_ONE_CK_RSSI_PHADC_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - phADC delay latch trimming"]
    #[inline(always)]
    pub fn ctrl_adc_phadc_dellatch(&self) -> CTRL_ADC_PHADC_DELLATCH_R {
        CTRL_ADC_PHADC_DELLATCH_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bits 24:28 - bits(1:0) => phADC reset delay, bits(3:2) phADC clock delay, bit(4) phADC latch idle"]
    #[inline(always)]
    pub fn ctrl_adc_ctrl_adc(&self) -> CTRL_ADC_CTRL_ADC_R {
        CTRL_ADC_CTRL_ADC_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 19 - Enable PTAT"]
    #[inline(always)]
    pub fn bias_en_2_en_ptat(&self) -> BIAS_EN_2_EN_PTAT_R {
        BIAS_EN_2_EN_PTAT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Bias enable for BB (same order as biases)"]
    #[inline(always)]
    pub fn bias_en_2_en_bias_bb_hi(&self) -> BIAS_EN_2_EN_BIAS_BB_HI_R {
        BIAS_EN_2_EN_BIAS_BB_HI_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:15 - Bias enable for BB (same order as biases)"]
    #[inline(always)]
    pub fn bias_en_1_en_bias_bb_lo(&self) -> BIAS_EN_1_EN_BIAS_BB_LO_R {
        BIAS_EN_1_EN_BIAS_BB_LO_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 7:11 - Bias enable for PLL (same order as biases)"]
    #[inline(always)]
    pub fn bias_en_1_en_bias_pll(&self) -> BIAS_EN_1_EN_BIAS_PLL_R {
        BIAS_EN_1_EN_BIAS_PLL_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 0:6 - Bias enable for RxTx (same order as biases)"]
    #[inline(always)]
    pub fn bias_en_1_en_bias_rxtx(&self) -> BIAS_EN_1_EN_BIAS_RXTX_R {
        BIAS_EN_1_EN_BIAS_RXTX_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - If set to 1, the RSSI and the phADC share the same clock"]
    #[inline(always)]
    pub fn ctrl_adc_one_ck_rssi_phadc(&mut self) -> CTRL_ADC_ONE_CK_RSSI_PHADC_W {
        CTRL_ADC_ONE_CK_RSSI_PHADC_W { w: self }
    }
    #[doc = "Bits 29:30 - phADC delay latch trimming"]
    #[inline(always)]
    pub fn ctrl_adc_phadc_dellatch(&mut self) -> CTRL_ADC_PHADC_DELLATCH_W {
        CTRL_ADC_PHADC_DELLATCH_W { w: self }
    }
    #[doc = "Bits 24:28 - bits(1:0) => phADC reset delay, bits(3:2) phADC clock delay, bit(4) phADC latch idle"]
    #[inline(always)]
    pub fn ctrl_adc_ctrl_adc(&mut self) -> CTRL_ADC_CTRL_ADC_W {
        CTRL_ADC_CTRL_ADC_W { w: self }
    }
    #[doc = "Bit 19 - Enable PTAT"]
    #[inline(always)]
    pub fn bias_en_2_en_ptat(&mut self) -> BIAS_EN_2_EN_PTAT_W {
        BIAS_EN_2_EN_PTAT_W { w: self }
    }
    #[doc = "Bits 16:18 - Bias enable for BB (same order as biases)"]
    #[inline(always)]
    pub fn bias_en_2_en_bias_bb_hi(&mut self) -> BIAS_EN_2_EN_BIAS_BB_HI_W {
        BIAS_EN_2_EN_BIAS_BB_HI_W { w: self }
    }
    #[doc = "Bits 12:15 - Bias enable for BB (same order as biases)"]
    #[inline(always)]
    pub fn bias_en_1_en_bias_bb_lo(&mut self) -> BIAS_EN_1_EN_BIAS_BB_LO_W {
        BIAS_EN_1_EN_BIAS_BB_LO_W { w: self }
    }
    #[doc = "Bits 7:11 - Bias enable for PLL (same order as biases)"]
    #[inline(always)]
    pub fn bias_en_1_en_bias_pll(&mut self) -> BIAS_EN_1_EN_BIAS_PLL_W {
        BIAS_EN_1_EN_BIAS_PLL_W { w: self }
    }
    #[doc = "Bits 0:6 - Bias enable for RxTx (same order as biases)"]
    #[inline(always)]
    pub fn bias_en_1_en_bias_rxtx(&mut self) -> BIAS_EN_1_EN_BIAS_RXTX_W {
        BIAS_EN_1_EN_BIAS_RXTX_W { w: self }
    }
}
