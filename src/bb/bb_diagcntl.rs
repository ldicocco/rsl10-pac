#[doc = "Reader of register BB_DIAGCNTL"]
pub type R = crate::R<u32, super::BB_DIAGCNTL>;
#[doc = "Writer for register BB_DIAGCNTL"]
pub type W = crate::W<u32, super::BB_DIAGCNTL>;
#[doc = "Register BB_DIAGCNTL `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_DIAGCNTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable diagnostic port 3 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIAG3_EN_A {
    #[doc = "0: Disable diagnostic port 3 output. All outputs are set to 0x0."]
    DIAG3_EN_0 = 0,
    #[doc = "1: Enable diagnostic port 3 output"]
    DIAG3_EN_1 = 1,
}
impl From<DIAG3_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DIAG3_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIAG3_EN`"]
pub type DIAG3_EN_R = crate::R<bool, DIAG3_EN_A>;
impl DIAG3_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIAG3_EN_A {
        match self.bits {
            false => DIAG3_EN_A::DIAG3_EN_0,
            true => DIAG3_EN_A::DIAG3_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIAG3_EN_0`"]
    #[inline(always)]
    pub fn is_diag3_en_0(&self) -> bool {
        *self == DIAG3_EN_A::DIAG3_EN_0
    }
    #[doc = "Checks if the value of the field is `DIAG3_EN_1`"]
    #[inline(always)]
    pub fn is_diag3_en_1(&self) -> bool {
        *self == DIAG3_EN_A::DIAG3_EN_1
    }
}
#[doc = "Write proxy for field `DIAG3_EN`"]
pub struct DIAG3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG3_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIAG3_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable diagnostic port 3 output. All outputs are set to 0x0."]
    #[inline(always)]
    pub fn diag3_en_0(self) -> &'a mut W {
        self.variant(DIAG3_EN_A::DIAG3_EN_0)
    }
    #[doc = "Enable diagnostic port 3 output"]
    #[inline(always)]
    pub fn diag3_en_1(self) -> &'a mut W {
        self.variant(DIAG3_EN_A::DIAG3_EN_1)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIAG3_A {
    #[doc = "0: Selection of the outputs that must be driven to the diagnostic port 3"]
    DIAG3_0 = 0,
}
impl From<DIAG3_A> for u8 {
    #[inline(always)]
    fn from(variant: DIAG3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIAG3`"]
pub type DIAG3_R = crate::R<u8, DIAG3_A>;
impl DIAG3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIAG3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIAG3_A::DIAG3_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIAG3_0`"]
    #[inline(always)]
    pub fn is_diag3_0(&self) -> bool {
        *self == DIAG3_A::DIAG3_0
    }
}
#[doc = "Write proxy for field `DIAG3`"]
pub struct DIAG3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIAG3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selection of the outputs that must be driven to the diagnostic port 3"]
    #[inline(always)]
    pub fn diag3_0(self) -> &'a mut W {
        self.variant(DIAG3_A::DIAG3_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Enable diagnostic port 2 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIAG2_EN_A {
    #[doc = "0: Disable diagnostic port 2 output. All outputs are set to 0x0."]
    DIAG2_EN_0 = 0,
    #[doc = "1: Enable diagnostic port 2 output"]
    DIAG2_EN_1 = 1,
}
impl From<DIAG2_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DIAG2_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIAG2_EN`"]
pub type DIAG2_EN_R = crate::R<bool, DIAG2_EN_A>;
impl DIAG2_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIAG2_EN_A {
        match self.bits {
            false => DIAG2_EN_A::DIAG2_EN_0,
            true => DIAG2_EN_A::DIAG2_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIAG2_EN_0`"]
    #[inline(always)]
    pub fn is_diag2_en_0(&self) -> bool {
        *self == DIAG2_EN_A::DIAG2_EN_0
    }
    #[doc = "Checks if the value of the field is `DIAG2_EN_1`"]
    #[inline(always)]
    pub fn is_diag2_en_1(&self) -> bool {
        *self == DIAG2_EN_A::DIAG2_EN_1
    }
}
#[doc = "Write proxy for field `DIAG2_EN`"]
pub struct DIAG2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG2_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIAG2_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable diagnostic port 2 output. All outputs are set to 0x0."]
    #[inline(always)]
    pub fn diag2_en_0(self) -> &'a mut W {
        self.variant(DIAG2_EN_A::DIAG2_EN_0)
    }
    #[doc = "Enable diagnostic port 2 output"]
    #[inline(always)]
    pub fn diag2_en_1(self) -> &'a mut W {
        self.variant(DIAG2_EN_A::DIAG2_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIAG2_A {
    #[doc = "0: Selection of the outputs that must be driven to the diagnostic port 2"]
    DIAG2_0 = 0,
}
impl From<DIAG2_A> for u8 {
    #[inline(always)]
    fn from(variant: DIAG2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIAG2`"]
pub type DIAG2_R = crate::R<u8, DIAG2_A>;
impl DIAG2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIAG2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIAG2_A::DIAG2_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIAG2_0`"]
    #[inline(always)]
    pub fn is_diag2_0(&self) -> bool {
        *self == DIAG2_A::DIAG2_0
    }
}
#[doc = "Write proxy for field `DIAG2`"]
pub struct DIAG2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIAG2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selection of the outputs that must be driven to the diagnostic port 2"]
    #[inline(always)]
    pub fn diag2_0(self) -> &'a mut W {
        self.variant(DIAG2_A::DIAG2_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Enable diagnostic port 1 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIAG1_EN_A {
    #[doc = "0: Disable diagnostic port 1 output. All outputs are set to 0x0."]
    DIAG1_EN_0 = 0,
    #[doc = "1: Enable diagnostic port 1 output"]
    DIAG1_EN_1 = 1,
}
impl From<DIAG1_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DIAG1_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIAG1_EN`"]
pub type DIAG1_EN_R = crate::R<bool, DIAG1_EN_A>;
impl DIAG1_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIAG1_EN_A {
        match self.bits {
            false => DIAG1_EN_A::DIAG1_EN_0,
            true => DIAG1_EN_A::DIAG1_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIAG1_EN_0`"]
    #[inline(always)]
    pub fn is_diag1_en_0(&self) -> bool {
        *self == DIAG1_EN_A::DIAG1_EN_0
    }
    #[doc = "Checks if the value of the field is `DIAG1_EN_1`"]
    #[inline(always)]
    pub fn is_diag1_en_1(&self) -> bool {
        *self == DIAG1_EN_A::DIAG1_EN_1
    }
}
#[doc = "Write proxy for field `DIAG1_EN`"]
pub struct DIAG1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG1_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIAG1_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable diagnostic port 1 output. All outputs are set to 0x0."]
    #[inline(always)]
    pub fn diag1_en_0(self) -> &'a mut W {
        self.variant(DIAG1_EN_A::DIAG1_EN_0)
    }
    #[doc = "Enable diagnostic port 1 output"]
    #[inline(always)]
    pub fn diag1_en_1(self) -> &'a mut W {
        self.variant(DIAG1_EN_A::DIAG1_EN_1)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIAG1_A {
    #[doc = "0: Selection of the outputs that must be driven to the diagnostic port 1"]
    DIAG1_0 = 0,
}
impl From<DIAG1_A> for u8 {
    #[inline(always)]
    fn from(variant: DIAG1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIAG1`"]
pub type DIAG1_R = crate::R<u8, DIAG1_A>;
impl DIAG1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIAG1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIAG1_A::DIAG1_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIAG1_0`"]
    #[inline(always)]
    pub fn is_diag1_0(&self) -> bool {
        *self == DIAG1_A::DIAG1_0
    }
}
#[doc = "Write proxy for field `DIAG1`"]
pub struct DIAG1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIAG1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selection of the outputs that must be driven to the diagnostic port 1"]
    #[inline(always)]
    pub fn diag1_0(self) -> &'a mut W {
        self.variant(DIAG1_A::DIAG1_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Enable diagnostic port 0 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIAG0_EN_A {
    #[doc = "0: Disable diagnostic port 0 output. All outputs are set to 0x0."]
    DIAG0_EN_0 = 0,
    #[doc = "1: Enable diagnostic port 0 output"]
    DIAG0_EN_1 = 1,
}
impl From<DIAG0_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DIAG0_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIAG0_EN`"]
pub type DIAG0_EN_R = crate::R<bool, DIAG0_EN_A>;
impl DIAG0_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIAG0_EN_A {
        match self.bits {
            false => DIAG0_EN_A::DIAG0_EN_0,
            true => DIAG0_EN_A::DIAG0_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIAG0_EN_0`"]
    #[inline(always)]
    pub fn is_diag0_en_0(&self) -> bool {
        *self == DIAG0_EN_A::DIAG0_EN_0
    }
    #[doc = "Checks if the value of the field is `DIAG0_EN_1`"]
    #[inline(always)]
    pub fn is_diag0_en_1(&self) -> bool {
        *self == DIAG0_EN_A::DIAG0_EN_1
    }
}
#[doc = "Write proxy for field `DIAG0_EN`"]
pub struct DIAG0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG0_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIAG0_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable diagnostic port 0 output. All outputs are set to 0x0."]
    #[inline(always)]
    pub fn diag0_en_0(self) -> &'a mut W {
        self.variant(DIAG0_EN_A::DIAG0_EN_0)
    }
    #[doc = "Enable diagnostic port 0 output"]
    #[inline(always)]
    pub fn diag0_en_1(self) -> &'a mut W {
        self.variant(DIAG0_EN_A::DIAG0_EN_1)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIAG0_A {
    #[doc = "0: Selection of the outputs that must be driven to the diagnostic port 0"]
    DIAG0_0 = 0,
}
impl From<DIAG0_A> for u8 {
    #[inline(always)]
    fn from(variant: DIAG0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIAG0`"]
pub type DIAG0_R = crate::R<u8, DIAG0_A>;
impl DIAG0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIAG0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIAG0_A::DIAG0_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIAG0_0`"]
    #[inline(always)]
    pub fn is_diag0_0(&self) -> bool {
        *self == DIAG0_A::DIAG0_0
    }
}
#[doc = "Write proxy for field `DIAG0`"]
pub struct DIAG0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIAG0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selection of the outputs that must be driven to the diagnostic port 0"]
    #[inline(always)]
    pub fn diag0_0(self) -> &'a mut W {
        self.variant(DIAG0_A::DIAG0_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Enable diagnostic port 3 output"]
    #[inline(always)]
    pub fn diag3_en(&self) -> DIAG3_EN_R {
        DIAG3_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn diag3(&self) -> DIAG3_R {
        DIAG3_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Enable diagnostic port 2 output"]
    #[inline(always)]
    pub fn diag2_en(&self) -> DIAG2_EN_R {
        DIAG2_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn diag2(&self) -> DIAG2_R {
        DIAG2_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Enable diagnostic port 1 output"]
    #[inline(always)]
    pub fn diag1_en(&self) -> DIAG1_EN_R {
        DIAG1_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn diag1(&self) -> DIAG1_R {
        DIAG1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Enable diagnostic port 0 output"]
    #[inline(always)]
    pub fn diag0_en(&self) -> DIAG0_EN_R {
        DIAG0_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn diag0(&self) -> DIAG0_R {
        DIAG0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Enable diagnostic port 3 output"]
    #[inline(always)]
    pub fn diag3_en(&mut self) -> DIAG3_EN_W {
        DIAG3_EN_W { w: self }
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn diag3(&mut self) -> DIAG3_W {
        DIAG3_W { w: self }
    }
    #[doc = "Bit 23 - Enable diagnostic port 2 output"]
    #[inline(always)]
    pub fn diag2_en(&mut self) -> DIAG2_EN_W {
        DIAG2_EN_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn diag2(&mut self) -> DIAG2_W {
        DIAG2_W { w: self }
    }
    #[doc = "Bit 15 - Enable diagnostic port 1 output"]
    #[inline(always)]
    pub fn diag1_en(&mut self) -> DIAG1_EN_W {
        DIAG1_EN_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn diag1(&mut self) -> DIAG1_W {
        DIAG1_W { w: self }
    }
    #[doc = "Bit 7 - Enable diagnostic port 0 output"]
    #[inline(always)]
    pub fn diag0_en(&mut self) -> DIAG0_EN_W {
        DIAG0_EN_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn diag0(&mut self) -> DIAG0_W {
        DIAG0_W { w: self }
    }
}
