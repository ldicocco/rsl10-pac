#[doc = "Reader of register RF_AGC_LUT3"]
pub type R = crate::R<u32, super::RF_AGC_LUT3>;
#[doc = "Writer for register RF_AGC_LUT3"]
pub type W = crate::W<u32, super::RF_AGC_LUT3>;
#[doc = "Register RF_AGC_LUT3 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_AGC_LUT3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_LUT_3_AGC_LEVEL_8_LO_A {
    #[doc = "0: `0`"]
    AGC_LUT_3_AGC_LEVEL_8_LO_DEFAULT = 0,
}
impl From<AGC_LUT_3_AGC_LEVEL_8_LO_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_LUT_3_AGC_LEVEL_8_LO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_LUT_3_AGC_LEVEL_8_LO`"]
pub type AGC_LUT_3_AGC_LEVEL_8_LO_R = crate::R<u8, AGC_LUT_3_AGC_LEVEL_8_LO_A>;
impl AGC_LUT_3_AGC_LEVEL_8_LO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_LUT_3_AGC_LEVEL_8_LO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AGC_LUT_3_AGC_LEVEL_8_LO_A::AGC_LUT_3_AGC_LEVEL_8_LO_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_LUT_3_AGC_LEVEL_8_LO_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_lut_3_agc_level_8_lo_default(&self) -> bool {
        *self == AGC_LUT_3_AGC_LEVEL_8_LO_A::AGC_LUT_3_AGC_LEVEL_8_LO_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_LUT_3_AGC_LEVEL_8_LO`"]
pub struct AGC_LUT_3_AGC_LEVEL_8_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_LUT_3_AGC_LEVEL_8_LO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_LUT_3_AGC_LEVEL_8_LO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_lut_3_agc_level_8_lo_default(self) -> &'a mut W {
        self.variant(AGC_LUT_3_AGC_LEVEL_8_LO_A::AGC_LUT_3_AGC_LEVEL_8_LO_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum AGC_LUT_3_AGC_LEVEL_7_A {
    #[doc = "0: `0`"]
    AGC_LUT_3_AGC_LEVEL_7_DEFAULT = 0,
}
impl From<AGC_LUT_3_AGC_LEVEL_7_A> for u16 {
    #[inline(always)]
    fn from(variant: AGC_LUT_3_AGC_LEVEL_7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_LUT_3_AGC_LEVEL_7`"]
pub type AGC_LUT_3_AGC_LEVEL_7_R = crate::R<u16, AGC_LUT_3_AGC_LEVEL_7_A>;
impl AGC_LUT_3_AGC_LEVEL_7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, AGC_LUT_3_AGC_LEVEL_7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AGC_LUT_3_AGC_LEVEL_7_A::AGC_LUT_3_AGC_LEVEL_7_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_LUT_3_AGC_LEVEL_7_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_lut_3_agc_level_7_default(&self) -> bool {
        *self == AGC_LUT_3_AGC_LEVEL_7_A::AGC_LUT_3_AGC_LEVEL_7_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_LUT_3_AGC_LEVEL_7`"]
pub struct AGC_LUT_3_AGC_LEVEL_7_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_LUT_3_AGC_LEVEL_7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_LUT_3_AGC_LEVEL_7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_lut_3_agc_level_7_default(self) -> &'a mut W {
        self.variant(AGC_LUT_3_AGC_LEVEL_7_A::AGC_LUT_3_AGC_LEVEL_7_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 13)) | (((value as u32) & 0x07ff) << 13);
        self.w
    }
}
#[doc = "Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum AGC_LUT_3_AGC_LEVEL_6_A {
    #[doc = "0: `0`"]
    AGC_LUT_3_AGC_LEVEL_6_DEFAULT = 0,
}
impl From<AGC_LUT_3_AGC_LEVEL_6_A> for u16 {
    #[inline(always)]
    fn from(variant: AGC_LUT_3_AGC_LEVEL_6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_LUT_3_AGC_LEVEL_6`"]
pub type AGC_LUT_3_AGC_LEVEL_6_R = crate::R<u16, AGC_LUT_3_AGC_LEVEL_6_A>;
impl AGC_LUT_3_AGC_LEVEL_6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, AGC_LUT_3_AGC_LEVEL_6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AGC_LUT_3_AGC_LEVEL_6_A::AGC_LUT_3_AGC_LEVEL_6_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_LUT_3_AGC_LEVEL_6_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_lut_3_agc_level_6_default(&self) -> bool {
        *self == AGC_LUT_3_AGC_LEVEL_6_A::AGC_LUT_3_AGC_LEVEL_6_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_LUT_3_AGC_LEVEL_6`"]
pub struct AGC_LUT_3_AGC_LEVEL_6_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_LUT_3_AGC_LEVEL_6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_LUT_3_AGC_LEVEL_6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_lut_3_agc_level_6_default(self) -> &'a mut W {
        self.variant(AGC_LUT_3_AGC_LEVEL_6_A::AGC_LUT_3_AGC_LEVEL_6_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 2)) | (((value as u32) & 0x07ff) << 2);
        self.w
    }
}
#[doc = "Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_LUT_3_AGC_LEVEL_5_HI_A {
    #[doc = "0: `0`"]
    AGC_LUT_3_AGC_LEVEL_5_HI_DEFAULT = 0,
}
impl From<AGC_LUT_3_AGC_LEVEL_5_HI_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_LUT_3_AGC_LEVEL_5_HI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_LUT_3_AGC_LEVEL_5_HI`"]
pub type AGC_LUT_3_AGC_LEVEL_5_HI_R = crate::R<u8, AGC_LUT_3_AGC_LEVEL_5_HI_A>;
impl AGC_LUT_3_AGC_LEVEL_5_HI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_LUT_3_AGC_LEVEL_5_HI_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AGC_LUT_3_AGC_LEVEL_5_HI_A::AGC_LUT_3_AGC_LEVEL_5_HI_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_LUT_3_AGC_LEVEL_5_HI_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_lut_3_agc_level_5_hi_default(&self) -> bool {
        *self == AGC_LUT_3_AGC_LEVEL_5_HI_A::AGC_LUT_3_AGC_LEVEL_5_HI_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_LUT_3_AGC_LEVEL_5_HI`"]
