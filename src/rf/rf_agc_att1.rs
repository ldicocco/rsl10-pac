#[doc = "Reader of register RF_AGC_ATT1"]
pub type R = crate::R<u32, super::RF_AGC_ATT1>;
#[doc = "Writer for register RF_AGC_ATT1"]
pub type W = crate::W<u32, super::RF_AGC_ATT1>;
#[doc = "Register RF_AGC_ATT1 `reset()`'s with value 0xdb6d_b6db"]
impl crate::ResetValue for super::RF_AGC_ATT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xdb6d_b6db
    }
}
#[doc = "\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_ATT_1_AGC_ATT_AB_LO_A {
    #[doc = "3: `11`"]
    AGC_ATT_1_AGC_ATT_AB_LO_DEFAULT = 3,
}
impl From<AGC_ATT_1_AGC_ATT_AB_LO_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_ATT_1_AGC_ATT_AB_LO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_ATT_1_AGC_ATT_AB_LO`"]
pub type AGC_ATT_1_AGC_ATT_AB_LO_R = crate::R<u8, AGC_ATT_1_AGC_ATT_AB_LO_A>;
impl AGC_ATT_1_AGC_ATT_AB_LO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_ATT_1_AGC_ATT_AB_LO_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(AGC_ATT_1_AGC_ATT_AB_LO_A::AGC_ATT_1_AGC_ATT_AB_LO_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_ATT_1_AGC_ATT_AB_LO_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_att_1_agc_att_ab_lo_default(&self) -> bool {
        *self == AGC_ATT_1_AGC_ATT_AB_LO_A::AGC_ATT_1_AGC_ATT_AB_LO_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_ATT_1_AGC_ATT_AB_LO`"]
