#[doc = "Reader of register RF_REG32"]
pub type R = crate::R<u32, super::RF_REG32>;
#[doc = "Reader of field `RX_ATT_LEVEL_RX_ATT_LEVEL_PKT_LVL`"]
pub type RX_ATT_LEVEL_RX_ATT_LEVEL_PKT_LVL_R = crate::R<u8, u8>;
#[doc = "Reader of field `RX_ATT_LEVEL_RX_ATT_LEVEL`"]
pub type RX_ATT_LEVEL_RX_ATT_LEVEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `RSSI_AVG_RSSI_AVG`"]
pub type RSSI_AVG_RSSI_AVG_R = crate::R<u8, u8>;
#[doc = "Reader of field `DR_ERR_IND_DR_ERR_IND`"]
pub type DR_ERR_IND_DR_ERR_IND_R = crate::R<u8, u8>;
#[doc = "Reader of field `RSSI_PKT_RSSI_PKT`"]
pub type RSSI_PKT_RSSI_PKT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 28:30 - Rx attenuation level (AGC level) during the packet reception"]
    #[inline(always)]
    pub fn rx_att_level_rx_att_level_pkt_lvl(&self) -> RX_ATT_LEVEL_RX_ATT_LEVEL_PKT_LVL_R {
        RX_ATT_LEVEL_RX_ATT_LEVEL_PKT_LVL_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Rx attenuation level (AGC level)"]
    #[inline(always)]
    pub fn rx_att_level_rx_att_level(&self) -> RX_ATT_LEVEL_RX_ATT_LEVEL_R {
        RX_ATT_LEVEL_RX_ATT_LEVEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:23 - Filtered RSSI value"]
    #[inline(always)]
    pub fn rssi_avg_rssi_avg(&self) -> RSSI_AVG_RSSI_AVG_R {
        RSSI_AVG_RSSI_AVG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data-rate error indicator"]
    #[inline(always)]
    pub fn dr_err_ind_dr_err_ind(&self) -> DR_ERR_IND_DR_ERR_IND_R {
        DR_ERR_IND_DR_ERR_IND_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Filtered RSSI value sampled during the packet reception"]
    #[inline(always)]
    pub fn rssi_pkt_rssi_pkt(&self) -> RSSI_PKT_RSSI_PKT_R {
        RSSI_PKT_RSSI_PKT_R::new((self.bits & 0xff) as u8)
    }
}
