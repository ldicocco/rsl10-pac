#[doc = "Reader of register ACS_BG_CTRL"]
pub type R = crate::R<u32, super::ACS_BG_CTRL>;
#[doc = "Writer for register ACS_BG_CTRL"]
pub type W = crate::W<u32, super::ACS_BG_CTRL>;
#[doc = "Register ACS_BG_CTRL `reset()`'s with value 0x0b1e"]
impl crate::ResetValue for super::ACS_BG_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0b1e
    }
}
#[doc = "Temperature coefficient trimming\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLOPE_TRIM_A {
    #[doc = "11: Temperature dependency 0 ppm/C"]
    BG_SLOPE_TRIM_VALUE = 11,
}
impl From<SLOPE_TRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: SLOPE_TRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SLOPE_TRIM`"]
pub type SLOPE_TRIM_R = crate::R<u8, SLOPE_TRIM_A>;
impl SLOPE_TRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SLOPE_TRIM_A> {
        use crate::Variant::*;
        match self.bits {
            11 => Val(SLOPE_TRIM_A::BG_SLOPE_TRIM_VALUE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BG_SLOPE_TRIM_VALUE`"]
    #[inline(always)]
    pub fn is_bg_slope_trim_value(&self) -> bool {
        *self == SLOPE_TRIM_A::BG_SLOPE_TRIM_VALUE
    }
}
#[doc = "Write proxy for field `SLOPE_TRIM`"]
pub struct SLOPE_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOPE_TRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOPE_TRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Temperature dependency 0 ppm/C"]
    #[inline(always)]
    pub fn bg_slope_trim_value(self) -> &'a mut W {
        self.variant(SLOPE_TRIM_A::BG_SLOPE_TRIM_VALUE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reference voltage trimming (2.5 mV steps)\n\nValue on reset: 30"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VTRIM_A {
    #[doc = "0: 0.6750 V"]
    BG_TRIM_0P675V = 0,
    #[doc = "1: 0.6775 V"]
    BG_TRIM_0P678V = 1,
    #[doc = "29: 0.7475 V"]
    BG_TRIM_0P748V = 29,
    #[doc = "30: 0.7500 V"]
    BG_TRIM_0P750V = 30,
    #[doc = "31: 0.7525 V"]
    BG_TRIM_0P753V = 31,
    #[doc = "62: 0.8300 V"]
    BG_TRIM_0P830V = 62,
    #[doc = "63: 0.8325 V"]
    BG_TRIM_0P833V = 63,
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
            0 => Val(VTRIM_A::BG_TRIM_0P675V),
            1 => Val(VTRIM_A::BG_TRIM_0P678V),
            29 => Val(VTRIM_A::BG_TRIM_0P748V),
            30 => Val(VTRIM_A::BG_TRIM_0P750V),
            31 => Val(VTRIM_A::BG_TRIM_0P753V),
            62 => Val(VTRIM_A::BG_TRIM_0P830V),
            63 => Val(VTRIM_A::BG_TRIM_0P833V),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BG_TRIM_0P675V`"]
    #[inline(always)]
    pub fn is_bg_trim_0p675v(&self) -> bool {
        *self == VTRIM_A::BG_TRIM_0P675V
    }
    #[doc = "Checks if the value of the field is `BG_TRIM_0P678V`"]
    #[inline(always)]
    pub fn is_bg_trim_0p678v(&self) -> bool {
        *self == VTRIM_A::BG_TRIM_0P678V
    }
    #[doc = "Checks if the value of the field is `BG_TRIM_0P748V`"]
    #[inline(always)]
    pub fn is_bg_trim_0p748v(&self) -> bool {
        *self == VTRIM_A::BG_TRIM_0P748V
    }
    #[doc = "Checks if the value of the field is `BG_TRIM_0P750V`"]
    #[inline(always)]
    pub fn is_bg_trim_0p750v(&self) -> bool {
        *self == VTRIM_A::BG_TRIM_0P750V
    }
    #[doc = "Checks if the value of the field is `BG_TRIM_0P753V`"]
    #[inline(always)]
    pub fn is_bg_trim_0p753v(&self) -> bool {
        *self == VTRIM_A::BG_TRIM_0P753V
    }
    #[doc = "Checks if the value of the field is `BG_TRIM_0P830V`"]
    #[inline(always)]
    pub fn is_bg_trim_0p830v(&self) -> bool {
        *self == VTRIM_A::BG_TRIM_0P830V
    }
    #[doc = "Checks if the value of the field is `BG_TRIM_0P833V`"]
    #[inline(always)]
    pub fn is_bg_trim_0p833v(&self) -> bool {
        *self == VTRIM_A::BG_TRIM_0P833V
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
    #[doc = "0.6750 V"]
    #[inline(always)]
    pub fn bg_trim_0p675v(self) -> &'a mut W {
        self.variant(VTRIM_A::BG_TRIM_0P675V)
    }
    #[doc = "0.6775 V"]
    #[inline(always)]
    pub fn bg_trim_0p678v(self) -> &'a mut W {
        self.variant(VTRIM_A::BG_TRIM_0P678V)
    }
    #[doc = "0.7475 V"]
    #[inline(always)]
    pub fn bg_trim_0p748v(self) -> &'a mut W {
        self.variant(VTRIM_A::BG_TRIM_0P748V)
    }
    #[doc = "0.7500 V"]
    #[inline(always)]
    pub fn bg_trim_0p750v(self) -> &'a mut W {
        self.variant(VTRIM_A::BG_TRIM_0P750V)
    }
    #[doc = "0.7525 V"]
    #[inline(always)]
    pub fn bg_trim_0p753v(self) -> &'a mut W {
        self.variant(VTRIM_A::BG_TRIM_0P753V)
    }
    #[doc = "0.8300 V"]
    #[inline(always)]
    pub fn bg_trim_0p830v(self) -> &'a mut W {
        self.variant(VTRIM_A::BG_TRIM_0P830V)
    }
    #[doc = "0.8325 V"]
    #[inline(always)]
    pub fn bg_trim_0p833v(self) -> &'a mut W {
        self.variant(VTRIM_A::BG_TRIM_0P833V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:12 - Temperature coefficient trimming"]
    #[inline(always)]
    pub fn slope_trim(&self) -> SLOPE_TRIM_R {
        SLOPE_TRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:5 - Reference voltage trimming (2.5 mV steps)"]
    #[inline(always)]
    pub fn vtrim(&self) -> VTRIM_R {
        VTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - Temperature coefficient trimming"]
    #[inline(always)]
    pub fn slope_trim(&mut self) -> SLOPE_TRIM_W {
        SLOPE_TRIM_W { w: self }
    }
    #[doc = "Bits 0:5 - Reference voltage trimming (2.5 mV steps)"]
    #[inline(always)]
    pub fn vtrim(&mut self) -> VTRIM_W {
        VTRIM_W { w: self }
    }
}
