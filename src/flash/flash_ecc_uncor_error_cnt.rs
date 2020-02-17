#[doc = "Reader of register FLASH_ECC_UNCOR_ERROR_CNT"]
pub type R = crate::R<u32, super::FLASH_ECC_UNCOR_ERROR_CNT>;
#[doc = "Reader of field `ECC_UNCOR_ERROR_CNT`"]
pub type ECC_UNCOR_ERROR_CNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Flash ECC uncorrected error counter"]
    #[inline(always)]
    pub fn ecc_uncor_error_cnt(&self) -> ECC_UNCOR_ERROR_CNT_R {
        ECC_UNCOR_ERROR_CNT_R::new((self.bits & 0xff) as u8)
    }
}
