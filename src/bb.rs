#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Baseband control register"]
    pub bb_rwbbcntl: BB_RWBBCNTL,
    #[doc = "0x04 - BLE revision register"]
    pub bb_version: BB_VERSION,
    #[doc = "0x08 - Baseband configuration register (compilation options dependant)"]
    pub bb_rwblebconf: BB_RWBLEBCONF,
    #[doc = "0x0c - Interrupts control register"]
    pub bb_intcntl: BB_INTCNTL,
    #[doc = "0x10 - Interrupts status register"]
    pub bb_intstat: BB_INTSTAT,
    #[doc = "0x14 - Interrupts raw status register"]
    pub bb_intrawstat: BB_INTRAWSTAT,
    #[doc = "0x18 - Interrupts acknowledgement register"]
    pub bb_intack: BB_INTACK,
    #[doc = "0x1c - base timer configuration register"]
    pub bb_basetimecnt: BB_BASETIMECNT,
    #[doc = "0x20 - Fine timer configuration register"]
    pub bb_finetimecnt: BB_FINETIMECNT,
    #[doc = "0x24 - BLE device address (LSB part) register"]
    pub bb_bdaddrl: BB_BDADDRL,
    #[doc = "0x28 - BLE device address (MSB part) register"]
    pub bb_bdaddru: BB_BDADDRU,
    #[doc = "0x2c - Rx descriptor pointer register"]
    pub bb_et_currentrxdescptr: BB_ET_CURRENTRXDESCPTR,
    #[doc = "0x30 - Deep sleep control register"]
    pub bb_deepslcntl: BB_DEEPSLCNTL,
    #[doc = "0x34 - Deep sleep wakeup register"]
    pub bb_deepslwkup: BB_DEEPSLWKUP,
    #[doc = "0x38 - Deep sleep status register"]
    pub bb_deepslstat: BB_DEEPSLSTAT,
    #[doc = "0x3c - Stabilization times"]
    pub bb_enbpreset: BB_ENBPRESET,
    #[doc = "0x40 - Fine timer correction register"]
    pub bb_finecntcorr: BB_FINECNTCORR,
    #[doc = "0x44 - Base timer correction register"]
    pub bb_basetimecntcorr: BB_BASETIMECNTCORR,
    _reserved18: [u8; 8usize],
    #[doc = "0x50 - Diagnostic ports control register"]
    pub bb_diagcntl: BB_DIAGCNTL,
    #[doc = "0x54 - Diagnostic ports status register"]
    pub bb_diagstat: BB_DIAGSTAT,
    #[doc = "0x58 - Diagnostic ports upper limit"]
    pub bb_debugaddmax: BB_DEBUGADDMAX,
    #[doc = "0x5c - Diagnostic ports lower limit"]
    pub bb_debugaddmin: BB_DEBUGADDMIN,
    #[doc = "0x60 - Diagnostic ports errors register"]
    pub bb_errortypestat: BB_ERRORTYPESTAT,
    #[doc = "0x64 - Software profiling register"]
    pub bb_swprofiling: BB_SWPROFILING,
    _reserved24: [u8; 8usize],
    #[doc = "0x70 - Principal control register for the radio interface"]
    pub bb_radiocntl0: BB_RADIOCNTL0,
    #[doc = "0x74 - Second control register for the radio interface"]
    pub bb_radiocntl1: BB_RADIOCNTL1,
    #[doc = "0x78 - Third control register for the radio interface"]
    pub bb_radiocntl2: BB_RADIOCNTL2,
    _reserved27: [u8; 4usize],
    #[doc = "0x80 - Principal control register for the radio interface power up/down delays"]
    pub bb_radiopwrupdn0: BB_RADIOPWRUPDN0,
    #[doc = "0x84 - Second control register for the radio interface power up/down delays"]
    pub bb_radiopwrupdn1: BB_RADIOPWRUPDN1,
    _reserved29: [u8; 8usize],
    #[doc = "0x90 - Principal control register for the radio interface timing compensation delays"]
    pub bb_radiotxrxtim0: BB_RADIOTXRXTIM0,
    #[doc = "0x94 - Second control register for the radio interface timing compensation delays"]
    pub bb_radiotxrxtim1: BB_RADIOTXRXTIM1,
    _reserved31: [u8; 8usize],
    #[doc = "0xa0 - First control register for the radio interface SPI pointers"]
    pub bb_spiptrcntl0: BB_SPIPTRCNTL0,
    #[doc = "0xa4 - Second control register for the radio interface SPI pointers"]
    pub bb_spiptrcntl1: BB_SPIPTRCNTL1,
    #[doc = "0xa8 - Third control register for the radio interface SPI pointers"]
    pub bb_spiptrcntl2: BB_SPIPTRCNTL2,
    _reserved34: [u8; 4usize],
    #[doc = "0xb0 - Advertising channel map register"]
    pub bb_advchmap: BB_ADVCHMAP,
    _reserved35: [u8; 12usize],
    #[doc = "0xc0 - Delay information register handling advertising event timers"]
    pub bb_advtim: BB_ADVTIM,
    #[doc = "0xc4 - Active scan mode control register"]
    pub bb_actscanstat: BB_ACTSCANSTAT,
    _reserved37: [u8; 8usize],
    #[doc = "0xd0 - Address pointer of public devices"]
    pub bb_wlpubaddptr: BB_WLPUBADDPTR,
    #[doc = "0xd4 - Address pointer of private devices"]
    pub bb_wlprivaddptr: BB_WLPRIVADDPTR,
    #[doc = "0xd8 - Devices in white list"]
    pub bb_wlnbdev: BB_WLNBDEV,
    _reserved40: [u8; 4usize],
    #[doc = "0xe0 - AES-128 ciphering control register"]
    pub bb_aescntl: BB_AESCNTL,
    #[doc = "0xe4 - AES encryption 128-bit key register (bits 31:0)"]
    pub bb_aeskey31_0: BB_AESKEY31_0,
    #[doc = "0xe8 - AES encryption 128-bit key register (bits 63:32)"]
    pub bb_aeskey63_32: BB_AESKEY63_32,
    #[doc = "0xec - AES encryption 128-bit key register (bits 95:64)"]
    pub bb_aeskey95_64: BB_AESKEY95_64,
    #[doc = "0xf0 - AES encryption 128-bit key register (bits 127:96)"]
    pub bb_aeskey127_96: BB_AESKEY127_96,
    #[doc = "0xf4 - AES memory zone pointer"]
    pub bb_aesptr: BB_AESPTR,
    #[doc = "0xf8 - AES-CCM plain MIC value register in Tx"]
    pub bb_txmicval: BB_TXMICVAL,
    #[doc = "0xfc - AES-CCM plain MIC value register in Rx"]
    pub bb_rxmicval: BB_RXMICVAL,
    #[doc = "0x100 - RF testing and regulatory body support register"]
    pub bb_rftestcntl: BB_RFTESTCNTL,
    #[doc = "0x104 - Number of transmitted packet during test modes"]
    pub bb_rftesttxstat: BB_RFTESTTXSTAT,
    #[doc = "0x108 - Number of correctly received packet during test modes"]
    pub bb_rftestrxstat: BB_RFTESTRXSTAT,
    _reserved51: [u8; 4usize],
    #[doc = "0x110 - Timing generator control register"]
    pub bb_timgencntl: BB_TIMGENCNTL,
    #[doc = "0x114 - Gross timer control register"]
    pub bb_grosstimtgt: BB_GROSSTIMTGT,
    #[doc = "0x118 - Fine timer control register"]
    pub bb_finetimtgt: BB_FINETIMTGT,
    _reserved54: [u8; 4usize],
    #[doc = "0x120 - Coexistence control register 0"]
    pub bb_coexifcntl0: BB_COEXIFCNTL0,
    #[doc = "0x124 - Coexistence control register 1"]
    pub bb_coexifcntl1: BB_COEXIFCNTL1,
    #[doc = "0x128 - Coexistence control register 2"]
    pub bb_coexifcntl2: BB_COEXIFCNTL2,
    #[doc = "0x12c - Priority control register 0"]
    pub bb_bbmprio0: BB_BBMPRIO0,
    #[doc = "0x130 - Priority control register 1"]
    pub bb_bbmprio1: BB_BBMPRIO1,
    _reserved59: [u8; 12usize],
    #[doc = "0x140 - Register used by the Resolving Address List engine"]
    pub bb_ralptr: BB_RALPTR,
    #[doc = "0x144 - Register used by the Resolving Address List engine"]
    pub bb_ralnbdev: BB_RALNBDEV,
    #[doc = "0x148 - Register used by the Resolving Address List engine"]
    pub bb_ral_local_rnd: BB_RAL_LOCAL_RND,
    #[doc = "0x14c - Register used by the Resolving Address List engine"]
    pub bb_ral_peer_rnd: BB_RAL_PEER_RND,
    #[doc = "0x150 - ISO Channel 0 control"]
    pub bb_isochancntl0: BB_ISOCHANCNTL0,
    #[doc = "0x154 - ISO Channel 0 mute control"]
    pub bb_isomutecntl0: BB_ISOMUTECNTL0,
    #[doc = "0x158 - ISO Channel 0 current Tx pointer"]
    pub bb_isocurrenttxptr0: BB_ISOCURRENTTXPTR0,
    #[doc = "0x15c - ISO Channel 0 current Rx pointer"]
    pub bb_isocurrentrxptr0: BB_ISOCURRENTRXPTR0,
    #[doc = "0x160 - ISO Channel 0 payloads"]
    pub bb_isotrcnl0: BB_ISOTRCNL0,
    #[doc = "0x164 - ISO Channel 0 mute control"]
    pub bb_isoevtcntloffsetl0: BB_ISOEVTCNTLOFFSETL0,
    #[doc = "0x168 - ISO Channel 0 mute control"]
    pub bb_isoevtcntloffsetu0: BB_ISOEVTCNTLOFFSETU0,
    _reserved70: [u8; 4usize],
    #[doc = "0x170 - ISO Channel 1 control"]
    pub bb_isochancntl1: BB_ISOCHANCNTL1,
    #[doc = "0x174 - ISO Channel 1 mute control"]
    pub bb_isomutecntl1: BB_ISOMUTECNTL1,
    #[doc = "0x178 - ISO Channel 1 current Tx pointer"]
    pub bb_isocurrenttxptr1: BB_ISOCURRENTTXPTR1,
    #[doc = "0x17c - ISO Channel 1 current Rx pointer"]
    pub bb_isocurrentrxptr1: BB_ISOCURRENTRXPTR1,
    #[doc = "0x180 - ISO Channel 1 payloads"]
    pub bb_isotrcnl1: BB_ISOTRCNL1,
    #[doc = "0x184 - ISO Channel 1 mute control"]
    pub bb_isoevtcntloffsetl1: BB_ISOEVTCNTLOFFSETL1,
    #[doc = "0x188 - ISO Channel 1 mute control"]
    pub bb_isoevtcntloffsetu1: BB_ISOEVTCNTLOFFSETU1,
    _reserved77: [u8; 4usize],
    #[doc = "0x190 - ISO Channel 2 control"]
    pub bb_isochancntl2: BB_ISOCHANCNTL2,
    #[doc = "0x194 - ISO Channel 2 mute control"]
    pub bb_isomutecntl2: BB_ISOMUTECNTL2,
    #[doc = "0x198 - ISO Channel 2 current Tx pointer"]
    pub bb_isocurrenttxptr2: BB_ISOCURRENTTXPTR2,
    #[doc = "0x19c - ISO Channel 2 current Rx pointer"]
    pub bb_isocurrentrxptr2: BB_ISOCURRENTRXPTR2,
    #[doc = "0x1a0 - ISO Channel 2 payloads"]
    pub bb_isotrcnl2: BB_ISOTRCNL2,
    #[doc = "0x1a4 - ISO Channel 2 mute control"]
    pub bb_isoevtcntloffsetl2: BB_ISOEVTCNTLOFFSETL2,
    #[doc = "0x1a8 - ISO Channel 2 mute control"]
    pub bb_isoevtcntloffsetu2: BB_ISOEVTCNTLOFFSETU2,
    _reserved84: [u8; 4usize],
    #[doc = "0x1b0 - Register controlling the decision instant for priority scheduling arbitration"]
    pub bb_bbprioscharb: BB_BBPRIOSCHARB,
}
#[doc = "Baseband control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_rwbbcntl](bb_rwbbcntl) module"]
pub type BB_RWBBCNTL = crate::Reg<u32, _BB_RWBBCNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_RWBBCNTL;
#[doc = "`read()` method returns [bb_rwbbcntl::R](bb_rwbbcntl::R) reader structure"]
impl crate::Readable for BB_RWBBCNTL {}
#[doc = "`write(|w| ..)` method takes [bb_rwbbcntl::W](bb_rwbbcntl::W) writer structure"]
impl crate::Writable for BB_RWBBCNTL {}
#[doc = "Baseband control register"]
pub mod bb_rwbbcntl;
#[doc = "BLE revision register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_version](bb_version) module"]
pub type BB_VERSION = crate::Reg<u32, _BB_VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_VERSION;
#[doc = "`read()` method returns [bb_version::R](bb_version::R) reader structure"]
impl crate::Readable for BB_VERSION {}
#[doc = "BLE revision register"]
pub mod bb_version;
#[doc = "Baseband configuration register (compilation options dependant)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_rwblebconf](bb_rwblebconf) module"]
pub type BB_RWBLEBCONF = crate::Reg<u32, _BB_RWBLEBCONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_RWBLEBCONF;
#[doc = "`read()` method returns [bb_rwblebconf::R](bb_rwblebconf::R) reader structure"]
impl crate::Readable for BB_RWBLEBCONF {}
#[doc = "Baseband configuration register (compilation options dependant)"]
pub mod bb_rwblebconf;
#[doc = "Interrupts control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_intcntl](bb_intcntl) module"]
pub type BB_INTCNTL = crate::Reg<u32, _BB_INTCNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_INTCNTL;
#[doc = "`read()` method returns [bb_intcntl::R](bb_intcntl::R) reader structure"]
impl crate::Readable for BB_INTCNTL {}
#[doc = "`write(|w| ..)` method takes [bb_intcntl::W](bb_intcntl::W) writer structure"]
impl crate::Writable for BB_INTCNTL {}
#[doc = "Interrupts control register"]
pub mod bb_intcntl;
#[doc = "Interrupts status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_intstat](bb_intstat) module"]
pub type BB_INTSTAT = crate::Reg<u32, _BB_INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_INTSTAT;
#[doc = "`read()` method returns [bb_intstat::R](bb_intstat::R) reader structure"]
impl crate::Readable for BB_INTSTAT {}
#[doc = "Interrupts status register"]
pub mod bb_intstat;
#[doc = "Interrupts raw status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_intrawstat](bb_intrawstat) module"]
pub type BB_INTRAWSTAT = crate::Reg<u32, _BB_INTRAWSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_INTRAWSTAT;
#[doc = "`read()` method returns [bb_intrawstat::R](bb_intrawstat::R) reader structure"]
impl crate::Readable for BB_INTRAWSTAT {}
#[doc = "Interrupts raw status register"]
pub mod bb_intrawstat;
#[doc = "Interrupts acknowledgement register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_intack](bb_intack) module"]
pub type BB_INTACK = crate::Reg<u32, _BB_INTACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_INTACK;
#[doc = "`read()` method returns [bb_intack::R](bb_intack::R) reader structure"]
impl crate::Readable for BB_INTACK {}
#[doc = "`write(|w| ..)` method takes [bb_intack::W](bb_intack::W) writer structure"]
impl crate::Writable for BB_INTACK {}
#[doc = "Interrupts acknowledgement register"]
pub mod bb_intack;
#[doc = "base timer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_basetimecnt](bb_basetimecnt) module"]
pub type BB_BASETIMECNT = crate::Reg<u32, _BB_BASETIMECNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_BASETIMECNT;
#[doc = "`read()` method returns [bb_basetimecnt::R](bb_basetimecnt::R) reader structure"]
impl crate::Readable for BB_BASETIMECNT {}
#[doc = "`write(|w| ..)` method takes [bb_basetimecnt::W](bb_basetimecnt::W) writer structure"]
impl crate::Writable for BB_BASETIMECNT {}
#[doc = "base timer configuration register"]
pub mod bb_basetimecnt;
#[doc = "Fine timer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_finetimecnt](bb_finetimecnt) module"]
pub type BB_FINETIMECNT = crate::Reg<u32, _BB_FINETIMECNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_FINETIMECNT;
#[doc = "`read()` method returns [bb_finetimecnt::R](bb_finetimecnt::R) reader structure"]
impl crate::Readable for BB_FINETIMECNT {}
#[doc = "Fine timer configuration register"]
pub mod bb_finetimecnt;
#[doc = "BLE device address (LSB part) register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_bdaddrl](bb_bdaddrl) module"]
pub type BB_BDADDRL = crate::Reg<u32, _BB_BDADDRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_BDADDRL;
#[doc = "`read()` method returns [bb_bdaddrl::R](bb_bdaddrl::R) reader structure"]
impl crate::Readable for BB_BDADDRL {}
#[doc = "`write(|w| ..)` method takes [bb_bdaddrl::W](bb_bdaddrl::W) writer structure"]
impl crate::Writable for BB_BDADDRL {}
#[doc = "BLE device address (LSB part) register"]
pub mod bb_bdaddrl;
#[doc = "BLE device address (MSB part) register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_bdaddru](bb_bdaddru) module"]
pub type BB_BDADDRU = crate::Reg<u32, _BB_BDADDRU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_BDADDRU;
#[doc = "`read()` method returns [bb_bdaddru::R](bb_bdaddru::R) reader structure"]
impl crate::Readable for BB_BDADDRU {}
#[doc = "`write(|w| ..)` method takes [bb_bdaddru::W](bb_bdaddru::W) writer structure"]
impl crate::Writable for BB_BDADDRU {}
#[doc = "BLE device address (MSB part) register"]
pub mod bb_bdaddru;
#[doc = "Rx descriptor pointer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_et_currentrxdescptr](bb_et_currentrxdescptr) module"]
pub type BB_ET_CURRENTRXDESCPTR = crate::Reg<u32, _BB_ET_CURRENTRXDESCPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ET_CURRENTRXDESCPTR;
#[doc = "`read()` method returns [bb_et_currentrxdescptr::R](bb_et_currentrxdescptr::R) reader structure"]
impl crate::Readable for BB_ET_CURRENTRXDESCPTR {}
#[doc = "`write(|w| ..)` method takes [bb_et_currentrxdescptr::W](bb_et_currentrxdescptr::W) writer structure"]
impl crate::Writable for BB_ET_CURRENTRXDESCPTR {}
#[doc = "Rx descriptor pointer register"]
pub mod bb_et_currentrxdescptr;
#[doc = "Deep sleep control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_deepslcntl](bb_deepslcntl) module"]
pub type BB_DEEPSLCNTL = crate::Reg<u32, _BB_DEEPSLCNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_DEEPSLCNTL;
#[doc = "`read()` method returns [bb_deepslcntl::R](bb_deepslcntl::R) reader structure"]
impl crate::Readable for BB_DEEPSLCNTL {}
#[doc = "`write(|w| ..)` method takes [bb_deepslcntl::W](bb_deepslcntl::W) writer structure"]
impl crate::Writable for BB_DEEPSLCNTL {}
#[doc = "Deep sleep control register"]
pub mod bb_deepslcntl;
#[doc = "Deep sleep wakeup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_deepslwkup](bb_deepslwkup) module"]
pub type BB_DEEPSLWKUP = crate::Reg<u32, _BB_DEEPSLWKUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_DEEPSLWKUP;
#[doc = "`read()` method returns [bb_deepslwkup::R](bb_deepslwkup::R) reader structure"]
impl crate::Readable for BB_DEEPSLWKUP {}
#[doc = "`write(|w| ..)` method takes [bb_deepslwkup::W](bb_deepslwkup::W) writer structure"]
impl crate::Writable for BB_DEEPSLWKUP {}
#[doc = "Deep sleep wakeup register"]
pub mod bb_deepslwkup;
#[doc = "Deep sleep status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_deepslstat](bb_deepslstat) module"]
pub type BB_DEEPSLSTAT = crate::Reg<u32, _BB_DEEPSLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_DEEPSLSTAT;
#[doc = "`read()` method returns [bb_deepslstat::R](bb_deepslstat::R) reader structure"]
impl crate::Readable for BB_DEEPSLSTAT {}
#[doc = "Deep sleep status register"]
pub mod bb_deepslstat;
#[doc = "Stabilization times\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_enbpreset](bb_enbpreset) module"]
pub type BB_ENBPRESET = crate::Reg<u32, _BB_ENBPRESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ENBPRESET;
#[doc = "`read()` method returns [bb_enbpreset::R](bb_enbpreset::R) reader structure"]
impl crate::Readable for BB_ENBPRESET {}
#[doc = "`write(|w| ..)` method takes [bb_enbpreset::W](bb_enbpreset::W) writer structure"]
impl crate::Writable for BB_ENBPRESET {}
#[doc = "Stabilization times"]
pub mod bb_enbpreset;
#[doc = "Fine timer correction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_finecntcorr](bb_finecntcorr) module"]
pub type BB_FINECNTCORR = crate::Reg<u32, _BB_FINECNTCORR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_FINECNTCORR;
#[doc = "`read()` method returns [bb_finecntcorr::R](bb_finecntcorr::R) reader structure"]
impl crate::Readable for BB_FINECNTCORR {}
#[doc = "`write(|w| ..)` method takes [bb_finecntcorr::W](bb_finecntcorr::W) writer structure"]
impl crate::Writable for BB_FINECNTCORR {}
#[doc = "Fine timer correction register"]
pub mod bb_finecntcorr;
#[doc = "Base timer correction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_basetimecntcorr](bb_basetimecntcorr) module"]
pub type BB_BASETIMECNTCORR = crate::Reg<u32, _BB_BASETIMECNTCORR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_BASETIMECNTCORR;
#[doc = "`read()` method returns [bb_basetimecntcorr::R](bb_basetimecntcorr::R) reader structure"]
impl crate::Readable for BB_BASETIMECNTCORR {}
#[doc = "`write(|w| ..)` method takes [bb_basetimecntcorr::W](bb_basetimecntcorr::W) writer structure"]
impl crate::Writable for BB_BASETIMECNTCORR {}
#[doc = "Base timer correction register"]
pub mod bb_basetimecntcorr;
#[doc = "Diagnostic ports control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_diagcntl](bb_diagcntl) module"]
pub type BB_DIAGCNTL = crate::Reg<u32, _BB_DIAGCNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_DIAGCNTL;
#[doc = "`read()` method returns [bb_diagcntl::R](bb_diagcntl::R) reader structure"]
impl crate::Readable for BB_DIAGCNTL {}
#[doc = "`write(|w| ..)` method takes [bb_diagcntl::W](bb_diagcntl::W) writer structure"]
impl crate::Writable for BB_DIAGCNTL {}
#[doc = "Diagnostic ports control register"]
pub mod bb_diagcntl;
#[doc = "Diagnostic ports status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_diagstat](bb_diagstat) module"]
pub type BB_DIAGSTAT = crate::Reg<u32, _BB_DIAGSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_DIAGSTAT;
#[doc = "`read()` method returns [bb_diagstat::R](bb_diagstat::R) reader structure"]
impl crate::Readable for BB_DIAGSTAT {}
#[doc = "Diagnostic ports status register"]
pub mod bb_diagstat;
#[doc = "Diagnostic ports upper limit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_debugaddmax](bb_debugaddmax) module"]
pub type BB_DEBUGADDMAX = crate::Reg<u32, _BB_DEBUGADDMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_DEBUGADDMAX;
#[doc = "`read()` method returns [bb_debugaddmax::R](bb_debugaddmax::R) reader structure"]
impl crate::Readable for BB_DEBUGADDMAX {}
#[doc = "`write(|w| ..)` method takes [bb_debugaddmax::W](bb_debugaddmax::W) writer structure"]
impl crate::Writable for BB_DEBUGADDMAX {}
#[doc = "Diagnostic ports upper limit"]
pub mod bb_debugaddmax;
#[doc = "Diagnostic ports lower limit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_debugaddmin](bb_debugaddmin) module"]
pub type BB_DEBUGADDMIN = crate::Reg<u32, _BB_DEBUGADDMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_DEBUGADDMIN;
#[doc = "`read()` method returns [bb_debugaddmin::R](bb_debugaddmin::R) reader structure"]
impl crate::Readable for BB_DEBUGADDMIN {}
#[doc = "`write(|w| ..)` method takes [bb_debugaddmin::W](bb_debugaddmin::W) writer structure"]
impl crate::Writable for BB_DEBUGADDMIN {}
#[doc = "Diagnostic ports lower limit"]
pub mod bb_debugaddmin;
#[doc = "Diagnostic ports errors register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_errortypestat](bb_errortypestat) module"]
pub type BB_ERRORTYPESTAT = crate::Reg<u32, _BB_ERRORTYPESTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ERRORTYPESTAT;
#[doc = "`read()` method returns [bb_errortypestat::R](bb_errortypestat::R) reader structure"]
impl crate::Readable for BB_ERRORTYPESTAT {}
#[doc = "Diagnostic ports errors register"]
pub mod bb_errortypestat;
#[doc = "Software profiling register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_swprofiling](bb_swprofiling) module"]
pub type BB_SWPROFILING = crate::Reg<u32, _BB_SWPROFILING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_SWPROFILING;
#[doc = "`read()` method returns [bb_swprofiling::R](bb_swprofiling::R) reader structure"]
impl crate::Readable for BB_SWPROFILING {}
#[doc = "`write(|w| ..)` method takes [bb_swprofiling::W](bb_swprofiling::W) writer structure"]
impl crate::Writable for BB_SWPROFILING {}
#[doc = "Software profiling register"]
pub mod bb_swprofiling;
#[doc = "Principal control register for the radio interface\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_radiocntl0](bb_radiocntl0) module"]
pub type BB_RADIOCNTL0 = crate::Reg<u32, _BB_RADIOCNTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_RADIOCNTL0;
#[doc = "`read()` method returns [bb_radiocntl0::R](bb_radiocntl0::R) reader structure"]
impl crate::Readable for BB_RADIOCNTL0 {}
#[doc = "`write(|w| ..)` method takes [bb_radiocntl0::W](bb_radiocntl0::W) writer structure"]
impl crate::Writable for BB_RADIOCNTL0 {}
#[doc = "Principal control register for the radio interface"]
pub mod bb_radiocntl0;
#[doc = "Second control register for the radio interface\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_radiocntl1](bb_radiocntl1) module"]
pub type BB_RADIOCNTL1 = crate::Reg<u32, _BB_RADIOCNTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_RADIOCNTL1;
#[doc = "`read()` method returns [bb_radiocntl1::R](bb_radiocntl1::R) reader structure"]
impl crate::Readable for BB_RADIOCNTL1 {}
#[doc = "`write(|w| ..)` method takes [bb_radiocntl1::W](bb_radiocntl1::W) writer structure"]
impl crate::Writable for BB_RADIOCNTL1 {}
#[doc = "Second control register for the radio interface"]
pub mod bb_radiocntl1;
#[doc = "Third control register for the radio interface\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_radiocntl2](bb_radiocntl2) module"]
pub type BB_RADIOCNTL2 = crate::Reg<u32, _BB_RADIOCNTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_RADIOCNTL2;
#[doc = "`read()` method returns [bb_radiocntl2::R](bb_radiocntl2::R) reader structure"]
impl crate::Readable for BB_RADIOCNTL2 {}
#[doc = "`write(|w| ..)` method takes [bb_radiocntl2::W](bb_radiocntl2::W) writer structure"]
impl crate::Writable for BB_RADIOCNTL2 {}
#[doc = "Third control register for the radio interface"]
pub mod bb_radiocntl2;
#[doc = "Principal control register for the radio interface power up/down delays\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_radiopwrupdn0](bb_radiopwrupdn0) module"]
pub type BB_RADIOPWRUPDN0 = crate::Reg<u32, _BB_RADIOPWRUPDN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_RADIOPWRUPDN0;
#[doc = "`read()` method returns [bb_radiopwrupdn0::R](bb_radiopwrupdn0::R) reader structure"]
impl crate::Readable for BB_RADIOPWRUPDN0 {}
#[doc = "`write(|w| ..)` method takes [bb_radiopwrupdn0::W](bb_radiopwrupdn0::W) writer structure"]
impl crate::Writable for BB_RADIOPWRUPDN0 {}
#[doc = "Principal control register for the radio interface power up/down delays"]
pub mod bb_radiopwrupdn0;
#[doc = "Second control register for the radio interface power up/down delays\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_radiopwrupdn1](bb_radiopwrupdn1) module"]
pub type BB_RADIOPWRUPDN1 = crate::Reg<u32, _BB_RADIOPWRUPDN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_RADIOPWRUPDN1;
#[doc = "`read()` method returns [bb_radiopwrupdn1::R](bb_radiopwrupdn1::R) reader structure"]
impl crate::Readable for BB_RADIOPWRUPDN1 {}
#[doc = "`write(|w| ..)` method takes [bb_radiopwrupdn1::W](bb_radiopwrupdn1::W) writer structure"]
impl crate::Writable for BB_RADIOPWRUPDN1 {}
#[doc = "Second control register for the radio interface power up/down delays"]
pub mod bb_radiopwrupdn1;
#[doc = "Principal control register for the radio interface timing compensation delays\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_radiotxrxtim0](bb_radiotxrxtim0) module"]
pub type BB_RADIOTXRXTIM0 = crate::Reg<u32, _BB_RADIOTXRXTIM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_RADIOTXRXTIM0;
#[doc = "`read()` method returns [bb_radiotxrxtim0::R](bb_radiotxrxtim0::R) reader structure"]
impl crate::Readable for BB_RADIOTXRXTIM0 {}
#[doc = "`write(|w| ..)` method takes [bb_radiotxrxtim0::W](bb_radiotxrxtim0::W) writer structure"]
impl crate::Writable for BB_RADIOTXRXTIM0 {}
#[doc = "Principal control register for the radio interface timing compensation delays"]
pub mod bb_radiotxrxtim0;
#[doc = "Second control register for the radio interface timing compensation delays\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_radiotxrxtim1](bb_radiotxrxtim1) module"]
pub type BB_RADIOTXRXTIM1 = crate::Reg<u32, _BB_RADIOTXRXTIM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_RADIOTXRXTIM1;
#[doc = "`read()` method returns [bb_radiotxrxtim1::R](bb_radiotxrxtim1::R) reader structure"]
impl crate::Readable for BB_RADIOTXRXTIM1 {}
#[doc = "`write(|w| ..)` method takes [bb_radiotxrxtim1::W](bb_radiotxrxtim1::W) writer structure"]
impl crate::Writable for BB_RADIOTXRXTIM1 {}
#[doc = "Second control register for the radio interface timing compensation delays"]
pub mod bb_radiotxrxtim1;
#[doc = "First control register for the radio interface SPI pointers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_spiptrcntl0](bb_spiptrcntl0) module"]
pub type BB_SPIPTRCNTL0 = crate::Reg<u32, _BB_SPIPTRCNTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_SPIPTRCNTL0;
#[doc = "`read()` method returns [bb_spiptrcntl0::R](bb_spiptrcntl0::R) reader structure"]
impl crate::Readable for BB_SPIPTRCNTL0 {}
#[doc = "`write(|w| ..)` method takes [bb_spiptrcntl0::W](bb_spiptrcntl0::W) writer structure"]
impl crate::Writable for BB_SPIPTRCNTL0 {}
#[doc = "First control register for the radio interface SPI pointers"]
pub mod bb_spiptrcntl0;
#[doc = "Second control register for the radio interface SPI pointers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_spiptrcntl1](bb_spiptrcntl1) module"]
pub type BB_SPIPTRCNTL1 = crate::Reg<u32, _BB_SPIPTRCNTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_SPIPTRCNTL1;
#[doc = "`read()` method returns [bb_spiptrcntl1::R](bb_spiptrcntl1::R) reader structure"]
impl crate::Readable for BB_SPIPTRCNTL1 {}
#[doc = "`write(|w| ..)` method takes [bb_spiptrcntl1::W](bb_spiptrcntl1::W) writer structure"]
impl crate::Writable for BB_SPIPTRCNTL1 {}
#[doc = "Second control register for the radio interface SPI pointers"]
pub mod bb_spiptrcntl1;
#[doc = "Third control register for the radio interface SPI pointers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_spiptrcntl2](bb_spiptrcntl2) module"]
pub type BB_SPIPTRCNTL2 = crate::Reg<u32, _BB_SPIPTRCNTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_SPIPTRCNTL2;
#[doc = "`read()` method returns [bb_spiptrcntl2::R](bb_spiptrcntl2::R) reader structure"]
impl crate::Readable for BB_SPIPTRCNTL2 {}
#[doc = "`write(|w| ..)` method takes [bb_spiptrcntl2::W](bb_spiptrcntl2::W) writer structure"]
impl crate::Writable for BB_SPIPTRCNTL2 {}
#[doc = "Third control register for the radio interface SPI pointers"]
pub mod bb_spiptrcntl2;
#[doc = "Advertising channel map register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_advchmap](bb_advchmap) module"]
pub type BB_ADVCHMAP = crate::Reg<u32, _BB_ADVCHMAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ADVCHMAP;
#[doc = "`read()` method returns [bb_advchmap::R](bb_advchmap::R) reader structure"]
impl crate::Readable for BB_ADVCHMAP {}
#[doc = "`write(|w| ..)` method takes [bb_advchmap::W](bb_advchmap::W) writer structure"]
impl crate::Writable for BB_ADVCHMAP {}
#[doc = "Advertising channel map register"]
pub mod bb_advchmap;
#[doc = "Delay information register handling advertising event timers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_advtim](bb_advtim) module"]
pub type BB_ADVTIM = crate::Reg<u32, _BB_ADVTIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ADVTIM;
#[doc = "`read()` method returns [bb_advtim::R](bb_advtim::R) reader structure"]
impl crate::Readable for BB_ADVTIM {}
#[doc = "`write(|w| ..)` method takes [bb_advtim::W](bb_advtim::W) writer structure"]
impl crate::Writable for BB_ADVTIM {}
#[doc = "Delay information register handling advertising event timers"]
pub mod bb_advtim;
#[doc = "Active scan mode control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_actscanstat](bb_actscanstat) module"]
pub type BB_ACTSCANSTAT = crate::Reg<u32, _BB_ACTSCANSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ACTSCANSTAT;
#[doc = "`read()` method returns [bb_actscanstat::R](bb_actscanstat::R) reader structure"]
impl crate::Readable for BB_ACTSCANSTAT {}
#[doc = "Active scan mode control register"]
pub mod bb_actscanstat;
#[doc = "Address pointer of public devices\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_wlpubaddptr](bb_wlpubaddptr) module"]
pub type BB_WLPUBADDPTR = crate::Reg<u32, _BB_WLPUBADDPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_WLPUBADDPTR;
#[doc = "`read()` method returns [bb_wlpubaddptr::R](bb_wlpubaddptr::R) reader structure"]
impl crate::Readable for BB_WLPUBADDPTR {}
#[doc = "`write(|w| ..)` method takes [bb_wlpubaddptr::W](bb_wlpubaddptr::W) writer structure"]
impl crate::Writable for BB_WLPUBADDPTR {}
#[doc = "Address pointer of public devices"]
pub mod bb_wlpubaddptr;
#[doc = "Address pointer of private devices\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_wlprivaddptr](bb_wlprivaddptr) module"]
pub type BB_WLPRIVADDPTR = crate::Reg<u32, _BB_WLPRIVADDPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_WLPRIVADDPTR;
#[doc = "`read()` method returns [bb_wlprivaddptr::R](bb_wlprivaddptr::R) reader structure"]
impl crate::Readable for BB_WLPRIVADDPTR {}
#[doc = "`write(|w| ..)` method takes [bb_wlprivaddptr::W](bb_wlprivaddptr::W) writer structure"]
impl crate::Writable for BB_WLPRIVADDPTR {}
#[doc = "Address pointer of private devices"]
pub mod bb_wlprivaddptr;
#[doc = "Devices in white list\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_wlnbdev](bb_wlnbdev) module"]
pub type BB_WLNBDEV = crate::Reg<u32, _BB_WLNBDEV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_WLNBDEV;
#[doc = "`read()` method returns [bb_wlnbdev::R](bb_wlnbdev::R) reader structure"]
impl crate::Readable for BB_WLNBDEV {}
#[doc = "`write(|w| ..)` method takes [bb_wlnbdev::W](bb_wlnbdev::W) writer structure"]
impl crate::Writable for BB_WLNBDEV {}
#[doc = "Devices in white list"]
pub mod bb_wlnbdev;
#[doc = "AES-128 ciphering control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_aescntl](bb_aescntl) module"]
pub type BB_AESCNTL = crate::Reg<u32, _BB_AESCNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_AESCNTL;
#[doc = "`read()` method returns [bb_aescntl::R](bb_aescntl::R) reader structure"]
impl crate::Readable for BB_AESCNTL {}
#[doc = "`write(|w| ..)` method takes [bb_aescntl::W](bb_aescntl::W) writer structure"]
impl crate::Writable for BB_AESCNTL {}
#[doc = "AES-128 ciphering control register"]
pub mod bb_aescntl;
#[doc = "AES encryption 128-bit key register (bits 31:0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_aeskey31_0](bb_aeskey31_0) module"]
pub type BB_AESKEY31_0 = crate::Reg<u32, _BB_AESKEY31_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_AESKEY31_0;
#[doc = "`read()` method returns [bb_aeskey31_0::R](bb_aeskey31_0::R) reader structure"]
impl crate::Readable for BB_AESKEY31_0 {}
#[doc = "`write(|w| ..)` method takes [bb_aeskey31_0::W](bb_aeskey31_0::W) writer structure"]
impl crate::Writable for BB_AESKEY31_0 {}
#[doc = "AES encryption 128-bit key register (bits 31:0)"]
pub mod bb_aeskey31_0;
#[doc = "AES encryption 128-bit key register (bits 63:32)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_aeskey63_32](bb_aeskey63_32) module"]
pub type BB_AESKEY63_32 = crate::Reg<u32, _BB_AESKEY63_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_AESKEY63_32;
#[doc = "`read()` method returns [bb_aeskey63_32::R](bb_aeskey63_32::R) reader structure"]
impl crate::Readable for BB_AESKEY63_32 {}
#[doc = "`write(|w| ..)` method takes [bb_aeskey63_32::W](bb_aeskey63_32::W) writer structure"]
impl crate::Writable for BB_AESKEY63_32 {}
#[doc = "AES encryption 128-bit key register (bits 63:32)"]
pub mod bb_aeskey63_32;
#[doc = "AES encryption 128-bit key register (bits 95:64)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_aeskey95_64](bb_aeskey95_64) module"]
pub type BB_AESKEY95_64 = crate::Reg<u32, _BB_AESKEY95_64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_AESKEY95_64;
#[doc = "`read()` method returns [bb_aeskey95_64::R](bb_aeskey95_64::R) reader structure"]
impl crate::Readable for BB_AESKEY95_64 {}
#[doc = "`write(|w| ..)` method takes [bb_aeskey95_64::W](bb_aeskey95_64::W) writer structure"]
impl crate::Writable for BB_AESKEY95_64 {}
#[doc = "AES encryption 128-bit key register (bits 95:64)"]
pub mod bb_aeskey95_64;
#[doc = "AES encryption 128-bit key register (bits 127:96)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_aeskey127_96](bb_aeskey127_96) module"]
pub type BB_AESKEY127_96 = crate::Reg<u32, _BB_AESKEY127_96>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_AESKEY127_96;
#[doc = "`read()` method returns [bb_aeskey127_96::R](bb_aeskey127_96::R) reader structure"]
impl crate::Readable for BB_AESKEY127_96 {}
#[doc = "`write(|w| ..)` method takes [bb_aeskey127_96::W](bb_aeskey127_96::W) writer structure"]
impl crate::Writable for BB_AESKEY127_96 {}
#[doc = "AES encryption 128-bit key register (bits 127:96)"]
pub mod bb_aeskey127_96;
#[doc = "AES memory zone pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_aesptr](bb_aesptr) module"]
pub type BB_AESPTR = crate::Reg<u32, _BB_AESPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_AESPTR;
#[doc = "`read()` method returns [bb_aesptr::R](bb_aesptr::R) reader structure"]
impl crate::Readable for BB_AESPTR {}
#[doc = "`write(|w| ..)` method takes [bb_aesptr::W](bb_aesptr::W) writer structure"]
impl crate::Writable for BB_AESPTR {}
#[doc = "AES memory zone pointer"]
pub mod bb_aesptr;
#[doc = "AES-CCM plain MIC value register in Tx\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_txmicval](bb_txmicval) module"]
pub type BB_TXMICVAL = crate::Reg<u32, _BB_TXMICVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_TXMICVAL;
#[doc = "`read()` method returns [bb_txmicval::R](bb_txmicval::R) reader structure"]
impl crate::Readable for BB_TXMICVAL {}
#[doc = "AES-CCM plain MIC value register in Tx"]
pub mod bb_txmicval;
#[doc = "AES-CCM plain MIC value register in Rx\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_rxmicval](bb_rxmicval) module"]
pub type BB_RXMICVAL = crate::Reg<u32, _BB_RXMICVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_RXMICVAL;
#[doc = "`read()` method returns [bb_rxmicval::R](bb_rxmicval::R) reader structure"]
impl crate::Readable for BB_RXMICVAL {}
#[doc = "AES-CCM plain MIC value register in Rx"]
pub mod bb_rxmicval;
#[doc = "RF testing and regulatory body support register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_rftestcntl](bb_rftestcntl) module"]
pub type BB_RFTESTCNTL = crate::Reg<u32, _BB_RFTESTCNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_RFTESTCNTL;
#[doc = "`read()` method returns [bb_rftestcntl::R](bb_rftestcntl::R) reader structure"]
impl crate::Readable for BB_RFTESTCNTL {}
#[doc = "`write(|w| ..)` method takes [bb_rftestcntl::W](bb_rftestcntl::W) writer structure"]
impl crate::Writable for BB_RFTESTCNTL {}
#[doc = "RF testing and regulatory body support register"]
pub mod bb_rftestcntl;
#[doc = "Number of transmitted packet during test modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_rftesttxstat](bb_rftesttxstat) module"]
pub type BB_RFTESTTXSTAT = crate::Reg<u32, _BB_RFTESTTXSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_RFTESTTXSTAT;
#[doc = "`read()` method returns [bb_rftesttxstat::R](bb_rftesttxstat::R) reader structure"]
impl crate::Readable for BB_RFTESTTXSTAT {}
#[doc = "`write(|w| ..)` method takes [bb_rftesttxstat::W](bb_rftesttxstat::W) writer structure"]
impl crate::Writable for BB_RFTESTTXSTAT {}
#[doc = "Number of transmitted packet during test modes"]
pub mod bb_rftesttxstat;
#[doc = "Number of correctly received packet during test modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_rftestrxstat](bb_rftestrxstat) module"]
pub type BB_RFTESTRXSTAT = crate::Reg<u32, _BB_RFTESTRXSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_RFTESTRXSTAT;
#[doc = "`read()` method returns [bb_rftestrxstat::R](bb_rftestrxstat::R) reader structure"]
impl crate::Readable for BB_RFTESTRXSTAT {}
#[doc = "`write(|w| ..)` method takes [bb_rftestrxstat::W](bb_rftestrxstat::W) writer structure"]
impl crate::Writable for BB_RFTESTRXSTAT {}
#[doc = "Number of correctly received packet during test modes"]
pub mod bb_rftestrxstat;
#[doc = "Timing generator control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_timgencntl](bb_timgencntl) module"]
pub type BB_TIMGENCNTL = crate::Reg<u32, _BB_TIMGENCNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_TIMGENCNTL;
#[doc = "`read()` method returns [bb_timgencntl::R](bb_timgencntl::R) reader structure"]
impl crate::Readable for BB_TIMGENCNTL {}
#[doc = "`write(|w| ..)` method takes [bb_timgencntl::W](bb_timgencntl::W) writer structure"]
impl crate::Writable for BB_TIMGENCNTL {}
#[doc = "Timing generator control register"]
pub mod bb_timgencntl;
#[doc = "Gross timer control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_grosstimtgt](bb_grosstimtgt) module"]
pub type BB_GROSSTIMTGT = crate::Reg<u32, _BB_GROSSTIMTGT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_GROSSTIMTGT;
#[doc = "`read()` method returns [bb_grosstimtgt::R](bb_grosstimtgt::R) reader structure"]
impl crate::Readable for BB_GROSSTIMTGT {}
#[doc = "`write(|w| ..)` method takes [bb_grosstimtgt::W](bb_grosstimtgt::W) writer structure"]
impl crate::Writable for BB_GROSSTIMTGT {}
#[doc = "Gross timer control register"]
pub mod bb_grosstimtgt;
#[doc = "Fine timer control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_finetimtgt](bb_finetimtgt) module"]
pub type BB_FINETIMTGT = crate::Reg<u32, _BB_FINETIMTGT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_FINETIMTGT;
#[doc = "`read()` method returns [bb_finetimtgt::R](bb_finetimtgt::R) reader structure"]
impl crate::Readable for BB_FINETIMTGT {}
#[doc = "`write(|w| ..)` method takes [bb_finetimtgt::W](bb_finetimtgt::W) writer structure"]
impl crate::Writable for BB_FINETIMTGT {}
#[doc = "Fine timer control register"]
pub mod bb_finetimtgt;
#[doc = "Coexistence control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_coexifcntl0](bb_coexifcntl0) module"]
pub type BB_COEXIFCNTL0 = crate::Reg<u32, _BB_COEXIFCNTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_COEXIFCNTL0;
#[doc = "`read()` method returns [bb_coexifcntl0::R](bb_coexifcntl0::R) reader structure"]
impl crate::Readable for BB_COEXIFCNTL0 {}
#[doc = "`write(|w| ..)` method takes [bb_coexifcntl0::W](bb_coexifcntl0::W) writer structure"]
impl crate::Writable for BB_COEXIFCNTL0 {}
#[doc = "Coexistence control register 0"]
pub mod bb_coexifcntl0;
#[doc = "Coexistence control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_coexifcntl1](bb_coexifcntl1) module"]
pub type BB_COEXIFCNTL1 = crate::Reg<u32, _BB_COEXIFCNTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_COEXIFCNTL1;
#[doc = "`read()` method returns [bb_coexifcntl1::R](bb_coexifcntl1::R) reader structure"]
impl crate::Readable for BB_COEXIFCNTL1 {}
#[doc = "`write(|w| ..)` method takes [bb_coexifcntl1::W](bb_coexifcntl1::W) writer structure"]
impl crate::Writable for BB_COEXIFCNTL1 {}
#[doc = "Coexistence control register 1"]
pub mod bb_coexifcntl1;
#[doc = "Coexistence control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_coexifcntl2](bb_coexifcntl2) module"]
pub type BB_COEXIFCNTL2 = crate::Reg<u32, _BB_COEXIFCNTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_COEXIFCNTL2;
#[doc = "`read()` method returns [bb_coexifcntl2::R](bb_coexifcntl2::R) reader structure"]
impl crate::Readable for BB_COEXIFCNTL2 {}
#[doc = "`write(|w| ..)` method takes [bb_coexifcntl2::W](bb_coexifcntl2::W) writer structure"]
impl crate::Writable for BB_COEXIFCNTL2 {}
#[doc = "Coexistence control register 2"]
pub mod bb_coexifcntl2;
#[doc = "Priority control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_bbmprio0](bb_bbmprio0) module"]
pub type BB_BBMPRIO0 = crate::Reg<u32, _BB_BBMPRIO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_BBMPRIO0;
#[doc = "`read()` method returns [bb_bbmprio0::R](bb_bbmprio0::R) reader structure"]
impl crate::Readable for BB_BBMPRIO0 {}
#[doc = "`write(|w| ..)` method takes [bb_bbmprio0::W](bb_bbmprio0::W) writer structure"]
impl crate::Writable for BB_BBMPRIO0 {}
#[doc = "Priority control register 0"]
pub mod bb_bbmprio0;
#[doc = "Priority control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_bbmprio1](bb_bbmprio1) module"]
pub type BB_BBMPRIO1 = crate::Reg<u32, _BB_BBMPRIO1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_BBMPRIO1;
#[doc = "`read()` method returns [bb_bbmprio1::R](bb_bbmprio1::R) reader structure"]
impl crate::Readable for BB_BBMPRIO1 {}
#[doc = "`write(|w| ..)` method takes [bb_bbmprio1::W](bb_bbmprio1::W) writer structure"]
impl crate::Writable for BB_BBMPRIO1 {}
#[doc = "Priority control register 1"]
pub mod bb_bbmprio1;
#[doc = "Register used by the Resolving Address List engine\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_ralptr](bb_ralptr) module"]
pub type BB_RALPTR = crate::Reg<u32, _BB_RALPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_RALPTR;
#[doc = "`read()` method returns [bb_ralptr::R](bb_ralptr::R) reader structure"]
impl crate::Readable for BB_RALPTR {}
#[doc = "`write(|w| ..)` method takes [bb_ralptr::W](bb_ralptr::W) writer structure"]
impl crate::Writable for BB_RALPTR {}
#[doc = "Register used by the Resolving Address List engine"]
pub mod bb_ralptr;
#[doc = "Register used by the Resolving Address List engine\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_ralnbdev](bb_ralnbdev) module"]
pub type BB_RALNBDEV = crate::Reg<u32, _BB_RALNBDEV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_RALNBDEV;
#[doc = "`read()` method returns [bb_ralnbdev::R](bb_ralnbdev::R) reader structure"]
impl crate::Readable for BB_RALNBDEV {}
#[doc = "`write(|w| ..)` method takes [bb_ralnbdev::W](bb_ralnbdev::W) writer structure"]
impl crate::Writable for BB_RALNBDEV {}
#[doc = "Register used by the Resolving Address List engine"]
pub mod bb_ralnbdev;
#[doc = "Register used by the Resolving Address List engine\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_ral_local_rnd](bb_ral_local_rnd) module"]
pub type BB_RAL_LOCAL_RND = crate::Reg<u32, _BB_RAL_LOCAL_RND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_RAL_LOCAL_RND;
#[doc = "`read()` method returns [bb_ral_local_rnd::R](bb_ral_local_rnd::R) reader structure"]
impl crate::Readable for BB_RAL_LOCAL_RND {}
#[doc = "`write(|w| ..)` method takes [bb_ral_local_rnd::W](bb_ral_local_rnd::W) writer structure"]
impl crate::Writable for BB_RAL_LOCAL_RND {}
#[doc = "Register used by the Resolving Address List engine"]
pub mod bb_ral_local_rnd;
#[doc = "Register used by the Resolving Address List engine\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_ral_peer_rnd](bb_ral_peer_rnd) module"]
pub type BB_RAL_PEER_RND = crate::Reg<u32, _BB_RAL_PEER_RND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_RAL_PEER_RND;
#[doc = "`read()` method returns [bb_ral_peer_rnd::R](bb_ral_peer_rnd::R) reader structure"]
impl crate::Readable for BB_RAL_PEER_RND {}
#[doc = "`write(|w| ..)` method takes [bb_ral_peer_rnd::W](bb_ral_peer_rnd::W) writer structure"]
impl crate::Writable for BB_RAL_PEER_RND {}
#[doc = "Register used by the Resolving Address List engine"]
pub mod bb_ral_peer_rnd;
#[doc = "ISO Channel 0 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isochancntl0](bb_isochancntl0) module"]
pub type BB_ISOCHANCNTL0 = crate::Reg<u32, _BB_ISOCHANCNTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOCHANCNTL0;
#[doc = "`read()` method returns [bb_isochancntl0::R](bb_isochancntl0::R) reader structure"]
impl crate::Readable for BB_ISOCHANCNTL0 {}
#[doc = "`write(|w| ..)` method takes [bb_isochancntl0::W](bb_isochancntl0::W) writer structure"]
impl crate::Writable for BB_ISOCHANCNTL0 {}
#[doc = "ISO Channel 0 control"]
pub mod bb_isochancntl0;
#[doc = "ISO Channel 0 mute control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isomutecntl0](bb_isomutecntl0) module"]
pub type BB_ISOMUTECNTL0 = crate::Reg<u32, _BB_ISOMUTECNTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOMUTECNTL0;
#[doc = "`read()` method returns [bb_isomutecntl0::R](bb_isomutecntl0::R) reader structure"]
impl crate::Readable for BB_ISOMUTECNTL0 {}
#[doc = "`write(|w| ..)` method takes [bb_isomutecntl0::W](bb_isomutecntl0::W) writer structure"]
impl crate::Writable for BB_ISOMUTECNTL0 {}
#[doc = "ISO Channel 0 mute control"]
pub mod bb_isomutecntl0;
#[doc = "ISO Channel 0 current Tx pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isocurrenttxptr0](bb_isocurrenttxptr0) module"]
pub type BB_ISOCURRENTTXPTR0 = crate::Reg<u32, _BB_ISOCURRENTTXPTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOCURRENTTXPTR0;
#[doc = "`read()` method returns [bb_isocurrenttxptr0::R](bb_isocurrenttxptr0::R) reader structure"]
impl crate::Readable for BB_ISOCURRENTTXPTR0 {}
#[doc = "`write(|w| ..)` method takes [bb_isocurrenttxptr0::W](bb_isocurrenttxptr0::W) writer structure"]
impl crate::Writable for BB_ISOCURRENTTXPTR0 {}
#[doc = "ISO Channel 0 current Tx pointer"]
pub mod bb_isocurrenttxptr0;
#[doc = "ISO Channel 0 current Rx pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isocurrentrxptr0](bb_isocurrentrxptr0) module"]
pub type BB_ISOCURRENTRXPTR0 = crate::Reg<u32, _BB_ISOCURRENTRXPTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOCURRENTRXPTR0;
#[doc = "`read()` method returns [bb_isocurrentrxptr0::R](bb_isocurrentrxptr0::R) reader structure"]
impl crate::Readable for BB_ISOCURRENTRXPTR0 {}
#[doc = "`write(|w| ..)` method takes [bb_isocurrentrxptr0::W](bb_isocurrentrxptr0::W) writer structure"]
impl crate::Writable for BB_ISOCURRENTRXPTR0 {}
#[doc = "ISO Channel 0 current Rx pointer"]
pub mod bb_isocurrentrxptr0;
#[doc = "ISO Channel 0 payloads\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isotrcnl0](bb_isotrcnl0) module"]
pub type BB_ISOTRCNL0 = crate::Reg<u32, _BB_ISOTRCNL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOTRCNL0;
#[doc = "`read()` method returns [bb_isotrcnl0::R](bb_isotrcnl0::R) reader structure"]
impl crate::Readable for BB_ISOTRCNL0 {}
#[doc = "`write(|w| ..)` method takes [bb_isotrcnl0::W](bb_isotrcnl0::W) writer structure"]
impl crate::Writable for BB_ISOTRCNL0 {}
#[doc = "ISO Channel 0 payloads"]
pub mod bb_isotrcnl0;
#[doc = "ISO Channel 0 mute control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isoevtcntloffsetl0](bb_isoevtcntloffsetl0) module"]
pub type BB_ISOEVTCNTLOFFSETL0 = crate::Reg<u32, _BB_ISOEVTCNTLOFFSETL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOEVTCNTLOFFSETL0;
#[doc = "`read()` method returns [bb_isoevtcntloffsetl0::R](bb_isoevtcntloffsetl0::R) reader structure"]
impl crate::Readable for BB_ISOEVTCNTLOFFSETL0 {}
#[doc = "`write(|w| ..)` method takes [bb_isoevtcntloffsetl0::W](bb_isoevtcntloffsetl0::W) writer structure"]
impl crate::Writable for BB_ISOEVTCNTLOFFSETL0 {}
#[doc = "ISO Channel 0 mute control"]
pub mod bb_isoevtcntloffsetl0;
#[doc = "ISO Channel 0 mute control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isoevtcntloffsetu0](bb_isoevtcntloffsetu0) module"]
pub type BB_ISOEVTCNTLOFFSETU0 = crate::Reg<u32, _BB_ISOEVTCNTLOFFSETU0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOEVTCNTLOFFSETU0;
#[doc = "`read()` method returns [bb_isoevtcntloffsetu0::R](bb_isoevtcntloffsetu0::R) reader structure"]
impl crate::Readable for BB_ISOEVTCNTLOFFSETU0 {}
#[doc = "`write(|w| ..)` method takes [bb_isoevtcntloffsetu0::W](bb_isoevtcntloffsetu0::W) writer structure"]
impl crate::Writable for BB_ISOEVTCNTLOFFSETU0 {}
#[doc = "ISO Channel 0 mute control"]
pub mod bb_isoevtcntloffsetu0;
#[doc = "ISO Channel 1 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isochancntl1](bb_isochancntl1) module"]
pub type BB_ISOCHANCNTL1 = crate::Reg<u32, _BB_ISOCHANCNTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOCHANCNTL1;
#[doc = "`read()` method returns [bb_isochancntl1::R](bb_isochancntl1::R) reader structure"]
impl crate::Readable for BB_ISOCHANCNTL1 {}
#[doc = "`write(|w| ..)` method takes [bb_isochancntl1::W](bb_isochancntl1::W) writer structure"]
impl crate::Writable for BB_ISOCHANCNTL1 {}
#[doc = "ISO Channel 1 control"]
pub mod bb_isochancntl1;
#[doc = "ISO Channel 1 mute control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isomutecntl1](bb_isomutecntl1) module"]
pub type BB_ISOMUTECNTL1 = crate::Reg<u32, _BB_ISOMUTECNTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOMUTECNTL1;
#[doc = "`read()` method returns [bb_isomutecntl1::R](bb_isomutecntl1::R) reader structure"]
impl crate::Readable for BB_ISOMUTECNTL1 {}
#[doc = "`write(|w| ..)` method takes [bb_isomutecntl1::W](bb_isomutecntl1::W) writer structure"]
impl crate::Writable for BB_ISOMUTECNTL1 {}
#[doc = "ISO Channel 1 mute control"]
pub mod bb_isomutecntl1;
#[doc = "ISO Channel 1 current Tx pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isocurrenttxptr1](bb_isocurrenttxptr1) module"]
pub type BB_ISOCURRENTTXPTR1 = crate::Reg<u32, _BB_ISOCURRENTTXPTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOCURRENTTXPTR1;
#[doc = "`read()` method returns [bb_isocurrenttxptr1::R](bb_isocurrenttxptr1::R) reader structure"]
impl crate::Readable for BB_ISOCURRENTTXPTR1 {}
#[doc = "`write(|w| ..)` method takes [bb_isocurrenttxptr1::W](bb_isocurrenttxptr1::W) writer structure"]
impl crate::Writable for BB_ISOCURRENTTXPTR1 {}
#[doc = "ISO Channel 1 current Tx pointer"]
pub mod bb_isocurrenttxptr1;
#[doc = "ISO Channel 1 current Rx pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isocurrentrxptr1](bb_isocurrentrxptr1) module"]
pub type BB_ISOCURRENTRXPTR1 = crate::Reg<u32, _BB_ISOCURRENTRXPTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOCURRENTRXPTR1;
#[doc = "`read()` method returns [bb_isocurrentrxptr1::R](bb_isocurrentrxptr1::R) reader structure"]
impl crate::Readable for BB_ISOCURRENTRXPTR1 {}
#[doc = "`write(|w| ..)` method takes [bb_isocurrentrxptr1::W](bb_isocurrentrxptr1::W) writer structure"]
impl crate::Writable for BB_ISOCURRENTRXPTR1 {}
#[doc = "ISO Channel 1 current Rx pointer"]
pub mod bb_isocurrentrxptr1;
#[doc = "ISO Channel 1 payloads\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isotrcnl1](bb_isotrcnl1) module"]
pub type BB_ISOTRCNL1 = crate::Reg<u32, _BB_ISOTRCNL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOTRCNL1;
#[doc = "`read()` method returns [bb_isotrcnl1::R](bb_isotrcnl1::R) reader structure"]
impl crate::Readable for BB_ISOTRCNL1 {}
#[doc = "`write(|w| ..)` method takes [bb_isotrcnl1::W](bb_isotrcnl1::W) writer structure"]
impl crate::Writable for BB_ISOTRCNL1 {}
#[doc = "ISO Channel 1 payloads"]
pub mod bb_isotrcnl1;
#[doc = "ISO Channel 1 mute control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isoevtcntloffsetl1](bb_isoevtcntloffsetl1) module"]
pub type BB_ISOEVTCNTLOFFSETL1 = crate::Reg<u32, _BB_ISOEVTCNTLOFFSETL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOEVTCNTLOFFSETL1;
#[doc = "`read()` method returns [bb_isoevtcntloffsetl1::R](bb_isoevtcntloffsetl1::R) reader structure"]
impl crate::Readable for BB_ISOEVTCNTLOFFSETL1 {}
#[doc = "`write(|w| ..)` method takes [bb_isoevtcntloffsetl1::W](bb_isoevtcntloffsetl1::W) writer structure"]
impl crate::Writable for BB_ISOEVTCNTLOFFSETL1 {}
#[doc = "ISO Channel 1 mute control"]
pub mod bb_isoevtcntloffsetl1;
#[doc = "ISO Channel 1 mute control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isoevtcntloffsetu1](bb_isoevtcntloffsetu1) module"]
pub type BB_ISOEVTCNTLOFFSETU1 = crate::Reg<u32, _BB_ISOEVTCNTLOFFSETU1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOEVTCNTLOFFSETU1;
#[doc = "`read()` method returns [bb_isoevtcntloffsetu1::R](bb_isoevtcntloffsetu1::R) reader structure"]
impl crate::Readable for BB_ISOEVTCNTLOFFSETU1 {}
#[doc = "`write(|w| ..)` method takes [bb_isoevtcntloffsetu1::W](bb_isoevtcntloffsetu1::W) writer structure"]
impl crate::Writable for BB_ISOEVTCNTLOFFSETU1 {}
#[doc = "ISO Channel 1 mute control"]
pub mod bb_isoevtcntloffsetu1;
#[doc = "ISO Channel 2 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isochancntl2](bb_isochancntl2) module"]
pub type BB_ISOCHANCNTL2 = crate::Reg<u32, _BB_ISOCHANCNTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOCHANCNTL2;
#[doc = "`read()` method returns [bb_isochancntl2::R](bb_isochancntl2::R) reader structure"]
impl crate::Readable for BB_ISOCHANCNTL2 {}
#[doc = "`write(|w| ..)` method takes [bb_isochancntl2::W](bb_isochancntl2::W) writer structure"]
impl crate::Writable for BB_ISOCHANCNTL2 {}
#[doc = "ISO Channel 2 control"]
pub mod bb_isochancntl2;
#[doc = "ISO Channel 2 mute control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isomutecntl2](bb_isomutecntl2) module"]
pub type BB_ISOMUTECNTL2 = crate::Reg<u32, _BB_ISOMUTECNTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOMUTECNTL2;
#[doc = "`read()` method returns [bb_isomutecntl2::R](bb_isomutecntl2::R) reader structure"]
impl crate::Readable for BB_ISOMUTECNTL2 {}
#[doc = "`write(|w| ..)` method takes [bb_isomutecntl2::W](bb_isomutecntl2::W) writer structure"]
impl crate::Writable for BB_ISOMUTECNTL2 {}
#[doc = "ISO Channel 2 mute control"]
pub mod bb_isomutecntl2;
#[doc = "ISO Channel 2 current Tx pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isocurrenttxptr2](bb_isocurrenttxptr2) module"]
pub type BB_ISOCURRENTTXPTR2 = crate::Reg<u32, _BB_ISOCURRENTTXPTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOCURRENTTXPTR2;
#[doc = "`read()` method returns [bb_isocurrenttxptr2::R](bb_isocurrenttxptr2::R) reader structure"]
impl crate::Readable for BB_ISOCURRENTTXPTR2 {}
#[doc = "`write(|w| ..)` method takes [bb_isocurrenttxptr2::W](bb_isocurrenttxptr2::W) writer structure"]
impl crate::Writable for BB_ISOCURRENTTXPTR2 {}
#[doc = "ISO Channel 2 current Tx pointer"]
pub mod bb_isocurrenttxptr2;
#[doc = "ISO Channel 2 current Rx pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isocurrentrxptr2](bb_isocurrentrxptr2) module"]
pub type BB_ISOCURRENTRXPTR2 = crate::Reg<u32, _BB_ISOCURRENTRXPTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOCURRENTRXPTR2;
#[doc = "`read()` method returns [bb_isocurrentrxptr2::R](bb_isocurrentrxptr2::R) reader structure"]
impl crate::Readable for BB_ISOCURRENTRXPTR2 {}
#[doc = "`write(|w| ..)` method takes [bb_isocurrentrxptr2::W](bb_isocurrentrxptr2::W) writer structure"]
impl crate::Writable for BB_ISOCURRENTRXPTR2 {}
#[doc = "ISO Channel 2 current Rx pointer"]
pub mod bb_isocurrentrxptr2;
#[doc = "ISO Channel 2 payloads\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isotrcnl2](bb_isotrcnl2) module"]
pub type BB_ISOTRCNL2 = crate::Reg<u32, _BB_ISOTRCNL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOTRCNL2;
#[doc = "`read()` method returns [bb_isotrcnl2::R](bb_isotrcnl2::R) reader structure"]
impl crate::Readable for BB_ISOTRCNL2 {}
#[doc = "`write(|w| ..)` method takes [bb_isotrcnl2::W](bb_isotrcnl2::W) writer structure"]
impl crate::Writable for BB_ISOTRCNL2 {}
#[doc = "ISO Channel 2 payloads"]
pub mod bb_isotrcnl2;
#[doc = "ISO Channel 2 mute control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isoevtcntloffsetl2](bb_isoevtcntloffsetl2) module"]
pub type BB_ISOEVTCNTLOFFSETL2 = crate::Reg<u32, _BB_ISOEVTCNTLOFFSETL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOEVTCNTLOFFSETL2;
#[doc = "`read()` method returns [bb_isoevtcntloffsetl2::R](bb_isoevtcntloffsetl2::R) reader structure"]
impl crate::Readable for BB_ISOEVTCNTLOFFSETL2 {}
#[doc = "`write(|w| ..)` method takes [bb_isoevtcntloffsetl2::W](bb_isoevtcntloffsetl2::W) writer structure"]
impl crate::Writable for BB_ISOEVTCNTLOFFSETL2 {}
#[doc = "ISO Channel 2 mute control"]
pub mod bb_isoevtcntloffsetl2;
#[doc = "ISO Channel 2 mute control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_isoevtcntloffsetu2](bb_isoevtcntloffsetu2) module"]
pub type BB_ISOEVTCNTLOFFSETU2 = crate::Reg<u32, _BB_ISOEVTCNTLOFFSETU2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_ISOEVTCNTLOFFSETU2;
#[doc = "`read()` method returns [bb_isoevtcntloffsetu2::R](bb_isoevtcntloffsetu2::R) reader structure"]
impl crate::Readable for BB_ISOEVTCNTLOFFSETU2 {}
#[doc = "`write(|w| ..)` method takes [bb_isoevtcntloffsetu2::W](bb_isoevtcntloffsetu2::W) writer structure"]
impl crate::Writable for BB_ISOEVTCNTLOFFSETU2 {}
#[doc = "ISO Channel 2 mute control"]
pub mod bb_isoevtcntloffsetu2;
#[doc = "Register controlling the decision instant for priority scheduling arbitration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_bbprioscharb](bb_bbprioscharb) module"]
pub type BB_BBPRIOSCHARB = crate::Reg<u32, _BB_BBPRIOSCHARB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_BBPRIOSCHARB;
#[doc = "`read()` method returns [bb_bbprioscharb::R](bb_bbprioscharb::R) reader structure"]
impl crate::Readable for BB_BBPRIOSCHARB {}
#[doc = "`write(|w| ..)` method takes [bb_bbprioscharb::W](bb_bbprioscharb::W) writer structure"]
impl crate::Writable for BB_BBPRIOSCHARB {}
#[doc = "Register controlling the decision instant for priority scheduling arbitration"]
pub mod bb_bbprioscharb;