pub struct AGC_LUT_3_AGC_LEVEL_5_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_LUT_3_AGC_LEVEL_5_HI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_LUT_3_AGC_LEVEL_5_HI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_lut_3_agc_level_5_hi_default(self) -> &'a mut W {
        self.variant(AGC_LUT_3_AGC_LEVEL_5_HI_A::AGC_LUT_3_AGC_LEVEL_5_HI_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_3_agc_level_8_lo(&self) -> AGC_LUT_3_AGC_LEVEL_8_LO_R {
        AGC_LUT_3_AGC_LEVEL_8_LO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 13:23 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_3_agc_level_7(&self) -> AGC_LUT_3_AGC_LEVEL_7_R {
        AGC_LUT_3_AGC_LEVEL_7_R::new(((self.bits >> 13) & 0x07ff) as u16)
    }
    #[doc = "Bits 2:12 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_3_agc_level_6(&self) -> AGC_LUT_3_AGC_LEVEL_6_R {
        AGC_LUT_3_AGC_LEVEL_6_R::new(((self.bits >> 2) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:1 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_3_agc_level_5_hi(&self) -> AGC_LUT_3_AGC_LEVEL_5_HI_R {
        AGC_LUT_3_AGC_LEVEL_5_HI_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_3_agc_level_8_lo(&mut self) -> AGC_LUT_3_AGC_LEVEL_8_LO_W {
        AGC_LUT_3_AGC_LEVEL_8_LO_W { w: self }
    }
    #[doc = "Bits 13:23 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_3_agc_level_7(&mut self) -> AGC_LUT_3_AGC_LEVEL_7_W {
        AGC_LUT_3_AGC_LEVEL_7_W { w: self }
    }
    #[doc = "Bits 2:12 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_3_agc_level_6(&mut self) -> AGC_LUT_3_AGC_LEVEL_6_W {
        AGC_LUT_3_AGC_LEVEL_6_W { w: self }
    }
    #[doc = "Bits 0:1 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_3_agc_level_5_hi(&mut self) -> AGC_LUT_3_AGC_LEVEL_5_HI_W {
        AGC_LUT_3_AGC_LEVEL_5_HI_W { w: self }
    }
}
