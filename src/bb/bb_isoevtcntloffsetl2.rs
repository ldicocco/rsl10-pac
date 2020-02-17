#[doc = "Reader of register BB_ISOEVTCNTLOFFSETL2"]
pub type R = crate::R<u32, super::BB_ISOEVTCNTLOFFSETL2>;
#[doc = "Writer for register BB_ISOEVTCNTLOFFSETL2"]
pub type W = crate::W<u32, super::BB_ISOEVTCNTLOFFSETL2>;
#[doc = "Register BB_ISOEVTCNTLOFFSETL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_ISOEVTCNTLOFFSETL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LSB part of EVT_CNT_OFFSET2\\[39:0\\]
field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum EVT_CNT_OFFSETL2_A {
    #[doc = "0: `0`"]
    EVT_CNT_OFFSETL2_0 = 0,
}
impl From<EVT_CNT_OFFSETL2_A> for u32 {
    #[inline(always)]
    fn from(variant: EVT_CNT_OFFSETL2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EVT_CNT_OFFSETL2`"]
pub type EVT_CNT_OFFSETL2_R = crate::R<u32, EVT_CNT_OFFSETL2_A>;
impl EVT_CNT_OFFSETL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, EVT_CNT_OFFSETL2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EVT_CNT_OFFSETL2_A::EVT_CNT_OFFSETL2_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EVT_CNT_OFFSETL2_0`"]
    #[inline(always)]
    pub fn is_evt_cnt_offsetl2_0(&self) -> bool {
        *self == EVT_CNT_OFFSETL2_A::EVT_CNT_OFFSETL2_0
    }
}
#[doc = "Write proxy for field `EVT_CNT_OFFSETL2`"]
pub struct EVT_CNT_OFFSETL2_W<'a> {
    w: &'a mut W,
}
impl<'a> EVT_CNT_OFFSETL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVT_CNT_OFFSETL2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn evt_cnt_offsetl2_0(self) -> &'a mut W {
        self.variant(EVT_CNT_OFFSETL2_A::EVT_CNT_OFFSETL2_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - LSB part of EVT_CNT_OFFSET2\\[39:0\\]
field"]
    #[inline(always)]
    pub fn evt_cnt_offsetl2(&self) -> EVT_CNT_OFFSETL2_R {
        EVT_CNT_OFFSETL2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - LSB part of EVT_CNT_OFFSET2\\[39:0\\]
field"]
    #[inline(always)]
    pub fn evt_cnt_offsetl2(&mut self) -> EVT_CNT_OFFSETL2_W {
        EVT_CNT_OFFSETL2_W { w: self }
    }
}
