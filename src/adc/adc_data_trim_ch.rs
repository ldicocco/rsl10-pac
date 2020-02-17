#[doc = "Reader of register ADC_DATA_TRIM_CH[%s]"]
pub type R = crate::R<u32, super::ADC_DATA_TRIM_CH>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - 14-bit ADC conversion result"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x3fff) as u16)
    }
}
