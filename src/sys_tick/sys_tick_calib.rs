#[doc = "Reader of register SysTick_CALIB"]
pub type R = crate::R<u32, super::SYSTICK_CALIB>;
#[doc = "Indicates if a reference clock is available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOREF_A {
    #[doc = "1: No external reference clock available"]
    SYSTICK_NOREF = 1,
    #[doc = "0: External reference clock available"]
    SYSTICK_REF = 0,
}
impl From<NOREF_A> for bool {
    #[inline(always)]
    fn from(variant: NOREF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NOREF`"]
pub type NOREF_R = crate::R<bool, NOREF_A>;
impl NOREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOREF_A {
        match self.bits {
            true => NOREF_A::SYSTICK_NOREF,
            false => NOREF_A::SYSTICK_REF,
        }
    }
    #[doc = "Checks if the value of the field is `SYSTICK_NOREF`"]
    #[inline(always)]
    pub fn is_systick_noref(&self) -> bool {
        *self == NOREF_A::SYSTICK_NOREF
    }
    #[doc = "Checks if the value of the field is `SYSTICK_REF`"]
    #[inline(always)]
    pub fn is_systick_ref(&self) -> bool {
        *self == NOREF_A::SYSTICK_REF
    }
}
#[doc = "Indicates if calibration value is exactly 10 ms or not\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKEW_A {
    #[doc = "1: Calibration value is not exactly 10 ms"]
    SYSTICK_SKEW = 1,
    #[doc = "0: Calibration value is exactly 10 ms"]
    SYSTICK_NOSKEW = 0,
}
impl From<SKEW_A> for bool {
    #[inline(always)]
    fn from(variant: SKEW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SKEW`"]
pub type SKEW_R = crate::R<bool, SKEW_A>;
impl SKEW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SKEW_A {
        match self.bits {
            true => SKEW_A::SYSTICK_SKEW,
            false => SKEW_A::SYSTICK_NOSKEW,
        }
    }
    #[doc = "Checks if the value of the field is `SYSTICK_SKEW`"]
    #[inline(always)]
    pub fn is_systick_skew(&self) -> bool {
        *self == SKEW_A::SYSTICK_SKEW
    }
    #[doc = "Checks if the value of the field is `SYSTICK_NOSKEW`"]
    #[inline(always)]
    pub fn is_systick_noskew(&self) -> bool {
        *self == SKEW_A::SYSTICK_NOSKEW
    }
}
#[doc = "Reader of field `TENMS`"]
pub type TENMS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bit 31 - Indicates if a reference clock is available"]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Indicates if calibration value is exactly 10 ms or not"]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 0:23 - SYSTICK counter calibration value for 10 ms. A value of 0 means the calibration value is not available"]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
