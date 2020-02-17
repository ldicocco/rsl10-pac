#[doc = "Reader of register SYSCTRL_FLASH_READ_CNT"]
pub type R = crate::R<u32, super::SYSCTRL_FLASH_READ_CNT>;
#[doc = "Writer for register SYSCTRL_FLASH_READ_CNT"]
pub type W = crate::W<u32, super::SYSCTRL_FLASH_READ_CNT>;
#[doc = "Register SYSCTRL_FLASH_READ_CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCTRL_FLASH_READ_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_READ_CNT`"]
pub type FLASH_READ_CNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FLASH_READ_CNT`"]
pub struct FLASH_READ_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_READ_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Flash read access counter value"]
    #[inline(always)]
    pub fn flash_read_cnt(&self) -> FLASH_READ_CNT_R {
        FLASH_READ_CNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Flash read access counter value"]
    #[inline(always)]
    pub fn flash_read_cnt(&mut self) -> FLASH_READ_CNT_W {
        FLASH_READ_CNT_W { w: self }
    }
}
