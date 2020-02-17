#[doc = "Reader of register ACS_RTC_CFG"]
pub type R = crate::R<u32, super::ACS_RTC_CFG>;
#[doc = "Writer for register ACS_RTC_CFG"]
pub type W = crate::W<u32, super::ACS_RTC_CFG>;
#[doc = "Register ACS_RTC_CFG `reset()`'s with value 0x7fff"]
impl crate::ResetValue for super::ACS_RTC_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7fff
    }
}
#[doc = "Start value for the RTC timer counter (counts from start_value down to 0)\n\nValue on reset: 32767"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum START_VALUE_A {
    #[doc = "0: Divide by 1"]
    RTC_CNT_START_0 = 0,
    #[doc = "1: Divide by 2"]
    RTC_CNT_START_1 = 1,
    #[doc = "32767: Divide by 32768"]
    RTC_CNT_START_32767 = 32767,
    #[doc = "4294967295: Divide by 2**32"]
    RTC_CNT_START_4294967295 = 4294967295,
}
impl From<START_VALUE_A> for u32 {
    #[inline(always)]
    fn from(variant: START_VALUE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `START_VALUE`"]
pub type START_VALUE_R = crate::R<u32, START_VALUE_A>;
impl START_VALUE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, START_VALUE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(START_VALUE_A::RTC_CNT_START_0),
            1 => Val(START_VALUE_A::RTC_CNT_START_1),
            32767 => Val(START_VALUE_A::RTC_CNT_START_32767),
            4294967295 => Val(START_VALUE_A::RTC_CNT_START_4294967295),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RTC_CNT_START_0`"]
    #[inline(always)]
    pub fn is_rtc_cnt_start_0(&self) -> bool {
        *self == START_VALUE_A::RTC_CNT_START_0
    }
    #[doc = "Checks if the value of the field is `RTC_CNT_START_1`"]
    #[inline(always)]
    pub fn is_rtc_cnt_start_1(&self) -> bool {
        *self == START_VALUE_A::RTC_CNT_START_1
    }
    #[doc = "Checks if the value of the field is `RTC_CNT_START_32767`"]
    #[inline(always)]
    pub fn is_rtc_cnt_start_32767(&self) -> bool {
        *self == START_VALUE_A::RTC_CNT_START_32767
    }
    #[doc = "Checks if the value of the field is `RTC_CNT_START_4294967295`"]
    #[inline(always)]
    pub fn is_rtc_cnt_start_4294967295(&self) -> bool {
        *self == START_VALUE_A::RTC_CNT_START_4294967295
    }
}
#[doc = "Write proxy for field `START_VALUE`"]
pub struct START_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> START_VALUE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_VALUE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn rtc_cnt_start_0(self) -> &'a mut W {
        self.variant(START_VALUE_A::RTC_CNT_START_0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn rtc_cnt_start_1(self) -> &'a mut W {
        self.variant(START_VALUE_A::RTC_CNT_START_1)
    }
    #[doc = "Divide by 32768"]
    #[inline(always)]
    pub fn rtc_cnt_start_32767(self) -> &'a mut W {
        self.variant(START_VALUE_A::RTC_CNT_START_32767)
    }
    #[doc = "Divide by 2**32"]
    #[inline(always)]
    pub fn rtc_cnt_start_4294967295(self) -> &'a mut W {
        self.variant(START_VALUE_A::RTC_CNT_START_4294967295)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Start value for the RTC timer counter (counts from start_value down to 0)"]
    #[inline(always)]
    pub fn start_value(&self) -> START_VALUE_R {
        START_VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start value for the RTC timer counter (counts from start_value down to 0)"]
    #[inline(always)]
    pub fn start_value(&mut self) -> START_VALUE_W {
        START_VALUE_W { w: self }
    }
}
