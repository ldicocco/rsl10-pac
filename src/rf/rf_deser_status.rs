#[doc = "Reader of register RF_DESER_STATUS"]
pub type R = crate::R<u32, super::RF_DESER_STATUS>;
#[doc = "Reader of field `DESER_STATUS_SIGNAL_RECEIVING`"]
pub type DESER_STATUS_SIGNAL_RECEIVING_R = crate::R<bool, bool>;
#[doc = "Reader of field `DESER_STATUS_SYNC_DETECTED`"]
pub type DESER_STATUS_SYNC_DETECTED_R = crate::R<bool, bool>;
#[doc = "Reader of field `DESER_STATUS_WAIT_SYNC`"]
pub type DESER_STATUS_WAIT_SYNC_R = crate::R<bool, bool>;
#[doc = "Reader of field `DESER_STATUS_IS_ADDRESS_BR`"]
pub type DESER_STATUS_IS_ADDRESS_BR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DESER_STATUS_PKT_LEN_ERR`"]
pub type DESER_STATUS_PKT_LEN_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DESER_STATUS_ADDRESS_ERR`"]
pub type DESER_STATUS_ADDRESS_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DESER_STATUS_CRC_ERR`"]
pub type DESER_STATUS_CRC_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DESER_STATUS_DESER_FINISH`"]
pub type DESER_STATUS_DESER_FINISH_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 7 - Is set to 1 if the deserializer is on"]
    #[inline(always)]
    pub fn deser_status_signal_receiving(&self) -> DESER_STATUS_SIGNAL_RECEIVING_R {
        DESER_STATUS_SIGNAL_RECEIVING_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Is set to 1 is the sync word (pattern) has been detected"]
    #[inline(always)]
    pub fn deser_status_sync_detected(&self) -> DESER_STATUS_SYNC_DETECTED_R {
        DESER_STATUS_SYNC_DETECTED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Is set to 1 if the deserializer is waiting the sync word"]
    #[inline(always)]
    pub fn deser_status_wait_sync(&self) -> DESER_STATUS_WAIT_SYNC_R {
        DESER_STATUS_WAIT_SYNC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Is set to 1 if the received address is the broadcast address."]
    #[inline(always)]
    pub fn deser_status_is_address_br(&self) -> DESER_STATUS_IS_ADDRESS_BR_R {
        DESER_STATUS_IS_ADDRESS_BR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Is set to 1 in case of the packet length is longer than the maximum acceptable packet length"]
    #[inline(always)]
    pub fn deser_status_pkt_len_err(&self) -> DESER_STATUS_PKT_LEN_ERR_R {
        DESER_STATUS_PKT_LEN_ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Is set to 1 in case of an address error"]
    #[inline(always)]
    pub fn deser_status_address_err(&self) -> DESER_STATUS_ADDRESS_ERR_R {
        DESER_STATUS_ADDRESS_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Is set to 1 in case of a CRC error"]
    #[inline(always)]
    pub fn deser_status_crc_err(&self) -> DESER_STATUS_CRC_ERR_R {
        DESER_STATUS_CRC_ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Is set to 1 when the deserializer has finished"]
    #[inline(always)]
    pub fn deser_status_deser_finish(&self) -> DESER_STATUS_DESER_FINISH_R {
        DESER_STATUS_DESER_FINISH_R::new((self.bits & 0x01) != 0)
    }
}
