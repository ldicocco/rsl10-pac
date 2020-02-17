#[doc = "Reader of register CLK_DIV_CFG0"]
pub type R = crate::R<u32, super::CLK_DIV_CFG0>;
#[doc = "Writer for register CLK_DIV_CFG0"]
pub type W = crate::W<u32, super::CLK_DIV_CFG0>;
#[doc = "Register CLK_DIV_CFG0 `reset()`'s with value 0x02"]
impl crate::ResetValue for super::CLK_DIV_CFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Prescale value for the USR clock (1 to 4096 in steps of 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum USRCLK_PRESCALE_A {
    #[doc = "0: Divide by 1"]
    USRCLK_PRESCALE_1 = 0,
    #[doc = "1: Divide by 2"]
    USRCLK_PRESCALE_2 = 1,
    #[doc = "2: Divide by 3"]
    USRCLK_PRESCALE_3 = 2,
    #[doc = "4094: Divide by 4095"]
    USRCLK_PRESCALE_4095 = 4094,
    #[doc = "4095: Divide by 4096"]
    USRCLK_PRESCALE_4096 = 4095,
}
impl From<USRCLK_PRESCALE_A> for u16 {
    #[inline(always)]
    fn from(variant: USRCLK_PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `USRCLK_PRESCALE`"]
pub type USRCLK_PRESCALE_R = crate::R<u16, USRCLK_PRESCALE_A>;
impl USRCLK_PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, USRCLK_PRESCALE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(USRCLK_PRESCALE_A::USRCLK_PRESCALE_1),
            1 => Val(USRCLK_PRESCALE_A::USRCLK_PRESCALE_2),
            2 => Val(USRCLK_PRESCALE_A::USRCLK_PRESCALE_3),
            4094 => Val(USRCLK_PRESCALE_A::USRCLK_PRESCALE_4095),
            4095 => Val(USRCLK_PRESCALE_A::USRCLK_PRESCALE_4096),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `USRCLK_PRESCALE_1`"]
    #[inline(always)]
    pub fn is_usrclk_prescale_1(&self) -> bool {
        *self == USRCLK_PRESCALE_A::USRCLK_PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `USRCLK_PRESCALE_2`"]
    #[inline(always)]
    pub fn is_usrclk_prescale_2(&self) -> bool {
        *self == USRCLK_PRESCALE_A::USRCLK_PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `USRCLK_PRESCALE_3`"]
    #[inline(always)]
    pub fn is_usrclk_prescale_3(&self) -> bool {
        *self == USRCLK_PRESCALE_A::USRCLK_PRESCALE_3
    }
    #[doc = "Checks if the value of the field is `USRCLK_PRESCALE_4095`"]
    #[inline(always)]
    pub fn is_usrclk_prescale_4095(&self) -> bool {
        *self == USRCLK_PRESCALE_A::USRCLK_PRESCALE_4095
    }
    #[doc = "Checks if the value of the field is `USRCLK_PRESCALE_4096`"]
    #[inline(always)]
    pub fn is_usrclk_prescale_4096(&self) -> bool {
        *self == USRCLK_PRESCALE_A::USRCLK_PRESCALE_4096
    }
}
#[doc = "Write proxy for field `USRCLK_PRESCALE`"]
pub struct USRCLK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> USRCLK_PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USRCLK_PRESCALE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn usrclk_prescale_1(self) -> &'a mut W {
        self.variant(USRCLK_PRESCALE_A::USRCLK_PRESCALE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn usrclk_prescale_2(self) -> &'a mut W {
        self.variant(USRCLK_PRESCALE_A::USRCLK_PRESCALE_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn usrclk_prescale_3(self) -> &'a mut W {
        self.variant(USRCLK_PRESCALE_A::USRCLK_PRESCALE_3)
    }
    #[doc = "Divide by 4095"]
    #[inline(always)]
    pub fn usrclk_prescale_4095(self) -> &'a mut W {
        self.variant(USRCLK_PRESCALE_A::USRCLK_PRESCALE_4095)
    }
    #[doc = "Divide by 4096"]
    #[inline(always)]
    pub fn usrclk_prescale_4096(self) -> &'a mut W {
        self.variant(USRCLK_PRESCALE_A::USRCLK_PRESCALE_4096)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Prescale value for the Baseband peripheral clock (1 to 8 in steps of 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BBCLK_PRESCALE_A {
    #[doc = "0: Divide by 1"]
    BBCLK_PRESCALE_1 = 0,
    #[doc = "1: Divide by 2"]
    BBCLK_PRESCALE_2 = 1,
    #[doc = "2: Divide by 3"]
    BBCLK_PRESCALE_3 = 2,
    #[doc = "3: Divide by 4"]
    BBCLK_PRESCALE_4 = 3,
    #[doc = "4: Divide by 5"]
    BBCLK_PRESCALE_5 = 4,
    #[doc = "5: Divide by 6"]
    BBCLK_PRESCALE_6 = 5,
    #[doc = "6: Divide by 7"]
    BBCLK_PRESCALE_7 = 6,
    #[doc = "7: Divide by 8"]
    BBCLK_PRESCALE_8 = 7,
}
impl From<BBCLK_PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: BBCLK_PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BBCLK_PRESCALE`"]
pub type BBCLK_PRESCALE_R = crate::R<u8, BBCLK_PRESCALE_A>;
impl BBCLK_PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BBCLK_PRESCALE_A {
        match self.bits {
            0 => BBCLK_PRESCALE_A::BBCLK_PRESCALE_1,
            1 => BBCLK_PRESCALE_A::BBCLK_PRESCALE_2,
            2 => BBCLK_PRESCALE_A::BBCLK_PRESCALE_3,
            3 => BBCLK_PRESCALE_A::BBCLK_PRESCALE_4,
            4 => BBCLK_PRESCALE_A::BBCLK_PRESCALE_5,
            5 => BBCLK_PRESCALE_A::BBCLK_PRESCALE_6,
            6 => BBCLK_PRESCALE_A::BBCLK_PRESCALE_7,
            7 => BBCLK_PRESCALE_A::BBCLK_PRESCALE_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BBCLK_PRESCALE_1`"]
    #[inline(always)]
    pub fn is_bbclk_prescale_1(&self) -> bool {
        *self == BBCLK_PRESCALE_A::BBCLK_PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `BBCLK_PRESCALE_2`"]
    #[inline(always)]
    pub fn is_bbclk_prescale_2(&self) -> bool {
        *self == BBCLK_PRESCALE_A::BBCLK_PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `BBCLK_PRESCALE_3`"]
    #[inline(always)]
    pub fn is_bbclk_prescale_3(&self) -> bool {
        *self == BBCLK_PRESCALE_A::BBCLK_PRESCALE_3
    }
    #[doc = "Checks if the value of the field is `BBCLK_PRESCALE_4`"]
    #[inline(always)]
    pub fn is_bbclk_prescale_4(&self) -> bool {
        *self == BBCLK_PRESCALE_A::BBCLK_PRESCALE_4
    }
    #[doc = "Checks if the value of the field is `BBCLK_PRESCALE_5`"]
    #[inline(always)]
    pub fn is_bbclk_prescale_5(&self) -> bool {
        *self == BBCLK_PRESCALE_A::BBCLK_PRESCALE_5
    }
    #[doc = "Checks if the value of the field is `BBCLK_PRESCALE_6`"]
    #[inline(always)]
    pub fn is_bbclk_prescale_6(&self) -> bool {
        *self == BBCLK_PRESCALE_A::BBCLK_PRESCALE_6
    }
    #[doc = "Checks if the value of the field is `BBCLK_PRESCALE_7`"]
    #[inline(always)]
    pub fn is_bbclk_prescale_7(&self) -> bool {
        *self == BBCLK_PRESCALE_A::BBCLK_PRESCALE_7
    }
    #[doc = "Checks if the value of the field is `BBCLK_PRESCALE_8`"]
    #[inline(always)]
    pub fn is_bbclk_prescale_8(&self) -> bool {
        *self == BBCLK_PRESCALE_A::BBCLK_PRESCALE_8
    }
}
#[doc = "Write proxy for field `BBCLK_PRESCALE`"]
pub struct BBCLK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> BBCLK_PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BBCLK_PRESCALE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn bbclk_prescale_1(self) -> &'a mut W {
        self.variant(BBCLK_PRESCALE_A::BBCLK_PRESCALE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn bbclk_prescale_2(self) -> &'a mut W {
        self.variant(BBCLK_PRESCALE_A::BBCLK_PRESCALE_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn bbclk_prescale_3(self) -> &'a mut W {
        self.variant(BBCLK_PRESCALE_A::BBCLK_PRESCALE_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn bbclk_prescale_4(self) -> &'a mut W {
        self.variant(BBCLK_PRESCALE_A::BBCLK_PRESCALE_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn bbclk_prescale_5(self) -> &'a mut W {
        self.variant(BBCLK_PRESCALE_A::BBCLK_PRESCALE_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn bbclk_prescale_6(self) -> &'a mut W {
        self.variant(BBCLK_PRESCALE_A::BBCLK_PRESCALE_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn bbclk_prescale_7(self) -> &'a mut W {
        self.variant(BBCLK_PRESCALE_A::BBCLK_PRESCALE_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn bbclk_prescale_8(self) -> &'a mut W {
        self.variant(BBCLK_PRESCALE_A::BBCLK_PRESCALE_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Prescale value for the SLOWCLK clock (1 to 64 in steps of 1)\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLOWCLK_PRESCALE_A {
    #[doc = "0: Divide by 1"]
    SLOWCLK_PRESCALE_1 = 0,
    #[doc = "1: Divide by 2"]
    SLOWCLK_PRESCALE_2 = 1,
    #[doc = "2: Divide by 3"]
    SLOWCLK_PRESCALE_3 = 2,
    #[doc = "3: Divide by 4"]
    SLOWCLK_PRESCALE_4 = 3,
    #[doc = "7: Divide by 8"]
    SLOWCLK_PRESCALE_8 = 7,
    #[doc = "9: Divide by 10"]
    SLOWCLK_PRESCALE_10 = 9,
    #[doc = "11: Divide by 12"]
    SLOWCLK_PRESCALE_12 = 11,
    #[doc = "15: Divide by 16"]
    SLOWCLK_PRESCALE_16 = 15,
    #[doc = "23: Divide by 24"]
    SLOWCLK_PRESCALE_24 = 23,
    #[doc = "47: Divide by 48"]
    SLOWCLK_PRESCALE_48 = 47,
    #[doc = "62: Divide by 63"]
    SLOWCLK_PRESCALE_63 = 62,
    #[doc = "63: Divide by 64"]
    SLOWCLK_PRESCALE_64 = 63,
}
impl From<SLOWCLK_PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: SLOWCLK_PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SLOWCLK_PRESCALE`"]
pub type SLOWCLK_PRESCALE_R = crate::R<u8, SLOWCLK_PRESCALE_A>;
impl SLOWCLK_PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SLOWCLK_PRESCALE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_1),
            1 => Val(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_2),
            2 => Val(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_3),
            3 => Val(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_4),
            7 => Val(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_8),
            9 => Val(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_10),
            11 => Val(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_12),
            15 => Val(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_16),
            23 => Val(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_24),
            47 => Val(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_48),
            62 => Val(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_63),
            63 => Val(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLOWCLK_PRESCALE_1`"]
    #[inline(always)]
    pub fn is_slowclk_prescale_1(&self) -> bool {
        *self == SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `SLOWCLK_PRESCALE_2`"]
    #[inline(always)]
    pub fn is_slowclk_prescale_2(&self) -> bool {
        *self == SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `SLOWCLK_PRESCALE_3`"]
    #[inline(always)]
    pub fn is_slowclk_prescale_3(&self) -> bool {
        *self == SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_3
    }
    #[doc = "Checks if the value of the field is `SLOWCLK_PRESCALE_4`"]
    #[inline(always)]
    pub fn is_slowclk_prescale_4(&self) -> bool {
        *self == SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_4
    }
    #[doc = "Checks if the value of the field is `SLOWCLK_PRESCALE_8`"]
    #[inline(always)]
    pub fn is_slowclk_prescale_8(&self) -> bool {
        *self == SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_8
    }
    #[doc = "Checks if the value of the field is `SLOWCLK_PRESCALE_10`"]
    #[inline(always)]
    pub fn is_slowclk_prescale_10(&self) -> bool {
        *self == SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_10
    }
    #[doc = "Checks if the value of the field is `SLOWCLK_PRESCALE_12`"]
    #[inline(always)]
    pub fn is_slowclk_prescale_12(&self) -> bool {
        *self == SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_12
    }
    #[doc = "Checks if the value of the field is `SLOWCLK_PRESCALE_16`"]
    #[inline(always)]
    pub fn is_slowclk_prescale_16(&self) -> bool {
        *self == SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_16
    }
    #[doc = "Checks if the value of the field is `SLOWCLK_PRESCALE_24`"]
    #[inline(always)]
    pub fn is_slowclk_prescale_24(&self) -> bool {
        *self == SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_24
    }
    #[doc = "Checks if the value of the field is `SLOWCLK_PRESCALE_48`"]
    #[inline(always)]
    pub fn is_slowclk_prescale_48(&self) -> bool {
        *self == SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_48
    }
    #[doc = "Checks if the value of the field is `SLOWCLK_PRESCALE_63`"]
    #[inline(always)]
    pub fn is_slowclk_prescale_63(&self) -> bool {
        *self == SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_63
    }
    #[doc = "Checks if the value of the field is `SLOWCLK_PRESCALE_64`"]
    #[inline(always)]
    pub fn is_slowclk_prescale_64(&self) -> bool {
        *self == SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_64
    }
}
#[doc = "Write proxy for field `SLOWCLK_PRESCALE`"]
pub struct SLOWCLK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOWCLK_PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOWCLK_PRESCALE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn slowclk_prescale_1(self) -> &'a mut W {
        self.variant(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn slowclk_prescale_2(self) -> &'a mut W {
        self.variant(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn slowclk_prescale_3(self) -> &'a mut W {
        self.variant(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn slowclk_prescale_4(self) -> &'a mut W {
        self.variant(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn slowclk_prescale_8(self) -> &'a mut W {
        self.variant(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_8)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn slowclk_prescale_10(self) -> &'a mut W {
        self.variant(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_10)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn slowclk_prescale_12(self) -> &'a mut W {
        self.variant(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_12)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn slowclk_prescale_16(self) -> &'a mut W {
        self.variant(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_16)
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn slowclk_prescale_24(self) -> &'a mut W {
        self.variant(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_24)
    }
    #[doc = "Divide by 48"]
    #[inline(always)]
    pub fn slowclk_prescale_48(self) -> &'a mut W {
        self.variant(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_48)
    }
    #[doc = "Divide by 63"]
    #[inline(always)]
    pub fn slowclk_prescale_63(self) -> &'a mut W {
        self.variant(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_63)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn slowclk_prescale_64(self) -> &'a mut W {
        self.variant(SLOWCLK_PRESCALE_A::SLOWCLK_PRESCALE_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27 - Prescale value for the USR clock (1 to 4096 in steps of 1)"]
    #[inline(always)]
    pub fn usrclk_prescale(&self) -> USRCLK_PRESCALE_R {
        USRCLK_PRESCALE_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 8:10 - Prescale value for the Baseband peripheral clock (1 to 8 in steps of 1)"]
    #[inline(always)]
    pub fn bbclk_prescale(&self) -> BBCLK_PRESCALE_R {
        BBCLK_PRESCALE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:5 - Prescale value for the SLOWCLK clock (1 to 64 in steps of 1)"]
    #[inline(always)]
    pub fn slowclk_prescale(&self) -> SLOWCLK_PRESCALE_R {
        SLOWCLK_PRESCALE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:27 - Prescale value for the USR clock (1 to 4096 in steps of 1)"]
    #[inline(always)]
    pub fn usrclk_prescale(&mut self) -> USRCLK_PRESCALE_W {
        USRCLK_PRESCALE_W { w: self }
    }
    #[doc = "Bits 8:10 - Prescale value for the Baseband peripheral clock (1 to 8 in steps of 1)"]
    #[inline(always)]
    pub fn bbclk_prescale(&mut self) -> BBCLK_PRESCALE_W {
        BBCLK_PRESCALE_W { w: self }
    }
    #[doc = "Bits 0:5 - Prescale value for the SLOWCLK clock (1 to 64 in steps of 1)"]
    #[inline(always)]
    pub fn slowclk_prescale(&mut self) -> SLOWCLK_PRESCALE_W {
        SLOWCLK_PRESCALE_W { w: self }
    }
}
