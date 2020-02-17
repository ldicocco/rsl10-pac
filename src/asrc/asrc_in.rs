#[doc = "Reader of register ASRC_IN"]
pub type R = crate::R<u32, super::ASRC_IN>;
#[doc = "Writer for register ASRC_IN"]
pub type W = crate::W<u32, super::ASRC_IN>;
#[doc = "Register ASRC_IN `reset()`'s with value 0"]
impl crate::ResetValue for super::ASRC_IN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASRC_IN`"]
pub type ASRC_IN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ASRC_IN`"]
pub struct ASRC_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> ASRC_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Audio sample input"]
    #[inline(always)]
    pub fn asrc_in(&self) -> ASRC_IN_R {
        ASRC_IN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Audio sample input"]
    #[inline(always)]
    pub fn asrc_in(&mut self) -> ASRC_IN_W {
        ASRC_IN_W { w: self }
    }
}
