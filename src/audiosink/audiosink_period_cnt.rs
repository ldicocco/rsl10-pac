#[doc = "Reader of register AUDIOSINK_PERIOD_CNT"]
pub type R = crate::R<u32, super::AUDIOSINK_PERIOD_CNT>;
#[doc = "Writer for register AUDIOSINK_PERIOD_CNT"]
pub type W = crate::W<u32, super::AUDIOSINK_PERIOD_CNT>;
#[doc = "Register AUDIOSINK_PERIOD_CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::AUDIOSINK_PERIOD_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PERIOD_CNT`"]
pub type PERIOD_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PERIOD_CNT`"]
pub struct PERIOD_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIOD_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Sink clock period counter value"]
    #[inline(always)]
    pub fn period_cnt(&self) -> PERIOD_CNT_R {
        PERIOD_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sink clock period counter value"]
    #[inline(always)]
    pub fn period_cnt(&mut self) -> PERIOD_CNT_W {
        PERIOD_CNT_W { w: self }
    }
}
