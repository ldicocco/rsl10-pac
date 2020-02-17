#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog timer Configuration Register"]
    pub watchdog_cfg: WATCHDOG_CFG,
    #[doc = "0x04 - Watchdog Refresh Control Register"]
    pub watchdog_ctrl: WATCHDOG_CTRL,
}
#[doc = "Watchdog timer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [watchdog_cfg](watchdog_cfg) module"]
pub type WATCHDOG_CFG = crate::Reg<u32, _WATCHDOG_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WATCHDOG_CFG;
#[doc = "`read()` method returns [watchdog_cfg::R](watchdog_cfg::R) reader structure"]
impl crate::Readable for WATCHDOG_CFG {}
#[doc = "`write(|w| ..)` method takes [watchdog_cfg::W](watchdog_cfg::W) writer structure"]
impl crate::Writable for WATCHDOG_CFG {}
#[doc = "Watchdog timer Configuration Register"]
pub mod watchdog_cfg;
#[doc = "Watchdog Refresh Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [watchdog_ctrl](watchdog_ctrl) module"]
pub type WATCHDOG_CTRL = crate::Reg<u32, _WATCHDOG_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WATCHDOG_CTRL;
#[doc = "`read()` method returns [watchdog_ctrl::R](watchdog_ctrl::R) reader structure"]
impl crate::Readable for WATCHDOG_CTRL {}
#[doc = "`write(|w| ..)` method takes [watchdog_ctrl::W](watchdog_ctrl::W) writer structure"]
impl crate::Writable for WATCHDOG_CTRL {}
#[doc = "Watchdog Refresh Control Register"]
pub mod watchdog_ctrl;
