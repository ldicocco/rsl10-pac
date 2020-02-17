#[doc = "Reader of register BB_WLNBDEV"]
pub type R = crate::R<u32, super::BB_WLNBDEV>;
#[doc = "Writer for register BB_WLNBDEV"]
pub type W = crate::W<u32, super::BB_WLNBDEV>;
#[doc = "Register BB_WLNBDEV `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_WLNBDEV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Number of private devices in the white list\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NBPRIVDEV_A {
    #[doc = "0: `0`"]
    NBPRIVDEV_0 = 0,
}
impl From<NBPRIVDEV_A> for u8 {
    #[inline(always)]
    fn from(variant: NBPRIVDEV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NBPRIVDEV`"]
pub type NBPRIVDEV_R = crate::R<u8, NBPRIVDEV_A>;
impl NBPRIVDEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NBPRIVDEV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NBPRIVDEV_A::NBPRIVDEV_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NBPRIVDEV_0`"]
    #[inline(always)]
    pub fn is_nbprivdev_0(&self) -> bool {
        *self == NBPRIVDEV_A::NBPRIVDEV_0
    }
}
#[doc = "Write proxy for field `NBPRIVDEV`"]
pub struct NBPRIVDEV_W<'a> {
    w: &'a mut W,
}
impl<'a> NBPRIVDEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NBPRIVDEV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nbprivdev_0(self) -> &'a mut W {
        self.variant(NBPRIVDEV_A::NBPRIVDEV_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Number of public devices in the white list\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NBPUBDEV_A {
    #[doc = "0: `0`"]
    NBPUBDEV_0 = 0,
}
impl From<NBPUBDEV_A> for u8 {
    #[inline(always)]
    fn from(variant: NBPUBDEV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NBPUBDEV`"]
pub type NBPUBDEV_R = crate::R<u8, NBPUBDEV_A>;
impl NBPUBDEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NBPUBDEV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NBPUBDEV_A::NBPUBDEV_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NBPUBDEV_0`"]
    #[inline(always)]
    pub fn is_nbpubdev_0(&self) -> bool {
        *self == NBPUBDEV_A::NBPUBDEV_0
    }
}
#[doc = "Write proxy for field `NBPUBDEV`"]
pub struct NBPUBDEV_W<'a> {
    w: &'a mut W,
}
impl<'a> NBPUBDEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NBPUBDEV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nbpubdev_0(self) -> &'a mut W {
        self.variant(NBPUBDEV_A::NBPUBDEV_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Number of private devices in the white list"]
    #[inline(always)]
    pub fn nbprivdev(&self) -> NBPRIVDEV_R {
        NBPRIVDEV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Number of public devices in the white list"]
    #[inline(always)]
    pub fn nbpubdev(&self) -> NBPUBDEV_R {
        NBPUBDEV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Number of private devices in the white list"]
    #[inline(always)]
    pub fn nbprivdev(&mut self) -> NBPRIVDEV_W {
        NBPRIVDEV_W { w: self }
    }
    #[doc = "Bits 0:7 - Number of public devices in the white list"]
    #[inline(always)]
    pub fn nbpubdev(&mut self) -> NBPUBDEV_W {
        NBPUBDEV_W { w: self }
    }
}
