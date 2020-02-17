#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DSS Control"]
    pub sysctrl_dss_ctrl: SYSCTRL_DSS_CTRL,
    #[doc = "0x04 - DSS Commands"]
    pub sysctrl_dss_cmd: SYSCTRL_DSS_CMD,
    #[doc = "0x08 - Flash Overlay Configuration"]
    pub sysctrl_flash_overlay_cfg: SYSCTRL_FLASH_OVERLAY_CFG,
    #[doc = "0x0c - CSS Loop Cache Configuration"]
    pub sysctrl_css_loop_cache_cfg: SYSCTRL_CSS_LOOP_CACHE_CFG,
    #[doc = "0x10 - DSS Loop Cache Configuration"]
    pub sysctrl_dss_loop_cache_cfg: SYSCTRL_DSS_LOOP_CACHE_CFG,
    #[doc = "0x14 - Memory Access Error Flags"]
    pub sysctrl_mem_error: SYSCTRL_MEM_ERROR,
    #[doc = "0x18 - Memory Power Configuration"]
    pub sysctrl_mem_power_cfg: SYSCTRL_MEM_POWER_CFG,
    #[doc = "0x1c - Memory Access Configuration and Wakeup Restore Address in packed 7-bit format"]
    pub sysctrl_mem_access_cfg: SYSCTRL_MEM_ACCESS_CFG,
    #[doc = "0x20 - Wakeup Restore Address in Unpacked 32-bit Format"]
    pub sysctrl_wakeup_addr: SYSCTRL_WAKEUP_ADDR,
    #[doc = "0x24 - Memory Retention Configuration"]
    pub sysctrl_mem_retention_cfg: SYSCTRL_MEM_RETENTION_CFG,
    #[doc = "0x28 - Memory Arbiter Configuration"]
    pub sysctrl_mem_arbiter_cfg: SYSCTRL_MEM_ARBITER_CFG,
    #[doc = "0x2c - Memory Timing Configuration"]
    pub sysctrl_mem_timing_cfg: SYSCTRL_MEM_TIMING_CFG,
    #[doc = "0x30 - Activity Counters Control"]
    pub sysctrl_cnt_ctrl: SYSCTRL_CNT_CTRL,
    #[doc = "0x34 - System Clock Counter Value"]
    pub sysctrl_sysclk_cnt: SYSCTRL_SYSCLK_CNT,
    #[doc = "0x38 - CM3 Activity Counter Value"]
    pub sysctrl_cm3_cnt: SYSCTRL_CM3_CNT,
    #[doc = "0x3c - LPDSP32 Activity Counter Value"]
    pub sysctrl_lpdsp32_cnt: SYSCTRL_LPDSP32_CNT,
    #[doc = "0x40 - Flash Read Access Counter Value"]
    pub sysctrl_flash_read_cnt: SYSCTRL_FLASH_READ_CNT,
    _reserved17: [u8; 4usize],
    #[doc = "0x48 - Critical Path Speed Measurement"]
    pub sysctrl_speed_measure: SYSCTRL_SPEED_MEASURE,
    #[doc = "0x4c - LPDSP32 Debug Port Configuration"]
    pub sysctrl_lpdsp32_debug_cfg: SYSCTRL_LPDSP32_DEBUG_CFG,
    #[doc = "0x50 - RF Power Configuration"]
    pub sysctrl_rf_power_cfg: SYSCTRL_RF_POWER_CFG,
    #[doc = "0x54 - RF Access Configuration"]
    pub sysctrl_rf_access_cfg: SYSCTRL_RF_ACCESS_CFG,
    #[doc = "0x58 - WAKEUP Pad Value"]
    pub sysctrl_wakeup_pad: SYSCTRL_WAKEUP_PAD,
    _reserved22: [u8; 128usize],
    #[doc = "0xdc - Debug Port Access Configuration"]
    pub sysctrl_dbg_lock: SYSCTRL_DBG_LOCK,
    #[doc = "0xe0 - Debug Port Lock Key Part 0 to 3"]
    pub sysctrl_dbg_lock_key: [SYSCTRL_DBG_LOCK_KEY; 4],
}
#[doc = "DSS Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_dss_ctrl](sysctrl_dss_ctrl) module"]
pub type SYSCTRL_DSS_CTRL = crate::Reg<u32, _SYSCTRL_DSS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_DSS_CTRL;
#[doc = "`read()` method returns [sysctrl_dss_ctrl::R](sysctrl_dss_ctrl::R) reader structure"]
impl crate::Readable for SYSCTRL_DSS_CTRL {}
#[doc = "`write(|w| ..)` method takes [sysctrl_dss_ctrl::W](sysctrl_dss_ctrl::W) writer structure"]
impl crate::Writable for SYSCTRL_DSS_CTRL {}
#[doc = "DSS Control"]
pub mod sysctrl_dss_ctrl;
#[doc = "DSS Commands\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_dss_cmd](sysctrl_dss_cmd) module"]
pub type SYSCTRL_DSS_CMD = crate::Reg<u32, _SYSCTRL_DSS_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_DSS_CMD;
#[doc = "`read()` method returns [sysctrl_dss_cmd::R](sysctrl_dss_cmd::R) reader structure"]
impl crate::Readable for SYSCTRL_DSS_CMD {}
#[doc = "`write(|w| ..)` method takes [sysctrl_dss_cmd::W](sysctrl_dss_cmd::W) writer structure"]
impl crate::Writable for SYSCTRL_DSS_CMD {}
#[doc = "DSS Commands"]
pub mod sysctrl_dss_cmd;
#[doc = "Flash Overlay Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_flash_overlay_cfg](sysctrl_flash_overlay_cfg) module"]
pub type SYSCTRL_FLASH_OVERLAY_CFG = crate::Reg<u32, _SYSCTRL_FLASH_OVERLAY_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_FLASH_OVERLAY_CFG;
#[doc = "`read()` method returns [sysctrl_flash_overlay_cfg::R](sysctrl_flash_overlay_cfg::R) reader structure"]
impl crate::Readable for SYSCTRL_FLASH_OVERLAY_CFG {}
#[doc = "`write(|w| ..)` method takes [sysctrl_flash_overlay_cfg::W](sysctrl_flash_overlay_cfg::W) writer structure"]
impl crate::Writable for SYSCTRL_FLASH_OVERLAY_CFG {}
#[doc = "Flash Overlay Configuration"]
pub mod sysctrl_flash_overlay_cfg;
#[doc = "CSS Loop Cache Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_css_loop_cache_cfg](sysctrl_css_loop_cache_cfg) module"]
pub type SYSCTRL_CSS_LOOP_CACHE_CFG = crate::Reg<u32, _SYSCTRL_CSS_LOOP_CACHE_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_CSS_LOOP_CACHE_CFG;
#[doc = "`read()` method returns [sysctrl_css_loop_cache_cfg::R](sysctrl_css_loop_cache_cfg::R) reader structure"]
impl crate::Readable for SYSCTRL_CSS_LOOP_CACHE_CFG {}
#[doc = "`write(|w| ..)` method takes [sysctrl_css_loop_cache_cfg::W](sysctrl_css_loop_cache_cfg::W) writer structure"]
impl crate::Writable for SYSCTRL_CSS_LOOP_CACHE_CFG {}
#[doc = "CSS Loop Cache Configuration"]
pub mod sysctrl_css_loop_cache_cfg;
#[doc = "DSS Loop Cache Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_dss_loop_cache_cfg](sysctrl_dss_loop_cache_cfg) module"]
pub type SYSCTRL_DSS_LOOP_CACHE_CFG = crate::Reg<u32, _SYSCTRL_DSS_LOOP_CACHE_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_DSS_LOOP_CACHE_CFG;
#[doc = "`read()` method returns [sysctrl_dss_loop_cache_cfg::R](sysctrl_dss_loop_cache_cfg::R) reader structure"]
impl crate::Readable for SYSCTRL_DSS_LOOP_CACHE_CFG {}
#[doc = "`write(|w| ..)` method takes [sysctrl_dss_loop_cache_cfg::W](sysctrl_dss_loop_cache_cfg::W) writer structure"]
impl crate::Writable for SYSCTRL_DSS_LOOP_CACHE_CFG {}
#[doc = "DSS Loop Cache Configuration"]
pub mod sysctrl_dss_loop_cache_cfg;
#[doc = "Memory Access Error Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_mem_error](sysctrl_mem_error) module"]
pub type SYSCTRL_MEM_ERROR = crate::Reg<u32, _SYSCTRL_MEM_ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_MEM_ERROR;
#[doc = "`read()` method returns [sysctrl_mem_error::R](sysctrl_mem_error::R) reader structure"]
impl crate::Readable for SYSCTRL_MEM_ERROR {}
#[doc = "`write(|w| ..)` method takes [sysctrl_mem_error::W](sysctrl_mem_error::W) writer structure"]
impl crate::Writable for SYSCTRL_MEM_ERROR {}
#[doc = "Memory Access Error Flags"]
pub mod sysctrl_mem_error;
#[doc = "Memory Power Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_mem_power_cfg](sysctrl_mem_power_cfg) module"]
pub type SYSCTRL_MEM_POWER_CFG = crate::Reg<u32, _SYSCTRL_MEM_POWER_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_MEM_POWER_CFG;
#[doc = "`read()` method returns [sysctrl_mem_power_cfg::R](sysctrl_mem_power_cfg::R) reader structure"]
impl crate::Readable for SYSCTRL_MEM_POWER_CFG {}
#[doc = "`write(|w| ..)` method takes [sysctrl_mem_power_cfg::W](sysctrl_mem_power_cfg::W) writer structure"]
impl crate::Writable for SYSCTRL_MEM_POWER_CFG {}
#[doc = "Memory Power Configuration"]
pub mod sysctrl_mem_power_cfg;
#[doc = "Memory Access Configuration and Wakeup Restore Address in packed 7-bit format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_mem_access_cfg](sysctrl_mem_access_cfg) module"]
pub type SYSCTRL_MEM_ACCESS_CFG = crate::Reg<u32, _SYSCTRL_MEM_ACCESS_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_MEM_ACCESS_CFG;
#[doc = "`read()` method returns [sysctrl_mem_access_cfg::R](sysctrl_mem_access_cfg::R) reader structure"]
impl crate::Readable for SYSCTRL_MEM_ACCESS_CFG {}
#[doc = "`write(|w| ..)` method takes [sysctrl_mem_access_cfg::W](sysctrl_mem_access_cfg::W) writer structure"]
impl crate::Writable for SYSCTRL_MEM_ACCESS_CFG {}
#[doc = "Memory Access Configuration and Wakeup Restore Address in packed 7-bit format"]
pub mod sysctrl_mem_access_cfg;
#[doc = "Wakeup Restore Address in Unpacked 32-bit Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_wakeup_addr](sysctrl_wakeup_addr) module"]
pub type SYSCTRL_WAKEUP_ADDR = crate::Reg<u32, _SYSCTRL_WAKEUP_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_WAKEUP_ADDR;
#[doc = "`read()` method returns [sysctrl_wakeup_addr::R](sysctrl_wakeup_addr::R) reader structure"]
impl crate::Readable for SYSCTRL_WAKEUP_ADDR {}
#[doc = "`write(|w| ..)` method takes [sysctrl_wakeup_addr::W](sysctrl_wakeup_addr::W) writer structure"]
impl crate::Writable for SYSCTRL_WAKEUP_ADDR {}
#[doc = "Wakeup Restore Address in Unpacked 32-bit Format"]
pub mod sysctrl_wakeup_addr;
#[doc = "Memory Retention Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_mem_retention_cfg](sysctrl_mem_retention_cfg) module"]
pub type SYSCTRL_MEM_RETENTION_CFG = crate::Reg<u32, _SYSCTRL_MEM_RETENTION_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_MEM_RETENTION_CFG;
#[doc = "`read()` method returns [sysctrl_mem_retention_cfg::R](sysctrl_mem_retention_cfg::R) reader structure"]
impl crate::Readable for SYSCTRL_MEM_RETENTION_CFG {}
#[doc = "`write(|w| ..)` method takes [sysctrl_mem_retention_cfg::W](sysctrl_mem_retention_cfg::W) writer structure"]
impl crate::Writable for SYSCTRL_MEM_RETENTION_CFG {}
#[doc = "Memory Retention Configuration"]
pub mod sysctrl_mem_retention_cfg;
#[doc = "Memory Arbiter Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_mem_arbiter_cfg](sysctrl_mem_arbiter_cfg) module"]
pub type SYSCTRL_MEM_ARBITER_CFG = crate::Reg<u32, _SYSCTRL_MEM_ARBITER_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_MEM_ARBITER_CFG;
#[doc = "`read()` method returns [sysctrl_mem_arbiter_cfg::R](sysctrl_mem_arbiter_cfg::R) reader structure"]
impl crate::Readable for SYSCTRL_MEM_ARBITER_CFG {}
#[doc = "`write(|w| ..)` method takes [sysctrl_mem_arbiter_cfg::W](sysctrl_mem_arbiter_cfg::W) writer structure"]
impl crate::Writable for SYSCTRL_MEM_ARBITER_CFG {}
#[doc = "Memory Arbiter Configuration"]
pub mod sysctrl_mem_arbiter_cfg;
#[doc = "Memory Timing Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_mem_timing_cfg](sysctrl_mem_timing_cfg) module"]
pub type SYSCTRL_MEM_TIMING_CFG = crate::Reg<u32, _SYSCTRL_MEM_TIMING_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_MEM_TIMING_CFG;
#[doc = "`read()` method returns [sysctrl_mem_timing_cfg::R](sysctrl_mem_timing_cfg::R) reader structure"]
impl crate::Readable for SYSCTRL_MEM_TIMING_CFG {}
#[doc = "`write(|w| ..)` method takes [sysctrl_mem_timing_cfg::W](sysctrl_mem_timing_cfg::W) writer structure"]
impl crate::Writable for SYSCTRL_MEM_TIMING_CFG {}
#[doc = "Memory Timing Configuration"]
pub mod sysctrl_mem_timing_cfg;
#[doc = "Activity Counters Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_cnt_ctrl](sysctrl_cnt_ctrl) module"]
pub type SYSCTRL_CNT_CTRL = crate::Reg<u32, _SYSCTRL_CNT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_CNT_CTRL;
#[doc = "`read()` method returns [sysctrl_cnt_ctrl::R](sysctrl_cnt_ctrl::R) reader structure"]
impl crate::Readable for SYSCTRL_CNT_CTRL {}
#[doc = "`write(|w| ..)` method takes [sysctrl_cnt_ctrl::W](sysctrl_cnt_ctrl::W) writer structure"]
impl crate::Writable for SYSCTRL_CNT_CTRL {}
#[doc = "Activity Counters Control"]
pub mod sysctrl_cnt_ctrl;
#[doc = "System Clock Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_sysclk_cnt](sysctrl_sysclk_cnt) module"]
pub type SYSCTRL_SYSCLK_CNT = crate::Reg<u32, _SYSCTRL_SYSCLK_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_SYSCLK_CNT;
#[doc = "`read()` method returns [sysctrl_sysclk_cnt::R](sysctrl_sysclk_cnt::R) reader structure"]
impl crate::Readable for SYSCTRL_SYSCLK_CNT {}
#[doc = "`write(|w| ..)` method takes [sysctrl_sysclk_cnt::W](sysctrl_sysclk_cnt::W) writer structure"]
impl crate::Writable for SYSCTRL_SYSCLK_CNT {}
#[doc = "System Clock Counter Value"]
pub mod sysctrl_sysclk_cnt;
#[doc = "CM3 Activity Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_cm3_cnt](sysctrl_cm3_cnt) module"]
pub type SYSCTRL_CM3_CNT = crate::Reg<u32, _SYSCTRL_CM3_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_CM3_CNT;
#[doc = "`read()` method returns [sysctrl_cm3_cnt::R](sysctrl_cm3_cnt::R) reader structure"]
impl crate::Readable for SYSCTRL_CM3_CNT {}
#[doc = "`write(|w| ..)` method takes [sysctrl_cm3_cnt::W](sysctrl_cm3_cnt::W) writer structure"]
impl crate::Writable for SYSCTRL_CM3_CNT {}
#[doc = "CM3 Activity Counter Value"]
pub mod sysctrl_cm3_cnt;
#[doc = "LPDSP32 Activity Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_lpdsp32_cnt](sysctrl_lpdsp32_cnt) module"]
pub type SYSCTRL_LPDSP32_CNT = crate::Reg<u32, _SYSCTRL_LPDSP32_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_LPDSP32_CNT;
#[doc = "`read()` method returns [sysctrl_lpdsp32_cnt::R](sysctrl_lpdsp32_cnt::R) reader structure"]
impl crate::Readable for SYSCTRL_LPDSP32_CNT {}
#[doc = "`write(|w| ..)` method takes [sysctrl_lpdsp32_cnt::W](sysctrl_lpdsp32_cnt::W) writer structure"]
impl crate::Writable for SYSCTRL_LPDSP32_CNT {}
#[doc = "LPDSP32 Activity Counter Value"]
pub mod sysctrl_lpdsp32_cnt;
#[doc = "Flash Read Access Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_flash_read_cnt](sysctrl_flash_read_cnt) module"]
pub type SYSCTRL_FLASH_READ_CNT = crate::Reg<u32, _SYSCTRL_FLASH_READ_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_FLASH_READ_CNT;
#[doc = "`read()` method returns [sysctrl_flash_read_cnt::R](sysctrl_flash_read_cnt::R) reader structure"]
impl crate::Readable for SYSCTRL_FLASH_READ_CNT {}
#[doc = "`write(|w| ..)` method takes [sysctrl_flash_read_cnt::W](sysctrl_flash_read_cnt::W) writer structure"]
impl crate::Writable for SYSCTRL_FLASH_READ_CNT {}
#[doc = "Flash Read Access Counter Value"]
pub mod sysctrl_flash_read_cnt;
#[doc = "Critical Path Speed Measurement\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_speed_measure](sysctrl_speed_measure) module"]
pub type SYSCTRL_SPEED_MEASURE = crate::Reg<u32, _SYSCTRL_SPEED_MEASURE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_SPEED_MEASURE;
#[doc = "`read()` method returns [sysctrl_speed_measure::R](sysctrl_speed_measure::R) reader structure"]
impl crate::Readable for SYSCTRL_SPEED_MEASURE {}
#[doc = "`write(|w| ..)` method takes [sysctrl_speed_measure::W](sysctrl_speed_measure::W) writer structure"]
impl crate::Writable for SYSCTRL_SPEED_MEASURE {}
#[doc = "Critical Path Speed Measurement"]
pub mod sysctrl_speed_measure;
#[doc = "LPDSP32 Debug Port Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_lpdsp32_debug_cfg](sysctrl_lpdsp32_debug_cfg) module"]
pub type SYSCTRL_LPDSP32_DEBUG_CFG = crate::Reg<u32, _SYSCTRL_LPDSP32_DEBUG_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_LPDSP32_DEBUG_CFG;
#[doc = "`read()` method returns [sysctrl_lpdsp32_debug_cfg::R](sysctrl_lpdsp32_debug_cfg::R) reader structure"]
impl crate::Readable for SYSCTRL_LPDSP32_DEBUG_CFG {}
#[doc = "`write(|w| ..)` method takes [sysctrl_lpdsp32_debug_cfg::W](sysctrl_lpdsp32_debug_cfg::W) writer structure"]
impl crate::Writable for SYSCTRL_LPDSP32_DEBUG_CFG {}
#[doc = "LPDSP32 Debug Port Configuration"]
pub mod sysctrl_lpdsp32_debug_cfg;
#[doc = "RF Power Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_rf_power_cfg](sysctrl_rf_power_cfg) module"]
pub type SYSCTRL_RF_POWER_CFG = crate::Reg<u32, _SYSCTRL_RF_POWER_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_RF_POWER_CFG;
#[doc = "`read()` method returns [sysctrl_rf_power_cfg::R](sysctrl_rf_power_cfg::R) reader structure"]
impl crate::Readable for SYSCTRL_RF_POWER_CFG {}
#[doc = "`write(|w| ..)` method takes [sysctrl_rf_power_cfg::W](sysctrl_rf_power_cfg::W) writer structure"]
impl crate::Writable for SYSCTRL_RF_POWER_CFG {}
#[doc = "RF Power Configuration"]
pub mod sysctrl_rf_power_cfg;
#[doc = "RF Access Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_rf_access_cfg](sysctrl_rf_access_cfg) module"]
pub type SYSCTRL_RF_ACCESS_CFG = crate::Reg<u32, _SYSCTRL_RF_ACCESS_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_RF_ACCESS_CFG;
#[doc = "`read()` method returns [sysctrl_rf_access_cfg::R](sysctrl_rf_access_cfg::R) reader structure"]
impl crate::Readable for SYSCTRL_RF_ACCESS_CFG {}
#[doc = "`write(|w| ..)` method takes [sysctrl_rf_access_cfg::W](sysctrl_rf_access_cfg::W) writer structure"]
impl crate::Writable for SYSCTRL_RF_ACCESS_CFG {}
#[doc = "RF Access Configuration"]
pub mod sysctrl_rf_access_cfg;
#[doc = "WAKEUP Pad Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_wakeup_pad](sysctrl_wakeup_pad) module"]
pub type SYSCTRL_WAKEUP_PAD = crate::Reg<u32, _SYSCTRL_WAKEUP_PAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_WAKEUP_PAD;
#[doc = "`read()` method returns [sysctrl_wakeup_pad::R](sysctrl_wakeup_pad::R) reader structure"]
impl crate::Readable for SYSCTRL_WAKEUP_PAD {}
#[doc = "WAKEUP Pad Value"]
pub mod sysctrl_wakeup_pad;
#[doc = "Debug Port Access Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_dbg_lock](sysctrl_dbg_lock) module"]
pub type SYSCTRL_DBG_LOCK = crate::Reg<u32, _SYSCTRL_DBG_LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_DBG_LOCK;
#[doc = "`read()` method returns [sysctrl_dbg_lock::R](sysctrl_dbg_lock::R) reader structure"]
impl crate::Readable for SYSCTRL_DBG_LOCK {}
#[doc = "`write(|w| ..)` method takes [sysctrl_dbg_lock::W](sysctrl_dbg_lock::W) writer structure"]
impl crate::Writable for SYSCTRL_DBG_LOCK {}
#[doc = "Debug Port Access Configuration"]
pub mod sysctrl_dbg_lock;
#[doc = "Debug Port Lock Key Part 0 to 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_dbg_lock_key](sysctrl_dbg_lock_key) module"]
pub type SYSCTRL_DBG_LOCK_KEY = crate::Reg<u32, _SYSCTRL_DBG_LOCK_KEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTRL_DBG_LOCK_KEY;
#[doc = "`read()` method returns [sysctrl_dbg_lock_key::R](sysctrl_dbg_lock_key::R) reader structure"]
impl crate::Readable for SYSCTRL_DBG_LOCK_KEY {}
#[doc = "`write(|w| ..)` method takes [sysctrl_dbg_lock_key::W](sysctrl_dbg_lock_key::W) writer structure"]
impl crate::Writable for SYSCTRL_DBG_LOCK_KEY {}
#[doc = "Debug Port Lock Key Part 0 to 3"]
pub mod sysctrl_dbg_lock_key;
