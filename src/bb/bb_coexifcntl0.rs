#[doc = "Reader of register BB_COEXIFCNTL0"]
pub type R = crate::R<u32, super::BB_COEXIFCNTL0>;
#[doc = "Writer for register BB_COEXIFCNTL0"]
pub type W = crate::W<u32, super::BB_COEXIFCNTL0>;
#[doc = "Register BB_COEXIFCNTL0 `reset()`'s with value 0x1110"]
impl crate::ResetValue for super::BB_COEXIFCNTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1110
    }
}
#[doc = "Determines how mws_scan_frequency impacts BLE Tx and Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MWSSCANFREQMSK_A {
    #[doc = "0: mws_scan_frequency has no impact"]
    MWSSCANFREQMSK_0 = 0,
    #[doc = "1: mws_scan_frequency can stop BLE Tx, no impact on BLE Rx"]
    MWSSCANFREQMSK_1 = 1,
    #[doc = "2: mws_scan_frequency can stop BLE Rx, no impact on BLE Tx"]
    MWSSCANFREQMSK_2 = 2,
    #[doc = "3: mws_scan_frequency can stop both BLE Tx and BLE Rx"]
    MWSSCANFREQMSK_3 = 3,
}
impl From<MWSSCANFREQMSK_A> for u8 {
    #[inline(always)]
    fn from(variant: MWSSCANFREQMSK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MWSSCANFREQMSK`"]
pub type MWSSCANFREQMSK_R = crate::R<u8, MWSSCANFREQMSK_A>;
impl MWSSCANFREQMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWSSCANFREQMSK_A {
        match self.bits {
            0 => MWSSCANFREQMSK_A::MWSSCANFREQMSK_0,
            1 => MWSSCANFREQMSK_A::MWSSCANFREQMSK_1,
            2 => MWSSCANFREQMSK_A::MWSSCANFREQMSK_2,
            3 => MWSSCANFREQMSK_A::MWSSCANFREQMSK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MWSSCANFREQMSK_0`"]
    #[inline(always)]
    pub fn is_mwsscanfreqmsk_0(&self) -> bool {
        *self == MWSSCANFREQMSK_A::MWSSCANFREQMSK_0
    }
    #[doc = "Checks if the value of the field is `MWSSCANFREQMSK_1`"]
    #[inline(always)]
    pub fn is_mwsscanfreqmsk_1(&self) -> bool {
        *self == MWSSCANFREQMSK_A::MWSSCANFREQMSK_1
    }
    #[doc = "Checks if the value of the field is `MWSSCANFREQMSK_2`"]
    #[inline(always)]
    pub fn is_mwsscanfreqmsk_2(&self) -> bool {
        *self == MWSSCANFREQMSK_A::MWSSCANFREQMSK_2
    }
    #[doc = "Checks if the value of the field is `MWSSCANFREQMSK_3`"]
    #[inline(always)]
    pub fn is_mwsscanfreqmsk_3(&self) -> bool {
        *self == MWSSCANFREQMSK_A::MWSSCANFREQMSK_3
    }
}
#[doc = "Write proxy for field `MWSSCANFREQMSK`"]
pub struct MWSSCANFREQMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> MWSSCANFREQMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MWSSCANFREQMSK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "mws_scan_frequency has no impact"]
    #[inline(always)]
    pub fn mwsscanfreqmsk_0(self) -> &'a mut W {
        self.variant(MWSSCANFREQMSK_A::MWSSCANFREQMSK_0)
    }
    #[doc = "mws_scan_frequency can stop BLE Tx, no impact on BLE Rx"]
    #[inline(always)]
    pub fn mwsscanfreqmsk_1(self) -> &'a mut W {
        self.variant(MWSSCANFREQMSK_A::MWSSCANFREQMSK_1)
    }
    #[doc = "mws_scan_frequency can stop BLE Rx, no impact on BLE Tx"]
    #[inline(always)]
    pub fn mwsscanfreqmsk_2(self) -> &'a mut W {
        self.variant(MWSSCANFREQMSK_A::MWSSCANFREQMSK_2)
    }
    #[doc = "mws_scan_frequency can stop both BLE Tx and BLE Rx"]
    #[inline(always)]
    pub fn mwsscanfreqmsk_3(self) -> &'a mut W {
        self.variant(MWSSCANFREQMSK_A::MWSSCANFREQMSK_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Defines BLE packet ble_rx mode behavior\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WLCRXPRIOMODE_A {
    #[doc = "0: Rx indication excluding Rx power up delay (starts when correlator is enabled)"]
    WLCRXPRIOMODE_0 = 0,
    #[doc = "1: Rx indication including Rx power up delay"]
    WLCRXPRIOMODE_1 = 1,
    #[doc = "2: Rx High priority indicator"]
    WLCRXPRIOMODE_2 = 2,
    #[doc = "3: NA"]
    WLCRXPRIOMODE_3 = 3,
}
impl From<WLCRXPRIOMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WLCRXPRIOMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WLCRXPRIOMODE`"]
pub type WLCRXPRIOMODE_R = crate::R<u8, WLCRXPRIOMODE_A>;
impl WLCRXPRIOMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WLCRXPRIOMODE_A {
        match self.bits {
            0 => WLCRXPRIOMODE_A::WLCRXPRIOMODE_0,
            1 => WLCRXPRIOMODE_A::WLCRXPRIOMODE_1,
            2 => WLCRXPRIOMODE_A::WLCRXPRIOMODE_2,
            3 => WLCRXPRIOMODE_A::WLCRXPRIOMODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WLCRXPRIOMODE_0`"]
    #[inline(always)]
    pub fn is_wlcrxpriomode_0(&self) -> bool {
        *self == WLCRXPRIOMODE_A::WLCRXPRIOMODE_0
    }
    #[doc = "Checks if the value of the field is `WLCRXPRIOMODE_1`"]
    #[inline(always)]
    pub fn is_wlcrxpriomode_1(&self) -> bool {
        *self == WLCRXPRIOMODE_A::WLCRXPRIOMODE_1
    }
    #[doc = "Checks if the value of the field is `WLCRXPRIOMODE_2`"]
    #[inline(always)]
    pub fn is_wlcrxpriomode_2(&self) -> bool {
        *self == WLCRXPRIOMODE_A::WLCRXPRIOMODE_2
    }
    #[doc = "Checks if the value of the field is `WLCRXPRIOMODE_3`"]
    #[inline(always)]
    pub fn is_wlcrxpriomode_3(&self) -> bool {
        *self == WLCRXPRIOMODE_A::WLCRXPRIOMODE_3
    }
}
#[doc = "Write proxy for field `WLCRXPRIOMODE`"]
pub struct WLCRXPRIOMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WLCRXPRIOMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLCRXPRIOMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Rx indication excluding Rx power up delay (starts when correlator is enabled)"]
    #[inline(always)]
    pub fn wlcrxpriomode_0(self) -> &'a mut W {
        self.variant(WLCRXPRIOMODE_A::WLCRXPRIOMODE_0)
    }
    #[doc = "Rx indication including Rx power up delay"]
    #[inline(always)]
    pub fn wlcrxpriomode_1(self) -> &'a mut W {
        self.variant(WLCRXPRIOMODE_A::WLCRXPRIOMODE_1)
    }
    #[doc = "Rx High priority indicator"]
    #[inline(always)]
    pub fn wlcrxpriomode_2(self) -> &'a mut W {
        self.variant(WLCRXPRIOMODE_A::WLCRXPRIOMODE_2)
    }
    #[doc = "NA"]
    #[inline(always)]
    pub fn wlcrxpriomode_3(self) -> &'a mut W {
        self.variant(WLCRXPRIOMODE_A::WLCRXPRIOMODE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Defines BLE packet ble_tx mode behavior\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WLCTXPRIOMODE_A {
    #[doc = "0: Tx indication excluding Tx power up delay"]
    WLCTXPRIOMODE_0 = 0,
    #[doc = "1: Tx indication including Tx power up delay"]
    WLCTXPRIOMODE_1 = 1,
    #[doc = "2: Tx High priority indicator"]
    WLCTXPRIOMODE_2 = 2,
    #[doc = "3: NA"]
    WLCTXPRIOMODE_3 = 3,
}
impl From<WLCTXPRIOMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WLCTXPRIOMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WLCTXPRIOMODE`"]
pub type WLCTXPRIOMODE_R = crate::R<u8, WLCTXPRIOMODE_A>;
impl WLCTXPRIOMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WLCTXPRIOMODE_A {
        match self.bits {
            0 => WLCTXPRIOMODE_A::WLCTXPRIOMODE_0,
            1 => WLCTXPRIOMODE_A::WLCTXPRIOMODE_1,
            2 => WLCTXPRIOMODE_A::WLCTXPRIOMODE_2,
            3 => WLCTXPRIOMODE_A::WLCTXPRIOMODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WLCTXPRIOMODE_0`"]
    #[inline(always)]
    pub fn is_wlctxpriomode_0(&self) -> bool {
        *self == WLCTXPRIOMODE_A::WLCTXPRIOMODE_0
    }
    #[doc = "Checks if the value of the field is `WLCTXPRIOMODE_1`"]
    #[inline(always)]
    pub fn is_wlctxpriomode_1(&self) -> bool {
        *self == WLCTXPRIOMODE_A::WLCTXPRIOMODE_1
    }
    #[doc = "Checks if the value of the field is `WLCTXPRIOMODE_2`"]
    #[inline(always)]
    pub fn is_wlctxpriomode_2(&self) -> bool {
        *self == WLCTXPRIOMODE_A::WLCTXPRIOMODE_2
    }
    #[doc = "Checks if the value of the field is `WLCTXPRIOMODE_3`"]
    #[inline(always)]
    pub fn is_wlctxpriomode_3(&self) -> bool {
        *self == WLCTXPRIOMODE_A::WLCTXPRIOMODE_3
    }
}
#[doc = "Write proxy for field `WLCTXPRIOMODE`"]
pub struct WLCTXPRIOMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WLCTXPRIOMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLCTXPRIOMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Tx indication excluding Tx power up delay"]
    #[inline(always)]
    pub fn wlctxpriomode_0(self) -> &'a mut W {
        self.variant(WLCTXPRIOMODE_A::WLCTXPRIOMODE_0)
    }
    #[doc = "Tx indication including Tx power up delay"]
    #[inline(always)]
    pub fn wlctxpriomode_1(self) -> &'a mut W {
        self.variant(WLCTXPRIOMODE_A::WLCTXPRIOMODE_1)
    }
    #[doc = "Tx High priority indicator"]
    #[inline(always)]
    pub fn wlctxpriomode_2(self) -> &'a mut W {
        self.variant(WLCTXPRIOMODE_A::WLCTXPRIOMODE_2)
    }
    #[doc = "NA"]
    #[inline(always)]
    pub fn wlctxpriomode_3(self) -> &'a mut W {
        self.variant(WLCTXPRIOMODE_A::WLCTXPRIOMODE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Determines how MWS Tx Frequency impacts BLE Tx and Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MWSTXFREQMSK_A {
    #[doc = "0: mws Tx Frequency has no impact"]
    MWSTXFREQMSK_0 = 0,
    #[doc = "1: mws Tx Frequency can stop BLE Tx, no impact on BLE Rx"]
    MWSTXFREQMSK_1 = 1,
    #[doc = "2: mws Tx Frequency can stop BLE Rx, no impact on BLE Tx"]
    MWSTXFREQMSK_2 = 2,
    #[doc = "3: mws Tx Frequency can stop both BLE Tx and BLE Rx"]
    MWSTXFREQMSK_3 = 3,
}
impl From<MWSTXFREQMSK_A> for u8 {
    #[inline(always)]
    fn from(variant: MWSTXFREQMSK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MWSTXFREQMSK`"]
pub type MWSTXFREQMSK_R = crate::R<u8, MWSTXFREQMSK_A>;
impl MWSTXFREQMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWSTXFREQMSK_A {
        match self.bits {
            0 => MWSTXFREQMSK_A::MWSTXFREQMSK_0,
            1 => MWSTXFREQMSK_A::MWSTXFREQMSK_1,
            2 => MWSTXFREQMSK_A::MWSTXFREQMSK_2,
            3 => MWSTXFREQMSK_A::MWSTXFREQMSK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MWSTXFREQMSK_0`"]
    #[inline(always)]
    pub fn is_mwstxfreqmsk_0(&self) -> bool {
        *self == MWSTXFREQMSK_A::MWSTXFREQMSK_0
    }
    #[doc = "Checks if the value of the field is `MWSTXFREQMSK_1`"]
    #[inline(always)]
    pub fn is_mwstxfreqmsk_1(&self) -> bool {
        *self == MWSTXFREQMSK_A::MWSTXFREQMSK_1
    }
    #[doc = "Checks if the value of the field is `MWSTXFREQMSK_2`"]
    #[inline(always)]
    pub fn is_mwstxfreqmsk_2(&self) -> bool {
        *self == MWSTXFREQMSK_A::MWSTXFREQMSK_2
    }
    #[doc = "Checks if the value of the field is `MWSTXFREQMSK_3`"]
    #[inline(always)]
    pub fn is_mwstxfreqmsk_3(&self) -> bool {
        *self == MWSTXFREQMSK_A::MWSTXFREQMSK_3
    }
}
#[doc = "Write proxy for field `MWSTXFREQMSK`"]
pub struct MWSTXFREQMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> MWSTXFREQMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MWSTXFREQMSK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "mws Tx Frequency has no impact"]
    #[inline(always)]
    pub fn mwstxfreqmsk_0(self) -> &'a mut W {
        self.variant(MWSTXFREQMSK_A::MWSTXFREQMSK_0)
    }
    #[doc = "mws Tx Frequency can stop BLE Tx, no impact on BLE Rx"]
    #[inline(always)]
    pub fn mwstxfreqmsk_1(self) -> &'a mut W {
        self.variant(MWSTXFREQMSK_A::MWSTXFREQMSK_1)
    }
    #[doc = "mws Tx Frequency can stop BLE Rx, no impact on BLE Tx"]
    #[inline(always)]
    pub fn mwstxfreqmsk_2(self) -> &'a mut W {
        self.variant(MWSTXFREQMSK_A::MWSTXFREQMSK_2)
    }
    #[doc = "mws Tx Frequency can stop both BLE Tx and BLE Rx"]
    #[inline(always)]
    pub fn mwstxfreqmsk_3(self) -> &'a mut W {
        self.variant(MWSTXFREQMSK_A::MWSTXFREQMSK_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Determines how MWS Rx Frequency impacts BLE Tx and Rx\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MWSRXFREQMSK_A {
    #[doc = "0: mws Tx Frequency has no impact"]
    MWSRXFREQMSK_0 = 0,
    #[doc = "1: mws Tx Frequency can stop BLE Tx, no impact on BLE Rx"]
    MWSRXFREQMSK_1 = 1,
    #[doc = "2: mws Tx Frequency can stop BLE Rx, no impact on BLE Tx"]
    MWSRXFREQMSK_2 = 2,
    #[doc = "3: mws Tx Frequency can stop both BLE Tx and BLE Rx"]
    MWSRXFREQMSK_3 = 3,
}
impl From<MWSRXFREQMSK_A> for u8 {
    #[inline(always)]
    fn from(variant: MWSRXFREQMSK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MWSRXFREQMSK`"]
pub type MWSRXFREQMSK_R = crate::R<u8, MWSRXFREQMSK_A>;
impl MWSRXFREQMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWSRXFREQMSK_A {
        match self.bits {
            0 => MWSRXFREQMSK_A::MWSRXFREQMSK_0,
            1 => MWSRXFREQMSK_A::MWSRXFREQMSK_1,
            2 => MWSRXFREQMSK_A::MWSRXFREQMSK_2,
            3 => MWSRXFREQMSK_A::MWSRXFREQMSK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MWSRXFREQMSK_0`"]
    #[inline(always)]
    pub fn is_mwsrxfreqmsk_0(&self) -> bool {
        *self == MWSRXFREQMSK_A::MWSRXFREQMSK_0
    }
    #[doc = "Checks if the value of the field is `MWSRXFREQMSK_1`"]
    #[inline(always)]
    pub fn is_mwsrxfreqmsk_1(&self) -> bool {
        *self == MWSRXFREQMSK_A::MWSRXFREQMSK_1
    }
    #[doc = "Checks if the value of the field is `MWSRXFREQMSK_2`"]
    #[inline(always)]
    pub fn is_mwsrxfreqmsk_2(&self) -> bool {
        *self == MWSRXFREQMSK_A::MWSRXFREQMSK_2
    }
    #[doc = "Checks if the value of the field is `MWSRXFREQMSK_3`"]
    #[inline(always)]
    pub fn is_mwsrxfreqmsk_3(&self) -> bool {
        *self == MWSRXFREQMSK_A::MWSRXFREQMSK_3
    }
}
#[doc = "Write proxy for field `MWSRXFREQMSK`"]
pub struct MWSRXFREQMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> MWSRXFREQMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MWSRXFREQMSK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "mws Tx Frequency has no impact"]
    #[inline(always)]
    pub fn mwsrxfreqmsk_0(self) -> &'a mut W {
        self.variant(MWSRXFREQMSK_A::MWSRXFREQMSK_0)
    }
    #[doc = "mws Tx Frequency can stop BLE Tx, no impact on BLE Rx"]
    #[inline(always)]
    pub fn mwsrxfreqmsk_1(self) -> &'a mut W {
        self.variant(MWSRXFREQMSK_A::MWSRXFREQMSK_1)
    }
    #[doc = "mws Tx Frequency can stop BLE Rx, no impact on BLE Tx"]
    #[inline(always)]
    pub fn mwsrxfreqmsk_2(self) -> &'a mut W {
        self.variant(MWSRXFREQMSK_A::MWSRXFREQMSK_2)
    }
    #[doc = "mws Tx Frequency can stop both BLE Tx and BLE Rx"]
    #[inline(always)]
    pub fn mwsrxfreqmsk_3(self) -> &'a mut W {
        self.variant(MWSRXFREQMSK_A::MWSRXFREQMSK_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Determines how mws_tx impacts BLE Tx and Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MWSTXMSK_A {
    #[doc = "0: mws_tx has no impact"]
    MWSTXMSK_0 = 0,
    #[doc = "1: mws_tx can stop BLE Tx, no impact on BLE Rx"]
    MWSTXMSK_1 = 1,
    #[doc = "2: mws_tx can stop BLE Rx, no impact on BLE Tx"]
    MWSTXMSK_2 = 2,
    #[doc = "3: mws_tx can stop both BLE Tx and BLE Rx"]
    MWSTXMSK_3 = 3,
}
impl From<MWSTXMSK_A> for u8 {
    #[inline(always)]
    fn from(variant: MWSTXMSK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MWSTXMSK`"]
pub type MWSTXMSK_R = crate::R<u8, MWSTXMSK_A>;
impl MWSTXMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWSTXMSK_A {
        match self.bits {
            0 => MWSTXMSK_A::MWSTXMSK_0,
            1 => MWSTXMSK_A::MWSTXMSK_1,
            2 => MWSTXMSK_A::MWSTXMSK_2,
            3 => MWSTXMSK_A::MWSTXMSK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MWSTXMSK_0`"]
    #[inline(always)]
    pub fn is_mwstxmsk_0(&self) -> bool {
        *self == MWSTXMSK_A::MWSTXMSK_0
    }
    #[doc = "Checks if the value of the field is `MWSTXMSK_1`"]
    #[inline(always)]
    pub fn is_mwstxmsk_1(&self) -> bool {
        *self == MWSTXMSK_A::MWSTXMSK_1
    }
    #[doc = "Checks if the value of the field is `MWSTXMSK_2`"]
    #[inline(always)]
    pub fn is_mwstxmsk_2(&self) -> bool {
        *self == MWSTXMSK_A::MWSTXMSK_2
    }
    #[doc = "Checks if the value of the field is `MWSTXMSK_3`"]
    #[inline(always)]
    pub fn is_mwstxmsk_3(&self) -> bool {
        *self == MWSTXMSK_A::MWSTXMSK_3
    }
}
#[doc = "Write proxy for field `MWSTXMSK`"]
pub struct MWSTXMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> MWSTXMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MWSTXMSK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "mws_tx has no impact"]
    #[inline(always)]
    pub fn mwstxmsk_0(self) -> &'a mut W {
        self.variant(MWSTXMSK_A::MWSTXMSK_0)
    }
    #[doc = "mws_tx can stop BLE Tx, no impact on BLE Rx"]
    #[inline(always)]
    pub fn mwstxmsk_1(self) -> &'a mut W {
        self.variant(MWSTXMSK_A::MWSTXMSK_1)
    }
    #[doc = "mws_tx can stop BLE Rx, no impact on BLE Tx"]
    #[inline(always)]
    pub fn mwstxmsk_2(self) -> &'a mut W {
        self.variant(MWSTXMSK_A::MWSTXMSK_2)
    }
    #[doc = "mws_tx can stop both BLE Tx and BLE Rx"]
    #[inline(always)]
    pub fn mwstxmsk_3(self) -> &'a mut W {
        self.variant(MWSTXMSK_A::MWSTXMSK_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Determines how mws_rx impacts BLE Tx and Rx\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MWSRXMSK_A {
    #[doc = "0: mws_tx has no impact"]
    MWSRXMSK_0 = 0,
    #[doc = "1: mws_tx can stop BLE Tx, no impact on BLE Rx"]
    MWSRXMSK_1 = 1,
    #[doc = "2: mws_tx can stop BLE Rx, no impact on BLE Tx"]
    MWSRXMSK_2 = 2,
    #[doc = "3: mws_tx can stop both BLE Tx and BLE Rx"]
    MWSRXMSK_3 = 3,
}
impl From<MWSRXMSK_A> for u8 {
    #[inline(always)]
    fn from(variant: MWSRXMSK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MWSRXMSK`"]
pub type MWSRXMSK_R = crate::R<u8, MWSRXMSK_A>;
impl MWSRXMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWSRXMSK_A {
        match self.bits {
            0 => MWSRXMSK_A::MWSRXMSK_0,
            1 => MWSRXMSK_A::MWSRXMSK_1,
            2 => MWSRXMSK_A::MWSRXMSK_2,
            3 => MWSRXMSK_A::MWSRXMSK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MWSRXMSK_0`"]
    #[inline(always)]
    pub fn is_mwsrxmsk_0(&self) -> bool {
        *self == MWSRXMSK_A::MWSRXMSK_0
    }
    #[doc = "Checks if the value of the field is `MWSRXMSK_1`"]
    #[inline(always)]
    pub fn is_mwsrxmsk_1(&self) -> bool {
        *self == MWSRXMSK_A::MWSRXMSK_1
    }
    #[doc = "Checks if the value of the field is `MWSRXMSK_2`"]
    #[inline(always)]
    pub fn is_mwsrxmsk_2(&self) -> bool {
        *self == MWSRXMSK_A::MWSRXMSK_2
    }
    #[doc = "Checks if the value of the field is `MWSRXMSK_3`"]
    #[inline(always)]
    pub fn is_mwsrxmsk_3(&self) -> bool {
        *self == MWSRXMSK_A::MWSRXMSK_3
    }
}
#[doc = "Write proxy for field `MWSRXMSK`"]
pub struct MWSRXMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> MWSRXMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MWSRXMSK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "mws_tx has no impact"]
    #[inline(always)]
    pub fn mwsrxmsk_0(self) -> &'a mut W {
        self.variant(MWSRXMSK_A::MWSRXMSK_0)
    }
    #[doc = "mws_tx can stop BLE Tx, no impact on BLE Rx"]
    #[inline(always)]
    pub fn mwsrxmsk_1(self) -> &'a mut W {
        self.variant(MWSRXMSK_A::MWSRXMSK_1)
    }
    #[doc = "mws_tx can stop BLE Rx, no impact on BLE Tx"]
    #[inline(always)]
    pub fn mwsrxmsk_2(self) -> &'a mut W {
        self.variant(MWSRXMSK_A::MWSRXMSK_2)
    }
    #[doc = "mws_tx can stop both BLE Tx and BLE Rx"]
    #[inline(always)]
    pub fn mwsrxmsk_3(self) -> &'a mut W {
        self.variant(MWSRXMSK_A::MWSRXMSK_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Determines how TXx impact BLE Tx and Rx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXMSK_A {
    #[doc = "0: tx has no impact"]
    TXMSK_0 = 0,
    #[doc = "1: tx can stop BLE Tx, no impact on BLE Rx"]
    TXMSK_1 = 1,
    #[doc = "2: tx can stop BLE Rx, no impact on BLE Tx"]
    TXMSK_2 = 2,
    #[doc = "3: tx can stop both BLE Tx and BLE Rx"]
    TXMSK_3 = 3,
}
impl From<TXMSK_A> for u8 {
    #[inline(always)]
    fn from(variant: TXMSK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXMSK`"]
pub type TXMSK_R = crate::R<u8, TXMSK_A>;
impl TXMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXMSK_A {
        match self.bits {
            0 => TXMSK_A::TXMSK_0,
            1 => TXMSK_A::TXMSK_1,
            2 => TXMSK_A::TXMSK_2,
            3 => TXMSK_A::TXMSK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TXMSK_0`"]
    #[inline(always)]
    pub fn is_txmsk_0(&self) -> bool {
        *self == TXMSK_A::TXMSK_0
    }
    #[doc = "Checks if the value of the field is `TXMSK_1`"]
    #[inline(always)]
    pub fn is_txmsk_1(&self) -> bool {
        *self == TXMSK_A::TXMSK_1
    }
    #[doc = "Checks if the value of the field is `TXMSK_2`"]
    #[inline(always)]
    pub fn is_txmsk_2(&self) -> bool {
        *self == TXMSK_A::TXMSK_2
    }
    #[doc = "Checks if the value of the field is `TXMSK_3`"]
    #[inline(always)]
    pub fn is_txmsk_3(&self) -> bool {
        *self == TXMSK_A::TXMSK_3
    }
}
#[doc = "Write proxy for field `TXMSK`"]
pub struct TXMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXMSK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "tx has no impact"]
    #[inline(always)]
    pub fn txmsk_0(self) -> &'a mut W {
        self.variant(TXMSK_A::TXMSK_0)
    }
    #[doc = "tx can stop BLE Tx, no impact on BLE Rx"]
    #[inline(always)]
    pub fn txmsk_1(self) -> &'a mut W {
        self.variant(TXMSK_A::TXMSK_1)
    }
    #[doc = "tx can stop BLE Rx, no impact on BLE Tx"]
    #[inline(always)]
    pub fn txmsk_2(self) -> &'a mut W {
        self.variant(TXMSK_A::TXMSK_2)
    }
    #[doc = "tx can stop both BLE Tx and BLE Rx"]
    #[inline(always)]
    pub fn txmsk_3(self) -> &'a mut W {
        self.variant(TXMSK_A::TXMSK_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Determines how RXx impact BLE Tx and Rx\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXMSK_A {
    #[doc = "0: rx has no impact"]
    RXMSK_0 = 0,
    #[doc = "1: rx can stop BLE Tx, no impact on BLE Rx"]
    RXMSK_1 = 1,
    #[doc = "2: rx can stop BLE Rx, no impact on BLE Tx"]
    RXMSK_2 = 2,
    #[doc = "3: rx can stop both BLE Tx and BLE Rx"]
    RXMSK_3 = 3,
}
impl From<RXMSK_A> for u8 {
    #[inline(always)]
    fn from(variant: RXMSK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXMSK`"]
pub type RXMSK_R = crate::R<u8, RXMSK_A>;
impl RXMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXMSK_A {
        match self.bits {
            0 => RXMSK_A::RXMSK_0,
            1 => RXMSK_A::RXMSK_1,
            2 => RXMSK_A::RXMSK_2,
            3 => RXMSK_A::RXMSK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RXMSK_0`"]
    #[inline(always)]
    pub fn is_rxmsk_0(&self) -> bool {
        *self == RXMSK_A::RXMSK_0
    }
    #[doc = "Checks if the value of the field is `RXMSK_1`"]
    #[inline(always)]
    pub fn is_rxmsk_1(&self) -> bool {
        *self == RXMSK_A::RXMSK_1
    }
    #[doc = "Checks if the value of the field is `RXMSK_2`"]
    #[inline(always)]
    pub fn is_rxmsk_2(&self) -> bool {
        *self == RXMSK_A::RXMSK_2
    }
    #[doc = "Checks if the value of the field is `RXMSK_3`"]
    #[inline(always)]
    pub fn is_rxmsk_3(&self) -> bool {
        *self == RXMSK_A::RXMSK_3
    }
}
#[doc = "Write proxy for field `RXMSK`"]
pub struct RXMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXMSK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "rx has no impact"]
    #[inline(always)]
    pub fn rxmsk_0(self) -> &'a mut W {
        self.variant(RXMSK_A::RXMSK_0)
    }
    #[doc = "rx can stop BLE Tx, no impact on BLE Rx"]
    #[inline(always)]
    pub fn rxmsk_1(self) -> &'a mut W {
        self.variant(RXMSK_A::RXMSK_1)
    }
    #[doc = "rx can stop BLE Rx, no impact on BLE Tx"]
    #[inline(always)]
    pub fn rxmsk_2(self) -> &'a mut W {
        self.variant(RXMSK_A::RXMSK_2)
    }
    #[doc = "rx can stop both BLE Tx and BLE Rx"]
    #[inline(always)]
    pub fn rxmsk_3(self) -> &'a mut W {
        self.variant(RXMSK_A::RXMSK_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Enable / Disable control of the WCI MWS Coexistence interface / Valid in Dual Mode only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWSWCI_EN_A {
    #[doc = "0: MWS WCI Interface disabled"]
    MWSWCI_EN_0 = 0,
    #[doc = "1: MWS WCI Interface enabled"]
    MWSWCI_EN_1 = 1,
}
impl From<MWSWCI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MWSWCI_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MWSWCI_EN`"]
pub type MWSWCI_EN_R = crate::R<bool, MWSWCI_EN_A>;
impl MWSWCI_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWSWCI_EN_A {
        match self.bits {
            false => MWSWCI_EN_A::MWSWCI_EN_0,
            true => MWSWCI_EN_A::MWSWCI_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MWSWCI_EN_0`"]
    #[inline(always)]
    pub fn is_mwswci_en_0(&self) -> bool {
        *self == MWSWCI_EN_A::MWSWCI_EN_0
    }
    #[doc = "Checks if the value of the field is `MWSWCI_EN_1`"]
    #[inline(always)]
    pub fn is_mwswci_en_1(&self) -> bool {
        *self == MWSWCI_EN_A::MWSWCI_EN_1
    }
}
#[doc = "Write proxy for field `MWSWCI_EN`"]
pub struct MWSWCI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MWSWCI_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MWSWCI_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MWS WCI Interface disabled"]
    #[inline(always)]
    pub fn mwswci_en_0(self) -> &'a mut W {
        self.variant(MWSWCI_EN_A::MWSWCI_EN_0)
    }
    #[doc = "MWS WCI Interface enabled"]
    #[inline(always)]
    pub fn mwswci_en_1(self) -> &'a mut W {
        self.variant(MWSWCI_EN_A::MWSWCI_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Enable / Disable control of the MWS Coexistence control / Valid in Dual Mode only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWSCOEX_EN_A {
    #[doc = "0: MWS Coexistence interface disabled"]
    MWSCOEX_EN_0 = 0,
    #[doc = "1: MWS Coexistence interface enabled"]
    MWSCOEX_EN_1 = 1,
}
impl From<MWSCOEX_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MWSCOEX_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MWSCOEX_EN`"]
pub type MWSCOEX_EN_R = crate::R<bool, MWSCOEX_EN_A>;
impl MWSCOEX_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWSCOEX_EN_A {
        match self.bits {
            false => MWSCOEX_EN_A::MWSCOEX_EN_0,
            true => MWSCOEX_EN_A::MWSCOEX_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MWSCOEX_EN_0`"]
    #[inline(always)]
    pub fn is_mwscoex_en_0(&self) -> bool {
        *self == MWSCOEX_EN_A::MWSCOEX_EN_0
    }
    #[doc = "Checks if the value of the field is `MWSCOEX_EN_1`"]
    #[inline(always)]
    pub fn is_mwscoex_en_1(&self) -> bool {
        *self == MWSCOEX_EN_A::MWSCOEX_EN_1
    }
}
#[doc = "Write proxy for field `MWSCOEX_EN`"]
pub struct MWSCOEX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MWSCOEX_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MWSCOEX_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MWS Coexistence interface disabled"]
    #[inline(always)]
    pub fn mwscoex_en_0(self) -> &'a mut W {
        self.variant(MWSCOEX_EN_A::MWSCOEX_EN_0)
    }
    #[doc = "MWS Coexistence interface enabled"]
    #[inline(always)]
    pub fn mwscoex_en_1(self) -> &'a mut W {
        self.variant(MWSCOEX_EN_A::MWSCOEX_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Determines whether ble_sync is generated or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCGEN_EN_A {
    #[doc = "0: ble_sync pulse not generated"]
    SYNCGEN_EN_0 = 0,
    #[doc = "1: ble_sync pulse generated"]
    SYNCGEN_EN_1 = 1,
}
impl From<SYNCGEN_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCGEN_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNCGEN_EN`"]
pub type SYNCGEN_EN_R = crate::R<bool, SYNCGEN_EN_A>;
impl SYNCGEN_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCGEN_EN_A {
        match self.bits {
            false => SYNCGEN_EN_A::SYNCGEN_EN_0,
            true => SYNCGEN_EN_A::SYNCGEN_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCGEN_EN_0`"]
    #[inline(always)]
    pub fn is_syncgen_en_0(&self) -> bool {
        *self == SYNCGEN_EN_A::SYNCGEN_EN_0
    }
    #[doc = "Checks if the value of the field is `SYNCGEN_EN_1`"]
    #[inline(always)]
    pub fn is_syncgen_en_1(&self) -> bool {
        *self == SYNCGEN_EN_A::SYNCGEN_EN_1
    }
}
#[doc = "Write proxy for field `SYNCGEN_EN`"]
pub struct SYNCGEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCGEN_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCGEN_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ble_sync pulse not generated"]
    #[inline(always)]
    pub fn syncgen_en_0(self) -> &'a mut W {
        self.variant(SYNCGEN_EN_A::SYNCGEN_EN_0)
    }
    #[doc = "ble_sync pulse generated"]
    #[inline(always)]
    pub fn syncgen_en_1(self) -> &'a mut W {
        self.variant(SYNCGEN_EN_A::SYNCGEN_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Enable / disable control of the coexistence control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COEX_EN_A {
    #[doc = "0: Coexistence interface disabled"]
    COEX_EN_0 = 0,
    #[doc = "1: Coexistence interface enabled"]
    COEX_EN_1 = 1,
}
impl From<COEX_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COEX_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COEX_EN`"]
pub type COEX_EN_R = crate::R<bool, COEX_EN_A>;
impl COEX_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COEX_EN_A {
        match self.bits {
            false => COEX_EN_A::COEX_EN_0,
            true => COEX_EN_A::COEX_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `COEX_EN_0`"]
    #[inline(always)]
    pub fn is_coex_en_0(&self) -> bool {
        *self == COEX_EN_A::COEX_EN_0
    }
    #[doc = "Checks if the value of the field is `COEX_EN_1`"]
    #[inline(always)]
    pub fn is_coex_en_1(&self) -> bool {
        *self == COEX_EN_A::COEX_EN_1
    }
}
#[doc = "Write proxy for field `COEX_EN`"]
pub struct COEX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COEX_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COEX_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Coexistence interface disabled"]
    #[inline(always)]
    pub fn coex_en_0(self) -> &'a mut W {
        self.variant(COEX_EN_A::COEX_EN_0)
    }
    #[doc = "Coexistence interface enabled"]
    #[inline(always)]
    pub fn coex_en_1(self) -> &'a mut W {
        self.variant(COEX_EN_A::COEX_EN_1)
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
    #[doc = "Bits 24:25 - Determines how mws_scan_frequency impacts BLE Tx and Rx"]
    #[inline(always)]
    pub fn mwsscanfreqmsk(&self) -> MWSSCANFREQMSK_R {
        MWSSCANFREQMSK_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Defines BLE packet ble_rx mode behavior"]
    #[inline(always)]
    pub fn wlcrxpriomode(&self) -> WLCRXPRIOMODE_R {
        WLCRXPRIOMODE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Defines BLE packet ble_tx mode behavior"]
    #[inline(always)]
    pub fn wlctxpriomode(&self) -> WLCTXPRIOMODE_R {
        WLCTXPRIOMODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Determines how MWS Tx Frequency impacts BLE Tx and Rx"]
    #[inline(always)]
    pub fn mwstxfreqmsk(&self) -> MWSTXFREQMSK_R {
        MWSTXFREQMSK_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Determines how MWS Rx Frequency impacts BLE Tx and Rx"]
    #[inline(always)]
    pub fn mwsrxfreqmsk(&self) -> MWSRXFREQMSK_R {
        MWSRXFREQMSK_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Determines how mws_tx impacts BLE Tx and Rx"]
    #[inline(always)]
    pub fn mwstxmsk(&self) -> MWSTXMSK_R {
        MWSTXMSK_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Determines how mws_rx impacts BLE Tx and Rx"]
    #[inline(always)]
    pub fn mwsrxmsk(&self) -> MWSRXMSK_R {
        MWSRXMSK_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Determines how TXx impact BLE Tx and Rx"]
    #[inline(always)]
    pub fn txmsk(&self) -> TXMSK_R {
        TXMSK_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Determines how RXx impact BLE Tx and Rx"]
    #[inline(always)]
    pub fn rxmsk(&self) -> RXMSK_R {
        RXMSK_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Enable / Disable control of the WCI MWS Coexistence interface / Valid in Dual Mode only"]
    #[inline(always)]
    pub fn mwswci_en(&self) -> MWSWCI_EN_R {
        MWSWCI_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable / Disable control of the MWS Coexistence control / Valid in Dual Mode only"]
    #[inline(always)]
    pub fn mwscoex_en(&self) -> MWSCOEX_EN_R {
        MWSCOEX_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Determines whether ble_sync is generated or not"]
    #[inline(always)]
    pub fn syncgen_en(&self) -> SYNCGEN_EN_R {
        SYNCGEN_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable / disable control of the coexistence control"]
    #[inline(always)]
    pub fn coex_en(&self) -> COEX_EN_R {
        COEX_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25 - Determines how mws_scan_frequency impacts BLE Tx and Rx"]
    #[inline(always)]
    pub fn mwsscanfreqmsk(&mut self) -> MWSSCANFREQMSK_W {
        MWSSCANFREQMSK_W { w: self }
    }
    #[doc = "Bits 20:21 - Defines BLE packet ble_rx mode behavior"]
    #[inline(always)]
    pub fn wlcrxpriomode(&mut self) -> WLCRXPRIOMODE_W {
        WLCRXPRIOMODE_W { w: self }
    }
    #[doc = "Bits 16:17 - Defines BLE packet ble_tx mode behavior"]
    #[inline(always)]
    pub fn wlctxpriomode(&mut self) -> WLCTXPRIOMODE_W {
        WLCTXPRIOMODE_W { w: self }
    }
    #[doc = "Bits 14:15 - Determines how MWS Tx Frequency impacts BLE Tx and Rx"]
    #[inline(always)]
    pub fn mwstxfreqmsk(&mut self) -> MWSTXFREQMSK_W {
        MWSTXFREQMSK_W { w: self }
    }
    #[doc = "Bits 12:13 - Determines how MWS Rx Frequency impacts BLE Tx and Rx"]
    #[inline(always)]
    pub fn mwsrxfreqmsk(&mut self) -> MWSRXFREQMSK_W {
        MWSRXFREQMSK_W { w: self }
    }
    #[doc = "Bits 10:11 - Determines how mws_tx impacts BLE Tx and Rx"]
    #[inline(always)]
    pub fn mwstxmsk(&mut self) -> MWSTXMSK_W {
        MWSTXMSK_W { w: self }
    }
    #[doc = "Bits 8:9 - Determines how mws_rx impacts BLE Tx and Rx"]
    #[inline(always)]
    pub fn mwsrxmsk(&mut self) -> MWSRXMSK_W {
        MWSRXMSK_W { w: self }
    }
    #[doc = "Bits 6:7 - Determines how TXx impact BLE Tx and Rx"]
    #[inline(always)]
    pub fn txmsk(&mut self) -> TXMSK_W {
        TXMSK_W { w: self }
    }
    #[doc = "Bits 4:5 - Determines how RXx impact BLE Tx and Rx"]
    #[inline(always)]
    pub fn rxmsk(&mut self) -> RXMSK_W {
        RXMSK_W { w: self }
    }
    #[doc = "Bit 3 - Enable / Disable control of the WCI MWS Coexistence interface / Valid in Dual Mode only"]
    #[inline(always)]
    pub fn mwswci_en(&mut self) -> MWSWCI_EN_W {
        MWSWCI_EN_W { w: self }
    }
    #[doc = "Bit 2 - Enable / Disable control of the MWS Coexistence control / Valid in Dual Mode only"]
    #[inline(always)]
    pub fn mwscoex_en(&mut self) -> MWSCOEX_EN_W {
        MWSCOEX_EN_W { w: self }
    }
    #[doc = "Bit 1 - Determines whether ble_sync is generated or not"]
    #[inline(always)]
    pub fn syncgen_en(&mut self) -> SYNCGEN_EN_W {
        SYNCGEN_EN_W { w: self }
    }
    #[doc = "Bit 0 - Enable / disable control of the coexistence control"]
    #[inline(always)]
    pub fn coex_en(&mut self) -> COEX_EN_W {
        COEX_EN_W { w: self }
    }
}
