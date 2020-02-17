#[doc = "Reader of register AUDIO_DMIC_DATA"]
pub type R = crate::R<u32, super::AUDIO_DMIC_DATA>;
#[doc = "Reader of field `DMIC1_DATA`"]
pub type DMIC1_DATA_R = crate::R<u16, u16>;
#[doc = "Reader of field `DMIC0_DATA`"]
pub type DMIC0_DATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - DMIC1 input data (16-bit)"]
    #[inline(always)]
    pub fn dmic1_data(&self) -> DMIC1_DATA_R {
        DMIC1_DATA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - DMIC0 input data (16-bit)"]
    #[inline(always)]
    pub fn dmic0_data(&self) -> DMIC0_DATA_R {
        DMIC0_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
