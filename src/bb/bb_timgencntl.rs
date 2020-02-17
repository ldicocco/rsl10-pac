#[doc = "Reader of register BB_TIMGENCNTL"]
pub type R = crate::R<u32, super::BB_TIMGENCNTL>;
#[doc = "Writer for register BB_TIMGENCNTL"]
pub type W = crate::W<u32, super::BB_TIMGENCNTL>;
#[doc = "Register BB_TIMGENCNTL `reset()`'s with value 0x81fe_0096"]
impl crate::ResetValue for super::BB_TIMGENCNTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x81fe_0096
    }
}
#[doc = "Controls the anticipated pre-fetch abort mechanism\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APFM_EN_A {
    #[doc = "0: Disabled"]
    APFM_EN_0 = 0,
    #[doc = "1: Enabled"]
    APFM_EN_1 = 1,
}
impl From<APFM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: APFM_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `APFM_EN`"]
pub type APFM_EN_R = crate::R<bool, APFM_EN_A>;
impl APFM_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APFM_EN_A {
        match self.bits {
            false => APFM_EN_A::APFM_EN_0,
            true => APFM_EN_A::APFM_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `APFM_EN_0`"]
    #[inline(always)]
    pub fn is_apfm_en_0(&self) -> bool {
        *self == APFM_EN_A::APFM_EN_0
    }
    #[doc = "Checks if the value of the field is `APFM_EN_1`"]
    #[inline(always)]
    pub fn is_apfm_en_1(&self) -> bool {
        *self == APFM_EN_A::APFM_EN_1
    }
}
#[doc = "Write proxy for field `APFM_EN`"]
pub struct APFM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APFM_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APFM_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn apfm_en_0(self) -> &'a mut W {
        self.variant(APFM_EN_A::APFM_EN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn apfm_en_1(self) -> &'a mut W {
        self.variant(APFM_EN_A::APFM_EN_1)
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
#[doc = "Defines the instant in us at which immediate abort is required after anticipated pre-fetch abort\n\nValue on reset: 510"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum PREFETCHABORT_TIME_A {
    #[doc = "510: `111111110`"]
    PREFETCHABORT_TIME_254 = 510,
}
impl From<PREFETCHABORT_TIME_A> for u16 {
    #[inline(always)]
    fn from(variant: PREFETCHABORT_TIME_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PREFETCHABORT_TIME`"]
pub type PREFETCHABORT_TIME_R = crate::R<u16, PREFETCHABORT_TIME_A>;
impl PREFETCHABORT_TIME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, PREFETCHABORT_TIME_A> {
        use crate::Variant::*;
        match self.bits {
            510 => Val(PREFETCHABORT_TIME_A::PREFETCHABORT_TIME_254),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PREFETCHABORT_TIME_254`"]
    #[inline(always)]
    pub fn is_prefetchabort_time_254(&self) -> bool {
        *self == PREFETCHABORT_TIME_A::PREFETCHABORT_TIME_254
    }
}
#[doc = "Write proxy for field `PREFETCHABORT_TIME`"]
pub struct PREFETCHABORT_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFETCHABORT_TIME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREFETCHABORT_TIME_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`111111110`"]
    #[inline(always)]
    pub fn prefetchabort_time_254(self) -> &'a mut W {
        self.variant(PREFETCHABORT_TIME_A::PREFETCHABORT_TIME_254)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Defines exchange table pre-fetch instant in us\n\nValue on reset: 150"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum PREFETCH_TIME_A {
    #[doc = "150: `10010110`"]
    PREFETCH_TIME_150 = 150,
}
impl From<PREFETCH_TIME_A> for u16 {
    #[inline(always)]
    fn from(variant: PREFETCH_TIME_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PREFETCH_TIME`"]
pub type PREFETCH_TIME_R = crate::R<u16, PREFETCH_TIME_A>;
impl PREFETCH_TIME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, PREFETCH_TIME_A> {
        use crate::Variant::*;
        match self.bits {
            150 => Val(PREFETCH_TIME_A::PREFETCH_TIME_150),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PREFETCH_TIME_150`"]
    #[inline(always)]
    pub fn is_prefetch_time_150(&self) -> bool {
        *self == PREFETCH_TIME_A::PREFETCH_TIME_150
    }
}
#[doc = "Write proxy for field `PREFETCH_TIME`"]
pub struct PREFETCH_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFETCH_TIME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREFETCH_TIME_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`10010110`"]
    #[inline(always)]
    pub fn prefetch_time_150(self) -> &'a mut W {
        self.variant(PREFETCH_TIME_A::PREFETCH_TIME_150)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Controls the anticipated pre-fetch abort mechanism"]
    #[inline(always)]
    pub fn apfm_en(&self) -> APFM_EN_R {
        APFM_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 16:25 - Defines the instant in us at which immediate abort is required after anticipated pre-fetch abort"]
    #[inline(always)]
    pub fn prefetchabort_time(&self) -> PREFETCHABORT_TIME_R {
        PREFETCHABORT_TIME_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:8 - Defines exchange table pre-fetch instant in us"]
    #[inline(always)]
    pub fn prefetch_time(&self) -> PREFETCH_TIME_R {
        PREFETCH_TIME_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Controls the anticipated pre-fetch abort mechanism"]
    #[inline(always)]
    pub fn apfm_en(&mut self) -> APFM_EN_W {
        APFM_EN_W { w: self }
    }
    #[doc = "Bits 16:25 - Defines the instant in us at which immediate abort is required after anticipated pre-fetch abort"]
    #[inline(always)]
    pub fn prefetchabort_time(&mut self) -> PREFETCHABORT_TIME_W {
        PREFETCHABORT_TIME_W { w: self }
    }
    #[doc = "Bits 0:8 - Defines exchange table pre-fetch instant in us"]
    #[inline(always)]
    pub fn prefetch_time(&mut self) -> PREFETCH_TIME_W {
        PREFETCH_TIME_W { w: self }
    }
}
