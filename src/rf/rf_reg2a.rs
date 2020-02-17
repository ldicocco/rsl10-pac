#[doc = "Reader of register RF_REG2A"]
pub type R = crate::R<u32, super::RF_REG2A>;
#[doc = "Writer for register RF_REG2A"]
pub type W = crate::W<u32, super::RF_REG2A>;
#[doc = "Register RF_REG2A `reset()`'s with value 0x6077"]
impl crate::ResetValue for super::RF_REG2A {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x6077
    }
}
#[doc = "If set to 1, the en PPA cascode bit is independent from the en PA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLES_SEPARATE_PPA_CASC_A {
    #[doc = "0: `0`"]
    ENABLES_SEPARATE_PPA_CASC_DEFAULT = 0,
}
impl From<ENABLES_SEPARATE_PPA_CASC_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLES_SEPARATE_PPA_CASC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLES_SEPARATE_PPA_CASC`"]
pub type ENABLES_SEPARATE_PPA_CASC_R = crate::R<bool, ENABLES_SEPARATE_PPA_CASC_A>;
impl ENABLES_SEPARATE_PPA_CASC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ENABLES_SEPARATE_PPA_CASC_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(ENABLES_SEPARATE_PPA_CASC_A::ENABLES_SEPARATE_PPA_CASC_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLES_SEPARATE_PPA_CASC_DEFAULT`"]
    #[inline(always)]
    pub fn is_enables_separate_ppa_casc_default(&self) -> bool {
        *self == ENABLES_SEPARATE_PPA_CASC_A::ENABLES_SEPARATE_PPA_CASC_DEFAULT
    }
}
#[doc = "Write proxy for field `ENABLES_SEPARATE_PPA_CASC`"]
pub struct ENABLES_SEPARATE_PPA_CASC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLES_SEPARATE_PPA_CASC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLES_SEPARATE_PPA_CASC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enables_separate_ppa_casc_default(self) -> &'a mut W {
        self.variant(ENABLES_SEPARATE_PPA_CASC_A::ENABLES_SEPARATE_PPA_CASC_DEFAULT)
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
#[doc = "Enable signals: 0 => LNA, 1 => LNA, 2 => IFA, 3 => Tx, 4 => PA, 5 => PPA casc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENABLES_EN_RXTX_A {
    #[doc = "0: `0`"]
    ENABLES_EN_RXTX_DEFAULT = 0,
}
impl From<ENABLES_EN_RXTX_A> for u8 {
    #[inline(always)]
    fn from(variant: ENABLES_EN_RXTX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ENABLES_EN_RXTX`"]
pub type ENABLES_EN_RXTX_R = crate::R<u8, ENABLES_EN_RXTX_A>;
impl ENABLES_EN_RXTX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENABLES_EN_RXTX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ENABLES_EN_RXTX_A::ENABLES_EN_RXTX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLES_EN_RXTX_DEFAULT`"]
    #[inline(always)]
    pub fn is_enables_en_rxtx_default(&self) -> bool {
        *self == ENABLES_EN_RXTX_A::ENABLES_EN_RXTX_DEFAULT
    }
}
#[doc = "Write proxy for field `ENABLES_EN_RXTX`"]
pub struct ENABLES_EN_RXTX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLES_EN_RXTX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLES_EN_RXTX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enables_en_rxtx_default(self) -> &'a mut W {
        self.variant(ENABLES_EN_RXTX_A::ENABLES_EN_RXTX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 22)) | (((value as u32) & 0x3f) << 22);
        self.w
    }
}
#[doc = "Enable signals for the BB: 0 => Filter, 1 => Filter central frequency bias, 2 => Filter bandwidth bias, 3 => ADC, 4 => RSSI, 5 => peak detector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENABLES_EN_BB_A {
    #[doc = "0: `0`"]
    ENABLES_EN_BB_DEFAULT = 0,
}
impl From<ENABLES_EN_BB_A> for u8 {
    #[inline(always)]
    fn from(variant: ENABLES_EN_BB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ENABLES_EN_BB`"]
pub type ENABLES_EN_BB_R = crate::R<u8, ENABLES_EN_BB_A>;
impl ENABLES_EN_BB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENABLES_EN_BB_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ENABLES_EN_BB_A::ENABLES_EN_BB_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLES_EN_BB_DEFAULT`"]
    #[inline(always)]
    pub fn is_enables_en_bb_default(&self) -> bool {
        *self == ENABLES_EN_BB_A::ENABLES_EN_BB_DEFAULT
    }
}
#[doc = "Write proxy for field `ENABLES_EN_BB`"]
pub struct ENABLES_EN_BB_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLES_EN_BB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLES_EN_BB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enables_en_bb_default(self) -> &'a mut W {
        self.variant(ENABLES_EN_BB_A::ENABLES_EN_BB_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "RSSI tuning for gain\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSSI_TUN_RSSI_TUN_GAIN_A {
    #[doc = "3: `11`"]
    RSSI_TUN_RSSI_TUN_GAIN_DEFAULT = 3,
}
impl From<RSSI_TUN_RSSI_TUN_GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: RSSI_TUN_RSSI_TUN_GAIN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSSI_TUN_RSSI_TUN_GAIN`"]
pub type RSSI_TUN_RSSI_TUN_GAIN_R = crate::R<u8, RSSI_TUN_RSSI_TUN_GAIN_A>;
impl RSSI_TUN_RSSI_TUN_GAIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSSI_TUN_RSSI_TUN_GAIN_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(RSSI_TUN_RSSI_TUN_GAIN_A::RSSI_TUN_RSSI_TUN_GAIN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_TUN_RSSI_TUN_GAIN_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_tun_rssi_tun_gain_default(&self) -> bool {
        *self == RSSI_TUN_RSSI_TUN_GAIN_A::RSSI_TUN_RSSI_TUN_GAIN_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_TUN_RSSI_TUN_GAIN`"]
pub struct RSSI_TUN_RSSI_TUN_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_TUN_RSSI_TUN_GAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_TUN_RSSI_TUN_GAIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn rssi_tun_rssi_tun_gain_default(self) -> &'a mut W {
        self.variant(RSSI_TUN_RSSI_TUN_GAIN_A::RSSI_TUN_RSSI_TUN_GAIN_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "RSSI tuning for odd stages: offset to the even triangular wave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSSI_TUN_RSSI_ODD_OFFSET_A {
    #[doc = "0: `0`"]
    RSSI_TUN_RSSI_ODD_OFFSET_DEFAULT = 0,
}
impl From<RSSI_TUN_RSSI_ODD_OFFSET_A> for u8 {
    #[inline(always)]
    fn from(variant: RSSI_TUN_RSSI_ODD_OFFSET_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSSI_TUN_RSSI_ODD_OFFSET`"]
pub type RSSI_TUN_RSSI_ODD_OFFSET_R = crate::R<u8, RSSI_TUN_RSSI_ODD_OFFSET_A>;
impl RSSI_TUN_RSSI_ODD_OFFSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSSI_TUN_RSSI_ODD_OFFSET_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RSSI_TUN_RSSI_ODD_OFFSET_A::RSSI_TUN_RSSI_ODD_OFFSET_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_TUN_RSSI_ODD_OFFSET_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_tun_rssi_odd_offset_default(&self) -> bool {
        *self == RSSI_TUN_RSSI_ODD_OFFSET_A::RSSI_TUN_RSSI_ODD_OFFSET_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_TUN_RSSI_ODD_OFFSET`"]
pub struct RSSI_TUN_RSSI_ODD_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_TUN_RSSI_ODD_OFFSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_TUN_RSSI_ODD_OFFSET_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssi_tun_rssi_odd_offset_default(self) -> &'a mut W {
        self.variant(RSSI_TUN_RSSI_ODD_OFFSET_A::RSSI_TUN_RSSI_ODD_OFFSET_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "RSSI tuning for even stages: maximum value of the triangular wave. If max = min, static signal.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSSI_TUN_RSSI_EVEN_MAX_A {
    #[doc = "7: `111`"]
    RSSI_TUN_RSSI_EVEN_MAX_DEFAULT = 7,
}
impl From<RSSI_TUN_RSSI_EVEN_MAX_A> for u8 {
    #[inline(always)]
    fn from(variant: RSSI_TUN_RSSI_EVEN_MAX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSSI_TUN_RSSI_EVEN_MAX`"]
pub type RSSI_TUN_RSSI_EVEN_MAX_R = crate::R<u8, RSSI_TUN_RSSI_EVEN_MAX_A>;
impl RSSI_TUN_RSSI_EVEN_MAX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSSI_TUN_RSSI_EVEN_MAX_A> {
        use crate::Variant::*;
        match self.bits {
            7 => Val(RSSI_TUN_RSSI_EVEN_MAX_A::RSSI_TUN_RSSI_EVEN_MAX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_TUN_RSSI_EVEN_MAX_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_tun_rssi_even_max_default(&self) -> bool {
        *self == RSSI_TUN_RSSI_EVEN_MAX_A::RSSI_TUN_RSSI_EVEN_MAX_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_TUN_RSSI_EVEN_MAX`"]
pub struct RSSI_TUN_RSSI_EVEN_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_TUN_RSSI_EVEN_MAX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_TUN_RSSI_EVEN_MAX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn rssi_tun_rssi_even_max_default(self) -> &'a mut W {
        self.variant(RSSI_TUN_RSSI_EVEN_MAX_A::RSSI_TUN_RSSI_EVEN_MAX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "RSSI tuning for even stages: minimum value of the triangular wave\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSSI_TUN_RSSI_EVEN_MIN_A {
    #[doc = "7: `111`"]
    RSSI_TUN_RSSI_EVEN_MIN_DEFAULT = 7,
}
impl From<RSSI_TUN_RSSI_EVEN_MIN_A> for u8 {
    #[inline(always)]
    fn from(variant: RSSI_TUN_RSSI_EVEN_MIN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSSI_TUN_RSSI_EVEN_MIN`"]
pub type RSSI_TUN_RSSI_EVEN_MIN_R = crate::R<u8, RSSI_TUN_RSSI_EVEN_MIN_A>;
impl RSSI_TUN_RSSI_EVEN_MIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSSI_TUN_RSSI_EVEN_MIN_A> {
        use crate::Variant::*;
        match self.bits {
            7 => Val(RSSI_TUN_RSSI_EVEN_MIN_A::RSSI_TUN_RSSI_EVEN_MIN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_TUN_RSSI_EVEN_MIN_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_tun_rssi_even_min_default(&self) -> bool {
        *self == RSSI_TUN_RSSI_EVEN_MIN_A::RSSI_TUN_RSSI_EVEN_MIN_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_TUN_RSSI_EVEN_MIN`"]
pub struct RSSI_TUN_RSSI_EVEN_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_TUN_RSSI_EVEN_MIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_TUN_RSSI_EVEN_MIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn rssi_tun_rssi_even_min_default(self) -> &'a mut W {
        self.variant(RSSI_TUN_RSSI_EVEN_MIN_A::RSSI_TUN_RSSI_EVEN_MIN_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 28 - If set to 1, the en PPA cascode bit is independent from the en PA"]
    #[inline(always)]
    pub fn enables_separate_ppa_casc(&self) -> ENABLES_SEPARATE_PPA_CASC_R {
        ENABLES_SEPARATE_PPA_CASC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 22:27 - Enable signals: 0 => LNA, 1 => LNA, 2 => IFA, 3 => Tx, 4 => PA, 5 => PPA casc"]
    #[inline(always)]
    pub fn enables_en_rxtx(&self) -> ENABLES_EN_RXTX_R {
        ENABLES_EN_RXTX_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Enable signals for the BB: 0 => Filter, 1 => Filter central frequency bias, 2 => Filter bandwidth bias, 3 => ADC, 4 => RSSI, 5 => peak detector"]
    #[inline(always)]
    pub fn enables_en_bb(&self) -> ENABLES_EN_BB_R {
        ENABLES_EN_BB_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 13:15 - RSSI tuning for gain"]
    #[inline(always)]
    pub fn rssi_tun_rssi_tun_gain(&self) -> RSSI_TUN_RSSI_TUN_GAIN_R {
        RSSI_TUN_RSSI_TUN_GAIN_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 8:12 - RSSI tuning for odd stages: offset to the even triangular wave"]
    #[inline(always)]
    pub fn rssi_tun_rssi_odd_offset(&self) -> RSSI_TUN_RSSI_ODD_OFFSET_R {
        RSSI_TUN_RSSI_ODD_OFFSET_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 4:7 - RSSI tuning for even stages: maximum value of the triangular wave. If max = min, static signal."]
    #[inline(always)]
    pub fn rssi_tun_rssi_even_max(&self) -> RSSI_TUN_RSSI_EVEN_MAX_R {
        RSSI_TUN_RSSI_EVEN_MAX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - RSSI tuning for even stages: minimum value of the triangular wave"]
    #[inline(always)]
    pub fn rssi_tun_rssi_even_min(&self) -> RSSI_TUN_RSSI_EVEN_MIN_R {
        RSSI_TUN_RSSI_EVEN_MIN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 28 - If set to 1, the en PPA cascode bit is independent from the en PA"]
    #[inline(always)]
    pub fn enables_separate_ppa_casc(&mut self) -> ENABLES_SEPARATE_PPA_CASC_W {
        ENABLES_SEPARATE_PPA_CASC_W { w: self }
    }
    #[doc = "Bits 22:27 - Enable signals: 0 => LNA, 1 => LNA, 2 => IFA, 3 => Tx, 4 => PA, 5 => PPA casc"]
    #[inline(always)]
    pub fn enables_en_rxtx(&mut self) -> ENABLES_EN_RXTX_W {
        ENABLES_EN_RXTX_W { w: self }
    }
    #[doc = "Bits 16:21 - Enable signals for the BB: 0 => Filter, 1 => Filter central frequency bias, 2 => Filter bandwidth bias, 3 => ADC, 4 => RSSI, 5 => peak detector"]
    #[inline(always)]
    pub fn enables_en_bb(&mut self) -> ENABLES_EN_BB_W {
        ENABLES_EN_BB_W { w: self }
    }
    #[doc = "Bits 13:15 - RSSI tuning for gain"]
    #[inline(always)]
    pub fn rssi_tun_rssi_tun_gain(&mut self) -> RSSI_TUN_RSSI_TUN_GAIN_W {
        RSSI_TUN_RSSI_TUN_GAIN_W { w: self }
    }
    #[doc = "Bits 8:12 - RSSI tuning for odd stages: offset to the even triangular wave"]
    #[inline(always)]
    pub fn rssi_tun_rssi_odd_offset(&mut self) -> RSSI_TUN_RSSI_ODD_OFFSET_W {
        RSSI_TUN_RSSI_ODD_OFFSET_W { w: self }
    }
    #[doc = "Bits 4:7 - RSSI tuning for even stages: maximum value of the triangular wave. If max = min, static signal."]
    #[inline(always)]
    pub fn rssi_tun_rssi_even_max(&mut self) -> RSSI_TUN_RSSI_EVEN_MAX_W {
        RSSI_TUN_RSSI_EVEN_MAX_W { w: self }
    }
    #[doc = "Bits 0:3 - RSSI tuning for even stages: minimum value of the triangular wave"]
    #[inline(always)]
    pub fn rssi_tun_rssi_even_min(&mut self) -> RSSI_TUN_RSSI_EVEN_MIN_W {
        RSSI_TUN_RSSI_EVEN_MIN_W { w: self }
    }
}
