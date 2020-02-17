#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Baseband controller control register"]
    pub bbif_ctrl: BBIF_CTRL,
    #[doc = "0x04 - Baseband controller status register"]
    pub bbif_status: BBIF_STATUS,
    #[doc = "0x08 - Coexistence control register"]
    pub bbif_coex_ctrl: BBIF_COEX_CTRL,
    #[doc = "0x0c - Coexistence status register"]
    pub bbif_coex_status: BBIF_COEX_STATUS,
    #[doc = "0x10 - Coexistence interrupts configuration register"]
    pub bbif_coex_int_cfg: BBIF_COEX_INT_CFG,
    #[doc = "0x14 - Coexistence interrupt status register"]
    pub bbif_coex_int_status: BBIF_COEX_INT_STATUS,
    #[doc = "0x18 - BLE and RF link synchronization configuration register"]
    pub bbif_sync_cfg: BBIF_SYNC_CFG,
}
#[doc = "Baseband controller control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbif_ctrl](bbif_ctrl) module"]
pub type BBIF_CTRL = crate::Reg<u32, _BBIF_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BBIF_CTRL;
#[doc = "`read()` method returns [bbif_ctrl::R](bbif_ctrl::R) reader structure"]
impl crate::Readable for BBIF_CTRL {}
#[doc = "`write(|w| ..)` method takes [bbif_ctrl::W](bbif_ctrl::W) writer structure"]
impl crate::Writable for BBIF_CTRL {}
#[doc = "Baseband controller control register"]
pub mod bbif_ctrl;
#[doc = "Baseband controller status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbif_status](bbif_status) module"]
pub type BBIF_STATUS = crate::Reg<u32, _BBIF_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BBIF_STATUS;
#[doc = "`read()` method returns [bbif_status::R](bbif_status::R) reader structure"]
impl crate::Readable for BBIF_STATUS {}
#[doc = "Baseband controller status register"]
pub mod bbif_status;
#[doc = "Coexistence control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbif_coex_ctrl](bbif_coex_ctrl) module"]
pub type BBIF_COEX_CTRL = crate::Reg<u32, _BBIF_COEX_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BBIF_COEX_CTRL;
#[doc = "`read()` method returns [bbif_coex_ctrl::R](bbif_coex_ctrl::R) reader structure"]
impl crate::Readable for BBIF_COEX_CTRL {}
#[doc = "`write(|w| ..)` method takes [bbif_coex_ctrl::W](bbif_coex_ctrl::W) writer structure"]
impl crate::Writable for BBIF_COEX_CTRL {}
#[doc = "Coexistence control register"]
pub mod bbif_coex_ctrl;
#[doc = "Coexistence status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbif_coex_status](bbif_coex_status) module"]
pub type BBIF_COEX_STATUS = crate::Reg<u32, _BBIF_COEX_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BBIF_COEX_STATUS;
#[doc = "`read()` method returns [bbif_coex_status::R](bbif_coex_status::R) reader structure"]
impl crate::Readable for BBIF_COEX_STATUS {}
#[doc = "Coexistence status register"]
pub mod bbif_coex_status;
#[doc = "Coexistence interrupts configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbif_coex_int_cfg](bbif_coex_int_cfg) module"]
pub type BBIF_COEX_INT_CFG = crate::Reg<u32, _BBIF_COEX_INT_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BBIF_COEX_INT_CFG;
#[doc = "`read()` method returns [bbif_coex_int_cfg::R](bbif_coex_int_cfg::R) reader structure"]
impl crate::Readable for BBIF_COEX_INT_CFG {}
#[doc = "`write(|w| ..)` method takes [bbif_coex_int_cfg::W](bbif_coex_int_cfg::W) writer structure"]
impl crate::Writable for BBIF_COEX_INT_CFG {}
#[doc = "Coexistence interrupts configuration register"]
pub mod bbif_coex_int_cfg;
#[doc = "Coexistence interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbif_coex_int_status](bbif_coex_int_status) module"]
pub type BBIF_COEX_INT_STATUS = crate::Reg<u32, _BBIF_COEX_INT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BBIF_COEX_INT_STATUS;
#[doc = "`read()` method returns [bbif_coex_int_status::R](bbif_coex_int_status::R) reader structure"]
impl crate::Readable for BBIF_COEX_INT_STATUS {}
#[doc = "Coexistence interrupt status register"]
pub mod bbif_coex_int_status;
#[doc = "BLE and RF link synchronization configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbif_sync_cfg](bbif_sync_cfg) module"]
pub type BBIF_SYNC_CFG = crate::Reg<u32, _BBIF_SYNC_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BBIF_SYNC_CFG;
#[doc = "`read()` method returns [bbif_sync_cfg::R](bbif_sync_cfg::R) reader structure"]
impl crate::Readable for BBIF_SYNC_CFG {}
#[doc = "`write(|w| ..)` method takes [bbif_sync_cfg::W](bbif_sync_cfg::W) writer structure"]
impl crate::Writable for BBIF_SYNC_CFG {}
#[doc = "BLE and RF link synchronization configuration register"]
pub mod bbif_sync_cfg;
