#[doc = "Reader of register ADC_DATA_AUDIO_CH[%s]"]
pub type R = crate::R<u32, super::ADC_DATA_AUDIO_CH>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 14-bit ADC conversion result (sign extended to 32 bits)"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
