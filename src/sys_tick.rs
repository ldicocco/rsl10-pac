#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SYSTICK Control and Status Register"]
    pub sys_tick_ctrl: SYSTICK_CTRL,
    #[doc = "0x04 - SYSTICK Reload Value Register"]
    pub sys_tick_load: SYSTICK_LOAD,
    #[doc = "0x08 - SYSTICK Current Value Register"]
    pub sys_tick_val: SYSTICK_VAL,
    #[doc = "0x0c - SYSTICK Calibration Register"]
    pub sys_tick_calib: SYSTICK_CALIB,
}
#[doc = "SYSTICK Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_tick_ctrl](sys_tick_ctrl) module"]
pub type SYSTICK_CTRL = crate::Reg<u32, _SYSTICK_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTICK_CTRL;
#[doc = "`read()` method returns [sys_tick_ctrl::R](sys_tick_ctrl::R) reader structure"]
impl crate::Readable for SYSTICK_CTRL {}
#[doc = "`write(|w| ..)` method takes [sys_tick_ctrl::W](sys_tick_ctrl::W) writer structure"]
impl crate::Writable for SYSTICK_CTRL {}
#[doc = "SYSTICK Control and Status Register"]
pub mod sys_tick_ctrl;
#[doc = "SYSTICK Reload Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_tick_load](sys_tick_load) module"]
pub type SYSTICK_LOAD = crate::Reg<u32, _SYSTICK_LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTICK_LOAD;
#[doc = "`read()` method returns [sys_tick_load::R](sys_tick_load::R) reader structure"]
impl crate::Readable for SYSTICK_LOAD {}
#[doc = "`write(|w| ..)` method takes [sys_tick_load::W](sys_tick_load::W) writer structure"]
impl crate::Writable for SYSTICK_LOAD {}
#[doc = "SYSTICK Reload Value Register"]
pub mod sys_tick_load;
#[doc = "SYSTICK Current Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_tick_val](sys_tick_val) module"]
pub type SYSTICK_VAL = crate::Reg<u32, _SYSTICK_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTICK_VAL;
#[doc = "`read()` method returns [sys_tick_val::R](sys_tick_val::R) reader structure"]
impl crate::Readable for SYSTICK_VAL {}
#[doc = "`write(|w| ..)` method takes [sys_tick_val::W](sys_tick_val::W) writer structure"]
impl crate::Writable for SYSTICK_VAL {}
#[doc = "SYSTICK Current Value Register"]
pub mod sys_tick_val;
#[doc = "SYSTICK Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_tick_calib](sys_tick_calib) module"]
pub type SYSTICK_CALIB = crate::Reg<u32, _SYSTICK_CALIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTICK_CALIB;
#[doc = "`read()` method returns [sys_tick_calib::R](sys_tick_calib::R) reader structure"]
impl crate::Readable for SYSTICK_CALIB {}
#[doc = "SYSTICK Calibration Register"]
pub mod sys_tick_calib;
