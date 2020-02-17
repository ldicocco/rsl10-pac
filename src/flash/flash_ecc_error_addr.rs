#[doc = "Reader of register FLASH_ECC_ERROR_ADDR"]
pub type R = crate::R<u32, super::FLASH_ECC_ERROR_ADDR>;
#[doc = "Reader of field `ECC_ERROR_ADDR`"]
pub type ECC_ERROR_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 2:20 - Store the Flash address of the latest Flash ECC error"]
    #[inline(always)]
    pub fn ecc_error_addr(&self) -> ECC_ERROR_ADDR_R {
        ECC_ERROR_ADDR_R::new(((self.bits >> 2) & 0x0007_ffff) as u32)
    }
}
