#[doc = "Reader of register FLASH_IF_STATUS"]
pub type R = crate::R<u32, super::FLASH_IF_STATUS>;
#[doc = "Flash trimming status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIMMED_STATUS_A {
    #[doc = "0: All NVR4 CBD0-CDB7 contents are equal to 0xFFFF. eFlash untrimmed."]
    FLASH_UNTRIMMED = 0,
    #[doc = "1: Some registers CBD0-CBD7 contents are not equal to 0xFFFF. eFlash trimmed."]
    FLASH_TRIMMED = 1,
}
impl From<TRIMMED_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: TRIMMED_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIMMED_STATUS`"]
pub type TRIMMED_STATUS_R = crate::R<bool, TRIMMED_STATUS_A>;
impl TRIMMED_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIMMED_STATUS_A {
        match self.bits {
            false => TRIMMED_STATUS_A::FLASH_UNTRIMMED,
            true => TRIMMED_STATUS_A::FLASH_TRIMMED,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_UNTRIMMED`"]
    #[inline(always)]
    pub fn is_flash_untrimmed(&self) -> bool {
        *self == TRIMMED_STATUS_A::FLASH_UNTRIMMED
    }
    #[doc = "Checks if the value of the field is `FLASH_TRIMMED`"]
    #[inline(always)]
    pub fn is_flash_trimmed(&self) -> bool {
        *self == TRIMMED_STATUS_A::FLASH_TRIMMED
    }
}
#[doc = "Flash isolate status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISOLATE_STATUS_A {
    #[doc = "0: Flash can be accessed (isolation inactive)"]
    FLASH_ACCESSIBLE = 0,
    #[doc = "1: Flash cannot be accessed (isolation active)"]
    FLASH_ISOLATE = 1,
}
impl From<ISOLATE_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: ISOLATE_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ISOLATE_STATUS`"]
pub type ISOLATE_STATUS_R = crate::R<bool, ISOLATE_STATUS_A>;
impl ISOLATE_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOLATE_STATUS_A {
        match self.bits {
            false => ISOLATE_STATUS_A::FLASH_ACCESSIBLE,
            true => ISOLATE_STATUS_A::FLASH_ISOLATE,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_flash_accessible(&self) -> bool {
        *self == ISOLATE_STATUS_A::FLASH_ACCESSIBLE
    }
    #[doc = "Checks if the value of the field is `FLASH_ISOLATE`"]
    #[inline(always)]
    pub fn is_flash_isolate(&self) -> bool {
        *self == ISOLATE_STATUS_A::FLASH_ISOLATE
    }
}
#[doc = "Request new data while in sequential program mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROG_SEQ_DATA_REQ_A {
    #[doc = "0: No new data is requested by a Sequential Program sequence"]
    FLASH_PROG_SEQ_IDLE = 0,
    #[doc = "1: New data is requested by a Sequential Program sequence"]
    FLASH_PROG_SEQ_REQ_NEW_DATA = 1,
}
impl From<PROG_SEQ_DATA_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: PROG_SEQ_DATA_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PROG_SEQ_DATA_REQ`"]
pub type PROG_SEQ_DATA_REQ_R = crate::R<bool, PROG_SEQ_DATA_REQ_A>;
impl PROG_SEQ_DATA_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROG_SEQ_DATA_REQ_A {
        match self.bits {
            false => PROG_SEQ_DATA_REQ_A::FLASH_PROG_SEQ_IDLE,
            true => PROG_SEQ_DATA_REQ_A::FLASH_PROG_SEQ_REQ_NEW_DATA,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_PROG_SEQ_IDLE`"]
    #[inline(always)]
    pub fn is_flash_prog_seq_idle(&self) -> bool {
        *self == PROG_SEQ_DATA_REQ_A::FLASH_PROG_SEQ_IDLE
    }
    #[doc = "Checks if the value of the field is `FLASH_PROG_SEQ_REQ_NEW_DATA`"]
    #[inline(always)]
    pub fn is_flash_prog_seq_req_new_data(&self) -> bool {
        *self == PROG_SEQ_DATA_REQ_A::FLASH_PROG_SEQ_REQ_NEW_DATA
    }
}
#[doc = "Flash interface busy status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: Indicates that the Flash interface is ready"]
    FLASH_IF_IDLE = 0,
    #[doc = "1: Indicates that the Flash interface is busy"]
    FLASH_IF_BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::FLASH_IF_IDLE,
            true => BUSY_A::FLASH_IF_BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_IF_IDLE`"]
    #[inline(always)]
    pub fn is_flash_if_idle(&self) -> bool {
        *self == BUSY_A::FLASH_IF_IDLE
    }
    #[doc = "Checks if the value of the field is `FLASH_IF_BUSY`"]
    #[inline(always)]
    pub fn is_flash_if_busy(&self) -> bool {
        *self == BUSY_A::FLASH_IF_BUSY
    }
}
#[doc = "Flash RED2 write unlock status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RED2_W_UNLOCK_A {
    #[doc = "0: Indicates that the Flash RED2 sector is protected against write accesses by the Flash interface"]
    FLASH_RED2_W_LOCKED = 0,
    #[doc = "1: Indicates that the Flash RED2 sector can be write accessed by the Flash interface"]
    FLASH_RED2_W_UNLOCKED = 1,
}
impl From<RED2_W_UNLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: RED2_W_UNLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RED2_W_UNLOCK`"]
pub type RED2_W_UNLOCK_R = crate::R<bool, RED2_W_UNLOCK_A>;
impl RED2_W_UNLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RED2_W_UNLOCK_A {
        match self.bits {
            false => RED2_W_UNLOCK_A::FLASH_RED2_W_LOCKED,
            true => RED2_W_UNLOCK_A::FLASH_RED2_W_UNLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_RED2_W_LOCKED`"]
    #[inline(always)]
    pub fn is_flash_red2_w_locked(&self) -> bool {
        *self == RED2_W_UNLOCK_A::FLASH_RED2_W_LOCKED
    }
    #[doc = "Checks if the value of the field is `FLASH_RED2_W_UNLOCKED`"]
    #[inline(always)]
    pub fn is_flash_red2_w_unlocked(&self) -> bool {
        *self == RED2_W_UNLOCK_A::FLASH_RED2_W_UNLOCKED
    }
}
#[doc = "Flash RED1 write unlock status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RED1_W_UNLOCK_A {
    #[doc = "0: Indicates that the Flash RED1 sector is protected against write accesses by the Flash interface"]
    FLASH_RED1_W_LOCKED = 0,
    #[doc = "1: Indicates that the Flash RED1 sector can be write accessed by the Flash interface"]
    FLASH_RED1_W_UNLOCKED = 1,
}
impl From<RED1_W_UNLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: RED1_W_UNLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RED1_W_UNLOCK`"]
pub type RED1_W_UNLOCK_R = crate::R<bool, RED1_W_UNLOCK_A>;
impl RED1_W_UNLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RED1_W_UNLOCK_A {
        match self.bits {
            false => RED1_W_UNLOCK_A::FLASH_RED1_W_LOCKED,
            true => RED1_W_UNLOCK_A::FLASH_RED1_W_UNLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_RED1_W_LOCKED`"]
    #[inline(always)]
    pub fn is_flash_red1_w_locked(&self) -> bool {
        *self == RED1_W_UNLOCK_A::FLASH_RED1_W_LOCKED
    }
    #[doc = "Checks if the value of the field is `FLASH_RED1_W_UNLOCKED`"]
    #[inline(always)]
    pub fn is_flash_red1_w_unlocked(&self) -> bool {
        *self == RED1_W_UNLOCK_A::FLASH_RED1_W_UNLOCKED
    }
}
#[doc = "Flash NVR3 write unlock status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NVR3_W_UNLOCK_A {
    #[doc = "0: Indicates that the Flash NVR3 sector is protected against write accesses by the Flash interface"]
    FLASH_NVR3_W_LOCKED = 0,
    #[doc = "1: Indicates that the Flash NVR3 sector can be write accessed by the Flash interface"]
    FLASH_NVR3_W_UNLOCKED = 1,
}
impl From<NVR3_W_UNLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: NVR3_W_UNLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NVR3_W_UNLOCK`"]
pub type NVR3_W_UNLOCK_R = crate::R<bool, NVR3_W_UNLOCK_A>;
impl NVR3_W_UNLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NVR3_W_UNLOCK_A {
        match self.bits {
            false => NVR3_W_UNLOCK_A::FLASH_NVR3_W_LOCKED,
            true => NVR3_W_UNLOCK_A::FLASH_NVR3_W_UNLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_NVR3_W_LOCKED`"]
    #[inline(always)]
    pub fn is_flash_nvr3_w_locked(&self) -> bool {
        *self == NVR3_W_UNLOCK_A::FLASH_NVR3_W_LOCKED
    }
    #[doc = "Checks if the value of the field is `FLASH_NVR3_W_UNLOCKED`"]
    #[inline(always)]
    pub fn is_flash_nvr3_w_unlocked(&self) -> bool {
        *self == NVR3_W_UNLOCK_A::FLASH_NVR3_W_UNLOCKED
    }
}
#[doc = "Flash NVR2 write unlock status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NVR2_W_UNLOCK_A {
    #[doc = "0: Indicates that the Flash NVR2 sector is protected against write accesses by the Flash interface"]
    FLASH_NVR2_W_LOCKED = 0,
    #[doc = "1: Indicates that the Flash NVR2 sector can be write accessed by the Flash interface"]
    FLASH_NVR2_W_UNLOCKED = 1,
}
impl From<NVR2_W_UNLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: NVR2_W_UNLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NVR2_W_UNLOCK`"]
pub type NVR2_W_UNLOCK_R = crate::R<bool, NVR2_W_UNLOCK_A>;
impl NVR2_W_UNLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NVR2_W_UNLOCK_A {
        match self.bits {
            false => NVR2_W_UNLOCK_A::FLASH_NVR2_W_LOCKED,
            true => NVR2_W_UNLOCK_A::FLASH_NVR2_W_UNLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_NVR2_W_LOCKED`"]
    #[inline(always)]
    pub fn is_flash_nvr2_w_locked(&self) -> bool {
        *self == NVR2_W_UNLOCK_A::FLASH_NVR2_W_LOCKED
    }
    #[doc = "Checks if the value of the field is `FLASH_NVR2_W_UNLOCKED`"]
    #[inline(always)]
    pub fn is_flash_nvr2_w_unlocked(&self) -> bool {
        *self == NVR2_W_UNLOCK_A::FLASH_NVR2_W_UNLOCKED
    }
}
#[doc = "Flash NVR1 write unlock status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NVR1_W_UNLOCK_A {
    #[doc = "0: Indicates that the Flash NVR1 sector is protected against write accesses by the Flash interface"]
    FLASH_NVR1_W_LOCKED = 0,
    #[doc = "1: Indicates that the Flash NVR1 sector can be write accessed by the Flash interface"]
    FLASH_NVR1_W_UNLOCKED = 1,
}
impl From<NVR1_W_UNLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: NVR1_W_UNLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NVR1_W_UNLOCK`"]
pub type NVR1_W_UNLOCK_R = crate::R<bool, NVR1_W_UNLOCK_A>;
impl NVR1_W_UNLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NVR1_W_UNLOCK_A {
        match self.bits {
            false => NVR1_W_UNLOCK_A::FLASH_NVR1_W_LOCKED,
            true => NVR1_W_UNLOCK_A::FLASH_NVR1_W_UNLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_NVR1_W_LOCKED`"]
    #[inline(always)]
    pub fn is_flash_nvr1_w_locked(&self) -> bool {
        *self == NVR1_W_UNLOCK_A::FLASH_NVR1_W_LOCKED
    }
    #[doc = "Checks if the value of the field is `FLASH_NVR1_W_UNLOCKED`"]
    #[inline(always)]
    pub fn is_flash_nvr1_w_unlocked(&self) -> bool {
        *self == NVR1_W_UNLOCK_A::FLASH_NVR1_W_UNLOCKED
    }
}
#[doc = "Write unlock status bit of the high part of the Flash MAIN block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAIN_HIGH_W_UNLOCK_A {
    #[doc = "0: Indicates that the high part of the Flash MAIN section is protected against write accesses by the Flash interface"]
    FLASH_MAIN_HIGH_W_LOCKED = 0,
    #[doc = "1: Indicates that the high part of the Flash MAIN section can be write accessed by the Flash interface"]
    FLASH_MAIN_HIGH_W_UNLOCKED = 1,
}
impl From<MAIN_HIGH_W_UNLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: MAIN_HIGH_W_UNLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAIN_HIGH_W_UNLOCK`"]
pub type MAIN_HIGH_W_UNLOCK_R = crate::R<bool, MAIN_HIGH_W_UNLOCK_A>;
impl MAIN_HIGH_W_UNLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAIN_HIGH_W_UNLOCK_A {
        match self.bits {
            false => MAIN_HIGH_W_UNLOCK_A::FLASH_MAIN_HIGH_W_LOCKED,
            true => MAIN_HIGH_W_UNLOCK_A::FLASH_MAIN_HIGH_W_UNLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_MAIN_HIGH_W_LOCKED`"]
    #[inline(always)]
    pub fn is_flash_main_high_w_locked(&self) -> bool {
        *self == MAIN_HIGH_W_UNLOCK_A::FLASH_MAIN_HIGH_W_LOCKED
    }
    #[doc = "Checks if the value of the field is `FLASH_MAIN_HIGH_W_UNLOCKED`"]
    #[inline(always)]
    pub fn is_flash_main_high_w_unlocked(&self) -> bool {
        *self == MAIN_HIGH_W_UNLOCK_A::FLASH_MAIN_HIGH_W_UNLOCKED
    }
}
#[doc = "Write unlock status bit of the middle part of the Flash MAIN block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAIN_MIDDLE_W_UNLOCK_A {
    #[doc = "0: Indicates that the middle part of the Flash MAIN section is protected against write accesses by the Flash interface"]
    FLASH_MAIN_MIDDLE_W_LOCKED = 0,
    #[doc = "1: Indicates that the middle part of the Flash MAIN section can be write accessed by the Flash interface"]
    FLASH_MAIN_MIDDLE_W_UNLOCKED = 1,
}
impl From<MAIN_MIDDLE_W_UNLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: MAIN_MIDDLE_W_UNLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAIN_MIDDLE_W_UNLOCK`"]
pub type MAIN_MIDDLE_W_UNLOCK_R = crate::R<bool, MAIN_MIDDLE_W_UNLOCK_A>;
impl MAIN_MIDDLE_W_UNLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAIN_MIDDLE_W_UNLOCK_A {
        match self.bits {
            false => MAIN_MIDDLE_W_UNLOCK_A::FLASH_MAIN_MIDDLE_W_LOCKED,
            true => MAIN_MIDDLE_W_UNLOCK_A::FLASH_MAIN_MIDDLE_W_UNLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_MAIN_MIDDLE_W_LOCKED`"]
    #[inline(always)]
    pub fn is_flash_main_middle_w_locked(&self) -> bool {
        *self == MAIN_MIDDLE_W_UNLOCK_A::FLASH_MAIN_MIDDLE_W_LOCKED
    }
    #[doc = "Checks if the value of the field is `FLASH_MAIN_MIDDLE_W_UNLOCKED`"]
    #[inline(always)]
    pub fn is_flash_main_middle_w_unlocked(&self) -> bool {
        *self == MAIN_MIDDLE_W_UNLOCK_A::FLASH_MAIN_MIDDLE_W_UNLOCKED
    }
}
#[doc = "Write unlock status bit of the lower part of the Flash MAIN block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAIN_LOW_W_UNLOCK_A {
    #[doc = "0: Indicates that the lower part of the Flash MAIN section is protected against write accesses by the Flash interface"]
    FLASH_MAIN_LOW_W_LOCKED = 0,
    #[doc = "1: Indicates that the lower part of the Flash MAIN section can be write accessed by the Flash interface"]
    FLASH_MAIN_LOW_W_UNLOCKED = 1,
}
impl From<MAIN_LOW_W_UNLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: MAIN_LOW_W_UNLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAIN_LOW_W_UNLOCK`"]
pub type MAIN_LOW_W_UNLOCK_R = crate::R<bool, MAIN_LOW_W_UNLOCK_A>;
impl MAIN_LOW_W_UNLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAIN_LOW_W_UNLOCK_A {
        match self.bits {
            false => MAIN_LOW_W_UNLOCK_A::FLASH_MAIN_LOW_W_LOCKED,
            true => MAIN_LOW_W_UNLOCK_A::FLASH_MAIN_LOW_W_UNLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_MAIN_LOW_W_LOCKED`"]
    #[inline(always)]
    pub fn is_flash_main_low_w_locked(&self) -> bool {
        *self == MAIN_LOW_W_UNLOCK_A::FLASH_MAIN_LOW_W_LOCKED
    }
    #[doc = "Checks if the value of the field is `FLASH_MAIN_LOW_W_UNLOCKED`"]
    #[inline(always)]
    pub fn is_flash_main_low_w_unlocked(&self) -> bool {
        *self == MAIN_LOW_W_UNLOCK_A::FLASH_MAIN_LOW_W_UNLOCKED
    }
}
impl R {
    #[doc = "Bit 13 - Flash trimming status"]
    #[inline(always)]
    pub fn trimmed_status(&self) -> TRIMMED_STATUS_R {
        TRIMMED_STATUS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Flash isolate status"]
    #[inline(always)]
    pub fn isolate_status(&self) -> ISOLATE_STATUS_R {
        ISOLATE_STATUS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Request new data while in sequential program mode"]
    #[inline(always)]
    pub fn prog_seq_data_req(&self) -> PROG_SEQ_DATA_REQ_R {
        PROG_SEQ_DATA_REQ_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Flash interface busy status bit"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Flash RED2 write unlock status bit"]
    #[inline(always)]
    pub fn red2_w_unlock(&self) -> RED2_W_UNLOCK_R {
        RED2_W_UNLOCK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flash RED1 write unlock status bit"]
    #[inline(always)]
    pub fn red1_w_unlock(&self) -> RED1_W_UNLOCK_R {
        RED1_W_UNLOCK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Flash NVR3 write unlock status bit"]
    #[inline(always)]
    pub fn nvr3_w_unlock(&self) -> NVR3_W_UNLOCK_R {
        NVR3_W_UNLOCK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Flash NVR2 write unlock status bit"]
    #[inline(always)]
    pub fn nvr2_w_unlock(&self) -> NVR2_W_UNLOCK_R {
        NVR2_W_UNLOCK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flash NVR1 write unlock status bit"]
    #[inline(always)]
    pub fn nvr1_w_unlock(&self) -> NVR1_W_UNLOCK_R {
        NVR1_W_UNLOCK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write unlock status bit of the high part of the Flash MAIN block"]
    #[inline(always)]
    pub fn main_high_w_unlock(&self) -> MAIN_HIGH_W_UNLOCK_R {
        MAIN_HIGH_W_UNLOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write unlock status bit of the middle part of the Flash MAIN block"]
    #[inline(always)]
    pub fn main_middle_w_unlock(&self) -> MAIN_MIDDLE_W_UNLOCK_R {
        MAIN_MIDDLE_W_UNLOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Write unlock status bit of the lower part of the Flash MAIN block"]
    #[inline(always)]
    pub fn main_low_w_unlock(&self) -> MAIN_LOW_W_UNLOCK_R {
        MAIN_LOW_W_UNLOCK_R::new((self.bits & 0x01) != 0)
    }
}
