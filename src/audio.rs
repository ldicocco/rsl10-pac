#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMIC and OD Configuration Register"]
    pub audio_cfg: AUDIO_CFG,
    #[doc = "0x04 - DMIC and OD Status Register"]
    pub audio_status: AUDIO_STATUS,
    #[doc = "0x08 - DMIC Configuration Register"]
    pub audio_dmic_cfg: AUDIO_DMIC_CFG,
    #[doc = "0x0c - DMIC0 Gain Configuration Register"]
    pub audio_dmic0_gain: AUDIO_DMIC0_GAIN,
    #[doc = "0x10 - DMIC1 Gain Configuration Register"]
    pub audio_dmic1_gain: AUDIO_DMIC1_GAIN,
    #[doc = "0x14 - DMIC0 and DMIC1 Data Register"]
    pub audio_dmic_data: AUDIO_DMIC_DATA,
    #[doc = "0x18 - DMIC0 Data Register"]
    pub audio_dmic0_data: AUDIO_DMIC0_DATA,
    #[doc = "0x1c - DMIC1 Data Register"]
    pub audio_dmic1_data: AUDIO_DMIC1_DATA,
    #[doc = "0x20 - Output Driver Configuration Register"]
    pub audio_od_cfg: AUDIO_OD_CFG,
    #[doc = "0x24 - Output Driver Gain Configuration Register"]
    pub audio_od_gain: AUDIO_OD_GAIN,
    #[doc = "0x28 - Output Driver Data Register"]
    pub audio_od_data: AUDIO_OD_DATA,
    _reserved11: [u8; 4usize],
    #[doc = "0x30 - Output Driver Sigma-delta Modulator Configuration Register"]
    pub audio_sdm_cfg: AUDIO_SDM_CFG,
}
#[doc = "DMIC and OD Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_cfg](audio_cfg) module"]
pub type AUDIO_CFG = crate::Reg<u32, _AUDIO_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDIO_CFG;
#[doc = "`read()` method returns [audio_cfg::R](audio_cfg::R) reader structure"]
impl crate::Readable for AUDIO_CFG {}
#[doc = "`write(|w| ..)` method takes [audio_cfg::W](audio_cfg::W) writer structure"]
impl crate::Writable for AUDIO_CFG {}
#[doc = "DMIC and OD Configuration Register"]
pub mod audio_cfg;
#[doc = "DMIC and OD Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_status](audio_status) module"]
pub type AUDIO_STATUS = crate::Reg<u32, _AUDIO_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDIO_STATUS;
#[doc = "`read()` method returns [audio_status::R](audio_status::R) reader structure"]
impl crate::Readable for AUDIO_STATUS {}
#[doc = "`write(|w| ..)` method takes [audio_status::W](audio_status::W) writer structure"]
impl crate::Writable for AUDIO_STATUS {}
#[doc = "DMIC and OD Status Register"]
pub mod audio_status;
#[doc = "DMIC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_dmic_cfg](audio_dmic_cfg) module"]
pub type AUDIO_DMIC_CFG = crate::Reg<u32, _AUDIO_DMIC_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDIO_DMIC_CFG;
#[doc = "`read()` method returns [audio_dmic_cfg::R](audio_dmic_cfg::R) reader structure"]
impl crate::Readable for AUDIO_DMIC_CFG {}
#[doc = "`write(|w| ..)` method takes [audio_dmic_cfg::W](audio_dmic_cfg::W) writer structure"]
impl crate::Writable for AUDIO_DMIC_CFG {}
#[doc = "DMIC Configuration Register"]
pub mod audio_dmic_cfg;
#[doc = "DMIC0 Gain Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_dmic0_gain](audio_dmic0_gain) module"]
pub type AUDIO_DMIC0_GAIN = crate::Reg<u32, _AUDIO_DMIC0_GAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDIO_DMIC0_GAIN;
#[doc = "`read()` method returns [audio_dmic0_gain::R](audio_dmic0_gain::R) reader structure"]
impl crate::Readable for AUDIO_DMIC0_GAIN {}
#[doc = "`write(|w| ..)` method takes [audio_dmic0_gain::W](audio_dmic0_gain::W) writer structure"]
impl crate::Writable for AUDIO_DMIC0_GAIN {}
#[doc = "DMIC0 Gain Configuration Register"]
pub mod audio_dmic0_gain;
#[doc = "DMIC1 Gain Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_dmic1_gain](audio_dmic1_gain) module"]
pub type AUDIO_DMIC1_GAIN = crate::Reg<u32, _AUDIO_DMIC1_GAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDIO_DMIC1_GAIN;
#[doc = "`read()` method returns [audio_dmic1_gain::R](audio_dmic1_gain::R) reader structure"]
impl crate::Readable for AUDIO_DMIC1_GAIN {}
#[doc = "`write(|w| ..)` method takes [audio_dmic1_gain::W](audio_dmic1_gain::W) writer structure"]
impl crate::Writable for AUDIO_DMIC1_GAIN {}
#[doc = "DMIC1 Gain Configuration Register"]
pub mod audio_dmic1_gain;
#[doc = "DMIC0 and DMIC1 Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_dmic_data](audio_dmic_data) module"]
pub type AUDIO_DMIC_DATA = crate::Reg<u32, _AUDIO_DMIC_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDIO_DMIC_DATA;
#[doc = "`read()` method returns [audio_dmic_data::R](audio_dmic_data::R) reader structure"]
impl crate::Readable for AUDIO_DMIC_DATA {}
#[doc = "DMIC0 and DMIC1 Data Register"]
pub mod audio_dmic_data;
#[doc = "DMIC0 Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_dmic0_data](audio_dmic0_data) module"]
pub type AUDIO_DMIC0_DATA = crate::Reg<u32, _AUDIO_DMIC0_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDIO_DMIC0_DATA;
#[doc = "`read()` method returns [audio_dmic0_data::R](audio_dmic0_data::R) reader structure"]
impl crate::Readable for AUDIO_DMIC0_DATA {}
#[doc = "DMIC0 Data Register"]
pub mod audio_dmic0_data;
#[doc = "DMIC1 Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_dmic1_data](audio_dmic1_data) module"]
pub type AUDIO_DMIC1_DATA = crate::Reg<u32, _AUDIO_DMIC1_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDIO_DMIC1_DATA;
#[doc = "`read()` method returns [audio_dmic1_data::R](audio_dmic1_data::R) reader structure"]
impl crate::Readable for AUDIO_DMIC1_DATA {}
#[doc = "DMIC1 Data Register"]
pub mod audio_dmic1_data;
#[doc = "Output Driver Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_od_cfg](audio_od_cfg) module"]
pub type AUDIO_OD_CFG = crate::Reg<u32, _AUDIO_OD_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDIO_OD_CFG;
#[doc = "`read()` method returns [audio_od_cfg::R](audio_od_cfg::R) reader structure"]
impl crate::Readable for AUDIO_OD_CFG {}
#[doc = "`write(|w| ..)` method takes [audio_od_cfg::W](audio_od_cfg::W) writer structure"]
impl crate::Writable for AUDIO_OD_CFG {}
#[doc = "Output Driver Configuration Register"]
pub mod audio_od_cfg;
#[doc = "Output Driver Gain Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_od_gain](audio_od_gain) module"]
pub type AUDIO_OD_GAIN = crate::Reg<u32, _AUDIO_OD_GAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDIO_OD_GAIN;
#[doc = "`read()` method returns [audio_od_gain::R](audio_od_gain::R) reader structure"]
impl crate::Readable for AUDIO_OD_GAIN {}
#[doc = "`write(|w| ..)` method takes [audio_od_gain::W](audio_od_gain::W) writer structure"]
impl crate::Writable for AUDIO_OD_GAIN {}
#[doc = "Output Driver Gain Configuration Register"]
pub mod audio_od_gain;
#[doc = "Output Driver Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_od_data](audio_od_data) module"]
pub type AUDIO_OD_DATA = crate::Reg<u32, _AUDIO_OD_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDIO_OD_DATA;
#[doc = "`read()` method returns [audio_od_data::R](audio_od_data::R) reader structure"]
impl crate::Readable for AUDIO_OD_DATA {}
#[doc = "`write(|w| ..)` method takes [audio_od_data::W](audio_od_data::W) writer structure"]
impl crate::Writable for AUDIO_OD_DATA {}
#[doc = "Output Driver Data Register"]
pub mod audio_od_data;
#[doc = "Output Driver Sigma-delta Modulator Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_sdm_cfg](audio_sdm_cfg) module"]
pub type AUDIO_SDM_CFG = crate::Reg<u32, _AUDIO_SDM_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDIO_SDM_CFG;
#[doc = "`read()` method returns [audio_sdm_cfg::R](audio_sdm_cfg::R) reader structure"]
impl crate::Readable for AUDIO_SDM_CFG {}
#[doc = "`write(|w| ..)` method takes [audio_sdm_cfg::W](audio_sdm_cfg::W) writer structure"]
impl crate::Writable for AUDIO_SDM_CFG {}
#[doc = "Output Driver Sigma-delta Modulator Configuration Register"]
pub mod audio_sdm_cfg;
