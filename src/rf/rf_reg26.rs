#[doc = "Reader of register RF_REG26"]
pub type R = crate::R<u32, super::RF_REG26>;
#[doc = "Writer for register RF_REG26"]
pub type W = crate::W<u32, super::RF_REG26>;
#[doc = "Register RF_REG26 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG26 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable the sigma delta mash\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SD_MASH_MASH_ENABLE_A {
    #[doc = "0: `0`"]
    SD_MASH_MASH_ENABLE_DEFAULT = 0,
}
impl From<SD_MASH_MASH_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: SD_MASH_MASH_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SD_MASH_MASH_ENABLE`"]
pub type SD_MASH_MASH_ENABLE_R = crate::R<bool, SD_MASH_MASH_ENABLE_A>;
impl SD_MASH_MASH_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SD_MASH_MASH_ENABLE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(SD_MASH_MASH_ENABLE_A::SD_MASH_MASH_ENABLE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_MASH_MASH_ENABLE_DEFAULT`"]
    #[inline(always)]
    pub fn is_sd_mash_mash_enable_default(&self) -> bool {
        *self == SD_MASH_MASH_ENABLE_A::SD_MASH_MASH_ENABLE_DEFAULT
    }
}
#[doc = "Write proxy for field `SD_MASH_MASH_ENABLE`"]
pub struct SD_MASH_MASH_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_MASH_MASH_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SD_MASH_MASH_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sd_mash_mash_enable_default(self) -> &'a mut W {
        self.variant(SD_MASH_MASH_ENABLE_A::SD_MASH_MASH_ENABLE_DEFAULT)
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
#[doc = "Enable dithering on the sigma delta mash\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SD_MASH_MASH_DITHER_A {
    #[doc = "0: `0`"]
    SD_MASH_MASH_DITHER_DEFAULT = 0,
}
impl From<SD_MASH_MASH_DITHER_A> for bool {
    #[inline(always)]
    fn from(variant: SD_MASH_MASH_DITHER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SD_MASH_MASH_DITHER`"]
pub type SD_MASH_MASH_DITHER_R = crate::R<bool, SD_MASH_MASH_DITHER_A>;
impl SD_MASH_MASH_DITHER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SD_MASH_MASH_DITHER_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(SD_MASH_MASH_DITHER_A::SD_MASH_MASH_DITHER_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_MASH_MASH_DITHER_DEFAULT`"]
    #[inline(always)]
    pub fn is_sd_mash_mash_dither_default(&self) -> bool {
        *self == SD_MASH_MASH_DITHER_A::SD_MASH_MASH_DITHER_DEFAULT
    }
}
#[doc = "Write proxy for field `SD_MASH_MASH_DITHER`"]
pub struct SD_MASH_MASH_DITHER_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_MASH_MASH_DITHER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SD_MASH_MASH_DITHER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sd_mash_mash_dither_default(self) -> &'a mut W {
        self.variant(SD_MASH_MASH_DITHER_A::SD_MASH_MASH_DITHER_DEFAULT)
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
#[doc = "Order of the sigma delta mash\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SD_MASH_MASH_ORDER_A {
    #[doc = "0: `0`"]
    SD_MASH_MASH_ORDER_DEFAULT = 0,
}
impl From<SD_MASH_MASH_ORDER_A> for u8 {
    #[inline(always)]
    fn from(variant: SD_MASH_MASH_ORDER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SD_MASH_MASH_ORDER`"]
pub type SD_MASH_MASH_ORDER_R = crate::R<u8, SD_MASH_MASH_ORDER_A>;
impl SD_MASH_MASH_ORDER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SD_MASH_MASH_ORDER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SD_MASH_MASH_ORDER_A::SD_MASH_MASH_ORDER_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_MASH_MASH_ORDER_DEFAULT`"]
    #[inline(always)]
    pub fn is_sd_mash_mash_order_default(&self) -> bool {
        *self == SD_MASH_MASH_ORDER_A::SD_MASH_MASH_ORDER_DEFAULT
    }
}
#[doc = "Write proxy for field `SD_MASH_MASH_ORDER`"]
pub struct SD_MASH_MASH_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_MASH_MASH_ORDER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SD_MASH_MASH_ORDER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sd_mash_mash_order_default(self) -> &'a mut W {
        self.variant(SD_MASH_MASH_ORDER_A::SD_MASH_MASH_ORDER_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Reset of the sigma delta mash (active low)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SD_MASH_MASH_RSTB_A {
    #[doc = "0: `0`"]
    SD_MASH_MASH_RSTB_DEFAULT = 0,
}
impl From<SD_MASH_MASH_RSTB_A> for bool {
    #[inline(always)]
    fn from(variant: SD_MASH_MASH_RSTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SD_MASH_MASH_RSTB`"]
pub type SD_MASH_MASH_RSTB_R = crate::R<bool, SD_MASH_MASH_RSTB_A>;
impl SD_MASH_MASH_RSTB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SD_MASH_MASH_RSTB_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(SD_MASH_MASH_RSTB_A::SD_MASH_MASH_RSTB_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_MASH_MASH_RSTB_DEFAULT`"]
    #[inline(always)]
    pub fn is_sd_mash_mash_rstb_default(&self) -> bool {
        *self == SD_MASH_MASH_RSTB_A::SD_MASH_MASH_RSTB_DEFAULT
    }
}
#[doc = "Write proxy for field `SD_MASH_MASH_RSTB`"]
pub struct SD_MASH_MASH_RSTB_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_MASH_MASH_RSTB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SD_MASH_MASH_RSTB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sd_mash_mash_rstb_default(self) -> &'a mut W {
        self.variant(SD_MASH_MASH_RSTB_A::SD_MASH_MASH_RSTB_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "LNA bias for AGC lvl 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_12_LNA_AGC_BIAS_3_A {
    #[doc = "0: `0`"]
    BIAS_12_LNA_AGC_BIAS_3_DEFAULT = 0,
}
impl From<BIAS_12_LNA_AGC_BIAS_3_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_12_LNA_AGC_BIAS_3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_12_LNA_AGC_BIAS_3`"]
pub type BIAS_12_LNA_AGC_BIAS_3_R = crate::R<u8, BIAS_12_LNA_AGC_BIAS_3_A>;
impl BIAS_12_LNA_AGC_BIAS_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_12_LNA_AGC_BIAS_3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_12_LNA_AGC_BIAS_3_A::BIAS_12_LNA_AGC_BIAS_3_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_12_LNA_AGC_BIAS_3_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_12_lna_agc_bias_3_default(&self) -> bool {
        *self == BIAS_12_LNA_AGC_BIAS_3_A::BIAS_12_LNA_AGC_BIAS_3_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_12_LNA_AGC_BIAS_3`"]
pub struct BIAS_12_LNA_AGC_BIAS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_12_LNA_AGC_BIAS_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_12_LNA_AGC_BIAS_3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_12_lna_agc_bias_3_default(self) -> &'a mut W {
        self.variant(BIAS_12_LNA_AGC_BIAS_3_A::BIAS_12_LNA_AGC_BIAS_3_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "LNA bias for AGC lvl 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_12_LNA_AGC_BIAS_2_A {
    #[doc = "0: `0`"]
    BIAS_12_LNA_AGC_BIAS_2_DEFAULT = 0,
}
impl From<BIAS_12_LNA_AGC_BIAS_2_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_12_LNA_AGC_BIAS_2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_12_LNA_AGC_BIAS_2`"]
pub type BIAS_12_LNA_AGC_BIAS_2_R = crate::R<u8, BIAS_12_LNA_AGC_BIAS_2_A>;
impl BIAS_12_LNA_AGC_BIAS_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_12_LNA_AGC_BIAS_2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_12_LNA_AGC_BIAS_2_A::BIAS_12_LNA_AGC_BIAS_2_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_12_LNA_AGC_BIAS_2_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_12_lna_agc_bias_2_default(&self) -> bool {
        *self == BIAS_12_LNA_AGC_BIAS_2_A::BIAS_12_LNA_AGC_BIAS_2_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_12_LNA_AGC_BIAS_2`"]
pub struct BIAS_12_LNA_AGC_BIAS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_12_LNA_AGC_BIAS_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_12_LNA_AGC_BIAS_2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_12_lna_agc_bias_2_default(self) -> &'a mut W {
        self.variant(BIAS_12_LNA_AGC_BIAS_2_A::BIAS_12_LNA_AGC_BIAS_2_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "LNA bias for AGC lvl 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_11_LNA_AGC_BIAS_1_A {
    #[doc = "0: `0`"]
    BIAS_11_LNA_AGC_BIAS_1_DEFAULT = 0,
}
impl From<BIAS_11_LNA_AGC_BIAS_1_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_11_LNA_AGC_BIAS_1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_11_LNA_AGC_BIAS_1`"]
pub type BIAS_11_LNA_AGC_BIAS_1_R = crate::R<u8, BIAS_11_LNA_AGC_BIAS_1_A>;
impl BIAS_11_LNA_AGC_BIAS_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_11_LNA_AGC_BIAS_1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_11_LNA_AGC_BIAS_1_A::BIAS_11_LNA_AGC_BIAS_1_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_11_LNA_AGC_BIAS_1_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_11_lna_agc_bias_1_default(&self) -> bool {
        *self == BIAS_11_LNA_AGC_BIAS_1_A::BIAS_11_LNA_AGC_BIAS_1_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_11_LNA_AGC_BIAS_1`"]
pub struct BIAS_11_LNA_AGC_BIAS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_11_LNA_AGC_BIAS_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_11_LNA_AGC_BIAS_1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_11_lna_agc_bias_1_default(self) -> &'a mut W {
        self.variant(BIAS_11_LNA_AGC_BIAS_1_A::BIAS_11_LNA_AGC_BIAS_1_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "LNA bias for AGC lvl 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_11_LNA_AGC_BIAS_0_A {
    #[doc = "0: `0`"]
    BIAS_11_LNA_AGC_BIAS_0_DEFAULT = 0,
}
impl From<BIAS_11_LNA_AGC_BIAS_0_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_11_LNA_AGC_BIAS_0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_11_LNA_AGC_BIAS_0`"]
pub type BIAS_11_LNA_AGC_BIAS_0_R = crate::R<u8, BIAS_11_LNA_AGC_BIAS_0_A>;
impl BIAS_11_LNA_AGC_BIAS_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_11_LNA_AGC_BIAS_0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_11_LNA_AGC_BIAS_0_A::BIAS_11_LNA_AGC_BIAS_0_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_11_LNA_AGC_BIAS_0_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_11_lna_agc_bias_0_default(&self) -> bool {
        *self == BIAS_11_LNA_AGC_BIAS_0_A::BIAS_11_LNA_AGC_BIAS_0_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_11_LNA_AGC_BIAS_0`"]
pub struct BIAS_11_LNA_AGC_BIAS_0_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_11_LNA_AGC_BIAS_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_11_LNA_AGC_BIAS_0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_11_lna_agc_bias_0_default(self) -> &'a mut W {
        self.variant(BIAS_11_LNA_AGC_BIAS_0_A::BIAS_11_LNA_AGC_BIAS_0_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Peak detector threshold bias 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_10_IQ_BB_8_A {
    #[doc = "0: `0`"]
    BIAS_10_IQ_BB_8_DEFAULT = 0,
}
impl From<BIAS_10_IQ_BB_8_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_10_IQ_BB_8_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_10_IQ_BB_8`"]
pub type BIAS_10_IQ_BB_8_R = crate::R<u8, BIAS_10_IQ_BB_8_A>;
impl BIAS_10_IQ_BB_8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_10_IQ_BB_8_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_10_IQ_BB_8_A::BIAS_10_IQ_BB_8_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_10_IQ_BB_8_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_10_iq_bb_8_default(&self) -> bool {
        *self == BIAS_10_IQ_BB_8_A::BIAS_10_IQ_BB_8_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_10_IQ_BB_8`"]
pub struct BIAS_10_IQ_BB_8_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_10_IQ_BB_8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_10_IQ_BB_8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_10_iq_bb_8_default(self) -> &'a mut W {
        self.variant(BIAS_10_IQ_BB_8_A::BIAS_10_IQ_BB_8_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Peak detector threshold bias 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_10_IQ_BB_7_A {
    #[doc = "0: `0`"]
    BIAS_10_IQ_BB_7_DEFAULT = 0,
}
impl From<BIAS_10_IQ_BB_7_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_10_IQ_BB_7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_10_IQ_BB_7`"]
pub type BIAS_10_IQ_BB_7_R = crate::R<u8, BIAS_10_IQ_BB_7_A>;
impl BIAS_10_IQ_BB_7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_10_IQ_BB_7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_10_IQ_BB_7_A::BIAS_10_IQ_BB_7_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_10_IQ_BB_7_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_10_iq_bb_7_default(&self) -> bool {
        *self == BIAS_10_IQ_BB_7_A::BIAS_10_IQ_BB_7_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_10_IQ_BB_7`"]
pub struct BIAS_10_IQ_BB_7_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_10_IQ_BB_7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_10_IQ_BB_7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_10_iq_bb_7_default(self) -> &'a mut W {
        self.variant(BIAS_10_IQ_BB_7_A::BIAS_10_IQ_BB_7_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 28 - Enable the sigma delta mash"]
    #[inline(always)]
    pub fn sd_mash_mash_enable(&self) -> SD_MASH_MASH_ENABLE_R {
        SD_MASH_MASH_ENABLE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable dithering on the sigma delta mash"]
    #[inline(always)]
    pub fn sd_mash_mash_dither(&self) -> SD_MASH_MASH_DITHER_R {
        SD_MASH_MASH_DITHER_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - Order of the sigma delta mash"]
    #[inline(always)]
    pub fn sd_mash_mash_order(&self) -> SD_MASH_MASH_ORDER_R {
        SD_MASH_MASH_ORDER_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Reset of the sigma delta mash (active low)"]
    #[inline(always)]
    pub fn sd_mash_mash_rstb(&self) -> SD_MASH_MASH_RSTB_R {
        SD_MASH_MASH_RSTB_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - LNA bias for AGC lvl 3"]
    #[inline(always)]
    pub fn bias_12_lna_agc_bias_3(&self) -> BIAS_12_LNA_AGC_BIAS_3_R {
        BIAS_12_LNA_AGC_BIAS_3_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - LNA bias for AGC lvl 2"]
    #[inline(always)]
    pub fn bias_12_lna_agc_bias_2(&self) -> BIAS_12_LNA_AGC_BIAS_2_R {
        BIAS_12_LNA_AGC_BIAS_2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - LNA bias for AGC lvl 1"]
    #[inline(always)]
    pub fn bias_11_lna_agc_bias_1(&self) -> BIAS_11_LNA_AGC_BIAS_1_R {
        BIAS_11_LNA_AGC_BIAS_1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - LNA bias for AGC lvl 0"]
    #[inline(always)]
    pub fn bias_11_lna_agc_bias_0(&self) -> BIAS_11_LNA_AGC_BIAS_0_R {
        BIAS_11_LNA_AGC_BIAS_0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Peak detector threshold bias 1"]
    #[inline(always)]
    pub fn bias_10_iq_bb_8(&self) -> BIAS_10_IQ_BB_8_R {
        BIAS_10_IQ_BB_8_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Peak detector threshold bias 2"]
    #[inline(always)]
    pub fn bias_10_iq_bb_7(&self) -> BIAS_10_IQ_BB_7_R {
        BIAS_10_IQ_BB_7_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 28 - Enable the sigma delta mash"]
    #[inline(always)]
    pub fn sd_mash_mash_enable(&mut self) -> SD_MASH_MASH_ENABLE_W {
        SD_MASH_MASH_ENABLE_W { w: self }
    }
    #[doc = "Bit 27 - Enable dithering on the sigma delta mash"]
    #[inline(always)]
    pub fn sd_mash_mash_dither(&mut self) -> SD_MASH_MASH_DITHER_W {
        SD_MASH_MASH_DITHER_W { w: self }
    }
    #[doc = "Bits 25:26 - Order of the sigma delta mash"]
    #[inline(always)]
    pub fn sd_mash_mash_order(&mut self) -> SD_MASH_MASH_ORDER_W {
        SD_MASH_MASH_ORDER_W { w: self }
    }
    #[doc = "Bit 24 - Reset of the sigma delta mash (active low)"]
    #[inline(always)]
    pub fn sd_mash_mash_rstb(&mut self) -> SD_MASH_MASH_RSTB_W {
        SD_MASH_MASH_RSTB_W { w: self }
    }
    #[doc = "Bits 20:23 - LNA bias for AGC lvl 3"]
    #[inline(always)]
    pub fn bias_12_lna_agc_bias_3(&mut self) -> BIAS_12_LNA_AGC_BIAS_3_W {
        BIAS_12_LNA_AGC_BIAS_3_W { w: self }
    }
    #[doc = "Bits 16:19 - LNA bias for AGC lvl 2"]
    #[inline(always)]
    pub fn bias_12_lna_agc_bias_2(&mut self) -> BIAS_12_LNA_AGC_BIAS_2_W {
        BIAS_12_LNA_AGC_BIAS_2_W { w: self }
    }
    #[doc = "Bits 12:15 - LNA bias for AGC lvl 1"]
    #[inline(always)]
    pub fn bias_11_lna_agc_bias_1(&mut self) -> BIAS_11_LNA_AGC_BIAS_1_W {
        BIAS_11_LNA_AGC_BIAS_1_W { w: self }
    }
    #[doc = "Bits 8:11 - LNA bias for AGC lvl 0"]
    #[inline(always)]
    pub fn bias_11_lna_agc_bias_0(&mut self) -> BIAS_11_LNA_AGC_BIAS_0_W {
        BIAS_11_LNA_AGC_BIAS_0_W { w: self }
    }
    #[doc = "Bits 4:7 - Peak detector threshold bias 1"]
    #[inline(always)]
    pub fn bias_10_iq_bb_8(&mut self) -> BIAS_10_IQ_BB_8_W {
        BIAS_10_IQ_BB_8_W { w: self }
    }
    #[doc = "Bits 0:3 - Peak detector threshold bias 2"]
    #[inline(always)]
    pub fn bias_10_iq_bb_7(&mut self) -> BIAS_10_IQ_BB_7_W {
        BIAS_10_IQ_BB_7_W { w: self }
    }
}
