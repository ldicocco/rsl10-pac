#[doc = "Reader of register FLASH_COPY_DST_ADDR_PTR"]
pub type R = crate::R<u32, super::FLASH_COPY_DST_ADDR_PTR>;
#[doc = "Writer for register FLASH_COPY_DST_ADDR_PTR"]
pub type W = crate::W<u32, super::FLASH_COPY_DST_ADDR_PTR>;
#[doc = "Register FLASH_COPY_DST_ADDR_PTR `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_COPY_DST_ADDR_PTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COPY_DST_ADDR_PTR`"]
pub type COPY_DST_ADDR_PTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COPY_DST_ADDR_PTR`"]
pub struct COPY_DST_ADDR_PTR_W<'a> {
    w: &'a mut W,
}
impl<'a> COPY_DST_ADDR_PTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Destination address pointer"]
    #[inline(always)]
    pub fn copy_dst_addr_ptr(&self) -> COPY_DST_ADDR_PTR_R {
        COPY_DST_ADDR_PTR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Destination address pointer"]
    #[inline(always)]
    pub fn copy_dst_addr_ptr(&mut self) -> COPY_DST_ADDR_PTR_W {
        COPY_DST_ADDR_PTR_W { w: self }
    }
}