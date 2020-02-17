#[doc = "Reader of register ASRC_CFG"]
pub type R = crate::R<u32, super::ASRC_CFG>;
#[doc = "Writer for register ASRC_CFG"]
pub type W = crate::W<u32, super::ASRC_CFG>;
#[doc = "Register ASRC_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::ASRC_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "WDF Type Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDF_TYPE_A {
    #[doc = "0: Low Delay filter"]
    LOW_DELAY = 0,
    #[doc = "1: Wide band response filter"]
    WIDE_BAND = 1,
}
impl From<WDF_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: WDF_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDF_TYPE`"]
pub type WDF_TYPE_R = crate::R<bool, WDF_TYPE_A>;
impl WDF_TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDF_TYPE_A {
        match self.bits {
            false => WDF_TYPE_A::LOW_DELAY,
            true => WDF_TYPE_A::WIDE_BAND,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DELAY`"]
    #[inline(always)]
    pub fn is_low_delay(&self) -> bool {
        *self == WDF_TYPE_A::LOW_DELAY
    }
    #[doc = "Checks if the value of the field is `WIDE_BAND`"]
    #[inline(always)]
    pub fn is_wide_band(&self) -> bool {
        *self == WDF_TYPE_A::WIDE_BAND
    }
}
#[doc = "Write proxy for field `WDF_TYPE`"]
pub struct WDF_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDF_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDF_TYPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low Delay filter"]
    #[inline(always)]
    pub fn low_delay(self) -> &'a mut W {
        self.variant(WDF_TYPE_A::LOW_DELAY)
    }
    #[doc = "Wide band response filter"]
    #[inline(always)]
    pub fn wide_band(self) -> &'a mut W {
        self.variant(WDF_TYPE_A::WIDE_BAND)
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
#[doc = "ASRC mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ASRC_MODE_A {
    #[doc = "0: Interpolation mode"]
    ASRC_INT_MODE = 0,
    #[doc = "1: Decimation mode 1"]
    ASRC_DEC_MODE1 = 1,
    #[doc = "2: Decimation mode 2"]
    ASRC_DEC_MODE2 = 2,
    #[doc = "3: Decimation mode 3"]
    ASRC_DEC_MODE3 = 3,
}
impl From<ASRC_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ASRC_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ASRC_MODE`"]
pub type ASRC_MODE_R = crate::R<u8, ASRC_MODE_A>;
impl ASRC_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASRC_MODE_A {
        match self.bits {
            0 => ASRC_MODE_A::ASRC_INT_MODE,
            1 => ASRC_MODE_A::ASRC_DEC_MODE1,
            2 => ASRC_MODE_A::ASRC_DEC_MODE2,
            3 => ASRC_MODE_A::ASRC_DEC_MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ASRC_INT_MODE`"]
    #[inline(always)]
    pub fn is_asrc_int_mode(&self) -> bool {
        *self == ASRC_MODE_A::ASRC_INT_MODE
    }
    #[doc = "Checks if the value of the field is `ASRC_DEC_MODE1`"]
    #[inline(always)]
    pub fn is_asrc_dec_mode1(&self) -> bool {
        *self == ASRC_MODE_A::ASRC_DEC_MODE1
    }
    #[doc = "Checks if the value of the field is `ASRC_DEC_MODE2`"]
    #[inline(always)]
    pub fn is_asrc_dec_mode2(&self) -> bool {
        *self == ASRC_MODE_A::ASRC_DEC_MODE2
    }
    #[doc = "Checks if the value of the field is `ASRC_DEC_MODE3`"]
    #[inline(always)]
    pub fn is_asrc_dec_mode3(&self) -> bool {
        *self == ASRC_MODE_A::ASRC_DEC_MODE3
    }
}
#[doc = "Write proxy for field `ASRC_MODE`"]
pub struct ASRC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASRC_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASRC_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interpolation mode"]
    #[inline(always)]
    pub fn asrc_int_mode(self) -> &'a mut W {
        self.variant(ASRC_MODE_A::ASRC_INT_MODE)
    }
    #[doc = "Decimation mode 1"]
    #[inline(always)]
    pub fn asrc_dec_mode1(self) -> &'a mut W {
        self.variant(ASRC_MODE_A::ASRC_DEC_MODE1)
    }
    #[doc = "Decimation mode 2"]
    #[inline(always)]
    pub fn asrc_dec_mode2(self) -> &'a mut W {
        self.variant(ASRC_MODE_A::ASRC_DEC_MODE2)
    }
    #[doc = "Decimation mode 3"]
    #[inline(always)]
    pub fn asrc_dec_mode3(self) -> &'a mut W {
        self.variant(ASRC_MODE_A::ASRC_DEC_MODE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - WDF Type Selection"]
    #[inline(always)]
    pub fn wdf_type(&self) -> WDF_TYPE_R {
        WDF_TYPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - ASRC mode"]
    #[inline(always)]
    pub fn asrc_mode(&self) -> ASRC_MODE_R {
        ASRC_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - WDF Type Selection"]
    #[inline(always)]
    pub fn wdf_type(&mut self) -> WDF_TYPE_W {
        WDF_TYPE_W { w: self }
    }
    #[doc = "Bits 0:1 - ASRC mode"]
    #[inline(always)]
    pub fn asrc_mode(&mut self) -> ASRC_MODE_W {
        ASRC_MODE_W { w: self }
    }
}
