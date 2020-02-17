#[doc = "Reader of register BB_ERRORTYPESTAT"]
pub type R = crate::R<u32, super::BB_ERRORTYPESTAT>;
#[doc = "Indicates Resolving Address List engine Under run issue, happens when RAL List parsing not finished on time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAL_UNDERRUN_A {
    #[doc = "0: No error"]
    RAL_UNDERRUN_0 = 0,
    #[doc = "1: Error occurred"]
    RAL_UNDERRUN_1 = 1,
}
impl From<RAL_UNDERRUN_A> for bool {
    #[inline(always)]
    fn from(variant: RAL_UNDERRUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RAL_UNDERRUN`"]
pub type RAL_UNDERRUN_R = crate::R<bool, RAL_UNDERRUN_A>;
impl RAL_UNDERRUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAL_UNDERRUN_A {
        match self.bits {
            false => RAL_UNDERRUN_A::RAL_UNDERRUN_0,
            true => RAL_UNDERRUN_A::RAL_UNDERRUN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RAL_UNDERRUN_0`"]
    #[inline(always)]
    pub fn is_ral_underrun_0(&self) -> bool {
        *self == RAL_UNDERRUN_A::RAL_UNDERRUN_0
    }
    #[doc = "Checks if the value of the field is `RAL_UNDERRUN_1`"]
    #[inline(always)]
    pub fn is_ral_underrun_1(&self) -> bool {
        *self == RAL_UNDERRUN_A::RAL_UNDERRUN_1
    }
}
#[doc = "Indicates Resolving Address List engine faced a bad setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAL_ERROR_A {
    #[doc = "0: No error"]
    RAL_ERROR_0 = 0,
    #[doc = "1: Error occurred"]
    RAL_ERROR_1 = 1,
}
impl From<RAL_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: RAL_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RAL_ERROR`"]
pub type RAL_ERROR_R = crate::R<bool, RAL_ERROR_A>;
impl RAL_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAL_ERROR_A {
        match self.bits {
            false => RAL_ERROR_A::RAL_ERROR_0,
            true => RAL_ERROR_A::RAL_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `RAL_ERROR_0`"]
    #[inline(always)]
    pub fn is_ral_error_0(&self) -> bool {
        *self == RAL_ERROR_A::RAL_ERROR_0
    }
    #[doc = "Checks if the value of the field is `RAL_ERROR_1`"]
    #[inline(always)]
    pub fn is_ral_error_1(&self) -> bool {
        *self == RAL_ERROR_A::RAL_ERROR_1
    }
}
#[doc = "Indicates whether two consecutive and concurrent ble_event_irq have been generated, and not acknowledged in time by the RW-BLE software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONCEVTIRQ_ERROR_A {
    #[doc = "0: No error"]
    CONCEVTIRQ_ERROR_0 = 0,
    #[doc = "1: Error occurred"]
    CONCEVTIRQ_ERROR_1 = 1,
}
impl From<CONCEVTIRQ_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: CONCEVTIRQ_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CONCEVTIRQ_ERROR`"]
pub type CONCEVTIRQ_ERROR_R = crate::R<bool, CONCEVTIRQ_ERROR_A>;
impl CONCEVTIRQ_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONCEVTIRQ_ERROR_A {
        match self.bits {
            false => CONCEVTIRQ_ERROR_A::CONCEVTIRQ_ERROR_0,
            true => CONCEVTIRQ_ERROR_A::CONCEVTIRQ_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONCEVTIRQ_ERROR_0`"]
    #[inline(always)]
    pub fn is_concevtirq_error_0(&self) -> bool {
        *self == CONCEVTIRQ_ERROR_A::CONCEVTIRQ_ERROR_0
    }
    #[doc = "Checks if the value of the field is `CONCEVTIRQ_ERROR_1`"]
    #[inline(always)]
    pub fn is_concevtirq_error_1(&self) -> bool {
        *self == CONCEVTIRQ_ERROR_A::CONCEVTIRQ_ERROR_1
    }
}
#[doc = "Indicates whether Rx data buffer pointer value programmed is null (major failure)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDATA_PTR_ERROR_A {
    #[doc = "0: No error"]
    RXDATA_PTR_ERROR_0 = 0,
    #[doc = "1: Error occurred"]
    RXDATA_PTR_ERROR_1 = 1,
}
impl From<RXDATA_PTR_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: RXDATA_PTR_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXDATA_PTR_ERROR`"]
pub type RXDATA_PTR_ERROR_R = crate::R<bool, RXDATA_PTR_ERROR_A>;
impl RXDATA_PTR_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDATA_PTR_ERROR_A {
        match self.bits {
            false => RXDATA_PTR_ERROR_A::RXDATA_PTR_ERROR_0,
            true => RXDATA_PTR_ERROR_A::RXDATA_PTR_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXDATA_PTR_ERROR_0`"]
    #[inline(always)]
    pub fn is_rxdata_ptr_error_0(&self) -> bool {
        *self == RXDATA_PTR_ERROR_A::RXDATA_PTR_ERROR_0
    }
    #[doc = "Checks if the value of the field is `RXDATA_PTR_ERROR_1`"]
    #[inline(always)]
    pub fn is_rxdata_ptr_error_1(&self) -> bool {
        *self == RXDATA_PTR_ERROR_A::RXDATA_PTR_ERROR_1
    }
}
#[doc = "Indicates whether Tx data buffer pointer value programmed is null during advertising / scanning / initiating events, or during master / slave connections with non-null packet length (major failure)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDATA_PTR_ERROR_A {
    #[doc = "0: No error"]
    TXDATA_PTR_ERROR_0 = 0,
    #[doc = "1: Error occurred"]
    TXDATA_PTR_ERROR_1 = 1,
}
impl From<TXDATA_PTR_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: TXDATA_PTR_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXDATA_PTR_ERROR`"]
pub type TXDATA_PTR_ERROR_R = crate::R<bool, TXDATA_PTR_ERROR_A>;
impl TXDATA_PTR_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDATA_PTR_ERROR_A {
        match self.bits {
            false => TXDATA_PTR_ERROR_A::TXDATA_PTR_ERROR_0,
            true => TXDATA_PTR_ERROR_A::TXDATA_PTR_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXDATA_PTR_ERROR_0`"]
    #[inline(always)]
    pub fn is_txdata_ptr_error_0(&self) -> bool {
        *self == TXDATA_PTR_ERROR_A::TXDATA_PTR_ERROR_0
    }
    #[doc = "Checks if the value of the field is `TXDATA_PTR_ERROR_1`"]
    #[inline(always)]
    pub fn is_txdata_ptr_error_1(&self) -> bool {
        *self == TXDATA_PTR_ERROR_A::TXDATA_PTR_ERROR_1
    }
}
#[doc = "Indicates whether Rx descriptor pointer value programmed in register is null (major failure)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDESC_EMPTY_ERROR_A {
    #[doc = "0: No error"]
    RXDESC_EMPTY_ERROR_0 = 0,
    #[doc = "1: Error occurred"]
    RXDESC_EMPTY_ERROR_1 = 1,
}
impl From<RXDESC_EMPTY_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: RXDESC_EMPTY_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXDESC_EMPTY_ERROR`"]
pub type RXDESC_EMPTY_ERROR_R = crate::R<bool, RXDESC_EMPTY_ERROR_A>;
impl RXDESC_EMPTY_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDESC_EMPTY_ERROR_A {
        match self.bits {
            false => RXDESC_EMPTY_ERROR_A::RXDESC_EMPTY_ERROR_0,
            true => RXDESC_EMPTY_ERROR_A::RXDESC_EMPTY_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXDESC_EMPTY_ERROR_0`"]
    #[inline(always)]
    pub fn is_rxdesc_empty_error_0(&self) -> bool {
        *self == RXDESC_EMPTY_ERROR_A::RXDESC_EMPTY_ERROR_0
    }
    #[doc = "Checks if the value of the field is `RXDESC_EMPTY_ERROR_1`"]
    #[inline(always)]
    pub fn is_rxdesc_empty_error_1(&self) -> bool {
        *self == RXDESC_EMPTY_ERROR_A::RXDESC_EMPTY_ERROR_1
    }
}
#[doc = "Indicates whether Tx descriptor pointer value programmed in control structure is null during advertising / scanning / initiating events (major failure)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDESC_EMPTY_ERROR_A {
    #[doc = "0: No error"]
    TXDESC_EMPTY_ERROR_0 = 0,
    #[doc = "1: Error occurred"]
    TXDESC_EMPTY_ERROR_1 = 1,
}
impl From<TXDESC_EMPTY_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: TXDESC_EMPTY_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXDESC_EMPTY_ERROR`"]
pub type TXDESC_EMPTY_ERROR_R = crate::R<bool, TXDESC_EMPTY_ERROR_A>;
impl TXDESC_EMPTY_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDESC_EMPTY_ERROR_A {
        match self.bits {
            false => TXDESC_EMPTY_ERROR_A::TXDESC_EMPTY_ERROR_0,
            true => TXDESC_EMPTY_ERROR_A::TXDESC_EMPTY_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXDESC_EMPTY_ERROR_0`"]
    #[inline(always)]
    pub fn is_txdesc_empty_error_0(&self) -> bool {
        *self == TXDESC_EMPTY_ERROR_A::TXDESC_EMPTY_ERROR_0
    }
    #[doc = "Checks if the value of the field is `TXDESC_EMPTY_ERROR_1`"]
    #[inline(always)]
    pub fn is_txdesc_empty_error_1(&self) -> bool {
        *self == TXDESC_EMPTY_ERROR_A::TXDESC_EMPTY_ERROR_1
    }
}
#[doc = "Indicates whether CS-FORMAT has been programmed with an invalid value (major failure)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSFORMAT_ERROR_A {
    #[doc = "0: No error"]
    CSFORMAT_ERROR_0 = 0,
    #[doc = "1: Error occurred"]
    CSFORMAT_ERROR_1 = 1,
}
impl From<CSFORMAT_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: CSFORMAT_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSFORMAT_ERROR`"]
pub type CSFORMAT_ERROR_R = crate::R<bool, CSFORMAT_ERROR_A>;
impl CSFORMAT_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSFORMAT_ERROR_A {
        match self.bits {
            false => CSFORMAT_ERROR_A::CSFORMAT_ERROR_0,
            true => CSFORMAT_ERROR_A::CSFORMAT_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `CSFORMAT_ERROR_0`"]
    #[inline(always)]
    pub fn is_csformat_error_0(&self) -> bool {
        *self == CSFORMAT_ERROR_A::CSFORMAT_ERROR_0
    }
    #[doc = "Checks if the value of the field is `CSFORMAT_ERROR_1`"]
    #[inline(always)]
    pub fn is_csformat_error_1(&self) -> bool {
        *self == CSFORMAT_ERROR_A::CSFORMAT_ERROR_1
    }
}
#[doc = "Indicates Link Layer channel map error, happens when actual number of CS-LLCHMAP bit set to one is different from CS-NBCHGOOD at the beginning of frequency hopping process\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLCHMAP_ERROR_A {
    #[doc = "0: No error"]
    LLCHMAP_ERROR_0 = 0,
    #[doc = "1: Error occurred"]
    LLCHMAP_ERROR_1 = 1,
}
impl From<LLCHMAP_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: LLCHMAP_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LLCHMAP_ERROR`"]
pub type LLCHMAP_ERROR_R = crate::R<bool, LLCHMAP_ERROR_A>;
impl LLCHMAP_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LLCHMAP_ERROR_A {
        match self.bits {
            false => LLCHMAP_ERROR_A::LLCHMAP_ERROR_0,
            true => LLCHMAP_ERROR_A::LLCHMAP_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `LLCHMAP_ERROR_0`"]
    #[inline(always)]
    pub fn is_llchmap_error_0(&self) -> bool {
        *self == LLCHMAP_ERROR_A::LLCHMAP_ERROR_0
    }
    #[doc = "Checks if the value of the field is `LLCHMAP_ERROR_1`"]
    #[inline(always)]
    pub fn is_llchmap_error_1(&self) -> bool {
        *self == LLCHMAP_ERROR_A::LLCHMAP_ERROR_1
    }
}
#[doc = "Indicates advertising interval under run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADV_UNDERRUN_A {
    #[doc = "0: No error"]
    ADV_UNDERRUN_0 = 0,
    #[doc = "1: Error occurred"]
    ADV_UNDERRUN_1 = 1,
}
impl From<ADV_UNDERRUN_A> for bool {
    #[inline(always)]
    fn from(variant: ADV_UNDERRUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADV_UNDERRUN`"]
pub type ADV_UNDERRUN_R = crate::R<bool, ADV_UNDERRUN_A>;
impl ADV_UNDERRUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADV_UNDERRUN_A {
        match self.bits {
            false => ADV_UNDERRUN_A::ADV_UNDERRUN_0,
            true => ADV_UNDERRUN_A::ADV_UNDERRUN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADV_UNDERRUN_0`"]
    #[inline(always)]
    pub fn is_adv_underrun_0(&self) -> bool {
        *self == ADV_UNDERRUN_A::ADV_UNDERRUN_0
    }
    #[doc = "Checks if the value of the field is `ADV_UNDERRUN_1`"]
    #[inline(always)]
    pub fn is_adv_underrun_1(&self) -> bool {
        *self == ADV_UNDERRUN_A::ADV_UNDERRUN_1
    }
}
#[doc = "Indicates inter frame space under run, occurs if IFS time is not enough to update and read control structure / descriptors, and/or white list parsing is not finished and/or decryption time is too long to be finished on time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFS_UNDERRUN_A {
    #[doc = "0: No error"]
    IFS_UNDERRUN_0 = 0,
    #[doc = "1: Error occurred"]
    IFS_UNDERRUN_1 = 1,
}
impl From<IFS_UNDERRUN_A> for bool {
    #[inline(always)]
    fn from(variant: IFS_UNDERRUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IFS_UNDERRUN`"]
pub type IFS_UNDERRUN_R = crate::R<bool, IFS_UNDERRUN_A>;
impl IFS_UNDERRUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFS_UNDERRUN_A {
        match self.bits {
            false => IFS_UNDERRUN_A::IFS_UNDERRUN_0,
            true => IFS_UNDERRUN_A::IFS_UNDERRUN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IFS_UNDERRUN_0`"]
    #[inline(always)]
    pub fn is_ifs_underrun_0(&self) -> bool {
        *self == IFS_UNDERRUN_A::IFS_UNDERRUN_0
    }
    #[doc = "Checks if the value of the field is `IFS_UNDERRUN_1`"]
    #[inline(always)]
    pub fn is_ifs_underrun_1(&self) -> bool {
        *self == IFS_UNDERRUN_A::IFS_UNDERRUN_1
    }
}
#[doc = "Indicates white list timeout error, occurs if white list parsing is not finished on time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WHITELIST_ERROR_A {
    #[doc = "0: No error"]
    WHITELIST_ERROR_0 = 0,
    #[doc = "1: Error occurred"]
    WHITELIST_ERROR_1 = 1,
}
impl From<WHITELIST_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: WHITELIST_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WHITELIST_ERROR`"]
pub type WHITELIST_ERROR_R = crate::R<bool, WHITELIST_ERROR_A>;
impl WHITELIST_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WHITELIST_ERROR_A {
        match self.bits {
            false => WHITELIST_ERROR_A::WHITELIST_ERROR_0,
            true => WHITELIST_ERROR_A::WHITELIST_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `WHITELIST_ERROR_0`"]
    #[inline(always)]
    pub fn is_whitelist_error_0(&self) -> bool {
        *self == WHITELIST_ERROR_A::WHITELIST_ERROR_0
    }
    #[doc = "Checks if the value of the field is `WHITELIST_ERROR_1`"]
    #[inline(always)]
    pub fn is_whitelist_error_1(&self) -> bool {
        *self == WHITELIST_ERROR_A::WHITELIST_ERROR_1
    }
}
#[doc = "Indicates anticipated pre-fetch mechanism error: happens when 2 consecutive events are programmed, and when the first event is not completely finished while second pre-fetch instant is reached\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVT_CNTL_APFM_ERROR_A {
    #[doc = "0: No error"]
    EVT_CNTL_APFM_ERROR_0 = 0,
    #[doc = "1: Error occurred"]
    EVT_CNTL_APFM_ERROR_1 = 1,
}
impl From<EVT_CNTL_APFM_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: EVT_CNTL_APFM_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVT_CNTL_APFM_ERROR`"]
pub type EVT_CNTL_APFM_ERROR_R = crate::R<bool, EVT_CNTL_APFM_ERROR_A>;
impl EVT_CNTL_APFM_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVT_CNTL_APFM_ERROR_A {
        match self.bits {
            false => EVT_CNTL_APFM_ERROR_A::EVT_CNTL_APFM_ERROR_0,
            true => EVT_CNTL_APFM_ERROR_A::EVT_CNTL_APFM_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `EVT_CNTL_APFM_ERROR_0`"]
    #[inline(always)]
    pub fn is_evt_cntl_apfm_error_0(&self) -> bool {
        *self == EVT_CNTL_APFM_ERROR_A::EVT_CNTL_APFM_ERROR_0
    }
    #[doc = "Checks if the value of the field is `EVT_CNTL_APFM_ERROR_1`"]
    #[inline(always)]
    pub fn is_evt_cntl_apfm_error_1(&self) -> bool {
        *self == EVT_CNTL_APFM_ERROR_A::EVT_CNTL_APFM_ERROR_1
    }
}
#[doc = "Indicates anticipated pre-fetch mechanism error: happens when 2 consecutive events are programmed, and when the first event is not completely finished while second pre-fetch instant is reached\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVT_SCHDL_APFM_ERROR_A {
    #[doc = "0: No error"]
    EVT_SCHDL_APFM_ERROR_0 = 0,
    #[doc = "1: Error occurred"]
    EVT_SCHDL_APFM_ERROR_1 = 1,
}
impl From<EVT_SCHDL_APFM_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: EVT_SCHDL_APFM_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVT_SCHDL_APFM_ERROR`"]
pub type EVT_SCHDL_APFM_ERROR_R = crate::R<bool, EVT_SCHDL_APFM_ERROR_A>;
impl EVT_SCHDL_APFM_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVT_SCHDL_APFM_ERROR_A {
        match self.bits {
            false => EVT_SCHDL_APFM_ERROR_A::EVT_SCHDL_APFM_ERROR_0,
            true => EVT_SCHDL_APFM_ERROR_A::EVT_SCHDL_APFM_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `EVT_SCHDL_APFM_ERROR_0`"]
    #[inline(always)]
    pub fn is_evt_schdl_apfm_error_0(&self) -> bool {
        *self == EVT_SCHDL_APFM_ERROR_A::EVT_SCHDL_APFM_ERROR_0
    }
    #[doc = "Checks if the value of the field is `EVT_SCHDL_APFM_ERROR_1`"]
    #[inline(always)]
    pub fn is_evt_schdl_apfm_error_1(&self) -> bool {
        *self == EVT_SCHDL_APFM_ERROR_A::EVT_SCHDL_APFM_ERROR_1
    }
}
#[doc = "Indicates event scheduler faced invalid timing programing on two consecutive ET entries (e.g first one with 624us offset and second one with no offset)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVT_SCHDL_ENTRY_ERROR_A {
    #[doc = "0: No error"]
    EVT_SCHDL_ENTRY_ERROR_0 = 0,
    #[doc = "1: Error occurred"]
    EVT_SCHDL_ENTRY_ERROR_1 = 1,
}
impl From<EVT_SCHDL_ENTRY_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: EVT_SCHDL_ENTRY_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVT_SCHDL_ENTRY_ERROR`"]
pub type EVT_SCHDL_ENTRY_ERROR_R = crate::R<bool, EVT_SCHDL_ENTRY_ERROR_A>;
impl EVT_SCHDL_ENTRY_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVT_SCHDL_ENTRY_ERROR_A {
        match self.bits {
            false => EVT_SCHDL_ENTRY_ERROR_A::EVT_SCHDL_ENTRY_ERROR_0,
            true => EVT_SCHDL_ENTRY_ERROR_A::EVT_SCHDL_ENTRY_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `EVT_SCHDL_ENTRY_ERROR_0`"]
    #[inline(always)]
    pub fn is_evt_schdl_entry_error_0(&self) -> bool {
        *self == EVT_SCHDL_ENTRY_ERROR_A::EVT_SCHDL_ENTRY_ERROR_0
    }
    #[doc = "Checks if the value of the field is `EVT_SCHDL_ENTRY_ERROR_1`"]
    #[inline(always)]
    pub fn is_evt_schdl_entry_error_1(&self) -> bool {
        *self == EVT_SCHDL_ENTRY_ERROR_A::EVT_SCHDL_ENTRY_ERROR_1
    }
}
#[doc = "Indicates event scheduler exchange memory access error, happens when exchange memory accesses are not served in time, and blocks the exchange table entry read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVT_SCHDL_EMACC_ERROR_A {
    #[doc = "0: No error"]
    EVT_SCHDL_EMACC_ERROR_0 = 0,
    #[doc = "1: Error occurred"]
    EVT_SCHDL_EMACC_ERROR_1 = 1,
}
impl From<EVT_SCHDL_EMACC_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: EVT_SCHDL_EMACC_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVT_SCHDL_EMACC_ERROR`"]
pub type EVT_SCHDL_EMACC_ERROR_R = crate::R<bool, EVT_SCHDL_EMACC_ERROR_A>;
impl EVT_SCHDL_EMACC_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVT_SCHDL_EMACC_ERROR_A {
        match self.bits {
            false => EVT_SCHDL_EMACC_ERROR_A::EVT_SCHDL_EMACC_ERROR_0,
            true => EVT_SCHDL_EMACC_ERROR_A::EVT_SCHDL_EMACC_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `EVT_SCHDL_EMACC_ERROR_0`"]
    #[inline(always)]
    pub fn is_evt_schdl_emacc_error_0(&self) -> bool {
        *self == EVT_SCHDL_EMACC_ERROR_A::EVT_SCHDL_EMACC_ERROR_0
    }
    #[doc = "Checks if the value of the field is `EVT_SCHDL_EMACC_ERROR_1`"]
    #[inline(always)]
    pub fn is_evt_schdl_emacc_error_1(&self) -> bool {
        *self == EVT_SCHDL_EMACC_ERROR_A::EVT_SCHDL_EMACC_ERROR_1
    }
}
#[doc = "Indicates radio controller exchange memory access error, happens when exchange memory accesses are not served in time and data are corrupted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RADIO_EMACC_ERROR_A {
    #[doc = "0: No error"]
    RADIO_EMACC_ERROR_0 = 0,
    #[doc = "1: Error occurred"]
    RADIO_EMACC_ERROR_1 = 1,
}
impl From<RADIO_EMACC_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: RADIO_EMACC_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RADIO_EMACC_ERROR`"]
pub type RADIO_EMACC_ERROR_R = crate::R<bool, RADIO_EMACC_ERROR_A>;
impl RADIO_EMACC_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RADIO_EMACC_ERROR_A {
        match self.bits {
            false => RADIO_EMACC_ERROR_A::RADIO_EMACC_ERROR_0,
            true => RADIO_EMACC_ERROR_A::RADIO_EMACC_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `RADIO_EMACC_ERROR_0`"]
    #[inline(always)]
    pub fn is_radio_emacc_error_0(&self) -> bool {
        *self == RADIO_EMACC_ERROR_A::RADIO_EMACC_ERROR_0
    }
    #[doc = "Checks if the value of the field is `RADIO_EMACC_ERROR_1`"]
    #[inline(always)]
    pub fn is_radio_emacc_error_1(&self) -> bool {
        *self == RADIO_EMACC_ERROR_A::RADIO_EMACC_ERROR_1
    }
}
#[doc = "Indicates packet controller exchange memory access error, happens when exchange memory accesses are not served in time and Tx/Rx data are corrupted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PKTCNTL_EMACC_ERROR_A {
    #[doc = "0: No error"]
    PKTCNTL_EMACC_ERROR_0 = 0,
    #[doc = "1: Error occurred"]
    PKTCNTL_EMACC_ERROR_1 = 1,
}
impl From<PKTCNTL_EMACC_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: PKTCNTL_EMACC_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PKTCNTL_EMACC_ERROR`"]
pub type PKTCNTL_EMACC_ERROR_R = crate::R<bool, PKTCNTL_EMACC_ERROR_A>;
impl PKTCNTL_EMACC_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PKTCNTL_EMACC_ERROR_A {
        match self.bits {
            false => PKTCNTL_EMACC_ERROR_A::PKTCNTL_EMACC_ERROR_0,
            true => PKTCNTL_EMACC_ERROR_A::PKTCNTL_EMACC_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `PKTCNTL_EMACC_ERROR_0`"]
    #[inline(always)]
    pub fn is_pktcntl_emacc_error_0(&self) -> bool {
        *self == PKTCNTL_EMACC_ERROR_A::PKTCNTL_EMACC_ERROR_0
    }
    #[doc = "Checks if the value of the field is `PKTCNTL_EMACC_ERROR_1`"]
    #[inline(always)]
    pub fn is_pktcntl_emacc_error_1(&self) -> bool {
        *self == PKTCNTL_EMACC_ERROR_A::PKTCNTL_EMACC_ERROR_1
    }
}
#[doc = "Indicates real time decryption error, happens when AES-CCM decryption is too slow compared to packet controller requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCRYPT_ERROR_A {
    #[doc = "0: No error"]
    RXCRYPT_ERROR_0 = 0,
    #[doc = "1: Error occurred"]
    RXCRYPT_ERROR_1 = 1,
}
impl From<RXCRYPT_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: RXCRYPT_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXCRYPT_ERROR`"]
pub type RXCRYPT_ERROR_R = crate::R<bool, RXCRYPT_ERROR_A>;
impl RXCRYPT_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXCRYPT_ERROR_A {
        match self.bits {
            false => RXCRYPT_ERROR_A::RXCRYPT_ERROR_0,
            true => RXCRYPT_ERROR_A::RXCRYPT_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXCRYPT_ERROR_0`"]
    #[inline(always)]
    pub fn is_rxcrypt_error_0(&self) -> bool {
        *self == RXCRYPT_ERROR_A::RXCRYPT_ERROR_0
    }
    #[doc = "Checks if the value of the field is `RXCRYPT_ERROR_1`"]
    #[inline(always)]
    pub fn is_rxcrypt_error_1(&self) -> bool {
        *self == RXCRYPT_ERROR_A::RXCRYPT_ERROR_1
    }
}
#[doc = "Indicates real time encryption error, happens when AES-CCM encryption is too slow compared to packet controller requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCRYPT_ERROR_A {
    #[doc = "0: No error"]
    TXCRYPT_ERROR_0 = 0,
    #[doc = "1: Error occurred"]
    TXCRYPT_ERROR_1 = 1,
}
impl From<TXCRYPT_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: TXCRYPT_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXCRYPT_ERROR`"]
pub type TXCRYPT_ERROR_R = crate::R<bool, TXCRYPT_ERROR_A>;
impl TXCRYPT_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXCRYPT_ERROR_A {
        match self.bits {
            false => TXCRYPT_ERROR_A::TXCRYPT_ERROR_0,
            true => TXCRYPT_ERROR_A::TXCRYPT_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXCRYPT_ERROR_0`"]
    #[inline(always)]
    pub fn is_txcrypt_error_0(&self) -> bool {
        *self == TXCRYPT_ERROR_A::TXCRYPT_ERROR_0
    }
    #[doc = "Checks if the value of the field is `TXCRYPT_ERROR_1`"]
    #[inline(always)]
    pub fn is_txcrypt_error_1(&self) -> bool {
        *self == TXCRYPT_ERROR_A::TXCRYPT_ERROR_1
    }
}
impl R {
    #[doc = "Bit 19 - Indicates Resolving Address List engine Under run issue, happens when RAL List parsing not finished on time"]
    #[inline(always)]
    pub fn ral_underrun(&self) -> RAL_UNDERRUN_R {
        RAL_UNDERRUN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Indicates Resolving Address List engine faced a bad setting"]
    #[inline(always)]
    pub fn ral_error(&self) -> RAL_ERROR_R {
        RAL_ERROR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Indicates whether two consecutive and concurrent ble_event_irq have been generated, and not acknowledged in time by the RW-BLE software"]
    #[inline(always)]
    pub fn concevtirq_error(&self) -> CONCEVTIRQ_ERROR_R {
        CONCEVTIRQ_ERROR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Indicates whether Rx data buffer pointer value programmed is null (major failure)"]
    #[inline(always)]
    pub fn rxdata_ptr_error(&self) -> RXDATA_PTR_ERROR_R {
        RXDATA_PTR_ERROR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Indicates whether Tx data buffer pointer value programmed is null during advertising / scanning / initiating events, or during master / slave connections with non-null packet length (major failure)"]
    #[inline(always)]
    pub fn txdata_ptr_error(&self) -> TXDATA_PTR_ERROR_R {
        TXDATA_PTR_ERROR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Indicates whether Rx descriptor pointer value programmed in register is null (major failure)"]
    #[inline(always)]
    pub fn rxdesc_empty_error(&self) -> RXDESC_EMPTY_ERROR_R {
        RXDESC_EMPTY_ERROR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Indicates whether Tx descriptor pointer value programmed in control structure is null during advertising / scanning / initiating events (major failure)"]
    #[inline(always)]
    pub fn txdesc_empty_error(&self) -> TXDESC_EMPTY_ERROR_R {
        TXDESC_EMPTY_ERROR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Indicates whether CS-FORMAT has been programmed with an invalid value (major failure)"]
    #[inline(always)]
    pub fn csformat_error(&self) -> CSFORMAT_ERROR_R {
        CSFORMAT_ERROR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Indicates Link Layer channel map error, happens when actual number of CS-LLCHMAP bit set to one is different from CS-NBCHGOOD at the beginning of frequency hopping process"]
    #[inline(always)]
    pub fn llchmap_error(&self) -> LLCHMAP_ERROR_R {
        LLCHMAP_ERROR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Indicates advertising interval under run"]
    #[inline(always)]
    pub fn adv_underrun(&self) -> ADV_UNDERRUN_R {
        ADV_UNDERRUN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates inter frame space under run, occurs if IFS time is not enough to update and read control structure / descriptors, and/or white list parsing is not finished and/or decryption time is too long to be finished on time"]
    #[inline(always)]
    pub fn ifs_underrun(&self) -> IFS_UNDERRUN_R {
        IFS_UNDERRUN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Indicates white list timeout error, occurs if white list parsing is not finished on time"]
    #[inline(always)]
    pub fn whitelist_error(&self) -> WHITELIST_ERROR_R {
        WHITELIST_ERROR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Indicates anticipated pre-fetch mechanism error: happens when 2 consecutive events are programmed, and when the first event is not completely finished while second pre-fetch instant is reached"]
    #[inline(always)]
    pub fn evt_cntl_apfm_error(&self) -> EVT_CNTL_APFM_ERROR_R {
        EVT_CNTL_APFM_ERROR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Indicates anticipated pre-fetch mechanism error: happens when 2 consecutive events are programmed, and when the first event is not completely finished while second pre-fetch instant is reached"]
    #[inline(always)]
    pub fn evt_schdl_apfm_error(&self) -> EVT_SCHDL_APFM_ERROR_R {
        EVT_SCHDL_APFM_ERROR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates event scheduler faced invalid timing programing on two consecutive ET entries (e.g first one with 624us offset and second one with no offset)"]
    #[inline(always)]
    pub fn evt_schdl_entry_error(&self) -> EVT_SCHDL_ENTRY_ERROR_R {
        EVT_SCHDL_ENTRY_ERROR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates event scheduler exchange memory access error, happens when exchange memory accesses are not served in time, and blocks the exchange table entry read"]
    #[inline(always)]
    pub fn evt_schdl_emacc_error(&self) -> EVT_SCHDL_EMACC_ERROR_R {
        EVT_SCHDL_EMACC_ERROR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicates radio controller exchange memory access error, happens when exchange memory accesses are not served in time and data are corrupted"]
    #[inline(always)]
    pub fn radio_emacc_error(&self) -> RADIO_EMACC_ERROR_R {
        RADIO_EMACC_ERROR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates packet controller exchange memory access error, happens when exchange memory accesses are not served in time and Tx/Rx data are corrupted"]
    #[inline(always)]
    pub fn pktcntl_emacc_error(&self) -> PKTCNTL_EMACC_ERROR_R {
        PKTCNTL_EMACC_ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates real time decryption error, happens when AES-CCM decryption is too slow compared to packet controller requests"]
    #[inline(always)]
    pub fn rxcrypt_error(&self) -> RXCRYPT_ERROR_R {
        RXCRYPT_ERROR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indicates real time encryption error, happens when AES-CCM encryption is too slow compared to packet controller requests"]
    #[inline(always)]
    pub fn txcrypt_error(&self) -> TXCRYPT_ERROR_R {
        TXCRYPT_ERROR_R::new((self.bits & 0x01) != 0)
    }
}
