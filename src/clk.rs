#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Clock Configuration Register"]
    pub clk_sys_cfg: CLK_SYS_CFG,
    #[doc = "0x04 - Prescale register for SLOWCLK, BBCLK and USRCLK clocks"]
    pub clk_div_cfg0: CLK_DIV_CFG0,
    #[doc = "0x08 - Prescale register for PWM clock, UART and DMIC clocks"]
    pub clk_div_cfg1: CLK_DIV_CFG1,
    #[doc = "0x0c - Prescale register for DC-DC converter and charge pump clocks"]
    pub clk_div_cfg2: CLK_DIV_CFG2,
    #[doc = "0x10 - External clock detector configuration register (including interrupt)"]
    pub clk_det_cfg: CLK_DET_CFG,
    #[doc = "0x14 - External clock detector status register"]
    pub clk_det_status: CLK_DET_STATUS,
}
#[doc = "System Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_sys_cfg](clk_sys_cfg) module"]
pub type CLK_SYS_CFG = crate::Reg<u32, _CLK_SYS_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_SYS_CFG;
#[doc = "`read()` method returns [clk_sys_cfg::R](clk_sys_cfg::R) reader structure"]
impl crate::Readable for CLK_SYS_CFG {}
#[doc = "`write(|w| ..)` method takes [clk_sys_cfg::W](clk_sys_cfg::W) writer structure"]
impl crate::Writable for CLK_SYS_CFG {}
#[doc = "System Clock Configuration Register"]
pub mod clk_sys_cfg;
#[doc = "Prescale register for SLOWCLK, BBCLK and USRCLK clocks\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_div_cfg0](clk_div_cfg0) module"]
pub type CLK_DIV_CFG0 = crate::Reg<u32, _CLK_DIV_CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_DIV_CFG0;
#[doc = "`read()` method returns [clk_div_cfg0::R](clk_div_cfg0::R) reader structure"]
impl crate::Readable for CLK_DIV_CFG0 {}
#[doc = "`write(|w| ..)` method takes [clk_div_cfg0::W](clk_div_cfg0::W) writer structure"]
impl crate::Writable for CLK_DIV_CFG0 {}
#[doc = "Prescale register for SLOWCLK, BBCLK and USRCLK clocks"]
pub mod clk_div_cfg0;
#[doc = "Prescale register for PWM clock, UART and DMIC clocks\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_div_cfg1](clk_div_cfg1) module"]
pub type CLK_DIV_CFG1 = crate::Reg<u32, _CLK_DIV_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_DIV_CFG1;
#[doc = "`read()` method returns [clk_div_cfg1::R](clk_div_cfg1::R) reader structure"]
impl crate::Readable for CLK_DIV_CFG1 {}
#[doc = "`write(|w| ..)` method takes [clk_div_cfg1::W](clk_div_cfg1::W) writer structure"]
impl crate::Writable for CLK_DIV_CFG1 {}
#[doc = "Prescale register for PWM clock, UART and DMIC clocks"]
pub mod clk_div_cfg1;
#[doc = "Prescale register for DC-DC converter and charge pump clocks\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_div_cfg2](clk_div_cfg2) module"]
pub type CLK_DIV_CFG2 = crate::Reg<u32, _CLK_DIV_CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_DIV_CFG2;
#[doc = "`read()` method returns [clk_div_cfg2::R](clk_div_cfg2::R) reader structure"]
impl crate::Readable for CLK_DIV_CFG2 {}
#[doc = "`write(|w| ..)` method takes [clk_div_cfg2::W](clk_div_cfg2::W) writer structure"]
impl crate::Writable for CLK_DIV_CFG2 {}
#[doc = "Prescale register for DC-DC converter and charge pump clocks"]
pub mod clk_div_cfg2;
#[doc = "External clock detector configuration register (including interrupt)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_det_cfg](clk_det_cfg) module"]
pub type CLK_DET_CFG = crate::Reg<u32, _CLK_DET_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_DET_CFG;
#[doc = "`read()` method returns [clk_det_cfg::R](clk_det_cfg::R) reader structure"]
impl crate::Readable for CLK_DET_CFG {}
#[doc = "`write(|w| ..)` method takes [clk_det_cfg::W](clk_det_cfg::W) writer structure"]
impl crate::Writable for CLK_DET_CFG {}
#[doc = "External clock detector configuration register (including interrupt)"]
pub mod clk_det_cfg;
#[doc = "External clock detector status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_det_status](clk_det_status) module"]
pub type CLK_DET_STATUS = crate::Reg<u32, _CLK_DET_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_DET_STATUS;
#[doc = "`read()` method returns [clk_det_status::R](clk_det_status::R) reader structure"]
impl crate::Readable for CLK_DET_STATUS {}
#[doc = "External clock detector status register"]
pub mod clk_det_status;
