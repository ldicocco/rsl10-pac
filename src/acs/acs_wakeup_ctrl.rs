#[doc = "Reader of register ACS_WAKEUP_CTRL"]
pub type R = crate::R<u32, super::ACS_WAKEUP_CTRL>;
#[doc = "Writer for register ACS_WAKEUP_CTRL"]
pub type W = crate::W<u32, super::ACS_WAKEUP_CTRL>;
#[doc = "Register ACS_WAKEUP_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ACS_WAKEUP_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable / Disable the retention mode of the pads\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PADS_RETENTION_EN_A {
    #[doc = "0: Disable the pad retention mode"]
    PADS_RETENTION_DISABLE = 0,
    #[doc = "1: Enable the pad retention mode"]
    PADS_RETENTION_ENABLE = 1,
}
impl From<PADS_RETENTION_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PADS_RETENTION_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PADS_RETENTION_EN`"]
pub type PADS_RETENTION_EN_R = crate::R<bool, PADS_RETENTION_EN_A>;
impl PADS_RETENTION_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PADS_RETENTION_EN_A {
        match self.bits {
            false => PADS_RETENTION_EN_A::PADS_RETENTION_DISABLE,
            true => PADS_RETENTION_EN_A::PADS_RETENTION_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PADS_RETENTION_DISABLE`"]
    #[inline(always)]
    pub fn is_pads_retention_disable(&self) -> bool {
        *self == PADS_RETENTION_EN_A::PADS_RETENTION_DISABLE
    }
    #[doc = "Checks if the value of the field is `PADS_RETENTION_ENABLE`"]
    #[inline(always)]
    pub fn is_pads_retention_enable(&self) -> bool {
        *self == PADS_RETENTION_EN_A::PADS_RETENTION_ENABLE
    }
}
#[doc = "Write proxy for field `PADS_RETENTION_EN`"]
pub struct PADS_RETENTION_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PADS_RETENTION_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PADS_RETENTION_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the pad retention mode"]
    #[inline(always)]
    pub fn pads_retention_disable(self) -> &'a mut W {
        self.variant(PADS_RETENTION_EN_A::PADS_RETENTION_DISABLE)
    }
    #[doc = "Enable the pad retention mode"]
    #[inline(always)]
    pub fn pads_retention_enable(self) -> &'a mut W {
        self.variant(PADS_RETENTION_EN_A::PADS_RETENTION_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Boot mode flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOT_FLASH_APP_REBOOT_A {
    #[doc = "0: The reboot mode flag is not set"]
    BOOT_FLASH_APP_REBOOT_DISABLE = 0,
    #[doc = "1: The reboot mode flag is set (ROM will not read the calibration values from flash and will directly execute the application)"]
    BOOT_FLASH_APP_REBOOT_ENABLE = 1,
}
impl From<BOOT_FLASH_APP_REBOOT_A> for bool {
    #[inline(always)]
    fn from(variant: BOOT_FLASH_APP_REBOOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOOT_FLASH_APP_REBOOT`"]
pub type BOOT_FLASH_APP_REBOOT_R = crate::R<bool, BOOT_FLASH_APP_REBOOT_A>;
impl BOOT_FLASH_APP_REBOOT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOT_FLASH_APP_REBOOT_A {
        match self.bits {
            false => BOOT_FLASH_APP_REBOOT_A::BOOT_FLASH_APP_REBOOT_DISABLE,
            true => BOOT_FLASH_APP_REBOOT_A::BOOT_FLASH_APP_REBOOT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BOOT_FLASH_APP_REBOOT_DISABLE`"]
    #[inline(always)]
    pub fn is_boot_flash_app_reboot_disable(&self) -> bool {
        *self == BOOT_FLASH_APP_REBOOT_A::BOOT_FLASH_APP_REBOOT_DISABLE
    }
    #[doc = "Checks if the value of the field is `BOOT_FLASH_APP_REBOOT_ENABLE`"]
    #[inline(always)]
    pub fn is_boot_flash_app_reboot_enable(&self) -> bool {
        *self == BOOT_FLASH_APP_REBOOT_A::BOOT_FLASH_APP_REBOOT_ENABLE
    }
}
#[doc = "Write proxy for field `BOOT_FLASH_APP_REBOOT`"]
pub struct BOOT_FLASH_APP_REBOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_FLASH_APP_REBOOT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOT_FLASH_APP_REBOOT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The reboot mode flag is not set"]
    #[inline(always)]
    pub fn boot_flash_app_reboot_disable(self) -> &'a mut W {
        self.variant(BOOT_FLASH_APP_REBOOT_A::BOOT_FLASH_APP_REBOOT_DISABLE)
    }
    #[doc = "The reboot mode flag is set (ROM will not read the calibration values from flash and will directly execute the application)"]
    #[inline(always)]
    pub fn boot_flash_app_reboot_enable(self) -> &'a mut W {
        self.variant(BOOT_FLASH_APP_REBOOT_A::BOOT_FLASH_APP_REBOOT_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "RC oscillator clock multiplier read only flag (mirror of CLOCK_MULT of ACS_RCOSC_CTRL register)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RC_CLOCK_MULT_A {
    #[doc = "0: The startup RC Oscillator is at 3 MHz"]
    RC_START_OSC_STATUS_3MHZ = 0,
    #[doc = "1: The startup RC Oscillator is at 12 MHz"]
    RC_START_OSC_STATUS_12MHZ = 1,
}
impl From<RC_CLOCK_MULT_A> for bool {
    #[inline(always)]
    fn from(variant: RC_CLOCK_MULT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RC_CLOCK_MULT`"]
pub type RC_CLOCK_MULT_R = crate::R<bool, RC_CLOCK_MULT_A>;
impl RC_CLOCK_MULT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RC_CLOCK_MULT_A {
        match self.bits {
            false => RC_CLOCK_MULT_A::RC_START_OSC_STATUS_3MHZ,
            true => RC_CLOCK_MULT_A::RC_START_OSC_STATUS_12MHZ,
        }
    }
    #[doc = "Checks if the value of the field is `RC_START_OSC_STATUS_3MHZ`"]
    #[inline(always)]
    pub fn is_rc_start_osc_status_3mhz(&self) -> bool {
        *self == RC_CLOCK_MULT_A::RC_START_OSC_STATUS_3MHZ
    }
    #[doc = "Checks if the value of the field is `RC_START_OSC_STATUS_12MHZ`"]
    #[inline(always)]
    pub fn is_rc_start_osc_status_12mhz(&self) -> bool {
        *self == RC_CLOCK_MULT_A::RC_START_OSC_STATUS_12MHZ
    }
}
#[doc = "RC oscillator trimming read only flag (mirror of FTRIM_FLAG of ACS_RCOSC_CTRL register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RC_FTRIM_FLAG_A {
    #[doc = "0: The oscillators are not calibrated"]
    RC_OSC_STATUS_UNCALIBRATED = 0,
    #[doc = "1: The oscillators are calibrated"]
    RC_OSC_STATUS_CALIBRATED = 1,
}
impl From<RC_FTRIM_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: RC_FTRIM_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RC_FTRIM_FLAG`"]
pub type RC_FTRIM_FLAG_R = crate::R<bool, RC_FTRIM_FLAG_A>;
impl RC_FTRIM_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RC_FTRIM_FLAG_A {
        match self.bits {
            false => RC_FTRIM_FLAG_A::RC_OSC_STATUS_UNCALIBRATED,
            true => RC_FTRIM_FLAG_A::RC_OSC_STATUS_CALIBRATED,
        }
    }
    #[doc = "Checks if the value of the field is `RC_OSC_STATUS_UNCALIBRATED`"]
    #[inline(always)]
    pub fn is_rc_osc_status_uncalibrated(&self) -> bool {
        *self == RC_FTRIM_FLAG_A::RC_OSC_STATUS_UNCALIBRATED
    }
    #[doc = "Checks if the value of the field is `RC_OSC_STATUS_CALIBRATED`"]
    #[inline(always)]
    pub fn is_rc_osc_status_calibrated(&self) -> bool {
        *self == RC_FTRIM_FLAG_A::RC_OSC_STATUS_CALIBRATED
    }
}
#[doc = "Boot selection to indicate boot source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BOOT_SELECT_A {
    #[doc = "0: The ARM Cortex-M3 executes code from the flash and the XTAL will not be started at boot"]
    BOOT_FLASH_XTAL_DISABLE = 0,
    #[doc = "1: The ARM Cortex-M3 executed code from the address specified in the wakeup information in retention RAM and the XTAL will not be started at boot"]
    BOOT_CUSTOM = 1,
    #[doc = "2: The ARM Cortex-M3 executes code from the flash and the XTAL will be started at boot with the default trim"]
    BOOT_FLASH_XTAL_DEFAULT_TRIM = 2,
    #[doc = "3: The ARM Cortex-M3 executes code from the flash and the XTAL will be started at boot with trim from ACS_WAKEUP_GP_DATA"]
    BOOT_FLASH_XTAL_CUSTOM_TRIM = 3,
}
impl From<BOOT_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOT_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BOOT_SELECT`"]
pub type BOOT_SELECT_R = crate::R<u8, BOOT_SELECT_A>;
impl BOOT_SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOT_SELECT_A {
        match self.bits {
            0 => BOOT_SELECT_A::BOOT_FLASH_XTAL_DISABLE,
            1 => BOOT_SELECT_A::BOOT_CUSTOM,
            2 => BOOT_SELECT_A::BOOT_FLASH_XTAL_DEFAULT_TRIM,
            3 => BOOT_SELECT_A::BOOT_FLASH_XTAL_CUSTOM_TRIM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BOOT_FLASH_XTAL_DISABLE`"]
    #[inline(always)]
    pub fn is_boot_flash_xtal_disable(&self) -> bool {
        *self == BOOT_SELECT_A::BOOT_FLASH_XTAL_DISABLE
    }
    #[doc = "Checks if the value of the field is `BOOT_CUSTOM`"]
    #[inline(always)]
    pub fn is_boot_custom(&self) -> bool {
        *self == BOOT_SELECT_A::BOOT_CUSTOM
    }
    #[doc = "Checks if the value of the field is `BOOT_FLASH_XTAL_DEFAULT_TRIM`"]
    #[inline(always)]
    pub fn is_boot_flash_xtal_default_trim(&self) -> bool {
        *self == BOOT_SELECT_A::BOOT_FLASH_XTAL_DEFAULT_TRIM
    }
    #[doc = "Checks if the value of the field is `BOOT_FLASH_XTAL_CUSTOM_TRIM`"]
    #[inline(always)]
    pub fn is_boot_flash_xtal_custom_trim(&self) -> bool {
        *self == BOOT_SELECT_A::BOOT_FLASH_XTAL_CUSTOM_TRIM
    }
}
#[doc = "Write proxy for field `BOOT_SELECT`"]
pub struct BOOT_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOT_SELECT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The ARM Cortex-M3 executes code from the flash and the XTAL will not be started at boot"]
    #[inline(always)]
    pub fn boot_flash_xtal_disable(self) -> &'a mut W {
        self.variant(BOOT_SELECT_A::BOOT_FLASH_XTAL_DISABLE)
    }
    #[doc = "The ARM Cortex-M3 executed code from the address specified in the wakeup information in retention RAM and the XTAL will not be started at boot"]
    #[inline(always)]
    pub fn boot_custom(self) -> &'a mut W {
        self.variant(BOOT_SELECT_A::BOOT_CUSTOM)
    }
    #[doc = "The ARM Cortex-M3 executes code from the flash and the XTAL will be started at boot with the default trim"]
    #[inline(always)]
    pub fn boot_flash_xtal_default_trim(self) -> &'a mut W {
        self.variant(BOOT_SELECT_A::BOOT_FLASH_XTAL_DEFAULT_TRIM)
    }
    #[doc = "The ARM Cortex-M3 executes code from the flash and the XTAL will be started at boot with trim from ACS_WAKEUP_GP_DATA"]
    #[inline(always)]
    pub fn boot_flash_xtal_custom_trim(self) -> &'a mut W {
        self.variant(BOOT_SELECT_A::BOOT_FLASH_XTAL_CUSTOM_TRIM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_OVERLOAD_WAKEUP_A {
    #[doc = "0: DCDC overload has not triggered a wakeup event"]
    WAKEUP_DCDC_OVERLOAD_NOT_SET = 0,
    #[doc = "1: DCDC overload has triggered a wakeup event at least once"]
    WAKEUP_DCDC_OVERLOAD_SET = 1,
}
impl From<DCDC_OVERLOAD_WAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: DCDC_OVERLOAD_WAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCDC_OVERLOAD_WAKEUP`"]
pub type DCDC_OVERLOAD_WAKEUP_R = crate::R<bool, DCDC_OVERLOAD_WAKEUP_A>;
impl DCDC_OVERLOAD_WAKEUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDC_OVERLOAD_WAKEUP_A {
        match self.bits {
            false => DCDC_OVERLOAD_WAKEUP_A::WAKEUP_DCDC_OVERLOAD_NOT_SET,
            true => DCDC_OVERLOAD_WAKEUP_A::WAKEUP_DCDC_OVERLOAD_SET,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DCDC_OVERLOAD_NOT_SET`"]
    #[inline(always)]
    pub fn is_wakeup_dcdc_overload_not_set(&self) -> bool {
        *self == DCDC_OVERLOAD_WAKEUP_A::WAKEUP_DCDC_OVERLOAD_NOT_SET
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DCDC_OVERLOAD_SET`"]
    #[inline(always)]
    pub fn is_wakeup_dcdc_overload_set(&self) -> bool {
        *self == DCDC_OVERLOAD_WAKEUP_A::WAKEUP_DCDC_OVERLOAD_SET
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_PAD_WAKEUP_A {
    #[doc = "0: Wakeup pad has not triggered a wakeup event"]
    WAKEUP_PAD_EVENT_NOT_SET = 0,
    #[doc = "1: Wakeup pad has triggered a wakeup event at least once"]
    WAKEUP_PAD_EVENT_SET = 1,
}
impl From<WAKEUP_PAD_WAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_PAD_WAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAKEUP_PAD_WAKEUP`"]
pub type WAKEUP_PAD_WAKEUP_R = crate::R<bool, WAKEUP_PAD_WAKEUP_A>;
impl WAKEUP_PAD_WAKEUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP_PAD_WAKEUP_A {
        match self.bits {
            false => WAKEUP_PAD_WAKEUP_A::WAKEUP_PAD_EVENT_NOT_SET,
            true => WAKEUP_PAD_WAKEUP_A::WAKEUP_PAD_EVENT_SET,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_PAD_EVENT_NOT_SET`"]
    #[inline(always)]
    pub fn is_wakeup_pad_event_not_set(&self) -> bool {
        *self == WAKEUP_PAD_WAKEUP_A::WAKEUP_PAD_EVENT_NOT_SET
    }
    #[doc = "Checks if the value of the field is `WAKEUP_PAD_EVENT_SET`"]
    #[inline(always)]
    pub fn is_wakeup_pad_event_set(&self) -> bool {
        *self == WAKEUP_PAD_WAKEUP_A::WAKEUP_PAD_EVENT_SET
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ALARM_WAKEUP_A {
    #[doc = "0: RTC alarm has not triggered a wakeup event"]
    WAKEUP_RTC_ALARM_EVENT_NOT_SET = 0,
    #[doc = "1: RTC alarm has triggered a wakeup event at least once"]
    WAKEUP_RTC_ALARM_EVENT_SET = 1,
}
impl From<RTC_ALARM_WAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_ALARM_WAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTC_ALARM_WAKEUP`"]
pub type RTC_ALARM_WAKEUP_R = crate::R<bool, RTC_ALARM_WAKEUP_A>;
impl RTC_ALARM_WAKEUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_ALARM_WAKEUP_A {
        match self.bits {
            false => RTC_ALARM_WAKEUP_A::WAKEUP_RTC_ALARM_EVENT_NOT_SET,
            true => RTC_ALARM_WAKEUP_A::WAKEUP_RTC_ALARM_EVENT_SET,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_RTC_ALARM_EVENT_NOT_SET`"]
    #[inline(always)]
    pub fn is_wakeup_rtc_alarm_event_not_set(&self) -> bool {
        *self == RTC_ALARM_WAKEUP_A::WAKEUP_RTC_ALARM_EVENT_NOT_SET
    }
    #[doc = "Checks if the value of the field is `WAKEUP_RTC_ALARM_EVENT_SET`"]
    #[inline(always)]
    pub fn is_wakeup_rtc_alarm_event_set(&self) -> bool {
        *self == RTC_ALARM_WAKEUP_A::WAKEUP_RTC_ALARM_EVENT_SET
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_TIMER_WAKEUP_A {
    #[doc = "0: BB timer has not triggered a wakeup event"]
    WAKEUP_BB_TIMER_EVENT_NOT_SET = 0,
    #[doc = "1: BB timer has triggered a wakeup event at least once"]
    WAKEUP_BB_TIMER_EVENT_SET = 1,
}
impl From<BB_TIMER_WAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: BB_TIMER_WAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BB_TIMER_WAKEUP`"]
pub type BB_TIMER_WAKEUP_R = crate::R<bool, BB_TIMER_WAKEUP_A>;
impl BB_TIMER_WAKEUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_TIMER_WAKEUP_A {
        match self.bits {
            false => BB_TIMER_WAKEUP_A::WAKEUP_BB_TIMER_EVENT_NOT_SET,
            true => BB_TIMER_WAKEUP_A::WAKEUP_BB_TIMER_EVENT_SET,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_BB_TIMER_EVENT_NOT_SET`"]
    #[inline(always)]
    pub fn is_wakeup_bb_timer_event_not_set(&self) -> bool {
        *self == BB_TIMER_WAKEUP_A::WAKEUP_BB_TIMER_EVENT_NOT_SET
    }
    #[doc = "Checks if the value of the field is `WAKEUP_BB_TIMER_EVENT_SET`"]
    #[inline(always)]
    pub fn is_wakeup_bb_timer_event_set(&self) -> bool {
        *self == BB_TIMER_WAKEUP_A::WAKEUP_BB_TIMER_EVENT_SET
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIO3_WAKEUP_A {
    #[doc = "0: DIO3 has not triggered a wakeup event"]
    WAKEUP_DIO3_EVENT_NOT_SET = 0,
    #[doc = "1: DIO3 has triggered a wakeup event at least once"]
    WAKEUP_DIO3_EVENT_SET = 1,
}
impl From<DIO3_WAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: DIO3_WAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIO3_WAKEUP`"]
pub type DIO3_WAKEUP_R = crate::R<bool, DIO3_WAKEUP_A>;
impl DIO3_WAKEUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIO3_WAKEUP_A {
        match self.bits {
            false => DIO3_WAKEUP_A::WAKEUP_DIO3_EVENT_NOT_SET,
            true => DIO3_WAKEUP_A::WAKEUP_DIO3_EVENT_SET,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO3_EVENT_NOT_SET`"]
    #[inline(always)]
    pub fn is_wakeup_dio3_event_not_set(&self) -> bool {
        *self == DIO3_WAKEUP_A::WAKEUP_DIO3_EVENT_NOT_SET
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO3_EVENT_SET`"]
    #[inline(always)]
    pub fn is_wakeup_dio3_event_set(&self) -> bool {
        *self == DIO3_WAKEUP_A::WAKEUP_DIO3_EVENT_SET
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIO2_WAKEUP_A {
    #[doc = "0: DIO2 has not triggered a wakeup event"]
    WAKEUP_DIO2_EVENT_NOT_SET = 0,
    #[doc = "1: DIO2 has triggered a wakeup event at least once"]
    WAKEUP_DIO2_EVENT_SET = 1,
}
impl From<DIO2_WAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: DIO2_WAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIO2_WAKEUP`"]
pub type DIO2_WAKEUP_R = crate::R<bool, DIO2_WAKEUP_A>;
impl DIO2_WAKEUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIO2_WAKEUP_A {
        match self.bits {
            false => DIO2_WAKEUP_A::WAKEUP_DIO2_EVENT_NOT_SET,
            true => DIO2_WAKEUP_A::WAKEUP_DIO2_EVENT_SET,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO2_EVENT_NOT_SET`"]
    #[inline(always)]
    pub fn is_wakeup_dio2_event_not_set(&self) -> bool {
        *self == DIO2_WAKEUP_A::WAKEUP_DIO2_EVENT_NOT_SET
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO2_EVENT_SET`"]
    #[inline(always)]
    pub fn is_wakeup_dio2_event_set(&self) -> bool {
        *self == DIO2_WAKEUP_A::WAKEUP_DIO2_EVENT_SET
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIO1_WAKEUP_A {
    #[doc = "0: DIO1 has not triggered a wakeup event"]
    WAKEUP_DIO1_EVENT_NOT_SET = 0,
    #[doc = "1: DIO1 has triggered a wakeup event at least once"]
    WAKEUP_DIO1_EVENT_SET = 1,
}
impl From<DIO1_WAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: DIO1_WAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIO1_WAKEUP`"]
pub type DIO1_WAKEUP_R = crate::R<bool, DIO1_WAKEUP_A>;
impl DIO1_WAKEUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIO1_WAKEUP_A {
        match self.bits {
            false => DIO1_WAKEUP_A::WAKEUP_DIO1_EVENT_NOT_SET,
            true => DIO1_WAKEUP_A::WAKEUP_DIO1_EVENT_SET,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO1_EVENT_NOT_SET`"]
    #[inline(always)]
    pub fn is_wakeup_dio1_event_not_set(&self) -> bool {
        *self == DIO1_WAKEUP_A::WAKEUP_DIO1_EVENT_NOT_SET
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO1_EVENT_SET`"]
    #[inline(always)]
    pub fn is_wakeup_dio1_event_set(&self) -> bool {
        *self == DIO1_WAKEUP_A::WAKEUP_DIO1_EVENT_SET
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIO0_WAKEUP_A {
    #[doc = "0: DIO0 has not triggered a wakeup event"]
    WAKEUP_DIO0_EVENT_NOT_SET = 0,
    #[doc = "1: DIO0 has triggered a wakeup event at least once"]
    WAKEUP_DIO0_EVENT_SET = 1,
}
impl From<DIO0_WAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: DIO0_WAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIO0_WAKEUP`"]
pub type DIO0_WAKEUP_R = crate::R<bool, DIO0_WAKEUP_A>;
impl DIO0_WAKEUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIO0_WAKEUP_A {
        match self.bits {
            false => DIO0_WAKEUP_A::WAKEUP_DIO0_EVENT_NOT_SET,
            true => DIO0_WAKEUP_A::WAKEUP_DIO0_EVENT_SET,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO0_EVENT_NOT_SET`"]
    #[inline(always)]
    pub fn is_wakeup_dio0_event_not_set(&self) -> bool {
        *self == DIO0_WAKEUP_A::WAKEUP_DIO0_EVENT_NOT_SET
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO0_EVENT_SET`"]
    #[inline(always)]
    pub fn is_wakeup_dio0_event_set(&self) -> bool {
        *self == DIO0_WAKEUP_A::WAKEUP_DIO0_EVENT_SET
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_OVERLOAD_CLEAR_AW {
    #[doc = "1: Reset the sticky WAKEUP_DCDC_OVERLOAD flag"]
    WAKEUP_DCDC_OVERLOAD_CLEAR = 1,
}
impl From<DCDC_OVERLOAD_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: DCDC_OVERLOAD_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DCDC_OVERLOAD_CLEAR`"]
pub struct DCDC_OVERLOAD_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_OVERLOAD_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCDC_OVERLOAD_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky WAKEUP_DCDC_OVERLOAD flag"]
    #[inline(always)]
    pub fn wakeup_dcdc_overload_clear(self) -> &'a mut W {
        self.variant(DCDC_OVERLOAD_CLEAR_AW::WAKEUP_DCDC_OVERLOAD_CLEAR)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_PAD_WAKEUP_CLEAR_AW {
    #[doc = "1: Reset the sticky WAKEUP_PAD_WAKEUP flag"]
    WAKEUP_PAD_EVENT_CLEAR = 1,
}
impl From<WAKEUP_PAD_WAKEUP_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_PAD_WAKEUP_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `WAKEUP_PAD_WAKEUP_CLEAR`"]
pub struct WAKEUP_PAD_WAKEUP_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_PAD_WAKEUP_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUP_PAD_WAKEUP_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky WAKEUP_PAD_WAKEUP flag"]
    #[inline(always)]
    pub fn wakeup_pad_event_clear(self) -> &'a mut W {
        self.variant(WAKEUP_PAD_WAKEUP_CLEAR_AW::WAKEUP_PAD_EVENT_CLEAR)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ALARM_WAKEUP_CLEAR_AW {
    #[doc = "1: Reset the sticky WAKEUP_RTC_ALARM flag"]
    WAKEUP_RTC_ALARM_CLEAR = 1,
}
impl From<RTC_ALARM_WAKEUP_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_ALARM_WAKEUP_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RTC_ALARM_WAKEUP_CLEAR`"]
pub struct RTC_ALARM_WAKEUP_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ALARM_WAKEUP_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_ALARM_WAKEUP_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky WAKEUP_RTC_ALARM flag"]
    #[inline(always)]
    pub fn wakeup_rtc_alarm_clear(self) -> &'a mut W {
        self.variant(RTC_ALARM_WAKEUP_CLEAR_AW::WAKEUP_RTC_ALARM_CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_TIMER_WAKEUP_CLEAR_AW {
    #[doc = "1: Reset the sticky WAKEUP_BB_TIMER flag"]
    WAKEUP_BB_TIMER_CLEAR = 1,
}
impl From<BB_TIMER_WAKEUP_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: BB_TIMER_WAKEUP_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BB_TIMER_WAKEUP_CLEAR`"]
pub struct BB_TIMER_WAKEUP_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_TIMER_WAKEUP_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_TIMER_WAKEUP_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky WAKEUP_BB_TIMER flag"]
    #[inline(always)]
    pub fn wakeup_bb_timer_clear(self) -> &'a mut W {
        self.variant(BB_TIMER_WAKEUP_CLEAR_AW::WAKEUP_BB_TIMER_CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIO3_WAKEUP_CLEAR_AW {
    #[doc = "1: Reset the sticky WAKEUP_DIO3_EVENT flag"]
    WAKEUP_DIO3_EVENT_CLEAR = 1,
}
impl From<DIO3_WAKEUP_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: DIO3_WAKEUP_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DIO3_WAKEUP_CLEAR`"]
pub struct DIO3_WAKEUP_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO3_WAKEUP_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIO3_WAKEUP_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky WAKEUP_DIO3_EVENT flag"]
    #[inline(always)]
    pub fn wakeup_dio3_event_clear(self) -> &'a mut W {
        self.variant(DIO3_WAKEUP_CLEAR_AW::WAKEUP_DIO3_EVENT_CLEAR)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIO2_WAKEUP_CLEAR_AW {
    #[doc = "1: Reset the sticky WAKEUP_DIO2_EVENT flag"]
    WAKEUP_DIO2_EVENT_CLEAR = 1,
}
impl From<DIO2_WAKEUP_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: DIO2_WAKEUP_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DIO2_WAKEUP_CLEAR`"]
pub struct DIO2_WAKEUP_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO2_WAKEUP_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIO2_WAKEUP_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky WAKEUP_DIO2_EVENT flag"]
    #[inline(always)]
    pub fn wakeup_dio2_event_clear(self) -> &'a mut W {
        self.variant(DIO2_WAKEUP_CLEAR_AW::WAKEUP_DIO2_EVENT_CLEAR)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIO1_WAKEUP_CLEAR_AW {
    #[doc = "1: Reset the sticky WAKEUP_DIO1_EVENT flag"]
    WAKEUP_DIO1_EVENT_CLEAR = 1,
}
impl From<DIO1_WAKEUP_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: DIO1_WAKEUP_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DIO1_WAKEUP_CLEAR`"]
pub struct DIO1_WAKEUP_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO1_WAKEUP_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIO1_WAKEUP_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky WAKEUP_DIO1_EVENT flag"]
    #[inline(always)]
    pub fn wakeup_dio1_event_clear(self) -> &'a mut W {
        self.variant(DIO1_WAKEUP_CLEAR_AW::WAKEUP_DIO1_EVENT_CLEAR)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIO0_WAKEUP_CLEAR_AW {
    #[doc = "1: Reset the sticky WAKEUP_DIO0_EVENT flag"]
    WAKEUP_DIO0_EVENT_CLEAR = 1,
}
impl From<DIO0_WAKEUP_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: DIO0_WAKEUP_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DIO0_WAKEUP_CLEAR`"]
pub struct DIO0_WAKEUP_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO0_WAKEUP_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIO0_WAKEUP_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky WAKEUP_DIO0_EVENT flag"]
    #[inline(always)]
    pub fn wakeup_dio0_event_clear(self) -> &'a mut W {
        self.variant(DIO0_WAKEUP_CLEAR_AW::WAKEUP_DIO0_EVENT_CLEAR)
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
    #[doc = "Bit 24 - Enable / Disable the retention mode of the pads"]
    #[inline(always)]
    pub fn pads_retention_en(&self) -> PADS_RETENTION_EN_R {
        PADS_RETENTION_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Boot mode flag"]
    #[inline(always)]
    pub fn boot_flash_app_reboot(&self) -> BOOT_FLASH_APP_REBOOT_R {
        BOOT_FLASH_APP_REBOOT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RC oscillator clock multiplier read only flag (mirror of CLOCK_MULT of ACS_RCOSC_CTRL register)"]
    #[inline(always)]
    pub fn rc_clock_mult(&self) -> RC_CLOCK_MULT_R {
        RC_CLOCK_MULT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RC oscillator trimming read only flag (mirror of FTRIM_FLAG of ACS_RCOSC_CTRL register"]
    #[inline(always)]
    pub fn rc_ftrim_flag(&self) -> RC_FTRIM_FLAG_R {
        RC_FTRIM_FLAG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Boot selection to indicate boot source"]
    #[inline(always)]
    pub fn boot_select(&self) -> BOOT_SELECT_R {
        BOOT_SELECT_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dcdc_overload_wakeup(&self) -> DCDC_OVERLOAD_WAKEUP_R {
        DCDC_OVERLOAD_WAKEUP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wakeup_pad_wakeup(&self) -> WAKEUP_PAD_WAKEUP_R {
        WAKEUP_PAD_WAKEUP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rtc_alarm_wakeup(&self) -> RTC_ALARM_WAKEUP_R {
        RTC_ALARM_WAKEUP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bb_timer_wakeup(&self) -> BB_TIMER_WAKEUP_R {
        BB_TIMER_WAKEUP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dio3_wakeup(&self) -> DIO3_WAKEUP_R {
        DIO3_WAKEUP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dio2_wakeup(&self) -> DIO2_WAKEUP_R {
        DIO2_WAKEUP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dio1_wakeup(&self) -> DIO1_WAKEUP_R {
        DIO1_WAKEUP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dio0_wakeup(&self) -> DIO0_WAKEUP_R {
        DIO0_WAKEUP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Enable / Disable the retention mode of the pads"]
    #[inline(always)]
    pub fn pads_retention_en(&mut self) -> PADS_RETENTION_EN_W {
        PADS_RETENTION_EN_W { w: self }
    }
    #[doc = "Bit 20 - Boot mode flag"]
    #[inline(always)]
    pub fn boot_flash_app_reboot(&mut self) -> BOOT_FLASH_APP_REBOOT_W {
        BOOT_FLASH_APP_REBOOT_W { w: self }
    }
    #[doc = "Bits 16:17 - Boot selection to indicate boot source"]
    #[inline(always)]
    pub fn boot_select(&mut self) -> BOOT_SELECT_W {
        BOOT_SELECT_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dcdc_overload_clear(&mut self) -> DCDC_OVERLOAD_CLEAR_W {
        DCDC_OVERLOAD_CLEAR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn wakeup_pad_wakeup_clear(&mut self) -> WAKEUP_PAD_WAKEUP_CLEAR_W {
        WAKEUP_PAD_WAKEUP_CLEAR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_alarm_wakeup_clear(&mut self) -> RTC_ALARM_WAKEUP_CLEAR_W {
        RTC_ALARM_WAKEUP_CLEAR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bb_timer_wakeup_clear(&mut self) -> BB_TIMER_WAKEUP_CLEAR_W {
        BB_TIMER_WAKEUP_CLEAR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dio3_wakeup_clear(&mut self) -> DIO3_WAKEUP_CLEAR_W {
        DIO3_WAKEUP_CLEAR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dio2_wakeup_clear(&mut self) -> DIO2_WAKEUP_CLEAR_W {
        DIO2_WAKEUP_CLEAR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dio1_wakeup_clear(&mut self) -> DIO1_WAKEUP_CLEAR_W {
        DIO1_WAKEUP_CLEAR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dio0_wakeup_clear(&mut self) -> DIO0_WAKEUP_CLEAR_W {
        DIO0_WAKEUP_CLEAR_W { w: self }
    }
}
