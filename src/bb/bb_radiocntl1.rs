#[doc = "Reader of register BB_RADIOCNTL1"]
pub type R = crate::R<u32, super::BB_RADIOCNTL1>;
#[doc = "Writer for register BB_RADIOCNTL1"]
pub type W = crate::W<u32, super::BB_RADIOCNTL1>;
#[doc = "Register BB_RADIOCNTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_RADIOCNTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Control ATLAS/Ripple AGC force mode based on radioCNTL2-FORCEAGC_LENGTH value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEAGC_EN_A {
    #[doc = "0: Disable"]
    FORCEAGC_EN_0 = 0,
    #[doc = "1: Enable"]
    FORCEAGC_EN_1 = 1,
}
impl From<FORCEAGC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEAGC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FORCEAGC_EN`"]
pub type FORCEAGC_EN_R = crate::R<bool, FORCEAGC_EN_A>;
impl FORCEAGC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEAGC_EN_A {
        match self.bits {
            false => FORCEAGC_EN_A::FORCEAGC_EN_0,
            true => FORCEAGC_EN_A::FORCEAGC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FORCEAGC_EN_0`"]
    #[inline(always)]
    pub fn is_forceagc_en_0(&self) -> bool {
        *self == FORCEAGC_EN_A::FORCEAGC_EN_0
    }
    #[doc = "Checks if the value of the field is `FORCEAGC_EN_1`"]
    #[inline(always)]
    pub fn is_forceagc_en_1(&self) -> bool {
        *self == FORCEAGC_EN_A::FORCEAGC_EN_1
    }
}
#[doc = "Write proxy for field `FORCEAGC_EN`"]
pub struct FORCEAGC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEAGC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEAGC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn forceagc_en_0(self) -> &'a mut W {
        self.variant(FORCEAGC_EN_A::FORCEAGC_EN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn forceagc_en_1(self) -> &'a mut W {
        self.variant(FORCEAGC_EN_A::FORCEAGC_EN_1)
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
#[doc = "Control Ripple modulation mode in between FM and I&Q\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEBLEIQ_A {
    #[doc = "0: FM modulation mode"]
    FORCEBLEIQ_0 = 0,
    #[doc = "1: I&Q modulation mode"]
    FORCEBLEIQ_1 = 1,
}
impl From<FORCEBLEIQ_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEBLEIQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FORCEBLEIQ`"]
pub type FORCEBLEIQ_R = crate::R<bool, FORCEBLEIQ_A>;
impl FORCEBLEIQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEBLEIQ_A {
        match self.bits {
            false => FORCEBLEIQ_A::FORCEBLEIQ_0,
            true => FORCEBLEIQ_A::FORCEBLEIQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FORCEBLEIQ_0`"]
    #[inline(always)]
    pub fn is_forcebleiq_0(&self) -> bool {
        *self == FORCEBLEIQ_A::FORCEBLEIQ_0
    }
    #[doc = "Checks if the value of the field is `FORCEBLEIQ_1`"]
    #[inline(always)]
    pub fn is_forcebleiq_1(&self) -> bool {
        *self == FORCEBLEIQ_A::FORCEBLEIQ_1
    }
}
#[doc = "Write proxy for field `FORCEBLEIQ`"]
pub struct FORCEBLEIQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEBLEIQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEBLEIQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FM modulation mode"]
    #[inline(always)]
    pub fn forcebleiq_0(self) -> &'a mut W {
        self.variant(FORCEBLEIQ_A::FORCEBLEIQ_0)
    }
    #[doc = "I&Q modulation mode"]
    #[inline(always)]
    pub fn forcebleiq_1(self) -> &'a mut W {
        self.variant(FORCEBLEIQ_A::FORCEBLEIQ_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Control ATLAS/Ripple AGC force mode based on radioCNTL2-FORCEAGC_LENGTH value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum FORCEAGC_LENGTH_A {
    #[doc = "0: `0`"]
    FORCEAGC_LENGTH_0 = 0,
}
impl From<FORCEAGC_LENGTH_A> for u16 {
    #[inline(always)]
    fn from(variant: FORCEAGC_LENGTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FORCEAGC_LENGTH`"]
pub type FORCEAGC_LENGTH_R = crate::R<u16, FORCEAGC_LENGTH_A>;
impl FORCEAGC_LENGTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, FORCEAGC_LENGTH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FORCEAGC_LENGTH_A::FORCEAGC_LENGTH_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FORCEAGC_LENGTH_0`"]
    #[inline(always)]
    pub fn is_forceagc_length_0(&self) -> bool {
        *self == FORCEAGC_LENGTH_A::FORCEAGC_LENGTH_0
    }
}
#[doc = "Write proxy for field `FORCEAGC_LENGTH`"]
pub struct FORCEAGC_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEAGC_LENGTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEAGC_LENGTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn forceagc_length_0(self) -> &'a mut W {
        self.variant(FORCEAGC_LENGTH_A::FORCEAGC_LENGTH_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Define whether the SYNC_P pulse is generated as pulse or level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_PULSE_MODE_A {
    #[doc = "0: SYNC_P generated as pulse"]
    SYNC_PULSE_MODE_0 = 0,
    #[doc = "1: SYNC_P generated as level"]
    SYNC_PULSE_MODE_1 = 1,
}
impl From<SYNC_PULSE_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_PULSE_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNC_PULSE_MODE`"]
pub type SYNC_PULSE_MODE_R = crate::R<bool, SYNC_PULSE_MODE_A>;
impl SYNC_PULSE_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_PULSE_MODE_A {
        match self.bits {
            false => SYNC_PULSE_MODE_A::SYNC_PULSE_MODE_0,
            true => SYNC_PULSE_MODE_A::SYNC_PULSE_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYNC_PULSE_MODE_0`"]
    #[inline(always)]
    pub fn is_sync_pulse_mode_0(&self) -> bool {
        *self == SYNC_PULSE_MODE_A::SYNC_PULSE_MODE_0
    }
    #[doc = "Checks if the value of the field is `SYNC_PULSE_MODE_1`"]
    #[inline(always)]
    pub fn is_sync_pulse_mode_1(&self) -> bool {
        *self == SYNC_PULSE_MODE_A::SYNC_PULSE_MODE_1
    }
}
#[doc = "Write proxy for field `SYNC_PULSE_MODE`"]
pub struct SYNC_PULSE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_PULSE_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_PULSE_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SYNC_P generated as pulse"]
    #[inline(always)]
    pub fn sync_pulse_mode_0(self) -> &'a mut W {
        self.variant(SYNC_PULSE_MODE_A::SYNC_PULSE_MODE_0)
    }
    #[doc = "SYNC_P generated as level"]
    #[inline(always)]
    pub fn sync_pulse_mode_1(self) -> &'a mut W {
        self.variant(SYNC_PULSE_MODE_A::SYNC_PULSE_MODE_1)
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
#[doc = "Enable the use of delayed DC compensated data path in radio correlator block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPCORR_EN_A {
    #[doc = "0: Disable"]
    DPCORR_EN_0 = 0,
    #[doc = "1: Enable"]
    DPCORR_EN_1 = 1,
}
impl From<DPCORR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DPCORR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPCORR_EN`"]
pub type DPCORR_EN_R = crate::R<bool, DPCORR_EN_A>;
impl DPCORR_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPCORR_EN_A {
        match self.bits {
            false => DPCORR_EN_A::DPCORR_EN_0,
            true => DPCORR_EN_A::DPCORR_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DPCORR_EN_0`"]
    #[inline(always)]
    pub fn is_dpcorr_en_0(&self) -> bool {
        *self == DPCORR_EN_A::DPCORR_EN_0
    }
    #[doc = "Checks if the value of the field is `DPCORR_EN_1`"]
    #[inline(always)]
    pub fn is_dpcorr_en_1(&self) -> bool {
        *self == DPCORR_EN_A::DPCORR_EN_1
    }
}
#[doc = "Write proxy for field `DPCORR_EN`"]
pub struct DPCORR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPCORR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPCORR_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dpcorr_en_0(self) -> &'a mut W {
        self.variant(DPCORR_EN_A::DPCORR_EN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn dpcorr_en_1(self) -> &'a mut W {
        self.variant(DPCORR_EN_A::DPCORR_EN_1)
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
#[doc = "Selects Jitter Elimination FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEF_SELECT_A {
    #[doc = "0: `0`"]
    JEF_SELECT_0 = 0,
    #[doc = "1: `1`"]
    JEF_SELECT_1 = 1,
}
impl From<JEF_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: JEF_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JEF_SELECT`"]
pub type JEF_SELECT_R = crate::R<bool, JEF_SELECT_A>;
impl JEF_SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEF_SELECT_A {
        match self.bits {
            false => JEF_SELECT_A::JEF_SELECT_0,
            true => JEF_SELECT_A::JEF_SELECT_1,
        }
    }
    #[doc = "Checks if the value of the field is `JEF_SELECT_0`"]
    #[inline(always)]
    pub fn is_jef_select_0(&self) -> bool {
        *self == JEF_SELECT_A::JEF_SELECT_0
    }
    #[doc = "Checks if the value of the field is `JEF_SELECT_1`"]
    #[inline(always)]
    pub fn is_jef_select_1(&self) -> bool {
        *self == JEF_SELECT_A::JEF_SELECT_1
    }
}
#[doc = "Write proxy for field `JEF_SELECT`"]
pub struct JEF_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> JEF_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JEF_SELECT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn jef_select_0(self) -> &'a mut W {
        self.variant(JEF_SELECT_A::JEF_SELECT_0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn jef_select_1(self) -> &'a mut W {
        self.variant(JEF_SELECT_A::JEF_SELECT_1)
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
#[doc = "Extended radio selection field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XRFSEL_A {
    #[doc = "0: No radio selected"]
    XRFSEL_0 = 0,
    #[doc = "1: Ripple radio (BT4.0)"]
    XRFSEL_1 = 1,
    #[doc = "2: External radio controller"]
    XRFSEL_2 = 2,
    #[doc = "3: ICYTRX radio (BLE)"]
    XRFSEL_3 = 3,
}
impl From<XRFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: XRFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XRFSEL`"]
pub type XRFSEL_R = crate::R<u8, XRFSEL_A>;
impl XRFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, XRFSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(XRFSEL_A::XRFSEL_0),
            1 => Val(XRFSEL_A::XRFSEL_1),
            2 => Val(XRFSEL_A::XRFSEL_2),
            3 => Val(XRFSEL_A::XRFSEL_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `XRFSEL_0`"]
    #[inline(always)]
    pub fn is_xrfsel_0(&self) -> bool {
        *self == XRFSEL_A::XRFSEL_0
    }
    #[doc = "Checks if the value of the field is `XRFSEL_1`"]
    #[inline(always)]
    pub fn is_xrfsel_1(&self) -> bool {
        *self == XRFSEL_A::XRFSEL_1
    }
    #[doc = "Checks if the value of the field is `XRFSEL_2`"]
    #[inline(always)]
    pub fn is_xrfsel_2(&self) -> bool {
        *self == XRFSEL_A::XRFSEL_2
    }
    #[doc = "Checks if the value of the field is `XRFSEL_3`"]
    #[inline(always)]
    pub fn is_xrfsel_3(&self) -> bool {
        *self == XRFSEL_A::XRFSEL_3
    }
}
#[doc = "Write proxy for field `XRFSEL`"]
pub struct XRFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XRFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XRFSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No radio selected"]
    #[inline(always)]
    pub fn xrfsel_0(self) -> &'a mut W {
        self.variant(XRFSEL_A::XRFSEL_0)
    }
    #[doc = "Ripple radio (BT4.0)"]
    #[inline(always)]
    pub fn xrfsel_1(self) -> &'a mut W {
        self.variant(XRFSEL_A::XRFSEL_1)
    }
    #[doc = "External radio controller"]
    #[inline(always)]
    pub fn xrfsel_2(self) -> &'a mut W {
        self.variant(XRFSEL_A::XRFSEL_2)
    }
    #[doc = "ICYTRX radio (BLE)"]
    #[inline(always)]
    pub fn xrfsel_3(self) -> &'a mut W {
        self.variant(XRFSEL_A::XRFSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
#[doc = "CSEM RF Sub-version selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBVERSION_A {
    #[doc = "0: IcyTRx 65nm (v0x12, v0x20, v0x22)"]
    SUBVERSION_0 = 0,
    #[doc = "1: IcyTRx CS1 (v0x30)"]
    SUBVERSION_1 = 1,
    #[doc = "2: IcyTRx CS2 (v0x31)"]
    SUBVERSION_2 = 2,
    #[doc = "3: IcyTRx GCS1 (v0x40)"]
    SUBVERSION_3 = 3,
}
impl From<SUBVERSION_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBVERSION_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SUBVERSION`"]
pub type SUBVERSION_R = crate::R<u8, SUBVERSION_A>;
impl SUBVERSION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUBVERSION_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SUBVERSION_A::SUBVERSION_0),
            1 => Val(SUBVERSION_A::SUBVERSION_1),
            2 => Val(SUBVERSION_A::SUBVERSION_2),
            3 => Val(SUBVERSION_A::SUBVERSION_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUBVERSION_0`"]
    #[inline(always)]
    pub fn is_subversion_0(&self) -> bool {
        *self == SUBVERSION_A::SUBVERSION_0
    }
    #[doc = "Checks if the value of the field is `SUBVERSION_1`"]
    #[inline(always)]
    pub fn is_subversion_1(&self) -> bool {
        *self == SUBVERSION_A::SUBVERSION_1
    }
    #[doc = "Checks if the value of the field is `SUBVERSION_2`"]
    #[inline(always)]
    pub fn is_subversion_2(&self) -> bool {
        *self == SUBVERSION_A::SUBVERSION_2
    }
    #[doc = "Checks if the value of the field is `SUBVERSION_3`"]
    #[inline(always)]
    pub fn is_subversion_3(&self) -> bool {
        *self == SUBVERSION_A::SUBVERSION_3
    }
}
#[doc = "Write proxy for field `SUBVERSION`"]
pub struct SUBVERSION_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBVERSION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUBVERSION_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "IcyTRx 65nm (v0x12, v0x20, v0x22)"]
    #[inline(always)]
    pub fn subversion_0(self) -> &'a mut W {
        self.variant(SUBVERSION_A::SUBVERSION_0)
    }
    #[doc = "IcyTRx CS1 (v0x30)"]
    #[inline(always)]
    pub fn subversion_1(self) -> &'a mut W {
        self.variant(SUBVERSION_A::SUBVERSION_1)
    }
    #[doc = "IcyTRx CS2 (v0x31)"]
    #[inline(always)]
    pub fn subversion_2(self) -> &'a mut W {
        self.variant(SUBVERSION_A::SUBVERSION_2)
    }
    #[doc = "IcyTRx GCS1 (v0x40)"]
    #[inline(always)]
    pub fn subversion_3(self) -> &'a mut W {
        self.variant(SUBVERSION_A::SUBVERSION_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Control ATLAS/Ripple AGC force mode based on radioCNTL2-FORCEAGC_LENGTH value"]
    #[inline(always)]
    pub fn forceagc_en(&self) -> FORCEAGC_EN_R {
        FORCEAGC_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Control Ripple modulation mode in between FM and I&Q"]
    #[inline(always)]
    pub fn forcebleiq(&self) -> FORCEBLEIQ_R {
        FORCEBLEIQ_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 16:27 - Control ATLAS/Ripple AGC force mode based on radioCNTL2-FORCEAGC_LENGTH value"]
    #[inline(always)]
    pub fn forceagc_length(&self) -> FORCEAGC_LENGTH_R {
        FORCEAGC_LENGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - Define whether the SYNC_P pulse is generated as pulse or level"]
    #[inline(always)]
    pub fn sync_pulse_mode(&self) -> SYNC_PULSE_MODE_R {
        SYNC_PULSE_MODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable the use of delayed DC compensated data path in radio correlator block"]
    #[inline(always)]
    pub fn dpcorr_en(&self) -> DPCORR_EN_R {
        DPCORR_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Selects Jitter Elimination FIFO"]
    #[inline(always)]
    pub fn jef_select(&self) -> JEF_SELECT_R {
        JEF_SELECT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 4:8 - Extended radio selection field"]
    #[inline(always)]
    pub fn xrfsel(&self) -> XRFSEL_R {
        XRFSEL_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 0:3 - CSEM RF Sub-version selection"]
    #[inline(always)]
    pub fn subversion(&self) -> SUBVERSION_R {
        SUBVERSION_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Control ATLAS/Ripple AGC force mode based on radioCNTL2-FORCEAGC_LENGTH value"]
    #[inline(always)]
    pub fn forceagc_en(&mut self) -> FORCEAGC_EN_W {
        FORCEAGC_EN_W { w: self }
    }
    #[doc = "Bit 30 - Control Ripple modulation mode in between FM and I&Q"]
    #[inline(always)]
    pub fn forcebleiq(&mut self) -> FORCEBLEIQ_W {
        FORCEBLEIQ_W { w: self }
    }
    #[doc = "Bits 16:27 - Control ATLAS/Ripple AGC force mode based on radioCNTL2-FORCEAGC_LENGTH value"]
    #[inline(always)]
    pub fn forceagc_length(&mut self) -> FORCEAGC_LENGTH_W {
        FORCEAGC_LENGTH_W { w: self }
    }
    #[doc = "Bit 15 - Define whether the SYNC_P pulse is generated as pulse or level"]
    #[inline(always)]
    pub fn sync_pulse_mode(&mut self) -> SYNC_PULSE_MODE_W {
        SYNC_PULSE_MODE_W { w: self }
    }
    #[doc = "Bit 13 - Enable the use of delayed DC compensated data path in radio correlator block"]
    #[inline(always)]
    pub fn dpcorr_en(&mut self) -> DPCORR_EN_W {
        DPCORR_EN_W { w: self }
    }
    #[doc = "Bit 12 - Selects Jitter Elimination FIFO"]
    #[inline(always)]
    pub fn jef_select(&mut self) -> JEF_SELECT_W {
        JEF_SELECT_W { w: self }
    }
    #[doc = "Bits 4:8 - Extended radio selection field"]
    #[inline(always)]
    pub fn xrfsel(&mut self) -> XRFSEL_W {
        XRFSEL_W { w: self }
    }
    #[doc = "Bits 0:3 - CSEM RF Sub-version selection"]
    #[inline(always)]
    pub fn subversion(&mut self) -> SUBVERSION_W {
        SUBVERSION_W { w: self }
    }
}
