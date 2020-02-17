#[doc = "Reader of register ASRC_OUT"]
pub type R = crate::R<u32, super::ASRC_OUT>;
#[doc = "Writer for register ASRC_OUT"]
pub type W = crate::W<u32, super::ASRC_OUT>;
#[doc = "Register ASRC_OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::ASRC_OUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASRC_OUT`"]
pub type ASRC_OUT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ASRC_OUT`"]
pub struct ASRC_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ASRC_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Audio sample output"]
    #[inline(always)]
    pub fn asrc_out(&self) -> ASRC_OUT_R {
        ASRC_OUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Audio sample output"]
    #[inline(always)]
    pub fn asrc_out(&mut self) -> ASRC_OUT_W {
        ASRC_OUT_W { w: self }
    }
}
