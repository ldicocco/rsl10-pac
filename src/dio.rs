#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Digital IO Configuration (ADC function only for pads 0 to 3)"]
    pub dio_cfg: [DIO_CFG; 16],
    #[doc = "0x40 - Digital IOs Data Access Register"]
    pub dio_data: DIO_DATA,
    #[doc = "0x44 - Digital IOs Direction State"]
    pub dio_dir: DIO_DIR,
    #[doc = "0x48 - Digital IOs Mode State"]
    pub dio_mode: DIO_MODE,
    #[doc = "0x4c - DIO Interrupt Configuration"]
    pub dio_int_cfg: [DIO_INT_CFG; 4],
    #[doc = "0x5c - DIO Interrupt Button Debounce Filter Time Configuration"]
    pub dio_int_debounce: DIO_INT_DEBOUNCE,
    #[doc = "0x60 - PCM Input Selection"]
    pub dio_pcm_src: DIO_PCM_SRC,
    #[doc = "0x64 - SPI\\[1:0\\]
Interface Input Selection"]
    pub dio_spi_src: [DIO_SPI_SRC; 2],
    #[doc = "0x6c - UART Interface Input Selection"]
    pub dio_uart_src: DIO_UART_SRC,
    #[doc = "0x70 - I2C Input Selection"]
    pub dio_i2c_src: DIO_I2C_SRC,
    #[doc = "0x74 - Audio Sink Input Selection"]
    pub dio_audiosink_src: DIO_AUDIOSINK_SRC,
    #[doc = "0x78 - NMI Input Selection"]
    pub dio_nmi_src: DIO_NMI_SRC,
    #[doc = "0x7c - Baseband Controller RX Data And Clock Input Selection"]
    pub dio_bb_rx_src: DIO_BB_RX_SRC,
    #[doc = "0x80 - Baseband Controller SPI Input Selection"]
    pub dio_bb_spi_src: DIO_BB_SPI_SRC,
    #[doc = "0x84 - RF Front-End SPI Input Selection"]
    pub dio_rf_spi_src: DIO_RF_SPI_SRC,
    #[doc = "0x88 - RF Front-End GPIOs 0-3 Input Selection"]
    pub dio_rf_gpio03_src: DIO_RF_GPIO03_SRC,
    #[doc = "0x8c - RF Front-End GPIOs 4-7 Input Selection"]
    pub dio_rf_gpio47_src: DIO_RF_GPIO47_SRC,
    #[doc = "0x90 - RF Front-End GPIOs 8-9 Input Selection"]
    pub dio_rf_gpio89_src: DIO_RF_GPIO89_SRC,
    #[doc = "0x94 - DMIC Data Input Selection Register"]
    pub dio_dmic_src: DIO_DMIC_SRC,
    #[doc = "0x98 - LPDSP32 JTAG Configuration Register"]
    pub dio_lpdsp32_jtag_src: DIO_LPDSP32_JTAG_SRC,
    #[doc = "0x9c - JTAG / SW Pad Configuration Register"]
    pub dio_jtag_sw_pad_cfg: DIO_JTAG_SW_PAD_CFG,
    #[doc = "0xa0 - External Clock Pad Configuration Register"]
    pub dio_extclk_cfg: DIO_EXTCLK_CFG,
    #[doc = "0xa4 - Global Pads Configuration Register"]
    pub dio_pad_cfg: DIO_PAD_CFG,
}
#[doc = "Digital IO Configuration (ADC function only for pads 0 to 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_cfg](dio_cfg) module"]
pub type DIO_CFG = crate::Reg<u32, _DIO_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_CFG;
#[doc = "`read()` method returns [dio_cfg::R](dio_cfg::R) reader structure"]
impl crate::Readable for DIO_CFG {}
#[doc = "`write(|w| ..)` method takes [dio_cfg::W](dio_cfg::W) writer structure"]
impl crate::Writable for DIO_CFG {}
#[doc = "Digital IO Configuration (ADC function only for pads 0 to 3)"]
pub mod dio_cfg;
#[doc = "Digital IOs Data Access Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_data](dio_data) module"]
pub type DIO_DATA = crate::Reg<u32, _DIO_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_DATA;
#[doc = "`read()` method returns [dio_data::R](dio_data::R) reader structure"]
impl crate::Readable for DIO_DATA {}
#[doc = "`write(|w| ..)` method takes [dio_data::W](dio_data::W) writer structure"]
impl crate::Writable for DIO_DATA {}
#[doc = "Digital IOs Data Access Register"]
pub mod dio_data;
#[doc = "Digital IOs Direction State\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_dir](dio_dir) module"]
pub type DIO_DIR = crate::Reg<u32, _DIO_DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_DIR;
#[doc = "`read()` method returns [dio_dir::R](dio_dir::R) reader structure"]
impl crate::Readable for DIO_DIR {}
#[doc = "`write(|w| ..)` method takes [dio_dir::W](dio_dir::W) writer structure"]
impl crate::Writable for DIO_DIR {}
#[doc = "Digital IOs Direction State"]
pub mod dio_dir;
#[doc = "Digital IOs Mode State\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_mode](dio_mode) module"]
pub type DIO_MODE = crate::Reg<u32, _DIO_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_MODE;
#[doc = "`read()` method returns [dio_mode::R](dio_mode::R) reader structure"]
impl crate::Readable for DIO_MODE {}
#[doc = "Digital IOs Mode State"]
pub mod dio_mode;
#[doc = "DIO Interrupt Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_int_cfg](dio_int_cfg) module"]
pub type DIO_INT_CFG = crate::Reg<u32, _DIO_INT_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_INT_CFG;
#[doc = "`read()` method returns [dio_int_cfg::R](dio_int_cfg::R) reader structure"]
impl crate::Readable for DIO_INT_CFG {}
#[doc = "`write(|w| ..)` method takes [dio_int_cfg::W](dio_int_cfg::W) writer structure"]
impl crate::Writable for DIO_INT_CFG {}
#[doc = "DIO Interrupt Configuration"]
pub mod dio_int_cfg;
#[doc = "DIO Interrupt Button Debounce Filter Time Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_int_debounce](dio_int_debounce) module"]
pub type DIO_INT_DEBOUNCE = crate::Reg<u32, _DIO_INT_DEBOUNCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_INT_DEBOUNCE;
#[doc = "`read()` method returns [dio_int_debounce::R](dio_int_debounce::R) reader structure"]
impl crate::Readable for DIO_INT_DEBOUNCE {}
#[doc = "`write(|w| ..)` method takes [dio_int_debounce::W](dio_int_debounce::W) writer structure"]
impl crate::Writable for DIO_INT_DEBOUNCE {}
#[doc = "DIO Interrupt Button Debounce Filter Time Configuration"]
pub mod dio_int_debounce;
#[doc = "PCM Input Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_pcm_src](dio_pcm_src) module"]
pub type DIO_PCM_SRC = crate::Reg<u32, _DIO_PCM_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_PCM_SRC;
#[doc = "`read()` method returns [dio_pcm_src::R](dio_pcm_src::R) reader structure"]
impl crate::Readable for DIO_PCM_SRC {}
#[doc = "`write(|w| ..)` method takes [dio_pcm_src::W](dio_pcm_src::W) writer structure"]
impl crate::Writable for DIO_PCM_SRC {}
#[doc = "PCM Input Selection"]
pub mod dio_pcm_src;
#[doc = "SPI\\[1:0\\]
Interface Input Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_spi_src](dio_spi_src) module"]
pub type DIO_SPI_SRC = crate::Reg<u32, _DIO_SPI_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_SPI_SRC;
#[doc = "`read()` method returns [dio_spi_src::R](dio_spi_src::R) reader structure"]
impl crate::Readable for DIO_SPI_SRC {}
#[doc = "`write(|w| ..)` method takes [dio_spi_src::W](dio_spi_src::W) writer structure"]
impl crate::Writable for DIO_SPI_SRC {}
#[doc = "SPI\\[1:0\\]
Interface Input Selection"]
pub mod dio_spi_src;
#[doc = "UART Interface Input Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_uart_src](dio_uart_src) module"]
pub type DIO_UART_SRC = crate::Reg<u32, _DIO_UART_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_UART_SRC;
#[doc = "`read()` method returns [dio_uart_src::R](dio_uart_src::R) reader structure"]
impl crate::Readable for DIO_UART_SRC {}
#[doc = "`write(|w| ..)` method takes [dio_uart_src::W](dio_uart_src::W) writer structure"]
impl crate::Writable for DIO_UART_SRC {}
#[doc = "UART Interface Input Selection"]
pub mod dio_uart_src;
#[doc = "I2C Input Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_i2c_src](dio_i2c_src) module"]
pub type DIO_I2C_SRC = crate::Reg<u32, _DIO_I2C_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_I2C_SRC;
#[doc = "`read()` method returns [dio_i2c_src::R](dio_i2c_src::R) reader structure"]
impl crate::Readable for DIO_I2C_SRC {}
#[doc = "`write(|w| ..)` method takes [dio_i2c_src::W](dio_i2c_src::W) writer structure"]
impl crate::Writable for DIO_I2C_SRC {}
#[doc = "I2C Input Selection"]
pub mod dio_i2c_src;
#[doc = "Audio Sink Input Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_audiosink_src](dio_audiosink_src) module"]
pub type DIO_AUDIOSINK_SRC = crate::Reg<u32, _DIO_AUDIOSINK_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_AUDIOSINK_SRC;
#[doc = "`read()` method returns [dio_audiosink_src::R](dio_audiosink_src::R) reader structure"]
impl crate::Readable for DIO_AUDIOSINK_SRC {}
#[doc = "`write(|w| ..)` method takes [dio_audiosink_src::W](dio_audiosink_src::W) writer structure"]
impl crate::Writable for DIO_AUDIOSINK_SRC {}
#[doc = "Audio Sink Input Selection"]
pub mod dio_audiosink_src;
#[doc = "NMI Input Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_nmi_src](dio_nmi_src) module"]
pub type DIO_NMI_SRC = crate::Reg<u32, _DIO_NMI_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_NMI_SRC;
#[doc = "`read()` method returns [dio_nmi_src::R](dio_nmi_src::R) reader structure"]
impl crate::Readable for DIO_NMI_SRC {}
#[doc = "`write(|w| ..)` method takes [dio_nmi_src::W](dio_nmi_src::W) writer structure"]
impl crate::Writable for DIO_NMI_SRC {}
#[doc = "NMI Input Selection"]
pub mod dio_nmi_src;
#[doc = "Baseband Controller RX Data And Clock Input Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_bb_rx_src](dio_bb_rx_src) module"]
pub type DIO_BB_RX_SRC = crate::Reg<u32, _DIO_BB_RX_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_BB_RX_SRC;
#[doc = "`read()` method returns [dio_bb_rx_src::R](dio_bb_rx_src::R) reader structure"]
impl crate::Readable for DIO_BB_RX_SRC {}
#[doc = "`write(|w| ..)` method takes [dio_bb_rx_src::W](dio_bb_rx_src::W) writer structure"]
impl crate::Writable for DIO_BB_RX_SRC {}
#[doc = "Baseband Controller RX Data And Clock Input Selection"]
pub mod dio_bb_rx_src;
#[doc = "Baseband Controller SPI Input Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_bb_spi_src](dio_bb_spi_src) module"]
pub type DIO_BB_SPI_SRC = crate::Reg<u32, _DIO_BB_SPI_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_BB_SPI_SRC;
#[doc = "`read()` method returns [dio_bb_spi_src::R](dio_bb_spi_src::R) reader structure"]
impl crate::Readable for DIO_BB_SPI_SRC {}
#[doc = "`write(|w| ..)` method takes [dio_bb_spi_src::W](dio_bb_spi_src::W) writer structure"]
impl crate::Writable for DIO_BB_SPI_SRC {}
#[doc = "Baseband Controller SPI Input Selection"]
pub mod dio_bb_spi_src;
#[doc = "RF Front-End SPI Input Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_rf_spi_src](dio_rf_spi_src) module"]
pub type DIO_RF_SPI_SRC = crate::Reg<u32, _DIO_RF_SPI_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_RF_SPI_SRC;
#[doc = "`read()` method returns [dio_rf_spi_src::R](dio_rf_spi_src::R) reader structure"]
impl crate::Readable for DIO_RF_SPI_SRC {}
#[doc = "`write(|w| ..)` method takes [dio_rf_spi_src::W](dio_rf_spi_src::W) writer structure"]
impl crate::Writable for DIO_RF_SPI_SRC {}
#[doc = "RF Front-End SPI Input Selection"]
pub mod dio_rf_spi_src;
#[doc = "RF Front-End GPIOs 0-3 Input Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_rf_gpio03_src](dio_rf_gpio03_src) module"]
pub type DIO_RF_GPIO03_SRC = crate::Reg<u32, _DIO_RF_GPIO03_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_RF_GPIO03_SRC;
#[doc = "`read()` method returns [dio_rf_gpio03_src::R](dio_rf_gpio03_src::R) reader structure"]
impl crate::Readable for DIO_RF_GPIO03_SRC {}
#[doc = "`write(|w| ..)` method takes [dio_rf_gpio03_src::W](dio_rf_gpio03_src::W) writer structure"]
impl crate::Writable for DIO_RF_GPIO03_SRC {}
#[doc = "RF Front-End GPIOs 0-3 Input Selection"]
pub mod dio_rf_gpio03_src;
#[doc = "RF Front-End GPIOs 4-7 Input Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_rf_gpio47_src](dio_rf_gpio47_src) module"]
pub type DIO_RF_GPIO47_SRC = crate::Reg<u32, _DIO_RF_GPIO47_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_RF_GPIO47_SRC;
#[doc = "`read()` method returns [dio_rf_gpio47_src::R](dio_rf_gpio47_src::R) reader structure"]
impl crate::Readable for DIO_RF_GPIO47_SRC {}
#[doc = "`write(|w| ..)` method takes [dio_rf_gpio47_src::W](dio_rf_gpio47_src::W) writer structure"]
impl crate::Writable for DIO_RF_GPIO47_SRC {}
#[doc = "RF Front-End GPIOs 4-7 Input Selection"]
pub mod dio_rf_gpio47_src;
#[doc = "RF Front-End GPIOs 8-9 Input Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_rf_gpio89_src](dio_rf_gpio89_src) module"]
pub type DIO_RF_GPIO89_SRC = crate::Reg<u32, _DIO_RF_GPIO89_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_RF_GPIO89_SRC;
#[doc = "`read()` method returns [dio_rf_gpio89_src::R](dio_rf_gpio89_src::R) reader structure"]
impl crate::Readable for DIO_RF_GPIO89_SRC {}
#[doc = "`write(|w| ..)` method takes [dio_rf_gpio89_src::W](dio_rf_gpio89_src::W) writer structure"]
impl crate::Writable for DIO_RF_GPIO89_SRC {}
#[doc = "RF Front-End GPIOs 8-9 Input Selection"]
pub mod dio_rf_gpio89_src;
#[doc = "DMIC Data Input Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_dmic_src](dio_dmic_src) module"]
pub type DIO_DMIC_SRC = crate::Reg<u32, _DIO_DMIC_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_DMIC_SRC;
#[doc = "`read()` method returns [dio_dmic_src::R](dio_dmic_src::R) reader structure"]
impl crate::Readable for DIO_DMIC_SRC {}
#[doc = "`write(|w| ..)` method takes [dio_dmic_src::W](dio_dmic_src::W) writer structure"]
impl crate::Writable for DIO_DMIC_SRC {}
#[doc = "DMIC Data Input Selection Register"]
pub mod dio_dmic_src;
#[doc = "LPDSP32 JTAG Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_lpdsp32_jtag_src](dio_lpdsp32_jtag_src) module"]
pub type DIO_LPDSP32_JTAG_SRC = crate::Reg<u32, _DIO_LPDSP32_JTAG_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_LPDSP32_JTAG_SRC;
#[doc = "`read()` method returns [dio_lpdsp32_jtag_src::R](dio_lpdsp32_jtag_src::R) reader structure"]
impl crate::Readable for DIO_LPDSP32_JTAG_SRC {}
#[doc = "`write(|w| ..)` method takes [dio_lpdsp32_jtag_src::W](dio_lpdsp32_jtag_src::W) writer structure"]
impl crate::Writable for DIO_LPDSP32_JTAG_SRC {}
#[doc = "LPDSP32 JTAG Configuration Register"]
pub mod dio_lpdsp32_jtag_src;
#[doc = "JTAG / SW Pad Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_jtag_sw_pad_cfg](dio_jtag_sw_pad_cfg) module"]
pub type DIO_JTAG_SW_PAD_CFG = crate::Reg<u32, _DIO_JTAG_SW_PAD_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_JTAG_SW_PAD_CFG;
#[doc = "`read()` method returns [dio_jtag_sw_pad_cfg::R](dio_jtag_sw_pad_cfg::R) reader structure"]
impl crate::Readable for DIO_JTAG_SW_PAD_CFG {}
#[doc = "`write(|w| ..)` method takes [dio_jtag_sw_pad_cfg::W](dio_jtag_sw_pad_cfg::W) writer structure"]
impl crate::Writable for DIO_JTAG_SW_PAD_CFG {}
#[doc = "JTAG / SW Pad Configuration Register"]
pub mod dio_jtag_sw_pad_cfg;
#[doc = "External Clock Pad Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_extclk_cfg](dio_extclk_cfg) module"]
pub type DIO_EXTCLK_CFG = crate::Reg<u32, _DIO_EXTCLK_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_EXTCLK_CFG;
#[doc = "`read()` method returns [dio_extclk_cfg::R](dio_extclk_cfg::R) reader structure"]
impl crate::Readable for DIO_EXTCLK_CFG {}
#[doc = "`write(|w| ..)` method takes [dio_extclk_cfg::W](dio_extclk_cfg::W) writer structure"]
impl crate::Writable for DIO_EXTCLK_CFG {}
#[doc = "External Clock Pad Configuration Register"]
pub mod dio_extclk_cfg;
#[doc = "Global Pads Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dio_pad_cfg](dio_pad_cfg) module"]
pub type DIO_PAD_CFG = crate::Reg<u32, _DIO_PAD_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIO_PAD_CFG;
#[doc = "`read()` method returns [dio_pad_cfg::R](dio_pad_cfg::R) reader structure"]
impl crate::Readable for DIO_PAD_CFG {}
#[doc = "`write(|w| ..)` method takes [dio_pad_cfg::W](dio_pad_cfg::W) writer structure"]
impl crate::Writable for DIO_PAD_CFG {}
#[doc = "Global Pads Configuration Register"]
pub mod dio_pad_cfg;
