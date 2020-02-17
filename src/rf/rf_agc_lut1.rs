#[doc = "Reader of register RF_AGC_LUT1"]
pub type R = crate::R<u32, super::RF_AGC_LUT1>;
#[doc = "Writer for register RF_AGC_LUT1"]
pub type W = crate::W<u32, super::RF_AGC_LUT1>;
#[doc = "Register RF_AGC_LUT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_AGC_LUT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum AGC_LUT_1_AGC_LEVEL_2_LO_A {
    #[doc = "0: `0`"]
    AGC_LUT_1_AGC_LEVEL_2_LO_DEFAULT = 0,
}
impl From<AGC_LUT_1_AGC_LEVEL_2_LO_A> for u16 {
    #[inline(always)]
    fn from(variant: AGC_LUT_1_AGC_LEVEL_2_LO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_LUT_1_AGC_LEVEL_2_LO`"]
pub type AGC_LUT_1_AGC_LEVEL_2_LO_R = crate::R<u16, AGC_LUT_1_AGC_LEVEL_2_LO_A>;
impl AGC_LUT_1_AGC_LEVEL_2_LO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, AGC_LUT_1_AGC_LEVEL_2_LO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AGC_LUT_1_AGC_LEVEL_2_LO_A::AGC_LUT_1_AGC_LEVEL_2_LO_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_LUT_1_AGC_LEVEL_2_LO_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_lut_1_agc_level_2_lo_default(&self) -> bool {
        *self == AGC_LUT_1_AGC_LEVEL_2_LO_A::AGC_LUT_1_AGC_LEVEL_2_LO_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_LUT_1_AGC_LEVEL_2_LO`"]
pub struct AGC_LUT_1_AGC_LEVEL_2_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_LUT_1_AGC_LEVEL_2_LO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_LUT_1_AGC_LEVEL_2_LO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_lut_1_agc_level_2_lo_default(self) -> &'a mut W {
        self.variant(AGC_LUT_1_AGC_LEVEL_2_LO_A::AGC_LUT_1_AGC_LEVEL_2_LO_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 22)) | (((value as u32) & 0x03ff) << 22);
        self.w
    }
}
#[doc = "Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum AGC_LUT_1_AGC_LEVEL_1_A {
    #[doc = "0: `0`"]
    AGC_LUT_1_AGC_LEVEL_1_DEFAULT = 0,
}
impl From<AGC_LUT_1_AGC_LEVEL_1_A> for u16 {
    #[inline(always)]
    fn from(variant: AGC_LUT_1_AGC_LEVEL_1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_LUT_1_AGC_LEVEL_1`"]
pub type AGC_LUT_1_AGC_LEVEL_1_R = crate::R<u16, AGC_LUT_1_AGC_LEVEL_1_A>;
impl AGC_LUT_1_AGC_LEVEL_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, AGC_LUT_1_AGC_LEVEL_1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AGC_LUT_1_AGC_LEVEL_1_A::AGC_LUT_1_AGC_LEVEL_1_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_LUT_1_AGC_LEVEL_1_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_lut_1_agc_level_1_default(&self) -> bool {
        *self == AGC_LUT_1_AGC_LEVEL_1_A::AGC_LUT_1_AGC_LEVEL_1_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_LUT_1_AGC_LEVEL_1`"]
pub struct AGC_LUT_1_AGC_LEVEL_1_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_LUT_1_AGC_LEVEL_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_LUT_1_AGC_LEVEL_1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_lut_1_agc_level_1_default(self) -> &'a mut W {
        self.variant(AGC_LUT_1_AGC_LEVEL_1_A::AGC_LUT_1_AGC_LEVEL_1_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | (((value as u32) & 0x07ff) << 11);
        self.w
    }
}
#[doc = "Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum AGC_LUT_1_AGC_LEVEL_0_A {
    #[doc = "0: `0`"]
    AGC_LUT_1_AGC_LEVEL_0_DEFAULT = 0,
}
impl From<AGC_LUT_1_AGC_LEVEL_0_A> for u16 {
    #[inline(always)]
    fn from(variant: AGC_LUT_1_AGC_LEVEL_0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_LUT_1_AGC_LEVEL_0`"]
pub type AGC_LUT_1_AGC_LEVEL_0_R = crate::R<u16, AGC_LUT_1_AGC_LEVEL_0_A>;
impl AGC_LUT_1_AGC_LEVEL_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, AGC_LUT_1_AGC_LEVEL_0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AGC_LUT_1_AGC_LEVEL_0_A::AGC_LUT_1_AGC_LEVEL_0_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_LUT_1_AGC_LEVEL_0_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_lut_1_agc_level_0_default(&self) -> bool {
        *self == AGC_LUT_1_AGC_LEVEL_0_A::AGC_LUT_1_AGC_LEVEL_0_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_LUT_1_AGC_LEVEL_0`"]
pub struct AGC_LUT_1_AGC_LEVEL_0_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_LUT_1_AGC_LEVEL_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_LUT_1_AGC_LEVEL_0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_lut_1_agc_level_0_default(self) -> &'a mut W {
        self.variant(AGC_LUT_1_AGC_LEVEL_0_A::AGC_LUT_1_AGC_LEVEL_0_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:31 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_1_agc_level_2_lo(&self) -> AGC_LUT_1_AGC_LEVEL_2_LO_R {
        AGC_LUT_1_AGC_LEVEL_2_LO_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
    #[doc = "Bits 11:21 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_1_agc_level_1(&self) -> AGC_LUT_1_AGC_LEVEL_1_R {
        AGC_LUT_1_AGC_LEVEL_1_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:10 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_1_agc_level_0(&self) -> AGC_LUT_1_AGC_LEVEL_0_R {
        AGC_LUT_1_AGC_LEVEL_0_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 22:31 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_1_agc_level_2_lo(&mut self) -> AGC_LUT_1_AGC_LEVEL_2_LO_W {
        AGC_LUT_1_AGC_LEVEL_2_LO_W { w: self }
    }
    #[doc = "Bits 11:21 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_1_agc_level_1(&mut self) -> AGC_LUT_1_AGC_LEVEL_1_W {
        AGC_LUT_1_AGC_LEVEL_1_W { w: self }
    }
    #[doc = "Bits 0:10 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_1_agc_level_0(&mut self) -> AGC_LUT_1_AGC_LEVEL_0_W {
        AGC_LUT_1_AGC_LEVEL_0_W { w: self }
    }
}
