#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bandgap Configuration / Control register"]
    pub acs_bg_ctrl: ACS_BG_CTRL,
    #[doc = "0x04 - DC-DC / LDO Supply Configuration / Control register"]
    pub acs_vcc_ctrl: ACS_VCC_CTRL,
    #[doc = "0x08 - Analog Voltage and Flash Charge Pump Configuration / Control register"]
    pub acs_vdda_cp_ctrl: ACS_VDDA_CP_CTRL,
    #[doc = "0x0c - Digital Core Voltage Regulator Configuration / Control register"]
    pub acs_vddc_ctrl: ACS_VDDC_CTRL,
    #[doc = "0x10 - Memories Voltage Regulator Configuration / Control register"]
    pub acs_vddm_ctrl: ACS_VDDM_CTRL,
    #[doc = "0x14 - RF Block Regulator Configuration / Control register"]
    pub acs_vddrf_ctrl: ACS_VDDRF_CTRL,
    #[doc = "0x18 - RF Block Regulator Configuration / Control register"]
    pub acs_vddpa_ctrl: ACS_VDDPA_CTRL,
    #[doc = "0x1c - Retention Regulator Configuration / Control register"]
    pub acs_vddret_ctrl: ACS_VDDRET_CTRL,
    #[doc = "0x20 - RC Oscillator Configuration / Control register"]
    pub acs_rcosc_ctrl: ACS_RCOSC_CTRL,
    #[doc = "0x24 - XTAL 32 kHz configuration register"]
    pub acs_xtal32k_ctrl: ACS_XTAL32K_CTRL,
    #[doc = "0x28 - Baseband timer and standby clock configuration register"]
    pub acs_bb_timer_ctrl: ACS_BB_TIMER_CTRL,
    #[doc = "0x2c - Clock Detector configuration register"]
    pub acs_clk_det_ctrl: ACS_CLK_DET_CTRL,
    #[doc = "0x30 - RTC Timer Counter Preload"]
    pub acs_rtc_cfg: ACS_RTC_CFG,
    #[doc = "0x34 - RTC Timer Counter Current Value (only reset by pmu reset or by writting 1 at ACS_RTC_CTRL.RESET)"]
    pub acs_rtc_count: ACS_RTC_COUNT,
    #[doc = "0x38 - RTC Control Register"]
    pub acs_rtc_ctrl: ACS_RTC_CTRL,
    _reserved15: [u8; 4usize],
    #[doc = "0x40 - Power Modes Control Register"]
    pub acs_pwr_modes_ctrl: ACS_PWR_MODES_CTRL,
    #[doc = "0x44 - Wake-Up Control / Status Register"]
    pub acs_wakeup_ctrl: ACS_WAKEUP_CTRL,
    #[doc = "0x48 - Wakeup configuration"]
    pub acs_wakeup_cfg: ACS_WAKEUP_CFG,
    #[doc = "0x4c - RTC Timer wakeup value and wakeup source"]
    pub acs_wakeup_state: ACS_WAKEUP_STATE,
    #[doc = "0x50 - Wake-Up Control Register / JIC RW registers"]
    pub acs_wakeup_gp_data: ACS_WAKEUP_GP_DATA,
    #[doc = "0x54 - ACS reset source status registers"]
    pub acs_reset_status: ACS_RESET_STATUS,
    #[doc = "0x58 - Analog output configuration register"]
    pub acs_aout_ctrl: ACS_AOUT_CTRL,
    #[doc = "0x5c - Just In Case register in the ACS block"]
    pub acs_jic_read: ACS_JIC_READ,
}
#[doc = "Bandgap Configuration / Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_bg_ctrl](acs_bg_ctrl) module"]
pub type ACS_BG_CTRL = crate::Reg<u32, _ACS_BG_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_BG_CTRL;
#[doc = "`read()` method returns [acs_bg_ctrl::R](acs_bg_ctrl::R) reader structure"]
impl crate::Readable for ACS_BG_CTRL {}
#[doc = "`write(|w| ..)` method takes [acs_bg_ctrl::W](acs_bg_ctrl::W) writer structure"]
impl crate::Writable for ACS_BG_CTRL {}
#[doc = "Bandgap Configuration / Control register"]
pub mod acs_bg_ctrl;
#[doc = "DC-DC / LDO Supply Configuration / Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_vcc_ctrl](acs_vcc_ctrl) module"]
pub type ACS_VCC_CTRL = crate::Reg<u32, _ACS_VCC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_VCC_CTRL;
#[doc = "`read()` method returns [acs_vcc_ctrl::R](acs_vcc_ctrl::R) reader structure"]
impl crate::Readable for ACS_VCC_CTRL {}
#[doc = "`write(|w| ..)` method takes [acs_vcc_ctrl::W](acs_vcc_ctrl::W) writer structure"]
impl crate::Writable for ACS_VCC_CTRL {}
#[doc = "DC-DC / LDO Supply Configuration / Control register"]
pub mod acs_vcc_ctrl;
#[doc = "Analog Voltage and Flash Charge Pump Configuration / Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_vdda_cp_ctrl](acs_vdda_cp_ctrl) module"]
pub type ACS_VDDA_CP_CTRL = crate::Reg<u32, _ACS_VDDA_CP_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_VDDA_CP_CTRL;
#[doc = "`read()` method returns [acs_vdda_cp_ctrl::R](acs_vdda_cp_ctrl::R) reader structure"]
impl crate::Readable for ACS_VDDA_CP_CTRL {}
#[doc = "`write(|w| ..)` method takes [acs_vdda_cp_ctrl::W](acs_vdda_cp_ctrl::W) writer structure"]
impl crate::Writable for ACS_VDDA_CP_CTRL {}
#[doc = "Analog Voltage and Flash Charge Pump Configuration / Control register"]
pub mod acs_vdda_cp_ctrl;
#[doc = "Digital Core Voltage Regulator Configuration / Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_vddc_ctrl](acs_vddc_ctrl) module"]
pub type ACS_VDDC_CTRL = crate::Reg<u32, _ACS_VDDC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_VDDC_CTRL;
#[doc = "`read()` method returns [acs_vddc_ctrl::R](acs_vddc_ctrl::R) reader structure"]
impl crate::Readable for ACS_VDDC_CTRL {}
#[doc = "`write(|w| ..)` method takes [acs_vddc_ctrl::W](acs_vddc_ctrl::W) writer structure"]
impl crate::Writable for ACS_VDDC_CTRL {}
#[doc = "Digital Core Voltage Regulator Configuration / Control register"]
pub mod acs_vddc_ctrl;
#[doc = "Memories Voltage Regulator Configuration / Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_vddm_ctrl](acs_vddm_ctrl) module"]
pub type ACS_VDDM_CTRL = crate::Reg<u32, _ACS_VDDM_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_VDDM_CTRL;
#[doc = "`read()` method returns [acs_vddm_ctrl::R](acs_vddm_ctrl::R) reader structure"]
impl crate::Readable for ACS_VDDM_CTRL {}
#[doc = "`write(|w| ..)` method takes [acs_vddm_ctrl::W](acs_vddm_ctrl::W) writer structure"]
impl crate::Writable for ACS_VDDM_CTRL {}
#[doc = "Memories Voltage Regulator Configuration / Control register"]
pub mod acs_vddm_ctrl;
#[doc = "RF Block Regulator Configuration / Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_vddrf_ctrl](acs_vddrf_ctrl) module"]
pub type ACS_VDDRF_CTRL = crate::Reg<u32, _ACS_VDDRF_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_VDDRF_CTRL;
#[doc = "`read()` method returns [acs_vddrf_ctrl::R](acs_vddrf_ctrl::R) reader structure"]
impl crate::Readable for ACS_VDDRF_CTRL {}
#[doc = "`write(|w| ..)` method takes [acs_vddrf_ctrl::W](acs_vddrf_ctrl::W) writer structure"]
impl crate::Writable for ACS_VDDRF_CTRL {}
#[doc = "RF Block Regulator Configuration / Control register"]
pub mod acs_vddrf_ctrl;
#[doc = "RF Block Regulator Configuration / Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_vddpa_ctrl](acs_vddpa_ctrl) module"]
pub type ACS_VDDPA_CTRL = crate::Reg<u32, _ACS_VDDPA_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_VDDPA_CTRL;
#[doc = "`read()` method returns [acs_vddpa_ctrl::R](acs_vddpa_ctrl::R) reader structure"]
impl crate::Readable for ACS_VDDPA_CTRL {}
#[doc = "`write(|w| ..)` method takes [acs_vddpa_ctrl::W](acs_vddpa_ctrl::W) writer structure"]
impl crate::Writable for ACS_VDDPA_CTRL {}
#[doc = "RF Block Regulator Configuration / Control register"]
pub mod acs_vddpa_ctrl;
#[doc = "Retention Regulator Configuration / Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_vddret_ctrl](acs_vddret_ctrl) module"]
pub type ACS_VDDRET_CTRL = crate::Reg<u32, _ACS_VDDRET_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_VDDRET_CTRL;
#[doc = "`read()` method returns [acs_vddret_ctrl::R](acs_vddret_ctrl::R) reader structure"]
impl crate::Readable for ACS_VDDRET_CTRL {}
#[doc = "`write(|w| ..)` method takes [acs_vddret_ctrl::W](acs_vddret_ctrl::W) writer structure"]
impl crate::Writable for ACS_VDDRET_CTRL {}
#[doc = "Retention Regulator Configuration / Control register"]
pub mod acs_vddret_ctrl;
#[doc = "RC Oscillator Configuration / Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_rcosc_ctrl](acs_rcosc_ctrl) module"]
pub type ACS_RCOSC_CTRL = crate::Reg<u32, _ACS_RCOSC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_RCOSC_CTRL;
#[doc = "`read()` method returns [acs_rcosc_ctrl::R](acs_rcosc_ctrl::R) reader structure"]
impl crate::Readable for ACS_RCOSC_CTRL {}
#[doc = "`write(|w| ..)` method takes [acs_rcosc_ctrl::W](acs_rcosc_ctrl::W) writer structure"]
impl crate::Writable for ACS_RCOSC_CTRL {}
#[doc = "RC Oscillator Configuration / Control register"]
pub mod acs_rcosc_ctrl;
#[doc = "XTAL 32 kHz configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_xtal32k_ctrl](acs_xtal32k_ctrl) module"]
pub type ACS_XTAL32K_CTRL = crate::Reg<u32, _ACS_XTAL32K_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_XTAL32K_CTRL;
#[doc = "`read()` method returns [acs_xtal32k_ctrl::R](acs_xtal32k_ctrl::R) reader structure"]
impl crate::Readable for ACS_XTAL32K_CTRL {}
#[doc = "`write(|w| ..)` method takes [acs_xtal32k_ctrl::W](acs_xtal32k_ctrl::W) writer structure"]
impl crate::Writable for ACS_XTAL32K_CTRL {}
#[doc = "XTAL 32 kHz configuration register"]
pub mod acs_xtal32k_ctrl;
#[doc = "Baseband timer and standby clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_bb_timer_ctrl](acs_bb_timer_ctrl) module"]
pub type ACS_BB_TIMER_CTRL = crate::Reg<u32, _ACS_BB_TIMER_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_BB_TIMER_CTRL;
#[doc = "`read()` method returns [acs_bb_timer_ctrl::R](acs_bb_timer_ctrl::R) reader structure"]
impl crate::Readable for ACS_BB_TIMER_CTRL {}
#[doc = "`write(|w| ..)` method takes [acs_bb_timer_ctrl::W](acs_bb_timer_ctrl::W) writer structure"]
impl crate::Writable for ACS_BB_TIMER_CTRL {}
#[doc = "Baseband timer and standby clock configuration register"]
pub mod acs_bb_timer_ctrl;
#[doc = "Clock Detector configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_clk_det_ctrl](acs_clk_det_ctrl) module"]
pub type ACS_CLK_DET_CTRL = crate::Reg<u32, _ACS_CLK_DET_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_CLK_DET_CTRL;
#[doc = "`read()` method returns [acs_clk_det_ctrl::R](acs_clk_det_ctrl::R) reader structure"]
impl crate::Readable for ACS_CLK_DET_CTRL {}
#[doc = "`write(|w| ..)` method takes [acs_clk_det_ctrl::W](acs_clk_det_ctrl::W) writer structure"]
impl crate::Writable for ACS_CLK_DET_CTRL {}
#[doc = "Clock Detector configuration register"]
pub mod acs_clk_det_ctrl;
#[doc = "RTC Timer Counter Preload\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_rtc_cfg](acs_rtc_cfg) module"]
pub type ACS_RTC_CFG = crate::Reg<u32, _ACS_RTC_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_RTC_CFG;
#[doc = "`read()` method returns [acs_rtc_cfg::R](acs_rtc_cfg::R) reader structure"]
impl crate::Readable for ACS_RTC_CFG {}
#[doc = "`write(|w| ..)` method takes [acs_rtc_cfg::W](acs_rtc_cfg::W) writer structure"]
impl crate::Writable for ACS_RTC_CFG {}
#[doc = "RTC Timer Counter Preload"]
pub mod acs_rtc_cfg;
#[doc = "RTC Timer Counter Current Value (only reset by pmu reset or by writting 1 at ACS_RTC_CTRL.RESET)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_rtc_count](acs_rtc_count) module"]
pub type ACS_RTC_COUNT = crate::Reg<u32, _ACS_RTC_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_RTC_COUNT;
#[doc = "`read()` method returns [acs_rtc_count::R](acs_rtc_count::R) reader structure"]
impl crate::Readable for ACS_RTC_COUNT {}
#[doc = "RTC Timer Counter Current Value (only reset by pmu reset or by writting 1 at ACS_RTC_CTRL.RESET)"]
pub mod acs_rtc_count;
#[doc = "RTC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_rtc_ctrl](acs_rtc_ctrl) module"]
pub type ACS_RTC_CTRL = crate::Reg<u32, _ACS_RTC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_RTC_CTRL;
#[doc = "`read()` method returns [acs_rtc_ctrl::R](acs_rtc_ctrl::R) reader structure"]
impl crate::Readable for ACS_RTC_CTRL {}
#[doc = "`write(|w| ..)` method takes [acs_rtc_ctrl::W](acs_rtc_ctrl::W) writer structure"]
impl crate::Writable for ACS_RTC_CTRL {}
#[doc = "RTC Control Register"]
pub mod acs_rtc_ctrl;
#[doc = "Power Modes Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_pwr_modes_ctrl](acs_pwr_modes_ctrl) module"]
pub type ACS_PWR_MODES_CTRL = crate::Reg<u32, _ACS_PWR_MODES_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_PWR_MODES_CTRL;
#[doc = "`read()` method returns [acs_pwr_modes_ctrl::R](acs_pwr_modes_ctrl::R) reader structure"]
impl crate::Readable for ACS_PWR_MODES_CTRL {}
#[doc = "`write(|w| ..)` method takes [acs_pwr_modes_ctrl::W](acs_pwr_modes_ctrl::W) writer structure"]
impl crate::Writable for ACS_PWR_MODES_CTRL {}
#[doc = "Power Modes Control Register"]
pub mod acs_pwr_modes_ctrl;
#[doc = "Wake-Up Control / Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_wakeup_ctrl](acs_wakeup_ctrl) module"]
pub type ACS_WAKEUP_CTRL = crate::Reg<u32, _ACS_WAKEUP_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_WAKEUP_CTRL;
#[doc = "`read()` method returns [acs_wakeup_ctrl::R](acs_wakeup_ctrl::R) reader structure"]
impl crate::Readable for ACS_WAKEUP_CTRL {}
#[doc = "`write(|w| ..)` method takes [acs_wakeup_ctrl::W](acs_wakeup_ctrl::W) writer structure"]
impl crate::Writable for ACS_WAKEUP_CTRL {}
#[doc = "Wake-Up Control / Status Register"]
pub mod acs_wakeup_ctrl;
#[doc = "Wakeup configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_wakeup_cfg](acs_wakeup_cfg) module"]
pub type ACS_WAKEUP_CFG = crate::Reg<u32, _ACS_WAKEUP_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_WAKEUP_CFG;
#[doc = "`read()` method returns [acs_wakeup_cfg::R](acs_wakeup_cfg::R) reader structure"]
impl crate::Readable for ACS_WAKEUP_CFG {}
#[doc = "`write(|w| ..)` method takes [acs_wakeup_cfg::W](acs_wakeup_cfg::W) writer structure"]
impl crate::Writable for ACS_WAKEUP_CFG {}
#[doc = "Wakeup configuration"]
pub mod acs_wakeup_cfg;
#[doc = "RTC Timer wakeup value and wakeup source\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_wakeup_state](acs_wakeup_state) module"]
pub type ACS_WAKEUP_STATE = crate::Reg<u32, _ACS_WAKEUP_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_WAKEUP_STATE;
#[doc = "`read()` method returns [acs_wakeup_state::R](acs_wakeup_state::R) reader structure"]
impl crate::Readable for ACS_WAKEUP_STATE {}
#[doc = "RTC Timer wakeup value and wakeup source"]
pub mod acs_wakeup_state;
#[doc = "Wake-Up Control Register / JIC RW registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_wakeup_gp_data](acs_wakeup_gp_data) module"]
pub type ACS_WAKEUP_GP_DATA = crate::Reg<u32, _ACS_WAKEUP_GP_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_WAKEUP_GP_DATA;
#[doc = "`read()` method returns [acs_wakeup_gp_data::R](acs_wakeup_gp_data::R) reader structure"]
impl crate::Readable for ACS_WAKEUP_GP_DATA {}
#[doc = "`write(|w| ..)` method takes [acs_wakeup_gp_data::W](acs_wakeup_gp_data::W) writer structure"]
impl crate::Writable for ACS_WAKEUP_GP_DATA {}
#[doc = "Wake-Up Control Register / JIC RW registers"]
pub mod acs_wakeup_gp_data;
#[doc = "ACS reset source status registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_reset_status](acs_reset_status) module"]
pub type ACS_RESET_STATUS = crate::Reg<u32, _ACS_RESET_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_RESET_STATUS;
#[doc = "`read()` method returns [acs_reset_status::R](acs_reset_status::R) reader structure"]
impl crate::Readable for ACS_RESET_STATUS {}
#[doc = "`write(|w| ..)` method takes [acs_reset_status::W](acs_reset_status::W) writer structure"]
impl crate::Writable for ACS_RESET_STATUS {}
#[doc = "ACS reset source status registers"]
pub mod acs_reset_status;
#[doc = "Analog output configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_aout_ctrl](acs_aout_ctrl) module"]
pub type ACS_AOUT_CTRL = crate::Reg<u32, _ACS_AOUT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_AOUT_CTRL;
#[doc = "`read()` method returns [acs_aout_ctrl::R](acs_aout_ctrl::R) reader structure"]
impl crate::Readable for ACS_AOUT_CTRL {}
#[doc = "`write(|w| ..)` method takes [acs_aout_ctrl::W](acs_aout_ctrl::W) writer structure"]
impl crate::Writable for ACS_AOUT_CTRL {}
#[doc = "Analog output configuration register"]
pub mod acs_aout_ctrl;
#[doc = "Just In Case register in the ACS block\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs_jic_read](acs_jic_read) module"]
pub type ACS_JIC_READ = crate::Reg<u32, _ACS_JIC_READ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACS_JIC_READ;
#[doc = "`read()` method returns [acs_jic_read::R](acs_jic_read::R) reader structure"]
impl crate::Readable for ACS_JIC_READ {}
#[doc = "Just In Case register in the ACS block"]
pub mod acs_jic_read;
