#[doc = "Reader of register AUDIO_OD_CFG"]
pub type R = crate::R<u32, super::AUDIO_OD_CFG>;
#[doc = "Writer for register AUDIO_OD_CFG"]
pub type W = crate::W<u32, super::AUDIO_OD_CFG>;
#[doc = "Register AUDIO_OD_CFG `reset()`'s with value 0x01"]
impl crate::ResetValue for super::AUDIO_OD_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Output driver DC removal filter enable and cut-off frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DCRM_A {
    #[doc = "0: DC removal filter disabled"]
    DCRM_DISABLE = 0,
    #[doc = "1: Cut-off frequency is OD_CLK/800000 (1.25 Hz at OD_CLK=1 MHz)"]
    DCRM_CUTOFF_1P25HZ = 1,
    #[doc = "2: Cut-off frequency is OD_CLK/400000 (2.5 Hz at OD_CLK=1 MHz)"]
    DCRM_CUTOFF_2P5HZ = 2,
    #[doc = "3: Cut-off frequency is OD_CLK/266667 (3.75 Hz at OD_CLK=1 MHz)"]
    DCRM_CUTOFF_3P75HZ = 3,
    #[doc = "4: Cut-off frequency is OD_CLK/200000 (5 Hz at OD_CLK=1 MHz)"]
    DCRM_CUTOFF_5HZ = 4,
    #[doc = "5: Cut-off frequency is OD_CLK/133333 (7.5 Hz at OD_CLK=1 MHz)"]
    DCRM_CUTOFF_7P5HZ = 5,
    #[doc = "6: Cut-off frequency is OD_CLK/100000 (10 Hz at OD_CLK=1 MHz)"]
    DCRM_CUTOFF_10HZ = 6,
    #[doc = "7: Cut-off frequency is OD_CLK/66667 (15 Hz at OD_CLK=1 MHz)"]
    DCRM_CUTOFF_15HZ = 7,
    #[doc = "8: Cut-off frequency is OD_CLK/50000 (20 Hz at OD_CLK=1 MHz)"]
    DCRM_CUTOFF_20HZ = 8,
    #[doc = "9: Cut-off frequency is OD_CLK/33333 (30 Hz at OD_CLK=1 MHz)"]
    DCRM_CUTOFF_30HZ = 9,
    #[doc = "10: Cut-off frequency is OD_CLK/25000 (40 Hz at OD_CLK=1 MHz)"]
    DCRM_CUTOFF_40HZ = 10,
    #[doc = "11: Cut-off frequency is OD_CLK/16667 (60 Hz at OD_CLK=1 MHz)"]
    DCRM_CUTOFF_60HZ = 11,
    #[doc = "12: Cut-off frequency is OD_CLK/12500 (80 Hz at OD_CLK=1 MHz)"]
    DCRM_CUTOFF_80HZ = 12,
    #[doc = "13: Cut-off frequency is OD_CLK/8333 (120 Hz at OD_CLK=1 MHz)"]
    DCRM_CUTOFF_120HZ = 13,
    #[doc = "14: Cut-off frequency is OD_CLK/6250 (160 Hz at OD_CLK=1 MHz)"]
    DCRM_CUTOFF_160HZ = 14,
    #[doc = "15: Cut-off frequency is OD_CLK/4167 (240 Hz at OD_CLK=1 MHz)"]
    DCRM_CUTOFF_240HZ = 15,
}
impl From<DCRM_A> for u8 {
    #[inline(always)]
    fn from(variant: DCRM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DCRM`"]
pub type DCRM_R = crate::R<u8, DCRM_A>;
impl DCRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCRM_A {
        match self.bits {
            0 => DCRM_A::DCRM_DISABLE,
            1 => DCRM_A::DCRM_CUTOFF_1P25HZ,
            2 => DCRM_A::DCRM_CUTOFF_2P5HZ,
            3 => DCRM_A::DCRM_CUTOFF_3P75HZ,
            4 => DCRM_A::DCRM_CUTOFF_5HZ,
            5 => DCRM_A::DCRM_CUTOFF_7P5HZ,
            6 => DCRM_A::DCRM_CUTOFF_10HZ,
            7 => DCRM_A::DCRM_CUTOFF_15HZ,
            8 => DCRM_A::DCRM_CUTOFF_20HZ,
            9 => DCRM_A::DCRM_CUTOFF_30HZ,
            10 => DCRM_A::DCRM_CUTOFF_40HZ,
            11 => DCRM_A::DCRM_CUTOFF_60HZ,
            12 => DCRM_A::DCRM_CUTOFF_80HZ,
            13 => DCRM_A::DCRM_CUTOFF_120HZ,
            14 => DCRM_A::DCRM_CUTOFF_160HZ,
            15 => DCRM_A::DCRM_CUTOFF_240HZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DCRM_DISABLE`"]
    #[inline(always)]
    pub fn is_dcrm_disable(&self) -> bool {
        *self == DCRM_A::DCRM_DISABLE
    }
    #[doc = "Checks if the value of the field is `DCRM_CUTOFF_1P25HZ`"]
    #[inline(always)]
    pub fn is_dcrm_cutoff_1p25hz(&self) -> bool {
        *self == DCRM_A::DCRM_CUTOFF_1P25HZ
    }
    #[doc = "Checks if the value of the field is `DCRM_CUTOFF_2P5HZ`"]
    #[inline(always)]
    pub fn is_dcrm_cutoff_2p5hz(&self) -> bool {
        *self == DCRM_A::DCRM_CUTOFF_2P5HZ
    }
    #[doc = "Checks if the value of the field is `DCRM_CUTOFF_3P75HZ`"]
    #[inline(always)]
    pub fn is_dcrm_cutoff_3p75hz(&self) -> bool {
        *self == DCRM_A::DCRM_CUTOFF_3P75HZ
    }
    #[doc = "Checks if the value of the field is `DCRM_CUTOFF_5HZ`"]
    #[inline(always)]
    pub fn is_dcrm_cutoff_5hz(&self) -> bool {
        *self == DCRM_A::DCRM_CUTOFF_5HZ
    }
    #[doc = "Checks if the value of the field is `DCRM_CUTOFF_7P5HZ`"]
    #[inline(always)]
    pub fn is_dcrm_cutoff_7p5hz(&self) -> bool {
        *self == DCRM_A::DCRM_CUTOFF_7P5HZ
    }
    #[doc = "Checks if the value of the field is `DCRM_CUTOFF_10HZ`"]
    #[inline(always)]
    pub fn is_dcrm_cutoff_10hz(&self) -> bool {
        *self == DCRM_A::DCRM_CUTOFF_10HZ
    }
    #[doc = "Checks if the value of the field is `DCRM_CUTOFF_15HZ`"]
    #[inline(always)]
    pub fn is_dcrm_cutoff_15hz(&self) -> bool {
        *self == DCRM_A::DCRM_CUTOFF_15HZ
    }
    #[doc = "Checks if the value of the field is `DCRM_CUTOFF_20HZ`"]
    #[inline(always)]
    pub fn is_dcrm_cutoff_20hz(&self) -> bool {
        *self == DCRM_A::DCRM_CUTOFF_20HZ
    }
    #[doc = "Checks if the value of the field is `DCRM_CUTOFF_30HZ`"]
    #[inline(always)]
    pub fn is_dcrm_cutoff_30hz(&self) -> bool {
        *self == DCRM_A::DCRM_CUTOFF_30HZ
    }
    #[doc = "Checks if the value of the field is `DCRM_CUTOFF_40HZ`"]
    #[inline(always)]
    pub fn is_dcrm_cutoff_40hz(&self) -> bool {
        *self == DCRM_A::DCRM_CUTOFF_40HZ
    }
    #[doc = "Checks if the value of the field is `DCRM_CUTOFF_60HZ`"]
    #[inline(always)]
    pub fn is_dcrm_cutoff_60hz(&self) -> bool {
        *self == DCRM_A::DCRM_CUTOFF_60HZ
    }
    #[doc = "Checks if the value of the field is `DCRM_CUTOFF_80HZ`"]
    #[inline(always)]
    pub fn is_dcrm_cutoff_80hz(&self) -> bool {
        *self == DCRM_A::DCRM_CUTOFF_80HZ
    }
    #[doc = "Checks if the value of the field is `DCRM_CUTOFF_120HZ`"]
    #[inline(always)]
    pub fn is_dcrm_cutoff_120hz(&self) -> bool {
        *self == DCRM_A::DCRM_CUTOFF_120HZ
    }
    #[doc = "Checks if the value of the field is `DCRM_CUTOFF_160HZ`"]
    #[inline(always)]
    pub fn is_dcrm_cutoff_160hz(&self) -> bool {
        *self == DCRM_A::DCRM_CUTOFF_160HZ
    }
    #[doc = "Checks if the value of the field is `DCRM_CUTOFF_240HZ`"]
    #[inline(always)]
    pub fn is_dcrm_cutoff_240hz(&self) -> bool {
        *self == DCRM_A::DCRM_CUTOFF_240HZ
    }
}
#[doc = "Write proxy for field `DCRM`"]
pub struct DCRM_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCRM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DC removal filter disabled"]
    #[inline(always)]
    pub fn dcrm_disable(self) -> &'a mut W {
        self.variant(DCRM_A::DCRM_DISABLE)
    }
    #[doc = "Cut-off frequency is OD_CLK/800000 (1.25 Hz at OD_CLK=1 MHz)"]
    #[inline(always)]
    pub fn dcrm_cutoff_1p25hz(self) -> &'a mut W {
        self.variant(DCRM_A::DCRM_CUTOFF_1P25HZ)
    }
    #[doc = "Cut-off frequency is OD_CLK/400000 (2.5 Hz at OD_CLK=1 MHz)"]
    #[inline(always)]
    pub fn dcrm_cutoff_2p5hz(self) -> &'a mut W {
        self.variant(DCRM_A::DCRM_CUTOFF_2P5HZ)
    }
    #[doc = "Cut-off frequency is OD_CLK/266667 (3.75 Hz at OD_CLK=1 MHz)"]
    #[inline(always)]
    pub fn dcrm_cutoff_3p75hz(self) -> &'a mut W {
        self.variant(DCRM_A::DCRM_CUTOFF_3P75HZ)
    }
    #[doc = "Cut-off frequency is OD_CLK/200000 (5 Hz at OD_CLK=1 MHz)"]
    #[inline(always)]
    pub fn dcrm_cutoff_5hz(self) -> &'a mut W {
        self.variant(DCRM_A::DCRM_CUTOFF_5HZ)
    }
    #[doc = "Cut-off frequency is OD_CLK/133333 (7.5 Hz at OD_CLK=1 MHz)"]
    #[inline(always)]
    pub fn dcrm_cutoff_7p5hz(self) -> &'a mut W {
        self.variant(DCRM_A::DCRM_CUTOFF_7P5HZ)
    }
    #[doc = "Cut-off frequency is OD_CLK/100000 (10 Hz at OD_CLK=1 MHz)"]
    #[inline(always)]
    pub fn dcrm_cutoff_10hz(self) -> &'a mut W {
        self.variant(DCRM_A::DCRM_CUTOFF_10HZ)
    }
    #[doc = "Cut-off frequency is OD_CLK/66667 (15 Hz at OD_CLK=1 MHz)"]
    #[inline(always)]
    pub fn dcrm_cutoff_15hz(self) -> &'a mut W {
        self.variant(DCRM_A::DCRM_CUTOFF_15HZ)
    }
    #[doc = "Cut-off frequency is OD_CLK/50000 (20 Hz at OD_CLK=1 MHz)"]
    #[inline(always)]
    pub fn dcrm_cutoff_20hz(self) -> &'a mut W {
        self.variant(DCRM_A::DCRM_CUTOFF_20HZ)
    }
    #[doc = "Cut-off frequency is OD_CLK/33333 (30 Hz at OD_CLK=1 MHz)"]
    #[inline(always)]
    pub fn dcrm_cutoff_30hz(self) -> &'a mut W {
        self.variant(DCRM_A::DCRM_CUTOFF_30HZ)
    }
    #[doc = "Cut-off frequency is OD_CLK/25000 (40 Hz at OD_CLK=1 MHz)"]
    #[inline(always)]
    pub fn dcrm_cutoff_40hz(self) -> &'a mut W {
        self.variant(DCRM_A::DCRM_CUTOFF_40HZ)
    }
    #[doc = "Cut-off frequency is OD_CLK/16667 (60 Hz at OD_CLK=1 MHz)"]
    #[inline(always)]
    pub fn dcrm_cutoff_60hz(self) -> &'a mut W {
        self.variant(DCRM_A::DCRM_CUTOFF_60HZ)
    }
    #[doc = "Cut-off frequency is OD_CLK/12500 (80 Hz at OD_CLK=1 MHz)"]
    #[inline(always)]
    pub fn dcrm_cutoff_80hz(self) -> &'a mut W {
        self.variant(DCRM_A::DCRM_CUTOFF_80HZ)
    }
    #[doc = "Cut-off frequency is OD_CLK/8333 (120 Hz at OD_CLK=1 MHz)"]
    #[inline(always)]
    pub fn dcrm_cutoff_120hz(self) -> &'a mut W {
        self.variant(DCRM_A::DCRM_CUTOFF_120HZ)
    }
    #[doc = "Cut-off frequency is OD_CLK/6250 (160 Hz at OD_CLK=1 MHz)"]
    #[inline(always)]
    pub fn dcrm_cutoff_160hz(self) -> &'a mut W {
        self.variant(DCRM_A::DCRM_CUTOFF_160HZ)
    }
    #[doc = "Cut-off frequency is OD_CLK/4167 (240 Hz at OD_CLK=1 MHz)"]
    #[inline(always)]
    pub fn dcrm_cutoff_240hz(self) -> &'a mut W {
        self.variant(DCRM_A::DCRM_CUTOFF_240HZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Sigma-delta modulator dithering enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DITHER_A {
    #[doc = "0: Dithering disabled"]
    DITHER_DISABLE = 0,
    #[doc = "1: Dithering enabled"]
    DITHER_ENABLE = 1,
}
impl From<DITHER_A> for bool {
    #[inline(always)]
    fn from(variant: DITHER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DITHER`"]
pub type DITHER_R = crate::R<bool, DITHER_A>;
impl DITHER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DITHER_A {
        match self.bits {
            false => DITHER_A::DITHER_DISABLE,
            true => DITHER_A::DITHER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DITHER_DISABLE`"]
    #[inline(always)]
    pub fn is_dither_disable(&self) -> bool {
        *self == DITHER_A::DITHER_DISABLE
    }
    #[doc = "Checks if the value of the field is `DITHER_ENABLE`"]
    #[inline(always)]
    pub fn is_dither_enable(&self) -> bool {
        *self == DITHER_A::DITHER_ENABLE
    }
}
#[doc = "Write proxy for field `DITHER`"]
pub struct DITHER_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DITHER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Dithering disabled"]
    #[inline(always)]
    pub fn dither_disable(self) -> &'a mut W {
        self.variant(DITHER_A::DITHER_DISABLE)
    }
    #[doc = "Dithering enabled"]
    #[inline(always)]
    pub fn dither_enable(self) -> &'a mut W {
        self.variant(DITHER_A::DITHER_ENABLE)
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
#[doc = "Output driver output clock edge\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_EDGE_A {
    #[doc = "0: Output OD data on the falling OD clock"]
    OD_FALLING_EDGE = 0,
    #[doc = "1: Output OD data on the rising OD clock"]
    OD_RISING_EDGE = 1,
}
impl From<CLK_EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_EDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLK_EDGE`"]
pub type CLK_EDGE_R = crate::R<bool, CLK_EDGE_A>;
impl CLK_EDGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_EDGE_A {
        match self.bits {
            false => CLK_EDGE_A::OD_FALLING_EDGE,
            true => CLK_EDGE_A::OD_RISING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `OD_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_od_falling_edge(&self) -> bool {
        *self == CLK_EDGE_A::OD_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `OD_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_od_rising_edge(&self) -> bool {
        *self == CLK_EDGE_A::OD_RISING_EDGE
    }
}
#[doc = "Write proxy for field `CLK_EDGE`"]
pub struct CLK_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_EDGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output OD data on the falling OD clock"]
    #[inline(always)]
    pub fn od_falling_edge(self) -> &'a mut W {
        self.variant(CLK_EDGE_A::OD_FALLING_EDGE)
    }
    #[doc = "Output OD data on the rising OD clock"]
    #[inline(always)]
    pub fn od_rising_edge(self) -> &'a mut W {
        self.variant(CLK_EDGE_A::OD_RISING_EDGE)
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
    #[doc = "Bits 16:19 - Output driver DC removal filter enable and cut-off frequency"]
    #[inline(always)]
    pub fn dcrm(&self) -> DCRM_R {
        DCRM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - Sigma-delta modulator dithering enable"]
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Output driver output clock edge"]
    #[inline(always)]
    pub fn clk_edge(&self) -> CLK_EDGE_R {
        CLK_EDGE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:19 - Output driver DC removal filter enable and cut-off frequency"]
    #[inline(always)]
    pub fn dcrm(&mut self) -> DCRM_W {
        DCRM_W { w: self }
    }
    #[doc = "Bit 10 - Sigma-delta modulator dithering enable"]
    #[inline(always)]
    pub fn dither(&mut self) -> DITHER_W {
        DITHER_W { w: self }
    }
    #[doc = "Bit 0 - Output driver output clock edge"]
    #[inline(always)]
    pub fn clk_edge(&mut self) -> CLK_EDGE_W {
        CLK_EDGE_W { w: self }
    }
}
