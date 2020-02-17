#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC Generator Control"]
    pub crc_ctrl: CRC_CTRL,
    #[doc = "0x04 - CRC Generator Current Value"]
    pub crc_value: CRC_VALUE,
    #[doc = "0x08 - CRC Generator - Add 1 Bit"]
    pub crc_add_1: CRC_ADD_1,
    #[doc = "0x0c - CRC Generator - Add 1 Byte"]
    pub crc_add_8: CRC_ADD_8,
    #[doc = "0x10 - CRC Generator - Add 1 Half-word"]
    pub crc_add_16: CRC_ADD_16,
    #[doc = "0x14 - CRC Generator - Add 3 Bytes"]
    pub crc_add_24: CRC_ADD_24,
    #[doc = "0x18 - CRC Generator - Add 1 Word"]
    pub crc_add_32: CRC_ADD_32,
    #[doc = "0x1c - CRC Generator Final Value"]
    pub crc_final: CRC_FINAL,
}
#[doc = "CRC Generator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_ctrl](crc_ctrl) module"]
pub type CRC_CTRL = crate::Reg<u32, _CRC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_CTRL;
#[doc = "`read()` method returns [crc_ctrl::R](crc_ctrl::R) reader structure"]
impl crate::Readable for CRC_CTRL {}
#[doc = "`write(|w| ..)` method takes [crc_ctrl::W](crc_ctrl::W) writer structure"]
impl crate::Writable for CRC_CTRL {}
#[doc = "CRC Generator Control"]
pub mod crc_ctrl;
#[doc = "CRC Generator Current Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_value](crc_value) module"]
pub type CRC_VALUE = crate::Reg<u32, _CRC_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_VALUE;
#[doc = "`read()` method returns [crc_value::R](crc_value::R) reader structure"]
impl crate::Readable for CRC_VALUE {}
#[doc = "`write(|w| ..)` method takes [crc_value::W](crc_value::W) writer structure"]
impl crate::Writable for CRC_VALUE {}
#[doc = "CRC Generator Current Value"]
pub mod crc_value;
#[doc = "CRC Generator - Add 1 Bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_add_1](crc_add_1) module"]
pub type CRC_ADD_1 = crate::Reg<u32, _CRC_ADD_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_ADD_1;
#[doc = "`read()` method returns [crc_add_1::R](crc_add_1::R) reader structure"]
impl crate::Readable for CRC_ADD_1 {}
#[doc = "`write(|w| ..)` method takes [crc_add_1::W](crc_add_1::W) writer structure"]
impl crate::Writable for CRC_ADD_1 {}
#[doc = "CRC Generator - Add 1 Bit"]
pub mod crc_add_1;
#[doc = "CRC Generator - Add 1 Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_add_8](crc_add_8) module"]
pub type CRC_ADD_8 = crate::Reg<u32, _CRC_ADD_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_ADD_8;
#[doc = "`read()` method returns [crc_add_8::R](crc_add_8::R) reader structure"]
impl crate::Readable for CRC_ADD_8 {}
#[doc = "`write(|w| ..)` method takes [crc_add_8::W](crc_add_8::W) writer structure"]
impl crate::Writable for CRC_ADD_8 {}
#[doc = "CRC Generator - Add 1 Byte"]
pub mod crc_add_8;
#[doc = "CRC Generator - Add 1 Half-word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_add_16](crc_add_16) module"]
pub type CRC_ADD_16 = crate::Reg<u32, _CRC_ADD_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_ADD_16;
#[doc = "`read()` method returns [crc_add_16::R](crc_add_16::R) reader structure"]
impl crate::Readable for CRC_ADD_16 {}
#[doc = "`write(|w| ..)` method takes [crc_add_16::W](crc_add_16::W) writer structure"]
impl crate::Writable for CRC_ADD_16 {}
#[doc = "CRC Generator - Add 1 Half-word"]
pub mod crc_add_16;
#[doc = "CRC Generator - Add 3 Bytes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_add_24](crc_add_24) module"]
pub type CRC_ADD_24 = crate::Reg<u32, _CRC_ADD_24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_ADD_24;
#[doc = "`read()` method returns [crc_add_24::R](crc_add_24::R) reader structure"]
impl crate::Readable for CRC_ADD_24 {}
#[doc = "`write(|w| ..)` method takes [crc_add_24::W](crc_add_24::W) writer structure"]
impl crate::Writable for CRC_ADD_24 {}
#[doc = "CRC Generator - Add 3 Bytes"]
pub mod crc_add_24;
#[doc = "CRC Generator - Add 1 Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_add_32](crc_add_32) module"]
pub type CRC_ADD_32 = crate::Reg<u32, _CRC_ADD_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_ADD_32;
#[doc = "`read()` method returns [crc_add_32::R](crc_add_32::R) reader structure"]
impl crate::Readable for CRC_ADD_32 {}
#[doc = "`write(|w| ..)` method takes [crc_add_32::W](crc_add_32::W) writer structure"]
impl crate::Writable for CRC_ADD_32 {}
#[doc = "CRC Generator - Add 1 Word"]
pub mod crc_add_32;
#[doc = "CRC Generator Final Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_final](crc_final) module"]
pub type CRC_FINAL = crate::Reg<u32, _CRC_FINAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_FINAL;
#[doc = "`read()` method returns [crc_final::R](crc_final::R) reader structure"]
impl crate::Readable for CRC_FINAL {}
#[doc = "CRC Generator Final Value"]
pub mod crc_final;
