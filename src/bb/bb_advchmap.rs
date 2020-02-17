#[doc = "Reader of register BB_ADVCHMAP"]
pub type R = crate::R<u32, super::BB_ADVCHMAP>;
#[doc = "Writer for register BB_ADVCHMAP"]
pub type W = crate::W<u32, super::BB_ADVCHMAP>;
#[doc = "Register BB_ADVCHMAP `reset()`'s with value 0x07"]
impl crate::ResetValue for super::BB_ADVCHMAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Advertising channel map, defined as per the advertising connection settings. Contains advertising channels index 37 to 39\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADVCHMAP_A {
    #[doc = "7: `111`"]
    ADVCHMAP_7 = 7,
}
impl From<ADVCHMAP_A> for u8 {
    #[inline(always)]
    fn from(variant: ADVCHMAP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADVCHMAP`"]
pub type ADVCHMAP_R = crate::R<u8, ADVCHMAP_A>;
impl ADVCHMAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADVCHMAP_A> {
        use crate::Variant::*;
        match self.bits {
            7 => Val(ADVCHMAP_A::ADVCHMAP_7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADVCHMAP_7`"]
    #[inline(always)]
    pub fn is_advchmap_7(&self) -> bool {
        *self == ADVCHMAP_A::ADVCHMAP_7
    }
}
#[doc = "Write proxy for field `ADVCHMAP`"]
pub struct ADVCHMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVCHMAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADVCHMAP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn advchmap_7(self) -> &'a mut W {
        self.variant(ADVCHMAP_A::ADVCHMAP_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Advertising channel map, defined as per the advertising connection settings. Contains advertising channels index 37 to 39"]
    #[inline(always)]
    pub fn advchmap(&self) -> ADVCHMAP_R {
        ADVCHMAP_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Advertising channel map, defined as per the advertising connection settings. Contains advertising channels index 37 to 39"]
    #[inline(always)]
    pub fn advchmap(&mut self) -> ADVCHMAP_W {
        ADVCHMAP_W { w: self }
    }
}
