#[doc = "Reader of register AUDIO_DMIC_CFG"]
pub type R = crate::R<u32, super::AUDIO_DMIC_CFG>;
#[doc = "Writer for register AUDIO_DMIC_CFG"]
pub type W = crate::W<u32, super::AUDIO_DMIC_CFG>;
#[doc = "Register AUDIO_DMIC_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::AUDIO_DMIC_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMIC1_FRAC_DELAY`"]
pub type DMIC1_FRAC_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMIC1_FRAC_DELAY`"]
pub struct DMIC1_FRAC_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC1_FRAC_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "DMIC1 delay (0 to 1.875 samples in steps of 0.125 samples)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMIC1_DELAY_A {
    #[doc = "0: Delay disabled"]
    DMIC1_DELAY_DISABLE = 0,
    #[doc = "1: Delay of 0.125 samples"]
    DMIC1_DELAY_0P125 = 1,
    #[doc = "2: Delay of 0.25 samples"]
    DMIC1_DELAY_0P25 = 2,
    #[doc = "3: Delay of 0.375 samples"]
    DMIC1_DELAY_0P375 = 3,
    #[doc = "4: Delay of 0.5 samples"]
    DMIC1_DELAY_0P5 = 4,
    #[doc = "5: Delay of 0.625 samples"]
    DMIC1_DELAY_0P625 = 5,
    #[doc = "6: Delay of 0.75 samples"]
    DMIC1_DELAY_0P75 = 6,
    #[doc = "7: Delay of 0.875 samples"]
    DMIC1_DELAY_0P875 = 7,
    #[doc = "8: Delay of 1 sample"]
    DMIC1_DELAY_1P0 = 8,
    #[doc = "9: Delay of 1.125 samples"]
    DMIC1_DELAY_1P125 = 9,
    #[doc = "10: Delay of 1.25 samples"]
    DMIC1_DELAY_1P25 = 10,
    #[doc = "11: Delay of 1.375 samples"]
    DMIC1_DELAY_1P375 = 11,
    #[doc = "12: Delay of 1.5 samples"]
    DMIC1_DELAY_1P5 = 12,
    #[doc = "13: Delay of 1.625 samples"]
    DMIC1_DELAY_1P625 = 13,
    #[doc = "14: Delay of 1.75 samples"]
    DMIC1_DELAY_1P75 = 14,
    #[doc = "15: Delay of 1.875 samples"]
    DMIC1_DELAY_1P875 = 15,
}
impl From<DMIC1_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: DMIC1_DELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMIC1_DELAY`"]
pub type DMIC1_DELAY_R = crate::R<u8, DMIC1_DELAY_A>;
impl DMIC1_DELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC1_DELAY_A {
        match self.bits {
            0 => DMIC1_DELAY_A::DMIC1_DELAY_DISABLE,
            1 => DMIC1_DELAY_A::DMIC1_DELAY_0P125,
            2 => DMIC1_DELAY_A::DMIC1_DELAY_0P25,
            3 => DMIC1_DELAY_A::DMIC1_DELAY_0P375,
            4 => DMIC1_DELAY_A::DMIC1_DELAY_0P5,
            5 => DMIC1_DELAY_A::DMIC1_DELAY_0P625,
            6 => DMIC1_DELAY_A::DMIC1_DELAY_0P75,
            7 => DMIC1_DELAY_A::DMIC1_DELAY_0P875,
            8 => DMIC1_DELAY_A::DMIC1_DELAY_1P0,
            9 => DMIC1_DELAY_A::DMIC1_DELAY_1P125,
            10 => DMIC1_DELAY_A::DMIC1_DELAY_1P25,
            11 => DMIC1_DELAY_A::DMIC1_DELAY_1P375,
            12 => DMIC1_DELAY_A::DMIC1_DELAY_1P5,
            13 => DMIC1_DELAY_A::DMIC1_DELAY_1P625,
            14 => DMIC1_DELAY_A::DMIC1_DELAY_1P75,
            15 => DMIC1_DELAY_A::DMIC1_DELAY_1P875,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMIC1_DELAY_DISABLE`"]
    #[inline(always)]
    pub fn is_dmic1_delay_disable(&self) -> bool {
        *self == DMIC1_DELAY_A::DMIC1_DELAY_DISABLE
    }
    #[doc = "Checks if the value of the field is `DMIC1_DELAY_0P125`"]
    #[inline(always)]
    pub fn is_dmic1_delay_0p125(&self) -> bool {
        *self == DMIC1_DELAY_A::DMIC1_DELAY_0P125
    }
    #[doc = "Checks if the value of the field is `DMIC1_DELAY_0P25`"]
    #[inline(always)]
    pub fn is_dmic1_delay_0p25(&self) -> bool {
        *self == DMIC1_DELAY_A::DMIC1_DELAY_0P25
    }
    #[doc = "Checks if the value of the field is `DMIC1_DELAY_0P375`"]
    #[inline(always)]
    pub fn is_dmic1_delay_0p375(&self) -> bool {
        *self == DMIC1_DELAY_A::DMIC1_DELAY_0P375
    }
    #[doc = "Checks if the value of the field is `DMIC1_DELAY_0P5`"]
    #[inline(always)]
    pub fn is_dmic1_delay_0p5(&self) -> bool {
        *self == DMIC1_DELAY_A::DMIC1_DELAY_0P5
    }
    #[doc = "Checks if the value of the field is `DMIC1_DELAY_0P625`"]
    #[inline(always)]
    pub fn is_dmic1_delay_0p625(&self) -> bool {
        *self == DMIC1_DELAY_A::DMIC1_DELAY_0P625
    }
    #[doc = "Checks if the value of the field is `DMIC1_DELAY_0P75`"]
    #[inline(always)]
    pub fn is_dmic1_delay_0p75(&self) -> bool {
        *self == DMIC1_DELAY_A::DMIC1_DELAY_0P75
    }
    #[doc = "Checks if the value of the field is `DMIC1_DELAY_0P875`"]
    #[inline(always)]
    pub fn is_dmic1_delay_0p875(&self) -> bool {
        *self == DMIC1_DELAY_A::DMIC1_DELAY_0P875
    }
    #[doc = "Checks if the value of the field is `DMIC1_DELAY_1P0`"]
    #[inline(always)]
    pub fn is_dmic1_delay_1p0(&self) -> bool {
        *self == DMIC1_DELAY_A::DMIC1_DELAY_1P0
    }
    #[doc = "Checks if the value of the field is `DMIC1_DELAY_1P125`"]
    #[inline(always)]
    pub fn is_dmic1_delay_1p125(&self) -> bool {
        *self == DMIC1_DELAY_A::DMIC1_DELAY_1P125
    }
    #[doc = "Checks if the value of the field is `DMIC1_DELAY_1P25`"]
    #[inline(always)]
    pub fn is_dmic1_delay_1p25(&self) -> bool {
        *self == DMIC1_DELAY_A::DMIC1_DELAY_1P25
    }
    #[doc = "Checks if the value of the field is `DMIC1_DELAY_1P375`"]
    #[inline(always)]
    pub fn is_dmic1_delay_1p375(&self) -> bool {
        *self == DMIC1_DELAY_A::DMIC1_DELAY_1P375
    }
    #[doc = "Checks if the value of the field is `DMIC1_DELAY_1P5`"]
    #[inline(always)]
    pub fn is_dmic1_delay_1p5(&self) -> bool {
        *self == DMIC1_DELAY_A::DMIC1_DELAY_1P5
    }
    #[doc = "Checks if the value of the field is `DMIC1_DELAY_1P625`"]
    #[inline(always)]
    pub fn is_dmic1_delay_1p625(&self) -> bool {
        *self == DMIC1_DELAY_A::DMIC1_DELAY_1P625
    }
    #[doc = "Checks if the value of the field is `DMIC1_DELAY_1P75`"]
    #[inline(always)]
    pub fn is_dmic1_delay_1p75(&self) -> bool {
        *self == DMIC1_DELAY_A::DMIC1_DELAY_1P75
    }
    #[doc = "Checks if the value of the field is `DMIC1_DELAY_1P875`"]
    #[inline(always)]
    pub fn is_dmic1_delay_1p875(&self) -> bool {
        *self == DMIC1_DELAY_A::DMIC1_DELAY_1P875
    }
}
#[doc = "Write proxy for field `DMIC1_DELAY`"]
pub struct DMIC1_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC1_DELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC1_DELAY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Delay disabled"]
    #[inline(always)]
    pub fn dmic1_delay_disable(self) -> &'a mut W {
        self.variant(DMIC1_DELAY_A::DMIC1_DELAY_DISABLE)
    }
    #[doc = "Delay of 0.125 samples"]
    #[inline(always)]
    pub fn dmic1_delay_0p125(self) -> &'a mut W {
        self.variant(DMIC1_DELAY_A::DMIC1_DELAY_0P125)
    }
    #[doc = "Delay of 0.25 samples"]
    #[inline(always)]
    pub fn dmic1_delay_0p25(self) -> &'a mut W {
        self.variant(DMIC1_DELAY_A::DMIC1_DELAY_0P25)
    }
    #[doc = "Delay of 0.375 samples"]
    #[inline(always)]
    pub fn dmic1_delay_0p375(self) -> &'a mut W {
        self.variant(DMIC1_DELAY_A::DMIC1_DELAY_0P375)
    }
    #[doc = "Delay of 0.5 samples"]
    #[inline(always)]
    pub fn dmic1_delay_0p5(self) -> &'a mut W {
        self.variant(DMIC1_DELAY_A::DMIC1_DELAY_0P5)
    }
    #[doc = "Delay of 0.625 samples"]
    #[inline(always)]
    pub fn dmic1_delay_0p625(self) -> &'a mut W {
        self.variant(DMIC1_DELAY_A::DMIC1_DELAY_0P625)
    }
    #[doc = "Delay of 0.75 samples"]
    #[inline(always)]
    pub fn dmic1_delay_0p75(self) -> &'a mut W {
        self.variant(DMIC1_DELAY_A::DMIC1_DELAY_0P75)
    }
    #[doc = "Delay of 0.875 samples"]
    #[inline(always)]
    pub fn dmic1_delay_0p875(self) -> &'a mut W {
        self.variant(DMIC1_DELAY_A::DMIC1_DELAY_0P875)
    }
    #[doc = "Delay of 1 sample"]
    #[inline(always)]
    pub fn dmic1_delay_1p0(self) -> &'a mut W {
        self.variant(DMIC1_DELAY_A::DMIC1_DELAY_1P0)
    }
    #[doc = "Delay of 1.125 samples"]
    #[inline(always)]
    pub fn dmic1_delay_1p125(self) -> &'a mut W {
        self.variant(DMIC1_DELAY_A::DMIC1_DELAY_1P125)
    }
    #[doc = "Delay of 1.25 samples"]
    #[inline(always)]
    pub fn dmic1_delay_1p25(self) -> &'a mut W {
        self.variant(DMIC1_DELAY_A::DMIC1_DELAY_1P25)
    }
    #[doc = "Delay of 1.375 samples"]
    #[inline(always)]
    pub fn dmic1_delay_1p375(self) -> &'a mut W {
        self.variant(DMIC1_DELAY_A::DMIC1_DELAY_1P375)
    }
    #[doc = "Delay of 1.5 samples"]
    #[inline(always)]
    pub fn dmic1_delay_1p5(self) -> &'a mut W {
        self.variant(DMIC1_DELAY_A::DMIC1_DELAY_1P5)
    }
    #[doc = "Delay of 1.625 samples"]
    #[inline(always)]
    pub fn dmic1_delay_1p625(self) -> &'a mut W {
        self.variant(DMIC1_DELAY_A::DMIC1_DELAY_1P625)
    }
    #[doc = "Delay of 1.75 samples"]
    #[inline(always)]
    pub fn dmic1_delay_1p75(self) -> &'a mut W {
        self.variant(DMIC1_DELAY_A::DMIC1_DELAY_1P75)
    }
    #[doc = "Delay of 1.875 samples"]
    #[inline(always)]
    pub fn dmic1_delay_1p875(self) -> &'a mut W {
        self.variant(DMIC1_DELAY_A::DMIC1_DELAY_1P875)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "DMIC1 DC removal filter enable and cut-off frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMIC1_DCRM_A {
    #[doc = "0: Cut-off frequency is Fs/3200 (5 Hz at Fs=16 kHz)"]
    DMIC1_DCRM_CUTOFF_5HZ = 0,
    #[doc = "1: Cut-off frequency is Fs/1600 (10 Hz at Fs=16 kHz)"]
    DMIC1_DCRM_CUTOFF_10HZ = 1,
    #[doc = "2: Cut-off frequency is Fs/800 (20 Hz at Fs=16 kHz)"]
    DMIC1_DCRM_CUTOFF_20HZ = 2,
    #[doc = "3: Cut-off frequency is Fs/400 (40 Hz at Fs=16 kHz)"]
    DMIC1_DCRM_CUTOFF_40HZ = 3,
    #[doc = "4: Cut-off frequency is Fs/200 (80 Hz at Fs=16 kHz)"]
    DMIC1_DCRM_CUTOFF_80HZ = 4,
    #[doc = "5: Cut-off frequency is Fs/100 (160 Hz at Fs=16 kHz)"]
    DMIC1_DCRM_CUTOFF_160HZ = 5,
    #[doc = "6: Cut-off frequency is Fs/50 (320 Hz at Fs=16 kHz)"]
    DMIC1_DCRM_CUTOFF_320HZ = 6,
    #[doc = "7: DC removal filter disabled"]
    DMIC1_DCRM_DISABLE = 7,
}
impl From<DMIC1_DCRM_A> for u8 {
    #[inline(always)]
    fn from(variant: DMIC1_DCRM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMIC1_DCRM`"]
pub type DMIC1_DCRM_R = crate::R<u8, DMIC1_DCRM_A>;
impl DMIC1_DCRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC1_DCRM_A {
        match self.bits {
            0 => DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_5HZ,
            1 => DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_10HZ,
            2 => DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_20HZ,
            3 => DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_40HZ,
            4 => DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_80HZ,
            5 => DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_160HZ,
            6 => DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_320HZ,
            7 => DMIC1_DCRM_A::DMIC1_DCRM_DISABLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMIC1_DCRM_CUTOFF_5HZ`"]
    #[inline(always)]
    pub fn is_dmic1_dcrm_cutoff_5hz(&self) -> bool {
        *self == DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_5HZ
    }
    #[doc = "Checks if the value of the field is `DMIC1_DCRM_CUTOFF_10HZ`"]
    #[inline(always)]
    pub fn is_dmic1_dcrm_cutoff_10hz(&self) -> bool {
        *self == DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_10HZ
    }
    #[doc = "Checks if the value of the field is `DMIC1_DCRM_CUTOFF_20HZ`"]
    #[inline(always)]
    pub fn is_dmic1_dcrm_cutoff_20hz(&self) -> bool {
        *self == DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_20HZ
    }
    #[doc = "Checks if the value of the field is `DMIC1_DCRM_CUTOFF_40HZ`"]
    #[inline(always)]
    pub fn is_dmic1_dcrm_cutoff_40hz(&self) -> bool {
        *self == DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_40HZ
    }
    #[doc = "Checks if the value of the field is `DMIC1_DCRM_CUTOFF_80HZ`"]
    #[inline(always)]
    pub fn is_dmic1_dcrm_cutoff_80hz(&self) -> bool {
        *self == DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_80HZ
    }
    #[doc = "Checks if the value of the field is `DMIC1_DCRM_CUTOFF_160HZ`"]
    #[inline(always)]
    pub fn is_dmic1_dcrm_cutoff_160hz(&self) -> bool {
        *self == DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_160HZ
    }
    #[doc = "Checks if the value of the field is `DMIC1_DCRM_CUTOFF_320HZ`"]
    #[inline(always)]
    pub fn is_dmic1_dcrm_cutoff_320hz(&self) -> bool {
        *self == DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_320HZ
    }
    #[doc = "Checks if the value of the field is `DMIC1_DCRM_DISABLE`"]
    #[inline(always)]
    pub fn is_dmic1_dcrm_disable(&self) -> bool {
        *self == DMIC1_DCRM_A::DMIC1_DCRM_DISABLE
    }
}
#[doc = "Write proxy for field `DMIC1_DCRM`"]
pub struct DMIC1_DCRM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC1_DCRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC1_DCRM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Cut-off frequency is Fs/3200 (5 Hz at Fs=16 kHz)"]
    #[inline(always)]
    pub fn dmic1_dcrm_cutoff_5hz(self) -> &'a mut W {
        self.variant(DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_5HZ)
    }
    #[doc = "Cut-off frequency is Fs/1600 (10 Hz at Fs=16 kHz)"]
    #[inline(always)]
    pub fn dmic1_dcrm_cutoff_10hz(self) -> &'a mut W {
        self.variant(DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_10HZ)
    }
    #[doc = "Cut-off frequency is Fs/800 (20 Hz at Fs=16 kHz)"]
    #[inline(always)]
    pub fn dmic1_dcrm_cutoff_20hz(self) -> &'a mut W {
        self.variant(DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_20HZ)
    }
    #[doc = "Cut-off frequency is Fs/400 (40 Hz at Fs=16 kHz)"]
    #[inline(always)]
    pub fn dmic1_dcrm_cutoff_40hz(self) -> &'a mut W {
        self.variant(DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_40HZ)
    }
    #[doc = "Cut-off frequency is Fs/200 (80 Hz at Fs=16 kHz)"]
    #[inline(always)]
    pub fn dmic1_dcrm_cutoff_80hz(self) -> &'a mut W {
        self.variant(DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_80HZ)
    }
    #[doc = "Cut-off frequency is Fs/100 (160 Hz at Fs=16 kHz)"]
    #[inline(always)]
    pub fn dmic1_dcrm_cutoff_160hz(self) -> &'a mut W {
        self.variant(DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_160HZ)
    }
    #[doc = "Cut-off frequency is Fs/50 (320 Hz at Fs=16 kHz)"]
    #[inline(always)]
    pub fn dmic1_dcrm_cutoff_320hz(self) -> &'a mut W {
        self.variant(DMIC1_DCRM_A::DMIC1_DCRM_CUTOFF_320HZ)
    }
    #[doc = "DC removal filter disabled"]
    #[inline(always)]
    pub fn dmic1_dcrm_disable(self) -> &'a mut W {
        self.variant(DMIC1_DCRM_A::DMIC1_DCRM_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "DMIC0 DC removal filter enable and cut-off frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMIC0_DCRM_A {
    #[doc = "0: Cut-off frequency is Fs/3200 (5 Hz at Fs=16 kHz)"]
    DMIC0_DCRM_CUTOFF_5HZ = 0,
    #[doc = "1: Cut-off frequency is Fs/1600 (10 Hz at Fs=16 kHz)"]
    DMIC0_DCRM_CUTOFF_10HZ = 1,
    #[doc = "2: Cut-off frequency is Fs/800 (20 Hz at Fs=16 kHz)"]
    DMIC0_DCRM_CUTOFF_20HZ = 2,
    #[doc = "3: Cut-off frequency is Fs/400 (40 Hz at Fs=16 kHz)"]
    DMIC0_DCRM_CUTOFF_40HZ = 3,
    #[doc = "4: Cut-off frequency is Fs/200 (80 Hz at Fs=16 kHz)"]
    DMIC0_DCRM_CUTOFF_80HZ = 4,
    #[doc = "5: Cut-off frequency is Fs/100 (160 Hz at Fs=16 kHz)"]
    DMIC0_DCRM_CUTOFF_160HZ = 5,
    #[doc = "6: Cut-off frequency is Fs/50 (320 Hz at Fs=16 kHz)"]
    DMIC0_DCRM_CUTOFF_320HZ = 6,
    #[doc = "7: DC removal filter disabled"]
    DMIC0_DCRM_DISABLE = 7,
}
impl From<DMIC0_DCRM_A> for u8 {
    #[inline(always)]
    fn from(variant: DMIC0_DCRM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMIC0_DCRM`"]
pub type DMIC0_DCRM_R = crate::R<u8, DMIC0_DCRM_A>;
impl DMIC0_DCRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_DCRM_A {
        match self.bits {
            0 => DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_5HZ,
            1 => DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_10HZ,
            2 => DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_20HZ,
            3 => DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_40HZ,
            4 => DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_80HZ,
            5 => DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_160HZ,
            6 => DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_320HZ,
            7 => DMIC0_DCRM_A::DMIC0_DCRM_DISABLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMIC0_DCRM_CUTOFF_5HZ`"]
    #[inline(always)]
    pub fn is_dmic0_dcrm_cutoff_5hz(&self) -> bool {
        *self == DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_5HZ
    }
    #[doc = "Checks if the value of the field is `DMIC0_DCRM_CUTOFF_10HZ`"]
    #[inline(always)]
    pub fn is_dmic0_dcrm_cutoff_10hz(&self) -> bool {
        *self == DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_10HZ
    }
    #[doc = "Checks if the value of the field is `DMIC0_DCRM_CUTOFF_20HZ`"]
    #[inline(always)]
    pub fn is_dmic0_dcrm_cutoff_20hz(&self) -> bool {
        *self == DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_20HZ
    }
    #[doc = "Checks if the value of the field is `DMIC0_DCRM_CUTOFF_40HZ`"]
    #[inline(always)]
    pub fn is_dmic0_dcrm_cutoff_40hz(&self) -> bool {
        *self == DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_40HZ
    }
    #[doc = "Checks if the value of the field is `DMIC0_DCRM_CUTOFF_80HZ`"]
    #[inline(always)]
    pub fn is_dmic0_dcrm_cutoff_80hz(&self) -> bool {
        *self == DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_80HZ
    }
    #[doc = "Checks if the value of the field is `DMIC0_DCRM_CUTOFF_160HZ`"]
    #[inline(always)]
    pub fn is_dmic0_dcrm_cutoff_160hz(&self) -> bool {
        *self == DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_160HZ
    }
    #[doc = "Checks if the value of the field is `DMIC0_DCRM_CUTOFF_320HZ`"]
    #[inline(always)]
    pub fn is_dmic0_dcrm_cutoff_320hz(&self) -> bool {
        *self == DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_320HZ
    }
    #[doc = "Checks if the value of the field is `DMIC0_DCRM_DISABLE`"]
    #[inline(always)]
    pub fn is_dmic0_dcrm_disable(&self) -> bool {
        *self == DMIC0_DCRM_A::DMIC0_DCRM_DISABLE
    }
}
#[doc = "Write proxy for field `DMIC0_DCRM`"]
pub struct DMIC0_DCRM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC0_DCRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC0_DCRM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Cut-off frequency is Fs/3200 (5 Hz at Fs=16 kHz)"]
    #[inline(always)]
    pub fn dmic0_dcrm_cutoff_5hz(self) -> &'a mut W {
        self.variant(DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_5HZ)
    }
    #[doc = "Cut-off frequency is Fs/1600 (10 Hz at Fs=16 kHz)"]
    #[inline(always)]
    pub fn dmic0_dcrm_cutoff_10hz(self) -> &'a mut W {
        self.variant(DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_10HZ)
    }
    #[doc = "Cut-off frequency is Fs/800 (20 Hz at Fs=16 kHz)"]
    #[inline(always)]
    pub fn dmic0_dcrm_cutoff_20hz(self) -> &'a mut W {
        self.variant(DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_20HZ)
    }
    #[doc = "Cut-off frequency is Fs/400 (40 Hz at Fs=16 kHz)"]
    #[inline(always)]
    pub fn dmic0_dcrm_cutoff_40hz(self) -> &'a mut W {
        self.variant(DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_40HZ)
    }
    #[doc = "Cut-off frequency is Fs/200 (80 Hz at Fs=16 kHz)"]
    #[inline(always)]
    pub fn dmic0_dcrm_cutoff_80hz(self) -> &'a mut W {
        self.variant(DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_80HZ)
    }
    #[doc = "Cut-off frequency is Fs/100 (160 Hz at Fs=16 kHz)"]
    #[inline(always)]
    pub fn dmic0_dcrm_cutoff_160hz(self) -> &'a mut W {
        self.variant(DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_160HZ)
    }
    #[doc = "Cut-off frequency is Fs/50 (320 Hz at Fs=16 kHz)"]
    #[inline(always)]
    pub fn dmic0_dcrm_cutoff_320hz(self) -> &'a mut W {
        self.variant(DMIC0_DCRM_A::DMIC0_DCRM_CUTOFF_320HZ)
    }
    #[doc = "DC removal filter disabled"]
    #[inline(always)]
    pub fn dmic0_dcrm_disable(self) -> &'a mut W {
        self.variant(DMIC0_DCRM_A::DMIC0_DCRM_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "DMIC1 input clock edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIC1_CLK_EDGE_A {
    #[doc = "0: Sample the DMIC1 input data on the falling DMIC clock"]
    DMIC1_FALLING_EDGE = 0,
    #[doc = "1: Sample the DMIC1 input data on the rising DMIC clock"]
    DMIC1_RISING_EDGE = 1,
}
impl From<DMIC1_CLK_EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC1_CLK_EDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMIC1_CLK_EDGE`"]
pub type DMIC1_CLK_EDGE_R = crate::R<bool, DMIC1_CLK_EDGE_A>;
impl DMIC1_CLK_EDGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC1_CLK_EDGE_A {
        match self.bits {
            false => DMIC1_CLK_EDGE_A::DMIC1_FALLING_EDGE,
            true => DMIC1_CLK_EDGE_A::DMIC1_RISING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC1_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_dmic1_falling_edge(&self) -> bool {
        *self == DMIC1_CLK_EDGE_A::DMIC1_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `DMIC1_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_dmic1_rising_edge(&self) -> bool {
        *self == DMIC1_CLK_EDGE_A::DMIC1_RISING_EDGE
    }
}
#[doc = "Write proxy for field `DMIC1_CLK_EDGE`"]
pub struct DMIC1_CLK_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC1_CLK_EDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC1_CLK_EDGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sample the DMIC1 input data on the falling DMIC clock"]
    #[inline(always)]
    pub fn dmic1_falling_edge(self) -> &'a mut W {
        self.variant(DMIC1_CLK_EDGE_A::DMIC1_FALLING_EDGE)
    }
    #[doc = "Sample the DMIC1 input data on the rising DMIC clock"]
    #[inline(always)]
    pub fn dmic1_rising_edge(self) -> &'a mut W {
        self.variant(DMIC1_CLK_EDGE_A::DMIC1_RISING_EDGE)
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
#[doc = "DMIC0 input clock edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIC0_CLK_EDGE_A {
    #[doc = "0: Sample the DMIC0 input data on the falling DMIC clock"]
    DMIC0_FALLING_EDGE = 0,
    #[doc = "1: Sample the DMIC0 input data on the rising DMIC clock"]
    DMIC0_RISING_EDGE = 1,
}
impl From<DMIC0_CLK_EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_CLK_EDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMIC0_CLK_EDGE`"]
pub type DMIC0_CLK_EDGE_R = crate::R<bool, DMIC0_CLK_EDGE_A>;
impl DMIC0_CLK_EDGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_CLK_EDGE_A {
        match self.bits {
            false => DMIC0_CLK_EDGE_A::DMIC0_FALLING_EDGE,
            true => DMIC0_CLK_EDGE_A::DMIC0_RISING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC0_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_dmic0_falling_edge(&self) -> bool {
        *self == DMIC0_CLK_EDGE_A::DMIC0_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `DMIC0_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_dmic0_rising_edge(&self) -> bool {
        *self == DMIC0_CLK_EDGE_A::DMIC0_RISING_EDGE
    }
}
#[doc = "Write proxy for field `DMIC0_CLK_EDGE`"]
pub struct DMIC0_CLK_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC0_CLK_EDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC0_CLK_EDGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sample the DMIC0 input data on the falling DMIC clock"]
    #[inline(always)]
    pub fn dmic0_falling_edge(self) -> &'a mut W {
        self.variant(DMIC0_CLK_EDGE_A::DMIC0_FALLING_EDGE)
    }
    #[doc = "Sample the DMIC0 input data on the rising DMIC clock"]
    #[inline(always)]
    pub fn dmic0_rising_edge(self) -> &'a mut W {
        self.variant(DMIC0_CLK_EDGE_A::DMIC0_RISING_EDGE)
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
    #[doc = "Bits 24:28 - DMIC1 fractional delay (each step represents a DMIC clock cycle)"]
    #[inline(always)]
    pub fn dmic1_frac_delay(&self) -> DMIC1_FRAC_DELAY_R {
        DMIC1_FRAC_DELAY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - DMIC1 delay (0 to 1.875 samples in steps of 0.125 samples)"]
    #[inline(always)]
    pub fn dmic1_delay(&self) -> DMIC1_DELAY_R {
        DMIC1_DELAY_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - DMIC1 DC removal filter enable and cut-off frequency"]
    #[inline(always)]
    pub fn dmic1_dcrm(&self) -> DMIC1_DCRM_R {
        DMIC1_DCRM_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - DMIC0 DC removal filter enable and cut-off frequency"]
    #[inline(always)]
    pub fn dmic0_dcrm(&self) -> DMIC0_DCRM_R {
        DMIC0_DCRM_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 1 - DMIC1 input clock edge"]
    #[inline(always)]
    pub fn dmic1_clk_edge(&self) -> DMIC1_CLK_EDGE_R {
        DMIC1_CLK_EDGE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DMIC0 input clock edge"]
    #[inline(always)]
    pub fn dmic0_clk_edge(&self) -> DMIC0_CLK_EDGE_R {
        DMIC0_CLK_EDGE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:28 - DMIC1 fractional delay (each step represents a DMIC clock cycle)"]
    #[inline(always)]
    pub fn dmic1_frac_delay(&mut self) -> DMIC1_FRAC_DELAY_W {
        DMIC1_FRAC_DELAY_W { w: self }
    }
    #[doc = "Bits 16:19 - DMIC1 delay (0 to 1.875 samples in steps of 0.125 samples)"]
    #[inline(always)]
    pub fn dmic1_delay(&mut self) -> DMIC1_DELAY_W {
        DMIC1_DELAY_W { w: self }
    }
    #[doc = "Bits 12:14 - DMIC1 DC removal filter enable and cut-off frequency"]
    #[inline(always)]
    pub fn dmic1_dcrm(&mut self) -> DMIC1_DCRM_W {
        DMIC1_DCRM_W { w: self }
    }
    #[doc = "Bits 8:10 - DMIC0 DC removal filter enable and cut-off frequency"]
    #[inline(always)]
    pub fn dmic0_dcrm(&mut self) -> DMIC0_DCRM_W {
        DMIC0_DCRM_W { w: self }
    }
    #[doc = "Bit 1 - DMIC1 input clock edge"]
    #[inline(always)]
    pub fn dmic1_clk_edge(&mut self) -> DMIC1_CLK_EDGE_W {
        DMIC1_CLK_EDGE_W { w: self }
    }
    #[doc = "Bit 0 - DMIC0 input clock edge"]
    #[inline(always)]
    pub fn dmic0_clk_edge(&mut self) -> DMIC0_CLK_EDGE_W {
        DMIC0_CLK_EDGE_W { w: self }
    }
}
