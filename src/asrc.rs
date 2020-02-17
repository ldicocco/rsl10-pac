#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ASRC Control Register"]
    pub asrc_ctrl: ASRC_CTRL,
    #[doc = "0x04 - ASRC Interrupt Mask Register"]
    pub asrc_int_enable: ASRC_INT_ENABLE,
    #[doc = "0x08 - ASRC Output Data Register"]
    pub asrc_out: ASRC_OUT,
    #[doc = "0x0c - ASRC Input Data Register"]
    pub asrc_in: ASRC_IN,
    #[doc = "0x10 - ASRC Configuration Register"]
    pub asrc_cfg: ASRC_CFG,
    #[doc = "0x14 - ASRC output sample counter"]
    pub asrc_output_cnt: ASRC_OUTPUT_CNT,
    #[doc = "0x18 - ASRC phase counter increment value"]
    pub asrc_phase_inc: ASRC_PHASE_INC,
    #[doc = "0x1c - ASRC phase counter"]
    pub asrc_phase_cnt: ASRC_PHASE_CNT,
    #[doc = "0x20 - ASRC State Memory 0 to 29 (32 bit)"]
    pub asrc_state_mem: [ASRC_STATE_MEM; 30],
}
#[doc = "ASRC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asrc_ctrl](asrc_ctrl) module"]
pub type ASRC_CTRL = crate::Reg<u32, _ASRC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASRC_CTRL;
#[doc = "`read()` method returns [asrc_ctrl::R](asrc_ctrl::R) reader structure"]
impl crate::Readable for ASRC_CTRL {}
#[doc = "`write(|w| ..)` method takes [asrc_ctrl::W](asrc_ctrl::W) writer structure"]
impl crate::Writable for ASRC_CTRL {}
#[doc = "ASRC Control Register"]
pub mod asrc_ctrl;
#[doc = "ASRC Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asrc_int_enable](asrc_int_enable) module"]
pub type ASRC_INT_ENABLE = crate::Reg<u32, _ASRC_INT_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASRC_INT_ENABLE;
#[doc = "`read()` method returns [asrc_int_enable::R](asrc_int_enable::R) reader structure"]
impl crate::Readable for ASRC_INT_ENABLE {}
#[doc = "`write(|w| ..)` method takes [asrc_int_enable::W](asrc_int_enable::W) writer structure"]
impl crate::Writable for ASRC_INT_ENABLE {}
#[doc = "ASRC Interrupt Mask Register"]
pub mod asrc_int_enable;
#[doc = "ASRC Output Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asrc_out](asrc_out) module"]
pub type ASRC_OUT = crate::Reg<u32, _ASRC_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASRC_OUT;
#[doc = "`read()` method returns [asrc_out::R](asrc_out::R) reader structure"]
impl crate::Readable for ASRC_OUT {}
#[doc = "`write(|w| ..)` method takes [asrc_out::W](asrc_out::W) writer structure"]
impl crate::Writable for ASRC_OUT {}
#[doc = "ASRC Output Data Register"]
pub mod asrc_out;
#[doc = "ASRC Input Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asrc_in](asrc_in) module"]
pub type ASRC_IN = crate::Reg<u32, _ASRC_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASRC_IN;
#[doc = "`read()` method returns [asrc_in::R](asrc_in::R) reader structure"]
impl crate::Readable for ASRC_IN {}
#[doc = "`write(|w| ..)` method takes [asrc_in::W](asrc_in::W) writer structure"]
impl crate::Writable for ASRC_IN {}
#[doc = "ASRC Input Data Register"]
pub mod asrc_in;
#[doc = "ASRC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asrc_cfg](asrc_cfg) module"]
pub type ASRC_CFG = crate::Reg<u32, _ASRC_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASRC_CFG;
#[doc = "`read()` method returns [asrc_cfg::R](asrc_cfg::R) reader structure"]
impl crate::Readable for ASRC_CFG {}
#[doc = "`write(|w| ..)` method takes [asrc_cfg::W](asrc_cfg::W) writer structure"]
impl crate::Writable for ASRC_CFG {}
#[doc = "ASRC Configuration Register"]
pub mod asrc_cfg;
#[doc = "ASRC output sample counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asrc_output_cnt](asrc_output_cnt) module"]
pub type ASRC_OUTPUT_CNT = crate::Reg<u32, _ASRC_OUTPUT_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASRC_OUTPUT_CNT;
#[doc = "`read()` method returns [asrc_output_cnt::R](asrc_output_cnt::R) reader structure"]
impl crate::Readable for ASRC_OUTPUT_CNT {}
#[doc = "`write(|w| ..)` method takes [asrc_output_cnt::W](asrc_output_cnt::W) writer structure"]
impl crate::Writable for ASRC_OUTPUT_CNT {}
#[doc = "ASRC output sample counter"]
pub mod asrc_output_cnt;
#[doc = "ASRC phase counter increment value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asrc_phase_inc](asrc_phase_inc) module"]
pub type ASRC_PHASE_INC = crate::Reg<u32, _ASRC_PHASE_INC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASRC_PHASE_INC;
#[doc = "`read()` method returns [asrc_phase_inc::R](asrc_phase_inc::R) reader structure"]
impl crate::Readable for ASRC_PHASE_INC {}
#[doc = "`write(|w| ..)` method takes [asrc_phase_inc::W](asrc_phase_inc::W) writer structure"]
impl crate::Writable for ASRC_PHASE_INC {}
#[doc = "ASRC phase counter increment value"]
pub mod asrc_phase_inc;
#[doc = "ASRC phase counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asrc_phase_cnt](asrc_phase_cnt) module"]
pub type ASRC_PHASE_CNT = crate::Reg<u32, _ASRC_PHASE_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASRC_PHASE_CNT;
#[doc = "`read()` method returns [asrc_phase_cnt::R](asrc_phase_cnt::R) reader structure"]
impl crate::Readable for ASRC_PHASE_CNT {}
#[doc = "`write(|w| ..)` method takes [asrc_phase_cnt::W](asrc_phase_cnt::W) writer structure"]
impl crate::Writable for ASRC_PHASE_CNT {}
#[doc = "ASRC phase counter"]
pub mod asrc_phase_cnt;
#[doc = "ASRC State Memory 0 to 29 (32 bit)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asrc_state_mem](asrc_state_mem) module"]
pub type ASRC_STATE_MEM = crate::Reg<u32, _ASRC_STATE_MEM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASRC_STATE_MEM;
#[doc = "`read()` method returns [asrc_state_mem::R](asrc_state_mem::R) reader structure"]
impl crate::Readable for ASRC_STATE_MEM {}
#[doc = "`write(|w| ..)` method takes [asrc_state_mem::W](asrc_state_mem::W) writer structure"]
impl crate::Writable for ASRC_STATE_MEM {}
#[doc = "ASRC State Memory 0 to 29 (32 bit)"]
pub mod asrc_state_mem;
