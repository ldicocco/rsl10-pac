#[doc = "Reader of register BBIF_SYNC_CFG"]
pub type R = crate::R<u32, super::BBIF_SYNC_CFG>;
#[doc = "Writer for register BBIF_SYNC_CFG"]
pub type W = crate::W<u32, super::BBIF_SYNC_CFG>;
#[doc = "Register BBIF_SYNC_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::BBIF_SYNC_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Specify if the RF front-end is currently receiving the audio link\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_RX_A {
    #[doc = "0: No audio link is currently received by the RF front-end"]
    RX_IDLE = 0,
    #[doc = "1: The audio link is currently received by the RF front-end"]
    RX_ACTIVE = 1,
}
impl From<RF_RX_A> for bool {
    #[inline(always)]
    fn from(variant: RF_RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RF_RX`"]
pub type RF_RX_R = crate::R<bool, RF_RX_A>;
impl RF_RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_RX_A {
        match self.bits {
            false => RF_RX_A::RX_IDLE,
            true => RF_RX_A::RX_ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `RX_IDLE`"]
    #[inline(always)]
    pub fn is_rx_idle(&self) -> bool {
        *self == RF_RX_A::RX_IDLE
    }
    #[doc = "Checks if the value of the field is `RX_ACTIVE`"]
    #[inline(always)]
    pub fn is_rx_active(&self) -> bool {
        *self == RF_RX_A::RX_ACTIVE
    }
}
#[doc = "Write proxy for field `RF_RX`"]
pub struct RF_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF_RX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No audio link is currently received by the RF front-end"]
    #[inline(always)]
    pub fn rx_idle(self) -> &'a mut W {
        self.variant(RF_RX_A::RX_IDLE)
    }
    #[doc = "The audio link is currently received by the RF front-end"]
    #[inline(always)]
    pub fn rx_active(self) -> &'a mut W {
        self.variant(RF_RX_A::RX_ACTIVE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Specify if the RF front-end is currently processing the audio link\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_ACTIVE_A {
    #[doc = "0: No audio link is currently processed by the RF front-end"]
    IDLE = 0,
    #[doc = "1: The audio link is currently processed by the RF front-end"]
    ACTIVE = 1,
}
impl From<RF_ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: RF_ACTIVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RF_ACTIVE`"]
pub type RF_ACTIVE_R = crate::R<bool, RF_ACTIVE_A>;
impl RF_ACTIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_ACTIVE_A {
        match self.bits {
            false => RF_ACTIVE_A::IDLE,
            true => RF_ACTIVE_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == RF_ACTIVE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == RF_ACTIVE_A::ACTIVE
    }
}
#[doc = "Write proxy for field `RF_ACTIVE`"]
pub struct RF_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ACTIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF_ACTIVE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No audio link is currently processed by the RF front-end"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(RF_ACTIVE_A::IDLE)
    }
    #[doc = "The audio link is currently processed by the RF front-end"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(RF_ACTIVE_A::ACTIVE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `LINK_FORMAT`"]
pub type LINK_FORMAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LINK_FORMAT`"]
pub struct LINK_FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_FORMAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `LINK_LABEL`"]
pub type LINK_LABEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LINK_LABEL`"]
pub struct LINK_LABEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_LABEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
#[doc = "Select the BLE/RF link synchronization source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SOURCE_A {
    #[doc = "0: Use the BLE bb_sync_p signal as synchronization source"]
    SYNC_SOURCE_BLE_RX = 0,
    #[doc = "1: Use the BLE audio0_sync_p signal as synchronization source"]
    SYNC_SOURCE_BLE_RX_AUDIO0 = 1,
    #[doc = "2: Use the BLE audio1_sync_p signal as synchronization source"]
    SYNC_SOURCE_BLE_RX_AUDIO1 = 2,
    #[doc = "3: Use the BLE audio2_sync_p signal as synchronization source"]
    SYNC_SOURCE_BLE_RX_AUDIO2 = 3,
    #[doc = "4: Use the RF front-end rf_sync_p signal as synchronization source"]
    SYNC_SOURCE_RF_RX = 4,
    #[doc = "5: Use the BLE tx_en signal as synchronization source"]
    SYNC_SOURCE_BLE_TX = 5,
}
impl From<SOURCE_A> for u8 {
    #[inline(always)]
    fn from(variant: SOURCE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SOURCE`"]
pub type SOURCE_R = crate::R<u8, SOURCE_A>;
impl SOURCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SOURCE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SOURCE_A::SYNC_SOURCE_BLE_RX),
            1 => Val(SOURCE_A::SYNC_SOURCE_BLE_RX_AUDIO0),
            2 => Val(SOURCE_A::SYNC_SOURCE_BLE_RX_AUDIO1),
            3 => Val(SOURCE_A::SYNC_SOURCE_BLE_RX_AUDIO2),
            4 => Val(SOURCE_A::SYNC_SOURCE_RF_RX),
            5 => Val(SOURCE_A::SYNC_SOURCE_BLE_TX),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYNC_SOURCE_BLE_RX`"]
    #[inline(always)]
    pub fn is_sync_source_ble_rx(&self) -> bool {
        *self == SOURCE_A::SYNC_SOURCE_BLE_RX
    }
    #[doc = "Checks if the value of the field is `SYNC_SOURCE_BLE_RX_AUDIO0`"]
    #[inline(always)]
    pub fn is_sync_source_ble_rx_audio0(&self) -> bool {
        *self == SOURCE_A::SYNC_SOURCE_BLE_RX_AUDIO0
    }
    #[doc = "Checks if the value of the field is `SYNC_SOURCE_BLE_RX_AUDIO1`"]
    #[inline(always)]
    pub fn is_sync_source_ble_rx_audio1(&self) -> bool {
        *self == SOURCE_A::SYNC_SOURCE_BLE_RX_AUDIO1
    }
    #[doc = "Checks if the value of the field is `SYNC_SOURCE_BLE_RX_AUDIO2`"]
    #[inline(always)]
    pub fn is_sync_source_ble_rx_audio2(&self) -> bool {
        *self == SOURCE_A::SYNC_SOURCE_BLE_RX_AUDIO2
    }
    #[doc = "Checks if the value of the field is `SYNC_SOURCE_RF_RX`"]
    #[inline(always)]
    pub fn is_sync_source_rf_rx(&self) -> bool {
        *self == SOURCE_A::SYNC_SOURCE_RF_RX
    }
    #[doc = "Checks if the value of the field is `SYNC_SOURCE_BLE_TX`"]
    #[inline(always)]
    pub fn is_sync_source_ble_tx(&self) -> bool {
        *self == SOURCE_A::SYNC_SOURCE_BLE_TX
    }
}
#[doc = "Write proxy for field `SOURCE`"]
pub struct SOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOURCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOURCE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Use the BLE bb_sync_p signal as synchronization source"]
    #[inline(always)]
    pub fn sync_source_ble_rx(self) -> &'a mut W {
        self.variant(SOURCE_A::SYNC_SOURCE_BLE_RX)
    }
    #[doc = "Use the BLE audio0_sync_p signal as synchronization source"]
    #[inline(always)]
    pub fn sync_source_ble_rx_audio0(self) -> &'a mut W {
        self.variant(SOURCE_A::SYNC_SOURCE_BLE_RX_AUDIO0)
    }
    #[doc = "Use the BLE audio1_sync_p signal as synchronization source"]
    #[inline(always)]
    pub fn sync_source_ble_rx_audio1(self) -> &'a mut W {
        self.variant(SOURCE_A::SYNC_SOURCE_BLE_RX_AUDIO1)
    }
    #[doc = "Use the BLE audio2_sync_p signal as synchronization source"]
    #[inline(always)]
    pub fn sync_source_ble_rx_audio2(self) -> &'a mut W {
        self.variant(SOURCE_A::SYNC_SOURCE_BLE_RX_AUDIO2)
    }
    #[doc = "Use the RF front-end rf_sync_p signal as synchronization source"]
    #[inline(always)]
    pub fn sync_source_rf_rx(self) -> &'a mut W {
        self.variant(SOURCE_A::SYNC_SOURCE_RF_RX)
    }
    #[doc = "Use the BLE tx_en signal as synchronization source"]
    #[inline(always)]
    pub fn sync_source_ble_tx(self) -> &'a mut W {
        self.variant(SOURCE_A::SYNC_SOURCE_BLE_TX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Enable the frame synchronization pulse filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Disable the frame synchronization pulse filter"]
    SYNC_DISABLE = 0,
    #[doc = "1: Enable the frame synchronization pulse filter"]
    SYNC_ENABLE = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::SYNC_DISABLE,
            true => ENABLE_A::SYNC_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `SYNC_DISABLE`"]
    #[inline(always)]
    pub fn is_sync_disable(&self) -> bool {
        *self == ENABLE_A::SYNC_DISABLE
    }
    #[doc = "Checks if the value of the field is `SYNC_ENABLE`"]
    #[inline(always)]
    pub fn is_sync_enable(&self) -> bool {
        *self == ENABLE_A::SYNC_ENABLE
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the frame synchronization pulse filter"]
    #[inline(always)]
    pub fn sync_disable(self) -> &'a mut W {
        self.variant(ENABLE_A::SYNC_DISABLE)
    }
    #[doc = "Enable the frame synchronization pulse filter"]
    #[inline(always)]
    pub fn sync_enable(self) -> &'a mut W {
        self.variant(ENABLE_A::SYNC_ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 17 - Specify if the RF front-end is currently receiving the audio link"]
    #[inline(always)]
    pub fn rf_rx(&self) -> RF_RX_R {
        RF_RX_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Specify if the RF front-end is currently processing the audio link"]
    #[inline(always)]
    pub fn rf_active(&self) -> RF_ACTIVE_R {
        RF_ACTIVE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:15 - Configure the BLE link format for synchronization"]
    #[inline(always)]
    pub fn link_format(&self) -> LINK_FORMAT_R {
        LINK_FORMAT_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 4:8 - Configure the BLE link label for synchronization"]
    #[inline(always)]
    pub fn link_label(&self) -> LINK_LABEL_R {
        LINK_LABEL_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 1:3 - Select the BLE/RF link synchronization source"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - Enable the frame synchronization pulse filter"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Specify if the RF front-end is currently receiving the audio link"]
    #[inline(always)]
    pub fn rf_rx(&mut self) -> RF_RX_W {
        RF_RX_W { w: self }
    }
    #[doc = "Bit 16 - Specify if the RF front-end is currently processing the audio link"]
    #[inline(always)]
    pub fn rf_active(&mut self) -> RF_ACTIVE_W {
        RF_ACTIVE_W { w: self }
    }
    #[doc = "Bits 11:15 - Configure the BLE link format for synchronization"]
    #[inline(always)]
    pub fn link_format(&mut self) -> LINK_FORMAT_W {
        LINK_FORMAT_W { w: self }
    }
    #[doc = "Bits 4:8 - Configure the BLE link label for synchronization"]
    #[inline(always)]
    pub fn link_label(&mut self) -> LINK_LABEL_W {
        LINK_LABEL_W { w: self }
    }
    #[doc = "Bits 1:3 - Select the BLE/RF link synchronization source"]
    #[inline(always)]
    pub fn source(&mut self) -> SOURCE_W {
        SOURCE_W { w: self }
    }
    #[doc = "Bit 0 - Enable the frame synchronization pulse filter"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
