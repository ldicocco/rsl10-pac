#[doc = "Reader of register DIG_RESET_STATUS"]
pub type R = crate::R<u32, super::DIG_RESET_STATUS>;
#[doc = "Writer for register DIG_RESET_STATUS"]
pub type W = crate::W<u32, super::DIG_RESET_STATUS>;
#[doc = "Register DIG_RESET_STATUS `reset()`'s with value 0x01"]
impl crate::ResetValue for super::DIG_RESET_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reset the sticky LOCKUP flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUP_RESET_FLAG_CLEAR_AW {
    #[doc = "1: Reset the sticky LOCKUP flag"]
    LOCKUP_FLAG_CLEAR = 1,
}
impl From<LOCKUP_RESET_FLAG_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_RESET_FLAG_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `LOCKUP_RESET_FLAG_CLEAR`"]
pub struct LOCKUP_RESET_FLAG_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKUP_RESET_FLAG_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKUP_RESET_FLAG_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky LOCKUP flag"]
    #[inline(always)]
    pub fn lockup_flag_clear(self) -> &'a mut W {
        self.variant(LOCKUP_RESET_FLAG_CLEAR_AW::LOCKUP_FLAG_CLEAR)
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
#[doc = "Reset the sticky Watchdog time-out reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WATCHDOG_RESET_FLAG_CLEAR_AW {
    #[doc = "1: Reset the sticky Watchdog time-out reset flag"]
    WATCHDOG_RESET_FLAG_CLEAR = 1,
}
impl From<WATCHDOG_RESET_FLAG_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: WATCHDOG_RESET_FLAG_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `WATCHDOG_RESET_FLAG_CLEAR`"]
pub struct WATCHDOG_RESET_FLAG_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> WATCHDOG_RESET_FLAG_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WATCHDOG_RESET_FLAG_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky Watchdog time-out reset flag"]
    #[inline(always)]
    pub fn watchdog_reset_flag_clear(self) -> &'a mut W {
        self.variant(WATCHDOG_RESET_FLAG_CLEAR_AW::WATCHDOG_RESET_FLAG_CLEAR)
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
#[doc = "Reset the sticky CM3 software reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM3_SW_RESET_FLAG_CLEAR_AW {
    #[doc = "1: Reset the sticky CM3 software reset flag"]
    CM3_SW_RESET_FLAG_CLEAR = 1,
}
impl From<CM3_SW_RESET_FLAG_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: CM3_SW_RESET_FLAG_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CM3_SW_RESET_FLAG_CLEAR`"]
pub struct CM3_SW_RESET_FLAG_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CM3_SW_RESET_FLAG_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM3_SW_RESET_FLAG_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky CM3 software reset flag"]
    #[inline(always)]
    pub fn cm3_sw_reset_flag_clear(self) -> &'a mut W {
        self.variant(CM3_SW_RESET_FLAG_CLEAR_AW::CM3_SW_RESET_FLAG_CLEAR)
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
#[doc = "Reset the sticky ACS reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACS_RESET_FLAG_CLEAR_AW {
    #[doc = "1: Reset the sticky ACS reset flag"]
    ACS_RESET_FLAG_CLEAR = 1,
}
impl From<ACS_RESET_FLAG_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: ACS_RESET_FLAG_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ACS_RESET_FLAG_CLEAR`"]
pub struct ACS_RESET_FLAG_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACS_RESET_FLAG_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACS_RESET_FLAG_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky ACS reset flag"]
    #[inline(always)]
    pub fn acs_reset_flag_clear(self) -> &'a mut W {
        self.variant(ACS_RESET_FLAG_CLEAR_AW::ACS_RESET_FLAG_CLEAR)
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
#[doc = "Sticky flag that detects that a LOCKUP occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUP_FLAG_A {
    #[doc = "0: The LOCKUP has not triggered at least once"]
    LOCKUP_NOT_SET = 0,
    #[doc = "1: The LOCKUP was triggered at least once"]
    LOCKUP_SET = 1,
}
impl From<LOCKUP_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCKUP_FLAG`"]
pub type LOCKUP_FLAG_R = crate::R<bool, LOCKUP_FLAG_A>;
impl LOCKUP_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUP_FLAG_A {
        match self.bits {
            false => LOCKUP_FLAG_A::LOCKUP_NOT_SET,
            true => LOCKUP_FLAG_A::LOCKUP_SET,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKUP_NOT_SET`"]
    #[inline(always)]
    pub fn is_lockup_not_set(&self) -> bool {
        *self == LOCKUP_FLAG_A::LOCKUP_NOT_SET
    }
    #[doc = "Checks if the value of the field is `LOCKUP_SET`"]
    #[inline(always)]
    pub fn is_lockup_set(&self) -> bool {
        *self == LOCKUP_FLAG_A::LOCKUP_SET
    }
}
#[doc = "Sticky flag that detects that a Watchdog time-out reset occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WATCHDOG_RESET_FLAG_A {
    #[doc = "0: The Watchdog time-out reset has not triggered at least once"]
    WATCHDOG_RESET_NOT_SET = 0,
    #[doc = "1: The Watchdog time-out reset was triggered at least once since this status bit was last cleared"]
    WATCHDOG_RESET_SET = 1,
}
impl From<WATCHDOG_RESET_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: WATCHDOG_RESET_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WATCHDOG_RESET_FLAG`"]
pub type WATCHDOG_RESET_FLAG_R = crate::R<bool, WATCHDOG_RESET_FLAG_A>;
impl WATCHDOG_RESET_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WATCHDOG_RESET_FLAG_A {
        match self.bits {
            false => WATCHDOG_RESET_FLAG_A::WATCHDOG_RESET_NOT_SET,
            true => WATCHDOG_RESET_FLAG_A::WATCHDOG_RESET_SET,
        }
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_RESET_NOT_SET`"]
    #[inline(always)]
    pub fn is_watchdog_reset_not_set(&self) -> bool {
        *self == WATCHDOG_RESET_FLAG_A::WATCHDOG_RESET_NOT_SET
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_RESET_SET`"]
    #[inline(always)]
    pub fn is_watchdog_reset_set(&self) -> bool {
        *self == WATCHDOG_RESET_FLAG_A::WATCHDOG_RESET_SET
    }
}
#[doc = "Sticky flag that detects that a CM3 software reset occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM3_SW_RESET_FLAG_A {
    #[doc = "0: The CM3 software system reset has not triggered at least once"]
    CM3_SW_RESET_NOT_SET = 0,
    #[doc = "1: The CM3 software system reset was triggered at least once since this status bit was last cleared"]
    CM3_SW_RESET_SET = 1,
}
impl From<CM3_SW_RESET_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: CM3_SW_RESET_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CM3_SW_RESET_FLAG`"]
pub type CM3_SW_RESET_FLAG_R = crate::R<bool, CM3_SW_RESET_FLAG_A>;
impl CM3_SW_RESET_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM3_SW_RESET_FLAG_A {
        match self.bits {
            false => CM3_SW_RESET_FLAG_A::CM3_SW_RESET_NOT_SET,
            true => CM3_SW_RESET_FLAG_A::CM3_SW_RESET_SET,
        }
    }
    #[doc = "Checks if the value of the field is `CM3_SW_RESET_NOT_SET`"]
    #[inline(always)]
    pub fn is_cm3_sw_reset_not_set(&self) -> bool {
        *self == CM3_SW_RESET_FLAG_A::CM3_SW_RESET_NOT_SET
    }
    #[doc = "Checks if the value of the field is `CM3_SW_RESET_SET`"]
    #[inline(always)]
    pub fn is_cm3_sw_reset_set(&self) -> bool {
        *self == CM3_SW_RESET_FLAG_A::CM3_SW_RESET_SET
    }
}
#[doc = "Sticky flag that detects that a ACS reset occurred\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACS_RESET_FLAG_A {
    #[doc = "0: The ACS reset has not triggered at least once"]
    ACS_RESET_NOT_SET = 0,
    #[doc = "1: The ACS reset was triggered at least once since this status bit was last cleared"]
    ACS_RESET_SET = 1,
}
impl From<ACS_RESET_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: ACS_RESET_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACS_RESET_FLAG`"]
pub type ACS_RESET_FLAG_R = crate::R<bool, ACS_RESET_FLAG_A>;
impl ACS_RESET_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACS_RESET_FLAG_A {
        match self.bits {
            false => ACS_RESET_FLAG_A::ACS_RESET_NOT_SET,
            true => ACS_RESET_FLAG_A::ACS_RESET_SET,
        }
    }
    #[doc = "Checks if the value of the field is `ACS_RESET_NOT_SET`"]
    #[inline(always)]
    pub fn is_acs_reset_not_set(&self) -> bool {
        *self == ACS_RESET_FLAG_A::ACS_RESET_NOT_SET
    }
    #[doc = "Checks if the value of the field is `ACS_RESET_SET`"]
    #[inline(always)]
    pub fn is_acs_reset_set(&self) -> bool {
        *self == ACS_RESET_FLAG_A::ACS_RESET_SET
    }
}
impl R {
    #[doc = "Bit 3 - Sticky flag that detects that a LOCKUP occurred"]
    #[inline(always)]
    pub fn lockup_flag(&self) -> LOCKUP_FLAG_R {
        LOCKUP_FLAG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sticky flag that detects that a Watchdog time-out reset occurred"]
    #[inline(always)]
    pub fn watchdog_reset_flag(&self) -> WATCHDOG_RESET_FLAG_R {
        WATCHDOG_RESET_FLAG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sticky flag that detects that a CM3 software reset occurred"]
    #[inline(always)]
    pub fn cm3_sw_reset_flag(&self) -> CM3_SW_RESET_FLAG_R {
        CM3_SW_RESET_FLAG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Sticky flag that detects that a ACS reset occurred"]
    #[inline(always)]
    pub fn acs_reset_flag(&self) -> ACS_RESET_FLAG_R {
        ACS_RESET_FLAG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Reset the sticky LOCKUP flag"]
    #[inline(always)]
    pub fn lockup_reset_flag_clear(&mut self) -> LOCKUP_RESET_FLAG_CLEAR_W {
        LOCKUP_RESET_FLAG_CLEAR_W { w: self }
    }
    #[doc = "Bit 6 - Reset the sticky Watchdog time-out reset flag"]
    #[inline(always)]
    pub fn watchdog_reset_flag_clear(&mut self) -> WATCHDOG_RESET_FLAG_CLEAR_W {
        WATCHDOG_RESET_FLAG_CLEAR_W { w: self }
    }
    #[doc = "Bit 5 - Reset the sticky CM3 software reset flag"]
    #[inline(always)]
    pub fn cm3_sw_reset_flag_clear(&mut self) -> CM3_SW_RESET_FLAG_CLEAR_W {
        CM3_SW_RESET_FLAG_CLEAR_W { w: self }
    }
    #[doc = "Bit 4 - Reset the sticky ACS reset flag"]
    #[inline(always)]
    pub fn acs_reset_flag_clear(&mut self) -> ACS_RESET_FLAG_CLEAR_W {
        ACS_RESET_FLAG_CLEAR_W { w: self }
    }
}
