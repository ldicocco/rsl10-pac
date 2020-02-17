#[doc = "Reader of register ACS_WAKEUP_STATE"]
pub type R = crate::R<u32, super::ACS_WAKEUP_STATE>;
#[doc = "Status register indicates the last wake-up source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAKEUP_SRC_A {
    #[doc = "7: The last wake-up was due to the DCDC overload"]
    WAKEUP_DUE_TO_DCDC_OVERLOAD = 7,
    #[doc = "6: The last wake-up was due to the WAKEUP pad"]
    WAKEUP_DUE_TO_WAKEUP_PAD = 6,
    #[doc = "5: The last wake-up was due to the RTC Timer alarm"]
    WAKEUP_DUE_TO_RTC_ALARM = 5,
    #[doc = "4: The last wake-up was due to the baseband timer alarm"]
    WAKEUP_DUE_TO_BB_TIMER = 4,
    #[doc = "3: The last wake-up was due to the DIO3 pad"]
    WAKEUP_DUE_TO_DIO3 = 3,
    #[doc = "2: The last wake-up was due to the DIO2 pad"]
    WAKEUP_DUE_TO_DIO2 = 2,
    #[doc = "1: The last wake-up was due to the DIO1 pad"]
    WAKEUP_DUE_TO_DIO1 = 1,
    #[doc = "0: The last wake-up was due to the DIO0 pad"]
    WAKEUP_DUE_TO_DIO0 = 0,
}
impl From<WAKEUP_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: WAKEUP_SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WAKEUP_SRC`"]
pub type WAKEUP_SRC_R = crate::R<u8, WAKEUP_SRC_A>;
impl WAKEUP_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP_SRC_A {
        match self.bits {
            7 => WAKEUP_SRC_A::WAKEUP_DUE_TO_DCDC_OVERLOAD,
            6 => WAKEUP_SRC_A::WAKEUP_DUE_TO_WAKEUP_PAD,
            5 => WAKEUP_SRC_A::WAKEUP_DUE_TO_RTC_ALARM,
            4 => WAKEUP_SRC_A::WAKEUP_DUE_TO_BB_TIMER,
            3 => WAKEUP_SRC_A::WAKEUP_DUE_TO_DIO3,
            2 => WAKEUP_SRC_A::WAKEUP_DUE_TO_DIO2,
            1 => WAKEUP_SRC_A::WAKEUP_DUE_TO_DIO1,
            0 => WAKEUP_SRC_A::WAKEUP_DUE_TO_DIO0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DUE_TO_DCDC_OVERLOAD`"]
    #[inline(always)]
    pub fn is_wakeup_due_to_dcdc_overload(&self) -> bool {
        *self == WAKEUP_SRC_A::WAKEUP_DUE_TO_DCDC_OVERLOAD
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DUE_TO_WAKEUP_PAD`"]
    #[inline(always)]
    pub fn is_wakeup_due_to_wakeup_pad(&self) -> bool {
        *self == WAKEUP_SRC_A::WAKEUP_DUE_TO_WAKEUP_PAD
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DUE_TO_RTC_ALARM`"]
    #[inline(always)]
    pub fn is_wakeup_due_to_rtc_alarm(&self) -> bool {
        *self == WAKEUP_SRC_A::WAKEUP_DUE_TO_RTC_ALARM
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DUE_TO_BB_TIMER`"]
    #[inline(always)]
    pub fn is_wakeup_due_to_bb_timer(&self) -> bool {
        *self == WAKEUP_SRC_A::WAKEUP_DUE_TO_BB_TIMER
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DUE_TO_DIO3`"]
    #[inline(always)]
    pub fn is_wakeup_due_to_dio3(&self) -> bool {
        *self == WAKEUP_SRC_A::WAKEUP_DUE_TO_DIO3
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DUE_TO_DIO2`"]
    #[inline(always)]
    pub fn is_wakeup_due_to_dio2(&self) -> bool {
        *self == WAKEUP_SRC_A::WAKEUP_DUE_TO_DIO2
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DUE_TO_DIO1`"]
    #[inline(always)]
    pub fn is_wakeup_due_to_dio1(&self) -> bool {
        *self == WAKEUP_SRC_A::WAKEUP_DUE_TO_DIO1
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DUE_TO_DIO0`"]
    #[inline(always)]
    pub fn is_wakeup_due_to_dio0(&self) -> bool {
        *self == WAKEUP_SRC_A::WAKEUP_DUE_TO_DIO0
    }
}
#[doc = "Reader of field `RTC_VALUE`"]
pub type RTC_VALUE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:18 - Status register indicates the last wake-up source"]
    #[inline(always)]
    pub fn wakeup_src(&self) -> WAKEUP_SRC_R {
        WAKEUP_SRC_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 0:7 - RTC counter value captured at wakeup event (only 8 LSBs, corresponds to 7.8 ms)"]
    #[inline(always)]
    pub fn rtc_value(&self) -> RTC_VALUE_R {
        RTC_VALUE_R::new((self.bits & 0xff) as u8)
    }
}
