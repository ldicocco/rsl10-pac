#[doc = "Reader of register ACS_VDDRET_CTRL"]
pub type R = crate::R<u32, super::ACS_VDDRET_CTRL>;
#[doc = "Writer for register ACS_VDDRET_CTRL"]
pub type W = crate::W<u32, super::ACS_VDDRET_CTRL>;
#[doc = "Register ACS_VDDRET_CTRL `reset()`'s with value 0x0006_0606"]
impl crate::ResetValue for super::ACS_VDDRET_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0006_0606
    }
}
#[doc = "VDDMRET retention regulator voltage trimming\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VDDMRET_VTRIM_A {
    #[doc = "3: VDDMRET trimming value"]
    VDDMRET_TRIM_VALUE = 3,
}
impl From<VDDMRET_VTRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: VDDMRET_VTRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VDDMRET_VTRIM`"]
pub type VDDMRET_VTRIM_R = crate::R<u8, VDDMRET_VTRIM_A>;
impl VDDMRET_VTRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VDDMRET_VTRIM_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(VDDMRET_VTRIM_A::VDDMRET_TRIM_VALUE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VDDMRET_TRIM_VALUE`"]
    #[inline(always)]
    pub fn is_vddmret_trim_value(&self) -> bool {
        *self == VDDMRET_VTRIM_A::VDDMRET_TRIM_VALUE
    }
}
#[doc = "Write proxy for field `VDDMRET_VTRIM`"]
pub struct VDDMRET_VTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDMRET_VTRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDMRET_VTRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "VDDMRET trimming value"]
    #[inline(always)]
    pub fn vddmret_trim_value(self) -> &'a mut W {
        self.variant(VDDMRET_VTRIM_A::VDDMRET_TRIM_VALUE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Enable/Disable the VDDMRET retention regulator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDMRET_ENABLE_A {
    #[doc = "0: The VDDMRET retention regulator is disabled"]
    VDDMRET_DISABLE = 0,
    #[doc = "1: The VDDMRET retention regulator is enabled"]
    VDDMRET_ENABLE = 1,
}
impl From<VDDMRET_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: VDDMRET_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDDMRET_ENABLE`"]
pub type VDDMRET_ENABLE_R = crate::R<bool, VDDMRET_ENABLE_A>;
impl VDDMRET_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDMRET_ENABLE_A {
        match self.bits {
            false => VDDMRET_ENABLE_A::VDDMRET_DISABLE,
            true => VDDMRET_ENABLE_A::VDDMRET_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `VDDMRET_DISABLE`"]
    #[inline(always)]
    pub fn is_vddmret_disable(&self) -> bool {
        *self == VDDMRET_ENABLE_A::VDDMRET_DISABLE
    }
    #[doc = "Checks if the value of the field is `VDDMRET_ENABLE`"]
    #[inline(always)]
    pub fn is_vddmret_enable(&self) -> bool {
        *self == VDDMRET_ENABLE_A::VDDMRET_ENABLE
    }
}
#[doc = "Write proxy for field `VDDMRET_ENABLE`"]
pub struct VDDMRET_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDMRET_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDMRET_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The VDDMRET retention regulator is disabled"]
    #[inline(always)]
    pub fn vddmret_disable(self) -> &'a mut W {
        self.variant(VDDMRET_ENABLE_A::VDDMRET_DISABLE)
    }
    #[doc = "The VDDMRET retention regulator is enabled"]
    #[inline(always)]
    pub fn vddmret_enable(self) -> &'a mut W {
        self.variant(VDDMRET_ENABLE_A::VDDMRET_ENABLE)
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
#[doc = "VDDTRET retention regulator voltage trimming\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VDDTRET_VTRIM_A {
    #[doc = "3: VDDTRET trimming value"]
    VDDTRET_TRIM_VALUE = 3,
}
impl From<VDDTRET_VTRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: VDDTRET_VTRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VDDTRET_VTRIM`"]
pub type VDDTRET_VTRIM_R = crate::R<u8, VDDTRET_VTRIM_A>;
impl VDDTRET_VTRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VDDTRET_VTRIM_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(VDDTRET_VTRIM_A::VDDTRET_TRIM_VALUE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VDDTRET_TRIM_VALUE`"]
    #[inline(always)]
    pub fn is_vddtret_trim_value(&self) -> bool {
        *self == VDDTRET_VTRIM_A::VDDTRET_TRIM_VALUE
    }
}
#[doc = "Write proxy for field `VDDTRET_VTRIM`"]
pub struct VDDTRET_VTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDTRET_VTRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDTRET_VTRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "VDDTRET trimming value"]
    #[inline(always)]
    pub fn vddtret_trim_value(self) -> &'a mut W {
        self.variant(VDDTRET_VTRIM_A::VDDTRET_TRIM_VALUE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Enable/Disable the VDDTRET retention regulator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDTRET_ENABLE_A {
    #[doc = "0: The VDDTRET retention regulator is disabled"]
    VDDTRET_DISABLE = 0,
    #[doc = "1: The VDDTRET retention regulator is enabled"]
    VDDTRET_ENABLE = 1,
}
impl From<VDDTRET_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: VDDTRET_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDDTRET_ENABLE`"]
pub type VDDTRET_ENABLE_R = crate::R<bool, VDDTRET_ENABLE_A>;
impl VDDTRET_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDTRET_ENABLE_A {
        match self.bits {
            false => VDDTRET_ENABLE_A::VDDTRET_DISABLE,
            true => VDDTRET_ENABLE_A::VDDTRET_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `VDDTRET_DISABLE`"]
    #[inline(always)]
    pub fn is_vddtret_disable(&self) -> bool {
        *self == VDDTRET_ENABLE_A::VDDTRET_DISABLE
    }
    #[doc = "Checks if the value of the field is `VDDTRET_ENABLE`"]
    #[inline(always)]
    pub fn is_vddtret_enable(&self) -> bool {
        *self == VDDTRET_ENABLE_A::VDDTRET_ENABLE
    }
}
#[doc = "Write proxy for field `VDDTRET_ENABLE`"]
pub struct VDDTRET_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDTRET_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDTRET_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The VDDTRET retention regulator is disabled"]
    #[inline(always)]
    pub fn vddtret_disable(self) -> &'a mut W {
        self.variant(VDDTRET_ENABLE_A::VDDTRET_DISABLE)
    }
    #[doc = "The VDDTRET retention regulator is enabled"]
    #[inline(always)]
    pub fn vddtret_enable(self) -> &'a mut W {
        self.variant(VDDTRET_ENABLE_A::VDDTRET_ENABLE)
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
#[doc = "VDDCRET retention regulator voltage trimming\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VDDCRET_VTRIM_A {
    #[doc = "3: VDDCRET trimming value"]
    VDDCRET_TRIM_VALUE = 3,
}
impl From<VDDCRET_VTRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: VDDCRET_VTRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VDDCRET_VTRIM`"]
pub type VDDCRET_VTRIM_R = crate::R<u8, VDDCRET_VTRIM_A>;
impl VDDCRET_VTRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VDDCRET_VTRIM_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(VDDCRET_VTRIM_A::VDDCRET_TRIM_VALUE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VDDCRET_TRIM_VALUE`"]
    #[inline(always)]
    pub fn is_vddcret_trim_value(&self) -> bool {
        *self == VDDCRET_VTRIM_A::VDDCRET_TRIM_VALUE
    }
}
#[doc = "Write proxy for field `VDDCRET_VTRIM`"]
pub struct VDDCRET_VTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDCRET_VTRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDCRET_VTRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "VDDCRET trimming value"]
    #[inline(always)]
    pub fn vddcret_trim_value(self) -> &'a mut W {
        self.variant(VDDCRET_VTRIM_A::VDDCRET_TRIM_VALUE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Enable/Disable the VDDCRET retention regulator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDCRET_ENABLE_A {
    #[doc = "0: The VDDCRET retention regulator is disabled"]
    VDDCRET_DISABLE = 0,
    #[doc = "1: The VDDCRET retention regulator is enabled"]
    VDDCRET_ENABLE = 1,
}
impl From<VDDCRET_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: VDDCRET_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDDCRET_ENABLE`"]
pub type VDDCRET_ENABLE_R = crate::R<bool, VDDCRET_ENABLE_A>;
impl VDDCRET_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDCRET_ENABLE_A {
        match self.bits {
            false => VDDCRET_ENABLE_A::VDDCRET_DISABLE,
            true => VDDCRET_ENABLE_A::VDDCRET_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `VDDCRET_DISABLE`"]
    #[inline(always)]
    pub fn is_vddcret_disable(&self) -> bool {
        *self == VDDCRET_ENABLE_A::VDDCRET_DISABLE
    }
    #[doc = "Checks if the value of the field is `VDDCRET_ENABLE`"]
    #[inline(always)]
    pub fn is_vddcret_enable(&self) -> bool {
        *self == VDDCRET_ENABLE_A::VDDCRET_ENABLE
    }
}
#[doc = "Write proxy for field `VDDCRET_ENABLE`"]
pub struct VDDCRET_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDCRET_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDCRET_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The VDDCRET retention regulator is disabled"]
    #[inline(always)]
    pub fn vddcret_disable(self) -> &'a mut W {
        self.variant(VDDCRET_ENABLE_A::VDDCRET_DISABLE)
    }
    #[doc = "The VDDCRET retention regulator is enabled"]
    #[inline(always)]
    pub fn vddcret_enable(self) -> &'a mut W {
        self.variant(VDDCRET_ENABLE_A::VDDCRET_ENABLE)
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
    #[doc = "Bits 17:18 - VDDMRET retention regulator voltage trimming"]
    #[inline(always)]
    pub fn vddmret_vtrim(&self) -> VDDMRET_VTRIM_R {
        VDDMRET_VTRIM_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Enable/Disable the VDDMRET retention regulator"]
    #[inline(always)]
    pub fn vddmret_enable(&self) -> VDDMRET_ENABLE_R {
        VDDMRET_ENABLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - VDDTRET retention regulator voltage trimming"]
    #[inline(always)]
    pub fn vddtret_vtrim(&self) -> VDDTRET_VTRIM_R {
        VDDTRET_VTRIM_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Enable/Disable the VDDTRET retention regulator"]
    #[inline(always)]
    pub fn vddtret_enable(&self) -> VDDTRET_ENABLE_R {
        VDDTRET_ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - VDDCRET retention regulator voltage trimming"]
    #[inline(always)]
    pub fn vddcret_vtrim(&self) -> VDDCRET_VTRIM_R {
        VDDCRET_VTRIM_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - Enable/Disable the VDDCRET retention regulator"]
    #[inline(always)]
    pub fn vddcret_enable(&self) -> VDDCRET_ENABLE_R {
        VDDCRET_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 17:18 - VDDMRET retention regulator voltage trimming"]
    #[inline(always)]
    pub fn vddmret_vtrim(&mut self) -> VDDMRET_VTRIM_W {
        VDDMRET_VTRIM_W { w: self }
    }
    #[doc = "Bit 16 - Enable/Disable the VDDMRET retention regulator"]
    #[inline(always)]
    pub fn vddmret_enable(&mut self) -> VDDMRET_ENABLE_W {
        VDDMRET_ENABLE_W { w: self }
    }
    #[doc = "Bits 9:10 - VDDTRET retention regulator voltage trimming"]
    #[inline(always)]
    pub fn vddtret_vtrim(&mut self) -> VDDTRET_VTRIM_W {
        VDDTRET_VTRIM_W { w: self }
    }
    #[doc = "Bit 8 - Enable/Disable the VDDTRET retention regulator"]
    #[inline(always)]
    pub fn vddtret_enable(&mut self) -> VDDTRET_ENABLE_W {
        VDDTRET_ENABLE_W { w: self }
    }
    #[doc = "Bits 1:2 - VDDCRET retention regulator voltage trimming"]
    #[inline(always)]
    pub fn vddcret_vtrim(&mut self) -> VDDCRET_VTRIM_W {
        VDDCRET_VTRIM_W { w: self }
    }
    #[doc = "Bit 0 - Enable/Disable the VDDCRET retention regulator"]
    #[inline(always)]
    pub fn vddcret_enable(&mut self) -> VDDCRET_ENABLE_W {
        VDDCRET_ENABLE_W { w: self }
    }
}
