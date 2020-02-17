#[doc = "Reader of register RF_AGC_LUT2"]
pub type R = crate::R<u32, super::RF_AGC_LUT2>;
#[doc = "Writer for register RF_AGC_LUT2"]
pub type W = crate::W<u32, super::RF_AGC_LUT2>;
#[doc = "Register RF_AGC_LUT2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_AGC_LUT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum AGC_LUT_2_AGC_LEVEL_5_LO_A {
    #[doc = "0: `0`"]
    AGC_LUT_2_AGC_LEVEL_5_LO_DEFAULT = 0,
}
impl From<AGC_LUT_2_AGC_LEVEL_5_LO_A> for u16 {
    #[inline(always)]
    fn from(variant: AGC_LUT_2_AGC_LEVEL_5_LO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_LUT_2_AGC_LEVEL_5_LO`"]
pub type AGC_LUT_2_AGC_LEVEL_5_LO_R = crate::R<u16, AGC_LUT_2_AGC_LEVEL_5_LO_A>;
impl AGC_LUT_2_AGC_LEVEL_5_LO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, AGC_LUT_2_AGC_LEVEL_5_LO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AGC_LUT_2_AGC_LEVEL_5_LO_A::AGC_LUT_2_AGC_LEVEL_5_LO_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_LUT_2_AGC_LEVEL_5_LO_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_lut_2_agc_level_5_lo_default(&self) -> bool {
        *self == AGC_LUT_2_AGC_LEVEL_5_LO_A::AGC_LUT_2_AGC_LEVEL_5_LO_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_LUT_2_AGC_LEVEL_5_LO`"]
pub struct AGC_LUT_2_AGC_LEVEL_5_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_LUT_2_AGC_LEVEL_5_LO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_LUT_2_AGC_LEVEL_5_LO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_lut_2_agc_level_5_lo_default(self) -> &'a mut W {
        self.variant(AGC_LUT_2_AGC_LEVEL_5_LO_A::AGC_LUT_2_AGC_LEVEL_5_LO_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 23)) | (((value as u32) & 0x01ff) << 23);
        self.w
    }
}
#[doc = "Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum AGC_LUT_2_AGC_LEVEL_4_A {
    #[doc = "0: `0`"]
    AGC_LUT_2_AGC_LEVEL_4_DEFAULT = 0,
}
impl From<AGC_LUT_2_AGC_LEVEL_4_A> for u16 {
    #[inline(always)]
    fn from(variant: AGC_LUT_2_AGC_LEVEL_4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_LUT_2_AGC_LEVEL_4`"]
pub type AGC_LUT_2_AGC_LEVEL_4_R = crate::R<u16, AGC_LUT_2_AGC_LEVEL_4_A>;
impl AGC_LUT_2_AGC_LEVEL_4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, AGC_LUT_2_AGC_LEVEL_4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AGC_LUT_2_AGC_LEVEL_4_A::AGC_LUT_2_AGC_LEVEL_4_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_LUT_2_AGC_LEVEL_4_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_lut_2_agc_level_4_default(&self) -> bool {
        *self == AGC_LUT_2_AGC_LEVEL_4_A::AGC_LUT_2_AGC_LEVEL_4_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_LUT_2_AGC_LEVEL_4`"]
pub struct AGC_LUT_2_AGC_LEVEL_4_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_LUT_2_AGC_LEVEL_4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_LUT_2_AGC_LEVEL_4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_lut_2_agc_level_4_default(self) -> &'a mut W {
        self.variant(AGC_LUT_2_AGC_LEVEL_4_A::AGC_LUT_2_AGC_LEVEL_4_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 12)) | (((value as u32) & 0x07ff) << 12);
        self.w
    }
}
#[doc = "Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum AGC_LUT_2_AGC_LEVEL_3_A {
    #[doc = "0: `0`"]
    AGC_LUT_2_AGC_LEVEL_3_DEFAULT = 0,
}
impl From<AGC_LUT_2_AGC_LEVEL_3_A> for u16 {
    #[inline(always)]
    fn from(variant: AGC_LUT_2_AGC_LEVEL_3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AGC_LUT_2_AGC_LEVEL_3`"]
pub type AGC_LUT_2_AGC_LEVEL_3_R = crate::R<u16, AGC_LUT_2_AGC_LEVEL_3_A>;
impl AGC_LUT_2_AGC_LEVEL_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, AGC_LUT_2_AGC_LEVEL_3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AGC_LUT_2_AGC_LEVEL_3_A::AGC_LUT_2_AGC_LEVEL_3_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_LUT_2_AGC_LEVEL_3_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_lut_2_agc_level_3_default(&self) -> bool {
        *self == AGC_LUT_2_AGC_LEVEL_3_A::AGC_LUT_2_AGC_LEVEL_3_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_LUT_2_AGC_LEVEL_3`"]
pub struct AGC_LUT_2_AGC_LEVEL_3_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_LUT_2_AGC_LEVEL_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_LUT_2_AGC_LEVEL_3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_lut_2_agc_level_3_default(self) -> &'a mut W {
        self.variant(AGC_LUT_2_AGC_LEVEL_3_A::AGC_LUT_2_AGC_LEVEL_3_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 1)) | (((value as u32) & 0x07ff) << 1);
        self.w
    }
}
#[doc = "Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AGC_LUT_2_AGC_LEVEL_2_HI_A {
    #[doc = "0: `0`"]
    AGC_LUT_2_AGC_LEVEL_2_HI_DEFAULT = 0,
}
impl From<AGC_LUT_2_AGC_LEVEL_2_HI_A> for bool {
    #[inline(always)]
    fn from(variant: AGC_LUT_2_AGC_LEVEL_2_HI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AGC_LUT_2_AGC_LEVEL_2_HI`"]
pub type AGC_LUT_2_AGC_LEVEL_2_HI_R = crate::R<bool, AGC_LUT_2_AGC_LEVEL_2_HI_A>;
impl AGC_LUT_2_AGC_LEVEL_2_HI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, AGC_LUT_2_AGC_LEVEL_2_HI_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(AGC_LUT_2_AGC_LEVEL_2_HI_A::AGC_LUT_2_AGC_LEVEL_2_HI_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AGC_LUT_2_AGC_LEVEL_2_HI_DEFAULT`"]
    #[inline(always)]
    pub fn is_agc_lut_2_agc_level_2_hi_default(&self) -> bool {
        *self == AGC_LUT_2_AGC_LEVEL_2_HI_A::AGC_LUT_2_AGC_LEVEL_2_HI_DEFAULT
    }
}
#[doc = "Write proxy for field `AGC_LUT_2_AGC_LEVEL_2_HI`"]
pub struct AGC_LUT_2_AGC_LEVEL_2_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_LUT_2_AGC_LEVEL_2_HI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGC_LUT_2_AGC_LEVEL_2_HI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn agc_lut_2_agc_level_2_hi_default(self) -> &'a mut W {
        self.variant(AGC_LUT_2_AGC_LEVEL_2_HI_A::AGC_LUT_2_AGC_LEVEL_2_HI_DEFAULT)
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
    #[doc = "Bits 23:31 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_2_agc_level_5_lo(&self) -> AGC_LUT_2_AGC_LEVEL_5_LO_R {
        AGC_LUT_2_AGC_LEVEL_5_LO_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:22 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_2_agc_level_4(&self) -> AGC_LUT_2_AGC_LEVEL_4_R {
        AGC_LUT_2_AGC_LEVEL_4_R::new(((self.bits >> 12) & 0x07ff) as u16)
    }
    #[doc = "Bits 1:11 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_2_agc_level_3(&self) -> AGC_LUT_2_AGC_LEVEL_3_R {
        AGC_LUT_2_AGC_LEVEL_3_R::new(((self.bits >> 1) & 0x07ff) as u16)
    }
    #[doc = "Bit 0 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_2_agc_level_2_hi(&self) -> AGC_LUT_2_AGC_LEVEL_2_HI_R {
        AGC_LUT_2_AGC_LEVEL_2_HI_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 23:31 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_2_agc_level_5_lo(&mut self) -> AGC_LUT_2_AGC_LEVEL_5_LO_W {
        AGC_LUT_2_AGC_LEVEL_5_LO_W { w: self }
    }
    #[doc = "Bits 12:22 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_2_agc_level_4(&mut self) -> AGC_LUT_2_AGC_LEVEL_4_W {
        AGC_LUT_2_AGC_LEVEL_4_W { w: self }
    }
    #[doc = "Bits 1:11 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_2_agc_level_3(&mut self) -> AGC_LUT_2_AGC_LEVEL_3_W {
        AGC_LUT_2_AGC_LEVEL_3_W { w: self }
    }
    #[doc = "Bit 0 - Look up table with the AGC values: agc_level_0 is supposed the lowest attenuation, while agc_level_11 is the one with a maximum of attenuation."]
    #[inline(always)]
    pub fn agc_lut_2_agc_level_2_hi(&mut self) -> AGC_LUT_2_AGC_LEVEL_2_HI_W {
        AGC_LUT_2_AGC_LEVEL_2_HI_W { w: self }
    }
}
