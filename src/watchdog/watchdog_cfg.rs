#[doc = "Reader of register WATCHDOG_CFG"]
pub type R = crate::R<u32, super::WATCHDOG_CFG>;
#[doc = "Writer for register WATCHDOG_CFG"]
pub type W = crate::W<u32, super::WATCHDOG_CFG>;
#[doc = "Register WATCHDOG_CFG `reset()`'s with value 0x0b"]
impl crate::ResetValue for super::WATCHDOG_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0b
    }
}
#[doc = "Watchdog timeout period. Values 0xC to 0xF result in the same timeout period as the value 0xB.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMEOUT_VALUE_A {
    #[doc = "0: 2.048 ms"]
    WATCHDOG_TIMEOUT_2M048 = 0,
    #[doc = "1: 4.096 ms"]
    WATCHDOG_TIMEOUT_4M096 = 1,
    #[doc = "2: 8.192 ms"]
    WATCHDOG_TIMEOUT_8M2 = 2,
    #[doc = "3: 16.384 ms"]
    WATCHDOG_TIMEOUT_16M4 = 3,
    #[doc = "4: 32.768 ms"]
    WATCHDOG_TIMEOUT_32M8 = 4,
    #[doc = "5: 65.536 ms"]
    WATCHDOG_TIMEOUT_65M5 = 5,
    #[doc = "6: 131.072 ms"]
    WATCHDOG_TIMEOUT_131M1 = 6,
    #[doc = "7: 262.144 ms"]
    WATCHDOG_TIMEOUT_262M1 = 7,
    #[doc = "8: 524.3 ms"]
    WATCHDOG_TIMEOUT_524M3 = 8,
    #[doc = "9: 1.048 sec"]
    WATCHDOG_TIMEOUT_1048M6 = 9,
    #[doc = "10: 2.097 sec"]
    WATCHDOG_TIMEOUT_2097M1 = 10,
    #[doc = "11: 4.194 sec"]
    WATCHDOG_TIMEOUT_4194M3 = 11,
}
impl From<TIMEOUT_VALUE_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT_VALUE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMEOUT_VALUE`"]
pub type TIMEOUT_VALUE_R = crate::R<u8, TIMEOUT_VALUE_A>;
impl TIMEOUT_VALUE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMEOUT_VALUE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_2M048),
            1 => Val(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_4M096),
            2 => Val(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_8M2),
            3 => Val(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_16M4),
            4 => Val(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_32M8),
            5 => Val(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_65M5),
            6 => Val(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_131M1),
            7 => Val(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_262M1),
            8 => Val(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_524M3),
            9 => Val(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_1048M6),
            10 => Val(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_2097M1),
            11 => Val(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_4194M3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_TIMEOUT_2M048`"]
    #[inline(always)]
    pub fn is_watchdog_timeout_2m048(&self) -> bool {
        *self == TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_2M048
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_TIMEOUT_4M096`"]
    #[inline(always)]
    pub fn is_watchdog_timeout_4m096(&self) -> bool {
        *self == TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_4M096
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_TIMEOUT_8M2`"]
    #[inline(always)]
    pub fn is_watchdog_timeout_8m2(&self) -> bool {
        *self == TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_8M2
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_TIMEOUT_16M4`"]
    #[inline(always)]
    pub fn is_watchdog_timeout_16m4(&self) -> bool {
        *self == TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_16M4
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_TIMEOUT_32M8`"]
    #[inline(always)]
    pub fn is_watchdog_timeout_32m8(&self) -> bool {
        *self == TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_32M8
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_TIMEOUT_65M5`"]
    #[inline(always)]
    pub fn is_watchdog_timeout_65m5(&self) -> bool {
        *self == TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_65M5
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_TIMEOUT_131M1`"]
    #[inline(always)]
    pub fn is_watchdog_timeout_131m1(&self) -> bool {
        *self == TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_131M1
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_TIMEOUT_262M1`"]
    #[inline(always)]
    pub fn is_watchdog_timeout_262m1(&self) -> bool {
        *self == TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_262M1
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_TIMEOUT_524M3`"]
    #[inline(always)]
    pub fn is_watchdog_timeout_524m3(&self) -> bool {
        *self == TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_524M3
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_TIMEOUT_1048M6`"]
    #[inline(always)]
    pub fn is_watchdog_timeout_1048m6(&self) -> bool {
        *self == TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_1048M6
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_TIMEOUT_2097M1`"]
    #[inline(always)]
    pub fn is_watchdog_timeout_2097m1(&self) -> bool {
        *self == TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_2097M1
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_TIMEOUT_4194M3`"]
    #[inline(always)]
    pub fn is_watchdog_timeout_4194m3(&self) -> bool {
        *self == TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_4194M3
    }
}
#[doc = "Write proxy for field `TIMEOUT_VALUE`"]
pub struct TIMEOUT_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_VALUE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUT_VALUE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "2.048 ms"]
    #[inline(always)]
    pub fn watchdog_timeout_2m048(self) -> &'a mut W {
        self.variant(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_2M048)
    }
    #[doc = "4.096 ms"]
    #[inline(always)]
    pub fn watchdog_timeout_4m096(self) -> &'a mut W {
        self.variant(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_4M096)
    }
    #[doc = "8.192 ms"]
    #[inline(always)]
    pub fn watchdog_timeout_8m2(self) -> &'a mut W {
        self.variant(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_8M2)
    }
    #[doc = "16.384 ms"]
    #[inline(always)]
    pub fn watchdog_timeout_16m4(self) -> &'a mut W {
        self.variant(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_16M4)
    }
    #[doc = "32.768 ms"]
    #[inline(always)]
    pub fn watchdog_timeout_32m8(self) -> &'a mut W {
        self.variant(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_32M8)
    }
    #[doc = "65.536 ms"]
    #[inline(always)]
    pub fn watchdog_timeout_65m5(self) -> &'a mut W {
        self.variant(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_65M5)
    }
    #[doc = "131.072 ms"]
    #[inline(always)]
    pub fn watchdog_timeout_131m1(self) -> &'a mut W {
        self.variant(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_131M1)
    }
    #[doc = "262.144 ms"]
    #[inline(always)]
    pub fn watchdog_timeout_262m1(self) -> &'a mut W {
        self.variant(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_262M1)
    }
    #[doc = "524.3 ms"]
    #[inline(always)]
    pub fn watchdog_timeout_524m3(self) -> &'a mut W {
        self.variant(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_524M3)
    }
    #[doc = "1.048 sec"]
    #[inline(always)]
    pub fn watchdog_timeout_1048m6(self) -> &'a mut W {
        self.variant(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_1048M6)
    }
    #[doc = "2.097 sec"]
    #[inline(always)]
    pub fn watchdog_timeout_2097m1(self) -> &'a mut W {
        self.variant(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_2097M1)
    }
    #[doc = "4.194 sec"]
    #[inline(always)]
    pub fn watchdog_timeout_4194m3(self) -> &'a mut W {
        self.variant(TIMEOUT_VALUE_A::WATCHDOG_TIMEOUT_4194M3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Watchdog timeout period. Values 0xC to 0xF result in the same timeout period as the value 0xB."]
    #[inline(always)]
    pub fn timeout_value(&self) -> TIMEOUT_VALUE_R {
        TIMEOUT_VALUE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Watchdog timeout period. Values 0xC to 0xF result in the same timeout period as the value 0xB."]
    #[inline(always)]
    pub fn timeout_value(&mut self) -> TIMEOUT_VALUE_W {
        TIMEOUT_VALUE_W { w: self }
    }
}
