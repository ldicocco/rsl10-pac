#[doc = "Reader of register DMA_DEST_BASE_ADDR[%s]"]
pub type R = crate::R<u32, super::DMA_DEST_BASE_ADDR>;
#[doc = "Writer for register DMA_DEST_BASE_ADDR[%s]"]
pub type W = crate::W<u32, super::DMA_DEST_BASE_ADDR>;
#[doc = "Register DMA_DEST_BASE_ADDR[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_DEST_BASE_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_DEST_BASE_ADDR`"]
pub type DMA_DEST_BASE_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DMA_DEST_BASE_ADDR`"]
pub struct DMA_DEST_BASE_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_DEST_BASE_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Base address for the destination of data transferred using DMA channel"]
    #[inline(always)]
    pub fn dma_dest_base_addr(&self) -> DMA_DEST_BASE_ADDR_R {
        DMA_DEST_BASE_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address for the destination of data transferred using DMA channel"]
    #[inline(always)]
    pub fn dma_dest_base_addr(&mut self) -> DMA_DEST_BASE_ADDR_W {
        DMA_DEST_BASE_ADDR_W { w: self }
    }
}
