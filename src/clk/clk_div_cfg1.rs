#[doc = "Reader of register CLK_DIV_CFG1"]
pub type R = crate::R<u32, super::CLK_DIV_CFG1>;
#[doc = "Writer for register CLK_DIV_CFG1"]
pub type W = crate::W<u32, super::CLK_DIV_CFG1>;
#[doc = "Register CLK_DIV_CFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_DIV_CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Prescale value for the slow audio clock down from the fast audio clock (1 to 4 in steps of 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUDIOSLOWCLK_PRESCALE_A {
    #[doc = "0: Divide by 1"]
    AUDIOSLOWCLK_PRESCALE_1 = 0,
    #[doc = "1: Divide by 2"]
    AUDIOSLOWCLK_PRESCALE_2 = 1,
    #[doc = "2: Divide by 3"]
    AUDIOSLOWCLK_PRESCALE_3 = 2,
    #[doc = "3: Divide by 4"]
    AUDIOSLOWCLK_PRESCALE_4 = 3,
}
impl From<AUDIOSLOWCLK_PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: AUDIOSLOWCLK_PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AUDIOSLOWCLK_PRESCALE`"]
pub type AUDIOSLOWCLK_PRESCALE_R = crate::R<u8, AUDIOSLOWCLK_PRESCALE_A>;
impl AUDIOSLOWCLK_PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIOSLOWCLK_PRESCALE_A {
        match self.bits {
            0 => AUDIOSLOWCLK_PRESCALE_A::AUDIOSLOWCLK_PRESCALE_1,
            1 => AUDIOSLOWCLK_PRESCALE_A::AUDIOSLOWCLK_PRESCALE_2,
            2 => AUDIOSLOWCLK_PRESCALE_A::AUDIOSLOWCLK_PRESCALE_3,
            3 => AUDIOSLOWCLK_PRESCALE_A::AUDIOSLOWCLK_PRESCALE_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUDIOSLOWCLK_PRESCALE_1`"]
    #[inline(always)]
    pub fn is_audioslowclk_prescale_1(&self) -> bool {
        *self == AUDIOSLOWCLK_PRESCALE_A::AUDIOSLOWCLK_PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `AUDIOSLOWCLK_PRESCALE_2`"]
    #[inline(always)]
    pub fn is_audioslowclk_prescale_2(&self) -> bool {
        *self == AUDIOSLOWCLK_PRESCALE_A::AUDIOSLOWCLK_PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `AUDIOSLOWCLK_PRESCALE_3`"]
    #[inline(always)]
    pub fn is_audioslowclk_prescale_3(&self) -> bool {
        *self == AUDIOSLOWCLK_PRESCALE_A::AUDIOSLOWCLK_PRESCALE_3
    }
    #[doc = "Checks if the value of the field is `AUDIOSLOWCLK_PRESCALE_4`"]
    #[inline(always)]
    pub fn is_audioslowclk_prescale_4(&self) -> bool {
        *self == AUDIOSLOWCLK_PRESCALE_A::AUDIOSLOWCLK_PRESCALE_4
    }
}
#[doc = "Write proxy for field `AUDIOSLOWCLK_PRESCALE`"]
pub struct AUDIOSLOWCLK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIOSLOWCLK_PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIOSLOWCLK_PRESCALE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn audioslowclk_prescale_1(self) -> &'a mut W {
        self.variant(AUDIOSLOWCLK_PRESCALE_A::AUDIOSLOWCLK_PRESCALE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn audioslowclk_prescale_2(self) -> &'a mut W {
        self.variant(AUDIOSLOWCLK_PRESCALE_A::AUDIOSLOWCLK_PRESCALE_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn audioslowclk_prescale_3(self) -> &'a mut W {
        self.variant(AUDIOSLOWCLK_PRESCALE_A::AUDIOSLOWCLK_PRESCALE_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn audioslowclk_prescale_4(self) -> &'a mut W {
        self.variant(AUDIOSLOWCLK_PRESCALE_A::AUDIOSLOWCLK_PRESCALE_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Prescale value for the fast audio clock (1 to 64 in steps of 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUDIOCLK_PRESCALE_A {
    #[doc = "0: Divide by 1"]
    AUDIOCLK_PRESCALE_1 = 0,
    #[doc = "1: Divide by 2"]
    AUDIOCLK_PRESCALE_2 = 1,
    #[doc = "2: Divide by 3"]
    AUDIOCLK_PRESCALE_3 = 2,
    #[doc = "3: Divide by 4"]
    AUDIOCLK_PRESCALE_4 = 3,
    #[doc = "4: Divide by 5"]
    AUDIOCLK_PRESCALE_5 = 4,
    #[doc = "5: Divide by 6"]
    AUDIOCLK_PRESCALE_6 = 5,
    #[doc = "6: Divide by 7"]
    AUDIOCLK_PRESCALE_7 = 6,
    #[doc = "7: Divide by 8"]
    AUDIOCLK_PRESCALE_8 = 7,
    #[doc = "8: Divide by 9"]
    AUDIOCLK_PRESCALE_9 = 8,
    #[doc = "9: Divide by 10"]
    AUDIOCLK_PRESCALE_10 = 9,
    #[doc = "10: Divide by 11"]
    AUDIOCLK_PRESCALE_11 = 10,
    #[doc = "11: Divide by 12"]
    AUDIOCLK_PRESCALE_12 = 11,
    #[doc = "12: Divide by 13"]
    AUDIOCLK_PRESCALE_13 = 12,
    #[doc = "13: Divide by 14"]
    AUDIOCLK_PRESCALE_14 = 13,
    #[doc = "14: Divide by 15"]
    AUDIOCLK_PRESCALE_15 = 14,
    #[doc = "15: Divide by 16"]
    AUDIOCLK_PRESCALE_16 = 15,
    #[doc = "62: Divide by 63"]
    AUDIOCLK_PRESCALE_63 = 62,
    #[doc = "63: Divide by 64"]
    AUDIOCLK_PRESCALE_64 = 63,
}
impl From<AUDIOCLK_PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: AUDIOCLK_PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AUDIOCLK_PRESCALE`"]
pub type AUDIOCLK_PRESCALE_R = crate::R<u8, AUDIOCLK_PRESCALE_A>;
impl AUDIOCLK_PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AUDIOCLK_PRESCALE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_1),
            1 => Val(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_2),
            2 => Val(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_3),
            3 => Val(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_4),
            4 => Val(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_5),
            5 => Val(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_6),
            6 => Val(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_7),
            7 => Val(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_8),
            8 => Val(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_9),
            9 => Val(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_10),
            10 => Val(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_11),
            11 => Val(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_12),
            12 => Val(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_13),
            13 => Val(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_14),
            14 => Val(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_15),
            15 => Val(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_16),
            62 => Val(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_63),
            63 => Val(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUDIOCLK_PRESCALE_1`"]
    #[inline(always)]
    pub fn is_audioclk_prescale_1(&self) -> bool {
        *self == AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `AUDIOCLK_PRESCALE_2`"]
    #[inline(always)]
    pub fn is_audioclk_prescale_2(&self) -> bool {
        *self == AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `AUDIOCLK_PRESCALE_3`"]
    #[inline(always)]
    pub fn is_audioclk_prescale_3(&self) -> bool {
        *self == AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_3
    }
    #[doc = "Checks if the value of the field is `AUDIOCLK_PRESCALE_4`"]
    #[inline(always)]
    pub fn is_audioclk_prescale_4(&self) -> bool {
        *self == AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_4
    }
    #[doc = "Checks if the value of the field is `AUDIOCLK_PRESCALE_5`"]
    #[inline(always)]
    pub fn is_audioclk_prescale_5(&self) -> bool {
        *self == AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_5
    }
    #[doc = "Checks if the value of the field is `AUDIOCLK_PRESCALE_6`"]
    #[inline(always)]
    pub fn is_audioclk_prescale_6(&self) -> bool {
        *self == AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_6
    }
    #[doc = "Checks if the value of the field is `AUDIOCLK_PRESCALE_7`"]
    #[inline(always)]
    pub fn is_audioclk_prescale_7(&self) -> bool {
        *self == AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_7
    }
    #[doc = "Checks if the value of the field is `AUDIOCLK_PRESCALE_8`"]
    #[inline(always)]
    pub fn is_audioclk_prescale_8(&self) -> bool {
        *self == AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_8
    }
    #[doc = "Checks if the value of the field is `AUDIOCLK_PRESCALE_9`"]
    #[inline(always)]
    pub fn is_audioclk_prescale_9(&self) -> bool {
        *self == AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_9
    }
    #[doc = "Checks if the value of the field is `AUDIOCLK_PRESCALE_10`"]
    #[inline(always)]
    pub fn is_audioclk_prescale_10(&self) -> bool {
        *self == AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_10
    }
    #[doc = "Checks if the value of the field is `AUDIOCLK_PRESCALE_11`"]
    #[inline(always)]
    pub fn is_audioclk_prescale_11(&self) -> bool {
        *self == AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_11
    }
    #[doc = "Checks if the value of the field is `AUDIOCLK_PRESCALE_12`"]
    #[inline(always)]
    pub fn is_audioclk_prescale_12(&self) -> bool {
        *self == AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_12
    }
    #[doc = "Checks if the value of the field is `AUDIOCLK_PRESCALE_13`"]
    #[inline(always)]
    pub fn is_audioclk_prescale_13(&self) -> bool {
        *self == AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_13
    }
    #[doc = "Checks if the value of the field is `AUDIOCLK_PRESCALE_14`"]
    #[inline(always)]
    pub fn is_audioclk_prescale_14(&self) -> bool {
        *self == AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_14
    }
    #[doc = "Checks if the value of the field is `AUDIOCLK_PRESCALE_15`"]
    #[inline(always)]
    pub fn is_audioclk_prescale_15(&self) -> bool {
        *self == AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_15
    }
    #[doc = "Checks if the value of the field is `AUDIOCLK_PRESCALE_16`"]
    #[inline(always)]
    pub fn is_audioclk_prescale_16(&self) -> bool {
        *self == AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_16
    }
    #[doc = "Checks if the value of the field is `AUDIOCLK_PRESCALE_63`"]
    #[inline(always)]
    pub fn is_audioclk_prescale_63(&self) -> bool {
        *self == AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_63
    }
    #[doc = "Checks if the value of the field is `AUDIOCLK_PRESCALE_64`"]
    #[inline(always)]
    pub fn is_audioclk_prescale_64(&self) -> bool {
        *self == AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_64
    }
}
#[doc = "Write proxy for field `AUDIOCLK_PRESCALE`"]
pub struct AUDIOCLK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIOCLK_PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIOCLK_PRESCALE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn audioclk_prescale_1(self) -> &'a mut W {
        self.variant(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn audioclk_prescale_2(self) -> &'a mut W {
        self.variant(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn audioclk_prescale_3(self) -> &'a mut W {
        self.variant(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn audioclk_prescale_4(self) -> &'a mut W {
        self.variant(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn audioclk_prescale_5(self) -> &'a mut W {
        self.variant(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn audioclk_prescale_6(self) -> &'a mut W {
        self.variant(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn audioclk_prescale_7(self) -> &'a mut W {
        self.variant(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn audioclk_prescale_8(self) -> &'a mut W {
        self.variant(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_8)
    }
    #[doc = "Divide by 9"]
    #[inline(always)]
    pub fn audioclk_prescale_9(self) -> &'a mut W {
        self.variant(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_9)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn audioclk_prescale_10(self) -> &'a mut W {
        self.variant(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_10)
    }
    #[doc = "Divide by 11"]
    #[inline(always)]
    pub fn audioclk_prescale_11(self) -> &'a mut W {
        self.variant(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_11)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn audioclk_prescale_12(self) -> &'a mut W {
        self.variant(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_12)
    }
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn audioclk_prescale_13(self) -> &'a mut W {
        self.variant(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_13)
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn audioclk_prescale_14(self) -> &'a mut W {
        self.variant(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_14)
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn audioclk_prescale_15(self) -> &'a mut W {
        self.variant(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_15)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn audioclk_prescale_16(self) -> &'a mut W {
        self.variant(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_16)
    }
    #[doc = "Divide by 63"]
    #[inline(always)]
    pub fn audioclk_prescale_63(self) -> &'a mut W {
        self.variant(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_63)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn audioclk_prescale_64(self) -> &'a mut W {
        self.variant(AUDIOCLK_PRESCALE_A::AUDIOCLK_PRESCALE_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Prescale value for the UART peripheral clock (1 to 32 in steps of 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UARTCLK_PRESCALE_A {
    #[doc = "0: Divide by 1"]
    UARTCLK_PRESCALE_1 = 0,
    #[doc = "1: Divide by 2"]
    UARTCLK_PRESCALE_2 = 1,
    #[doc = "30: Divide by 31"]
    UARTCLK_PRESCALE_31 = 30,
    #[doc = "31: Divide by 32"]
    UARTCLK_PRESCALE_32 = 31,
}
impl From<UARTCLK_PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: UARTCLK_PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UARTCLK_PRESCALE`"]
pub type UARTCLK_PRESCALE_R = crate::R<u8, UARTCLK_PRESCALE_A>;
impl UARTCLK_PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UARTCLK_PRESCALE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UARTCLK_PRESCALE_A::UARTCLK_PRESCALE_1),
            1 => Val(UARTCLK_PRESCALE_A::UARTCLK_PRESCALE_2),
            30 => Val(UARTCLK_PRESCALE_A::UARTCLK_PRESCALE_31),
            31 => Val(UARTCLK_PRESCALE_A::UARTCLK_PRESCALE_32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UARTCLK_PRESCALE_1`"]
    #[inline(always)]
    pub fn is_uartclk_prescale_1(&self) -> bool {
        *self == UARTCLK_PRESCALE_A::UARTCLK_PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `UARTCLK_PRESCALE_2`"]
    #[inline(always)]
    pub fn is_uartclk_prescale_2(&self) -> bool {
        *self == UARTCLK_PRESCALE_A::UARTCLK_PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `UARTCLK_PRESCALE_31`"]
    #[inline(always)]
    pub fn is_uartclk_prescale_31(&self) -> bool {
        *self == UARTCLK_PRESCALE_A::UARTCLK_PRESCALE_31
    }
    #[doc = "Checks if the value of the field is `UARTCLK_PRESCALE_32`"]
    #[inline(always)]
    pub fn is_uartclk_prescale_32(&self) -> bool {
        *self == UARTCLK_PRESCALE_A::UARTCLK_PRESCALE_32
    }
}
#[doc = "Write proxy for field `UARTCLK_PRESCALE`"]
pub struct UARTCLK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTCLK_PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UARTCLK_PRESCALE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn uartclk_prescale_1(self) -> &'a mut W {
        self.variant(UARTCLK_PRESCALE_A::UARTCLK_PRESCALE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn uartclk_prescale_2(self) -> &'a mut W {
        self.variant(UARTCLK_PRESCALE_A::UARTCLK_PRESCALE_2)
    }
    #[doc = "Divide by 31"]
    #[inline(always)]
    pub fn uartclk_prescale_31(self) -> &'a mut W {
        self.variant(UARTCLK_PRESCALE_A::UARTCLK_PRESCALE_31)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn uartclk_prescale_32(self) -> &'a mut W {
        self.variant(UARTCLK_PRESCALE_A::UARTCLK_PRESCALE_32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Prescale value for the PWM1 peripheral clock (1 to 64 in steps of 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM1CLK_PRESCALE_A {
    #[doc = "0: Divide by 1"]
    PWM1CLK_PRESCALE_1 = 0,
    #[doc = "1: Divide by 2"]
    PWM1CLK_PRESCALE_2 = 1,
    #[doc = "62: Divide by 63"]
    PWM1CLK_PRESCALE_63 = 62,
    #[doc = "63: Divide by 64"]
    PWM1CLK_PRESCALE_64 = 63,
}
impl From<PWM1CLK_PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM1CLK_PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWM1CLK_PRESCALE`"]
pub type PWM1CLK_PRESCALE_R = crate::R<u8, PWM1CLK_PRESCALE_A>;
impl PWM1CLK_PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PWM1CLK_PRESCALE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PWM1CLK_PRESCALE_A::PWM1CLK_PRESCALE_1),
            1 => Val(PWM1CLK_PRESCALE_A::PWM1CLK_PRESCALE_2),
            62 => Val(PWM1CLK_PRESCALE_A::PWM1CLK_PRESCALE_63),
            63 => Val(PWM1CLK_PRESCALE_A::PWM1CLK_PRESCALE_64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWM1CLK_PRESCALE_1`"]
    #[inline(always)]
    pub fn is_pwm1clk_prescale_1(&self) -> bool {
        *self == PWM1CLK_PRESCALE_A::PWM1CLK_PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `PWM1CLK_PRESCALE_2`"]
    #[inline(always)]
    pub fn is_pwm1clk_prescale_2(&self) -> bool {
        *self == PWM1CLK_PRESCALE_A::PWM1CLK_PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `PWM1CLK_PRESCALE_63`"]
    #[inline(always)]
    pub fn is_pwm1clk_prescale_63(&self) -> bool {
        *self == PWM1CLK_PRESCALE_A::PWM1CLK_PRESCALE_63
    }
    #[doc = "Checks if the value of the field is `PWM1CLK_PRESCALE_64`"]
    #[inline(always)]
    pub fn is_pwm1clk_prescale_64(&self) -> bool {
        *self == PWM1CLK_PRESCALE_A::PWM1CLK_PRESCALE_64
    }
}
#[doc = "Write proxy for field `PWM1CLK_PRESCALE`"]
pub struct PWM1CLK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM1CLK_PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM1CLK_PRESCALE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn pwm1clk_prescale_1(self) -> &'a mut W {
        self.variant(PWM1CLK_PRESCALE_A::PWM1CLK_PRESCALE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn pwm1clk_prescale_2(self) -> &'a mut W {
        self.variant(PWM1CLK_PRESCALE_A::PWM1CLK_PRESCALE_2)
    }
    #[doc = "Divide by 63"]
    #[inline(always)]
    pub fn pwm1clk_prescale_63(self) -> &'a mut W {
        self.variant(PWM1CLK_PRESCALE_A::PWM1CLK_PRESCALE_63)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn pwm1clk_prescale_64(self) -> &'a mut W {
        self.variant(PWM1CLK_PRESCALE_A::PWM1CLK_PRESCALE_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Prescale value for the PWM0 peripheral clock (1 to 64 in steps of 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM0CLK_PRESCALE_A {
    #[doc = "0: Divide by 1"]
    PWM0CLK_PRESCALE_1 = 0,
    #[doc = "1: Divide by 2"]
    PWM0CLK_PRESCALE_2 = 1,
    #[doc = "62: Divide by 63"]
    PWM0CLK_PRESCALE_63 = 62,
    #[doc = "63: Divide by 64"]
    PWM0CLK_PRESCALE_64 = 63,
}
impl From<PWM0CLK_PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM0CLK_PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWM0CLK_PRESCALE`"]
pub type PWM0CLK_PRESCALE_R = crate::R<u8, PWM0CLK_PRESCALE_A>;
impl PWM0CLK_PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PWM0CLK_PRESCALE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PWM0CLK_PRESCALE_A::PWM0CLK_PRESCALE_1),
            1 => Val(PWM0CLK_PRESCALE_A::PWM0CLK_PRESCALE_2),
            62 => Val(PWM0CLK_PRESCALE_A::PWM0CLK_PRESCALE_63),
            63 => Val(PWM0CLK_PRESCALE_A::PWM0CLK_PRESCALE_64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWM0CLK_PRESCALE_1`"]
    #[inline(always)]
    pub fn is_pwm0clk_prescale_1(&self) -> bool {
        *self == PWM0CLK_PRESCALE_A::PWM0CLK_PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `PWM0CLK_PRESCALE_2`"]
    #[inline(always)]
    pub fn is_pwm0clk_prescale_2(&self) -> bool {
        *self == PWM0CLK_PRESCALE_A::PWM0CLK_PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `PWM0CLK_PRESCALE_63`"]
    #[inline(always)]
    pub fn is_pwm0clk_prescale_63(&self) -> bool {
        *self == PWM0CLK_PRESCALE_A::PWM0CLK_PRESCALE_63
    }
    #[doc = "Checks if the value of the field is `PWM0CLK_PRESCALE_64`"]
    #[inline(always)]
    pub fn is_pwm0clk_prescale_64(&self) -> bool {
        *self == PWM0CLK_PRESCALE_A::PWM0CLK_PRESCALE_64
    }
}
#[doc = "Write proxy for field `PWM0CLK_PRESCALE`"]
pub struct PWM0CLK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM0CLK_PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM0CLK_PRESCALE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn pwm0clk_prescale_1(self) -> &'a mut W {
        self.variant(PWM0CLK_PRESCALE_A::PWM0CLK_PRESCALE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn pwm0clk_prescale_2(self) -> &'a mut W {
        self.variant(PWM0CLK_PRESCALE_A::PWM0CLK_PRESCALE_2)
    }
    #[doc = "Divide by 63"]
    #[inline(always)]
    pub fn pwm0clk_prescale_63(self) -> &'a mut W {
        self.variant(PWM0CLK_PRESCALE_A::PWM0CLK_PRESCALE_63)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn pwm0clk_prescale_64(self) -> &'a mut W {
        self.variant(PWM0CLK_PRESCALE_A::PWM0CLK_PRESCALE_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Prescale value for the slow audio clock down from the fast audio clock (1 to 4 in steps of 1)"]
    #[inline(always)]
    pub fn audioslowclk_prescale(&self) -> AUDIOSLOWCLK_PRESCALE_R {
        AUDIOSLOWCLK_PRESCALE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 24:29 - Prescale value for the fast audio clock (1 to 64 in steps of 1)"]
    #[inline(always)]
    pub fn audioclk_prescale(&self) -> AUDIOCLK_PRESCALE_R {
        AUDIOCLK_PRESCALE_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Prescale value for the UART peripheral clock (1 to 32 in steps of 1)"]
    #[inline(always)]
    pub fn uartclk_prescale(&self) -> UARTCLK_PRESCALE_R {
        UARTCLK_PRESCALE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Prescale value for the PWM1 peripheral clock (1 to 64 in steps of 1)"]
    #[inline(always)]
    pub fn pwm1clk_prescale(&self) -> PWM1CLK_PRESCALE_R {
        PWM1CLK_PRESCALE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - Prescale value for the PWM0 peripheral clock (1 to 64 in steps of 1)"]
    #[inline(always)]
    pub fn pwm0clk_prescale(&self) -> PWM0CLK_PRESCALE_R {
        PWM0CLK_PRESCALE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Prescale value for the slow audio clock down from the fast audio clock (1 to 4 in steps of 1)"]
    #[inline(always)]
    pub fn audioslowclk_prescale(&mut self) -> AUDIOSLOWCLK_PRESCALE_W {
        AUDIOSLOWCLK_PRESCALE_W { w: self }
    }
    #[doc = "Bits 24:29 - Prescale value for the fast audio clock (1 to 64 in steps of 1)"]
    #[inline(always)]
    pub fn audioclk_prescale(&mut self) -> AUDIOCLK_PRESCALE_W {
        AUDIOCLK_PRESCALE_W { w: self }
    }
    #[doc = "Bits 16:20 - Prescale value for the UART peripheral clock (1 to 32 in steps of 1)"]
    #[inline(always)]
    pub fn uartclk_prescale(&mut self) -> UARTCLK_PRESCALE_W {
        UARTCLK_PRESCALE_W { w: self }
    }
    #[doc = "Bits 8:13 - Prescale value for the PWM1 peripheral clock (1 to 64 in steps of 1)"]
    #[inline(always)]
    pub fn pwm1clk_prescale(&mut self) -> PWM1CLK_PRESCALE_W {
        PWM1CLK_PRESCALE_W { w: self }
    }
    #[doc = "Bits 0:5 - Prescale value for the PWM0 peripheral clock (1 to 64 in steps of 1)"]
    #[inline(always)]
    pub fn pwm0clk_prescale(&mut self) -> PWM0CLK_PRESCALE_W {
        PWM0CLK_PRESCALE_W { w: self }
    }
}
