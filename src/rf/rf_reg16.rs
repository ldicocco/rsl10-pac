#[doc = "Reader of register RF_REG16"]
pub type R = crate::R<u32, super::RF_REG16>;
#[doc = "Writer for register RF_REG16"]
pub type W = crate::W<u32, super::RF_REG16>;
#[doc = "Register RF_REG16 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG16 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "IF value for the phase resampler.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_IF_RESAMPLE_PH_IF_A {
    #[doc = "0: `0`"]
    RX_IF_RESAMPLE_PH_IF_DEFAULT = 0,
}
impl From<RX_IF_RESAMPLE_PH_IF_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_IF_RESAMPLE_PH_IF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RX_IF_RESAMPLE_PH_IF`"]
pub type RX_IF_RESAMPLE_PH_IF_R = crate::R<u8, RX_IF_RESAMPLE_PH_IF_A>;
impl RX_IF_RESAMPLE_PH_IF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RX_IF_RESAMPLE_PH_IF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RX_IF_RESAMPLE_PH_IF_A::RX_IF_RESAMPLE_PH_IF_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX_IF_RESAMPLE_PH_IF_DEFAULT`"]
    #[inline(always)]
    pub fn is_rx_if_resample_ph_if_default(&self) -> bool {
        *self == RX_IF_RESAMPLE_PH_IF_A::RX_IF_RESAMPLE_PH_IF_DEFAULT
    }
}
#[doc = "Write proxy for field `RX_IF_RESAMPLE_PH_IF`"]
pub struct RX_IF_RESAMPLE_PH_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_IF_RESAMPLE_PH_IF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_IF_RESAMPLE_PH_IF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rx_if_resample_ph_if_default(self) -> &'a mut W {
        self.variant(RX_IF_RESAMPLE_PH_IF_A::RX_IF_RESAMPLE_PH_IF_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | (((value as u32) & 0x0f) << 25);
        self.w
    }
}
#[doc = "IF value for the carrier recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum RX_IF_IF2_CLK_OS_A {
    #[doc = "0: `0`"]
    RX_IF_IF2_CLK_OS_DEFAULT = 0,
}
impl From<RX_IF_IF2_CLK_OS_A> for u16 {
    #[inline(always)]
    fn from(variant: RX_IF_IF2_CLK_OS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RX_IF_IF2_CLK_OS`"]
pub type RX_IF_IF2_CLK_OS_R = crate::R<u16, RX_IF_IF2_CLK_OS_A>;
impl RX_IF_IF2_CLK_OS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, RX_IF_IF2_CLK_OS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RX_IF_IF2_CLK_OS_A::RX_IF_IF2_CLK_OS_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX_IF_IF2_CLK_OS_DEFAULT`"]
    #[inline(always)]
    pub fn is_rx_if_if2_clk_os_default(&self) -> bool {
        *self == RX_IF_IF2_CLK_OS_A::RX_IF_IF2_CLK_OS_DEFAULT
    }
}
#[doc = "Write proxy for field `RX_IF_IF2_CLK_OS`"]
pub struct RX_IF_IF2_CLK_OS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_IF_IF2_CLK_OS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_IF_IF2_CLK_OS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rx_if_if2_clk_os_default(self) -> &'a mut W {
        self.variant(RX_IF_IF2_CLK_OS_A::RX_IF_IF2_CLK_OS_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
#[doc = "FSK amplitude 1 (lowest): in FSK w/o ISI is used to specify the expected amplitude. In 4FSK is the lowest amplitude (+/-1). in FSK w/ ISI it specify the lowest amplitude (generally it corresponds to a sequence 0-1-0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FSK_FCR_AMP_1_FSK_FCR_AMP1_A {
    #[doc = "0: `0`"]
    FSK_FCR_AMP_1_FSK_FCR_AMP1_DEFAULT = 0,
}
impl From<FSK_FCR_AMP_1_FSK_FCR_AMP1_A> for u8 {
    #[inline(always)]
    fn from(variant: FSK_FCR_AMP_1_FSK_FCR_AMP1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FSK_FCR_AMP_1_FSK_FCR_AMP1`"]
pub type FSK_FCR_AMP_1_FSK_FCR_AMP1_R = crate::R<u8, FSK_FCR_AMP_1_FSK_FCR_AMP1_A>;
impl FSK_FCR_AMP_1_FSK_FCR_AMP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FSK_FCR_AMP_1_FSK_FCR_AMP1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FSK_FCR_AMP_1_FSK_FCR_AMP1_A::FSK_FCR_AMP_1_FSK_FCR_AMP1_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FSK_FCR_AMP_1_FSK_FCR_AMP1_DEFAULT`"]
    #[inline(always)]
    pub fn is_fsk_fcr_amp_1_fsk_fcr_amp1_default(&self) -> bool {
        *self == FSK_FCR_AMP_1_FSK_FCR_AMP1_A::FSK_FCR_AMP_1_FSK_FCR_AMP1_DEFAULT
    }
}
#[doc = "Write proxy for field `FSK_FCR_AMP_1_FSK_FCR_AMP1`"]
pub struct FSK_FCR_AMP_1_FSK_FCR_AMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> FSK_FCR_AMP_1_FSK_FCR_AMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSK_FCR_AMP_1_FSK_FCR_AMP1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fsk_fcr_amp_1_fsk_fcr_amp1_default(self) -> &'a mut W {
        self.variant(FSK_FCR_AMP_1_FSK_FCR_AMP1_A::FSK_FCR_AMP_1_FSK_FCR_AMP1_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Set the data-rate recovery limits: 00 => 0 percent, 01 => 3.125 percent, 10 => 6.25 percent, 11 => 12.5 percent\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILTER_GAIN_DR_LIMIT_A {
    #[doc = "0: `0`"]
    FILTER_GAIN_DR_LIMIT_DEFAULT = 0,
}
impl From<FILTER_GAIN_DR_LIMIT_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_GAIN_DR_LIMIT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FILTER_GAIN_DR_LIMIT`"]
pub type FILTER_GAIN_DR_LIMIT_R = crate::R<u8, FILTER_GAIN_DR_LIMIT_A>;
impl FILTER_GAIN_DR_LIMIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FILTER_GAIN_DR_LIMIT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FILTER_GAIN_DR_LIMIT_A::FILTER_GAIN_DR_LIMIT_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FILTER_GAIN_DR_LIMIT_DEFAULT`"]
    #[inline(always)]
    pub fn is_filter_gain_dr_limit_default(&self) -> bool {
        *self == FILTER_GAIN_DR_LIMIT_A::FILTER_GAIN_DR_LIMIT_DEFAULT
    }
}
#[doc = "Write proxy for field `FILTER_GAIN_DR_LIMIT`"]
pub struct FILTER_GAIN_DR_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_GAIN_DR_LIMIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTER_GAIN_DR_LIMIT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn filter_gain_dr_limit_default(self) -> &'a mut W {
        self.variant(FILTER_GAIN_DR_LIMIT_A::FILTER_GAIN_DR_LIMIT_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Mantissa of the final stage gain of the matched filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILTER_GAIN_FILTER_GAIN_M_A {
    #[doc = "0: `0`"]
    FILTER_GAIN_FILTER_GAIN_M_DEFAULT = 0,
}
impl From<FILTER_GAIN_FILTER_GAIN_M_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_GAIN_FILTER_GAIN_M_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FILTER_GAIN_FILTER_GAIN_M`"]
pub type FILTER_GAIN_FILTER_GAIN_M_R = crate::R<u8, FILTER_GAIN_FILTER_GAIN_M_A>;
impl FILTER_GAIN_FILTER_GAIN_M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FILTER_GAIN_FILTER_GAIN_M_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FILTER_GAIN_FILTER_GAIN_M_A::FILTER_GAIN_FILTER_GAIN_M_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FILTER_GAIN_FILTER_GAIN_M_DEFAULT`"]
    #[inline(always)]
    pub fn is_filter_gain_filter_gain_m_default(&self) -> bool {
        *self == FILTER_GAIN_FILTER_GAIN_M_A::FILTER_GAIN_FILTER_GAIN_M_DEFAULT
    }
}
#[doc = "Write proxy for field `FILTER_GAIN_FILTER_GAIN_M`"]
pub struct FILTER_GAIN_FILTER_GAIN_M_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_GAIN_FILTER_GAIN_M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTER_GAIN_FILTER_GAIN_M_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn filter_gain_filter_gain_m_default(self) -> &'a mut W {
        self.variant(FILTER_GAIN_FILTER_GAIN_M_A::FILTER_GAIN_FILTER_GAIN_M_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Exponent of the final stage gain of the matched filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILTER_GAIN_FILTER_GAIN_E_A {
    #[doc = "0: `0`"]
    FILTER_GAIN_FILTER_GAIN_E_DEFAULT = 0,
}
impl From<FILTER_GAIN_FILTER_GAIN_E_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_GAIN_FILTER_GAIN_E_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FILTER_GAIN_FILTER_GAIN_E`"]
pub type FILTER_GAIN_FILTER_GAIN_E_R = crate::R<u8, FILTER_GAIN_FILTER_GAIN_E_A>;
impl FILTER_GAIN_FILTER_GAIN_E_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FILTER_GAIN_FILTER_GAIN_E_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FILTER_GAIN_FILTER_GAIN_E_A::FILTER_GAIN_FILTER_GAIN_E_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FILTER_GAIN_FILTER_GAIN_E_DEFAULT`"]
    #[inline(always)]
    pub fn is_filter_gain_filter_gain_e_default(&self) -> bool {
        *self == FILTER_GAIN_FILTER_GAIN_E_A::FILTER_GAIN_FILTER_GAIN_E_DEFAULT
    }
}
#[doc = "Write proxy for field `FILTER_GAIN_FILTER_GAIN_E`"]
pub struct FILTER_GAIN_FILTER_GAIN_E_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_GAIN_FILTER_GAIN_E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTER_GAIN_FILTER_GAIN_E_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn filter_gain_filter_gain_e_default(self) -> &'a mut W {
        self.variant(FILTER_GAIN_FILTER_GAIN_E_A::FILTER_GAIN_FILTER_GAIN_E_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:28 - IF value for the phase resampler."]
    #[inline(always)]
    pub fn rx_if_resample_ph_if(&self) -> RX_IF_RESAMPLE_PH_IF_R {
        RX_IF_RESAMPLE_PH_IF_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bits 16:24 - IF value for the carrier recovery"]
    #[inline(always)]
    pub fn rx_if_if2_clk_os(&self) -> RX_IF_IF2_CLK_OS_R {
        RX_IF_IF2_CLK_OS_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 8:15 - FSK amplitude 1 (lowest): in FSK w/o ISI is used to specify the expected amplitude. In 4FSK is the lowest amplitude (+/-1). in FSK w/ ISI it specify the lowest amplitude (generally it corresponds to a sequence 0-1-0."]
    #[inline(always)]
    pub fn fsk_fcr_amp_1_fsk_fcr_amp1(&self) -> FSK_FCR_AMP_1_FSK_FCR_AMP1_R {
        FSK_FCR_AMP_1_FSK_FCR_AMP1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 6:7 - Set the data-rate recovery limits: 00 => 0 percent, 01 => 3.125 percent, 10 => 6.25 percent, 11 => 12.5 percent"]
    #[inline(always)]
    pub fn filter_gain_dr_limit(&self) -> FILTER_GAIN_DR_LIMIT_R {
        FILTER_GAIN_DR_LIMIT_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:5 - Mantissa of the final stage gain of the matched filter"]
    #[inline(always)]
    pub fn filter_gain_filter_gain_m(&self) -> FILTER_GAIN_FILTER_GAIN_M_R {
        FILTER_GAIN_FILTER_GAIN_M_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Exponent of the final stage gain of the matched filter"]
    #[inline(always)]
    pub fn filter_gain_filter_gain_e(&self) -> FILTER_GAIN_FILTER_GAIN_E_R {
        FILTER_GAIN_FILTER_GAIN_E_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 25:28 - IF value for the phase resampler."]
    #[inline(always)]
    pub fn rx_if_resample_ph_if(&mut self) -> RX_IF_RESAMPLE_PH_IF_W {
        RX_IF_RESAMPLE_PH_IF_W { w: self }
    }
    #[doc = "Bits 16:24 - IF value for the carrier recovery"]
    #[inline(always)]
    pub fn rx_if_if2_clk_os(&mut self) -> RX_IF_IF2_CLK_OS_W {
        RX_IF_IF2_CLK_OS_W { w: self }
    }
    #[doc = "Bits 8:15 - FSK amplitude 1 (lowest): in FSK w/o ISI is used to specify the expected amplitude. In 4FSK is the lowest amplitude (+/-1). in FSK w/ ISI it specify the lowest amplitude (generally it corresponds to a sequence 0-1-0."]
    #[inline(always)]
    pub fn fsk_fcr_amp_1_fsk_fcr_amp1(&mut self) -> FSK_FCR_AMP_1_FSK_FCR_AMP1_W {
        FSK_FCR_AMP_1_FSK_FCR_AMP1_W { w: self }
    }
    #[doc = "Bits 6:7 - Set the data-rate recovery limits: 00 => 0 percent, 01 => 3.125 percent, 10 => 6.25 percent, 11 => 12.5 percent"]
    #[inline(always)]
    pub fn filter_gain_dr_limit(&mut self) -> FILTER_GAIN_DR_LIMIT_W {
        FILTER_GAIN_DR_LIMIT_W { w: self }
    }
    #[doc = "Bits 3:5 - Mantissa of the final stage gain of the matched filter"]
    #[inline(always)]
    pub fn filter_gain_filter_gain_m(&mut self) -> FILTER_GAIN_FILTER_GAIN_M_W {
        FILTER_GAIN_FILTER_GAIN_M_W { w: self }
    }
    #[doc = "Bits 0:2 - Exponent of the final stage gain of the matched filter"]
    #[inline(always)]
    pub fn filter_gain_filter_gain_e(&mut self) -> FILTER_GAIN_FILTER_GAIN_E_W {
        FILTER_GAIN_FILTER_GAIN_E_W { w: self }
    }
}
