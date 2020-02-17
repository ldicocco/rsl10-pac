#[doc = "Reader of register SYSCTRL_WAKEUP_PAD"]
pub type R = crate::R<u32, super::SYSCTRL_WAKEUP_PAD>;
#[doc = "WAKEUP pad value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_PAD_VALUE_A {
    #[doc = "0: WAKEUP pad value equal to '0'"]
    WAKEUP_PAD_LOW = 0,
    #[doc = "1: WAKEUP pad value equal to '1'"]
    WAKEUP_PAD_HIGH = 1,
}
impl From<WAKEUP_PAD_VALUE_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_PAD_VALUE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAKEUP_PAD_VALUE`"]
pub type WAKEUP_PAD_VALUE_R = crate::R<bool, WAKEUP_PAD_VALUE_A>;
impl WAKEUP_PAD_VALUE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP_PAD_VALUE_A {
        match self.bits {
            false => WAKEUP_PAD_VALUE_A::WAKEUP_PAD_LOW,
            true => WAKEUP_PAD_VALUE_A::WAKEUP_PAD_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_PAD_LOW`"]
    #[inline(always)]
    pub fn is_wakeup_pad_low(&self) -> bool {
        *self == WAKEUP_PAD_VALUE_A::WAKEUP_PAD_LOW
    }
    #[doc = "Checks if the value of the field is `WAKEUP_PAD_HIGH`"]
    #[inline(always)]
    pub fn is_wakeup_pad_high(&self) -> bool {
        *self == WAKEUP_PAD_VALUE_A::WAKEUP_PAD_HIGH
    }
}
impl R {
    #[doc = "Bit 0 - WAKEUP pad value"]
    #[inline(always)]
    pub fn wakeup_pad_value(&self) -> WAKEUP_PAD_VALUE_R {
        WAKEUP_PAD_VALUE_R::new((self.bits & 0x01) != 0)
    }
}
