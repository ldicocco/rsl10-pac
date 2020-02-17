#[doc = "Reader of register CLK_DIV_CFG2"]
pub type R = crate::R<u32, super::CLK_DIV_CFG2>;
#[doc = "Writer for register CLK_DIV_CFG2"]
pub type W = crate::W<u32, super::CLK_DIV_CFG2>;
#[doc = "Register CLK_DIV_CFG2 `reset()`'s with value 0x0700"]
impl crate::ResetValue for super::CLK_DIV_CFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0700
    }
}
#[doc = "Charge pump clock disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPCLK_DISABLE_A {
    #[doc = "0: Charge pump clock enabled"]
    CPCLK_ENABLE = 0,
    #[doc = "1: Charge pump clock disabled"]
    CPCLK_DISABLE = 1,
}
impl From<CPCLK_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CPCLK_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPCLK_DISABLE`"]
pub type CPCLK_DISABLE_R = crate::R<bool, CPCLK_DISABLE_A>;
impl CPCLK_DISABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPCLK_DISABLE_A {
        match self.bits {
            false => CPCLK_DISABLE_A::CPCLK_ENABLE,
            true => CPCLK_DISABLE_A::CPCLK_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CPCLK_ENABLE`"]
    #[inline(always)]
    pub fn is_cpclk_enable(&self) -> bool {
        *self == CPCLK_DISABLE_A::CPCLK_ENABLE
    }
    #[doc = "Checks if the value of the field is `CPCLK_DISABLE`"]
    #[inline(always)]
    pub fn is_cpclk_disable(&self) -> bool {
        *self == CPCLK_DISABLE_A::CPCLK_DISABLE
    }
}
#[doc = "Write proxy for field `CPCLK_DISABLE`"]
pub struct CPCLK_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPCLK_DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPCLK_DISABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Charge pump clock enabled"]
    #[inline(always)]
    pub fn cpclk_enable(self) -> &'a mut W {
        self.variant(CPCLK_DISABLE_A::CPCLK_ENABLE)
    }
    #[doc = "Charge pump clock disabled"]
    #[inline(always)]
    pub fn cpclk_disable(self) -> &'a mut W {
        self.variant(CPCLK_DISABLE_A::CPCLK_DISABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Prescale value for the charge pump clock from the SLOWCLK clock (1 to 64 in steps of 1)\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPCLK_PRESCALE_A {
    #[doc = "0: Divide by 1"]
    CPCLK_PRESCALE_1 = 0,
    #[doc = "1: Divide by 2"]
    CPCLK_PRESCALE_2 = 1,
    #[doc = "2: Divide by 3"]
    CPCLK_PRESCALE_3 = 2,
    #[doc = "3: Divide by 4"]
    CPCLK_PRESCALE_4 = 3,
    #[doc = "4: Divide by 5"]
    CPCLK_PRESCALE_5 = 4,
    #[doc = "5: Divide by 6"]
    CPCLK_PRESCALE_6 = 5,
    #[doc = "6: Divide by 7"]
    CPCLK_PRESCALE_7 = 6,
    #[doc = "7: Divide by 8"]
    CPCLK_PRESCALE_8 = 7,
    #[doc = "8: Divide by 9"]
    CPCLK_PRESCALE_9 = 8,
    #[doc = "9: Divide by 10"]
    CPCLK_PRESCALE_10 = 9,
    #[doc = "62: Divide by 63"]
    CPCLK_PRESCALE_63 = 62,
    #[doc = "63: Divide by 64"]
    CPCLK_PRESCALE_64 = 63,
}
impl From<CPCLK_PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: CPCLK_PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CPCLK_PRESCALE`"]
pub type CPCLK_PRESCALE_R = crate::R<u8, CPCLK_PRESCALE_A>;
impl CPCLK_PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CPCLK_PRESCALE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CPCLK_PRESCALE_A::CPCLK_PRESCALE_1),
            1 => Val(CPCLK_PRESCALE_A::CPCLK_PRESCALE_2),
            2 => Val(CPCLK_PRESCALE_A::CPCLK_PRESCALE_3),
            3 => Val(CPCLK_PRESCALE_A::CPCLK_PRESCALE_4),
            4 => Val(CPCLK_PRESCALE_A::CPCLK_PRESCALE_5),
            5 => Val(CPCLK_PRESCALE_A::CPCLK_PRESCALE_6),
            6 => Val(CPCLK_PRESCALE_A::CPCLK_PRESCALE_7),
            7 => Val(CPCLK_PRESCALE_A::CPCLK_PRESCALE_8),
            8 => Val(CPCLK_PRESCALE_A::CPCLK_PRESCALE_9),
            9 => Val(CPCLK_PRESCALE_A::CPCLK_PRESCALE_10),
            62 => Val(CPCLK_PRESCALE_A::CPCLK_PRESCALE_63),
            63 => Val(CPCLK_PRESCALE_A::CPCLK_PRESCALE_64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CPCLK_PRESCALE_1`"]
    #[inline(always)]
    pub fn is_cpclk_prescale_1(&self) -> bool {
        *self == CPCLK_PRESCALE_A::CPCLK_PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `CPCLK_PRESCALE_2`"]
    #[inline(always)]
    pub fn is_cpclk_prescale_2(&self) -> bool {
        *self == CPCLK_PRESCALE_A::CPCLK_PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `CPCLK_PRESCALE_3`"]
    #[inline(always)]
    pub fn is_cpclk_prescale_3(&self) -> bool {
        *self == CPCLK_PRESCALE_A::CPCLK_PRESCALE_3
    }
    #[doc = "Checks if the value of the field is `CPCLK_PRESCALE_4`"]
    #[inline(always)]
    pub fn is_cpclk_prescale_4(&self) -> bool {
        *self == CPCLK_PRESCALE_A::CPCLK_PRESCALE_4
    }
    #[doc = "Checks if the value of the field is `CPCLK_PRESCALE_5`"]
    #[inline(always)]
    pub fn is_cpclk_prescale_5(&self) -> bool {
        *self == CPCLK_PRESCALE_A::CPCLK_PRESCALE_5
    }
    #[doc = "Checks if the value of the field is `CPCLK_PRESCALE_6`"]
    #[inline(always)]
    pub fn is_cpclk_prescale_6(&self) -> bool {
        *self == CPCLK_PRESCALE_A::CPCLK_PRESCALE_6
    }
    #[doc = "Checks if the value of the field is `CPCLK_PRESCALE_7`"]
    #[inline(always)]
    pub fn is_cpclk_prescale_7(&self) -> bool {
        *self == CPCLK_PRESCALE_A::CPCLK_PRESCALE_7
    }
    #[doc = "Checks if the value of the field is `CPCLK_PRESCALE_8`"]
    #[inline(always)]
    pub fn is_cpclk_prescale_8(&self) -> bool {
        *self == CPCLK_PRESCALE_A::CPCLK_PRESCALE_8
    }
    #[doc = "Checks if the value of the field is `CPCLK_PRESCALE_9`"]
    #[inline(always)]
    pub fn is_cpclk_prescale_9(&self) -> bool {
        *self == CPCLK_PRESCALE_A::CPCLK_PRESCALE_9
    }
    #[doc = "Checks if the value of the field is `CPCLK_PRESCALE_10`"]
    #[inline(always)]
    pub fn is_cpclk_prescale_10(&self) -> bool {
        *self == CPCLK_PRESCALE_A::CPCLK_PRESCALE_10
    }
    #[doc = "Checks if the value of the field is `CPCLK_PRESCALE_63`"]
    #[inline(always)]
    pub fn is_cpclk_prescale_63(&self) -> bool {
        *self == CPCLK_PRESCALE_A::CPCLK_PRESCALE_63
    }
    #[doc = "Checks if the value of the field is `CPCLK_PRESCALE_64`"]
    #[inline(always)]
    pub fn is_cpclk_prescale_64(&self) -> bool {
        *self == CPCLK_PRESCALE_A::CPCLK_PRESCALE_64
    }
}
#[doc = "Write proxy for field `CPCLK_PRESCALE`"]
pub struct CPCLK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPCLK_PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPCLK_PRESCALE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn cpclk_prescale_1(self) -> &'a mut W {
        self.variant(CPCLK_PRESCALE_A::CPCLK_PRESCALE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn cpclk_prescale_2(self) -> &'a mut W {
        self.variant(CPCLK_PRESCALE_A::CPCLK_PRESCALE_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn cpclk_prescale_3(self) -> &'a mut W {
        self.variant(CPCLK_PRESCALE_A::CPCLK_PRESCALE_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn cpclk_prescale_4(self) -> &'a mut W {
        self.variant(CPCLK_PRESCALE_A::CPCLK_PRESCALE_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn cpclk_prescale_5(self) -> &'a mut W {
        self.variant(CPCLK_PRESCALE_A::CPCLK_PRESCALE_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn cpclk_prescale_6(self) -> &'a mut W {
        self.variant(CPCLK_PRESCALE_A::CPCLK_PRESCALE_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn cpclk_prescale_7(self) -> &'a mut W {
        self.variant(CPCLK_PRESCALE_A::CPCLK_PRESCALE_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn cpclk_prescale_8(self) -> &'a mut W {
        self.variant(CPCLK_PRESCALE_A::CPCLK_PRESCALE_8)
    }
    #[doc = "Divide by 9"]
    #[inline(always)]
    pub fn cpclk_prescale_9(self) -> &'a mut W {
        self.variant(CPCLK_PRESCALE_A::CPCLK_PRESCALE_9)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn cpclk_prescale_10(self) -> &'a mut W {
        self.variant(CPCLK_PRESCALE_A::CPCLK_PRESCALE_10)
    }
    #[doc = "Divide by 63"]
    #[inline(always)]
    pub fn cpclk_prescale_63(self) -> &'a mut W {
        self.variant(CPCLK_PRESCALE_A::CPCLK_PRESCALE_63)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn cpclk_prescale_64(self) -> &'a mut W {
        self.variant(CPCLK_PRESCALE_A::CPCLK_PRESCALE_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "DC-DC converter clock disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCCLK_DISABLE_A {
    #[doc = "0: DC-DC converter clock enabled"]
    DCCLK_ENABLE = 0,
    #[doc = "1: DC-DC converter clock disabled"]
    DCCLK_DISABLE = 1,
}
impl From<DCCLK_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DCCLK_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCCLK_DISABLE`"]
pub type DCCLK_DISABLE_R = crate::R<bool, DCCLK_DISABLE_A>;
impl DCCLK_DISABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCCLK_DISABLE_A {
        match self.bits {
            false => DCCLK_DISABLE_A::DCCLK_ENABLE,
            true => DCCLK_DISABLE_A::DCCLK_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DCCLK_ENABLE`"]
    #[inline(always)]
    pub fn is_dcclk_enable(&self) -> bool {
        *self == DCCLK_DISABLE_A::DCCLK_ENABLE
    }
    #[doc = "Checks if the value of the field is `DCCLK_DISABLE`"]
    #[inline(always)]
    pub fn is_dcclk_disable(&self) -> bool {
        *self == DCCLK_DISABLE_A::DCCLK_DISABLE
    }
}
#[doc = "Write proxy for field `DCCLK_DISABLE`"]
pub struct DCCLK_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCCLK_DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCCLK_DISABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DC-DC converter clock enabled"]
    #[inline(always)]
    pub fn dcclk_enable(self) -> &'a mut W {
        self.variant(DCCLK_DISABLE_A::DCCLK_ENABLE)
    }
    #[doc = "DC-DC converter clock disabled"]
    #[inline(always)]
    pub fn dcclk_disable(self) -> &'a mut W {
        self.variant(DCCLK_DISABLE_A::DCCLK_DISABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Prescale value for the DC-DC converter clock (1 to 64 in steps of 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DCCLK_PRESCALE_A {
    #[doc = "0: Divide by 1"]
    DCCLK_PRESCALE_1 = 0,
    #[doc = "1: Divide by 2"]
    DCCLK_PRESCALE_2 = 1,
    #[doc = "2: Divide by 3"]
    DCCLK_PRESCALE_3 = 2,
    #[doc = "3: Divide by 4"]
    DCCLK_PRESCALE_4 = 3,
    #[doc = "4: Divide by 5"]
    DCCLK_PRESCALE_5 = 4,
    #[doc = "5: Divide by 6"]
    DCCLK_PRESCALE_6 = 5,
    #[doc = "6: Divide by 7"]
    DCCLK_PRESCALE_7 = 6,
    #[doc = "7: Divide by 8"]
    DCCLK_PRESCALE_8 = 7,
    #[doc = "8: Divide by 9"]
    DCCLK_PRESCALE_9 = 8,
    #[doc = "9: Divide by 10"]
    DCCLK_PRESCALE_10 = 9,
    #[doc = "11: Divide by 12"]
    DCCLK_PRESCALE_12 = 11,
    #[doc = "62: Divide by 63"]
    DCCLK_PRESCALE_63 = 62,
    #[doc = "63: Divide by 64"]
    DCCLK_PRESCALE_64 = 63,
}
impl From<DCCLK_PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: DCCLK_PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DCCLK_PRESCALE`"]
pub type DCCLK_PRESCALE_R = crate::R<u8, DCCLK_PRESCALE_A>;
impl DCCLK_PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DCCLK_PRESCALE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DCCLK_PRESCALE_A::DCCLK_PRESCALE_1),
            1 => Val(DCCLK_PRESCALE_A::DCCLK_PRESCALE_2),
            2 => Val(DCCLK_PRESCALE_A::DCCLK_PRESCALE_3),
            3 => Val(DCCLK_PRESCALE_A::DCCLK_PRESCALE_4),
            4 => Val(DCCLK_PRESCALE_A::DCCLK_PRESCALE_5),
            5 => Val(DCCLK_PRESCALE_A::DCCLK_PRESCALE_6),
            6 => Val(DCCLK_PRESCALE_A::DCCLK_PRESCALE_7),
            7 => Val(DCCLK_PRESCALE_A::DCCLK_PRESCALE_8),
            8 => Val(DCCLK_PRESCALE_A::DCCLK_PRESCALE_9),
            9 => Val(DCCLK_PRESCALE_A::DCCLK_PRESCALE_10),
            11 => Val(DCCLK_PRESCALE_A::DCCLK_PRESCALE_12),
            62 => Val(DCCLK_PRESCALE_A::DCCLK_PRESCALE_63),
            63 => Val(DCCLK_PRESCALE_A::DCCLK_PRESCALE_64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DCCLK_PRESCALE_1`"]
    #[inline(always)]
    pub fn is_dcclk_prescale_1(&self) -> bool {
        *self == DCCLK_PRESCALE_A::DCCLK_PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `DCCLK_PRESCALE_2`"]
    #[inline(always)]
    pub fn is_dcclk_prescale_2(&self) -> bool {
        *self == DCCLK_PRESCALE_A::DCCLK_PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `DCCLK_PRESCALE_3`"]
    #[inline(always)]
    pub fn is_dcclk_prescale_3(&self) -> bool {
        *self == DCCLK_PRESCALE_A::DCCLK_PRESCALE_3
    }
    #[doc = "Checks if the value of the field is `DCCLK_PRESCALE_4`"]
    #[inline(always)]
    pub fn is_dcclk_prescale_4(&self) -> bool {
        *self == DCCLK_PRESCALE_A::DCCLK_PRESCALE_4
    }
    #[doc = "Checks if the value of the field is `DCCLK_PRESCALE_5`"]
    #[inline(always)]
    pub fn is_dcclk_prescale_5(&self) -> bool {
        *self == DCCLK_PRESCALE_A::DCCLK_PRESCALE_5
    }
    #[doc = "Checks if the value of the field is `DCCLK_PRESCALE_6`"]
    #[inline(always)]
    pub fn is_dcclk_prescale_6(&self) -> bool {
        *self == DCCLK_PRESCALE_A::DCCLK_PRESCALE_6
    }
    #[doc = "Checks if the value of the field is `DCCLK_PRESCALE_7`"]
    #[inline(always)]
    pub fn is_dcclk_prescale_7(&self) -> bool {
        *self == DCCLK_PRESCALE_A::DCCLK_PRESCALE_7
    }
    #[doc = "Checks if the value of the field is `DCCLK_PRESCALE_8`"]
    #[inline(always)]
    pub fn is_dcclk_prescale_8(&self) -> bool {
        *self == DCCLK_PRESCALE_A::DCCLK_PRESCALE_8
    }
    #[doc = "Checks if the value of the field is `DCCLK_PRESCALE_9`"]
    #[inline(always)]
    pub fn is_dcclk_prescale_9(&self) -> bool {
        *self == DCCLK_PRESCALE_A::DCCLK_PRESCALE_9
    }
    #[doc = "Checks if the value of the field is `DCCLK_PRESCALE_10`"]
    #[inline(always)]
    pub fn is_dcclk_prescale_10(&self) -> bool {
        *self == DCCLK_PRESCALE_A::DCCLK_PRESCALE_10
    }
    #[doc = "Checks if the value of the field is `DCCLK_PRESCALE_12`"]
    #[inline(always)]
    pub fn is_dcclk_prescale_12(&self) -> bool {
        *self == DCCLK_PRESCALE_A::DCCLK_PRESCALE_12
    }
    #[doc = "Checks if the value of the field is `DCCLK_PRESCALE_63`"]
    #[inline(always)]
    pub fn is_dcclk_prescale_63(&self) -> bool {
        *self == DCCLK_PRESCALE_A::DCCLK_PRESCALE_63
    }
    #[doc = "Checks if the value of the field is `DCCLK_PRESCALE_64`"]
    #[inline(always)]
    pub fn is_dcclk_prescale_64(&self) -> bool {
        *self == DCCLK_PRESCALE_A::DCCLK_PRESCALE_64
    }
}
#[doc = "Write proxy for field `DCCLK_PRESCALE`"]
pub struct DCCLK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCCLK_PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCCLK_PRESCALE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn dcclk_prescale_1(self) -> &'a mut W {
        self.variant(DCCLK_PRESCALE_A::DCCLK_PRESCALE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn dcclk_prescale_2(self) -> &'a mut W {
        self.variant(DCCLK_PRESCALE_A::DCCLK_PRESCALE_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn dcclk_prescale_3(self) -> &'a mut W {
        self.variant(DCCLK_PRESCALE_A::DCCLK_PRESCALE_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn dcclk_prescale_4(self) -> &'a mut W {
        self.variant(DCCLK_PRESCALE_A::DCCLK_PRESCALE_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn dcclk_prescale_5(self) -> &'a mut W {
        self.variant(DCCLK_PRESCALE_A::DCCLK_PRESCALE_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn dcclk_prescale_6(self) -> &'a mut W {
        self.variant(DCCLK_PRESCALE_A::DCCLK_PRESCALE_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn dcclk_prescale_7(self) -> &'a mut W {
        self.variant(DCCLK_PRESCALE_A::DCCLK_PRESCALE_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn dcclk_prescale_8(self) -> &'a mut W {
        self.variant(DCCLK_PRESCALE_A::DCCLK_PRESCALE_8)
    }
    #[doc = "Divide by 9"]
    #[inline(always)]
    pub fn dcclk_prescale_9(self) -> &'a mut W {
        self.variant(DCCLK_PRESCALE_A::DCCLK_PRESCALE_9)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn dcclk_prescale_10(self) -> &'a mut W {
        self.variant(DCCLK_PRESCALE_A::DCCLK_PRESCALE_10)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn dcclk_prescale_12(self) -> &'a mut W {
        self.variant(DCCLK_PRESCALE_A::DCCLK_PRESCALE_12)
    }
    #[doc = "Divide by 63"]
    #[inline(always)]
    pub fn dcclk_prescale_63(self) -> &'a mut W {
        self.variant(DCCLK_PRESCALE_A::DCCLK_PRESCALE_63)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn dcclk_prescale_64(self) -> &'a mut W {
        self.variant(DCCLK_PRESCALE_A::DCCLK_PRESCALE_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Charge pump clock disable"]
    #[inline(always)]
    pub fn cpclk_disable(&self) -> CPCLK_DISABLE_R {
        CPCLK_DISABLE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Prescale value for the charge pump clock from the SLOWCLK clock (1 to 64 in steps of 1)"]
    #[inline(always)]
    pub fn cpclk_prescale(&self) -> CPCLK_PRESCALE_R {
        CPCLK_PRESCALE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - DC-DC converter clock disable"]
    #[inline(always)]
    pub fn dcclk_disable(&self) -> DCCLK_DISABLE_R {
        DCCLK_DISABLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - Prescale value for the DC-DC converter clock (1 to 64 in steps of 1)"]
    #[inline(always)]
    pub fn dcclk_prescale(&self) -> DCCLK_PRESCALE_R {
        DCCLK_PRESCALE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - Charge pump clock disable"]
    #[inline(always)]
    pub fn cpclk_disable(&mut self) -> CPCLK_DISABLE_W {
        CPCLK_DISABLE_W { w: self }
    }
    #[doc = "Bits 8:13 - Prescale value for the charge pump clock from the SLOWCLK clock (1 to 64 in steps of 1)"]
    #[inline(always)]
    pub fn cpclk_prescale(&mut self) -> CPCLK_PRESCALE_W {
        CPCLK_PRESCALE_W { w: self }
    }
    #[doc = "Bit 7 - DC-DC converter clock disable"]
    #[inline(always)]
    pub fn dcclk_disable(&mut self) -> DCCLK_DISABLE_W {
        DCCLK_DISABLE_W { w: self }
    }
    #[doc = "Bits 0:5 - Prescale value for the DC-DC converter clock (1 to 64 in steps of 1)"]
    #[inline(always)]
    pub fn dcclk_prescale(&mut self) -> DCCLK_PRESCALE_W {
        DCCLK_PRESCALE_W { w: self }
    }
}