pub struct AGC_ATT_1_AGC_ATT_AB_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_ATT_1_AGC_ATT_AB_LO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_ATT_1_AGC_ATT_AB_LO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_ab_lo_default(self) -> &'a mut W {
        self.variant(AGC_ATT_1_AGC_ATT_AB_LO_A::AGC_ATT_1_AGC_ATT_AB_LO_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_ATT_1_AGC_ATT_9A_A {
    #[doc = "3: `11`"]
    AGC_ATT_1_AGC_ATT_9A_DEFAULT = 3,
}
impl From<AGC_ATT_1_AGC_ATT_9A_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_ATT_1_AGC_ATT_9A_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_ATT_1_AGC_ATT_9A`"]
pub type AGC_ATT_1_AGC_ATT_9A_R = crate::R<u8, AGC_ATT_1_AGC_ATT_9A_A>;
impl AGC_ATT_1_AGC_ATT_9A_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_ATT_1_AGC_ATT_9A_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(AGC_ATT_1_AGC_ATT_9A_A::AGC_ATT_1_AGC_ATT_9A_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_ATT_1_AGC_ATT_9A_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_att_1_agc_att_9a_default(&self) -> bool {
        *self == AGC_ATT_1_AGC_ATT_9A_A::AGC_ATT_1_AGC_ATT_9A_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_ATT_1_AGC_ATT_9A`"]
pub struct AGC_ATT_1_AGC_ATT_9A_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_ATT_1_AGC_ATT_9A_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_ATT_1_AGC_ATT_9A_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_9a_default(self) -> &'a mut W {
        self.variant(AGC_ATT_1_AGC_ATT_9A_A::AGC_ATT_1_AGC_ATT_9A_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_ATT_1_AGC_ATT_89_A {
    #[doc = "3: `11`"]
    AGC_ATT_1_AGC_ATT_89_DEFAULT = 3,
}
impl From<AGC_ATT_1_AGC_ATT_89_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_ATT_1_AGC_ATT_89_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_ATT_1_AGC_ATT_89`"]
pub type AGC_ATT_1_AGC_ATT_89_R = crate::R<u8, AGC_ATT_1_AGC_ATT_89_A>;
impl AGC_ATT_1_AGC_ATT_89_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_ATT_1_AGC_ATT_89_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(AGC_ATT_1_AGC_ATT_89_A::AGC_ATT_1_AGC_ATT_89_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_ATT_1_AGC_ATT_89_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_att_1_agc_att_89_default(&self) -> bool {
        *self == AGC_ATT_1_AGC_ATT_89_A::AGC_ATT_1_AGC_ATT_89_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_ATT_1_AGC_ATT_89`"]
pub struct AGC_ATT_1_AGC_ATT_89_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_ATT_1_AGC_ATT_89_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_ATT_1_AGC_ATT_89_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_89_default(self) -> &'a mut W {
        self.variant(AGC_ATT_1_AGC_ATT_89_A::AGC_ATT_1_AGC_ATT_89_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_ATT_1_AGC_ATT_78_A {
    #[doc = "3: `11`"]
    AGC_ATT_1_AGC_ATT_78_DEFAULT = 3,
}
impl From<AGC_ATT_1_AGC_ATT_78_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_ATT_1_AGC_ATT_78_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_ATT_1_AGC_ATT_78`"]
pub type AGC_ATT_1_AGC_ATT_78_R = crate::R<u8, AGC_ATT_1_AGC_ATT_78_A>;
impl AGC_ATT_1_AGC_ATT_78_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_ATT_1_AGC_ATT_78_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(AGC_ATT_1_AGC_ATT_78_A::AGC_ATT_1_AGC_ATT_78_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_ATT_1_AGC_ATT_78_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_att_1_agc_att_78_default(&self) -> bool {
        *self == AGC_ATT_1_AGC_ATT_78_A::AGC_ATT_1_AGC_ATT_78_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_ATT_1_AGC_ATT_78`"]
pub struct AGC_ATT_1_AGC_ATT_78_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_ATT_1_AGC_ATT_78_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_ATT_1_AGC_ATT_78_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_78_default(self) -> &'a mut W {
        self.variant(AGC_ATT_1_AGC_ATT_78_A::AGC_ATT_1_AGC_ATT_78_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_ATT_1_AGC_ATT_67_A {
    #[doc = "3: `11`"]
    AGC_ATT_1_AGC_ATT_67_DEFAULT = 3,
}
impl From<AGC_ATT_1_AGC_ATT_67_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_ATT_1_AGC_ATT_67_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_ATT_1_AGC_ATT_67`"]
pub type AGC_ATT_1_AGC_ATT_67_R = crate::R<u8, AGC_ATT_1_AGC_ATT_67_A>;
impl AGC_ATT_1_AGC_ATT_67_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_ATT_1_AGC_ATT_67_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(AGC_ATT_1_AGC_ATT_67_A::AGC_ATT_1_AGC_ATT_67_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_ATT_1_AGC_ATT_67_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_att_1_agc_att_67_default(&self) -> bool {
        *self == AGC_ATT_1_AGC_ATT_67_A::AGC_ATT_1_AGC_ATT_67_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_ATT_1_AGC_ATT_67`"]
pub struct AGC_ATT_1_AGC_ATT_67_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_ATT_1_AGC_ATT_67_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_ATT_1_AGC_ATT_67_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_67_default(self) -> &'a mut W {
        self.variant(AGC_ATT_1_AGC_ATT_67_A::AGC_ATT_1_AGC_ATT_67_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_ATT_1_AGC_ATT_56_A {
    #[doc = "3: `11`"]
    AGC_ATT_1_AGC_ATT_56_DEFAULT = 3,
}
impl From<AGC_ATT_1_AGC_ATT_56_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_ATT_1_AGC_ATT_56_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_ATT_1_AGC_ATT_56`"]
pub type AGC_ATT_1_AGC_ATT_56_R = crate::R<u8, AGC_ATT_1_AGC_ATT_56_A>;
impl AGC_ATT_1_AGC_ATT_56_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_ATT_1_AGC_ATT_56_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(AGC_ATT_1_AGC_ATT_56_A::AGC_ATT_1_AGC_ATT_56_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_ATT_1_AGC_ATT_56_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_att_1_agc_att_56_default(&self) -> bool {
        *self == AGC_ATT_1_AGC_ATT_56_A::AGC_ATT_1_AGC_ATT_56_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_ATT_1_AGC_ATT_56`"]
pub struct AGC_ATT_1_AGC_ATT_56_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_ATT_1_AGC_ATT_56_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_ATT_1_AGC_ATT_56_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_56_default(self) -> &'a mut W {
        self.variant(AGC_ATT_1_AGC_ATT_56_A::AGC_ATT_1_AGC_ATT_56_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | (((value as u32) & 0x07) << 15);
        self.w
    }
}
#[doc = "\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_ATT_1_AGC_ATT_45_A {
    #[doc = "3: `11`"]
    AGC_ATT_1_AGC_ATT_45_DEFAULT = 3,
}
impl From<AGC_ATT_1_AGC_ATT_45_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_ATT_1_AGC_ATT_45_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_ATT_1_AGC_ATT_45`"]
pub type AGC_ATT_1_AGC_ATT_45_R = crate::R<u8, AGC_ATT_1_AGC_ATT_45_A>;
impl AGC_ATT_1_AGC_ATT_45_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_ATT_1_AGC_ATT_45_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(AGC_ATT_1_AGC_ATT_45_A::AGC_ATT_1_AGC_ATT_45_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_ATT_1_AGC_ATT_45_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_att_1_agc_att_45_default(&self) -> bool {
        *self == AGC_ATT_1_AGC_ATT_45_A::AGC_ATT_1_AGC_ATT_45_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_ATT_1_AGC_ATT_45`"]
pub struct AGC_ATT_1_AGC_ATT_45_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_ATT_1_AGC_ATT_45_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_ATT_1_AGC_ATT_45_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_45_default(self) -> &'a mut W {
        self.variant(AGC_ATT_1_AGC_ATT_45_A::AGC_ATT_1_AGC_ATT_45_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_ATT_1_AGC_ATT_34_A {
    #[doc = "3: `11`"]
    AGC_ATT_1_AGC_ATT_34_DEFAULT = 3,
}
impl From<AGC_ATT_1_AGC_ATT_34_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_ATT_1_AGC_ATT_34_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_ATT_1_AGC_ATT_34`"]
pub type AGC_ATT_1_AGC_ATT_34_R = crate::R<u8, AGC_ATT_1_AGC_ATT_34_A>;
impl AGC_ATT_1_AGC_ATT_34_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_ATT_1_AGC_ATT_34_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(AGC_ATT_1_AGC_ATT_34_A::AGC_ATT_1_AGC_ATT_34_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_ATT_1_AGC_ATT_34_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_att_1_agc_att_34_default(&self) -> bool {
        *self == AGC_ATT_1_AGC_ATT_34_A::AGC_ATT_1_AGC_ATT_34_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_ATT_1_AGC_ATT_34`"]
pub struct AGC_ATT_1_AGC_ATT_34_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_ATT_1_AGC_ATT_34_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_ATT_1_AGC_ATT_34_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_34_default(self) -> &'a mut W {
        self.variant(AGC_ATT_1_AGC_ATT_34_A::AGC_ATT_1_AGC_ATT_34_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_ATT_1_AGC_ATT_23_A {
    #[doc = "3: `11`"]
    AGC_ATT_1_AGC_ATT_23_DEFAULT = 3,
}
impl From<AGC_ATT_1_AGC_ATT_23_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_ATT_1_AGC_ATT_23_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_ATT_1_AGC_ATT_23`"]
pub type AGC_ATT_1_AGC_ATT_23_R = crate::R<u8, AGC_ATT_1_AGC_ATT_23_A>;
impl AGC_ATT_1_AGC_ATT_23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_ATT_1_AGC_ATT_23_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(AGC_ATT_1_AGC_ATT_23_A::AGC_ATT_1_AGC_ATT_23_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_ATT_1_AGC_ATT_23_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_att_1_agc_att_23_default(&self) -> bool {
        *self == AGC_ATT_1_AGC_ATT_23_A::AGC_ATT_1_AGC_ATT_23_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_ATT_1_AGC_ATT_23`"]
pub struct AGC_ATT_1_AGC_ATT_23_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_ATT_1_AGC_ATT_23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_ATT_1_AGC_ATT_23_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_23_default(self) -> &'a mut W {
        self.variant(AGC_ATT_1_AGC_ATT_23_A::AGC_ATT_1_AGC_ATT_23_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_ATT_1_AGC_ATT_12_A {
    #[doc = "3: `11`"]
    AGC_ATT_1_AGC_ATT_12_DEFAULT = 3,
}
impl From<AGC_ATT_1_AGC_ATT_12_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_ATT_1_AGC_ATT_12_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_ATT_1_AGC_ATT_12`"]
pub type AGC_ATT_1_AGC_ATT_12_R = crate::R<u8, AGC_ATT_1_AGC_ATT_12_A>;
impl AGC_ATT_1_AGC_ATT_12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_ATT_1_AGC_ATT_12_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(AGC_ATT_1_AGC_ATT_12_A::AGC_ATT_1_AGC_ATT_12_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_ATT_1_AGC_ATT_12_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_att_1_agc_att_12_default(&self) -> bool {
        *self == AGC_ATT_1_AGC_ATT_12_A::AGC_ATT_1_AGC_ATT_12_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_ATT_1_AGC_ATT_12`"]
pub struct AGC_ATT_1_AGC_ATT_12_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_ATT_1_AGC_ATT_12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_ATT_1_AGC_ATT_12_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_12_default(self) -> &'a mut W {
        self.variant(AGC_ATT_1_AGC_ATT_12_A::AGC_ATT_1_AGC_ATT_12_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "These fields specify the attenuation levels\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_ATT_1_AGC_ATT_01_A {
    #[doc = "3: `11`"]
    AGC_ATT_1_AGC_ATT_01_DEFAULT = 3,
}
impl From<AGC_ATT_1_AGC_ATT_01_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_ATT_1_AGC_ATT_01_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_ATT_1_AGC_ATT_01`"]
pub type AGC_ATT_1_AGC_ATT_01_R = crate::R<u8, AGC_ATT_1_AGC_ATT_01_A>;
impl AGC_ATT_1_AGC_ATT_01_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_ATT_1_AGC_ATT_01_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(AGC_ATT_1_AGC_ATT_01_A::AGC_ATT_1_AGC_ATT_01_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_ATT_1_AGC_ATT_01_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_att_1_agc_att_01_default(&self) -> bool {
        *self == AGC_ATT_1_AGC_ATT_01_A::AGC_ATT_1_AGC_ATT_01_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_ATT_1_AGC_ATT_01`"]
pub struct AGC_ATT_1_AGC_ATT_01_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_ATT_1_AGC_ATT_01_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_ATT_1_AGC_ATT_01_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_01_default(self) -> &'a mut W {
        self.variant(AGC_ATT_1_AGC_ATT_01_A::AGC_ATT_1_AGC_ATT_01_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_ab_lo(&self) -> AGC_ATT_1_AGC_ATT_AB_LO_R {
        AGC_ATT_1_AGC_ATT_AB_LO_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_9a(&self) -> AGC_ATT_1_AGC_ATT_9A_R {
        AGC_ATT_1_AGC_ATT_9A_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_89(&self) -> AGC_ATT_1_AGC_ATT_89_R {
        AGC_ATT_1_AGC_ATT_89_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_78(&self) -> AGC_ATT_1_AGC_ATT_78_R {
        AGC_ATT_1_AGC_ATT_78_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_67(&self) -> AGC_ATT_1_AGC_ATT_67_R {
        AGC_ATT_1_AGC_ATT_67_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_56(&self) -> AGC_ATT_1_AGC_ATT_56_R {
        AGC_ATT_1_AGC_ATT_56_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_45(&self) -> AGC_ATT_1_AGC_ATT_45_R {
        AGC_ATT_1_AGC_ATT_45_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_34(&self) -> AGC_ATT_1_AGC_ATT_34_R {
        AGC_ATT_1_AGC_ATT_34_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_23(&self) -> AGC_ATT_1_AGC_ATT_23_R {
        AGC_ATT_1_AGC_ATT_23_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_12(&self) -> AGC_ATT_1_AGC_ATT_12_R {
        AGC_ATT_1_AGC_ATT_12_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - These fields specify the attenuation levels"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_01(&self) -> AGC_ATT_1_AGC_ATT_01_R {
        AGC_ATT_1_AGC_ATT_01_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_ab_lo(&mut self) -> AGC_ATT_1_AGC_ATT_AB_LO_W {
        AGC_ATT_1_AGC_ATT_AB_LO_W { w: self }
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_9a(&mut self) -> AGC_ATT_1_AGC_ATT_9A_W {
        AGC_ATT_1_AGC_ATT_9A_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_89(&mut self) -> AGC_ATT_1_AGC_ATT_89_W {
        AGC_ATT_1_AGC_ATT_89_W { w: self }
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_78(&mut self) -> AGC_ATT_1_AGC_ATT_78_W {
        AGC_ATT_1_AGC_ATT_78_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_67(&mut self) -> AGC_ATT_1_AGC_ATT_67_W {
        AGC_ATT_1_AGC_ATT_67_W { w: self }
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_56(&mut self) -> AGC_ATT_1_AGC_ATT_56_W {
        AGC_ATT_1_AGC_ATT_56_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_45(&mut self) -> AGC_ATT_1_AGC_ATT_45_W {
        AGC_ATT_1_AGC_ATT_45_W { w: self }
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_34(&mut self) -> AGC_ATT_1_AGC_ATT_34_W {
        AGC_ATT_1_AGC_ATT_34_W { w: self }
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_23(&mut self) -> AGC_ATT_1_AGC_ATT_23_W {
        AGC_ATT_1_AGC_ATT_23_W { w: self }
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_12(&mut self) -> AGC_ATT_1_AGC_ATT_12_W {
        AGC_ATT_1_AGC_ATT_12_W { w: self }
    }
    #[doc = "Bits 0:2 - These fields specify the attenuation levels"]
    #[inline(always)]
    pub fn agc_att_1_agc_att_01(&mut self) -> AGC_ATT_1_AGC_ATT_01_W {
        AGC_ATT_1_AGC_ATT_01_W { w: self }
    }
}
