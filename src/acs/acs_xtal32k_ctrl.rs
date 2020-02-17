#[doc = "Reader of register ACS_XTAL32K_CTRL"]
pub type R = crate::R<u32, super::ACS_XTAL32K_CTRL>;
#[doc = "Writer for register ACS_XTAL32K_CTRL"]
pub type W = crate::W<u32, super::ACS_XTAL32K_CTRL>;
#[doc = "Register ACS_XTAL32K_CTRL `reset()`'s with value 0x0970"]
impl crate::ResetValue for super::ACS_XTAL32K_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0970
    }
}
#[doc = "XTAL ready status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_A {
    #[doc = "0: XTAL 32K not available"]
    XTAL32K_NOT_OK = 0,
    #[doc = "1: XTAL 32K is OK"]
    XTAL32K_OK = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READY`"]
pub type READY_R = crate::R<bool, READY_A>;
impl READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::XTAL32K_NOT_OK,
            true => READY_A::XTAL32K_OK,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL32K_NOT_OK`"]
    #[inline(always)]
    pub fn is_xtal32k_not_ok(&self) -> bool {
        *self == READY_A::XTAL32K_NOT_OK
    }
    #[doc = "Checks if the value of the field is `XTAL32K_OK`"]
    #[inline(always)]
    pub fn is_xtal32k_ok(&self) -> bool {
        *self == READY_A::XTAL32K_OK
    }
}
#[doc = "Switch to bypass the added XIN serial cap to reduce the leakage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XIN_CAP_BYPASS_EN_A {
    #[doc = "0: Disable the XTAL bypass switch of the XIN serial cap"]
    XTAL32K_XIN_CAP_BYPASS_DISABLE = 0,
    #[doc = "1: Enable the XTAL bypass switch of the XIN serial cap"]
    XTAL32K_XIN_CAP_BYPASS_ENABLE = 1,
}
impl From<XIN_CAP_BYPASS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: XIN_CAP_BYPASS_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XIN_CAP_BYPASS_EN`"]
pub type XIN_CAP_BYPASS_EN_R = crate::R<bool, XIN_CAP_BYPASS_EN_A>;
impl XIN_CAP_BYPASS_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XIN_CAP_BYPASS_EN_A {
        match self.bits {
            false => XIN_CAP_BYPASS_EN_A::XTAL32K_XIN_CAP_BYPASS_DISABLE,
            true => XIN_CAP_BYPASS_EN_A::XTAL32K_XIN_CAP_BYPASS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL32K_XIN_CAP_BYPASS_DISABLE`"]
    #[inline(always)]
    pub fn is_xtal32k_xin_cap_bypass_disable(&self) -> bool {
        *self == XIN_CAP_BYPASS_EN_A::XTAL32K_XIN_CAP_BYPASS_DISABLE
    }
    #[doc = "Checks if the value of the field is `XTAL32K_XIN_CAP_BYPASS_ENABLE`"]
    #[inline(always)]
    pub fn is_xtal32k_xin_cap_bypass_enable(&self) -> bool {
        *self == XIN_CAP_BYPASS_EN_A::XTAL32K_XIN_CAP_BYPASS_ENABLE
    }
}
#[doc = "Write proxy for field `XIN_CAP_BYPASS_EN`"]
pub struct XIN_CAP_BYPASS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XIN_CAP_BYPASS_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XIN_CAP_BYPASS_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the XTAL bypass switch of the XIN serial cap"]
    #[inline(always)]
    pub fn xtal32k_xin_cap_bypass_disable(self) -> &'a mut W {
        self.variant(XIN_CAP_BYPASS_EN_A::XTAL32K_XIN_CAP_BYPASS_DISABLE)
    }
    #[doc = "Enable the XTAL bypass switch of the XIN serial cap"]
    #[inline(always)]
    pub fn xtal32k_xin_cap_bypass_enable(self) -> &'a mut W {
        self.variant(XIN_CAP_BYPASS_EN_A::XTAL32K_XIN_CAP_BYPASS_ENABLE)
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
#[doc = "XTAL enable amplitude control (regulation)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_AMPL_CTRL_A {
    #[doc = "0: XTAL 32K amplitude control disabled"]
    XTAL32K_AMPL_CTRL_DISABLE = 0,
    #[doc = "1: XTAL 32K amplitude control enabled"]
    XTAL32K_AMPL_CTRL_ENABLE = 1,
}
impl From<EN_AMPL_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: EN_AMPL_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN_AMPL_CTRL`"]
pub type EN_AMPL_CTRL_R = crate::R<bool, EN_AMPL_CTRL_A>;
impl EN_AMPL_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_AMPL_CTRL_A {
        match self.bits {
            false => EN_AMPL_CTRL_A::XTAL32K_AMPL_CTRL_DISABLE,
            true => EN_AMPL_CTRL_A::XTAL32K_AMPL_CTRL_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL32K_AMPL_CTRL_DISABLE`"]
    #[inline(always)]
    pub fn is_xtal32k_ampl_ctrl_disable(&self) -> bool {
        *self == EN_AMPL_CTRL_A::XTAL32K_AMPL_CTRL_DISABLE
    }
    #[doc = "Checks if the value of the field is `XTAL32K_AMPL_CTRL_ENABLE`"]
    #[inline(always)]
    pub fn is_xtal32k_ampl_ctrl_enable(&self) -> bool {
        *self == EN_AMPL_CTRL_A::XTAL32K_AMPL_CTRL_ENABLE
    }
}
#[doc = "Write proxy for field `EN_AMPL_CTRL`"]
pub struct EN_AMPL_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_AMPL_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_AMPL_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XTAL 32K amplitude control disabled"]
    #[inline(always)]
    pub fn xtal32k_ampl_ctrl_disable(self) -> &'a mut W {
        self.variant(EN_AMPL_CTRL_A::XTAL32K_AMPL_CTRL_DISABLE)
    }
    #[doc = "XTAL 32K amplitude control enabled"]
    #[inline(always)]
    pub fn xtal32k_ampl_ctrl_enable(self) -> &'a mut W {
        self.variant(EN_AMPL_CTRL_A::XTAL32K_AMPL_CTRL_ENABLE)
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
#[doc = "XTAL bypass the ready detector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCE_READY_A {
    #[doc = "0: XTAL 32K ready not forced"]
    XTAL32K_NOT_FORCE_READY = 0,
    #[doc = "1: XTAL 32K ready forced"]
    XTAL32K_FORCE_READY = 1,
}
impl From<FORCE_READY_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE_READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FORCE_READY`"]
pub type FORCE_READY_R = crate::R<bool, FORCE_READY_A>;
impl FORCE_READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCE_READY_A {
        match self.bits {
            false => FORCE_READY_A::XTAL32K_NOT_FORCE_READY,
            true => FORCE_READY_A::XTAL32K_FORCE_READY,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL32K_NOT_FORCE_READY`"]
    #[inline(always)]
    pub fn is_xtal32k_not_force_ready(&self) -> bool {
        *self == FORCE_READY_A::XTAL32K_NOT_FORCE_READY
    }
    #[doc = "Checks if the value of the field is `XTAL32K_FORCE_READY`"]
    #[inline(always)]
    pub fn is_xtal32k_force_ready(&self) -> bool {
        *self == FORCE_READY_A::XTAL32K_FORCE_READY
    }
}
#[doc = "Write proxy for field `FORCE_READY`"]
pub struct FORCE_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCE_READY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XTAL 32K ready not forced"]
    #[inline(always)]
    pub fn xtal32k_not_force_ready(self) -> &'a mut W {
        self.variant(FORCE_READY_A::XTAL32K_NOT_FORCE_READY)
    }
    #[doc = "XTAL 32K ready forced"]
    #[inline(always)]
    pub fn xtal32k_force_ready(self) -> &'a mut W {
        self.variant(FORCE_READY_A::XTAL32K_FORCE_READY)
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
#[doc = "XTAL load capacitance configuration\n\nValue on reset: 9"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLOAD_TRIM_A {
    #[doc = "0: 0 pF internal capacitor"]
    XTAL32K_CTRIM_0P0PF = 0,
    #[doc = "1: 0.4 pF internal capacitor"]
    XTAL32K_CTRIM_0P4PF = 1,
    #[doc = "9: 3.6 pF internal capacitor"]
    XTAL32K_CTRIM_3P6PF = 9,
    #[doc = "63: 25.2 pF internal capacitor"]
    XTAL32K_CTRIM_25P2PF = 63,
}
impl From<CLOAD_TRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: CLOAD_TRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLOAD_TRIM`"]
pub type CLOAD_TRIM_R = crate::R<u8, CLOAD_TRIM_A>;
impl CLOAD_TRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLOAD_TRIM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLOAD_TRIM_A::XTAL32K_CTRIM_0P0PF),
            1 => Val(CLOAD_TRIM_A::XTAL32K_CTRIM_0P4PF),
            9 => Val(CLOAD_TRIM_A::XTAL32K_CTRIM_3P6PF),
            63 => Val(CLOAD_TRIM_A::XTAL32K_CTRIM_25P2PF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL32K_CTRIM_0P0PF`"]
    #[inline(always)]
    pub fn is_xtal32k_ctrim_0p0pf(&self) -> bool {
        *self == CLOAD_TRIM_A::XTAL32K_CTRIM_0P0PF
    }
    #[doc = "Checks if the value of the field is `XTAL32K_CTRIM_0P4PF`"]
    #[inline(always)]
    pub fn is_xtal32k_ctrim_0p4pf(&self) -> bool {
        *self == CLOAD_TRIM_A::XTAL32K_CTRIM_0P4PF
    }
    #[doc = "Checks if the value of the field is `XTAL32K_CTRIM_3P6PF`"]
    #[inline(always)]
    pub fn is_xtal32k_ctrim_3p6pf(&self) -> bool {
        *self == CLOAD_TRIM_A::XTAL32K_CTRIM_3P6PF
    }
    #[doc = "Checks if the value of the field is `XTAL32K_CTRIM_25P2PF`"]
    #[inline(always)]
    pub fn is_xtal32k_ctrim_25p2pf(&self) -> bool {
        *self == CLOAD_TRIM_A::XTAL32K_CTRIM_25P2PF
    }
}
#[doc = "Write proxy for field `CLOAD_TRIM`"]
pub struct CLOAD_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOAD_TRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLOAD_TRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0 pF internal capacitor"]
    #[inline(always)]
    pub fn xtal32k_ctrim_0p0pf(self) -> &'a mut W {
        self.variant(CLOAD_TRIM_A::XTAL32K_CTRIM_0P0PF)
    }
    #[doc = "0.4 pF internal capacitor"]
    #[inline(always)]
    pub fn xtal32k_ctrim_0p4pf(self) -> &'a mut W {
        self.variant(CLOAD_TRIM_A::XTAL32K_CTRIM_0P4PF)
    }
    #[doc = "3.6 pF internal capacitor"]
    #[inline(always)]
    pub fn xtal32k_ctrim_3p6pf(self) -> &'a mut W {
        self.variant(CLOAD_TRIM_A::XTAL32K_CTRIM_3P6PF)
    }
    #[doc = "25.2 pF internal capacitor"]
    #[inline(always)]
    pub fn xtal32k_ctrim_25p2pf(self) -> &'a mut W {
        self.variant(CLOAD_TRIM_A::XTAL32K_CTRIM_25P2PF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "XTAL current trimming\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ITRIM_A {
    #[doc = "0: 20 nA startup current"]
    XTAL32K_ITRIM_20NA = 0,
    #[doc = "3: 80 nA startup current"]
    XTAL32K_ITRIM_80NA = 3,
    #[doc = "7: 160 nA startup current"]
    XTAL32K_ITRIM_160NA = 7,
    #[doc = "15: 320 nA startup current"]
    XTAL32K_ITRIM_320NA = 15,
}
impl From<ITRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: ITRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ITRIM`"]
pub type ITRIM_R = crate::R<u8, ITRIM_A>;
impl ITRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ITRIM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ITRIM_A::XTAL32K_ITRIM_20NA),
            3 => Val(ITRIM_A::XTAL32K_ITRIM_80NA),
            7 => Val(ITRIM_A::XTAL32K_ITRIM_160NA),
            15 => Val(ITRIM_A::XTAL32K_ITRIM_320NA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL32K_ITRIM_20NA`"]
    #[inline(always)]
    pub fn is_xtal32k_itrim_20na(&self) -> bool {
        *self == ITRIM_A::XTAL32K_ITRIM_20NA
    }
    #[doc = "Checks if the value of the field is `XTAL32K_ITRIM_80NA`"]
    #[inline(always)]
    pub fn is_xtal32k_itrim_80na(&self) -> bool {
        *self == ITRIM_A::XTAL32K_ITRIM_80NA
    }
    #[doc = "Checks if the value of the field is `XTAL32K_ITRIM_160NA`"]
    #[inline(always)]
    pub fn is_xtal32k_itrim_160na(&self) -> bool {
        *self == ITRIM_A::XTAL32K_ITRIM_160NA
    }
    #[doc = "Checks if the value of the field is `XTAL32K_ITRIM_320NA`"]
    #[inline(always)]
    pub fn is_xtal32k_itrim_320na(&self) -> bool {
        *self == ITRIM_A::XTAL32K_ITRIM_320NA
    }
}
#[doc = "Write proxy for field `ITRIM`"]
pub struct ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "20 nA startup current"]
    #[inline(always)]
    pub fn xtal32k_itrim_20na(self) -> &'a mut W {
        self.variant(ITRIM_A::XTAL32K_ITRIM_20NA)
    }
    #[doc = "80 nA startup current"]
    #[inline(always)]
    pub fn xtal32k_itrim_80na(self) -> &'a mut W {
        self.variant(ITRIM_A::XTAL32K_ITRIM_80NA)
    }
    #[doc = "160 nA startup current"]
    #[inline(always)]
    pub fn xtal32k_itrim_160na(self) -> &'a mut W {
        self.variant(ITRIM_A::XTAL32K_ITRIM_160NA)
    }
    #[doc = "320 nA startup current"]
    #[inline(always)]
    pub fn xtal32k_itrim_320na(self) -> &'a mut W {
        self.variant(ITRIM_A::XTAL32K_ITRIM_320NA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "XTAL current boosting (4x)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBOOST_A {
    #[doc = "0: Disable the XTAL 32 kHz current boosting mode"]
    XTAL32K_IBOOST_DISABLE = 0,
    #[doc = "1: Disable the XTAL 32 kHz enable boosting mode (4x itrim currents)"]
    XTAL32K_IBOOST_ENABLE = 1,
}
impl From<IBOOST_A> for bool {
    #[inline(always)]
    fn from(variant: IBOOST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IBOOST`"]
pub type IBOOST_R = crate::R<bool, IBOOST_A>;
impl IBOOST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBOOST_A {
        match self.bits {
            false => IBOOST_A::XTAL32K_IBOOST_DISABLE,
            true => IBOOST_A::XTAL32K_IBOOST_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL32K_IBOOST_DISABLE`"]
    #[inline(always)]
    pub fn is_xtal32k_iboost_disable(&self) -> bool {
        *self == IBOOST_A::XTAL32K_IBOOST_DISABLE
    }
    #[doc = "Checks if the value of the field is `XTAL32K_IBOOST_ENABLE`"]
    #[inline(always)]
    pub fn is_xtal32k_iboost_enable(&self) -> bool {
        *self == IBOOST_A::XTAL32K_IBOOST_ENABLE
    }
}
#[doc = "Write proxy for field `IBOOST`"]
pub struct IBOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> IBOOST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBOOST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the XTAL 32 kHz current boosting mode"]
    #[inline(always)]
    pub fn xtal32k_iboost_disable(self) -> &'a mut W {
        self.variant(IBOOST_A::XTAL32K_IBOOST_DISABLE)
    }
    #[doc = "Disable the XTAL 32 kHz enable boosting mode (4x itrim currents)"]
    #[inline(always)]
    pub fn xtal32k_iboost_enable(self) -> &'a mut W {
        self.variant(IBOOST_A::XTAL32K_IBOOST_ENABLE)
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
#[doc = "Enable the XTAL 32 kHz oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Disable the XTAL 32 kHz oscillator"]
    XTAL32K_DISABLE = 0,
    #[doc = "1: Enable the XTAL 32 kHz oscillator"]
    XTAL32K_ENABLE = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::XTAL32K_DISABLE,
            true => ENABLE_A::XTAL32K_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL32K_DISABLE`"]
    #[inline(always)]
    pub fn is_xtal32k_disable(&self) -> bool {
        *self == ENABLE_A::XTAL32K_DISABLE
    }
    #[doc = "Checks if the value of the field is `XTAL32K_ENABLE`"]
    #[inline(always)]
    pub fn is_xtal32k_enable(&self) -> bool {
        *self == ENABLE_A::XTAL32K_ENABLE
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the XTAL 32 kHz oscillator"]
    #[inline(always)]
    pub fn xtal32k_disable(self) -> &'a mut W {
        self.variant(ENABLE_A::XTAL32K_DISABLE)
    }
    #[doc = "Enable the XTAL 32 kHz oscillator"]
    #[inline(always)]
    pub fn xtal32k_enable(self) -> &'a mut W {
        self.variant(ENABLE_A::XTAL32K_ENABLE)
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
    #[doc = "Bit 24 - XTAL ready status"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Switch to bypass the added XIN serial cap to reduce the leakage"]
    #[inline(always)]
    pub fn xin_cap_bypass_en(&self) -> XIN_CAP_BYPASS_EN_R {
        XIN_CAP_BYPASS_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - XTAL enable amplitude control (regulation)"]
    #[inline(always)]
    pub fn en_ampl_ctrl(&self) -> EN_AMPL_CTRL_R {
        EN_AMPL_CTRL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - XTAL bypass the ready detector"]
    #[inline(always)]
    pub fn force_ready(&self) -> FORCE_READY_R {
        FORCE_READY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - XTAL load capacitance configuration"]
    #[inline(always)]
    pub fn cload_trim(&self) -> CLOAD_TRIM_R {
        CLOAD_TRIM_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 4:7 - XTAL current trimming"]
    #[inline(always)]
    pub fn itrim(&self) -> ITRIM_R {
        ITRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - XTAL current boosting (4x)"]
    #[inline(always)]
    pub fn iboost(&self) -> IBOOST_R {
        IBOOST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable the XTAL 32 kHz oscillator"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Switch to bypass the added XIN serial cap to reduce the leakage"]
    #[inline(always)]
    pub fn xin_cap_bypass_en(&mut self) -> XIN_CAP_BYPASS_EN_W {
        XIN_CAP_BYPASS_EN_W { w: self }
    }
    #[doc = "Bit 17 - XTAL enable amplitude control (regulation)"]
    #[inline(always)]
    pub fn en_ampl_ctrl(&mut self) -> EN_AMPL_CTRL_W {
        EN_AMPL_CTRL_W { w: self }
    }
    #[doc = "Bit 16 - XTAL bypass the ready detector"]
    #[inline(always)]
    pub fn force_ready(&mut self) -> FORCE_READY_W {
        FORCE_READY_W { w: self }
    }
    #[doc = "Bits 8:13 - XTAL load capacitance configuration"]
    #[inline(always)]
    pub fn cload_trim(&mut self) -> CLOAD_TRIM_W {
        CLOAD_TRIM_W { w: self }
    }
    #[doc = "Bits 4:7 - XTAL current trimming"]
    #[inline(always)]
    pub fn itrim(&mut self) -> ITRIM_W {
        ITRIM_W { w: self }
    }
    #[doc = "Bit 1 - XTAL current boosting (4x)"]
    #[inline(always)]
    pub fn iboost(&mut self) -> IBOOST_W {
        IBOOST_W { w: self }
    }
    #[doc = "Bit 0 - Enable the XTAL 32 kHz oscillator"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
