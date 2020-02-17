#[doc = "Reader of register FLASH_IF_CTRL"]
pub type R = crate::R<u32, super::FLASH_IF_CTRL>;
#[doc = "Writer for register FLASH_IF_CTRL"]
pub type W = crate::W<u32, super::FLASH_IF_CTRL>;
#[doc = "Register FLASH_IF_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_IF_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pre-fetch on D-Bus control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREFETCH_DBUS_A {
    #[doc = "0: Not pre-fetch the n+1 address on D-Bus"]
    FLASH_PREFETCH_DBUS_DISABLE = 0,
    #[doc = "1: Pre-fetch the n+1 address on D-Bus"]
    FLASH_PREFETCH_DBUS_ENABLE = 1,
}
impl From<PREFETCH_DBUS_A> for bool {
    #[inline(always)]
    fn from(variant: PREFETCH_DBUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PREFETCH_DBUS`"]
pub type PREFETCH_DBUS_R = crate::R<bool, PREFETCH_DBUS_A>;
impl PREFETCH_DBUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREFETCH_DBUS_A {
        match self.bits {
            false => PREFETCH_DBUS_A::FLASH_PREFETCH_DBUS_DISABLE,
            true => PREFETCH_DBUS_A::FLASH_PREFETCH_DBUS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_PREFETCH_DBUS_DISABLE`"]
    #[inline(always)]
    pub fn is_flash_prefetch_dbus_disable(&self) -> bool {
        *self == PREFETCH_DBUS_A::FLASH_PREFETCH_DBUS_DISABLE
    }
    #[doc = "Checks if the value of the field is `FLASH_PREFETCH_DBUS_ENABLE`"]
    #[inline(always)]
    pub fn is_flash_prefetch_dbus_enable(&self) -> bool {
        *self == PREFETCH_DBUS_A::FLASH_PREFETCH_DBUS_ENABLE
    }
}
#[doc = "Write proxy for field `PREFETCH_DBUS`"]
pub struct PREFETCH_DBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFETCH_DBUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREFETCH_DBUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not pre-fetch the n+1 address on D-Bus"]
    #[inline(always)]
    pub fn flash_prefetch_dbus_disable(self) -> &'a mut W {
        self.variant(PREFETCH_DBUS_A::FLASH_PREFETCH_DBUS_DISABLE)
    }
    #[doc = "Pre-fetch the n+1 address on D-Bus"]
    #[inline(always)]
    pub fn flash_prefetch_dbus_enable(self) -> &'a mut W {
        self.variant(PREFETCH_DBUS_A::FLASH_PREFETCH_DBUS_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Pre-fetch on I-Bus control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREFETCH_IBUS_A {
    #[doc = "0: Not pre-fetch the n+1 address on I-Bus"]
    FLASH_PREFETCH_IBUS_DISABLE = 0,
    #[doc = "1: Pre-fetch the n+1 address on I-Bus"]
    FLASH_PREFETCH_IBUS_ENABLE = 1,
}
impl From<PREFETCH_IBUS_A> for bool {
    #[inline(always)]
    fn from(variant: PREFETCH_IBUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PREFETCH_IBUS`"]
pub type PREFETCH_IBUS_R = crate::R<bool, PREFETCH_IBUS_A>;
impl PREFETCH_IBUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREFETCH_IBUS_A {
        match self.bits {
            false => PREFETCH_IBUS_A::FLASH_PREFETCH_IBUS_DISABLE,
            true => PREFETCH_IBUS_A::FLASH_PREFETCH_IBUS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_PREFETCH_IBUS_DISABLE`"]
    #[inline(always)]
    pub fn is_flash_prefetch_ibus_disable(&self) -> bool {
        *self == PREFETCH_IBUS_A::FLASH_PREFETCH_IBUS_DISABLE
    }
    #[doc = "Checks if the value of the field is `FLASH_PREFETCH_IBUS_ENABLE`"]
    #[inline(always)]
    pub fn is_flash_prefetch_ibus_enable(&self) -> bool {
        *self == PREFETCH_IBUS_A::FLASH_PREFETCH_IBUS_ENABLE
    }
}
#[doc = "Write proxy for field `PREFETCH_IBUS`"]
pub struct PREFETCH_IBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFETCH_IBUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREFETCH_IBUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not pre-fetch the n+1 address on I-Bus"]
    #[inline(always)]
    pub fn flash_prefetch_ibus_disable(self) -> &'a mut W {
        self.variant(PREFETCH_IBUS_A::FLASH_PREFETCH_IBUS_DISABLE)
    }
    #[doc = "Pre-fetch the n+1 address on I-Bus"]
    #[inline(always)]
    pub fn flash_prefetch_ibus_enable(self) -> &'a mut W {
        self.variant(PREFETCH_IBUS_A::FLASH_PREFETCH_IBUS_ENABLE)
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
#[doc = "Do not automatically load the configuration registers and the patch information from NVR4 sector after the command WAKEUP is completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOT_LOAD_AUTO_A {
    #[doc = "0: No automatic load done after the WAKEUP command"]
    FLASH_LOAD_AUTO_ENABLE = 0,
    #[doc = "1: The CMD_WAKEUP includes the loading of internal registers and patch information."]
    FLASH_LOAD_AUTO_DISABLE = 1,
}
impl From<NOT_LOAD_AUTO_A> for bool {
    #[inline(always)]
    fn from(variant: NOT_LOAD_AUTO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NOT_LOAD_AUTO`"]
pub type NOT_LOAD_AUTO_R = crate::R<bool, NOT_LOAD_AUTO_A>;
impl NOT_LOAD_AUTO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOT_LOAD_AUTO_A {
        match self.bits {
            false => NOT_LOAD_AUTO_A::FLASH_LOAD_AUTO_ENABLE,
            true => NOT_LOAD_AUTO_A::FLASH_LOAD_AUTO_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_LOAD_AUTO_ENABLE`"]
    #[inline(always)]
    pub fn is_flash_load_auto_enable(&self) -> bool {
        *self == NOT_LOAD_AUTO_A::FLASH_LOAD_AUTO_ENABLE
    }
    #[doc = "Checks if the value of the field is `FLASH_LOAD_AUTO_DISABLE`"]
    #[inline(always)]
    pub fn is_flash_load_auto_disable(&self) -> bool {
        *self == NOT_LOAD_AUTO_A::FLASH_LOAD_AUTO_DISABLE
    }
}
#[doc = "Write proxy for field `NOT_LOAD_AUTO`"]
pub struct NOT_LOAD_AUTO_W<'a> {
    w: &'a mut W,
}
impl<'a> NOT_LOAD_AUTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOT_LOAD_AUTO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No automatic load done after the WAKEUP command"]
    #[inline(always)]
    pub fn flash_load_auto_enable(self) -> &'a mut W {
        self.variant(NOT_LOAD_AUTO_A::FLASH_LOAD_AUTO_ENABLE)
    }
    #[doc = "The CMD_WAKEUP includes the loading of internal registers and patch information."]
    #[inline(always)]
    pub fn flash_load_auto_disable(self) -> &'a mut W {
        self.variant(NOT_LOAD_AUTO_A::FLASH_LOAD_AUTO_DISABLE)
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
#[doc = "Control VREAD1: Read data after erase with more stringent condition than normal read. Changing this bit will execute the CMD_SET_VREAD1 or CMD_UNSET_VREAD1 command.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREAD1_MODE_A {
    #[doc = "0: After erase, read data with a normal condition"]
    FLASH_VREAD1_DISABLE = 0,
    #[doc = "1: After erase, read data with a more stringent condition"]
    FLASH_VREAD1_ENABLE = 1,
}
impl From<VREAD1_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: VREAD1_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VREAD1_MODE`"]
pub type VREAD1_MODE_R = crate::R<bool, VREAD1_MODE_A>;
impl VREAD1_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREAD1_MODE_A {
        match self.bits {
            false => VREAD1_MODE_A::FLASH_VREAD1_DISABLE,
            true => VREAD1_MODE_A::FLASH_VREAD1_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_VREAD1_DISABLE`"]
    #[inline(always)]
    pub fn is_flash_vread1_disable(&self) -> bool {
        *self == VREAD1_MODE_A::FLASH_VREAD1_DISABLE
    }
    #[doc = "Checks if the value of the field is `FLASH_VREAD1_ENABLE`"]
    #[inline(always)]
    pub fn is_flash_vread1_enable(&self) -> bool {
        *self == VREAD1_MODE_A::FLASH_VREAD1_ENABLE
    }
}
#[doc = "Write proxy for field `VREAD1_MODE`"]
pub struct VREAD1_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> VREAD1_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREAD1_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "After erase, read data with a normal condition"]
    #[inline(always)]
    pub fn flash_vread1_disable(self) -> &'a mut W {
        self.variant(VREAD1_MODE_A::FLASH_VREAD1_DISABLE)
    }
    #[doc = "After erase, read data with a more stringent condition"]
    #[inline(always)]
    pub fn flash_vread1_enable(self) -> &'a mut W {
        self.variant(VREAD1_MODE_A::FLASH_VREAD1_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Control VREAD0: Read data after program with more stringent condition than normal read. Changing this bit will execute the CMD_SET_VREAD0 or CMD_UNSET_VREAD0 command.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREAD0_MODE_A {
    #[doc = "0: After programming, read data with a normal condition"]
    FLASH_VREAD0_DISABLE = 0,
    #[doc = "1: After programming, read data with a more stringent condition"]
    FLASH_VREAD0_ENABLE = 1,
}
impl From<VREAD0_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: VREAD0_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VREAD0_MODE`"]
pub type VREAD0_MODE_R = crate::R<bool, VREAD0_MODE_A>;
impl VREAD0_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREAD0_MODE_A {
        match self.bits {
            false => VREAD0_MODE_A::FLASH_VREAD0_DISABLE,
            true => VREAD0_MODE_A::FLASH_VREAD0_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_VREAD0_DISABLE`"]
    #[inline(always)]
    pub fn is_flash_vread0_disable(&self) -> bool {
        *self == VREAD0_MODE_A::FLASH_VREAD0_DISABLE
    }
    #[doc = "Checks if the value of the field is `FLASH_VREAD0_ENABLE`"]
    #[inline(always)]
    pub fn is_flash_vread0_enable(&self) -> bool {
        *self == VREAD0_MODE_A::FLASH_VREAD0_ENABLE
    }
}
#[doc = "Write proxy for field `VREAD0_MODE`"]
pub struct VREAD0_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> VREAD0_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREAD0_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "After programming, read data with a normal condition"]
    #[inline(always)]
    pub fn flash_vread0_disable(self) -> &'a mut W {
        self.variant(VREAD0_MODE_A::FLASH_VREAD0_DISABLE)
    }
    #[doc = "After programming, read data with a more stringent condition"]
    #[inline(always)]
    pub fn flash_vread0_enable(self) -> &'a mut W {
        self.variant(VREAD0_MODE_A::FLASH_VREAD0_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Set the recall pins mode during CMD_READ. Changing this bit will execute the CMD_SET_RECALL or CMD_UNSET_RECALL command.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECALL_A {
    #[doc = "0: RECALL pin low during read command"]
    FLASH_RECALL_DISABLE = 0,
    #[doc = "1: RECALL pin high during read command"]
    FLASH_RECALL_ENABLE = 1,
}
impl From<RECALL_A> for bool {
    #[inline(always)]
    fn from(variant: RECALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RECALL`"]
pub type RECALL_R = crate::R<bool, RECALL_A>;
impl RECALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECALL_A {
        match self.bits {
            false => RECALL_A::FLASH_RECALL_DISABLE,
            true => RECALL_A::FLASH_RECALL_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_RECALL_DISABLE`"]
    #[inline(always)]
    pub fn is_flash_recall_disable(&self) -> bool {
        *self == RECALL_A::FLASH_RECALL_DISABLE
    }
    #[doc = "Checks if the value of the field is `FLASH_RECALL_ENABLE`"]
    #[inline(always)]
    pub fn is_flash_recall_enable(&self) -> bool {
        *self == RECALL_A::FLASH_RECALL_ENABLE
    }
}
#[doc = "Write proxy for field `RECALL`"]
pub struct RECALL_W<'a> {
    w: &'a mut W,
}
impl<'a> RECALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECALL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RECALL pin low during read command"]
    #[inline(always)]
    pub fn flash_recall_disable(self) -> &'a mut W {
        self.variant(RECALL_A::FLASH_RECALL_DISABLE)
    }
    #[doc = "RECALL pin high during read command"]
    #[inline(always)]
    pub fn flash_recall_enable(self) -> &'a mut W {
        self.variant(RECALL_A::FLASH_RECALL_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Configures the erase retry iteration. This impacts the eFlash endurance time. Also used by Flash programming.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RETRY_A {
    #[doc = "0: for 1st erase pulse"]
    FLASH_RETRY_1 = 0,
    #[doc = "1: for 2nd erase pulse"]
    FLASH_RETRY_2 = 1,
    #[doc = "2: for 3rd erase pulse"]
    FLASH_RETRY_3 = 2,
    #[doc = "3: for 4th erase pulse or required during programming"]
    FLASH_RETRY_4 = 3,
}
impl From<RETRY_A> for u8 {
    #[inline(always)]
    fn from(variant: RETRY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RETRY`"]
pub type RETRY_R = crate::R<u8, RETRY_A>;
impl RETRY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RETRY_A {
        match self.bits {
            0 => RETRY_A::FLASH_RETRY_1,
            1 => RETRY_A::FLASH_RETRY_2,
            2 => RETRY_A::FLASH_RETRY_3,
            3 => RETRY_A::FLASH_RETRY_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_RETRY_1`"]
    #[inline(always)]
    pub fn is_flash_retry_1(&self) -> bool {
        *self == RETRY_A::FLASH_RETRY_1
    }
    #[doc = "Checks if the value of the field is `FLASH_RETRY_2`"]
    #[inline(always)]
    pub fn is_flash_retry_2(&self) -> bool {
        *self == RETRY_A::FLASH_RETRY_2
    }
    #[doc = "Checks if the value of the field is `FLASH_RETRY_3`"]
    #[inline(always)]
    pub fn is_flash_retry_3(&self) -> bool {
        *self == RETRY_A::FLASH_RETRY_3
    }
    #[doc = "Checks if the value of the field is `FLASH_RETRY_4`"]
    #[inline(always)]
    pub fn is_flash_retry_4(&self) -> bool {
        *self == RETRY_A::FLASH_RETRY_4
    }
}
#[doc = "Write proxy for field `RETRY`"]
pub struct RETRY_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RETRY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "for 1st erase pulse"]
    #[inline(always)]
    pub fn flash_retry_1(self) -> &'a mut W {
        self.variant(RETRY_A::FLASH_RETRY_1)
    }
    #[doc = "for 2nd erase pulse"]
    #[inline(always)]
    pub fn flash_retry_2(self) -> &'a mut W {
        self.variant(RETRY_A::FLASH_RETRY_2)
    }
    #[doc = "for 3rd erase pulse"]
    #[inline(always)]
    pub fn flash_retry_3(self) -> &'a mut W {
        self.variant(RETRY_A::FLASH_RETRY_3)
    }
    #[doc = "for 4th erase pulse or required during programming"]
    #[inline(always)]
    pub fn flash_retry_4(self) -> &'a mut W {
        self.variant(RETRY_A::FLASH_RETRY_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Set the low power mode. Changing this bit will execute the CMD_SET_LOW_POWER or CMD_UNSET_LOW_POWER command.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP_MODE_A {
    #[doc = "0: Disable the Flash low power mode"]
    FLASH_LOW_POWER_DISABLE = 0,
    #[doc = "1: Enable the Flash low power mode"]
    FLASH_LOW_POWER_ENABLE = 1,
}
impl From<LP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LP_MODE`"]
pub type LP_MODE_R = crate::R<bool, LP_MODE_A>;
impl LP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP_MODE_A {
        match self.bits {
            false => LP_MODE_A::FLASH_LOW_POWER_DISABLE,
            true => LP_MODE_A::FLASH_LOW_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_LOW_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_flash_low_power_disable(&self) -> bool {
        *self == LP_MODE_A::FLASH_LOW_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `FLASH_LOW_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_flash_low_power_enable(&self) -> bool {
        *self == LP_MODE_A::FLASH_LOW_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `LP_MODE`"]
pub struct LP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the Flash low power mode"]
    #[inline(always)]
    pub fn flash_low_power_disable(self) -> &'a mut W {
        self.variant(LP_MODE_A::FLASH_LOW_POWER_DISABLE)
    }
    #[doc = "Enable the Flash low power mode"]
    #[inline(always)]
    pub fn flash_low_power_enable(self) -> &'a mut W {
        self.variant(LP_MODE_A::FLASH_LOW_POWER_ENABLE)
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
    #[doc = "Bit 18 - Pre-fetch on D-Bus control"]
    #[inline(always)]
    pub fn prefetch_dbus(&self) -> PREFETCH_DBUS_R {
        PREFETCH_DBUS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pre-fetch on I-Bus control"]
    #[inline(always)]
    pub fn prefetch_ibus(&self) -> PREFETCH_IBUS_R {
        PREFETCH_IBUS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Do not automatically load the configuration registers and the patch information from NVR4 sector after the command WAKEUP is completed."]
    #[inline(always)]
    pub fn not_load_auto(&self) -> NOT_LOAD_AUTO_R {
        NOT_LOAD_AUTO_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Control VREAD1: Read data after erase with more stringent condition than normal read. Changing this bit will execute the CMD_SET_VREAD1 or CMD_UNSET_VREAD1 command."]
    #[inline(always)]
    pub fn vread1_mode(&self) -> VREAD1_MODE_R {
        VREAD1_MODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Control VREAD0: Read data after program with more stringent condition than normal read. Changing this bit will execute the CMD_SET_VREAD0 or CMD_UNSET_VREAD0 command."]
    #[inline(always)]
    pub fn vread0_mode(&self) -> VREAD0_MODE_R {
        VREAD0_MODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Set the recall pins mode during CMD_READ. Changing this bit will execute the CMD_SET_RECALL or CMD_UNSET_RECALL command."]
    #[inline(always)]
    pub fn recall(&self) -> RECALL_R {
        RECALL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Configures the erase retry iteration. This impacts the eFlash endurance time. Also used by Flash programming."]
    #[inline(always)]
    pub fn retry(&self) -> RETRY_R {
        RETRY_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 0 - Set the low power mode. Changing this bit will execute the CMD_SET_LOW_POWER or CMD_UNSET_LOW_POWER command."]
    #[inline(always)]
    pub fn lp_mode(&self) -> LP_MODE_R {
        LP_MODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Pre-fetch on D-Bus control"]
    #[inline(always)]
    pub fn prefetch_dbus(&mut self) -> PREFETCH_DBUS_W {
        PREFETCH_DBUS_W { w: self }
    }
    #[doc = "Bit 17 - Pre-fetch on I-Bus control"]
    #[inline(always)]
    pub fn prefetch_ibus(&mut self) -> PREFETCH_IBUS_W {
        PREFETCH_IBUS_W { w: self }
    }
    #[doc = "Bit 16 - Do not automatically load the configuration registers and the patch information from NVR4 sector after the command WAKEUP is completed."]
    #[inline(always)]
    pub fn not_load_auto(&mut self) -> NOT_LOAD_AUTO_W {
        NOT_LOAD_AUTO_W { w: self }
    }
    #[doc = "Bit 12 - Control VREAD1: Read data after erase with more stringent condition than normal read. Changing this bit will execute the CMD_SET_VREAD1 or CMD_UNSET_VREAD1 command."]
    #[inline(always)]
    pub fn vread1_mode(&mut self) -> VREAD1_MODE_W {
        VREAD1_MODE_W { w: self }
    }
    #[doc = "Bit 11 - Control VREAD0: Read data after program with more stringent condition than normal read. Changing this bit will execute the CMD_SET_VREAD0 or CMD_UNSET_VREAD0 command."]
    #[inline(always)]
    pub fn vread0_mode(&mut self) -> VREAD0_MODE_W {
        VREAD0_MODE_W { w: self }
    }
    #[doc = "Bit 10 - Set the recall pins mode during CMD_READ. Changing this bit will execute the CMD_SET_RECALL or CMD_UNSET_RECALL command."]
    #[inline(always)]
    pub fn recall(&mut self) -> RECALL_W {
        RECALL_W { w: self }
    }
    #[doc = "Bits 8:9 - Configures the erase retry iteration. This impacts the eFlash endurance time. Also used by Flash programming."]
    #[inline(always)]
    pub fn retry(&mut self) -> RETRY_W {
        RETRY_W { w: self }
    }
    #[doc = "Bit 0 - Set the low power mode. Changing this bit will execute the CMD_SET_LOW_POWER or CMD_UNSET_LOW_POWER command."]
    #[inline(always)]
    pub fn lp_mode(&mut self) -> LP_MODE_W {
        LP_MODE_W { w: self }
    }
}
