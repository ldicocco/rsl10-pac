#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PCM Control"]
    pub pcm_ctrl: PCM_CTRL,
    #[doc = "0x04 - PCM Transmit Data"]
    pub pcm_tx_data: PCM_TX_DATA,
    #[doc = "0x08 - PCM Receive Data"]
    pub pcm_rx_data: PCM_RX_DATA,
    #[doc = "0x0c - PCM Status"]
    pub pcm_status: PCM_STATUS,
}
#[doc = "PCM Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcm_ctrl](pcm_ctrl) module"]
pub type PCM_CTRL = crate::Reg<u32, _PCM_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCM_CTRL;
#[doc = "`read()` method returns [pcm_ctrl::R](pcm_ctrl::R) reader structure"]
impl crate::Readable for PCM_CTRL {}
#[doc = "`write(|w| ..)` method takes [pcm_ctrl::W](pcm_ctrl::W) writer structure"]
impl crate::Writable for PCM_CTRL {}
#[doc = "PCM Control"]
pub mod pcm_ctrl;
#[doc = "PCM Transmit Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcm_tx_data](pcm_tx_data) module"]
pub type PCM_TX_DATA = crate::Reg<u32, _PCM_TX_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCM_TX_DATA;
#[doc = "`read()` method returns [pcm_tx_data::R](pcm_tx_data::R) reader structure"]
impl crate::Readable for PCM_TX_DATA {}
#[doc = "`write(|w| ..)` method takes [pcm_tx_data::W](pcm_tx_data::W) writer structure"]
impl crate::Writable for PCM_TX_DATA {}
#[doc = "PCM Transmit Data"]
pub mod pcm_tx_data;
#[doc = "PCM Receive Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcm_rx_data](pcm_rx_data) module"]
pub type PCM_RX_DATA = crate::Reg<u32, _PCM_RX_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCM_RX_DATA;
#[doc = "`read()` method returns [pcm_rx_data::R](pcm_rx_data::R) reader structure"]
impl crate::Readable for PCM_RX_DATA {}
#[doc = "PCM Receive Data"]
pub mod pcm_rx_data;
#[doc = "PCM Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcm_status](pcm_status) module"]
pub type PCM_STATUS = crate::Reg<u32, _PCM_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCM_STATUS;
#[doc = "`read()` method returns [pcm_status::R](pcm_status::R) reader structure"]
impl crate::Readable for PCM_STATUS {}
#[doc = "`write(|w| ..)` method takes [pcm_status::W](pcm_status::W) writer structure"]
impl crate::Writable for PCM_STATUS {}
#[doc = "PCM Status"]
pub mod pcm_status;
