#[doc = "Reader of register AUDIO_DMIC1_DATA"]
pub type R = crate::R<u32, super::AUDIO_DMIC1_DATA>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMIC1 input data (LSB or MSB aligned according to AUDIO_CFG); data is sign extended from 16-bit to 32-bit when read in LSB aligned mode or zero padded when read in MSB aligned mode"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
