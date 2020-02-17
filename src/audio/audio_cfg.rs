#[doc = "Reader of register AUDIO_CFG"]
pub type R = crate::R<u32, super::AUDIO_CFG>;
#[doc = "Writer for register AUDIO_CFG"]
pub type W = crate::W<u32, super::AUDIO_CFG>;
#[doc = "Register AUDIO_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::AUDIO_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Output driver clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD_CLK_SRC_A {
    #[doc = "0: OD uses AUDIOCLK"]
    OD_AUDIOCLK = 0,
    #[doc = "1: OD uses AUDIOSLOWCLK"]
    OD_AUDIOSLOWCLK = 1,
}
impl From<OD_CLK_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: OD_CLK_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OD_CLK_SRC`"]
pub type OD_CLK_SRC_R = crate::R<bool, OD_CLK_SRC_A>;
impl OD_CLK_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OD_CLK_SRC_A {
        match self.bits {
            false => OD_CLK_SRC_A::OD_AUDIOCLK,
            true => OD_CLK_SRC_A::OD_AUDIOSLOWCLK,
        }
    }
    #[doc = "Checks if the value of the field is `OD_AUDIOCLK`"]
    #[inline(always)]
    pub fn is_od_audioclk(&self) -> bool {
        *self == OD_CLK_SRC_A::OD_AUDIOCLK
    }
    #[doc = "Checks if the value of the field is `OD_AUDIOSLOWCLK`"]
    #[inline(always)]
    pub fn is_od_audioslowclk(&self) -> bool {
        *self == OD_CLK_SRC_A::OD_AUDIOSLOWCLK
    }
}
#[doc = "Write proxy for field `OD_CLK_SRC`"]
pub struct OD_CLK_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> OD_CLK_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD_CLK_SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "OD uses AUDIOCLK"]
    #[inline(always)]
    pub fn od_audioclk(self) -> &'a mut W {
        self.variant(OD_CLK_SRC_A::OD_AUDIOCLK)
    }
    #[doc = "OD uses AUDIOSLOWCLK"]
    #[inline(always)]
    pub fn od_audioslowclk(self) -> &'a mut W {
        self.variant(OD_CLK_SRC_A::OD_AUDIOSLOWCLK)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "DMIC clock selection (the same clock must be output to the DMIC_CLK DIO pad)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIC_CLK_SRC_A {
    #[doc = "0: DMIC uses AUDIOCLK"]
    DMIC_AUDIOCLK = 0,
    #[doc = "1: DMIC uses AUDIOSLOWCLK"]
    DMIC_AUDIOSLOWCLK = 1,
}
impl From<DMIC_CLK_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC_CLK_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMIC_CLK_SRC`"]
pub type DMIC_CLK_SRC_R = crate::R<bool, DMIC_CLK_SRC_A>;
impl DMIC_CLK_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC_CLK_SRC_A {
        match self.bits {
            false => DMIC_CLK_SRC_A::DMIC_AUDIOCLK,
            true => DMIC_CLK_SRC_A::DMIC_AUDIOSLOWCLK,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC_AUDIOCLK`"]
    #[inline(always)]
    pub fn is_dmic_audioclk(&self) -> bool {
        *self == DMIC_CLK_SRC_A::DMIC_AUDIOCLK
    }
    #[doc = "Checks if the value of the field is `DMIC_AUDIOSLOWCLK`"]
    #[inline(always)]
    pub fn is_dmic_audioslowclk(&self) -> bool {
        *self == DMIC_CLK_SRC_A::DMIC_AUDIOSLOWCLK
    }
}
#[doc = "Write proxy for field `DMIC_CLK_SRC`"]
pub struct DMIC_CLK_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC_CLK_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC_CLK_SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMIC uses AUDIOCLK"]
    #[inline(always)]
    pub fn dmic_audioclk(self) -> &'a mut W {
        self.variant(DMIC_CLK_SRC_A::DMIC_AUDIOCLK)
    }
    #[doc = "DMIC uses AUDIOSLOWCLK"]
    #[inline(always)]
    pub fn dmic_audioslowclk(self) -> &'a mut W {
        self.variant(DMIC_CLK_SRC_A::DMIC_AUDIOSLOWCLK)
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
#[doc = "DMIC input data decimation rate (also determines the OD interpolation rate in combination with DMIC_CLK_SRC and OD_CLK_SRC configuration bits)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEC_RATE_A {
    #[doc = "0: Decimate the DMIC input data by 64"]
    DECIMATE_BY_64 = 0,
    #[doc = "1: Decimate the DMIC input data by 72"]
    DECIMATE_BY_72 = 1,
    #[doc = "2: Decimate the DMIC input data by 80"]
    DECIMATE_BY_80 = 2,
    #[doc = "8: Decimate the DMIC input data by 128"]
    DECIMATE_BY_128 = 8,
    #[doc = "9: Decimate the DMIC input data by 136"]
    DECIMATE_BY_136 = 9,
    #[doc = "22: Decimate the DMIC input data by 256"]
    DECIMATE_BY_256 = 22,
    #[doc = "28: Decimate the DMIC input data by 288"]
    DECIMATE_BY_288 = 28,
}
impl From<DEC_RATE_A> for u8 {
    #[inline(always)]
    fn from(variant: DEC_RATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DEC_RATE`"]
pub type DEC_RATE_R = crate::R<u8, DEC_RATE_A>;
impl DEC_RATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DEC_RATE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DEC_RATE_A::DECIMATE_BY_64),
            1 => Val(DEC_RATE_A::DECIMATE_BY_72),
            2 => Val(DEC_RATE_A::DECIMATE_BY_80),
            8 => Val(DEC_RATE_A::DECIMATE_BY_128),
            9 => Val(DEC_RATE_A::DECIMATE_BY_136),
            22 => Val(DEC_RATE_A::DECIMATE_BY_256),
            28 => Val(DEC_RATE_A::DECIMATE_BY_288),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DECIMATE_BY_64`"]
    #[inline(always)]
    pub fn is_decimate_by_64(&self) -> bool {
        *self == DEC_RATE_A::DECIMATE_BY_64
    }
    #[doc = "Checks if the value of the field is `DECIMATE_BY_72`"]
    #[inline(always)]
    pub fn is_decimate_by_72(&self) -> bool {
        *self == DEC_RATE_A::DECIMATE_BY_72
    }
    #[doc = "Checks if the value of the field is `DECIMATE_BY_80`"]
    #[inline(always)]
    pub fn is_decimate_by_80(&self) -> bool {
        *self == DEC_RATE_A::DECIMATE_BY_80
    }
    #[doc = "Checks if the value of the field is `DECIMATE_BY_128`"]
    #[inline(always)]
    pub fn is_decimate_by_128(&self) -> bool {
        *self == DEC_RATE_A::DECIMATE_BY_128
    }
    #[doc = "Checks if the value of the field is `DECIMATE_BY_136`"]
    #[inline(always)]
    pub fn is_decimate_by_136(&self) -> bool {
        *self == DEC_RATE_A::DECIMATE_BY_136
    }
    #[doc = "Checks if the value of the field is `DECIMATE_BY_256`"]
    #[inline(always)]
    pub fn is_decimate_by_256(&self) -> bool {
        *self == DEC_RATE_A::DECIMATE_BY_256
    }
    #[doc = "Checks if the value of the field is `DECIMATE_BY_288`"]
    #[inline(always)]
    pub fn is_decimate_by_288(&self) -> bool {
        *self == DEC_RATE_A::DECIMATE_BY_288
    }
}
#[doc = "Write proxy for field `DEC_RATE`"]
pub struct DEC_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEC_RATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEC_RATE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Decimate the DMIC input data by 64"]
    #[inline(always)]
    pub fn decimate_by_64(self) -> &'a mut W {
        self.variant(DEC_RATE_A::DECIMATE_BY_64)
    }
    #[doc = "Decimate the DMIC input data by 72"]
    #[inline(always)]
    pub fn decimate_by_72(self) -> &'a mut W {
        self.variant(DEC_RATE_A::DECIMATE_BY_72)
    }
    #[doc = "Decimate the DMIC input data by 80"]
    #[inline(always)]
    pub fn decimate_by_80(self) -> &'a mut W {
        self.variant(DEC_RATE_A::DECIMATE_BY_80)
    }
    #[doc = "Decimate the DMIC input data by 128"]
    #[inline(always)]
    pub fn decimate_by_128(self) -> &'a mut W {
        self.variant(DEC_RATE_A::DECIMATE_BY_128)
    }
    #[doc = "Decimate the DMIC input data by 136"]
    #[inline(always)]
    pub fn decimate_by_136(self) -> &'a mut W {
        self.variant(DEC_RATE_A::DECIMATE_BY_136)
    }
    #[doc = "Decimate the DMIC input data by 256"]
    #[inline(always)]
    pub fn decimate_by_256(self) -> &'a mut W {
        self.variant(DEC_RATE_A::DECIMATE_BY_256)
    }
    #[doc = "Decimate the DMIC input data by 288"]
    #[inline(always)]
    pub fn decimate_by_288(self) -> &'a mut W {
        self.variant(DEC_RATE_A::DECIMATE_BY_288)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Enable OD_DATA underrun protection (automatically resets OD_DATA if it hasn't been updated during 16 sample periods)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD_UNDERRUN_PROTECT_A {
    #[doc = "0: OD_DATA underrun protection disabled"]
    OD_UNDERRUN_PROTECT_DISABLE = 0,
    #[doc = "1: OD_DATA underrun protection enabled"]
    OD_UNDERRUN_PROTECT_ENABLE = 1,
}
impl From<OD_UNDERRUN_PROTECT_A> for bool {
    #[inline(always)]
    fn from(variant: OD_UNDERRUN_PROTECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OD_UNDERRUN_PROTECT`"]
pub type OD_UNDERRUN_PROTECT_R = crate::R<bool, OD_UNDERRUN_PROTECT_A>;
impl OD_UNDERRUN_PROTECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OD_UNDERRUN_PROTECT_A {
        match self.bits {
            false => OD_UNDERRUN_PROTECT_A::OD_UNDERRUN_PROTECT_DISABLE,
            true => OD_UNDERRUN_PROTECT_A::OD_UNDERRUN_PROTECT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `OD_UNDERRUN_PROTECT_DISABLE`"]
    #[inline(always)]
    pub fn is_od_underrun_protect_disable(&self) -> bool {
        *self == OD_UNDERRUN_PROTECT_A::OD_UNDERRUN_PROTECT_DISABLE
    }
    #[doc = "Checks if the value of the field is `OD_UNDERRUN_PROTECT_ENABLE`"]
    #[inline(always)]
    pub fn is_od_underrun_protect_enable(&self) -> bool {
        *self == OD_UNDERRUN_PROTECT_A::OD_UNDERRUN_PROTECT_ENABLE
    }
}
#[doc = "Write proxy for field `OD_UNDERRUN_PROTECT`"]
pub struct OD_UNDERRUN_PROTECT_W<'a> {
    w: &'a mut W,
}
impl<'a> OD_UNDERRUN_PROTECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD_UNDERRUN_PROTECT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "OD_DATA underrun protection disabled"]
    #[inline(always)]
    pub fn od_underrun_protect_disable(self) -> &'a mut W {
        self.variant(OD_UNDERRUN_PROTECT_A::OD_UNDERRUN_PROTECT_DISABLE)
    }
    #[doc = "OD_DATA underrun protection enabled"]
    #[inline(always)]
    pub fn od_underrun_protect_enable(self) -> &'a mut W {
        self.variant(OD_UNDERRUN_PROTECT_A::OD_UNDERRUN_PROTECT_ENABLE)
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
#[doc = "Enable the DMA request when a new output driver sample is required\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD_DMA_REQ_EN_A {
    #[doc = "0: Disable the DMA request when a new OD sample is required"]
    OD_DMA_REQ_DISABLE = 0,
    #[doc = "1: Enable the DMA request when a new OD sample is required"]
    OD_DMA_REQ_ENABLE = 1,
}
impl From<OD_DMA_REQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OD_DMA_REQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OD_DMA_REQ_EN`"]
pub type OD_DMA_REQ_EN_R = crate::R<bool, OD_DMA_REQ_EN_A>;
impl OD_DMA_REQ_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OD_DMA_REQ_EN_A {
        match self.bits {
            false => OD_DMA_REQ_EN_A::OD_DMA_REQ_DISABLE,
            true => OD_DMA_REQ_EN_A::OD_DMA_REQ_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `OD_DMA_REQ_DISABLE`"]
    #[inline(always)]
    pub fn is_od_dma_req_disable(&self) -> bool {
        *self == OD_DMA_REQ_EN_A::OD_DMA_REQ_DISABLE
    }
    #[doc = "Checks if the value of the field is `OD_DMA_REQ_ENABLE`"]
    #[inline(always)]
    pub fn is_od_dma_req_enable(&self) -> bool {
        *self == OD_DMA_REQ_EN_A::OD_DMA_REQ_ENABLE
    }
}
#[doc = "Write proxy for field `OD_DMA_REQ_EN`"]
pub struct OD_DMA_REQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OD_DMA_REQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD_DMA_REQ_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the DMA request when a new OD sample is required"]
    #[inline(always)]
    pub fn od_dma_req_disable(self) -> &'a mut W {
        self.variant(OD_DMA_REQ_EN_A::OD_DMA_REQ_DISABLE)
    }
    #[doc = "Enable the DMA request when a new OD sample is required"]
    #[inline(always)]
    pub fn od_dma_req_enable(self) -> &'a mut W {
        self.variant(OD_DMA_REQ_EN_A::OD_DMA_REQ_ENABLE)
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
#[doc = "Enable the interrupt generation when a new output driver sample is required\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD_INT_GEN_EN_A {
    #[doc = "0: Disable the interrupt generation when a new OD sample is required"]
    OD_INT_GEN_DISABLE = 0,
    #[doc = "1: Enable the interrupt generation when a new OD sample is required"]
    OD_INT_GEN_ENABLE = 1,
}
impl From<OD_INT_GEN_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OD_INT_GEN_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OD_INT_GEN_EN`"]
pub type OD_INT_GEN_EN_R = crate::R<bool, OD_INT_GEN_EN_A>;
impl OD_INT_GEN_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OD_INT_GEN_EN_A {
        match self.bits {
            false => OD_INT_GEN_EN_A::OD_INT_GEN_DISABLE,
            true => OD_INT_GEN_EN_A::OD_INT_GEN_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `OD_INT_GEN_DISABLE`"]
    #[inline(always)]
    pub fn is_od_int_gen_disable(&self) -> bool {
        *self == OD_INT_GEN_EN_A::OD_INT_GEN_DISABLE
    }
    #[doc = "Checks if the value of the field is `OD_INT_GEN_ENABLE`"]
    #[inline(always)]
    pub fn is_od_int_gen_enable(&self) -> bool {
        *self == OD_INT_GEN_EN_A::OD_INT_GEN_ENABLE
    }
}
#[doc = "Write proxy for field `OD_INT_GEN_EN`"]
pub struct OD_INT_GEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OD_INT_GEN_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD_INT_GEN_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the interrupt generation when a new OD sample is required"]
    #[inline(always)]
    pub fn od_int_gen_disable(self) -> &'a mut W {
        self.variant(OD_INT_GEN_EN_A::OD_INT_GEN_DISABLE)
    }
    #[doc = "Enable the interrupt generation when a new OD sample is required"]
    #[inline(always)]
    pub fn od_int_gen_enable(self) -> &'a mut W {
        self.variant(OD_INT_GEN_EN_A::OD_INT_GEN_ENABLE)
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
#[doc = "Data alignment in AUDIO_OD_DATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD_DATA_ALIGN_A {
    #[doc = "0: OD_DATA is 16-bit LSB aligned"]
    OD_DATA_LSB_ALIGNED = 0,
    #[doc = "1: OD_DATA is 18-bit MSB aligned"]
    OD_DATA_MSB_ALIGNED = 1,
}
impl From<OD_DATA_ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: OD_DATA_ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OD_DATA_ALIGN`"]
pub type OD_DATA_ALIGN_R = crate::R<bool, OD_DATA_ALIGN_A>;
impl OD_DATA_ALIGN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OD_DATA_ALIGN_A {
        match self.bits {
            false => OD_DATA_ALIGN_A::OD_DATA_LSB_ALIGNED,
            true => OD_DATA_ALIGN_A::OD_DATA_MSB_ALIGNED,
        }
    }
    #[doc = "Checks if the value of the field is `OD_DATA_LSB_ALIGNED`"]
    #[inline(always)]
    pub fn is_od_data_lsb_aligned(&self) -> bool {
        *self == OD_DATA_ALIGN_A::OD_DATA_LSB_ALIGNED
    }
    #[doc = "Checks if the value of the field is `OD_DATA_MSB_ALIGNED`"]
    #[inline(always)]
    pub fn is_od_data_msb_aligned(&self) -> bool {
        *self == OD_DATA_ALIGN_A::OD_DATA_MSB_ALIGNED
    }
}
#[doc = "Write proxy for field `OD_DATA_ALIGN`"]
pub struct OD_DATA_ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> OD_DATA_ALIGN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD_DATA_ALIGN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "OD_DATA is 16-bit LSB aligned"]
    #[inline(always)]
    pub fn od_data_lsb_aligned(self) -> &'a mut W {
        self.variant(OD_DATA_ALIGN_A::OD_DATA_LSB_ALIGNED)
    }
    #[doc = "OD_DATA is 18-bit MSB aligned"]
    #[inline(always)]
    pub fn od_data_msb_aligned(self) -> &'a mut W {
        self.variant(OD_DATA_ALIGN_A::OD_DATA_MSB_ALIGNED)
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
#[doc = "Enable output driver output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD_ENABLE_A {
    #[doc = "0: Disable the OD output"]
    OD_DISABLE = 0,
    #[doc = "1: Enable the OD output"]
    OD_ENABLE = 1,
}
impl From<OD_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: OD_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OD_ENABLE`"]
pub type OD_ENABLE_R = crate::R<bool, OD_ENABLE_A>;
impl OD_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OD_ENABLE_A {
        match self.bits {
            false => OD_ENABLE_A::OD_DISABLE,
            true => OD_ENABLE_A::OD_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `OD_DISABLE`"]
    #[inline(always)]
    pub fn is_od_disable(&self) -> bool {
        *self == OD_ENABLE_A::OD_DISABLE
    }
    #[doc = "Checks if the value of the field is `OD_ENABLE`"]
    #[inline(always)]
    pub fn is_od_enable(&self) -> bool {
        *self == OD_ENABLE_A::OD_ENABLE
    }
}
#[doc = "Write proxy for field `OD_ENABLE`"]
pub struct OD_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> OD_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the OD output"]
    #[inline(always)]
    pub fn od_disable(self) -> &'a mut W {
        self.variant(OD_ENABLE_A::OD_DISABLE)
    }
    #[doc = "Enable the OD output"]
    #[inline(always)]
    pub fn od_enable(self) -> &'a mut W {
        self.variant(OD_ENABLE_A::OD_ENABLE)
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
#[doc = "Enable the DMA request when a new DMIC1 sample is ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIC1_DMA_REQ_EN_A {
    #[doc = "0: Disable the DMA request when a new DMIC1 sample is ready"]
    DMIC1_DMA_REQ_DISABLE = 0,
    #[doc = "1: Enable the DMA request when a new DMIC1 sample is ready"]
    DMIC1_DMA_REQ_ENABLE = 1,
}
impl From<DMIC1_DMA_REQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC1_DMA_REQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMIC1_DMA_REQ_EN`"]
pub type DMIC1_DMA_REQ_EN_R = crate::R<bool, DMIC1_DMA_REQ_EN_A>;
impl DMIC1_DMA_REQ_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC1_DMA_REQ_EN_A {
        match self.bits {
            false => DMIC1_DMA_REQ_EN_A::DMIC1_DMA_REQ_DISABLE,
            true => DMIC1_DMA_REQ_EN_A::DMIC1_DMA_REQ_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC1_DMA_REQ_DISABLE`"]
    #[inline(always)]
    pub fn is_dmic1_dma_req_disable(&self) -> bool {
        *self == DMIC1_DMA_REQ_EN_A::DMIC1_DMA_REQ_DISABLE
    }
    #[doc = "Checks if the value of the field is `DMIC1_DMA_REQ_ENABLE`"]
    #[inline(always)]
    pub fn is_dmic1_dma_req_enable(&self) -> bool {
        *self == DMIC1_DMA_REQ_EN_A::DMIC1_DMA_REQ_ENABLE
    }
}
#[doc = "Write proxy for field `DMIC1_DMA_REQ_EN`"]
pub struct DMIC1_DMA_REQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC1_DMA_REQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC1_DMA_REQ_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the DMA request when a new DMIC1 sample is ready"]
    #[inline(always)]
    pub fn dmic1_dma_req_disable(self) -> &'a mut W {
        self.variant(DMIC1_DMA_REQ_EN_A::DMIC1_DMA_REQ_DISABLE)
    }
    #[doc = "Enable the DMA request when a new DMIC1 sample is ready"]
    #[inline(always)]
    pub fn dmic1_dma_req_enable(self) -> &'a mut W {
        self.variant(DMIC1_DMA_REQ_EN_A::DMIC1_DMA_REQ_ENABLE)
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
#[doc = "Enable the interrupt generation when a new DMIC1 sample is ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIC1_INT_GEN_EN_A {
    #[doc = "0: Disable the interrupt generation when a new DMIC1 sample is ready"]
    DMIC1_INT_GEN_DISABLE = 0,
    #[doc = "1: Enable the interrupt generation when a new DMIC1 sample is ready"]
    DMIC1_INT_GEN_ENABLE = 1,
}
impl From<DMIC1_INT_GEN_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC1_INT_GEN_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMIC1_INT_GEN_EN`"]
pub type DMIC1_INT_GEN_EN_R = crate::R<bool, DMIC1_INT_GEN_EN_A>;
impl DMIC1_INT_GEN_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC1_INT_GEN_EN_A {
        match self.bits {
            false => DMIC1_INT_GEN_EN_A::DMIC1_INT_GEN_DISABLE,
            true => DMIC1_INT_GEN_EN_A::DMIC1_INT_GEN_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC1_INT_GEN_DISABLE`"]
    #[inline(always)]
    pub fn is_dmic1_int_gen_disable(&self) -> bool {
        *self == DMIC1_INT_GEN_EN_A::DMIC1_INT_GEN_DISABLE
    }
    #[doc = "Checks if the value of the field is `DMIC1_INT_GEN_ENABLE`"]
    #[inline(always)]
    pub fn is_dmic1_int_gen_enable(&self) -> bool {
        *self == DMIC1_INT_GEN_EN_A::DMIC1_INT_GEN_ENABLE
    }
}
#[doc = "Write proxy for field `DMIC1_INT_GEN_EN`"]
pub struct DMIC1_INT_GEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC1_INT_GEN_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC1_INT_GEN_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the interrupt generation when a new DMIC1 sample is ready"]
    #[inline(always)]
    pub fn dmic1_int_gen_disable(self) -> &'a mut W {
        self.variant(DMIC1_INT_GEN_EN_A::DMIC1_INT_GEN_DISABLE)
    }
    #[doc = "Enable the interrupt generation when a new DMIC1 sample is ready"]
    #[inline(always)]
    pub fn dmic1_int_gen_enable(self) -> &'a mut W {
        self.variant(DMIC1_INT_GEN_EN_A::DMIC1_INT_GEN_ENABLE)
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
#[doc = "Data alignment in AUDIO_DMIC_DATA_1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIC1_DATA_ALIGN_A {
    #[doc = "0: DMIC1_DATA is 16-bit LSB aligned"]
    DMIC1_DATA_LSB_ALIGNED = 0,
    #[doc = "1: DMIC1_DATA is 18-bit MSB aligned"]
    DMIC1_DATA_MSB_ALIGNED = 1,
}
impl From<DMIC1_DATA_ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC1_DATA_ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMIC1_DATA_ALIGN`"]
pub type DMIC1_DATA_ALIGN_R = crate::R<bool, DMIC1_DATA_ALIGN_A>;
impl DMIC1_DATA_ALIGN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC1_DATA_ALIGN_A {
        match self.bits {
            false => DMIC1_DATA_ALIGN_A::DMIC1_DATA_LSB_ALIGNED,
            true => DMIC1_DATA_ALIGN_A::DMIC1_DATA_MSB_ALIGNED,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC1_DATA_LSB_ALIGNED`"]
    #[inline(always)]
    pub fn is_dmic1_data_lsb_aligned(&self) -> bool {
        *self == DMIC1_DATA_ALIGN_A::DMIC1_DATA_LSB_ALIGNED
    }
    #[doc = "Checks if the value of the field is `DMIC1_DATA_MSB_ALIGNED`"]
    #[inline(always)]
    pub fn is_dmic1_data_msb_aligned(&self) -> bool {
        *self == DMIC1_DATA_ALIGN_A::DMIC1_DATA_MSB_ALIGNED
    }
}
#[doc = "Write proxy for field `DMIC1_DATA_ALIGN`"]
pub struct DMIC1_DATA_ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC1_DATA_ALIGN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC1_DATA_ALIGN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMIC1_DATA is 16-bit LSB aligned"]
    #[inline(always)]
    pub fn dmic1_data_lsb_aligned(self) -> &'a mut W {
        self.variant(DMIC1_DATA_ALIGN_A::DMIC1_DATA_LSB_ALIGNED)
    }
    #[doc = "DMIC1_DATA is 18-bit MSB aligned"]
    #[inline(always)]
    pub fn dmic1_data_msb_aligned(self) -> &'a mut W {
        self.variant(DMIC1_DATA_ALIGN_A::DMIC1_DATA_MSB_ALIGNED)
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
#[doc = "Enable DMIC1 input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIC1_ENABLE_A {
    #[doc = "0: Disable the DMIC1 input"]
    DMIC1_DISABLE = 0,
    #[doc = "1: Enable the DMIC1 input"]
    DMIC1_ENABLE = 1,
}
impl From<DMIC1_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC1_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMIC1_ENABLE`"]
pub type DMIC1_ENABLE_R = crate::R<bool, DMIC1_ENABLE_A>;
impl DMIC1_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC1_ENABLE_A {
        match self.bits {
            false => DMIC1_ENABLE_A::DMIC1_DISABLE,
            true => DMIC1_ENABLE_A::DMIC1_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC1_DISABLE`"]
    #[inline(always)]
    pub fn is_dmic1_disable(&self) -> bool {
        *self == DMIC1_ENABLE_A::DMIC1_DISABLE
    }
    #[doc = "Checks if the value of the field is `DMIC1_ENABLE`"]
    #[inline(always)]
    pub fn is_dmic1_enable(&self) -> bool {
        *self == DMIC1_ENABLE_A::DMIC1_ENABLE
    }
}
#[doc = "Write proxy for field `DMIC1_ENABLE`"]
pub struct DMIC1_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC1_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC1_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the DMIC1 input"]
    #[inline(always)]
    pub fn dmic1_disable(self) -> &'a mut W {
        self.variant(DMIC1_ENABLE_A::DMIC1_DISABLE)
    }
    #[doc = "Enable the DMIC1 input"]
    #[inline(always)]
    pub fn dmic1_enable(self) -> &'a mut W {
        self.variant(DMIC1_ENABLE_A::DMIC1_ENABLE)
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
#[doc = "Enable the DMA request when a new DMIC0 sample is ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIC0_DMA_REQ_EN_A {
    #[doc = "0: Disable the DMA request when a new DMIC0 sample is ready"]
    DMIC0_DMA_REQ_DISABLE = 0,
    #[doc = "1: Enable the DMA request when a new DMIC0 sample is ready"]
    DMIC0_DMA_REQ_ENABLE = 1,
}
impl From<DMIC0_DMA_REQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_DMA_REQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMIC0_DMA_REQ_EN`"]
pub type DMIC0_DMA_REQ_EN_R = crate::R<bool, DMIC0_DMA_REQ_EN_A>;
impl DMIC0_DMA_REQ_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_DMA_REQ_EN_A {
        match self.bits {
            false => DMIC0_DMA_REQ_EN_A::DMIC0_DMA_REQ_DISABLE,
            true => DMIC0_DMA_REQ_EN_A::DMIC0_DMA_REQ_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC0_DMA_REQ_DISABLE`"]
    #[inline(always)]
    pub fn is_dmic0_dma_req_disable(&self) -> bool {
        *self == DMIC0_DMA_REQ_EN_A::DMIC0_DMA_REQ_DISABLE
    }
    #[doc = "Checks if the value of the field is `DMIC0_DMA_REQ_ENABLE`"]
    #[inline(always)]
    pub fn is_dmic0_dma_req_enable(&self) -> bool {
        *self == DMIC0_DMA_REQ_EN_A::DMIC0_DMA_REQ_ENABLE
    }
}
#[doc = "Write proxy for field `DMIC0_DMA_REQ_EN`"]
pub struct DMIC0_DMA_REQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC0_DMA_REQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC0_DMA_REQ_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the DMA request when a new DMIC0 sample is ready"]
    #[inline(always)]
    pub fn dmic0_dma_req_disable(self) -> &'a mut W {
        self.variant(DMIC0_DMA_REQ_EN_A::DMIC0_DMA_REQ_DISABLE)
    }
    #[doc = "Enable the DMA request when a new DMIC0 sample is ready"]
    #[inline(always)]
    pub fn dmic0_dma_req_enable(self) -> &'a mut W {
        self.variant(DMIC0_DMA_REQ_EN_A::DMIC0_DMA_REQ_ENABLE)
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
#[doc = "Enable the interrupt generation when a new DMIC0 sample is ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIC0_INT_GEN_EN_A {
    #[doc = "0: Disable the interrupt generation when a new DMIC0 sample is ready"]
    DMIC0_INT_GEN_DISABLE = 0,
    #[doc = "1: Enable the interrupt generation when a new DMIC0 sample is ready"]
    DMIC0_INT_GEN_ENABLE = 1,
}
impl From<DMIC0_INT_GEN_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_INT_GEN_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMIC0_INT_GEN_EN`"]
pub type DMIC0_INT_GEN_EN_R = crate::R<bool, DMIC0_INT_GEN_EN_A>;
impl DMIC0_INT_GEN_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_INT_GEN_EN_A {
        match self.bits {
            false => DMIC0_INT_GEN_EN_A::DMIC0_INT_GEN_DISABLE,
            true => DMIC0_INT_GEN_EN_A::DMIC0_INT_GEN_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC0_INT_GEN_DISABLE`"]
    #[inline(always)]
    pub fn is_dmic0_int_gen_disable(&self) -> bool {
        *self == DMIC0_INT_GEN_EN_A::DMIC0_INT_GEN_DISABLE
    }
    #[doc = "Checks if the value of the field is `DMIC0_INT_GEN_ENABLE`"]
    #[inline(always)]
    pub fn is_dmic0_int_gen_enable(&self) -> bool {
        *self == DMIC0_INT_GEN_EN_A::DMIC0_INT_GEN_ENABLE
    }
}
#[doc = "Write proxy for field `DMIC0_INT_GEN_EN`"]
pub struct DMIC0_INT_GEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC0_INT_GEN_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC0_INT_GEN_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the interrupt generation when a new DMIC0 sample is ready"]
    #[inline(always)]
    pub fn dmic0_int_gen_disable(self) -> &'a mut W {
        self.variant(DMIC0_INT_GEN_EN_A::DMIC0_INT_GEN_DISABLE)
    }
    #[doc = "Enable the interrupt generation when a new DMIC0 sample is ready"]
    #[inline(always)]
    pub fn dmic0_int_gen_enable(self) -> &'a mut W {
        self.variant(DMIC0_INT_GEN_EN_A::DMIC0_INT_GEN_ENABLE)
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
#[doc = "Data alignment in AUDIO_DMIC_DATA_0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIC0_DATA_ALIGN_A {
    #[doc = "0: DMIC0_DATA is 16-bit LSB aligned"]
    DMIC0_DATA_LSB_ALIGNED = 0,
    #[doc = "1: DMIC0_DATA is 18-bit MSB aligned"]
    DMIC0_DATA_MSB_ALIGNED = 1,
}
impl From<DMIC0_DATA_ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_DATA_ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMIC0_DATA_ALIGN`"]
pub type DMIC0_DATA_ALIGN_R = crate::R<bool, DMIC0_DATA_ALIGN_A>;
impl DMIC0_DATA_ALIGN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_DATA_ALIGN_A {
        match self.bits {
            false => DMIC0_DATA_ALIGN_A::DMIC0_DATA_LSB_ALIGNED,
            true => DMIC0_DATA_ALIGN_A::DMIC0_DATA_MSB_ALIGNED,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC0_DATA_LSB_ALIGNED`"]
    #[inline(always)]
    pub fn is_dmic0_data_lsb_aligned(&self) -> bool {
        *self == DMIC0_DATA_ALIGN_A::DMIC0_DATA_LSB_ALIGNED
    }
    #[doc = "Checks if the value of the field is `DMIC0_DATA_MSB_ALIGNED`"]
    #[inline(always)]
    pub fn is_dmic0_data_msb_aligned(&self) -> bool {
        *self == DMIC0_DATA_ALIGN_A::DMIC0_DATA_MSB_ALIGNED
    }
}
#[doc = "Write proxy for field `DMIC0_DATA_ALIGN`"]
pub struct DMIC0_DATA_ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC0_DATA_ALIGN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC0_DATA_ALIGN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMIC0_DATA is 16-bit LSB aligned"]
    #[inline(always)]
    pub fn dmic0_data_lsb_aligned(self) -> &'a mut W {
        self.variant(DMIC0_DATA_ALIGN_A::DMIC0_DATA_LSB_ALIGNED)
    }
    #[doc = "DMIC0_DATA is 18-bit MSB aligned"]
    #[inline(always)]
    pub fn dmic0_data_msb_aligned(self) -> &'a mut W {
        self.variant(DMIC0_DATA_ALIGN_A::DMIC0_DATA_MSB_ALIGNED)
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
#[doc = "Enable DMIC0 input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIC0_ENABLE_A {
    #[doc = "0: Disable the DMIC0 input"]
    DMIC0_DISABLE = 0,
    #[doc = "1: Enable the DMIC0 input"]
    DMIC0_ENABLE = 1,
}
impl From<DMIC0_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMIC0_ENABLE`"]
pub type DMIC0_ENABLE_R = crate::R<bool, DMIC0_ENABLE_A>;
impl DMIC0_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_ENABLE_A {
        match self.bits {
            false => DMIC0_ENABLE_A::DMIC0_DISABLE,
            true => DMIC0_ENABLE_A::DMIC0_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC0_DISABLE`"]
    #[inline(always)]
    pub fn is_dmic0_disable(&self) -> bool {
        *self == DMIC0_ENABLE_A::DMIC0_DISABLE
    }
    #[doc = "Checks if the value of the field is `DMIC0_ENABLE`"]
    #[inline(always)]
    pub fn is_dmic0_enable(&self) -> bool {
        *self == DMIC0_ENABLE_A::DMIC0_ENABLE
    }
}
#[doc = "Write proxy for field `DMIC0_ENABLE`"]
pub struct DMIC0_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC0_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC0_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the DMIC0 input"]
    #[inline(always)]
    pub fn dmic0_disable(self) -> &'a mut W {
        self.variant(DMIC0_ENABLE_A::DMIC0_DISABLE)
    }
    #[doc = "Enable the DMIC0 input"]
    #[inline(always)]
    pub fn dmic0_enable(self) -> &'a mut W {
        self.variant(DMIC0_ENABLE_A::DMIC0_ENABLE)
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
    #[doc = "Bit 25 - Output driver clock selection"]
    #[inline(always)]
    pub fn od_clk_src(&self) -> OD_CLK_SRC_R {
        OD_CLK_SRC_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMIC clock selection (the same clock must be output to the DMIC_CLK DIO pad)"]
    #[inline(always)]
    pub fn dmic_clk_src(&self) -> DMIC_CLK_SRC_R {
        DMIC_CLK_SRC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - DMIC input data decimation rate (also determines the OD interpolation rate in combination with DMIC_CLK_SRC and OD_CLK_SRC configuration bits)"]
    #[inline(always)]
    pub fn dec_rate(&self) -> DEC_RATE_R {
        DEC_RATE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 12 - Enable OD_DATA underrun protection (automatically resets OD_DATA if it hasn't been updated during 16 sample periods)"]
    #[inline(always)]
    pub fn od_underrun_protect(&self) -> OD_UNDERRUN_PROTECT_R {
        OD_UNDERRUN_PROTECT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable the DMA request when a new output driver sample is required"]
    #[inline(always)]
    pub fn od_dma_req_en(&self) -> OD_DMA_REQ_EN_R {
        OD_DMA_REQ_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable the interrupt generation when a new output driver sample is required"]
    #[inline(always)]
    pub fn od_int_gen_en(&self) -> OD_INT_GEN_EN_R {
        OD_INT_GEN_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Data alignment in AUDIO_OD_DATA"]
    #[inline(always)]
    pub fn od_data_align(&self) -> OD_DATA_ALIGN_R {
        OD_DATA_ALIGN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable output driver output"]
    #[inline(always)]
    pub fn od_enable(&self) -> OD_ENABLE_R {
        OD_ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable the DMA request when a new DMIC1 sample is ready"]
    #[inline(always)]
    pub fn dmic1_dma_req_en(&self) -> DMIC1_DMA_REQ_EN_R {
        DMIC1_DMA_REQ_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable the interrupt generation when a new DMIC1 sample is ready"]
    #[inline(always)]
    pub fn dmic1_int_gen_en(&self) -> DMIC1_INT_GEN_EN_R {
        DMIC1_INT_GEN_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data alignment in AUDIO_DMIC_DATA_1"]
    #[inline(always)]
    pub fn dmic1_data_align(&self) -> DMIC1_DATA_ALIGN_R {
        DMIC1_DATA_ALIGN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable DMIC1 input"]
    #[inline(always)]
    pub fn dmic1_enable(&self) -> DMIC1_ENABLE_R {
        DMIC1_ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable the DMA request when a new DMIC0 sample is ready"]
    #[inline(always)]
    pub fn dmic0_dma_req_en(&self) -> DMIC0_DMA_REQ_EN_R {
        DMIC0_DMA_REQ_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable the interrupt generation when a new DMIC0 sample is ready"]
    #[inline(always)]
    pub fn dmic0_int_gen_en(&self) -> DMIC0_INT_GEN_EN_R {
        DMIC0_INT_GEN_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data alignment in AUDIO_DMIC_DATA_0"]
    #[inline(always)]
    pub fn dmic0_data_align(&self) -> DMIC0_DATA_ALIGN_R {
        DMIC0_DATA_ALIGN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable DMIC0 input"]
    #[inline(always)]
    pub fn dmic0_enable(&self) -> DMIC0_ENABLE_R {
        DMIC0_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - Output driver clock selection"]
    #[inline(always)]
    pub fn od_clk_src(&mut self) -> OD_CLK_SRC_W {
        OD_CLK_SRC_W { w: self }
    }
    #[doc = "Bit 24 - DMIC clock selection (the same clock must be output to the DMIC_CLK DIO pad)"]
    #[inline(always)]
    pub fn dmic_clk_src(&mut self) -> DMIC_CLK_SRC_W {
        DMIC_CLK_SRC_W { w: self }
    }
    #[doc = "Bits 16:20 - DMIC input data decimation rate (also determines the OD interpolation rate in combination with DMIC_CLK_SRC and OD_CLK_SRC configuration bits)"]
    #[inline(always)]
    pub fn dec_rate(&mut self) -> DEC_RATE_W {
        DEC_RATE_W { w: self }
    }
    #[doc = "Bit 12 - Enable OD_DATA underrun protection (automatically resets OD_DATA if it hasn't been updated during 16 sample periods)"]
    #[inline(always)]
    pub fn od_underrun_protect(&mut self) -> OD_UNDERRUN_PROTECT_W {
        OD_UNDERRUN_PROTECT_W { w: self }
    }
    #[doc = "Bit 11 - Enable the DMA request when a new output driver sample is required"]
    #[inline(always)]
    pub fn od_dma_req_en(&mut self) -> OD_DMA_REQ_EN_W {
        OD_DMA_REQ_EN_W { w: self }
    }
    #[doc = "Bit 10 - Enable the interrupt generation when a new output driver sample is required"]
    #[inline(always)]
    pub fn od_int_gen_en(&mut self) -> OD_INT_GEN_EN_W {
        OD_INT_GEN_EN_W { w: self }
    }
    #[doc = "Bit 9 - Data alignment in AUDIO_OD_DATA"]
    #[inline(always)]
    pub fn od_data_align(&mut self) -> OD_DATA_ALIGN_W {
        OD_DATA_ALIGN_W { w: self }
    }
    #[doc = "Bit 8 - Enable output driver output"]
    #[inline(always)]
    pub fn od_enable(&mut self) -> OD_ENABLE_W {
        OD_ENABLE_W { w: self }
    }
    #[doc = "Bit 7 - Enable the DMA request when a new DMIC1 sample is ready"]
    #[inline(always)]
    pub fn dmic1_dma_req_en(&mut self) -> DMIC1_DMA_REQ_EN_W {
        DMIC1_DMA_REQ_EN_W { w: self }
    }
    #[doc = "Bit 6 - Enable the interrupt generation when a new DMIC1 sample is ready"]
    #[inline(always)]
    pub fn dmic1_int_gen_en(&mut self) -> DMIC1_INT_GEN_EN_W {
        DMIC1_INT_GEN_EN_W { w: self }
    }
    #[doc = "Bit 5 - Data alignment in AUDIO_DMIC_DATA_1"]
    #[inline(always)]
    pub fn dmic1_data_align(&mut self) -> DMIC1_DATA_ALIGN_W {
        DMIC1_DATA_ALIGN_W { w: self }
    }
    #[doc = "Bit 4 - Enable DMIC1 input"]
    #[inline(always)]
    pub fn dmic1_enable(&mut self) -> DMIC1_ENABLE_W {
        DMIC1_ENABLE_W { w: self }
    }
    #[doc = "Bit 3 - Enable the DMA request when a new DMIC0 sample is ready"]
    #[inline(always)]
    pub fn dmic0_dma_req_en(&mut self) -> DMIC0_DMA_REQ_EN_W {
        DMIC0_DMA_REQ_EN_W { w: self }
    }
    #[doc = "Bit 2 - Enable the interrupt generation when a new DMIC0 sample is ready"]
    #[inline(always)]
    pub fn dmic0_int_gen_en(&mut self) -> DMIC0_INT_GEN_EN_W {
        DMIC0_INT_GEN_EN_W { w: self }
    }
    #[doc = "Bit 1 - Data alignment in AUDIO_DMIC_DATA_0"]
    #[inline(always)]
    pub fn dmic0_data_align(&mut self) -> DMIC0_DATA_ALIGN_W {
        DMIC0_DATA_ALIGN_W { w: self }
    }
    #[doc = "Bit 0 - Enable DMIC0 input"]
    #[inline(always)]
    pub fn dmic0_enable(&mut self) -> DMIC0_ENABLE_W {
        DMIC0_ENABLE_W { w: self }
    }
}
