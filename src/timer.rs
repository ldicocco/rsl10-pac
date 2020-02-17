#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Configuration Registers"]
    pub timer_cfg: [TIMER_CFG; 4],
    #[doc = "0x10 - General-Purpose timer Control / Status Register"]
    pub timer_ctrl: [TIMER_CTRL; 4],
    #[doc = "0x20 - Timer Current Value Register"]
    pub timer_val: [TIMER_VAL; 4],
}
#[doc = "Timer Configuration Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_cfg](timer_cfg) module"]
pub type TIMER_CFG = crate::Reg<u32, _TIMER_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER_CFG;
#[doc = "`read()` method returns [timer_cfg::R](timer_cfg::R) reader structure"]
impl crate::Readable for TIMER_CFG {}
#[doc = "`write(|w| ..)` method takes [timer_cfg::W](timer_cfg::W) writer structure"]
impl crate::Writable for TIMER_CFG {}
#[doc = "Timer Configuration Registers"]
pub mod timer_cfg;
#[doc = "General-Purpose timer Control / Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_ctrl](timer_ctrl) module"]
pub type TIMER_CTRL = crate::Reg<u32, _TIMER_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER_CTRL;
#[doc = "`read()` method returns [timer_ctrl::R](timer_ctrl::R) reader structure"]
impl crate::Readable for TIMER_CTRL {}
#[doc = "`write(|w| ..)` method takes [timer_ctrl::W](timer_ctrl::W) writer structure"]
impl crate::Writable for TIMER_CTRL {}
#[doc = "General-Purpose timer Control / Status Register"]
pub mod timer_ctrl;
#[doc = "Timer Current Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_val](timer_val) module"]
pub type TIMER_VAL = crate::Reg<u32, _TIMER_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER_VAL;
#[doc = "`read()` method returns [timer_val::R](timer_val::R) reader structure"]
impl crate::Readable for TIMER_VAL {}
#[doc = "Timer Current Value Register"]
pub mod timer_val;
