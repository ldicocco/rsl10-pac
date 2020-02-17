#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Audio Sink Clock Control Register"]
    pub audiosink_ctrl: AUDIOSINK_CTRL,
    #[doc = "0x04 - Audio Sink Clock Configuration Register"]
    pub audiosink_cfg: AUDIOSINK_CFG,
    #[doc = "0x08 - Audio Sink Clock Counter Register"]
    pub audiosink_cnt: AUDIOSINK_CNT,
    #[doc = "0x0c - Audio Sink Clock Phase Counter Register"]
    pub audiosink_phase_cnt: AUDIOSINK_PHASE_CNT,
    #[doc = "0x10 - Audio Sink Clock Period Counter Register"]
    pub audiosink_period_cnt: AUDIOSINK_PERIOD_CNT,
}
#[doc = "Audio Sink Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audiosink_ctrl](audiosink_ctrl) module"]
pub type AUDIOSINK_CTRL = crate::Reg<u32, _AUDIOSINK_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDIOSINK_CTRL;
#[doc = "`read()` method returns [audiosink_ctrl::R](audiosink_ctrl::R) reader structure"]
impl crate::Readable for AUDIOSINK_CTRL {}
#[doc = "`write(|w| ..)` method takes [audiosink_ctrl::W](audiosink_ctrl::W) writer structure"]
impl crate::Writable for AUDIOSINK_CTRL {}
#[doc = "Audio Sink Clock Control Register"]
pub mod audiosink_ctrl;
#[doc = "Audio Sink Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audiosink_cfg](audiosink_cfg) module"]
pub type AUDIOSINK_CFG = crate::Reg<u32, _AUDIOSINK_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDIOSINK_CFG;
#[doc = "`read()` method returns [audiosink_cfg::R](audiosink_cfg::R) reader structure"]
impl crate::Readable for AUDIOSINK_CFG {}
#[doc = "`write(|w| ..)` method takes [audiosink_cfg::W](audiosink_cfg::W) writer structure"]
impl crate::Writable for AUDIOSINK_CFG {}
#[doc = "Audio Sink Clock Configuration Register"]
pub mod audiosink_cfg;
#[doc = "Audio Sink Clock Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audiosink_cnt](audiosink_cnt) module"]
pub type AUDIOSINK_CNT = crate::Reg<u32, _AUDIOSINK_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDIOSINK_CNT;
#[doc = "`read()` method returns [audiosink_cnt::R](audiosink_cnt::R) reader structure"]
impl crate::Readable for AUDIOSINK_CNT {}
#[doc = "Audio Sink Clock Counter Register"]
pub mod audiosink_cnt;
#[doc = "Audio Sink Clock Phase Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audiosink_phase_cnt](audiosink_phase_cnt) module"]
pub type AUDIOSINK_PHASE_CNT = crate::Reg<u32, _AUDIOSINK_PHASE_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDIOSINK_PHASE_CNT;
#[doc = "`read()` method returns [audiosink_phase_cnt::R](audiosink_phase_cnt::R) reader structure"]
impl crate::Readable for AUDIOSINK_PHASE_CNT {}
#[doc = "`write(|w| ..)` method takes [audiosink_phase_cnt::W](audiosink_phase_cnt::W) writer structure"]
impl crate::Writable for AUDIOSINK_PHASE_CNT {}
#[doc = "Audio Sink Clock Phase Counter Register"]
pub mod audiosink_phase_cnt;
#[doc = "Audio Sink Clock Period Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audiosink_period_cnt](audiosink_period_cnt) module"]
pub type AUDIOSINK_PERIOD_CNT = crate::Reg<u32, _AUDIOSINK_PERIOD_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDIOSINK_PERIOD_CNT;
#[doc = "`read()` method returns [audiosink_period_cnt::R](audiosink_period_cnt::R) reader structure"]
impl crate::Readable for AUDIOSINK_PERIOD_CNT {}
#[doc = "`write(|w| ..)` method takes [audiosink_period_cnt::W](audiosink_period_cnt::W) writer structure"]
impl crate::Writable for AUDIOSINK_PERIOD_CNT {}
#[doc = "Audio Sink Clock Period Counter Register"]
pub mod audiosink_period_cnt;
