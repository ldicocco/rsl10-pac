#[doc = "Reader of register DMA_WORD_CNT[%s]"]
pub type R = crate::R<u32, super::DMA_WORD_CNT>;
#[doc = "Reader of field `DMA_WORD_CNT`"]
pub type DMA_WORD_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - The number of words that have been transferred using DMA channel during the current transfer"]
    #[inline(always)]
    pub fn dma_word_cnt(&self) -> DMA_WORD_CNT_R {
        DMA_WORD_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
