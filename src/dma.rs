#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Channel Control and Configuration"]
    pub dma_ctrl0: [DMA_CTRL0; 8],
    #[doc = "0x20 - DMA Channel Source Base Address"]
    pub dma_src_base_addr: [DMA_SRC_BASE_ADDR; 8],
    #[doc = "0x40 - DMA Channel Destination Base Address"]
    pub dma_dest_base_addr: [DMA_DEST_BASE_ADDR; 8],
    #[doc = "0x60 - DMA Channel Transfer Control"]
    pub dma_ctrl1: [DMA_CTRL1; 8],
    #[doc = "0x80 - DMA Channel Next Source Address"]
    pub dma_next_src_addr: [DMA_NEXT_SRC_ADDR; 8],
    #[doc = "0xa0 - DMA Channel Next Destination Address"]
    pub dma_next_dest_addr: [DMA_NEXT_DEST_ADDR; 8],
    #[doc = "0xc0 - DMA Channel Word Count"]
    pub dma_word_cnt: [DMA_WORD_CNT; 8],
    #[doc = "0xe0 - DMA Status channel"]
    pub dma_status: [DMA_STATUS; 8],
}
#[doc = "DMA Channel Control and Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_ctrl0](dma_ctrl0) module"]
pub type DMA_CTRL0 = crate::Reg<u32, _DMA_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CTRL0;
#[doc = "`read()` method returns [dma_ctrl0::R](dma_ctrl0::R) reader structure"]
impl crate::Readable for DMA_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [dma_ctrl0::W](dma_ctrl0::W) writer structure"]
impl crate::Writable for DMA_CTRL0 {}
#[doc = "DMA Channel Control and Configuration"]
pub mod dma_ctrl0;
#[doc = "DMA Channel Source Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_src_base_addr](dma_src_base_addr) module"]
pub type DMA_SRC_BASE_ADDR = crate::Reg<u32, _DMA_SRC_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_SRC_BASE_ADDR;
#[doc = "`read()` method returns [dma_src_base_addr::R](dma_src_base_addr::R) reader structure"]
impl crate::Readable for DMA_SRC_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [dma_src_base_addr::W](dma_src_base_addr::W) writer structure"]
impl crate::Writable for DMA_SRC_BASE_ADDR {}
#[doc = "DMA Channel Source Base Address"]
pub mod dma_src_base_addr;
#[doc = "DMA Channel Destination Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_dest_base_addr](dma_dest_base_addr) module"]
pub type DMA_DEST_BASE_ADDR = crate::Reg<u32, _DMA_DEST_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_DEST_BASE_ADDR;
#[doc = "`read()` method returns [dma_dest_base_addr::R](dma_dest_base_addr::R) reader structure"]
impl crate::Readable for DMA_DEST_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [dma_dest_base_addr::W](dma_dest_base_addr::W) writer structure"]
impl crate::Writable for DMA_DEST_BASE_ADDR {}
#[doc = "DMA Channel Destination Base Address"]
pub mod dma_dest_base_addr;
#[doc = "DMA Channel Transfer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_ctrl1](dma_ctrl1) module"]
pub type DMA_CTRL1 = crate::Reg<u32, _DMA_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CTRL1;
#[doc = "`read()` method returns [dma_ctrl1::R](dma_ctrl1::R) reader structure"]
impl crate::Readable for DMA_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [dma_ctrl1::W](dma_ctrl1::W) writer structure"]
impl crate::Writable for DMA_CTRL1 {}
#[doc = "DMA Channel Transfer Control"]
pub mod dma_ctrl1;
#[doc = "DMA Channel Next Source Address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_next_src_addr](dma_next_src_addr) module"]
pub type DMA_NEXT_SRC_ADDR = crate::Reg<u32, _DMA_NEXT_SRC_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_NEXT_SRC_ADDR;
#[doc = "`read()` method returns [dma_next_src_addr::R](dma_next_src_addr::R) reader structure"]
impl crate::Readable for DMA_NEXT_SRC_ADDR {}
#[doc = "DMA Channel Next Source Address"]
pub mod dma_next_src_addr;
#[doc = "DMA Channel Next Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_next_dest_addr](dma_next_dest_addr) module"]
pub type DMA_NEXT_DEST_ADDR = crate::Reg<u32, _DMA_NEXT_DEST_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_NEXT_DEST_ADDR;
#[doc = "`read()` method returns [dma_next_dest_addr::R](dma_next_dest_addr::R) reader structure"]
impl crate::Readable for DMA_NEXT_DEST_ADDR {}
#[doc = "DMA Channel Next Destination Address"]
pub mod dma_next_dest_addr;
#[doc = "DMA Channel Word Count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_word_cnt](dma_word_cnt) module"]
pub type DMA_WORD_CNT = crate::Reg<u32, _DMA_WORD_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_WORD_CNT;
#[doc = "`read()` method returns [dma_word_cnt::R](dma_word_cnt::R) reader structure"]
impl crate::Readable for DMA_WORD_CNT {}
#[doc = "DMA Channel Word Count"]
pub mod dma_word_cnt;
#[doc = "DMA Status channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_status](dma_status) module"]
pub type DMA_STATUS = crate::Reg<u32, _DMA_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_STATUS;
#[doc = "`read()` method returns [dma_status::R](dma_status::R) reader structure"]
impl crate::Readable for DMA_STATUS {}
#[doc = "`write(|w| ..)` method takes [dma_status::W](dma_status::W) writer structure"]
impl crate::Writable for DMA_STATUS {}
#[doc = "DMA Status channel"]
pub mod dma_status;
