#[doc = "Reader of register PCM_RX_DATA"]
pub type R = crate::R<u32, super::PCM_RX_DATA>;
#[doc = "Reader of field `PCM_RX_DATA`"]
pub type PCM_RX_DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data received from the PCM interface"]
    #[inline(always)]
    pub fn pcm_rx_data(&self) -> PCM_RX_DATA_R {
        PCM_RX_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
