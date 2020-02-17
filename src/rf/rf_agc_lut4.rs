#[doc = "Reader of register RF_AGC_LUT4"]
pub type R = crate::R<u32, super::RF_AGC_LUT4>;
#[doc = "Writer for register RF_AGC_LUT4"]
pub type W = crate::W<u32, super::RF_AGC_LUT4>;
#[doc = "Register RF_AGC_LUT4 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_AGC_LUT4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_LUT_4_AGC_LEVEL_11_LO_A {
    #[doc = "0: `0`"]
    AGC_LUT_4_AGC_LEVEL_11_LO_DEFAULT = 0,
}
impl From<AGC_LUT_4_AGC_LEVEL_11_LO_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_LUT_4_AGC_LEVEL_11_LO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_LUT_4_AGC_LEVEL_11_LO`"]
pub type AGC_LUT_4_AGC_LEVEL_11_LO_R = crate::R<u8, AGC_LUT_4_AGC_LEVEL_11_LO_A>;
impl AGC_LUT_4_AGC_LEVEL_11_LO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_LUT_4_AGC_LEVEL_11_LO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AGC_LUT_4_AGC_LEVEL_11_LO_A::AGC_LUT_4_AGC_LEVEL_11_LO_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_LUT_4_AGC_LEVEL_11_LO_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_lut_4_agc_level_11_lo_default(&self) -> bool {
        *self == AGC_LUT_4_AGC_LEVEL_11_LO_A::AGC_LUT_4_AGC_LEVEL_11_LO_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_LUT_4_AGC_LEVEL_11_LO`"]
