#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - REG00"]
    pub rf_reg00: RF_REG00,
    #[doc = "0x04 - REG01"]
    pub rf_reg01: RF_REG01,
    #[doc = "0x08 - REG02"]
    pub rf_reg02: RF_REG02,
    #[doc = "0x0c - REG03"]
    pub rf_reg03: RF_REG03,
    #[doc = "0x10 - REG04"]
    pub rf_reg04: RF_REG04,
    #[doc = "0x14 - REG05"]
    pub rf_reg05: RF_REG05,
    #[doc = "0x18 - CENTER_FREQ"]
    pub rf_center_freq: RF_CENTER_FREQ,
    #[doc = "0x1c - REG07"]
    pub rf_reg07: RF_REG07,
    #[doc = "0x20 - REG08"]
    pub rf_reg08: RF_REG08,
    #[doc = "0x24 - REG09"]
    pub rf_reg09: RF_REG09,
    #[doc = "0x28 - REG0A"]
    pub rf_reg0a: RF_REG0A,
    #[doc = "0x2c - SYNC_PATTERN"]
    pub rf_sync_pattern: RF_SYNC_PATTERN,
    #[doc = "0x30 - REG0C"]
    pub rf_reg0c: RF_REG0C,
    #[doc = "0x34 - CRC_POLYNOMIAL"]
    pub rf_crc_polynomial: RF_CRC_POLYNOMIAL,
    #[doc = "0x38 - CRC_RST"]
    pub rf_crc_rst: RF_CRC_RST,
    #[doc = "0x3c - REG0F"]
    pub rf_reg0f: RF_REG0F,
    #[doc = "0x40 - REG10"]
    pub rf_reg10: RF_REG10,
    #[doc = "0x44 - TX_PULSE0"]
    pub rf_tx_pulse0: RF_TX_PULSE0,
    #[doc = "0x48 - TX_PULSE1"]
    pub rf_tx_pulse1: RF_TX_PULSE1,
    #[doc = "0x4c - TX_PULSE2"]
    pub rf_tx_pulse2: RF_TX_PULSE2,
    #[doc = "0x50 - TX_PULSE3"]
    pub rf_tx_pulse3: RF_TX_PULSE3,
    #[doc = "0x54 - RX_PULSE"]
    pub rf_rx_pulse: RF_RX_PULSE,
    #[doc = "0x58 - REG16"]
    pub rf_reg16: RF_REG16,
    #[doc = "0x5c - REG17"]
    pub rf_reg17: RF_REG17,
    #[doc = "0x60 - REG18"]
    pub rf_reg18: RF_REG18,
    #[doc = "0x64 - REG19"]
    pub rf_reg19: RF_REG19,
    #[doc = "0x68 - REG1A"]
    pub rf_reg1a: RF_REG1A,
    #[doc = "0x6c - REG1B"]
    pub rf_reg1b: RF_REG1B,
    #[doc = "0x70 - AGC_LUT1"]
    pub rf_agc_lut1: RF_AGC_LUT1,
    #[doc = "0x74 - AGC_LUT2"]
    pub rf_agc_lut2: RF_AGC_LUT2,
    #[doc = "0x78 - AGC_LUT3"]
    pub rf_agc_lut3: RF_AGC_LUT3,
    #[doc = "0x7c - AGC_LUT4"]
    pub rf_agc_lut4: RF_AGC_LUT4,
    #[doc = "0x80 - REG20"]
    pub rf_reg20: RF_REG20,
    #[doc = "0x84 - AGC_ATT1"]
    pub rf_agc_att1: RF_AGC_ATT1,
    #[doc = "0x88 - REG22"]
    pub rf_reg22: RF_REG22,
    #[doc = "0x8c - REG23"]
    pub rf_reg23: RF_REG23,
    #[doc = "0x90 - REG24"]
    pub rf_reg24: RF_REG24,
    #[doc = "0x94 - REG25"]
    pub rf_reg25: RF_REG25,
    #[doc = "0x98 - REG26"]
    pub rf_reg26: RF_REG26,
    #[doc = "0x9c - REG27"]
    pub rf_reg27: RF_REG27,
    #[doc = "0xa0 - REG28"]
    pub rf_reg28: RF_REG28,
    #[doc = "0xa4 - PLL_CTRL"]
    pub rf_pll_ctrl: RF_PLL_CTRL,
    #[doc = "0xa8 - REG2A"]
    pub rf_reg2a: RF_REG2A,
    #[doc = "0xac - XTAL_CTRL"]
    pub rf_xtal_ctrl: RF_XTAL_CTRL,
    #[doc = "0xb0 - REG2C"]
    pub rf_reg2c: RF_REG2C,
    #[doc = "0xb4 - REG2D"]
    pub rf_reg2d: RF_REG2D,
    #[doc = "0xb8 - REG2E"]
    pub rf_reg2e: RF_REG2E,
    #[doc = "0xbc - REG2F"]
    pub rf_reg2f: RF_REG2F,
    #[doc = "0xc0 - REG30"]
    pub rf_reg30: RF_REG30,
    #[doc = "0xc4 - REG31"]
    pub rf_reg31: RF_REG31,
    #[doc = "0xc8 - REG32"]
    pub rf_reg32: RF_REG32,
    #[doc = "0xcc - TXFIFO"]
    pub rf_txfifo: RF_TXFIFO,
    #[doc = "0xd0 - RXFIFO"]
    pub rf_rxfifo: RF_RXFIFO,
    #[doc = "0xd4 - DESER_STATUS"]
    pub rf_deser_status: RF_DESER_STATUS,
    #[doc = "0xd8 - IRQ_STATUS"]
    pub rf_irq_status: RF_IRQ_STATUS,
    #[doc = "0xdc - REG37"]
    pub rf_reg37: RF_REG37,
    #[doc = "0xe0 - REG38"]
    pub rf_reg38: RF_REG38,
    #[doc = "0xe4 - REG39"]
    pub rf_reg39: RF_REG39,
    _reserved58: [u8; 20usize],
    #[doc = "0xfc - REVISION"]
    pub rf_revision: RF_REVISION,
}
#[doc = "REG00\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg00](rf_reg00) module"]
pub type RF_REG00 = crate::Reg<u32, _RF_REG00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG00;
#[doc = "`read()` method returns [rf_reg00::R](rf_reg00::R) reader structure"]
impl crate::Readable for RF_REG00 {}
#[doc = "`write(|w| ..)` method takes [rf_reg00::W](rf_reg00::W) writer structure"]
impl crate::Writable for RF_REG00 {}
#[doc = "REG00"]
pub mod rf_reg00;
#[doc = "REG01\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg01](rf_reg01) module"]
pub type RF_REG01 = crate::Reg<u32, _RF_REG01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG01;
#[doc = "`read()` method returns [rf_reg01::R](rf_reg01::R) reader structure"]
impl crate::Readable for RF_REG01 {}
#[doc = "`write(|w| ..)` method takes [rf_reg01::W](rf_reg01::W) writer structure"]
impl crate::Writable for RF_REG01 {}
#[doc = "REG01"]
pub mod rf_reg01;
#[doc = "REG02\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg02](rf_reg02) module"]
pub type RF_REG02 = crate::Reg<u32, _RF_REG02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG02;
#[doc = "`read()` method returns [rf_reg02::R](rf_reg02::R) reader structure"]
impl crate::Readable for RF_REG02 {}
#[doc = "`write(|w| ..)` method takes [rf_reg02::W](rf_reg02::W) writer structure"]
impl crate::Writable for RF_REG02 {}
#[doc = "REG02"]
pub mod rf_reg02;
#[doc = "REG03\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg03](rf_reg03) module"]
pub type RF_REG03 = crate::Reg<u32, _RF_REG03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG03;
#[doc = "`read()` method returns [rf_reg03::R](rf_reg03::R) reader structure"]
impl crate::Readable for RF_REG03 {}
#[doc = "`write(|w| ..)` method takes [rf_reg03::W](rf_reg03::W) writer structure"]
impl crate::Writable for RF_REG03 {}
#[doc = "REG03"]
pub mod rf_reg03;
#[doc = "REG04\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg04](rf_reg04) module"]
pub type RF_REG04 = crate::Reg<u32, _RF_REG04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG04;
#[doc = "`read()` method returns [rf_reg04::R](rf_reg04::R) reader structure"]
impl crate::Readable for RF_REG04 {}
#[doc = "`write(|w| ..)` method takes [rf_reg04::W](rf_reg04::W) writer structure"]
impl crate::Writable for RF_REG04 {}
#[doc = "REG04"]
pub mod rf_reg04;
#[doc = "REG05\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg05](rf_reg05) module"]
pub type RF_REG05 = crate::Reg<u32, _RF_REG05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG05;
#[doc = "`read()` method returns [rf_reg05::R](rf_reg05::R) reader structure"]
impl crate::Readable for RF_REG05 {}
#[doc = "`write(|w| ..)` method takes [rf_reg05::W](rf_reg05::W) writer structure"]
impl crate::Writable for RF_REG05 {}
#[doc = "REG05"]
pub mod rf_reg05;
#[doc = "CENTER_FREQ\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_center_freq](rf_center_freq) module"]
pub type RF_CENTER_FREQ = crate::Reg<u32, _RF_CENTER_FREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_CENTER_FREQ;
#[doc = "`read()` method returns [rf_center_freq::R](rf_center_freq::R) reader structure"]
impl crate::Readable for RF_CENTER_FREQ {}
#[doc = "`write(|w| ..)` method takes [rf_center_freq::W](rf_center_freq::W) writer structure"]
impl crate::Writable for RF_CENTER_FREQ {}
#[doc = "CENTER_FREQ"]
pub mod rf_center_freq;
#[doc = "REG07\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg07](rf_reg07) module"]
pub type RF_REG07 = crate::Reg<u32, _RF_REG07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG07;
#[doc = "`read()` method returns [rf_reg07::R](rf_reg07::R) reader structure"]
impl crate::Readable for RF_REG07 {}
#[doc = "`write(|w| ..)` method takes [rf_reg07::W](rf_reg07::W) writer structure"]
impl crate::Writable for RF_REG07 {}
#[doc = "REG07"]
pub mod rf_reg07;
#[doc = "REG08\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg08](rf_reg08) module"]
pub type RF_REG08 = crate::Reg<u32, _RF_REG08>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG08;
#[doc = "`read()` method returns [rf_reg08::R](rf_reg08::R) reader structure"]
impl crate::Readable for RF_REG08 {}
#[doc = "`write(|w| ..)` method takes [rf_reg08::W](rf_reg08::W) writer structure"]
impl crate::Writable for RF_REG08 {}
#[doc = "REG08"]
pub mod rf_reg08;
#[doc = "REG09\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg09](rf_reg09) module"]
pub type RF_REG09 = crate::Reg<u32, _RF_REG09>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG09;
#[doc = "`read()` method returns [rf_reg09::R](rf_reg09::R) reader structure"]
impl crate::Readable for RF_REG09 {}
#[doc = "`write(|w| ..)` method takes [rf_reg09::W](rf_reg09::W) writer structure"]
impl crate::Writable for RF_REG09 {}
#[doc = "REG09"]
pub mod rf_reg09;
#[doc = "REG0A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg0a](rf_reg0a) module"]
pub type RF_REG0A = crate::Reg<u32, _RF_REG0A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG0A;
#[doc = "`read()` method returns [rf_reg0a::R](rf_reg0a::R) reader structure"]
impl crate::Readable for RF_REG0A {}
#[doc = "`write(|w| ..)` method takes [rf_reg0a::W](rf_reg0a::W) writer structure"]
impl crate::Writable for RF_REG0A {}
#[doc = "REG0A"]
pub mod rf_reg0a;
#[doc = "SYNC_PATTERN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_sync_pattern](rf_sync_pattern) module"]
pub type RF_SYNC_PATTERN = crate::Reg<u32, _RF_SYNC_PATTERN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_SYNC_PATTERN;
#[doc = "`read()` method returns [rf_sync_pattern::R](rf_sync_pattern::R) reader structure"]
impl crate::Readable for RF_SYNC_PATTERN {}
#[doc = "`write(|w| ..)` method takes [rf_sync_pattern::W](rf_sync_pattern::W) writer structure"]
impl crate::Writable for RF_SYNC_PATTERN {}
#[doc = "SYNC_PATTERN"]
pub mod rf_sync_pattern;
#[doc = "REG0C\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg0c](rf_reg0c) module"]
pub type RF_REG0C = crate::Reg<u32, _RF_REG0C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG0C;
#[doc = "`read()` method returns [rf_reg0c::R](rf_reg0c::R) reader structure"]
impl crate::Readable for RF_REG0C {}
#[doc = "`write(|w| ..)` method takes [rf_reg0c::W](rf_reg0c::W) writer structure"]
impl crate::Writable for RF_REG0C {}
#[doc = "REG0C"]
pub mod rf_reg0c;
#[doc = "CRC_POLYNOMIAL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_crc_polynomial](rf_crc_polynomial) module"]
pub type RF_CRC_POLYNOMIAL = crate::Reg<u32, _RF_CRC_POLYNOMIAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_CRC_POLYNOMIAL;
#[doc = "`read()` method returns [rf_crc_polynomial::R](rf_crc_polynomial::R) reader structure"]
impl crate::Readable for RF_CRC_POLYNOMIAL {}
#[doc = "`write(|w| ..)` method takes [rf_crc_polynomial::W](rf_crc_polynomial::W) writer structure"]
impl crate::Writable for RF_CRC_POLYNOMIAL {}
#[doc = "CRC_POLYNOMIAL"]
pub mod rf_crc_polynomial;
#[doc = "CRC_RST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_crc_rst](rf_crc_rst) module"]
pub type RF_CRC_RST = crate::Reg<u32, _RF_CRC_RST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_CRC_RST;
#[doc = "`read()` method returns [rf_crc_rst::R](rf_crc_rst::R) reader structure"]
impl crate::Readable for RF_CRC_RST {}
#[doc = "`write(|w| ..)` method takes [rf_crc_rst::W](rf_crc_rst::W) writer structure"]
impl crate::Writable for RF_CRC_RST {}
#[doc = "CRC_RST"]
pub mod rf_crc_rst;
#[doc = "REG0F\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg0f](rf_reg0f) module"]
pub type RF_REG0F = crate::Reg<u32, _RF_REG0F>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG0F;
#[doc = "`read()` method returns [rf_reg0f::R](rf_reg0f::R) reader structure"]
impl crate::Readable for RF_REG0F {}
#[doc = "`write(|w| ..)` method takes [rf_reg0f::W](rf_reg0f::W) writer structure"]
impl crate::Writable for RF_REG0F {}
#[doc = "REG0F"]
pub mod rf_reg0f;
#[doc = "REG10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg10](rf_reg10) module"]
pub type RF_REG10 = crate::Reg<u32, _RF_REG10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG10;
#[doc = "`read()` method returns [rf_reg10::R](rf_reg10::R) reader structure"]
impl crate::Readable for RF_REG10 {}
#[doc = "`write(|w| ..)` method takes [rf_reg10::W](rf_reg10::W) writer structure"]
impl crate::Writable for RF_REG10 {}
#[doc = "REG10"]
pub mod rf_reg10;
#[doc = "TX_PULSE0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_tx_pulse0](rf_tx_pulse0) module"]
pub type RF_TX_PULSE0 = crate::Reg<u32, _RF_TX_PULSE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_TX_PULSE0;
#[doc = "`read()` method returns [rf_tx_pulse0::R](rf_tx_pulse0::R) reader structure"]
impl crate::Readable for RF_TX_PULSE0 {}
#[doc = "`write(|w| ..)` method takes [rf_tx_pulse0::W](rf_tx_pulse0::W) writer structure"]
impl crate::Writable for RF_TX_PULSE0 {}
#[doc = "TX_PULSE0"]
pub mod rf_tx_pulse0;
#[doc = "TX_PULSE1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_tx_pulse1](rf_tx_pulse1) module"]
pub type RF_TX_PULSE1 = crate::Reg<u32, _RF_TX_PULSE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_TX_PULSE1;
#[doc = "`read()` method returns [rf_tx_pulse1::R](rf_tx_pulse1::R) reader structure"]
impl crate::Readable for RF_TX_PULSE1 {}
#[doc = "`write(|w| ..)` method takes [rf_tx_pulse1::W](rf_tx_pulse1::W) writer structure"]
impl crate::Writable for RF_TX_PULSE1 {}
#[doc = "TX_PULSE1"]
pub mod rf_tx_pulse1;
#[doc = "TX_PULSE2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_tx_pulse2](rf_tx_pulse2) module"]
pub type RF_TX_PULSE2 = crate::Reg<u32, _RF_TX_PULSE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_TX_PULSE2;
#[doc = "`read()` method returns [rf_tx_pulse2::R](rf_tx_pulse2::R) reader structure"]
impl crate::Readable for RF_TX_PULSE2 {}
#[doc = "`write(|w| ..)` method takes [rf_tx_pulse2::W](rf_tx_pulse2::W) writer structure"]
impl crate::Writable for RF_TX_PULSE2 {}
#[doc = "TX_PULSE2"]
pub mod rf_tx_pulse2;
#[doc = "TX_PULSE3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_tx_pulse3](rf_tx_pulse3) module"]
pub type RF_TX_PULSE3 = crate::Reg<u32, _RF_TX_PULSE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_TX_PULSE3;
#[doc = "`read()` method returns [rf_tx_pulse3::R](rf_tx_pulse3::R) reader structure"]
impl crate::Readable for RF_TX_PULSE3 {}
#[doc = "`write(|w| ..)` method takes [rf_tx_pulse3::W](rf_tx_pulse3::W) writer structure"]
impl crate::Writable for RF_TX_PULSE3 {}
#[doc = "TX_PULSE3"]
pub mod rf_tx_pulse3;
#[doc = "RX_PULSE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_rx_pulse](rf_rx_pulse) module"]
pub type RF_RX_PULSE = crate::Reg<u32, _RF_RX_PULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_RX_PULSE;
#[doc = "`read()` method returns [rf_rx_pulse::R](rf_rx_pulse::R) reader structure"]
impl crate::Readable for RF_RX_PULSE {}
#[doc = "`write(|w| ..)` method takes [rf_rx_pulse::W](rf_rx_pulse::W) writer structure"]
impl crate::Writable for RF_RX_PULSE {}
#[doc = "RX_PULSE"]
pub mod rf_rx_pulse;
#[doc = "REG16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg16](rf_reg16) module"]
pub type RF_REG16 = crate::Reg<u32, _RF_REG16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG16;
#[doc = "`read()` method returns [rf_reg16::R](rf_reg16::R) reader structure"]
impl crate::Readable for RF_REG16 {}
#[doc = "`write(|w| ..)` method takes [rf_reg16::W](rf_reg16::W) writer structure"]
impl crate::Writable for RF_REG16 {}
#[doc = "REG16"]
pub mod rf_reg16;
#[doc = "REG17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg17](rf_reg17) module"]
pub type RF_REG17 = crate::Reg<u32, _RF_REG17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG17;
#[doc = "`read()` method returns [rf_reg17::R](rf_reg17::R) reader structure"]
impl crate::Readable for RF_REG17 {}
#[doc = "`write(|w| ..)` method takes [rf_reg17::W](rf_reg17::W) writer structure"]
impl crate::Writable for RF_REG17 {}
#[doc = "REG17"]
pub mod rf_reg17;
#[doc = "REG18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg18](rf_reg18) module"]
pub type RF_REG18 = crate::Reg<u32, _RF_REG18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG18;
#[doc = "`read()` method returns [rf_reg18::R](rf_reg18::R) reader structure"]
impl crate::Readable for RF_REG18 {}
#[doc = "`write(|w| ..)` method takes [rf_reg18::W](rf_reg18::W) writer structure"]
impl crate::Writable for RF_REG18 {}
#[doc = "REG18"]
pub mod rf_reg18;
#[doc = "REG19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg19](rf_reg19) module"]
pub type RF_REG19 = crate::Reg<u32, _RF_REG19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG19;
#[doc = "`read()` method returns [rf_reg19::R](rf_reg19::R) reader structure"]
impl crate::Readable for RF_REG19 {}
#[doc = "`write(|w| ..)` method takes [rf_reg19::W](rf_reg19::W) writer structure"]
impl crate::Writable for RF_REG19 {}
#[doc = "REG19"]
pub mod rf_reg19;
#[doc = "REG1A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg1a](rf_reg1a) module"]
pub type RF_REG1A = crate::Reg<u32, _RF_REG1A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG1A;
#[doc = "`read()` method returns [rf_reg1a::R](rf_reg1a::R) reader structure"]
impl crate::Readable for RF_REG1A {}
#[doc = "`write(|w| ..)` method takes [rf_reg1a::W](rf_reg1a::W) writer structure"]
impl crate::Writable for RF_REG1A {}
#[doc = "REG1A"]
pub mod rf_reg1a;
#[doc = "REG1B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg1b](rf_reg1b) module"]
pub type RF_REG1B = crate::Reg<u32, _RF_REG1B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG1B;
#[doc = "`read()` method returns [rf_reg1b::R](rf_reg1b::R) reader structure"]
impl crate::Readable for RF_REG1B {}
#[doc = "`write(|w| ..)` method takes [rf_reg1b::W](rf_reg1b::W) writer structure"]
impl crate::Writable for RF_REG1B {}
#[doc = "REG1B"]
pub mod rf_reg1b;
#[doc = "AGC_LUT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_agc_lut1](rf_agc_lut1) module"]
pub type RF_AGC_LUT1 = crate::Reg<u32, _RF_AGC_LUT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_AGC_LUT1;
#[doc = "`read()` method returns [rf_agc_lut1::R](rf_agc_lut1::R) reader structure"]
impl crate::Readable for RF_AGC_LUT1 {}
#[doc = "`write(|w| ..)` method takes [rf_agc_lut1::W](rf_agc_lut1::W) writer structure"]
impl crate::Writable for RF_AGC_LUT1 {}
#[doc = "AGC_LUT1"]
pub mod rf_agc_lut1;
#[doc = "AGC_LUT2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_agc_lut2](rf_agc_lut2) module"]
pub type RF_AGC_LUT2 = crate::Reg<u32, _RF_AGC_LUT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_AGC_LUT2;
#[doc = "`read()` method returns [rf_agc_lut2::R](rf_agc_lut2::R) reader structure"]
impl crate::Readable for RF_AGC_LUT2 {}
#[doc = "`write(|w| ..)` method takes [rf_agc_lut2::W](rf_agc_lut2::W) writer structure"]
impl crate::Writable for RF_AGC_LUT2 {}
#[doc = "AGC_LUT2"]
pub mod rf_agc_lut2;
#[doc = "AGC_LUT3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_agc_lut3](rf_agc_lut3) module"]
pub type RF_AGC_LUT3 = crate::Reg<u32, _RF_AGC_LUT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_AGC_LUT3;
#[doc = "`read()` method returns [rf_agc_lut3::R](rf_agc_lut3::R) reader structure"]
impl crate::Readable for RF_AGC_LUT3 {}
#[doc = "`write(|w| ..)` method takes [rf_agc_lut3::W](rf_agc_lut3::W) writer structure"]
impl crate::Writable for RF_AGC_LUT3 {}
#[doc = "AGC_LUT3"]
pub mod rf_agc_lut3;
#[doc = "AGC_LUT4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_agc_lut4](rf_agc_lut4) module"]
pub type RF_AGC_LUT4 = crate::Reg<u32, _RF_AGC_LUT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_AGC_LUT4;
#[doc = "`read()` method returns [rf_agc_lut4::R](rf_agc_lut4::R) reader structure"]
impl crate::Readable for RF_AGC_LUT4 {}
#[doc = "`write(|w| ..)` method takes [rf_agc_lut4::W](rf_agc_lut4::W) writer structure"]
impl crate::Writable for RF_AGC_LUT4 {}
#[doc = "AGC_LUT4"]
pub mod rf_agc_lut4;
#[doc = "REG20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg20](rf_reg20) module"]
pub type RF_REG20 = crate::Reg<u32, _RF_REG20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG20;
#[doc = "`read()` method returns [rf_reg20::R](rf_reg20::R) reader structure"]
impl crate::Readable for RF_REG20 {}
#[doc = "`write(|w| ..)` method takes [rf_reg20::W](rf_reg20::W) writer structure"]
impl crate::Writable for RF_REG20 {}
#[doc = "REG20"]
pub mod rf_reg20;
#[doc = "AGC_ATT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_agc_att1](rf_agc_att1) module"]
pub type RF_AGC_ATT1 = crate::Reg<u32, _RF_AGC_ATT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_AGC_ATT1;
#[doc = "`read()` method returns [rf_agc_att1::R](rf_agc_att1::R) reader structure"]
impl crate::Readable for RF_AGC_ATT1 {}
#[doc = "`write(|w| ..)` method takes [rf_agc_att1::W](rf_agc_att1::W) writer structure"]
impl crate::Writable for RF_AGC_ATT1 {}
#[doc = "AGC_ATT1"]
pub mod rf_agc_att1;
#[doc = "REG22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg22](rf_reg22) module"]
pub type RF_REG22 = crate::Reg<u32, _RF_REG22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG22;
#[doc = "`read()` method returns [rf_reg22::R](rf_reg22::R) reader structure"]
impl crate::Readable for RF_REG22 {}
#[doc = "`write(|w| ..)` method takes [rf_reg22::W](rf_reg22::W) writer structure"]
impl crate::Writable for RF_REG22 {}
#[doc = "REG22"]
pub mod rf_reg22;
#[doc = "REG23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg23](rf_reg23) module"]
pub type RF_REG23 = crate::Reg<u32, _RF_REG23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG23;
#[doc = "`read()` method returns [rf_reg23::R](rf_reg23::R) reader structure"]
impl crate::Readable for RF_REG23 {}
#[doc = "`write(|w| ..)` method takes [rf_reg23::W](rf_reg23::W) writer structure"]
impl crate::Writable for RF_REG23 {}
#[doc = "REG23"]
pub mod rf_reg23;
#[doc = "REG24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg24](rf_reg24) module"]
pub type RF_REG24 = crate::Reg<u32, _RF_REG24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG24;
#[doc = "`read()` method returns [rf_reg24::R](rf_reg24::R) reader structure"]
impl crate::Readable for RF_REG24 {}
#[doc = "`write(|w| ..)` method takes [rf_reg24::W](rf_reg24::W) writer structure"]
impl crate::Writable for RF_REG24 {}
#[doc = "REG24"]
pub mod rf_reg24;
#[doc = "REG25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg25](rf_reg25) module"]
pub type RF_REG25 = crate::Reg<u32, _RF_REG25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG25;
#[doc = "`read()` method returns [rf_reg25::R](rf_reg25::R) reader structure"]
impl crate::Readable for RF_REG25 {}
#[doc = "`write(|w| ..)` method takes [rf_reg25::W](rf_reg25::W) writer structure"]
impl crate::Writable for RF_REG25 {}
#[doc = "REG25"]
pub mod rf_reg25;
#[doc = "REG26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg26](rf_reg26) module"]
pub type RF_REG26 = crate::Reg<u32, _RF_REG26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG26;
#[doc = "`read()` method returns [rf_reg26::R](rf_reg26::R) reader structure"]
impl crate::Readable for RF_REG26 {}
#[doc = "`write(|w| ..)` method takes [rf_reg26::W](rf_reg26::W) writer structure"]
impl crate::Writable for RF_REG26 {}
#[doc = "REG26"]
pub mod rf_reg26;
#[doc = "REG27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg27](rf_reg27) module"]
pub type RF_REG27 = crate::Reg<u32, _RF_REG27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG27;
#[doc = "`read()` method returns [rf_reg27::R](rf_reg27::R) reader structure"]
impl crate::Readable for RF_REG27 {}
#[doc = "`write(|w| ..)` method takes [rf_reg27::W](rf_reg27::W) writer structure"]
impl crate::Writable for RF_REG27 {}
#[doc = "REG27"]
pub mod rf_reg27;
#[doc = "REG28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg28](rf_reg28) module"]
pub type RF_REG28 = crate::Reg<u32, _RF_REG28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG28;
#[doc = "`read()` method returns [rf_reg28::R](rf_reg28::R) reader structure"]
impl crate::Readable for RF_REG28 {}
#[doc = "`write(|w| ..)` method takes [rf_reg28::W](rf_reg28::W) writer structure"]
impl crate::Writable for RF_REG28 {}
#[doc = "REG28"]
pub mod rf_reg28;
#[doc = "PLL_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_pll_ctrl](rf_pll_ctrl) module"]
pub type RF_PLL_CTRL = crate::Reg<u32, _RF_PLL_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_PLL_CTRL;
#[doc = "`read()` method returns [rf_pll_ctrl::R](rf_pll_ctrl::R) reader structure"]
impl crate::Readable for RF_PLL_CTRL {}
#[doc = "`write(|w| ..)` method takes [rf_pll_ctrl::W](rf_pll_ctrl::W) writer structure"]
impl crate::Writable for RF_PLL_CTRL {}
#[doc = "PLL_CTRL"]
pub mod rf_pll_ctrl;
#[doc = "REG2A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg2a](rf_reg2a) module"]
pub type RF_REG2A = crate::Reg<u32, _RF_REG2A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG2A;
#[doc = "`read()` method returns [rf_reg2a::R](rf_reg2a::R) reader structure"]
impl crate::Readable for RF_REG2A {}
#[doc = "`write(|w| ..)` method takes [rf_reg2a::W](rf_reg2a::W) writer structure"]
impl crate::Writable for RF_REG2A {}
#[doc = "REG2A"]
pub mod rf_reg2a;
#[doc = "XTAL_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_xtal_ctrl](rf_xtal_ctrl) module"]
pub type RF_XTAL_CTRL = crate::Reg<u32, _RF_XTAL_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_XTAL_CTRL;
#[doc = "`read()` method returns [rf_xtal_ctrl::R](rf_xtal_ctrl::R) reader structure"]
impl crate::Readable for RF_XTAL_CTRL {}
#[doc = "`write(|w| ..)` method takes [rf_xtal_ctrl::W](rf_xtal_ctrl::W) writer structure"]
impl crate::Writable for RF_XTAL_CTRL {}
#[doc = "XTAL_CTRL"]
pub mod rf_xtal_ctrl;
#[doc = "REG2C\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg2c](rf_reg2c) module"]
pub type RF_REG2C = crate::Reg<u32, _RF_REG2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG2C;
#[doc = "`read()` method returns [rf_reg2c::R](rf_reg2c::R) reader structure"]
impl crate::Readable for RF_REG2C {}
#[doc = "`write(|w| ..)` method takes [rf_reg2c::W](rf_reg2c::W) writer structure"]
impl crate::Writable for RF_REG2C {}
#[doc = "REG2C"]
pub mod rf_reg2c;
#[doc = "REG2D\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg2d](rf_reg2d) module"]
pub type RF_REG2D = crate::Reg<u32, _RF_REG2D>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG2D;
#[doc = "`read()` method returns [rf_reg2d::R](rf_reg2d::R) reader structure"]
impl crate::Readable for RF_REG2D {}
#[doc = "`write(|w| ..)` method takes [rf_reg2d::W](rf_reg2d::W) writer structure"]
impl crate::Writable for RF_REG2D {}
#[doc = "REG2D"]
pub mod rf_reg2d;
#[doc = "REG2E\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg2e](rf_reg2e) module"]
pub type RF_REG2E = crate::Reg<u32, _RF_REG2E>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG2E;
#[doc = "`read()` method returns [rf_reg2e::R](rf_reg2e::R) reader structure"]
impl crate::Readable for RF_REG2E {}
#[doc = "`write(|w| ..)` method takes [rf_reg2e::W](rf_reg2e::W) writer structure"]
impl crate::Writable for RF_REG2E {}
#[doc = "REG2E"]
pub mod rf_reg2e;
#[doc = "REG2F\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg2f](rf_reg2f) module"]
pub type RF_REG2F = crate::Reg<u32, _RF_REG2F>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG2F;
#[doc = "`read()` method returns [rf_reg2f::R](rf_reg2f::R) reader structure"]
impl crate::Readable for RF_REG2F {}
#[doc = "`write(|w| ..)` method takes [rf_reg2f::W](rf_reg2f::W) writer structure"]
impl crate::Writable for RF_REG2F {}
#[doc = "REG2F"]
pub mod rf_reg2f;
#[doc = "REG30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg30](rf_reg30) module"]
pub type RF_REG30 = crate::Reg<u32, _RF_REG30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG30;
#[doc = "`read()` method returns [rf_reg30::R](rf_reg30::R) reader structure"]
impl crate::Readable for RF_REG30 {}
#[doc = "`write(|w| ..)` method takes [rf_reg30::W](rf_reg30::W) writer structure"]
impl crate::Writable for RF_REG30 {}
#[doc = "REG30"]
pub mod rf_reg30;
#[doc = "REG31\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg31](rf_reg31) module"]
pub type RF_REG31 = crate::Reg<u32, _RF_REG31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG31;
#[doc = "`read()` method returns [rf_reg31::R](rf_reg31::R) reader structure"]
impl crate::Readable for RF_REG31 {}
#[doc = "REG31"]
pub mod rf_reg31;
#[doc = "REG32\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg32](rf_reg32) module"]
pub type RF_REG32 = crate::Reg<u32, _RF_REG32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG32;
#[doc = "`read()` method returns [rf_reg32::R](rf_reg32::R) reader structure"]
impl crate::Readable for RF_REG32 {}
#[doc = "REG32"]
pub mod rf_reg32;
#[doc = "TXFIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_txfifo](rf_txfifo) module"]
pub type RF_TXFIFO = crate::Reg<u32, _RF_TXFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_TXFIFO;
#[doc = "`read()` method returns [rf_txfifo::R](rf_txfifo::R) reader structure"]
impl crate::Readable for RF_TXFIFO {}
#[doc = "`write(|w| ..)` method takes [rf_txfifo::W](rf_txfifo::W) writer structure"]
impl crate::Writable for RF_TXFIFO {}
#[doc = "TXFIFO"]
pub mod rf_txfifo;
#[doc = "RXFIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_rxfifo](rf_rxfifo) module"]
pub type RF_RXFIFO = crate::Reg<u32, _RF_RXFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_RXFIFO;
#[doc = "`read()` method returns [rf_rxfifo::R](rf_rxfifo::R) reader structure"]
impl crate::Readable for RF_RXFIFO {}
#[doc = "RXFIFO"]
pub mod rf_rxfifo;
#[doc = "DESER_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_deser_status](rf_deser_status) module"]
pub type RF_DESER_STATUS = crate::Reg<u32, _RF_DESER_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_DESER_STATUS;
#[doc = "`read()` method returns [rf_deser_status::R](rf_deser_status::R) reader structure"]
impl crate::Readable for RF_DESER_STATUS {}
#[doc = "DESER_STATUS"]
pub mod rf_deser_status;
#[doc = "IRQ_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_irq_status](rf_irq_status) module"]
pub type RF_IRQ_STATUS = crate::Reg<u32, _RF_IRQ_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_IRQ_STATUS;
#[doc = "`read()` method returns [rf_irq_status::R](rf_irq_status::R) reader structure"]
impl crate::Readable for RF_IRQ_STATUS {}
#[doc = "IRQ_STATUS"]
pub mod rf_irq_status;
#[doc = "REG37\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg37](rf_reg37) module"]
pub type RF_REG37 = crate::Reg<u32, _RF_REG37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG37;
#[doc = "`read()` method returns [rf_reg37::R](rf_reg37::R) reader structure"]
impl crate::Readable for RF_REG37 {}
#[doc = "REG37"]
pub mod rf_reg37;
#[doc = "REG38\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg38](rf_reg38) module"]
pub type RF_REG38 = crate::Reg<u32, _RF_REG38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG38;
#[doc = "`read()` method returns [rf_reg38::R](rf_reg38::R) reader structure"]
impl crate::Readable for RF_REG38 {}
#[doc = "REG38"]
pub mod rf_reg38;
#[doc = "REG39\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reg39](rf_reg39) module"]
pub type RF_REG39 = crate::Reg<u32, _RF_REG39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REG39;
#[doc = "`read()` method returns [rf_reg39::R](rf_reg39::R) reader structure"]
impl crate::Readable for RF_REG39 {}
#[doc = "REG39"]
pub mod rf_reg39;
#[doc = "REVISION\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_revision](rf_revision) module"]
pub type RF_REVISION = crate::Reg<u32, _RF_REVISION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REVISION;
#[doc = "`read()` method returns [rf_revision::R](rf_revision::R) reader structure"]
impl crate::Readable for RF_REVISION {}
#[doc = "REVISION"]
pub mod rf_revision;
