#[doc = "Reader of register ASRC_STATE_MEM[%s]"]
pub type R = crate::R<u32, super::ASRC_STATE_MEM>;
#[doc = "Writer for register ASRC_STATE_MEM[%s]"]
pub type W = crate::W<u32, super::ASRC_STATE_MEM>;
#[doc = "Register ASRC_STATE_MEM[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::ASRC_STATE_MEM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASRC_STATE_MEM`"]
pub type ASRC_STATE_MEM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ASRC_STATE_MEM`"]
pub struct ASRC_STATE_MEM_W<'a> {
    w: &'a mut W,
}
impl<'a> ASRC_STATE_MEM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ASRC State Memory 0 to 29"]
    #[inline(always)]
    pub fn asrc_state_mem(&self) -> ASRC_STATE_MEM_R {
        ASRC_STATE_MEM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ASRC State Memory 0 to 29"]
    #[inline(always)]
    pub fn asrc_state_mem(&mut self) -> ASRC_STATE_MEM_W {
        ASRC_STATE_MEM_W { w: self }
    }
}
