#[doc = "Reader of register ACS_VDDPA_CTRL"]
pub type R = crate::R<u32, super::ACS_VDDPA_CTRL>;
#[doc = "Writer for register ACS_VDDPA_CTRL"]
pub type W = crate::W<u32, super::ACS_VDDPA_CTRL>;
#[doc = "Register ACS_VDDPA_CTRL `reset()`'s with value 0x37"]
impl crate::ResetValue for super::ACS_VDDPA_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x37
    }
}
#[doc = "Power amplifier supply control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDPA_SW_CTRL_A {
    #[doc = "0: Set the output HIZ (floating) in disable mode"]
    VDDPA_SW_HIZ = 0,
    #[doc = "1: Connect switched output to VDDRF regulator (ENABLE bit must be reset)"]
    VDDPA_SW_VDDRF = 1,
}
impl From<VDDPA_SW_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: VDDPA_SW_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDDPA_SW_CTRL`"]
pub type VDDPA_SW_CTRL_R = crate::R<bool, VDDPA_SW_CTRL_A>;
impl VDDPA_SW_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDPA_SW_CTRL_A {
        match self.bits {
            false => VDDPA_SW_CTRL_A::VDDPA_SW_HIZ,
            true => VDDPA_SW_CTRL_A::VDDPA_SW_VDDRF,
        }
    }
    #[doc = "Checks if the value of the field is `VDDPA_SW_HIZ`"]
    #[inline(always)]
    pub fn is_vddpa_sw_hiz(&self) -> bool {
        *self == VDDPA_SW_CTRL_A::VDDPA_SW_HIZ
    }
    #[doc = "Checks if the value of the field is `VDDPA_SW_VDDRF`"]
    #[inline(always)]
    pub fn is_vddpa_sw_vddrf(&self) -> bool {
        *self == VDDPA_SW_CTRL_A::VDDPA_SW_VDDRF
    }
}
#[doc = "Write proxy for field `VDDPA_SW_CTRL`"]
pub struct VDDPA_SW_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDPA_SW_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDPA_SW_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set the output HIZ (floating) in disable mode"]
    #[inline(always)]
    pub fn vddpa_sw_hiz(self) -> &'a mut W {
        self.variant(VDDPA_SW_CTRL_A::VDDPA_SW_HIZ)
    }
    #[doc = "Connect switched output to VDDRF regulator (ENABLE bit must be reset)"]
    #[inline(always)]
    pub fn vddpa_sw_vddrf(self) -> &'a mut W {
        self.variant(VDDPA_SW_CTRL_A::VDDPA_SW_VDDRF)
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
#[doc = "Enable current sensing circuit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_ISENSE_A {
    #[doc = "0: Disable the VDDPA regulator current sensing circuit"]
    VDDPA_ISENSE_DISABLE = 0,
    #[doc = "1: Enable the VDDPA regulator current sensing circuit"]
    VDDPA_ISENSE_ENABLE = 1,
}
impl From<ENABLE_ISENSE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_ISENSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLE_ISENSE`"]
pub type ENABLE_ISENSE_R = crate::R<bool, ENABLE_ISENSE_A>;
impl ENABLE_ISENSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_ISENSE_A {
        match self.bits {
            false => ENABLE_ISENSE_A::VDDPA_ISENSE_DISABLE,
            true => ENABLE_ISENSE_A::VDDPA_ISENSE_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `VDDPA_ISENSE_DISABLE`"]
    #[inline(always)]
    pub fn is_vddpa_isense_disable(&self) -> bool {
        *self == ENABLE_ISENSE_A::VDDPA_ISENSE_DISABLE
    }
    #[doc = "Checks if the value of the field is `VDDPA_ISENSE_ENABLE`"]
    #[inline(always)]
    pub fn is_vddpa_isense_enable(&self) -> bool {
        *self == ENABLE_ISENSE_A::VDDPA_ISENSE_ENABLE
    }
}
#[doc = "Write proxy for field `ENABLE_ISENSE`"]
pub struct ENABLE_ISENSE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_ISENSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_ISENSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the VDDPA regulator current sensing circuit"]
    #[inline(always)]
    pub fn vddpa_isense_disable(self) -> &'a mut W {
        self.variant(ENABLE_ISENSE_A::VDDPA_ISENSE_DISABLE)
    }
    #[doc = "Enable the VDDPA regulator current sensing circuit"]
    #[inline(always)]
    pub fn vddpa_isense_enable(self) -> &'a mut W {
        self.variant(ENABLE_ISENSE_A::VDDPA_ISENSE_ENABLE)
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
#[doc = "Enable control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Disable the VDDPA regulator"]
    VDDPA_DISABLE = 0,
    #[doc = "1: Enable the VDDPA regulator"]
    VDDPA_ENABLE = 1,
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
            false => ENABLE_A::VDDPA_DISABLE,
            true => ENABLE_A::VDDPA_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `VDDPA_DISABLE`"]
    #[inline(always)]
    pub fn is_vddpa_disable(&self) -> bool {
        *self == ENABLE_A::VDDPA_DISABLE
    }
    #[doc = "Checks if the value of the field is `VDDPA_ENABLE`"]
    #[inline(always)]
    pub fn is_vddpa_enable(&self) -> bool {
        *self == ENABLE_A::VDDPA_ENABLE
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
    #[doc = "Disable the VDDPA regulator"]
    #[inline(always)]
    pub fn vddpa_disable(self) -> &'a mut W {
        self.variant(ENABLE_A::VDDPA_DISABLE)
    }
    #[doc = "Enable the VDDPA regulator"]
    #[inline(always)]
    pub fn vddpa_enable(self) -> &'a mut W {
        self.variant(ENABLE_A::VDDPA_ENABLE)
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
#[doc = "Output voltage trimming configuration in 10 mV steps\n\nValue on reset: 55"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VTRIM_A {
    #[doc = "0: 1.05 V"]
    VDDPA_TRIM_1P05V = 0,
    #[doc = "1: 1.06 V"]
    VDDPA_TRIM_1P06V = 1,
    #[doc = "54: 1.59 V"]
    VDDPA_TRIM_1P59V = 54,
    #[doc = "55: 1.60 V"]
    VDDPA_TRIM_1P60V = 55,
    #[doc = "56: 1.61 V"]
    VDDPA_TRIM_1P61V = 56,
    #[doc = "63: 1.68 V"]
    VDDPA_TRIM_1P68V = 63,
}
impl From<VTRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: VTRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VTRIM`"]
pub type VTRIM_R = crate::R<u8, VTRIM_A>;
impl VTRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VTRIM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VTRIM_A::VDDPA_TRIM_1P05V),
            1 => Val(VTRIM_A::VDDPA_TRIM_1P06V),
            54 => Val(VTRIM_A::VDDPA_TRIM_1P59V),
            55 => Val(VTRIM_A::VDDPA_TRIM_1P60V),
            56 => Val(VTRIM_A::VDDPA_TRIM_1P61V),
            63 => Val(VTRIM_A::VDDPA_TRIM_1P68V),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VDDPA_TRIM_1P05V`"]
    #[inline(always)]
    pub fn is_vddpa_trim_1p05v(&self) -> bool {
        *self == VTRIM_A::VDDPA_TRIM_1P05V
    }
    #[doc = "Checks if the value of the field is `VDDPA_TRIM_1P06V`"]
    #[inline(always)]
    pub fn is_vddpa_trim_1p06v(&self) -> bool {
        *self == VTRIM_A::VDDPA_TRIM_1P06V
    }
    #[doc = "Checks if the value of the field is `VDDPA_TRIM_1P59V`"]
    #[inline(always)]
    pub fn is_vddpa_trim_1p59v(&self) -> bool {
        *self == VTRIM_A::VDDPA_TRIM_1P59V
    }
    #[doc = "Checks if the value of the field is `VDDPA_TRIM_1P60V`"]
    #[inline(always)]
    pub fn is_vddpa_trim_1p60v(&self) -> bool {
        *self == VTRIM_A::VDDPA_TRIM_1P60V
    }
    #[doc = "Checks if the value of the field is `VDDPA_TRIM_1P61V`"]
    #[inline(always)]
    pub fn is_vddpa_trim_1p61v(&self) -> bool {
        *self == VTRIM_A::VDDPA_TRIM_1P61V
    }
    #[doc = "Checks if the value of the field is `VDDPA_TRIM_1P68V`"]
    #[inline(always)]
    pub fn is_vddpa_trim_1p68v(&self) -> bool {
        *self == VTRIM_A::VDDPA_TRIM_1P68V
    }
}
#[doc = "Write proxy for field `VTRIM`"]
pub struct VTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VTRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1.05 V"]
    #[inline(always)]
    pub fn vddpa_trim_1p05v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDPA_TRIM_1P05V)
    }
    #[doc = "1.06 V"]
    #[inline(always)]
    pub fn vddpa_trim_1p06v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDPA_TRIM_1P06V)
    }
    #[doc = "1.59 V"]
    #[inline(always)]
    pub fn vddpa_trim_1p59v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDPA_TRIM_1P59V)
    }
    #[doc = "1.60 V"]
    #[inline(always)]
    pub fn vddpa_trim_1p60v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDPA_TRIM_1P60V)
    }
    #[doc = "1.61 V"]
    #[inline(always)]
    pub fn vddpa_trim_1p61v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDPA_TRIM_1P61V)
    }
    #[doc = "1.68 V"]
    #[inline(always)]
    pub fn vddpa_trim_1p68v(self) -> &'a mut W {
        self.variant(VTRIM_A::VDDPA_TRIM_1P68V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 12 - Power amplifier supply control"]
    #[inline(always)]
    pub fn vddpa_sw_ctrl(&self) -> VDDPA_SW_CTRL_R {
        VDDPA_SW_CTRL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable current sensing circuit"]
    #[inline(always)]
    pub fn enable_isense(&self) -> ENABLE_ISENSE_R {
        ENABLE_ISENSE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable control"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - Output voltage trimming configuration in 10 mV steps"]
    #[inline(always)]
    pub fn vtrim(&self) -> VTRIM_R {
        VTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - Power amplifier supply control"]
    #[inline(always)]
    pub fn vddpa_sw_ctrl(&mut self) -> VDDPA_SW_CTRL_W {
        VDDPA_SW_CTRL_W { w: self }
    }
    #[doc = "Bit 9 - Enable current sensing circuit"]
    #[inline(always)]
    pub fn enable_isense(&mut self) -> ENABLE_ISENSE_W {
        ENABLE_ISENSE_W { w: self }
    }
    #[doc = "Bit 8 - Enable control"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 0:5 - Output voltage trimming configuration in 10 mV steps"]
    #[inline(always)]
    pub fn vtrim(&mut self) -> VTRIM_W {
        VTRIM_W { w: self }
    }
}
