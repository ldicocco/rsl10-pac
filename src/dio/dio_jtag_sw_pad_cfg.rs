#[doc = "Reader of register DIO_JTAG_SW_PAD_CFG"]
pub type R = crate::R<u32, super::DIO_JTAG_SW_PAD_CFG>;
#[doc = "Writer for register DIO_JTAG_SW_PAD_CFG"]
pub type W = crate::W<u32, super::DIO_JTAG_SW_PAD_CFG>;
#[doc = "Register DIO_JTAG_SW_PAD_CFG `reset()`'s with value 0xdd"]
impl crate::ResetValue for super::DIO_JTAG_SW_PAD_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xdd
    }
}
#[doc = "JTCK Low-Pass-Filter enable / disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JTCK_LPF_A {
    #[doc = "0: Disable low pass filter"]
    JTCK_LPF_DISABLED = 0,
    #[doc = "1: Enable low pass filter"]
    JTCK_LPF_ENABLED = 1,
}
impl From<JTCK_LPF_A> for bool {
    #[inline(always)]
    fn from(variant: JTCK_LPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JTCK_LPF`"]
pub type JTCK_LPF_R = crate::R<bool, JTCK_LPF_A>;
impl JTCK_LPF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JTCK_LPF_A {
        match self.bits {
            false => JTCK_LPF_A::JTCK_LPF_DISABLED,
            true => JTCK_LPF_A::JTCK_LPF_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `JTCK_LPF_DISABLED`"]
    #[inline(always)]
    pub fn is_jtck_lpf_disabled(&self) -> bool {
        *self == JTCK_LPF_A::JTCK_LPF_DISABLED
    }
    #[doc = "Checks if the value of the field is `JTCK_LPF_ENABLED`"]
    #[inline(always)]
    pub fn is_jtck_lpf_enabled(&self) -> bool {
        *self == JTCK_LPF_A::JTCK_LPF_ENABLED
    }
}
#[doc = "Write proxy for field `JTCK_LPF`"]
pub struct JTCK_LPF_W<'a> {
    w: &'a mut W,
}
impl<'a> JTCK_LPF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JTCK_LPF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable low pass filter"]
    #[inline(always)]
    pub fn jtck_lpf_disabled(self) -> &'a mut W {
        self.variant(JTCK_LPF_A::JTCK_LPF_DISABLED)
    }
    #[doc = "Enable low pass filter"]
    #[inline(always)]
    pub fn jtck_lpf_enabled(self) -> &'a mut W {
        self.variant(JTCK_LPF_A::JTCK_LPF_ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "JTMS Low-Pass-Filter enable / disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JTMS_LPF_A {
    #[doc = "0: Disable low pass filter"]
    JTMS_LPF_DISABLED = 0,
    #[doc = "1: Enable low pass filter"]
    JTMS_LPF_ENABLED = 1,
}
impl From<JTMS_LPF_A> for bool {
    #[inline(always)]
    fn from(variant: JTMS_LPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JTMS_LPF`"]
pub type JTMS_LPF_R = crate::R<bool, JTMS_LPF_A>;
impl JTMS_LPF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JTMS_LPF_A {
        match self.bits {
            false => JTMS_LPF_A::JTMS_LPF_DISABLED,
            true => JTMS_LPF_A::JTMS_LPF_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `JTMS_LPF_DISABLED`"]
    #[inline(always)]
    pub fn is_jtms_lpf_disabled(&self) -> bool {
        *self == JTMS_LPF_A::JTMS_LPF_DISABLED
    }
    #[doc = "Checks if the value of the field is `JTMS_LPF_ENABLED`"]
    #[inline(always)]
    pub fn is_jtms_lpf_enabled(&self) -> bool {
        *self == JTMS_LPF_A::JTMS_LPF_ENABLED
    }
}
#[doc = "Write proxy for field `JTMS_LPF`"]
pub struct JTMS_LPF_W<'a> {
    w: &'a mut W,
}
impl<'a> JTMS_LPF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JTMS_LPF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable low pass filter"]
    #[inline(always)]
    pub fn jtms_lpf_disabled(self) -> &'a mut W {
        self.variant(JTMS_LPF_A::JTMS_LPF_DISABLED)
    }
    #[doc = "Enable low pass filter"]
    #[inline(always)]
    pub fn jtms_lpf_enabled(self) -> &'a mut W {
        self.variant(JTMS_LPF_A::JTMS_LPF_ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "CM3 JTAG on DIO\\[14:15\\]\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM3_JTAG_DATA_EN_A {
    #[doc = "0: CM3 JTAG data (TDI & TDO) not available on DIO\\[14:15\\]"]
    CM3_JTAG_DATA_DISABLED = 0,
    #[doc = "1: CM3 JTAG data (TDI & TDO) connected through DIO\\[14:15\\]"]
    CM3_JTAG_DATA_ENABLED = 1,
}
impl From<CM3_JTAG_DATA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CM3_JTAG_DATA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CM3_JTAG_DATA_EN`"]
pub type CM3_JTAG_DATA_EN_R = crate::R<bool, CM3_JTAG_DATA_EN_A>;
impl CM3_JTAG_DATA_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM3_JTAG_DATA_EN_A {
        match self.bits {
            false => CM3_JTAG_DATA_EN_A::CM3_JTAG_DATA_DISABLED,
            true => CM3_JTAG_DATA_EN_A::CM3_JTAG_DATA_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `CM3_JTAG_DATA_DISABLED`"]
    #[inline(always)]
    pub fn is_cm3_jtag_data_disabled(&self) -> bool {
        *self == CM3_JTAG_DATA_EN_A::CM3_JTAG_DATA_DISABLED
    }
    #[doc = "Checks if the value of the field is `CM3_JTAG_DATA_ENABLED`"]
    #[inline(always)]
    pub fn is_cm3_jtag_data_enabled(&self) -> bool {
        *self == CM3_JTAG_DATA_EN_A::CM3_JTAG_DATA_ENABLED
    }
}
#[doc = "Write proxy for field `CM3_JTAG_DATA_EN`"]
pub struct CM3_JTAG_DATA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CM3_JTAG_DATA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM3_JTAG_DATA_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CM3 JTAG data (TDI & TDO) not available on DIO\\[14:15\\]"]
    #[inline(always)]
    pub fn cm3_jtag_data_disabled(self) -> &'a mut W {
        self.variant(CM3_JTAG_DATA_EN_A::CM3_JTAG_DATA_DISABLED)
    }
    #[doc = "CM3 JTAG data (TDI & TDO) connected through DIO\\[14:15\\]"]
    #[inline(always)]
    pub fn cm3_jtag_data_enabled(self) -> &'a mut W {
        self.variant(CM3_JTAG_DATA_EN_A::CM3_JTAG_DATA_ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "CM3 JTAG TRST on DIO13\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM3_JTAG_TRST_EN_A {
    #[doc = "0: CM3 JTAG TRST not available on DIO13"]
    CM3_JTAG_TRST_DISABLED = 0,
    #[doc = "1: CM3 JTAG TRST connected through DIO13"]
    CM3_JTAG_TRST_ENABLED = 1,
}
impl From<CM3_JTAG_TRST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CM3_JTAG_TRST_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CM3_JTAG_TRST_EN`"]
pub type CM3_JTAG_TRST_EN_R = crate::R<bool, CM3_JTAG_TRST_EN_A>;
impl CM3_JTAG_TRST_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM3_JTAG_TRST_EN_A {
        match self.bits {
            false => CM3_JTAG_TRST_EN_A::CM3_JTAG_TRST_DISABLED,
            true => CM3_JTAG_TRST_EN_A::CM3_JTAG_TRST_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `CM3_JTAG_TRST_DISABLED`"]
    #[inline(always)]
    pub fn is_cm3_jtag_trst_disabled(&self) -> bool {
        *self == CM3_JTAG_TRST_EN_A::CM3_JTAG_TRST_DISABLED
    }
    #[doc = "Checks if the value of the field is `CM3_JTAG_TRST_ENABLED`"]
    #[inline(always)]
    pub fn is_cm3_jtag_trst_enabled(&self) -> bool {
        *self == CM3_JTAG_TRST_EN_A::CM3_JTAG_TRST_ENABLED
    }
}
#[doc = "Write proxy for field `CM3_JTAG_TRST_EN`"]
pub struct CM3_JTAG_TRST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CM3_JTAG_TRST_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM3_JTAG_TRST_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CM3 JTAG TRST not available on DIO13"]
    #[inline(always)]
    pub fn cm3_jtag_trst_disabled(self) -> &'a mut W {
        self.variant(CM3_JTAG_TRST_EN_A::CM3_JTAG_TRST_DISABLED)
    }
    #[doc = "CM3 JTAG TRST connected through DIO13"]
    #[inline(always)]
    pub fn cm3_jtag_trst_enabled(self) -> &'a mut W {
        self.variant(CM3_JTAG_TRST_EN_A::CM3_JTAG_TRST_ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "JTCK pull-up enable / disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum JTCK_PULL_A {
    #[doc = "0: No pull selected"]
    JTCK_NO_PULL = 0,
    #[doc = "1: Weak pull-up selected"]
    JTCK_WEAK_PULL_UP = 1,
    #[doc = "2: Weak pull-down selected"]
    JTCK_WEAK_PULL_DOWN = 2,
    #[doc = "3: Strong pull-up selected"]
    JTCK_STRONG_PULL_UP = 3,
}
impl From<JTCK_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: JTCK_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `JTCK_PULL`"]
pub type JTCK_PULL_R = crate::R<u8, JTCK_PULL_A>;
impl JTCK_PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JTCK_PULL_A {
        match self.bits {
            0 => JTCK_PULL_A::JTCK_NO_PULL,
            1 => JTCK_PULL_A::JTCK_WEAK_PULL_UP,
            2 => JTCK_PULL_A::JTCK_WEAK_PULL_DOWN,
            3 => JTCK_PULL_A::JTCK_STRONG_PULL_UP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `JTCK_NO_PULL`"]
    #[inline(always)]
    pub fn is_jtck_no_pull(&self) -> bool {
        *self == JTCK_PULL_A::JTCK_NO_PULL
    }
    #[doc = "Checks if the value of the field is `JTCK_WEAK_PULL_UP`"]
    #[inline(always)]
    pub fn is_jtck_weak_pull_up(&self) -> bool {
        *self == JTCK_PULL_A::JTCK_WEAK_PULL_UP
    }
    #[doc = "Checks if the value of the field is `JTCK_WEAK_PULL_DOWN`"]
    #[inline(always)]
    pub fn is_jtck_weak_pull_down(&self) -> bool {
        *self == JTCK_PULL_A::JTCK_WEAK_PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `JTCK_STRONG_PULL_UP`"]
    #[inline(always)]
    pub fn is_jtck_strong_pull_up(&self) -> bool {
        *self == JTCK_PULL_A::JTCK_STRONG_PULL_UP
    }
}
#[doc = "Write proxy for field `JTCK_PULL`"]
pub struct JTCK_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> JTCK_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JTCK_PULL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No pull selected"]
    #[inline(always)]
    pub fn jtck_no_pull(self) -> &'a mut W {
        self.variant(JTCK_PULL_A::JTCK_NO_PULL)
    }
    #[doc = "Weak pull-up selected"]
    #[inline(always)]
    pub fn jtck_weak_pull_up(self) -> &'a mut W {
        self.variant(JTCK_PULL_A::JTCK_WEAK_PULL_UP)
    }
    #[doc = "Weak pull-down selected"]
    #[inline(always)]
    pub fn jtck_weak_pull_down(self) -> &'a mut W {
        self.variant(JTCK_PULL_A::JTCK_WEAK_PULL_DOWN)
    }
    #[doc = "Strong pull-up selected"]
    #[inline(always)]
    pub fn jtck_strong_pull_up(self) -> &'a mut W {
        self.variant(JTCK_PULL_A::JTCK_STRONG_PULL_UP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "JTMS drive strength\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum JTMS_DRIVE_A {
    #[doc = "0: 2x drive strength"]
    JTMS_2X_DRIVE = 0,
    #[doc = "1: 3x drive strength"]
    JTMS_3X_DRIVE = 1,
    #[doc = "2: 5x drive strength"]
    JTMS_5X_DRIVE = 2,
    #[doc = "3: 6x drive strength"]
    JTMS_6X_DRIVE = 3,
}
impl From<JTMS_DRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: JTMS_DRIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `JTMS_DRIVE`"]
pub type JTMS_DRIVE_R = crate::R<u8, JTMS_DRIVE_A>;
impl JTMS_DRIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JTMS_DRIVE_A {
        match self.bits {
            0 => JTMS_DRIVE_A::JTMS_2X_DRIVE,
            1 => JTMS_DRIVE_A::JTMS_3X_DRIVE,
            2 => JTMS_DRIVE_A::JTMS_5X_DRIVE,
            3 => JTMS_DRIVE_A::JTMS_6X_DRIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `JTMS_2X_DRIVE`"]
    #[inline(always)]
    pub fn is_jtms_2x_drive(&self) -> bool {
        *self == JTMS_DRIVE_A::JTMS_2X_DRIVE
    }
    #[doc = "Checks if the value of the field is `JTMS_3X_DRIVE`"]
    #[inline(always)]
    pub fn is_jtms_3x_drive(&self) -> bool {
        *self == JTMS_DRIVE_A::JTMS_3X_DRIVE
    }
    #[doc = "Checks if the value of the field is `JTMS_5X_DRIVE`"]
    #[inline(always)]
    pub fn is_jtms_5x_drive(&self) -> bool {
        *self == JTMS_DRIVE_A::JTMS_5X_DRIVE
    }
    #[doc = "Checks if the value of the field is `JTMS_6X_DRIVE`"]
    #[inline(always)]
    pub fn is_jtms_6x_drive(&self) -> bool {
        *self == JTMS_DRIVE_A::JTMS_6X_DRIVE
    }
}
#[doc = "Write proxy for field `JTMS_DRIVE`"]
pub struct JTMS_DRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> JTMS_DRIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JTMS_DRIVE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "2x drive strength"]
    #[inline(always)]
    pub fn jtms_2x_drive(self) -> &'a mut W {
        self.variant(JTMS_DRIVE_A::JTMS_2X_DRIVE)
    }
    #[doc = "3x drive strength"]
    #[inline(always)]
    pub fn jtms_3x_drive(self) -> &'a mut W {
        self.variant(JTMS_DRIVE_A::JTMS_3X_DRIVE)
    }
    #[doc = "5x drive strength"]
    #[inline(always)]
    pub fn jtms_5x_drive(self) -> &'a mut W {
        self.variant(JTMS_DRIVE_A::JTMS_5X_DRIVE)
    }
    #[doc = "6x drive strength"]
    #[inline(always)]
    pub fn jtms_6x_drive(self) -> &'a mut W {
        self.variant(JTMS_DRIVE_A::JTMS_6X_DRIVE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "JTMS pull-up enable / disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum JTMS_PULL_A {
    #[doc = "0: No pull selected"]
    JTMS_NO_PULL = 0,
    #[doc = "1: Weak pull-up selected"]
    JTMS_WEAK_PULL_UP = 1,
    #[doc = "2: Weak pull-down selected"]
    JTMS_WEAK_PULL_DOWN = 2,
    #[doc = "3: Strong pull-up selected"]
    JTMS_STRONG_PULL_UP = 3,
}
impl From<JTMS_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: JTMS_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `JTMS_PULL`"]
pub type JTMS_PULL_R = crate::R<u8, JTMS_PULL_A>;
impl JTMS_PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JTMS_PULL_A {
        match self.bits {
            0 => JTMS_PULL_A::JTMS_NO_PULL,
            1 => JTMS_PULL_A::JTMS_WEAK_PULL_UP,
            2 => JTMS_PULL_A::JTMS_WEAK_PULL_DOWN,
            3 => JTMS_PULL_A::JTMS_STRONG_PULL_UP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `JTMS_NO_PULL`"]
    #[inline(always)]
    pub fn is_jtms_no_pull(&self) -> bool {
        *self == JTMS_PULL_A::JTMS_NO_PULL
    }
    #[doc = "Checks if the value of the field is `JTMS_WEAK_PULL_UP`"]
    #[inline(always)]
    pub fn is_jtms_weak_pull_up(&self) -> bool {
        *self == JTMS_PULL_A::JTMS_WEAK_PULL_UP
    }
    #[doc = "Checks if the value of the field is `JTMS_WEAK_PULL_DOWN`"]
    #[inline(always)]
    pub fn is_jtms_weak_pull_down(&self) -> bool {
        *self == JTMS_PULL_A::JTMS_WEAK_PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `JTMS_STRONG_PULL_UP`"]
    #[inline(always)]
    pub fn is_jtms_strong_pull_up(&self) -> bool {
        *self == JTMS_PULL_A::JTMS_STRONG_PULL_UP
    }
}
#[doc = "Write proxy for field `JTMS_PULL`"]
pub struct JTMS_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> JTMS_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JTMS_PULL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No pull selected"]
    #[inline(always)]
    pub fn jtms_no_pull(self) -> &'a mut W {
        self.variant(JTMS_PULL_A::JTMS_NO_PULL)
    }
    #[doc = "Weak pull-up selected"]
    #[inline(always)]
    pub fn jtms_weak_pull_up(self) -> &'a mut W {
        self.variant(JTMS_PULL_A::JTMS_WEAK_PULL_UP)
    }
    #[doc = "Weak pull-down selected"]
    #[inline(always)]
    pub fn jtms_weak_pull_down(self) -> &'a mut W {
        self.variant(JTMS_PULL_A::JTMS_WEAK_PULL_DOWN)
    }
    #[doc = "Strong pull-up selected"]
    #[inline(always)]
    pub fn jtms_strong_pull_up(self) -> &'a mut W {
        self.variant(JTMS_PULL_A::JTMS_STRONG_PULL_UP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 9 - JTCK Low-Pass-Filter enable / disable"]
    #[inline(always)]
    pub fn jtck_lpf(&self) -> JTCK_LPF_R {
        JTCK_LPF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - JTMS Low-Pass-Filter enable / disable"]
    #[inline(always)]
    pub fn jtms_lpf(&self) -> JTMS_LPF_R {
        JTMS_LPF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CM3 JTAG on DIO\\[14:15\\]"]
    #[inline(always)]
    pub fn cm3_jtag_data_en(&self) -> CM3_JTAG_DATA_EN_R {
        CM3_JTAG_DATA_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CM3 JTAG TRST on DIO13"]
    #[inline(always)]
    pub fn cm3_jtag_trst_en(&self) -> CM3_JTAG_TRST_EN_R {
        CM3_JTAG_TRST_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - JTCK pull-up enable / disable"]
    #[inline(always)]
    pub fn jtck_pull(&self) -> JTCK_PULL_R {
        JTCK_PULL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - JTMS drive strength"]
    #[inline(always)]
    pub fn jtms_drive(&self) -> JTMS_DRIVE_R {
        JTMS_DRIVE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - JTMS pull-up enable / disable"]
    #[inline(always)]
    pub fn jtms_pull(&self) -> JTMS_PULL_R {
        JTMS_PULL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 9 - JTCK Low-Pass-Filter enable / disable"]
    #[inline(always)]
    pub fn jtck_lpf(&mut self) -> JTCK_LPF_W {
        JTCK_LPF_W { w: self }
    }
    #[doc = "Bit 8 - JTMS Low-Pass-Filter enable / disable"]
    #[inline(always)]
    pub fn jtms_lpf(&mut self) -> JTMS_LPF_W {
        JTMS_LPF_W { w: self }
    }
    #[doc = "Bit 7 - CM3 JTAG on DIO\\[14:15\\]"]
    #[inline(always)]
    pub fn cm3_jtag_data_en(&mut self) -> CM3_JTAG_DATA_EN_W {
        CM3_JTAG_DATA_EN_W { w: self }
    }
    #[doc = "Bit 6 - CM3 JTAG TRST on DIO13"]
    #[inline(always)]
    pub fn cm3_jtag_trst_en(&mut self) -> CM3_JTAG_TRST_EN_W {
        CM3_JTAG_TRST_EN_W { w: self }
    }
    #[doc = "Bits 4:5 - JTCK pull-up enable / disable"]
    #[inline(always)]
    pub fn jtck_pull(&mut self) -> JTCK_PULL_W {
        JTCK_PULL_W { w: self }
    }
    #[doc = "Bits 2:3 - JTMS drive strength"]
    #[inline(always)]
    pub fn jtms_drive(&mut self) -> JTMS_DRIVE_W {
        JTMS_DRIVE_W { w: self }
    }
    #[doc = "Bits 0:1 - JTMS pull-up enable / disable"]
    #[inline(always)]
    pub fn jtms_pull(&mut self) -> JTMS_PULL_W {
        JTMS_PULL_W { w: self }
    }
}
