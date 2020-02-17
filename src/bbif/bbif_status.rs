#[doc = "Reader of register BBIF_STATUS"]
pub type R = crate::R<u32, super::BBIF_STATUS>;
#[doc = "BLE link format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINK_FORMAT_A {
    #[doc = "0: Reset value"]
    LINK_FORMAT_RESET = 0,
    #[doc = "2: Master connect"]
    MASTER_CONNECT = 2,
    #[doc = "3: Slave connect"]
    SLAVE_CONNECT = 3,
    #[doc = "4: Low Duty Cycle Advertiser"]
    LOW_DUTY_ADVERTISER = 4,
    #[doc = "5: High Duty Cycle Advertiser"]
    HIGH_DUTY_ADVERTISER = 5,
    #[doc = "8: Passive Scanner"]
    PASSIVE_SCANNER = 8,
    #[doc = "9: Active Scanner"]
    ACTIVE_SCANNER = 9,
    #[doc = "15: Initiator"]
    INITIATOR = 15,
    #[doc = "28: Tx Test Mode"]
    TX_TEST_MODE = 28,
    #[doc = "29: Rx Test Mode"]
    RX_TEST_MODE = 29,
    #[doc = "30: Tx / Rx Test Mode"]
    TX_RX_TEST_MODE = 30,
}
impl From<LINK_FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: LINK_FORMAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LINK_FORMAT`"]
pub type LINK_FORMAT_R = crate::R<u8, LINK_FORMAT_A>;
impl LINK_FORMAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LINK_FORMAT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LINK_FORMAT_A::LINK_FORMAT_RESET),
            2 => Val(LINK_FORMAT_A::MASTER_CONNECT),
            3 => Val(LINK_FORMAT_A::SLAVE_CONNECT),
            4 => Val(LINK_FORMAT_A::LOW_DUTY_ADVERTISER),
            5 => Val(LINK_FORMAT_A::HIGH_DUTY_ADVERTISER),
            8 => Val(LINK_FORMAT_A::PASSIVE_SCANNER),
            9 => Val(LINK_FORMAT_A::ACTIVE_SCANNER),
            15 => Val(LINK_FORMAT_A::INITIATOR),
            28 => Val(LINK_FORMAT_A::TX_TEST_MODE),
            29 => Val(LINK_FORMAT_A::RX_TEST_MODE),
            30 => Val(LINK_FORMAT_A::TX_RX_TEST_MODE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LINK_FORMAT_RESET`"]
    #[inline(always)]
    pub fn is_link_format_reset(&self) -> bool {
        *self == LINK_FORMAT_A::LINK_FORMAT_RESET
    }
    #[doc = "Checks if the value of the field is `MASTER_CONNECT`"]
    #[inline(always)]
    pub fn is_master_connect(&self) -> bool {
        *self == LINK_FORMAT_A::MASTER_CONNECT
    }
    #[doc = "Checks if the value of the field is `SLAVE_CONNECT`"]
    #[inline(always)]
    pub fn is_slave_connect(&self) -> bool {
        *self == LINK_FORMAT_A::SLAVE_CONNECT
    }
    #[doc = "Checks if the value of the field is `LOW_DUTY_ADVERTISER`"]
    #[inline(always)]
    pub fn is_low_duty_advertiser(&self) -> bool {
        *self == LINK_FORMAT_A::LOW_DUTY_ADVERTISER
    }
    #[doc = "Checks if the value of the field is `HIGH_DUTY_ADVERTISER`"]
    #[inline(always)]
    pub fn is_high_duty_advertiser(&self) -> bool {
        *self == LINK_FORMAT_A::HIGH_DUTY_ADVERTISER
    }
    #[doc = "Checks if the value of the field is `PASSIVE_SCANNER`"]
    #[inline(always)]
    pub fn is_passive_scanner(&self) -> bool {
        *self == LINK_FORMAT_A::PASSIVE_SCANNER
    }
    #[doc = "Checks if the value of the field is `ACTIVE_SCANNER`"]
    #[inline(always)]
    pub fn is_active_scanner(&self) -> bool {
        *self == LINK_FORMAT_A::ACTIVE_SCANNER
    }
    #[doc = "Checks if the value of the field is `INITIATOR`"]
    #[inline(always)]
    pub fn is_initiator(&self) -> bool {
        *self == LINK_FORMAT_A::INITIATOR
    }
    #[doc = "Checks if the value of the field is `TX_TEST_MODE`"]
    #[inline(always)]
    pub fn is_tx_test_mode(&self) -> bool {
        *self == LINK_FORMAT_A::TX_TEST_MODE
    }
    #[doc = "Checks if the value of the field is `RX_TEST_MODE`"]
    #[inline(always)]
    pub fn is_rx_test_mode(&self) -> bool {
        *self == LINK_FORMAT_A::RX_TEST_MODE
    }
    #[doc = "Checks if the value of the field is `TX_RX_TEST_MODE`"]
    #[inline(always)]
    pub fn is_tx_rx_test_mode(&self) -> bool {
        *self == LINK_FORMAT_A::TX_RX_TEST_MODE
    }
}
#[doc = "Reader of field `LINK_LABEL`"]
pub type LINK_LABEL_R = crate::R<u8, u8>;
#[doc = "Audio over BLE feature status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AOBLE_STATUS_A {
    #[doc = "0: Device does not have the Audio over BLE feature"]
    AOBLE_DISABLED_DEVICE = 0,
    #[doc = "1: Device has the Audio over BLE feature"]
    AOBLE_ENABLED_DEVICE = 1,
}
impl From<AOBLE_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: AOBLE_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AOBLE_STATUS`"]
pub type AOBLE_STATUS_R = crate::R<bool, AOBLE_STATUS_A>;
impl AOBLE_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AOBLE_STATUS_A {
        match self.bits {
            false => AOBLE_STATUS_A::AOBLE_DISABLED_DEVICE,
            true => AOBLE_STATUS_A::AOBLE_ENABLED_DEVICE,
        }
    }
    #[doc = "Checks if the value of the field is `AOBLE_DISABLED_DEVICE`"]
    #[inline(always)]
    pub fn is_aoble_disabled_device(&self) -> bool {
        *self == AOBLE_STATUS_A::AOBLE_DISABLED_DEVICE
    }
    #[doc = "Checks if the value of the field is `AOBLE_ENABLED_DEVICE`"]
    #[inline(always)]
    pub fn is_aoble_enabled_device(&self) -> bool {
        *self == AOBLE_STATUS_A::AOBLE_ENABLED_DEVICE
    }
}
#[doc = "Clock status defining the current active clock in use\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_STATUS_A {
    #[doc = "0: Baseband controller running on master1/2_clk"]
    MASTER_CLK = 0,
    #[doc = "1: Baseband controller running on low_power_clk"]
    LOW_POWER_CLK = 1,
}
impl From<CLK_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLK_STATUS`"]
pub type CLK_STATUS_R = crate::R<bool, CLK_STATUS_A>;
impl CLK_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_STATUS_A {
        match self.bits {
            false => CLK_STATUS_A::MASTER_CLK,
            true => CLK_STATUS_A::LOW_POWER_CLK,
        }
    }
    #[doc = "Checks if the value of the field is `MASTER_CLK`"]
    #[inline(always)]
    pub fn is_master_clk(&self) -> bool {
        *self == CLK_STATUS_A::MASTER_CLK
    }
    #[doc = "Checks if the value of the field is `LOW_POWER_CLK`"]
    #[inline(always)]
    pub fn is_low_power_clk(&self) -> bool {
        *self == CLK_STATUS_A::LOW_POWER_CLK
    }
}
#[doc = "Oscillator front-end enabling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC_EN_A {
    #[doc = "0: Oscillator can be safely disabled"]
    OSC_DISABLED = 0,
    #[doc = "1: Oscillator must be enabled"]
    OSC_ENABLED = 1,
}
impl From<OSC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OSC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OSC_EN`"]
pub type OSC_EN_R = crate::R<bool, OSC_EN_A>;
impl OSC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSC_EN_A {
        match self.bits {
            false => OSC_EN_A::OSC_DISABLED,
            true => OSC_EN_A::OSC_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `OSC_DISABLED`"]
    #[inline(always)]
    pub fn is_osc_disabled(&self) -> bool {
        *self == OSC_EN_A::OSC_DISABLED
    }
    #[doc = "Checks if the value of the field is `OSC_ENABLED`"]
    #[inline(always)]
    pub fn is_osc_enabled(&self) -> bool {
        *self == OSC_EN_A::OSC_ENABLED
    }
}
#[doc = "RF front-end enabling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RADIO_EN_A {
    #[doc = "0: RF front-end can be safely disabled"]
    RF_DISABLED = 0,
    #[doc = "1: RF front-end must be enabled"]
    RF_ENABLED = 1,
}
impl From<RADIO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RADIO_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RADIO_EN`"]
pub type RADIO_EN_R = crate::R<bool, RADIO_EN_A>;
impl RADIO_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RADIO_EN_A {
        match self.bits {
            false => RADIO_EN_A::RF_DISABLED,
            true => RADIO_EN_A::RF_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `RF_DISABLED`"]
    #[inline(always)]
    pub fn is_rf_disabled(&self) -> bool {
        *self == RADIO_EN_A::RF_DISABLED
    }
    #[doc = "Checks if the value of the field is `RF_ENABLED`"]
    #[inline(always)]
    pub fn is_rf_enabled(&self) -> bool {
        *self == RADIO_EN_A::RF_ENABLED
    }
}
impl R {
    #[doc = "Bits 11:15 - BLE link format"]
    #[inline(always)]
    pub fn link_format(&self) -> LINK_FORMAT_R {
        LINK_FORMAT_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 4:8 - BLE link label"]
    #[inline(always)]
    pub fn link_label(&self) -> LINK_LABEL_R {
        LINK_LABEL_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 3 - Audio over BLE feature status"]
    #[inline(always)]
    pub fn aoble_status(&self) -> AOBLE_STATUS_R {
        AOBLE_STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clock status defining the current active clock in use"]
    #[inline(always)]
    pub fn clk_status(&self) -> CLK_STATUS_R {
        CLK_STATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Oscillator front-end enabling"]
    #[inline(always)]
    pub fn osc_en(&self) -> OSC_EN_R {
        OSC_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RF front-end enabling"]
    #[inline(always)]
    pub fn radio_en(&self) -> RADIO_EN_R {
        RADIO_EN_R::new((self.bits & 0x01) != 0)
    }
}
