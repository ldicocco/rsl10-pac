#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip ID number"]
    pub ahbregs_chip_id_num: AHBREGS_CHIP_ID_NUM,
}
#[doc = "Chip ID number\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbregs_chip_id_num](ahbregs_chip_id_num) module"]
pub type AHBREGS_CHIP_ID_NUM = crate::Reg<u32, _AHBREGS_CHIP_ID_NUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBREGS_CHIP_ID_NUM;
#[doc = "`read()` method returns [ahbregs_chip_id_num::R](ahbregs_chip_id_num::R) reader structure"]
impl crate::Readable for AHBREGS_CHIP_ID_NUM {}
#[doc = "Chip ID number"]
pub mod ahbregs_chip_id_num;
