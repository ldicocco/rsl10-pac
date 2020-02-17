#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Debug Halting Control and Status Register"]
    pub debug_dhcsr: DEBUG_DHCSR,
    #[doc = "0x04 - Debug Core Register Selector Register"]
    pub debug_dcrsr: DEBUG_DCRSR,
    #[doc = "0x08 - Debug Core Register Data Register"]
    pub debug_dcrdr: DEBUG_DCRDR,
    #[doc = "0x0c - Debug Exception and Monitor Control Register"]
    pub debug_demcr: DEBUG_DEMCR,
}
#[doc = "Debug Halting Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_dhcsr](debug_dhcsr) module"]
pub type DEBUG_DHCSR = crate::Reg<u32, _DEBUG_DHCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG_DHCSR;
#[doc = "`read()` method returns [debug_dhcsr::R](debug_dhcsr::R) reader structure"]
impl crate::Readable for DEBUG_DHCSR {}
#[doc = "`write(|w| ..)` method takes [debug_dhcsr::W](debug_dhcsr::W) writer structure"]
impl crate::Writable for DEBUG_DHCSR {}
#[doc = "Debug Halting Control and Status Register"]
pub mod debug_dhcsr;
#[doc = "Debug Core Register Selector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_dcrsr](debug_dcrsr) module"]
pub type DEBUG_DCRSR = crate::Reg<u32, _DEBUG_DCRSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG_DCRSR;
#[doc = "`read()` method returns [debug_dcrsr::R](debug_dcrsr::R) reader structure"]
impl crate::Readable for DEBUG_DCRSR {}
#[doc = "`write(|w| ..)` method takes [debug_dcrsr::W](debug_dcrsr::W) writer structure"]
impl crate::Writable for DEBUG_DCRSR {}
#[doc = "Debug Core Register Selector Register"]
pub mod debug_dcrsr;
#[doc = "Debug Core Register Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_dcrdr](debug_dcrdr) module"]
pub type DEBUG_DCRDR = crate::Reg<u32, _DEBUG_DCRDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG_DCRDR;
#[doc = "`read()` method returns [debug_dcrdr::R](debug_dcrdr::R) reader structure"]
impl crate::Readable for DEBUG_DCRDR {}
#[doc = "`write(|w| ..)` method takes [debug_dcrdr::W](debug_dcrdr::W) writer structure"]
impl crate::Writable for DEBUG_DCRDR {}
#[doc = "Debug Core Register Data Register"]
pub mod debug_dcrdr;
#[doc = "Debug Exception and Monitor Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_demcr](debug_demcr) module"]
pub type DEBUG_DEMCR = crate::Reg<u32, _DEBUG_DEMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG_DEMCR;
#[doc = "`read()` method returns [debug_demcr::R](debug_demcr::R) reader structure"]
impl crate::Readable for DEBUG_DEMCR {}
#[doc = "`write(|w| ..)` method takes [debug_demcr::W](debug_demcr::W) writer structure"]
impl crate::Writable for DEBUG_DEMCR {}
#[doc = "Debug Exception and Monitor Control Register"]
pub mod debug_demcr;
