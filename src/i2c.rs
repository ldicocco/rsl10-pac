#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Interface Configuration and Control"]
    pub i2c_ctrl0: I2C_CTRL0,
    #[doc = "0x04 - I2C Interface Status and Control"]
    pub i2c_ctrl1: I2C_CTRL1,
    #[doc = "0x08 - I2C Interface Data"]
    pub i2c_data: I2C_DATA,
    #[doc = "0x0c - I2C Interface Data (Mirror)"]
    pub i2c_data_m: I2C_DATA_M,
    #[doc = "0x10 - I2C Master Address and Start"]
    pub i2c_addr_start: I2C_ADDR_START,
    #[doc = "0x14 - I2C Status"]
    pub i2c_status: I2C_STATUS,
}
#[doc = "I2C Interface Configuration and Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ctrl0](i2c_ctrl0) module"]
pub type I2C_CTRL0 = crate::Reg<u32, _I2C_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_CTRL0;
#[doc = "`read()` method returns [i2c_ctrl0::R](i2c_ctrl0::R) reader structure"]
impl crate::Readable for I2C_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [i2c_ctrl0::W](i2c_ctrl0::W) writer structure"]
impl crate::Writable for I2C_CTRL0 {}
#[doc = "I2C Interface Configuration and Control"]
pub mod i2c_ctrl0;
#[doc = "I2C Interface Status and Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ctrl1](i2c_ctrl1) module"]
pub type I2C_CTRL1 = crate::Reg<u32, _I2C_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_CTRL1;
#[doc = "`read()` method returns [i2c_ctrl1::R](i2c_ctrl1::R) reader structure"]
impl crate::Readable for I2C_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [i2c_ctrl1::W](i2c_ctrl1::W) writer structure"]
impl crate::Writable for I2C_CTRL1 {}
#[doc = "I2C Interface Status and Control"]
pub mod i2c_ctrl1;
#[doc = "I2C Interface Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_data](i2c_data) module"]
pub type I2C_DATA = crate::Reg<u32, _I2C_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_DATA;
#[doc = "`read()` method returns [i2c_data::R](i2c_data::R) reader structure"]
impl crate::Readable for I2C_DATA {}
#[doc = "`write(|w| ..)` method takes [i2c_data::W](i2c_data::W) writer structure"]
impl crate::Writable for I2C_DATA {}
#[doc = "I2C Interface Data"]
pub mod i2c_data;
#[doc = "I2C Interface Data (Mirror)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_data_m](i2c_data_m) module"]
pub type I2C_DATA_M = crate::Reg<u32, _I2C_DATA_M>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_DATA_M;
#[doc = "`read()` method returns [i2c_data_m::R](i2c_data_m::R) reader structure"]
impl crate::Readable for I2C_DATA_M {}
#[doc = "`write(|w| ..)` method takes [i2c_data_m::W](i2c_data_m::W) writer structure"]
impl crate::Writable for I2C_DATA_M {}
#[doc = "I2C Interface Data (Mirror)"]
pub mod i2c_data_m;
#[doc = "I2C Master Address and Start\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_addr_start](i2c_addr_start) module"]
pub type I2C_ADDR_START = crate::Reg<u32, _I2C_ADDR_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_ADDR_START;
#[doc = "`read()` method returns [i2c_addr_start::R](i2c_addr_start::R) reader structure"]
impl crate::Readable for I2C_ADDR_START {}
#[doc = "`write(|w| ..)` method takes [i2c_addr_start::W](i2c_addr_start::W) writer structure"]
impl crate::Writable for I2C_ADDR_START {}
#[doc = "I2C Master Address and Start"]
pub mod i2c_addr_start;
#[doc = "I2C Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_status](i2c_status) module"]
pub type I2C_STATUS = crate::Reg<u32, _I2C_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_STATUS;
#[doc = "`read()` method returns [i2c_status::R](i2c_status::R) reader structure"]
impl crate::Readable for I2C_STATUS {}
#[doc = "I2C Status"]
pub mod i2c_status;
