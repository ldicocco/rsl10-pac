#[doc = "Reader of register FLASH_COPY_SRC_ADDR_PTR"]
pub type R = crate::R<u32, super::FLASH_COPY_SRC_ADDR_PTR>;
#[doc = "Writer for register FLASH_COPY_SRC_ADDR_PTR"]
pub type W = crate::W<u32, super::FLASH_COPY_SRC_ADDR_PTR>;
#[doc = "Register FLASH_COPY_SRC_ADDR_PTR `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_COPY_SRC_ADDR_PTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COPY_SRC_ADDR_PTR`"]
pub type COPY_SRC_ADDR_PTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COPY_SRC_ADDR_PTR`"]
pub struct COPY_SRC_ADDR_PTR_W<'a> {
    w: &'a mut W,
}
impl<'a> COPY_SRC_ADDR_PTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x001f_ffff) | ((value as u32) & 0x001f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:20 - Source address pointer"]
    #[inline(always)]
    pub fn copy_src_addr_ptr(&self) -> COPY_SRC_ADDR_PTR_R {
        COPY_SRC_ADDR_PTR_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:20 - Source address pointer"]
    #[inline(always)]
    pub fn copy_src_addr_ptr(&mut self) -> COPY_SRC_ADDR_PTR_W {
        COPY_SRC_ADDR_PTR_W { w: self }
    }
}
