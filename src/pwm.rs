#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Configuration Register"]
    pub pwm_cfg: [PWM_CFG; 2],
    #[doc = "0x08 - PWM 0 and 1 Control Register"]
    pub pwm_ctrl: PWM_CTRL,
}
#[doc = "PWM Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cfg](pwm_cfg) module"]
pub type PWM_CFG = crate::Reg<u32, _PWM_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_CFG;
#[doc = "`read()` method returns [pwm_cfg::R](pwm_cfg::R) reader structure"]
impl crate::Readable for PWM_CFG {}
#[doc = "`write(|w| ..)` method takes [pwm_cfg::W](pwm_cfg::W) writer structure"]
impl crate::Writable for PWM_CFG {}
#[doc = "PWM Configuration Register"]
pub mod pwm_cfg;
#[doc = "PWM 0 and 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ctrl](pwm_ctrl) module"]
pub type PWM_CTRL = crate::Reg<u32, _PWM_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_CTRL;
#[doc = "`read()` method returns [pwm_ctrl::R](pwm_ctrl::R) reader structure"]
impl crate::Readable for PWM_CTRL {}
#[doc = "`write(|w| ..)` method takes [pwm_ctrl::W](pwm_ctrl::W) writer structure"]
impl crate::Writable for PWM_CTRL {}
#[doc = "PWM 0 and 1 Control Register"]
pub mod pwm_ctrl;
