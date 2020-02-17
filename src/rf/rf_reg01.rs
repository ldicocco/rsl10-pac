#[doc = "Reader of register RF_REG01"]
pub type R = crate::R<u32, super::RF_REG01>;
#[doc = "Writer for register RF_REG01"]
pub type W = crate::W<u32, super::RF_REG01>;
#[doc = "Register RF_REG01 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG01 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Time constant of the fine carrier recovery block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAU_PHASE_RECOV_TAU_PHASE_RECOV_A {
    #[doc = "0: `0`"]
    TAU_PHASE_RECOV_TAU_PHASE_RECOV_DEFAULT = 0,
}
impl From<TAU_PHASE_RECOV_TAU_PHASE_RECOV_A> for u8 {
    #[inline(always)]
    fn from(variant: TAU_PHASE_RECOV_TAU_PHASE_RECOV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TAU_PHASE_RECOV_TAU_PHASE_RECOV`"]
pub type TAU_PHASE_RECOV_TAU_PHASE_RECOV_R = crate::R<u8, TAU_PHASE_RECOV_TAU_PHASE_RECOV_A>;
impl TAU_PHASE_RECOV_TAU_PHASE_RECOV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TAU_PHASE_RECOV_TAU_PHASE_RECOV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TAU_PHASE_RECOV_TAU_PHASE_RECOV_A::TAU_PHASE_RECOV_TAU_PHASE_RECOV_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TAU_PHASE_RECOV_TAU_PHASE_RECOV_DEFAULT`"]
    #[inline(always)]
    pub fn is_tau_phase_recov_tau_phase_recov_default(&self) -> bool {
        *self == TAU_PHASE_RECOV_TAU_PHASE_RECOV_A::TAU_PHASE_RECOV_TAU_PHASE_RECOV_DEFAULT
    }
}
#[doc = "Write proxy for field `TAU_PHASE_RECOV_TAU_PHASE_RECOV`"]
pub struct TAU_PHASE_RECOV_TAU_PHASE_RECOV_W<'a> {
    w: &'a mut W,
}
impl<'a> TAU_PHASE_RECOV_TAU_PHASE_RECOV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAU_PHASE_RECOV_TAU_PHASE_RECOV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tau_phase_recov_tau_phase_recov_default(self) -> &'a mut W {
        self.variant(TAU_PHASE_RECOV_TAU_PHASE_RECOV_A::TAU_PHASE_RECOV_TAU_PHASE_RECOV_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Time constant of the rough carrier recovery block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_A {
    #[doc = "0: `0`"]
    TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_DEFAULT = 0,
}
impl From<TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_A> for u8 {
    #[inline(always)]
    fn from(variant: TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TAU_ROUGH_RECOV_TAU_ROUGH_RECOV`"]
pub type TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_R = crate::R<u8, TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_A>;
impl TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_A::TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_DEFAULT`"]
    #[inline(always)]
    pub fn is_tau_rough_recov_tau_rough_recov_default(&self) -> bool {
        *self == TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_A::TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_DEFAULT
    }
}
#[doc = "Write proxy for field `TAU_ROUGH_RECOV_TAU_ROUGH_RECOV`"]
pub struct TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_W<'a> {
    w: &'a mut W,
}
impl<'a> TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tau_rough_recov_tau_rough_recov_default(self) -> &'a mut W {
        self.variant(TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_A::TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "If set to 1, enables the automatic AFC correction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_A {
    #[doc = "0: `0`"]
    CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_DEFAULT = 0,
}
impl From<CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_A> for bool {
    #[inline(always)]
    fn from(variant: CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC`"]
pub type CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_R =
    crate::R<bool, CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_A>;
impl CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_A> {
        use crate::Variant::*;
        match self . bits { false => Val ( CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_A :: CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_DEFAULT ) , i => Res ( i ) }
    }
    #[doc = "Checks if the value of the field is `CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_DEFAULT`"]
    #[inline(always)]
    pub fn is_carrier_recovery_en_correct_cfreq_afc_default(&self) -> bool {
        * self == CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_A :: CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_DEFAULT
    }
}
#[doc = "Write proxy for field `CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC`"]
pub struct CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn carrier_recovery_en_correct_cfreq_afc_default(self) -> &'a mut W {
        self.variant(
            CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_A::CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_DEFAULT,
        )
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
#[doc = "If set to 1, the IF correction is negative\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_A {
    #[doc = "0: `0`"]
    CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_DEFAULT = 0,
}
impl From<CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_A> for bool {
    #[inline(always)]
    fn from(variant: CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG`"]
pub type CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_R =
    crate::R<bool, CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_A>;
impl CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_A> {
        use crate::Variant::*;
        match self . bits { false => Val ( CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_A :: CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_DEFAULT ) , i => Res ( i ) }
    }
    #[doc = "Checks if the value of the field is `CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_DEFAULT`"]
    #[inline(always)]
    pub fn is_carrier_recovery_correct_cfreq_if_neg_default(&self) -> bool {
        * self == CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_A :: CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_DEFAULT
    }
}
#[doc = "Write proxy for field `CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG`"]
pub struct CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn carrier_recovery_correct_cfreq_if_neg_default(self) -> &'a mut W {
        self.variant(
            CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_A::CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_DEFAULT,
        )
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "If set to 1, enables the automatic IF correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_A {
    #[doc = "0: `0`"]
    CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_DEFAULT = 0,
}
impl From<CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_A> for bool {
    #[inline(always)]
    fn from(variant: CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF`"]
pub type CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_R =
    crate::R<bool, CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_A>;
impl CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_A> {
        use crate::Variant::*;
        match self . bits { false => Val ( CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_A :: CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_DEFAULT ) , i => Res ( i ) }
    }
    #[doc = "Checks if the value of the field is `CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_DEFAULT`"]
    #[inline(always)]
    pub fn is_carrier_recovery_en_correct_cfreq_if_default(&self) -> bool {
        *self
            == CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_A::CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_DEFAULT
    }
}
#[doc = "Write proxy for field `CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF`"]
pub struct CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn carrier_recovery_en_correct_cfreq_if_default(self) -> &'a mut W {
        self.variant(
            CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_A::CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_DEFAULT,
        )
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
#[doc = "If set to 1 correct the AFC negatively\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARRIER_RECOVERY_AFC_NEG_A {
    #[doc = "0: `0`"]
    CARRIER_RECOVERY_AFC_NEG_DEFAULT = 0,
}
impl From<CARRIER_RECOVERY_AFC_NEG_A> for bool {
    #[inline(always)]
    fn from(variant: CARRIER_RECOVERY_AFC_NEG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARRIER_RECOVERY_AFC_NEG`"]
pub type CARRIER_RECOVERY_AFC_NEG_R = crate::R<bool, CARRIER_RECOVERY_AFC_NEG_A>;
impl CARRIER_RECOVERY_AFC_NEG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CARRIER_RECOVERY_AFC_NEG_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CARRIER_RECOVERY_AFC_NEG_A::CARRIER_RECOVERY_AFC_NEG_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CARRIER_RECOVERY_AFC_NEG_DEFAULT`"]
    #[inline(always)]
    pub fn is_carrier_recovery_afc_neg_default(&self) -> bool {
        *self == CARRIER_RECOVERY_AFC_NEG_A::CARRIER_RECOVERY_AFC_NEG_DEFAULT
    }
}
#[doc = "Write proxy for field `CARRIER_RECOVERY_AFC_NEG`"]
pub struct CARRIER_RECOVERY_AFC_NEG_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_RECOVERY_AFC_NEG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARRIER_RECOVERY_AFC_NEG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn carrier_recovery_afc_neg_default(self) -> &'a mut W {
        self.variant(CARRIER_RECOVERY_AFC_NEG_A::CARRIER_RECOVERY_AFC_NEG_DEFAULT)
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
#[doc = "If set to 1 enables the starter mode, i.e. a 32x faster carrier recovery.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARRIER_RECOVERY_STARTER_MODE_A {
    #[doc = "0: `0`"]
    CARRIER_RECOVERY_STARTER_MODE_DEFAULT = 0,
}
impl From<CARRIER_RECOVERY_STARTER_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: CARRIER_RECOVERY_STARTER_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARRIER_RECOVERY_STARTER_MODE`"]
pub type CARRIER_RECOVERY_STARTER_MODE_R = crate::R<bool, CARRIER_RECOVERY_STARTER_MODE_A>;
impl CARRIER_RECOVERY_STARTER_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CARRIER_RECOVERY_STARTER_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CARRIER_RECOVERY_STARTER_MODE_A::CARRIER_RECOVERY_STARTER_MODE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CARRIER_RECOVERY_STARTER_MODE_DEFAULT`"]
    #[inline(always)]
    pub fn is_carrier_recovery_starter_mode_default(&self) -> bool {
        *self == CARRIER_RECOVERY_STARTER_MODE_A::CARRIER_RECOVERY_STARTER_MODE_DEFAULT
    }
}
#[doc = "Write proxy for field `CARRIER_RECOVERY_STARTER_MODE`"]
pub struct CARRIER_RECOVERY_STARTER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_RECOVERY_STARTER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARRIER_RECOVERY_STARTER_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn carrier_recovery_starter_mode_default(self) -> &'a mut W {
        self.variant(CARRIER_RECOVERY_STARTER_MODE_A::CARRIER_RECOVERY_STARTER_MODE_DEFAULT)
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
#[doc = "if set to 1 enables the Automatic Frequency Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARRIER_RECOVERY_EN_AFC_A {
    #[doc = "0: `0`"]
    CARRIER_RECOVERY_EN_AFC_DEFAULT = 0,
}
impl From<CARRIER_RECOVERY_EN_AFC_A> for bool {
    #[inline(always)]
    fn from(variant: CARRIER_RECOVERY_EN_AFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARRIER_RECOVERY_EN_AFC`"]
pub type CARRIER_RECOVERY_EN_AFC_R = crate::R<bool, CARRIER_RECOVERY_EN_AFC_A>;
impl CARRIER_RECOVERY_EN_AFC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CARRIER_RECOVERY_EN_AFC_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CARRIER_RECOVERY_EN_AFC_A::CARRIER_RECOVERY_EN_AFC_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CARRIER_RECOVERY_EN_AFC_DEFAULT`"]
    #[inline(always)]
    pub fn is_carrier_recovery_en_afc_default(&self) -> bool {
        *self == CARRIER_RECOVERY_EN_AFC_A::CARRIER_RECOVERY_EN_AFC_DEFAULT
    }
}
#[doc = "Write proxy for field `CARRIER_RECOVERY_EN_AFC`"]
pub struct CARRIER_RECOVERY_EN_AFC_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_RECOVERY_EN_AFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARRIER_RECOVERY_EN_AFC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn carrier_recovery_en_afc_default(self) -> &'a mut W {
        self.variant(CARRIER_RECOVERY_EN_AFC_A::CARRIER_RECOVERY_EN_AFC_DEFAULT)
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
#[doc = "If set to 1 enables the fine carrier recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARRIER_RECOVERY_EN_FINE_RECOV_A {
    #[doc = "0: `0`"]
    CARRIER_RECOVERY_EN_FINE_RECOV_DEFAULT = 0,
}
impl From<CARRIER_RECOVERY_EN_FINE_RECOV_A> for bool {
    #[inline(always)]
    fn from(variant: CARRIER_RECOVERY_EN_FINE_RECOV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARRIER_RECOVERY_EN_FINE_RECOV`"]
pub type CARRIER_RECOVERY_EN_FINE_RECOV_R = crate::R<bool, CARRIER_RECOVERY_EN_FINE_RECOV_A>;
impl CARRIER_RECOVERY_EN_FINE_RECOV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CARRIER_RECOVERY_EN_FINE_RECOV_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CARRIER_RECOVERY_EN_FINE_RECOV_A::CARRIER_RECOVERY_EN_FINE_RECOV_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CARRIER_RECOVERY_EN_FINE_RECOV_DEFAULT`"]
    #[inline(always)]
    pub fn is_carrier_recovery_en_fine_recov_default(&self) -> bool {
        *self == CARRIER_RECOVERY_EN_FINE_RECOV_A::CARRIER_RECOVERY_EN_FINE_RECOV_DEFAULT
    }
}
#[doc = "Write proxy for field `CARRIER_RECOVERY_EN_FINE_RECOV`"]
pub struct CARRIER_RECOVERY_EN_FINE_RECOV_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_RECOVERY_EN_FINE_RECOV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARRIER_RECOVERY_EN_FINE_RECOV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn carrier_recovery_en_fine_recov_default(self) -> &'a mut W {
        self.variant(CARRIER_RECOVERY_EN_FINE_RECOV_A::CARRIER_RECOVERY_EN_FINE_RECOV_DEFAULT)
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
#[doc = "If set to 1 enables the rough carrier recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARRIER_RECOVERY_EN_ROUGH_RECOV_A {
    #[doc = "0: `0`"]
    CARRIER_RECOVERY_EN_ROUGH_RECOV_DEFAULT = 0,
}
impl From<CARRIER_RECOVERY_EN_ROUGH_RECOV_A> for bool {
    #[inline(always)]
    fn from(variant: CARRIER_RECOVERY_EN_ROUGH_RECOV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARRIER_RECOVERY_EN_ROUGH_RECOV`"]
pub type CARRIER_RECOVERY_EN_ROUGH_RECOV_R = crate::R<bool, CARRIER_RECOVERY_EN_ROUGH_RECOV_A>;
impl CARRIER_RECOVERY_EN_ROUGH_RECOV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CARRIER_RECOVERY_EN_ROUGH_RECOV_A> {
        use crate::Variant::*;
        match self.bits {
            false => {
                Val(CARRIER_RECOVERY_EN_ROUGH_RECOV_A::CARRIER_RECOVERY_EN_ROUGH_RECOV_DEFAULT)
            }
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CARRIER_RECOVERY_EN_ROUGH_RECOV_DEFAULT`"]
    #[inline(always)]
    pub fn is_carrier_recovery_en_rough_recov_default(&self) -> bool {
        *self == CARRIER_RECOVERY_EN_ROUGH_RECOV_A::CARRIER_RECOVERY_EN_ROUGH_RECOV_DEFAULT
    }
}
#[doc = "Write proxy for field `CARRIER_RECOVERY_EN_ROUGH_RECOV`"]
pub struct CARRIER_RECOVERY_EN_ROUGH_RECOV_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_RECOVERY_EN_ROUGH_RECOV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARRIER_RECOVERY_EN_ROUGH_RECOV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn carrier_recovery_en_rough_recov_default(self) -> &'a mut W {
        self.variant(CARRIER_RECOVERY_EN_ROUGH_RECOV_A::CARRIER_RECOVERY_EN_ROUGH_RECOV_DEFAULT)
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
#[doc = "If set to 1, the Tx pulse shape is an odd function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_TX_PULSE_NSYM_A {
    #[doc = "0: `0`"]
    MOD_TX_PULSE_NSYM_DEFAULT = 0,
}
impl From<MOD_TX_PULSE_NSYM_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_TX_PULSE_NSYM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MOD_TX_PULSE_NSYM`"]
pub type MOD_TX_PULSE_NSYM_R = crate::R<bool, MOD_TX_PULSE_NSYM_A>;
impl MOD_TX_PULSE_NSYM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MOD_TX_PULSE_NSYM_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MOD_TX_PULSE_NSYM_A::MOD_TX_PULSE_NSYM_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MOD_TX_PULSE_NSYM_DEFAULT`"]
    #[inline(always)]
    pub fn is_mod_tx_pulse_nsym_default(&self) -> bool {
        *self == MOD_TX_PULSE_NSYM_A::MOD_TX_PULSE_NSYM_DEFAULT
    }
}
#[doc = "Write proxy for field `MOD_TX_PULSE_NSYM`"]
pub struct MOD_TX_PULSE_NSYM_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_TX_PULSE_NSYM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_TX_PULSE_NSYM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mod_tx_pulse_nsym_default(self) -> &'a mut W {
        self.variant(MOD_TX_PULSE_NSYM_A::MOD_TX_PULSE_NSYM_DEFAULT)
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
#[doc = "If set to 1, enables the Tx CIC interpolator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_TX_EN_INTERP_A {
    #[doc = "0: `0`"]
    MOD_TX_EN_INTERP_DEFAULT = 0,
}
impl From<MOD_TX_EN_INTERP_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_TX_EN_INTERP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MOD_TX_EN_INTERP`"]
pub type MOD_TX_EN_INTERP_R = crate::R<bool, MOD_TX_EN_INTERP_A>;
impl MOD_TX_EN_INTERP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MOD_TX_EN_INTERP_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MOD_TX_EN_INTERP_A::MOD_TX_EN_INTERP_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MOD_TX_EN_INTERP_DEFAULT`"]
    #[inline(always)]
    pub fn is_mod_tx_en_interp_default(&self) -> bool {
        *self == MOD_TX_EN_INTERP_A::MOD_TX_EN_INTERP_DEFAULT
    }
}
#[doc = "Write proxy for field `MOD_TX_EN_INTERP`"]
pub struct MOD_TX_EN_INTERP_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_TX_EN_INTERP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_TX_EN_INTERP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mod_tx_en_interp_default(self) -> &'a mut W {
        self.variant(MOD_TX_EN_INTERP_A::MOD_TX_EN_INTERP_DEFAULT)
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
#[doc = "Unsigned value that determine the Tx CIC interpolator frequency. The formula is similar to the evaluation of the oversampling frequency.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MOD_TX_CK_TX_M_A {
    #[doc = "0: `0`"]
    MOD_TX_CK_TX_M_DEFAULT = 0,
}
impl From<MOD_TX_CK_TX_M_A> for u8 {
    #[inline(always)]
    fn from(variant: MOD_TX_CK_TX_M_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MOD_TX_CK_TX_M`"]
pub type MOD_TX_CK_TX_M_R = crate::R<u8, MOD_TX_CK_TX_M_A>;
impl MOD_TX_CK_TX_M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MOD_TX_CK_TX_M_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MOD_TX_CK_TX_M_A::MOD_TX_CK_TX_M_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MOD_TX_CK_TX_M_DEFAULT`"]
    #[inline(always)]
    pub fn is_mod_tx_ck_tx_m_default(&self) -> bool {
        *self == MOD_TX_CK_TX_M_A::MOD_TX_CK_TX_M_DEFAULT
    }
}
#[doc = "Write proxy for field `MOD_TX_CK_TX_M`"]
pub struct MOD_TX_CK_TX_M_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_TX_CK_TX_M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_TX_CK_TX_M_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mod_tx_ck_tx_m_default(self) -> &'a mut W {
        self.variant(MOD_TX_CK_TX_M_A::MOD_TX_CK_TX_M_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Time constant of the fine carrier recovery block"]
    #[inline(always)]
    pub fn tau_phase_recov_tau_phase_recov(&self) -> TAU_PHASE_RECOV_TAU_PHASE_RECOV_R {
        TAU_PHASE_RECOV_TAU_PHASE_RECOV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Time constant of the rough carrier recovery block"]
    #[inline(always)]
    pub fn tau_rough_recov_tau_rough_recov(&self) -> TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_R {
        TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 15 - If set to 1, enables the automatic AFC correction."]
    #[inline(always)]
    pub fn carrier_recovery_en_correct_cfreq_afc(&self) -> CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_R {
        CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - If set to 1, the IF correction is negative"]
    #[inline(always)]
    pub fn carrier_recovery_correct_cfreq_if_neg(&self) -> CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_R {
        CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - If set to 1, enables the automatic IF correction"]
    #[inline(always)]
    pub fn carrier_recovery_en_correct_cfreq_if(&self) -> CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_R {
        CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - If set to 1 correct the AFC negatively"]
    #[inline(always)]
    pub fn carrier_recovery_afc_neg(&self) -> CARRIER_RECOVERY_AFC_NEG_R {
        CARRIER_RECOVERY_AFC_NEG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - If set to 1 enables the starter mode, i.e. a 32x faster carrier recovery."]
    #[inline(always)]
    pub fn carrier_recovery_starter_mode(&self) -> CARRIER_RECOVERY_STARTER_MODE_R {
        CARRIER_RECOVERY_STARTER_MODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - if set to 1 enables the Automatic Frequency Control"]
    #[inline(always)]
    pub fn carrier_recovery_en_afc(&self) -> CARRIER_RECOVERY_EN_AFC_R {
        CARRIER_RECOVERY_EN_AFC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - If set to 1 enables the fine carrier recovery"]
    #[inline(always)]
    pub fn carrier_recovery_en_fine_recov(&self) -> CARRIER_RECOVERY_EN_FINE_RECOV_R {
        CARRIER_RECOVERY_EN_FINE_RECOV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - If set to 1 enables the rough carrier recovery"]
    #[inline(always)]
    pub fn carrier_recovery_en_rough_recov(&self) -> CARRIER_RECOVERY_EN_ROUGH_RECOV_R {
        CARRIER_RECOVERY_EN_ROUGH_RECOV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - If set to 1, the Tx pulse shape is an odd function."]
    #[inline(always)]
    pub fn mod_tx_pulse_nsym(&self) -> MOD_TX_PULSE_NSYM_R {
        MOD_TX_PULSE_NSYM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If set to 1, enables the Tx CIC interpolator."]
    #[inline(always)]
    pub fn mod_tx_en_interp(&self) -> MOD_TX_EN_INTERP_R {
        MOD_TX_EN_INTERP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - Unsigned value that determine the Tx CIC interpolator frequency. The formula is similar to the evaluation of the oversampling frequency."]
    #[inline(always)]
    pub fn mod_tx_ck_tx_m(&self) -> MOD_TX_CK_TX_M_R {
        MOD_TX_CK_TX_M_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Time constant of the fine carrier recovery block"]
    #[inline(always)]
    pub fn tau_phase_recov_tau_phase_recov(&mut self) -> TAU_PHASE_RECOV_TAU_PHASE_RECOV_W {
        TAU_PHASE_RECOV_TAU_PHASE_RECOV_W { w: self }
    }
    #[doc = "Bits 16:23 - Time constant of the rough carrier recovery block"]
    #[inline(always)]
    pub fn tau_rough_recov_tau_rough_recov(&mut self) -> TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_W {
        TAU_ROUGH_RECOV_TAU_ROUGH_RECOV_W { w: self }
    }
    #[doc = "Bit 15 - If set to 1, enables the automatic AFC correction."]
    #[inline(always)]
    pub fn carrier_recovery_en_correct_cfreq_afc(
        &mut self,
    ) -> CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_W {
        CARRIER_RECOVERY_EN_CORRECT_CFREQ_AFC_W { w: self }
    }
    #[doc = "Bit 14 - If set to 1, the IF correction is negative"]
    #[inline(always)]
    pub fn carrier_recovery_correct_cfreq_if_neg(
        &mut self,
    ) -> CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_W {
        CARRIER_RECOVERY_CORRECT_CFREQ_IF_NEG_W { w: self }
    }
    #[doc = "Bit 13 - If set to 1, enables the automatic IF correction"]
    #[inline(always)]
    pub fn carrier_recovery_en_correct_cfreq_if(
        &mut self,
    ) -> CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_W {
        CARRIER_RECOVERY_EN_CORRECT_CFREQ_IF_W { w: self }
    }
    #[doc = "Bit 12 - If set to 1 correct the AFC negatively"]
    #[inline(always)]
    pub fn carrier_recovery_afc_neg(&mut self) -> CARRIER_RECOVERY_AFC_NEG_W {
        CARRIER_RECOVERY_AFC_NEG_W { w: self }
    }
    #[doc = "Bit 11 - If set to 1 enables the starter mode, i.e. a 32x faster carrier recovery."]
    #[inline(always)]
    pub fn carrier_recovery_starter_mode(&mut self) -> CARRIER_RECOVERY_STARTER_MODE_W {
        CARRIER_RECOVERY_STARTER_MODE_W { w: self }
    }
    #[doc = "Bit 10 - if set to 1 enables the Automatic Frequency Control"]
    #[inline(always)]
    pub fn carrier_recovery_en_afc(&mut self) -> CARRIER_RECOVERY_EN_AFC_W {
        CARRIER_RECOVERY_EN_AFC_W { w: self }
    }
    #[doc = "Bit 9 - If set to 1 enables the fine carrier recovery"]
    #[inline(always)]
    pub fn carrier_recovery_en_fine_recov(&mut self) -> CARRIER_RECOVERY_EN_FINE_RECOV_W {
        CARRIER_RECOVERY_EN_FINE_RECOV_W { w: self }
    }
    #[doc = "Bit 8 - If set to 1 enables the rough carrier recovery"]
    #[inline(always)]
    pub fn carrier_recovery_en_rough_recov(&mut self) -> CARRIER_RECOVERY_EN_ROUGH_RECOV_W {
        CARRIER_RECOVERY_EN_ROUGH_RECOV_W { w: self }
    }
    #[doc = "Bit 6 - If set to 1, the Tx pulse shape is an odd function."]
    #[inline(always)]
    pub fn mod_tx_pulse_nsym(&mut self) -> MOD_TX_PULSE_NSYM_W {
        MOD_TX_PULSE_NSYM_W { w: self }
    }
    #[doc = "Bit 5 - If set to 1, enables the Tx CIC interpolator."]
    #[inline(always)]
    pub fn mod_tx_en_interp(&mut self) -> MOD_TX_EN_INTERP_W {
        MOD_TX_EN_INTERP_W { w: self }
    }
    #[doc = "Bits 0:4 - Unsigned value that determine the Tx CIC interpolator frequency. The formula is similar to the evaluation of the oversampling frequency."]
    #[inline(always)]
    pub fn mod_tx_ck_tx_m(&mut self) -> MOD_TX_CK_TX_M_W {
        MOD_TX_CK_TX_M_W { w: self }
    }
}
