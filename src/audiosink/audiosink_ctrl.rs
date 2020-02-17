#[doc = "Reader of register AUDIOSINK_CTRL"]
pub type R = crate::R<u32, super::AUDIOSINK_CTRL>;
#[doc = "Writer for register AUDIOSINK_CTRL"]
pub type W = crate::W<u32, super::AUDIOSINK_CTRL>;
#[doc = "Register AUDIOSINK_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::AUDIOSINK_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Start the audio sink clock phase counter mechanism without waiting on a sync pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHASE_CNT_START_NO_WAIT_AW {
    #[doc = "1: Reset PHASE_CNT and start the audio sink clock phase counter mechanism without waiting on a sync pulse"]
    PHASE_CNT_START_NO_WAIT = 1,
}
impl From<PHASE_CNT_START_NO_WAIT_AW> for bool {
    #[inline(always)]
    fn from(variant: PHASE_CNT_START_NO_WAIT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PHASE_CNT_START_NO_WAIT`"]
pub struct PHASE_CNT_START_NO_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE_CNT_START_NO_WAIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHASE_CNT_START_NO_WAIT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset PHASE_CNT and start the audio sink clock phase counter mechanism without waiting on a sync pulse"]
    #[inline(always)]
    pub fn phase_cnt_start_no_wait(self) -> &'a mut W {
        self.variant(PHASE_CNT_START_NO_WAIT_AW::PHASE_CNT_START_NO_WAIT)
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
#[doc = "Audio sink clock period counter status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIOD_CNT_STATUS_A {
    #[doc = "0: The audio sink clock period counter mechanism is idle"]
    PERIOD_CNT_IDLE = 0,
    #[doc = "1: The audio sink clock period counter mechanism is busy"]
    PERIOD_CNT_BUSY = 1,
}
impl From<PERIOD_CNT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: PERIOD_CNT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PERIOD_CNT_STATUS`"]
pub type PERIOD_CNT_STATUS_R = crate::R<bool, PERIOD_CNT_STATUS_A>;
impl PERIOD_CNT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIOD_CNT_STATUS_A {
        match self.bits {
            false => PERIOD_CNT_STATUS_A::PERIOD_CNT_IDLE,
            true => PERIOD_CNT_STATUS_A::PERIOD_CNT_BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `PERIOD_CNT_IDLE`"]
    #[inline(always)]
    pub fn is_period_cnt_idle(&self) -> bool {
        *self == PERIOD_CNT_STATUS_A::PERIOD_CNT_IDLE
    }
    #[doc = "Checks if the value of the field is `PERIOD_CNT_BUSY`"]
    #[inline(always)]
    pub fn is_period_cnt_busy(&self) -> bool {
        *self == PERIOD_CNT_STATUS_A::PERIOD_CNT_BUSY
    }
}
#[doc = "Stop the audio sink clock period counter mechanism\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIOD_CNT_STOP_AW {
    #[doc = "1: Stop the audio sink clock period counter mechanism"]
    PERIOD_CNT_STOP = 1,
}
impl From<PERIOD_CNT_STOP_AW> for bool {
    #[inline(always)]
    fn from(variant: PERIOD_CNT_STOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PERIOD_CNT_STOP`"]
pub struct PERIOD_CNT_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIOD_CNT_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERIOD_CNT_STOP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop the audio sink clock period counter mechanism"]
    #[inline(always)]
    pub fn period_cnt_stop(self) -> &'a mut W {
        self.variant(PERIOD_CNT_STOP_AW::PERIOD_CNT_STOP)
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
#[doc = "Start the audio sink clock period counter mechanism\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIOD_CNT_START_AW {
    #[doc = "1: Reset PERIOD_CNT and start the audio sink clock period counter mechanism"]
    PERIOD_CNT_START = 1,
}
impl From<PERIOD_CNT_START_AW> for bool {
    #[inline(always)]
    fn from(variant: PERIOD_CNT_START_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PERIOD_CNT_START`"]
pub struct PERIOD_CNT_START_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIOD_CNT_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERIOD_CNT_START_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset PERIOD_CNT and start the audio sink clock period counter mechanism"]
    #[inline(always)]
    pub fn period_cnt_start(self) -> &'a mut W {
        self.variant(PERIOD_CNT_START_AW::PERIOD_CNT_START)
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
#[doc = "Audio sink clock phase counter missed status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHASE_CNT_MISSED_STATUS_A {
    #[doc = "0: The audio sink clock phase counter mechanism was not stopped due to a missed synchronization signal"]
    PHASE_CNT_NORMAL = 0,
    #[doc = "1: The audio sink clock phase counter mechanism was stopped due to a missed synchronization signal"]
    PHASE_CNT_MISSED = 1,
}
impl From<PHASE_CNT_MISSED_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: PHASE_CNT_MISSED_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PHASE_CNT_MISSED_STATUS`"]
pub type PHASE_CNT_MISSED_STATUS_R = crate::R<bool, PHASE_CNT_MISSED_STATUS_A>;
impl PHASE_CNT_MISSED_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHASE_CNT_MISSED_STATUS_A {
        match self.bits {
            false => PHASE_CNT_MISSED_STATUS_A::PHASE_CNT_NORMAL,
            true => PHASE_CNT_MISSED_STATUS_A::PHASE_CNT_MISSED,
        }
    }
    #[doc = "Checks if the value of the field is `PHASE_CNT_NORMAL`"]
    #[inline(always)]
    pub fn is_phase_cnt_normal(&self) -> bool {
        *self == PHASE_CNT_MISSED_STATUS_A::PHASE_CNT_NORMAL
    }
    #[doc = "Checks if the value of the field is `PHASE_CNT_MISSED`"]
    #[inline(always)]
    pub fn is_phase_cnt_missed(&self) -> bool {
        *self == PHASE_CNT_MISSED_STATUS_A::PHASE_CNT_MISSED
    }
}
#[doc = "Audio sink clock phase counter status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHASE_CNT_STATUS_A {
    #[doc = "0: The audio sink clock phase counter mechanism is idle"]
    PHASE_CNT_IDLE = 0,
    #[doc = "1: The audio sink clock phase counter mechanism is busy"]
    PHASE_CNT_BUSY = 1,
}
impl From<PHASE_CNT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: PHASE_CNT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PHASE_CNT_STATUS`"]
pub type PHASE_CNT_STATUS_R = crate::R<bool, PHASE_CNT_STATUS_A>;
impl PHASE_CNT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHASE_CNT_STATUS_A {
        match self.bits {
            false => PHASE_CNT_STATUS_A::PHASE_CNT_IDLE,
            true => PHASE_CNT_STATUS_A::PHASE_CNT_BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `PHASE_CNT_IDLE`"]
    #[inline(always)]
    pub fn is_phase_cnt_idle(&self) -> bool {
        *self == PHASE_CNT_STATUS_A::PHASE_CNT_IDLE
    }
    #[doc = "Checks if the value of the field is `PHASE_CNT_BUSY`"]
    #[inline(always)]
    pub fn is_phase_cnt_busy(&self) -> bool {
        *self == PHASE_CNT_STATUS_A::PHASE_CNT_BUSY
    }
}
#[doc = "Stop the audio sink clock phase counter mechanism\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHASE_CNT_STOP_AW {
    #[doc = "1: Stop the audio sink clock phase counter mechanism"]
    PHASE_CNT_STOP = 1,
}
impl From<PHASE_CNT_STOP_AW> for bool {
    #[inline(always)]
    fn from(variant: PHASE_CNT_STOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PHASE_CNT_STOP`"]
pub struct PHASE_CNT_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE_CNT_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHASE_CNT_STOP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop the audio sink clock phase counter mechanism"]
    #[inline(always)]
    pub fn phase_cnt_stop(self) -> &'a mut W {
        self.variant(PHASE_CNT_STOP_AW::PHASE_CNT_STOP)
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
#[doc = "Start the audio sink clock PHASE counter mechanism and wait for sync pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHASE_CNT_START_AW {
    #[doc = "1: Reset PHASE_CNT and start the audio sink clock phase counter mechanism and wait on a sync pulse"]
    PHASE_CNT_START = 1,
}
impl From<PHASE_CNT_START_AW> for bool {
    #[inline(always)]
    fn from(variant: PHASE_CNT_START_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PHASE_CNT_START`"]
pub struct PHASE_CNT_START_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE_CNT_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHASE_CNT_START_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset PHASE_CNT and start the audio sink clock phase counter mechanism and wait on a sync pulse"]
    #[inline(always)]
    pub fn phase_cnt_start(self) -> &'a mut W {
        self.variant(PHASE_CNT_START_AW::PHASE_CNT_START)
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
#[doc = "Reset audio sink clock counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNT_RESET_AW {
    #[doc = "1: Reset audio sink clock counter"]
    CNT_RESET = 1,
}
impl From<CNT_RESET_AW> for bool {
    #[inline(always)]
    fn from(variant: CNT_RESET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CNT_RESET`"]
pub struct CNT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNT_RESET_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset audio sink clock counter"]
    #[inline(always)]
    pub fn cnt_reset(self) -> &'a mut W {
        self.variant(CNT_RESET_AW::CNT_RESET)
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
    #[doc = "Bit 7 - Audio sink clock period counter status"]
    #[inline(always)]
    pub fn period_cnt_status(&self) -> PERIOD_CNT_STATUS_R {
        PERIOD_CNT_STATUS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Audio sink clock phase counter missed status"]
    #[inline(always)]
    pub fn phase_cnt_missed_status(&self) -> PHASE_CNT_MISSED_STATUS_R {
        PHASE_CNT_MISSED_STATUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Audio sink clock phase counter status"]
    #[inline(always)]
    pub fn phase_cnt_status(&self) -> PHASE_CNT_STATUS_R {
        PHASE_CNT_STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Start the audio sink clock phase counter mechanism without waiting on a sync pulse"]
    #[inline(always)]
    pub fn phase_cnt_start_no_wait(&mut self) -> PHASE_CNT_START_NO_WAIT_W {
        PHASE_CNT_START_NO_WAIT_W { w: self }
    }
    #[doc = "Bit 6 - Stop the audio sink clock period counter mechanism"]
    #[inline(always)]
    pub fn period_cnt_stop(&mut self) -> PERIOD_CNT_STOP_W {
        PERIOD_CNT_STOP_W { w: self }
    }
    #[doc = "Bit 5 - Start the audio sink clock period counter mechanism"]
    #[inline(always)]
    pub fn period_cnt_start(&mut self) -> PERIOD_CNT_START_W {
        PERIOD_CNT_START_W { w: self }
    }
    #[doc = "Bit 2 - Stop the audio sink clock phase counter mechanism"]
    #[inline(always)]
    pub fn phase_cnt_stop(&mut self) -> PHASE_CNT_STOP_W {
        PHASE_CNT_STOP_W { w: self }
    }
    #[doc = "Bit 1 - Start the audio sink clock PHASE counter mechanism and wait for sync pulse"]
    #[inline(always)]
    pub fn phase_cnt_start(&mut self) -> PHASE_CNT_START_W {
        PHASE_CNT_START_W { w: self }
    }
    #[doc = "Bit 0 - Reset audio sink clock counter"]
    #[inline(always)]
    pub fn cnt_reset(&mut self) -> CNT_RESET_W {
        CNT_RESET_W { w: self }
    }
}
