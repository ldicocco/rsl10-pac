#[doc = "Reader of register SYSCTRL_SPEED_MEASURE"]
pub type R = crate::R<u32, super::SYSCTRL_SPEED_MEASURE>;
#[doc = "Writer for register SYSCTRL_SPEED_MEASURE"]
pub type W = crate::W<u32, super::SYSCTRL_SPEED_MEASURE>;
#[doc = "Register SYSCTRL_SPEED_MEASURE `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCTRL_SPEED_MEASURE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Start critical path speed measurement\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEED_MEASURE_START_AW {
    #[doc = "1: Start critical path speed measurement"]
    SPEED_MEASURE_START = 1,
}
impl From<SPEED_MEASURE_START_AW> for bool {
    #[inline(always)]
    fn from(variant: SPEED_MEASURE_START_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SPEED_MEASURE_START`"]
pub struct SPEED_MEASURE_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEED_MEASURE_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPEED_MEASURE_START_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Start critical path speed measurement"]
    #[inline(always)]
    pub fn speed_measure_start(self) -> &'a mut W {
        self.variant(SPEED_MEASURE_START_AW::SPEED_MEASURE_START)
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
#[doc = "Critical path speed measurement status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEED_MEASURE_STATUS_A {
    #[doc = "0: Critical path speed measurement idle"]
    SPEED_MEASURE_IDLE = 0,
    #[doc = "1: Critical path speed measurement busy"]
    SPEED_MEASURE_BUSY = 1,
}
impl From<SPEED_MEASURE_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: SPEED_MEASURE_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPEED_MEASURE_STATUS`"]
pub type SPEED_MEASURE_STATUS_R = crate::R<bool, SPEED_MEASURE_STATUS_A>;
impl SPEED_MEASURE_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPEED_MEASURE_STATUS_A {
        match self.bits {
            false => SPEED_MEASURE_STATUS_A::SPEED_MEASURE_IDLE,
            true => SPEED_MEASURE_STATUS_A::SPEED_MEASURE_BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `SPEED_MEASURE_IDLE`"]
    #[inline(always)]
    pub fn is_speed_measure_idle(&self) -> bool {
        *self == SPEED_MEASURE_STATUS_A::SPEED_MEASURE_IDLE
    }
    #[doc = "Checks if the value of the field is `SPEED_MEASURE_BUSY`"]
    #[inline(always)]
    pub fn is_speed_measure_busy(&self) -> bool {
        *self == SPEED_MEASURE_STATUS_A::SPEED_MEASURE_BUSY
    }
}
#[doc = "Reader of field `SPEED_MEASURE_RESULT`"]
pub type SPEED_MEASURE_RESULT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 3 - Critical path speed measurement status"]
    #[inline(always)]
    pub fn speed_measure_status(&self) -> SPEED_MEASURE_STATUS_R {
        SPEED_MEASURE_STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Critical path speed measurement result"]
    #[inline(always)]
    pub fn speed_measure_result(&self) -> SPEED_MEASURE_RESULT_R {
        SPEED_MEASURE_RESULT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Start critical path speed measurement"]
    #[inline(always)]
    pub fn speed_measure_start(&mut self) -> SPEED_MEASURE_START_W {
        SPEED_MEASURE_START_W { w: self }
    }
}
