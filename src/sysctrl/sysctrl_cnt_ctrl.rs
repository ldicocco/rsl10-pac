#[doc = "Reader of register SYSCTRL_CNT_CTRL"]
pub type R = crate::R<u32, super::SYSCTRL_CNT_CTRL>;
#[doc = "Writer for register SYSCTRL_CNT_CTRL"]
pub type W = crate::W<u32, super::SYSCTRL_CNT_CTRL>;
#[doc = "Register SYSCTRL_CNT_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCTRL_CNT_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Activity counters status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNT_STATUS_A {
    #[doc = "0: Activity counters stopped"]
    CNT_STOPPED = 0,
    #[doc = "1: Activity counters running"]
    CNT_RUNNING = 1,
}
impl From<CNT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: CNT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CNT_STATUS`"]
pub type CNT_STATUS_R = crate::R<bool, CNT_STATUS_A>;
impl CNT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNT_STATUS_A {
        match self.bits {
            false => CNT_STATUS_A::CNT_STOPPED,
            true => CNT_STATUS_A::CNT_RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `CNT_STOPPED`"]
    #[inline(always)]
    pub fn is_cnt_stopped(&self) -> bool {
        *self == CNT_STATUS_A::CNT_STOPPED
    }
    #[doc = "Checks if the value of the field is `CNT_RUNNING`"]
    #[inline(always)]
    pub fn is_cnt_running(&self) -> bool {
        *self == CNT_STATUS_A::CNT_RUNNING
    }
}
#[doc = "Clear activity counters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNT_CLEAR_AW {
    #[doc = "1: Clear activity counters"]
    CNT_CLEAR = 1,
}
impl From<CNT_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: CNT_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CNT_CLEAR`"]
pub struct CNT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNT_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear activity counters"]
    #[inline(always)]
    pub fn cnt_clear(self) -> &'a mut W {
        self.variant(CNT_CLEAR_AW::CNT_CLEAR)
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
#[doc = "Stop activity counters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNT_STOP_AW {
    #[doc = "1: Stop activity counters"]
    CNT_STOP = 1,
}
impl From<CNT_STOP_AW> for bool {
    #[inline(always)]
    fn from(variant: CNT_STOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CNT_STOP`"]
pub struct CNT_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNT_STOP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop activity counters"]
    #[inline(always)]
    pub fn cnt_stop(self) -> &'a mut W {
        self.variant(CNT_STOP_AW::CNT_STOP)
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
#[doc = "Start activity counters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNT_START_AW {
    #[doc = "1: Start activity counters"]
    CNT_START = 1,
}
impl From<CNT_START_AW> for bool {
    #[inline(always)]
    fn from(variant: CNT_START_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CNT_START`"]
pub struct CNT_START_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNT_START_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Start activity counters"]
    #[inline(always)]
    pub fn cnt_start(self) -> &'a mut W {
        self.variant(CNT_START_AW::CNT_START)
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
    #[doc = "Bit 3 - Activity counters status bit"]
    #[inline(always)]
    pub fn cnt_status(&self) -> CNT_STATUS_R {
        CNT_STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Clear activity counters"]
    #[inline(always)]
    pub fn cnt_clear(&mut self) -> CNT_CLEAR_W {
        CNT_CLEAR_W { w: self }
    }
    #[doc = "Bit 1 - Stop activity counters"]
    #[inline(always)]
    pub fn cnt_stop(&mut self) -> CNT_STOP_W {
        CNT_STOP_W { w: self }
    }
    #[doc = "Bit 0 - Start activity counters"]
    #[inline(always)]
    pub fn cnt_start(&mut self) -> CNT_START_W {
        CNT_START_W { w: self }
    }
}
