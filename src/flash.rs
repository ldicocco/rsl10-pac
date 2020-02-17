#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Interface Control Register"]
    pub flash_if_ctrl: FLASH_IF_CTRL,
    #[doc = "0x04 - Flash Main Write Unlock Register"]
    pub flash_main_write_unlock: FLASH_MAIN_WRITE_UNLOCK,
    #[doc = "0x08 - Flash Main Write Control Register"]
    pub flash_main_ctrl: FLASH_MAIN_CTRL,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Flash, Memory and RF Power-Up Delay Configuration"]
    pub flash_delay_ctrl: FLASH_DELAY_CTRL,
    _reserved4: [u8; 32usize],
    #[doc = "0x34 - Flash Command Control Register"]
    pub flash_cmd_ctrl: FLASH_CMD_CTRL,
    #[doc = "0x38 - Flash Interface Status Register"]
    pub flash_if_status: FLASH_IF_STATUS,
    #[doc = "0x3c - Flash Address Register"]
    pub flash_addr: FLASH_ADDR,
    #[doc = "0x40 - Flash Read/Write Data Register"]
    pub flash_data: [FLASH_DATA; 2],
    #[doc = "0x48 - Flash NVR Write Unlock Register"]
    pub flash_nvr_write_unlock: FLASH_NVR_WRITE_UNLOCK,
    #[doc = "0x4c - Flash NVR Control Register"]
    pub flash_nvr_ctrl: FLASH_NVR_CTRL,
    _reserved10: [u8; 24usize],
    #[doc = "0x68 - Flash Patch Address Register"]
    pub flash_patch_addr: [FLASH_PATCH_ADDR; 4],
    _reserved11: [u8; 8usize],
    #[doc = "0x80 - Flash Copier Config Register"]
    pub flash_copy_cfg: FLASH_COPY_CFG,
    _reserved12: [u8; 68usize],
    #[doc = "0xc8 - Flash-to-Memory Copier Control and Status"]
    pub flash_copy_ctrl: FLASH_COPY_CTRL,
    _reserved13: [u8; 4usize],
    #[doc = "0xd0 - Flash-to-Memory Copier Source Address Pointer"]
    pub flash_copy_src_addr_ptr: FLASH_COPY_SRC_ADDR_PTR,
    #[doc = "0xd4 - Flash-to-Memory Copier Destination Address Pointer"]
    pub flash_copy_dst_addr_ptr: FLASH_COPY_DST_ADDR_PTR,
    #[doc = "0xd8 - Flash-to-Memory Copier Word Count"]
    pub flash_copy_word_cnt: FLASH_COPY_WORD_CNT,
    #[doc = "0xdc - Flash ECC Control Register"]
    pub flash_ecc_ctrl: FLASH_ECC_CTRL,
    #[doc = "0xe0 - Flash ECC Status Register"]
    pub flash_ecc_status: FLASH_ECC_STATUS,
    #[doc = "0xe4 - Flash Address of the Latest Detected Error"]
    pub flash_ecc_error_addr: FLASH_ECC_ERROR_ADDR,
    #[doc = "0xe8 - Flash ECC Uncorrected Error Counter"]
    pub flash_ecc_uncor_error_cnt: FLASH_ECC_UNCOR_ERROR_CNT,
    #[doc = "0xec - Flash ECC Corrected Error Counter"]
    pub flash_ecc_cor_error_cnt: FLASH_ECC_COR_ERROR_CNT,
}
#[doc = "Flash Interface Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_if_ctrl](flash_if_ctrl) module"]
pub type FLASH_IF_CTRL = crate::Reg<u32, _FLASH_IF_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_IF_CTRL;
#[doc = "`read()` method returns [flash_if_ctrl::R](flash_if_ctrl::R) reader structure"]
impl crate::Readable for FLASH_IF_CTRL {}
#[doc = "`write(|w| ..)` method takes [flash_if_ctrl::W](flash_if_ctrl::W) writer structure"]
impl crate::Writable for FLASH_IF_CTRL {}
#[doc = "Flash Interface Control Register"]
pub mod flash_if_ctrl;
#[doc = "Flash Main Write Unlock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_main_write_unlock](flash_main_write_unlock) module"]
pub type FLASH_MAIN_WRITE_UNLOCK = crate::Reg<u32, _FLASH_MAIN_WRITE_UNLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_MAIN_WRITE_UNLOCK;
#[doc = "`read()` method returns [flash_main_write_unlock::R](flash_main_write_unlock::R) reader structure"]
impl crate::Readable for FLASH_MAIN_WRITE_UNLOCK {}
#[doc = "`write(|w| ..)` method takes [flash_main_write_unlock::W](flash_main_write_unlock::W) writer structure"]
impl crate::Writable for FLASH_MAIN_WRITE_UNLOCK {}
#[doc = "Flash Main Write Unlock Register"]
pub mod flash_main_write_unlock;
#[doc = "Flash Main Write Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_main_ctrl](flash_main_ctrl) module"]
pub type FLASH_MAIN_CTRL = crate::Reg<u32, _FLASH_MAIN_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_MAIN_CTRL;
#[doc = "`read()` method returns [flash_main_ctrl::R](flash_main_ctrl::R) reader structure"]
impl crate::Readable for FLASH_MAIN_CTRL {}
#[doc = "`write(|w| ..)` method takes [flash_main_ctrl::W](flash_main_ctrl::W) writer structure"]
impl crate::Writable for FLASH_MAIN_CTRL {}
#[doc = "Flash Main Write Control Register"]
pub mod flash_main_ctrl;
#[doc = "Flash, Memory and RF Power-Up Delay Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_delay_ctrl](flash_delay_ctrl) module"]
pub type FLASH_DELAY_CTRL = crate::Reg<u32, _FLASH_DELAY_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_DELAY_CTRL;
#[doc = "`read()` method returns [flash_delay_ctrl::R](flash_delay_ctrl::R) reader structure"]
impl crate::Readable for FLASH_DELAY_CTRL {}
#[doc = "`write(|w| ..)` method takes [flash_delay_ctrl::W](flash_delay_ctrl::W) writer structure"]
impl crate::Writable for FLASH_DELAY_CTRL {}
#[doc = "Flash, Memory and RF Power-Up Delay Configuration"]
pub mod flash_delay_ctrl;
#[doc = "Flash Command Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_cmd_ctrl](flash_cmd_ctrl) module"]
pub type FLASH_CMD_CTRL = crate::Reg<u32, _FLASH_CMD_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_CMD_CTRL;
#[doc = "`read()` method returns [flash_cmd_ctrl::R](flash_cmd_ctrl::R) reader structure"]
impl crate::Readable for FLASH_CMD_CTRL {}
#[doc = "`write(|w| ..)` method takes [flash_cmd_ctrl::W](flash_cmd_ctrl::W) writer structure"]
impl crate::Writable for FLASH_CMD_CTRL {}
#[doc = "Flash Command Control Register"]
pub mod flash_cmd_ctrl;
#[doc = "Flash Interface Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_if_status](flash_if_status) module"]
pub type FLASH_IF_STATUS = crate::Reg<u32, _FLASH_IF_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_IF_STATUS;
#[doc = "`read()` method returns [flash_if_status::R](flash_if_status::R) reader structure"]
impl crate::Readable for FLASH_IF_STATUS {}
#[doc = "Flash Interface Status Register"]
pub mod flash_if_status;
#[doc = "Flash Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_addr](flash_addr) module"]
pub type FLASH_ADDR = crate::Reg<u32, _FLASH_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ADDR;
#[doc = "`read()` method returns [flash_addr::R](flash_addr::R) reader structure"]
impl crate::Readable for FLASH_ADDR {}
#[doc = "`write(|w| ..)` method takes [flash_addr::W](flash_addr::W) writer structure"]
impl crate::Writable for FLASH_ADDR {}
#[doc = "Flash Address Register"]
pub mod flash_addr;
#[doc = "Flash Read/Write Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_data](flash_data) module"]
pub type FLASH_DATA = crate::Reg<u32, _FLASH_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_DATA;
#[doc = "`read()` method returns [flash_data::R](flash_data::R) reader structure"]
impl crate::Readable for FLASH_DATA {}
#[doc = "`write(|w| ..)` method takes [flash_data::W](flash_data::W) writer structure"]
impl crate::Writable for FLASH_DATA {}
#[doc = "Flash Read/Write Data Register"]
pub mod flash_data;
#[doc = "Flash NVR Write Unlock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_nvr_write_unlock](flash_nvr_write_unlock) module"]
pub type FLASH_NVR_WRITE_UNLOCK = crate::Reg<u32, _FLASH_NVR_WRITE_UNLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_NVR_WRITE_UNLOCK;
#[doc = "`read()` method returns [flash_nvr_write_unlock::R](flash_nvr_write_unlock::R) reader structure"]
impl crate::Readable for FLASH_NVR_WRITE_UNLOCK {}
#[doc = "`write(|w| ..)` method takes [flash_nvr_write_unlock::W](flash_nvr_write_unlock::W) writer structure"]
impl crate::Writable for FLASH_NVR_WRITE_UNLOCK {}
#[doc = "Flash NVR Write Unlock Register"]
pub mod flash_nvr_write_unlock;
#[doc = "Flash NVR Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_nvr_ctrl](flash_nvr_ctrl) module"]
pub type FLASH_NVR_CTRL = crate::Reg<u32, _FLASH_NVR_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_NVR_CTRL;
#[doc = "`read()` method returns [flash_nvr_ctrl::R](flash_nvr_ctrl::R) reader structure"]
impl crate::Readable for FLASH_NVR_CTRL {}
#[doc = "`write(|w| ..)` method takes [flash_nvr_ctrl::W](flash_nvr_ctrl::W) writer structure"]
impl crate::Writable for FLASH_NVR_CTRL {}
#[doc = "Flash NVR Control Register"]
pub mod flash_nvr_ctrl;
#[doc = "Flash Patch Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_patch_addr](flash_patch_addr) module"]
pub type FLASH_PATCH_ADDR = crate::Reg<u32, _FLASH_PATCH_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_PATCH_ADDR;
#[doc = "`read()` method returns [flash_patch_addr::R](flash_patch_addr::R) reader structure"]
impl crate::Readable for FLASH_PATCH_ADDR {}
#[doc = "`write(|w| ..)` method takes [flash_patch_addr::W](flash_patch_addr::W) writer structure"]
impl crate::Writable for FLASH_PATCH_ADDR {}
#[doc = "Flash Patch Address Register"]
pub mod flash_patch_addr;
#[doc = "Flash Copier Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_copy_cfg](flash_copy_cfg) module"]
pub type FLASH_COPY_CFG = crate::Reg<u32, _FLASH_COPY_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_COPY_CFG;
#[doc = "`read()` method returns [flash_copy_cfg::R](flash_copy_cfg::R) reader structure"]
impl crate::Readable for FLASH_COPY_CFG {}
#[doc = "`write(|w| ..)` method takes [flash_copy_cfg::W](flash_copy_cfg::W) writer structure"]
impl crate::Writable for FLASH_COPY_CFG {}
#[doc = "Flash Copier Config Register"]
pub mod flash_copy_cfg;
#[doc = "Flash-to-Memory Copier Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_copy_ctrl](flash_copy_ctrl) module"]
pub type FLASH_COPY_CTRL = crate::Reg<u32, _FLASH_COPY_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_COPY_CTRL;
#[doc = "`read()` method returns [flash_copy_ctrl::R](flash_copy_ctrl::R) reader structure"]
impl crate::Readable for FLASH_COPY_CTRL {}
#[doc = "`write(|w| ..)` method takes [flash_copy_ctrl::W](flash_copy_ctrl::W) writer structure"]
impl crate::Writable for FLASH_COPY_CTRL {}
#[doc = "Flash-to-Memory Copier Control and Status"]
pub mod flash_copy_ctrl;
#[doc = "Flash-to-Memory Copier Source Address Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_copy_src_addr_ptr](flash_copy_src_addr_ptr) module"]
pub type FLASH_COPY_SRC_ADDR_PTR = crate::Reg<u32, _FLASH_COPY_SRC_ADDR_PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_COPY_SRC_ADDR_PTR;
#[doc = "`read()` method returns [flash_copy_src_addr_ptr::R](flash_copy_src_addr_ptr::R) reader structure"]
impl crate::Readable for FLASH_COPY_SRC_ADDR_PTR {}
#[doc = "`write(|w| ..)` method takes [flash_copy_src_addr_ptr::W](flash_copy_src_addr_ptr::W) writer structure"]
impl crate::Writable for FLASH_COPY_SRC_ADDR_PTR {}
#[doc = "Flash-to-Memory Copier Source Address Pointer"]
pub mod flash_copy_src_addr_ptr;
#[doc = "Flash-to-Memory Copier Destination Address Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_copy_dst_addr_ptr](flash_copy_dst_addr_ptr) module"]
pub type FLASH_COPY_DST_ADDR_PTR = crate::Reg<u32, _FLASH_COPY_DST_ADDR_PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_COPY_DST_ADDR_PTR;
#[doc = "`read()` method returns [flash_copy_dst_addr_ptr::R](flash_copy_dst_addr_ptr::R) reader structure"]
impl crate::Readable for FLASH_COPY_DST_ADDR_PTR {}
#[doc = "`write(|w| ..)` method takes [flash_copy_dst_addr_ptr::W](flash_copy_dst_addr_ptr::W) writer structure"]
impl crate::Writable for FLASH_COPY_DST_ADDR_PTR {}
#[doc = "Flash-to-Memory Copier Destination Address Pointer"]
pub mod flash_copy_dst_addr_ptr;
#[doc = "Flash-to-Memory Copier Word Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_copy_word_cnt](flash_copy_word_cnt) module"]
pub type FLASH_COPY_WORD_CNT = crate::Reg<u32, _FLASH_COPY_WORD_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_COPY_WORD_CNT;
#[doc = "`read()` method returns [flash_copy_word_cnt::R](flash_copy_word_cnt::R) reader structure"]
impl crate::Readable for FLASH_COPY_WORD_CNT {}
#[doc = "`write(|w| ..)` method takes [flash_copy_word_cnt::W](flash_copy_word_cnt::W) writer structure"]
impl crate::Writable for FLASH_COPY_WORD_CNT {}
#[doc = "Flash-to-Memory Copier Word Count"]
pub mod flash_copy_word_cnt;
#[doc = "Flash ECC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ecc_ctrl](flash_ecc_ctrl) module"]
pub type FLASH_ECC_CTRL = crate::Reg<u32, _FLASH_ECC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ECC_CTRL;
#[doc = "`read()` method returns [flash_ecc_ctrl::R](flash_ecc_ctrl::R) reader structure"]
impl crate::Readable for FLASH_ECC_CTRL {}
#[doc = "`write(|w| ..)` method takes [flash_ecc_ctrl::W](flash_ecc_ctrl::W) writer structure"]
impl crate::Writable for FLASH_ECC_CTRL {}
#[doc = "Flash ECC Control Register"]
pub mod flash_ecc_ctrl;
#[doc = "Flash ECC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ecc_status](flash_ecc_status) module"]
pub type FLASH_ECC_STATUS = crate::Reg<u32, _FLASH_ECC_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ECC_STATUS;
#[doc = "`read()` method returns [flash_ecc_status::R](flash_ecc_status::R) reader structure"]
impl crate::Readable for FLASH_ECC_STATUS {}
#[doc = "`write(|w| ..)` method takes [flash_ecc_status::W](flash_ecc_status::W) writer structure"]
impl crate::Writable for FLASH_ECC_STATUS {}
#[doc = "Flash ECC Status Register"]
pub mod flash_ecc_status;
#[doc = "Flash Address of the Latest Detected Error\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ecc_error_addr](flash_ecc_error_addr) module"]
pub type FLASH_ECC_ERROR_ADDR = crate::Reg<u32, _FLASH_ECC_ERROR_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ECC_ERROR_ADDR;
#[doc = "`read()` method returns [flash_ecc_error_addr::R](flash_ecc_error_addr::R) reader structure"]
impl crate::Readable for FLASH_ECC_ERROR_ADDR {}
#[doc = "Flash Address of the Latest Detected Error"]
pub mod flash_ecc_error_addr;
#[doc = "Flash ECC Uncorrected Error Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ecc_uncor_error_cnt](flash_ecc_uncor_error_cnt) module"]
pub type FLASH_ECC_UNCOR_ERROR_CNT = crate::Reg<u32, _FLASH_ECC_UNCOR_ERROR_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ECC_UNCOR_ERROR_CNT;
#[doc = "`read()` method returns [flash_ecc_uncor_error_cnt::R](flash_ecc_uncor_error_cnt::R) reader structure"]
impl crate::Readable for FLASH_ECC_UNCOR_ERROR_CNT {}
#[doc = "Flash ECC Uncorrected Error Counter"]
pub mod flash_ecc_uncor_error_cnt;
#[doc = "Flash ECC Corrected Error Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ecc_cor_error_cnt](flash_ecc_cor_error_cnt) module"]
pub type FLASH_ECC_COR_ERROR_CNT = crate::Reg<u32, _FLASH_ECC_COR_ERROR_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ECC_COR_ERROR_CNT;
#[doc = "`read()` method returns [flash_ecc_cor_error_cnt::R](flash_ecc_cor_error_cnt::R) reader structure"]
impl crate::Readable for FLASH_ECC_COR_ERROR_CNT {}
#[doc = "Flash ECC Corrected Error Counter"]
pub mod flash_ecc_cor_error_cnt;
