#[doc = "Reader of register BB_ISOEVTCNTLOFFSETU0"]
pub type R = crate::R<u32, super::BB_ISOEVTCNTLOFFSETU0>;
#[doc = "Writer for register BB_ISOEVTCNTLOFFSETU0"]
pub type W = crate::W<u32, super::BB_ISOEVTCNTLOFFSETU0>;
#[doc = "Register BB_ISOEVTCNTLOFFSETU0 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_ISOEVTCNTLOFFSETU0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MSB part of EVT_CNT_OFFSET0\\[39:0\\]
field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EVT_CNT_OFFSETU0_A {
    #[doc = "0: `0`"]
    EVT_CNT_OFFSETU0_0 = 0,
}
impl From<EVT_CNT_OFFSETU0_A> for u8 {
    #[inline(always)]
    fn from(variant: EVT_CNT_OFFSETU0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EVT_CNT_OFFSETU0`"]
pub type EVT_CNT_OFFSETU0_R = crate::R<u8, EVT_CNT_OFFSETU0_A>;
impl EVT_CNT_OFFSETU0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EVT_CNT_OFFSETU0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EVT_CNT_OFFSETU0_A::EVT_CNT_OFFSETU0_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EVT_CNT_OFFSETU0_0`"]
    #[inline(always)]
    pub fn is_evt_cnt_offsetu0_0(&self) -> bool {
        *self == EVT_CNT_OFFSETU0_A::EVT_CNT_OFFSETU0_0
    }
}
#[doc = "Write proxy for field `EVT_CNT_OFFSETU0`"]
pub struct EVT_CNT_OFFSETU0_W<'a> {
    w: &'a mut W,
}
impl<'a> EVT_CNT_OFFSETU0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVT_CNT_OFFSETU0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn evt_cnt_offsetu0_0(self) -> &'a mut W {
        self.variant(EVT_CNT_OFFSETU0_A::EVT_CNT_OFFSETU0_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - MSB part of EVT_CNT_OFFSET0\\[39:0\\]
field"]
    #[inline(always)]
    pub fn evt_cnt_offsetu0(&self) -> EVT_CNT_OFFSETU0_R {
        EVT_CNT_OFFSETU0_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - MSB part of EVT_CNT_OFFSET0\\[39:0\\]
field"]
    #[inline(always)]
    pub fn evt_cnt_offsetu0(&mut self) -> EVT_CNT_OFFSETU0_W {
        EVT_CNT_OFFSETU0_W { w: self }
    }
}
