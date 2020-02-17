#[doc = "Reader of register BB_RALNBDEV"]
pub type R = crate::R<u32, super::BB_RALNBDEV>;
#[doc = "Writer for register BB_RALNBDEV"]
pub type W = crate::W<u32, super::BB_RALNBDEV>;
#[doc = "Register BB_RALNBDEV `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_RALNBDEV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Number of devices in RAL Structure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RALNBDEV_A {
    #[doc = "0: `0`"]
    RALNBDEV_0 = 0,
}
impl From<RALNBDEV_A> for u8 {
    #[inline(always)]
    fn from(variant: RALNBDEV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RALNBDEV`"]
pub type RALNBDEV_R = crate::R<u8, RALNBDEV_A>;
impl RALNBDEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RALNBDEV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RALNBDEV_A::RALNBDEV_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RALNBDEV_0`"]
    #[inline(always)]
    pub fn is_ralnbdev_0(&self) -> bool {
        *self == RALNBDEV_A::RALNBDEV_0
    }
}
#[doc = "Write proxy for field `RALNBDEV`"]
pub struct RALNBDEV_W<'a> {
    w: &'a mut W,
}
impl<'a> RALNBDEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RALNBDEV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ralnbdev_0(self) -> &'a mut W {
        self.variant(RALNBDEV_A::RALNBDEV_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of devices in RAL Structure"]
    #[inline(always)]
    pub fn ralnbdev(&self) -> RALNBDEV_R {
        RALNBDEV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of devices in RAL Structure"]
    #[inline(always)]
    pub fn ralnbdev(&mut self) -> RALNBDEV_W {
        RALNBDEV_W { w: self }
    }
}
