#[doc = "Reader of register BB_BASETIMECNT"]
pub type R = crate::R<u32, super::BB_BASETIMECNT>;
#[doc = "Writer for register BB_BASETIMECNT"]
pub type W = crate::W<u32, super::BB_BASETIMECNT>;
#[doc = "Register BB_BASETIMECNT `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_BASETIMECNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sample the base time counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMP_A {
    #[doc = "0: `0`"]
    SAMP_0 = 0,
    #[doc = "1: Samples the base time counter value in BASETIMECNT register and resets at 0 when action is performed"]
    SAMP_1 = 1,
}
impl From<SAMP_A> for bool {
    #[inline(always)]
    fn from(variant: SAMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAMP`"]
pub type SAMP_R = crate::R<bool, SAMP_A>;
impl SAMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMP_A {
        match self.bits {
            false => SAMP_A::SAMP_0,
            true => SAMP_A::SAMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAMP_0`"]
    #[inline(always)]
    pub fn is_samp_0(&self) -> bool {
        *self == SAMP_A::SAMP_0
    }
    #[doc = "Checks if the value of the field is `SAMP_1`"]
    #[inline(always)]
    pub fn is_samp_1(&self) -> bool {
        *self == SAMP_A::SAMP_1
    }
}
#[doc = "Write proxy for field `SAMP`"]
pub struct SAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn samp_0(self) -> &'a mut W {
        self.variant(SAMP_A::SAMP_0)
    }
    #[doc = "Samples the base time counter value in BASETIMECNT register and resets at 0 when action is performed"]
    #[inline(always)]
    pub fn samp_1(self) -> &'a mut W {
        self.variant(SAMP_A::SAMP_1)
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
#[doc = "Value of the 625us base time reference counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum BASETIMECNT_A {
    #[doc = "0: `0`"]
    BASETIMECNT_0 = 0,
}
impl From<BASETIMECNT_A> for u32 {
    #[inline(always)]
    fn from(variant: BASETIMECNT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BASETIMECNT`"]
pub type BASETIMECNT_R = crate::R<u32, BASETIMECNT_A>;
impl BASETIMECNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, BASETIMECNT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BASETIMECNT_A::BASETIMECNT_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BASETIMECNT_0`"]
    #[inline(always)]
    pub fn is_basetimecnt_0(&self) -> bool {
        *self == BASETIMECNT_A::BASETIMECNT_0
    }
}
impl R {
    #[doc = "Bit 31 - Sample the base time counter"]
    #[inline(always)]
    pub fn samp(&self) -> SAMP_R {
        SAMP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:26 - Value of the 625us base time reference counter"]
    #[inline(always)]
    pub fn basetimecnt(&self) -> BASETIMECNT_R {
        BASETIMECNT_R::new((self.bits & 0x07ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - Sample the base time counter"]
    #[inline(always)]
    pub fn samp(&mut self) -> SAMP_W {
        SAMP_W { w: self }
    }
}
