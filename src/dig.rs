#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reset status register"]
    pub dig_reset_status: DIG_RESET_STATUS,
}
#[doc = "Reset status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_reset_status](dig_reset_status) module"]
pub type DIG_RESET_STATUS = crate::Reg<u32, _DIG_RESET_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIG_RESET_STATUS;
#[doc = "`read()` method returns [dig_reset_status::R](dig_reset_status::R) reader structure"]
impl crate::Readable for DIG_RESET_STATUS {}
#[doc = "`write(|w| ..)` method takes [dig_reset_status::W](dig_reset_status::W) writer structure"]
impl crate::Writable for DIG_RESET_STATUS {}
#[doc = "Reset status register"]
pub mod dig_reset_status;
