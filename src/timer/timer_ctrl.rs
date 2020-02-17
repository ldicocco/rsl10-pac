#[doc = "Reader of register TIMER_CTRL[%s]"]
pub type R = crate::R<u32, super::TIMER_CTRL>;
#[doc = "Writer for register TIMER_CTRL[%s]"]
pub type W = crate::W<u32, super::TIMER_CTRL>;
#[doc = "Register TIMER_CTRL[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::TIMER_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Indicate if the timer is active or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_STATUS_A {
    #[doc = "0: The timer is inactive"]
    TIMER_INACTIVE = 0,
    #[doc = "1: The timer is active"]
    TIMER_ACTIVE = 1,
}
impl From<TIMER_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMER_STATUS`"]
pub type TIMER_STATUS_R = crate::R<bool, TIMER_STATUS_A>;
impl TIMER_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_STATUS_A {
        match self.bits {
            false => TIMER_STATUS_A::TIMER_INACTIVE,
            true => TIMER_STATUS_A::TIMER_ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_INACTIVE`"]
    #[inline(always)]
    pub fn is_timer_inactive(&self) -> bool {
        *self == TIMER_STATUS_A::TIMER_INACTIVE
    }
    #[doc = "Checks if the value of the field is `TIMER_ACTIVE`"]
    #[inline(always)]
    pub fn is_timer_active(&self) -> bool {
        *self == TIMER_STATUS_A::TIMER_ACTIVE
    }
}
#[doc = "Start or restart the timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_START_AW {
    #[doc = "1: Writing a 1 will start or restart the timer"]
    TIMER_START = 1,
}
impl From<TIMER_START_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMER_START_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TIMER_START`"]
pub struct TIMER_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_START_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing a 1 will start or restart the timer"]
    #[inline(always)]
    pub fn timer_start(self) -> &'a mut W {
        self.variant(TIMER_START_AW::TIMER_START)
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
#[doc = "Stop the timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_STOP_AW {
    #[doc = "1: Writing a 1 will stop the timer"]
    TIMER_STOP = 1,
}
impl From<TIMER_STOP_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMER_STOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TIMER_STOP`"]
pub struct TIMER_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_STOP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing a 1 will stop the timer"]
    #[inline(always)]
    pub fn timer_stop(self) -> &'a mut W {
        self.variant(TIMER_STOP_AW::TIMER_STOP)
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
    #[doc = "Bit 2 - Indicate if the timer is active or not"]
    #[inline(always)]
    pub fn timer_status(&self) -> TIMER_STATUS_R {
        TIMER_STATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Start or restart the timer"]
    #[inline(always)]
    pub fn timer_start(&mut self) -> TIMER_START_W {
        TIMER_START_W { w: self }
    }
    #[doc = "Bit 0 - Stop the timer"]
    #[inline(always)]
    pub fn timer_stop(&mut self) -> TIMER_STOP_W {
        TIMER_STOP_W { w: self }
    }
}
