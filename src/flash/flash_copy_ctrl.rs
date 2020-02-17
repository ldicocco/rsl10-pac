#[doc = "Reader of register FLASH_COPY_CTRL"]
pub type R = crate::R<u32, super::FLASH_COPY_CTRL>;
#[doc = "Writer for register FLASH_COPY_CTRL"]
pub type W = crate::W<u32, super::FLASH_COPY_CTRL>;
#[doc = "Register FLASH_COPY_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_COPY_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Error status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "0: No write / comparison error"]
    COPY_NO_ERROR = 0,
    #[doc = "1: Write or comparison error"]
    COPY_ERROR = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERROR`"]
pub type ERROR_R = crate::R<bool, ERROR_A>;
impl ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::COPY_NO_ERROR,
            true => ERROR_A::COPY_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `COPY_NO_ERROR`"]
    #[inline(always)]
    pub fn is_copy_no_error(&self) -> bool {
        *self == ERROR_A::COPY_NO_ERROR
    }
    #[doc = "Checks if the value of the field is `COPY_ERROR`"]
    #[inline(always)]
    pub fn is_copy_error(&self) -> bool {
        *self == ERROR_A::COPY_ERROR
    }
}
#[doc = "Stop the transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_AW {
    #[doc = "1: Stop the current transfer"]
    COPY_STOP = 1,
}
impl From<STOP_AW> for bool {
    #[inline(always)]
    fn from(variant: STOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `STOP`"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop the current transfer"]
    #[inline(always)]
    pub fn copy_stop(self) -> &'a mut W {
        self.variant(STOP_AW::COPY_STOP)
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
#[doc = "Start the transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_AW {
    #[doc = "1: Start the current transfer"]
    COPY_START = 1,
}
impl From<START_AW> for bool {
    #[inline(always)]
    fn from(variant: START_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Start the current transfer"]
    #[inline(always)]
    pub fn copy_start(self) -> &'a mut W {
        self.variant(START_AW::COPY_START)
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
#[doc = "Busy status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: Flash copier is idle"]
    COPY_IDLE = 0,
    #[doc = "1: Flash copier is busy"]
    COPY_BUSY = 1,
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
            false => BUSY_A::COPY_IDLE,
            true => BUSY_A::COPY_BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `COPY_IDLE`"]
    #[inline(always)]
    pub fn is_copy_idle(&self) -> bool {
        *self == BUSY_A::COPY_IDLE
    }
    #[doc = "Checks if the value of the field is `COPY_BUSY`"]
    #[inline(always)]
    pub fn is_copy_busy(&self) -> bool {
        *self == BUSY_A::COPY_BUSY
    }
}
impl R {
    #[doc = "Bit 3 - Error status"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Busy status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Stop the transfer"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 1 - Start the transfer"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
}