pub struct AGC_LUT_4_AGC_LEVEL_11_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_LUT_4_AGC_LEVEL_11_LO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_LUT_4_AGC_LEVEL_11_LO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_lut_4_agc_level_11_lo_default(self) -> &'a mut W {
        self.variant(AGC_LUT_4_AGC_LEVEL_11_LO_A::AGC_LUT_4_AGC_LEVEL_11_LO_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum AGC_LUT_4_AGC_LEVEL_10_A {
    #[doc = "0: `0`"]
    AGC_LUT_4_AGC_LEVEL_10_DEFAULT = 0,
}
impl From<AGC_LUT_4_AGC_LEVEL_10_A> for u16 {
    #[inline(always)]
    fn from(variant: AGC_LUT_4_AGC_LEVEL_10_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_LUT_4_AGC_LEVEL_10`"]
pub type AGC_LUT_4_AGC_LEVEL_10_R = crate::R<u16, AGC_LUT_4_AGC_LEVEL_10_A>;
impl AGC_LUT_4_AGC_LEVEL_10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, AGC_LUT_4_AGC_LEVEL_10_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AGC_LUT_4_AGC_LEVEL_10_A::AGC_LUT_4_AGC_LEVEL_10_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_LUT_4_AGC_LEVEL_10_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_lut_4_agc_level_10_default(&self) -> bool {
        *self == AGC_LUT_4_AGC_LEVEL_10_A::AGC_LUT_4_AGC_LEVEL_10_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_LUT_4_AGC_LEVEL_10`"]
pub struct AGC_LUT_4_AGC_LEVEL_10_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_LUT_4_AGC_LEVEL_10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_LUT_4_AGC_LEVEL_10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_lut_4_agc_level_10_default(self) -> &'a mut W {
        self.variant(AGC_LUT_4_AGC_LEVEL_10_A::AGC_LUT_4_AGC_LEVEL_10_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 14)) | (((value as u32) & 0x07ff) << 14);
        self.w
    }
}
#[doc = "Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum AGC_LUT_4_AGC_LEVEL_9_A {
    #[doc = "0: `0`"]
    AGC_LUT_4_AGC_LEVEL_9_DEFAULT = 0,
}
impl From<AGC_LUT_4_AGC_LEVEL_9_A> for u16 {
    #[inline(always)]
    fn from(variant: AGC_LUT_4_AGC_LEVEL_9_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_LUT_4_AGC_LEVEL_9`"]
pub type AGC_LUT_4_AGC_LEVEL_9_R = crate::R<u16, AGC_LUT_4_AGC_LEVEL_9_A>;
impl AGC_LUT_4_AGC_LEVEL_9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, AGC_LUT_4_AGC_LEVEL_9_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AGC_LUT_4_AGC_LEVEL_9_A::AGC_LUT_4_AGC_LEVEL_9_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_LUT_4_AGC_LEVEL_9_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_lut_4_agc_level_9_default(&self) -> bool {
        *self == AGC_LUT_4_AGC_LEVEL_9_A::AGC_LUT_4_AGC_LEVEL_9_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_LUT_4_AGC_LEVEL_9`"]
pub struct AGC_LUT_4_AGC_LEVEL_9_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_LUT_4_AGC_LEVEL_9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_LUT_4_AGC_LEVEL_9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_lut_4_agc_level_9_default(self) -> &'a mut W {
        self.variant(AGC_LUT_4_AGC_LEVEL_9_A::AGC_LUT_4_AGC_LEVEL_9_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 3)) | (((value as u32) & 0x07ff) << 3);
        self.w
    }
}
#[doc = "Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AGC_LUT_4_AGC_LEVEL_8_HI_A {
    #[doc = "0: `0`"]
    AGC_LUT_4_AGC_LEVEL_8_HI_DEFAULT = 0,
}
impl From<AGC_LUT_4_AGC_LEVEL_8_HI_A> for u8 {
    #[inline(always)]
    fn from(variant: AGC_LUT_4_AGC_LEVEL_8_HI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_LUT_4_AGC_LEVEL_8_HI`"]
pub type AGC_LUT_4_AGC_LEVEL_8_HI_R = crate::R<u8, AGC_LUT_4_AGC_LEVEL_8_HI_A>;
impl AGC_LUT_4_AGC_LEVEL_8_HI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AGC_LUT_4_AGC_LEVEL_8_HI_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AGC_LUT_4_AGC_LEVEL_8_HI_A::AGC_LUT_4_AGC_LEVEL_8_HI_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_LUT_4_AGC_LEVEL_8_HI_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_lut_4_agc_level_8_hi_default(&self) -> bool {
        *self == AGC_LUT_4_AGC_LEVEL_8_HI_A::AGC_LUT_4_AGC_LEVEL_8_HI_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_LUT_4_AGC_LEVEL_8_HI`"]
pub struct AGC_LUT_4_AGC_LEVEL_8_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_LUT_4_AGC_LEVEL_8_HI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_LUT_4_AGC_LEVEL_8_HI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_lut_4_agc_level_8_hi_default(self) -> &'a mut W {
        self.variant(AGC_LUT_4_AGC_LEVEL_8_HI_A::AGC_LUT_4_AGC_LEVEL_8_HI_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_4_agc_level_11_lo(&self) -> AGC_LUT_4_AGC_LEVEL_11_LO_R {
        AGC_LUT_4_AGC_LEVEL_11_LO_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bits 14:24 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_4_agc_level_10(&self) -> AGC_LUT_4_AGC_LEVEL_10_R {
        AGC_LUT_4_AGC_LEVEL_10_R::new(((self.bits >> 14) & 0x07ff) as u16)
    }
    #[doc = "Bits 3:13 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_4_agc_level_9(&self) -> AGC_LUT_4_AGC_LEVEL_9_R {
        AGC_LUT_4_AGC_LEVEL_9_R::new(((self.bits >> 3) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:2 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_4_agc_level_8_hi(&self) -> AGC_LUT_4_AGC_LEVEL_8_HI_R {
        AGC_LUT_4_AGC_LEVEL_8_HI_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 25:31 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_4_agc_level_11_lo(&mut self) -> AGC_LUT_4_AGC_LEVEL_11_LO_W {
        AGC_LUT_4_AGC_LEVEL_11_LO_W { w: self }
    }
    #[doc = "Bits 14:24 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_4_agc_level_10(&mut self) -> AGC_LUT_4_AGC_LEVEL_10_W {
        AGC_LUT_4_AGC_LEVEL_10_W { w: self }
    }
    #[doc = "Bits 3:13 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_4_agc_level_9(&mut self) -> AGC_LUT_4_AGC_LEVEL_9_W {
        AGC_LUT_4_AGC_LEVEL_9_W { w: self }
    }
    #[doc = "Bits 0:2 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_4_agc_level_8_hi(&mut self) -> AGC_LUT_4_AGC_LEVEL_8_HI_W {
        AGC_LUT_4_AGC_LEVEL_8_HI_W { w: self }
    }
}
