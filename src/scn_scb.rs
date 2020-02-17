#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Controller Type Register"]
    pub scn_scb_ictr: SCNSCB_ICTR,
}
#[doc = "Interrupt Controller Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scn_scb_ictr](scn_scb_ictr) module"]
pub type SCNSCB_ICTR = crate::Reg<u32, _SCNSCB_ICTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCNSCB_ICTR;
#[doc = "`read()` method returns [scn_scb_ictr::R](scn_scb_ictr::R) reader structure"]
impl crate::Readable for SCNSCB_ICTR {}
#[doc = "Interrupt Controller Type Register"]
pub mod scn_scb_ictr;
