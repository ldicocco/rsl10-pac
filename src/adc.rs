#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC conversion result for channel 0 to 7 in trimmer mode"]
    pub adc_data_trim_ch: [ADC_DATA_TRIM_CH; 8],
    #[doc = "0x20 - ADC conversion result for channel 0 to 7 in audio mode (signed)"]
    pub adc_data_audio_ch: [ADC_DATA_AUDIO_CH; 8],
    #[doc = "0x40 - ADC input selection for channel 0 to 7"]
    pub adc_input_sel: [ADC_INPUT_SEL; 8],
    #[doc = "0x60 - ADC Configuration Register"]
    pub adc_cfg: ADC_CFG,
    #[doc = "0x64 - ADC conversion result for ADC GND"]
    pub adc_offset: ADC_OFFSET,
    _reserved5: [u8; 8usize],
    #[doc = "0x70 - Battery Monitoring Configuration Register"]
    pub adc_batmon_cfg: ADC_BATMON_CFG,
    #[doc = "0x74 - ADC / BATMON Interrupt Mask Register"]
    pub adc_batmon_int_enable: ADC_BATMON_INT_ENABLE,
    #[doc = "0x78 - Battery Monitoring Status Register"]
    pub adc_batmon_count_val: ADC_BATMON_COUNT_VAL,
    #[doc = "0x7c - ADC / BATMON Status Register"]
    pub adc_batmon_status: ADC_BATMON_STATUS,
}
#[doc = "ADC conversion result for channel 0 to 7 in trimmer mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_data_trim_ch](adc_data_trim_ch) module"]
pub type ADC_DATA_TRIM_CH = crate::Reg<u32, _ADC_DATA_TRIM_CH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DATA_TRIM_CH;
#[doc = "`read()` method returns [adc_data_trim_ch::R](adc_data_trim_ch::R) reader structure"]
impl crate::Readable for ADC_DATA_TRIM_CH {}
#[doc = "ADC conversion result for channel 0 to 7 in trimmer mode"]
pub mod adc_data_trim_ch;
#[doc = "ADC conversion result for channel 0 to 7 in audio mode (signed)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_data_audio_ch](adc_data_audio_ch) module"]
pub type ADC_DATA_AUDIO_CH = crate::Reg<u32, _ADC_DATA_AUDIO_CH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DATA_AUDIO_CH;
#[doc = "`read()` method returns [adc_data_audio_ch::R](adc_data_audio_ch::R) reader structure"]
impl crate::Readable for ADC_DATA_AUDIO_CH {}
#[doc = "ADC conversion result for channel 0 to 7 in audio mode (signed)"]
pub mod adc_data_audio_ch;
#[doc = "ADC input selection for channel 0 to 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_input_sel](adc_input_sel) module"]
pub type ADC_INPUT_SEL = crate::Reg<u32, _ADC_INPUT_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_INPUT_SEL;
#[doc = "`read()` method returns [adc_input_sel::R](adc_input_sel::R) reader structure"]
impl crate::Readable for ADC_INPUT_SEL {}
#[doc = "`write(|w| ..)` method takes [adc_input_sel::W](adc_input_sel::W) writer structure"]
impl crate::Writable for ADC_INPUT_SEL {}
#[doc = "ADC input selection for channel 0 to 7"]
pub mod adc_input_sel;
#[doc = "ADC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_cfg](adc_cfg) module"]
pub type ADC_CFG = crate::Reg<u32, _ADC_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CFG;
#[doc = "`read()` method returns [adc_cfg::R](adc_cfg::R) reader structure"]
impl crate::Readable for ADC_CFG {}
#[doc = "`write(|w| ..)` method takes [adc_cfg::W](adc_cfg::W) writer structure"]
impl crate::Writable for ADC_CFG {}
#[doc = "ADC Configuration Register"]
pub mod adc_cfg;
#[doc = "ADC conversion result for ADC GND\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_offset](adc_offset) module"]
pub type ADC_OFFSET = crate::Reg<u32, _ADC_OFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFFSET;
#[doc = "`read()` method returns [adc_offset::R](adc_offset::R) reader structure"]
impl crate::Readable for ADC_OFFSET {}
#[doc = "`write(|w| ..)` method takes [adc_offset::W](adc_offset::W) writer structure"]
impl crate::Writable for ADC_OFFSET {}
#[doc = "ADC conversion result for ADC GND"]
pub mod adc_offset;
#[doc = "Battery Monitoring Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_batmon_cfg](adc_batmon_cfg) module"]
pub type ADC_BATMON_CFG = crate::Reg<u32, _ADC_BATMON_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_BATMON_CFG;
#[doc = "`read()` method returns [adc_batmon_cfg::R](adc_batmon_cfg::R) reader structure"]
impl crate::Readable for ADC_BATMON_CFG {}
#[doc = "`write(|w| ..)` method takes [adc_batmon_cfg::W](adc_batmon_cfg::W) writer structure"]
impl crate::Writable for ADC_BATMON_CFG {}
#[doc = "Battery Monitoring Configuration Register"]
pub mod adc_batmon_cfg;
#[doc = "ADC / BATMON Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_batmon_int_enable](adc_batmon_int_enable) module"]
pub type ADC_BATMON_INT_ENABLE = crate::Reg<u32, _ADC_BATMON_INT_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_BATMON_INT_ENABLE;
#[doc = "`read()` method returns [adc_batmon_int_enable::R](adc_batmon_int_enable::R) reader structure"]
impl crate::Readable for ADC_BATMON_INT_ENABLE {}
#[doc = "`write(|w| ..)` method takes [adc_batmon_int_enable::W](adc_batmon_int_enable::W) writer structure"]
impl crate::Writable for ADC_BATMON_INT_ENABLE {}
#[doc = "ADC / BATMON Interrupt Mask Register"]
pub mod adc_batmon_int_enable;
#[doc = "Battery Monitoring Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_batmon_count_val](adc_batmon_count_val) module"]
pub type ADC_BATMON_COUNT_VAL = crate::Reg<u32, _ADC_BATMON_COUNT_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_BATMON_COUNT_VAL;
#[doc = "`read()` method returns [adc_batmon_count_val::R](adc_batmon_count_val::R) reader structure"]
impl crate::Readable for ADC_BATMON_COUNT_VAL {}
#[doc = "Battery Monitoring Status Register"]
pub mod adc_batmon_count_val;
#[doc = "ADC / BATMON Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_batmon_status](adc_batmon_status) module"]
pub type ADC_BATMON_STATUS = crate::Reg<u32, _ADC_BATMON_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_BATMON_STATUS;
#[doc = "`read()` method returns [adc_batmon_status::R](adc_batmon_status::R) reader structure"]
impl crate::Readable for ADC_BATMON_STATUS {}
#[doc = "`write(|w| ..)` method takes [adc_batmon_status::W](adc_batmon_status::W) writer structure"]
impl crate::Writable for ADC_BATMON_STATUS {}
#[doc = "ADC / BATMON Status Register"]
pub mod adc_batmon_status;
