#[doc = "Reader of register ACS_WAKEUP_GP_DATA"]
pub type R = crate::R<u32, super::ACS_WAKEUP_GP_DATA>;
#[doc = "Writer for register ACS_WAKEUP_GP_DATA"]
pub type W = crate::W<u32, super::ACS_WAKEUP_GP_DATA>;
#[doc = "Register ACS_WAKEUP_GP_DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::ACS_WAKEUP_GP_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GP_DATA`"]
pub type GP_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GP_DATA`"]
pub struct GP_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 32-bit General-Purpose RW Data"]
    #[inline(always)]
    pub fn gp_data(&self) -> GP_DATA_R {
        GP_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit General-Purpose RW Data"]
    #[inline(always)]
    pub fn gp_data(&mut self) -> GP_DATA_W {
        GP_DATA_W { w: self }
    }
}
