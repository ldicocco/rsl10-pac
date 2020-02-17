#[doc = "Reader of register DMA_NEXT_DEST_ADDR[%s]"]
pub type R = crate::R<u32, super::DMA_NEXT_DEST_ADDR>;
#[doc = "Reader of field `DMA_NEXT_DEST_ADDR`"]
pub type DMA_NEXT_DEST_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address where the next data to be transferred using DMA channel will be stored"]
    #[inline(always)]
    pub fn dma_next_dest_addr(&self) -> DMA_NEXT_DEST_ADDR_R {
        DMA_NEXT_DEST_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
