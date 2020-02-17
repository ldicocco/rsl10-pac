#[doc = "Reader of register DIO_RF_GPIO47_SRC"]
pub type R = crate::R<u32, super::DIO_RF_GPIO47_SRC>;
#[doc = "Writer for register DIO_RF_GPIO47_SRC"]
pub type W = crate::W<u32, super::DIO_RF_GPIO47_SRC>;
#[doc = "Register DIO_RF_GPIO47_SRC `reset()`'s with value 0x1010_1012"]
impl crate::ResetValue for super::DIO_RF_GPIO47_SRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1010_1012
    }
}
#[doc = "RF front-end GPIO7 input selection\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO7_A {
    #[doc = "0: Select DIO\\[0\\]
as source"]
    RF_GPIO7_SRC_DIO_0 = 0,
    #[doc = "1: Select DIO\\[1\\]
as source"]
    RF_GPIO7_SRC_DIO_1 = 1,
    #[doc = "2: Select DIO\\[5\\]
as source"]
    RF_GPIO7_SRC_DIO_2 = 2,
    #[doc = "3: Select DIO\\[3\\]
as source"]
    RF_GPIO7_SRC_DIO_3 = 3,
    #[doc = "4: Select DIO\\[4\\]
as source"]
    RF_GPIO7_SRC_DIO_4 = 4,
    #[doc = "5: Select DIO\\[5\\]
as source"]
    RF_GPIO7_SRC_DIO_5 = 5,
    #[doc = "6: Select DIO\\[6\\]
as source"]
    RF_GPIO7_SRC_DIO_6 = 6,
    #[doc = "7: Select DIO\\[7\\]
as source"]
    RF_GPIO7_SRC_DIO_7 = 7,
    #[doc = "8: Select DIO\\[8\\]
as source"]
    RF_GPIO7_SRC_DIO_8 = 8,
    #[doc = "9: Select DIO\\[9\\]
as source"]
    RF_GPIO7_SRC_DIO_9 = 9,
    #[doc = "10: Select DIO\\[10\\]
as source"]
    RF_GPIO7_SRC_DIO_10 = 10,
    #[doc = "11: Select DIO\\[11\\]
as source"]
    RF_GPIO7_SRC_DIO_11 = 11,
    #[doc = "12: Select DIO\\[15\\]
as source"]
    RF_GPIO7_SRC_DIO_12 = 12,
    #[doc = "13: Select DIO\\[13\\]
as source"]
    RF_GPIO7_SRC_DIO_13 = 13,
    #[doc = "14: Select DIO\\[14\\]
as source"]
    RF_GPIO7_SRC_DIO_14 = 14,
    #[doc = "15: Select DIO\\[15\\]
as source"]
    RF_GPIO7_SRC_DIO_15 = 15,
    #[doc = "16: Select constant low as source"]
    RF_GPIO7_SRC_CONST_LOW = 16,
    #[doc = "17: Select constant high as source"]
    RF_GPIO7_SRC_CONST_HIGH = 17,
}
impl From<GPIO7_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO7`"]
pub type GPIO7_R = crate::R<u8, GPIO7_A>;
impl GPIO7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GPIO7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO7_A::RF_GPIO7_SRC_DIO_0),
            1 => Val(GPIO7_A::RF_GPIO7_SRC_DIO_1),
            2 => Val(GPIO7_A::RF_GPIO7_SRC_DIO_2),
            3 => Val(GPIO7_A::RF_GPIO7_SRC_DIO_3),
            4 => Val(GPIO7_A::RF_GPIO7_SRC_DIO_4),
            5 => Val(GPIO7_A::RF_GPIO7_SRC_DIO_5),
            6 => Val(GPIO7_A::RF_GPIO7_SRC_DIO_6),
            7 => Val(GPIO7_A::RF_GPIO7_SRC_DIO_7),
            8 => Val(GPIO7_A::RF_GPIO7_SRC_DIO_8),
            9 => Val(GPIO7_A::RF_GPIO7_SRC_DIO_9),
            10 => Val(GPIO7_A::RF_GPIO7_SRC_DIO_10),
            11 => Val(GPIO7_A::RF_GPIO7_SRC_DIO_11),
            12 => Val(GPIO7_A::RF_GPIO7_SRC_DIO_12),
            13 => Val(GPIO7_A::RF_GPIO7_SRC_DIO_13),
            14 => Val(GPIO7_A::RF_GPIO7_SRC_DIO_14),
            15 => Val(GPIO7_A::RF_GPIO7_SRC_DIO_15),
            16 => Val(GPIO7_A::RF_GPIO7_SRC_CONST_LOW),
            17 => Val(GPIO7_A::RF_GPIO7_SRC_CONST_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RF_GPIO7_SRC_DIO_0`"]
    #[inline(always)]
    pub fn is_rf_gpio7_src_dio_0(&self) -> bool {
        *self == GPIO7_A::RF_GPIO7_SRC_DIO_0
    }
    #[doc = "Checks if the value of the field is `RF_GPIO7_SRC_DIO_1`"]
    #[inline(always)]
    pub fn is_rf_gpio7_src_dio_1(&self) -> bool {
        *self == GPIO7_A::RF_GPIO7_SRC_DIO_1
    }
    #[doc = "Checks if the value of the field is `RF_GPIO7_SRC_DIO_2`"]
    #[inline(always)]
    pub fn is_rf_gpio7_src_dio_2(&self) -> bool {
        *self == GPIO7_A::RF_GPIO7_SRC_DIO_2
    }
    #[doc = "Checks if the value of the field is `RF_GPIO7_SRC_DIO_3`"]
    #[inline(always)]
    pub fn is_rf_gpio7_src_dio_3(&self) -> bool {
        *self == GPIO7_A::RF_GPIO7_SRC_DIO_3
    }
    #[doc = "Checks if the value of the field is `RF_GPIO7_SRC_DIO_4`"]
    #[inline(always)]
    pub fn is_rf_gpio7_src_dio_4(&self) -> bool {
        *self == GPIO7_A::RF_GPIO7_SRC_DIO_4
    }
    #[doc = "Checks if the value of the field is `RF_GPIO7_SRC_DIO_5`"]
    #[inline(always)]
    pub fn is_rf_gpio7_src_dio_5(&self) -> bool {
        *self == GPIO7_A::RF_GPIO7_SRC_DIO_5
    }
    #[doc = "Checks if the value of the field is `RF_GPIO7_SRC_DIO_6`"]
    #[inline(always)]
    pub fn is_rf_gpio7_src_dio_6(&self) -> bool {
        *self == GPIO7_A::RF_GPIO7_SRC_DIO_6
    }
    #[doc = "Checks if the value of the field is `RF_GPIO7_SRC_DIO_7`"]
    #[inline(always)]
    pub fn is_rf_gpio7_src_dio_7(&self) -> bool {
        *self == GPIO7_A::RF_GPIO7_SRC_DIO_7
    }
    #[doc = "Checks if the value of the field is `RF_GPIO7_SRC_DIO_8`"]
    #[inline(always)]
    pub fn is_rf_gpio7_src_dio_8(&self) -> bool {
        *self == GPIO7_A::RF_GPIO7_SRC_DIO_8
    }
    #[doc = "Checks if the value of the field is `RF_GPIO7_SRC_DIO_9`"]
    #[inline(always)]
    pub fn is_rf_gpio7_src_dio_9(&self) -> bool {
        *self == GPIO7_A::RF_GPIO7_SRC_DIO_9
    }
    #[doc = "Checks if the value of the field is `RF_GPIO7_SRC_DIO_10`"]
    #[inline(always)]
    pub fn is_rf_gpio7_src_dio_10(&self) -> bool {
        *self == GPIO7_A::RF_GPIO7_SRC_DIO_10
    }
    #[doc = "Checks if the value of the field is `RF_GPIO7_SRC_DIO_11`"]
    #[inline(always)]
    pub fn is_rf_gpio7_src_dio_11(&self) -> bool {
        *self == GPIO7_A::RF_GPIO7_SRC_DIO_11
    }
    #[doc = "Checks if the value of the field is `RF_GPIO7_SRC_DIO_12`"]
    #[inline(always)]
    pub fn is_rf_gpio7_src_dio_12(&self) -> bool {
        *self == GPIO7_A::RF_GPIO7_SRC_DIO_12
    }
    #[doc = "Checks if the value of the field is `RF_GPIO7_SRC_DIO_13`"]
    #[inline(always)]
    pub fn is_rf_gpio7_src_dio_13(&self) -> bool {
        *self == GPIO7_A::RF_GPIO7_SRC_DIO_13
    }
    #[doc = "Checks if the value of the field is `RF_GPIO7_SRC_DIO_14`"]
    #[inline(always)]
    pub fn is_rf_gpio7_src_dio_14(&self) -> bool {
        *self == GPIO7_A::RF_GPIO7_SRC_DIO_14
    }
    #[doc = "Checks if the value of the field is `RF_GPIO7_SRC_DIO_15`"]
    #[inline(always)]
    pub fn is_rf_gpio7_src_dio_15(&self) -> bool {
        *self == GPIO7_A::RF_GPIO7_SRC_DIO_15
    }
    #[doc = "Checks if the value of the field is `RF_GPIO7_SRC_CONST_LOW`"]
    #[inline(always)]
    pub fn is_rf_gpio7_src_const_low(&self) -> bool {
        *self == GPIO7_A::RF_GPIO7_SRC_CONST_LOW
    }
    #[doc = "Checks if the value of the field is `RF_GPIO7_SRC_CONST_HIGH`"]
    #[inline(always)]
    pub fn is_rf_gpio7_src_const_high(&self) -> bool {
        *self == GPIO7_A::RF_GPIO7_SRC_CONST_HIGH
    }
}
#[doc = "Write proxy for field `GPIO7`"]
pub struct GPIO7_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio7_src_dio_0(self) -> &'a mut W {
        self.variant(GPIO7_A::RF_GPIO7_SRC_DIO_0)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio7_src_dio_1(self) -> &'a mut W {
        self.variant(GPIO7_A::RF_GPIO7_SRC_DIO_1)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio7_src_dio_2(self) -> &'a mut W {
        self.variant(GPIO7_A::RF_GPIO7_SRC_DIO_2)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio7_src_dio_3(self) -> &'a mut W {
        self.variant(GPIO7_A::RF_GPIO7_SRC_DIO_3)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio7_src_dio_4(self) -> &'a mut W {
        self.variant(GPIO7_A::RF_GPIO7_SRC_DIO_4)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio7_src_dio_5(self) -> &'a mut W {
        self.variant(GPIO7_A::RF_GPIO7_SRC_DIO_5)
    }
    #[doc = "Select DIO\\[6\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio7_src_dio_6(self) -> &'a mut W {
        self.variant(GPIO7_A::RF_GPIO7_SRC_DIO_6)
    }
    #[doc = "Select DIO\\[7\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio7_src_dio_7(self) -> &'a mut W {
        self.variant(GPIO7_A::RF_GPIO7_SRC_DIO_7)
    }
    #[doc = "Select DIO\\[8\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio7_src_dio_8(self) -> &'a mut W {
        self.variant(GPIO7_A::RF_GPIO7_SRC_DIO_8)
    }
    #[doc = "Select DIO\\[9\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio7_src_dio_9(self) -> &'a mut W {
        self.variant(GPIO7_A::RF_GPIO7_SRC_DIO_9)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio7_src_dio_10(self) -> &'a mut W {
        self.variant(GPIO7_A::RF_GPIO7_SRC_DIO_10)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio7_src_dio_11(self) -> &'a mut W {
        self.variant(GPIO7_A::RF_GPIO7_SRC_DIO_11)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio7_src_dio_12(self) -> &'a mut W {
        self.variant(GPIO7_A::RF_GPIO7_SRC_DIO_12)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio7_src_dio_13(self) -> &'a mut W {
        self.variant(GPIO7_A::RF_GPIO7_SRC_DIO_13)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio7_src_dio_14(self) -> &'a mut W {
        self.variant(GPIO7_A::RF_GPIO7_SRC_DIO_14)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio7_src_dio_15(self) -> &'a mut W {
        self.variant(GPIO7_A::RF_GPIO7_SRC_DIO_15)
    }
    #[doc = "Select constant low as source"]
    #[inline(always)]
    pub fn rf_gpio7_src_const_low(self) -> &'a mut W {
        self.variant(GPIO7_A::RF_GPIO7_SRC_CONST_LOW)
    }
    #[doc = "Select constant high as source"]
    #[inline(always)]
    pub fn rf_gpio7_src_const_high(self) -> &'a mut W {
        self.variant(GPIO7_A::RF_GPIO7_SRC_CONST_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "RF front-end GPIO6 input selection\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO6_A {
    #[doc = "0: Select DIO\\[0\\]
as source"]
    RF_GPIO6_SRC_DIO_0 = 0,
    #[doc = "1: Select DIO\\[1\\]
as source"]
    RF_GPIO6_SRC_DIO_1 = 1,
    #[doc = "2: Select DIO\\[5\\]
as source"]
    RF_GPIO6_SRC_DIO_2 = 2,
    #[doc = "3: Select DIO\\[3\\]
as source"]
    RF_GPIO6_SRC_DIO_3 = 3,
    #[doc = "4: Select DIO\\[4\\]
as source"]
    RF_GPIO6_SRC_DIO_4 = 4,
    #[doc = "5: Select DIO\\[5\\]
as source"]
    RF_GPIO6_SRC_DIO_5 = 5,
    #[doc = "6: Select DIO\\[6\\]
as source"]
    RF_GPIO6_SRC_DIO_6 = 6,
    #[doc = "7: Select DIO\\[7\\]
as source"]
    RF_GPIO6_SRC_DIO_7 = 7,
    #[doc = "8: Select DIO\\[8\\]
as source"]
    RF_GPIO6_SRC_DIO_8 = 8,
    #[doc = "9: Select DIO\\[9\\]
as source"]
    RF_GPIO6_SRC_DIO_9 = 9,
    #[doc = "10: Select DIO\\[10\\]
as source"]
    RF_GPIO6_SRC_DIO_10 = 10,
    #[doc = "11: Select DIO\\[11\\]
as source"]
    RF_GPIO6_SRC_DIO_11 = 11,
    #[doc = "12: Select DIO\\[15\\]
as source"]
    RF_GPIO6_SRC_DIO_12 = 12,
    #[doc = "13: Select DIO\\[13\\]
as source"]
    RF_GPIO6_SRC_DIO_13 = 13,
    #[doc = "14: Select DIO\\[14\\]
as source"]
    RF_GPIO6_SRC_DIO_14 = 14,
    #[doc = "15: Select DIO\\[15\\]
as source"]
    RF_GPIO6_SRC_DIO_15 = 15,
    #[doc = "16: Select constant low as source"]
    RF_GPIO6_SRC_CONST_LOW = 16,
    #[doc = "17: Select constant high as source"]
    RF_GPIO6_SRC_CONST_HIGH = 17,
}
impl From<GPIO6_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO6`"]
pub type GPIO6_R = crate::R<u8, GPIO6_A>;
impl GPIO6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GPIO6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO6_A::RF_GPIO6_SRC_DIO_0),
            1 => Val(GPIO6_A::RF_GPIO6_SRC_DIO_1),
            2 => Val(GPIO6_A::RF_GPIO6_SRC_DIO_2),
            3 => Val(GPIO6_A::RF_GPIO6_SRC_DIO_3),
            4 => Val(GPIO6_A::RF_GPIO6_SRC_DIO_4),
            5 => Val(GPIO6_A::RF_GPIO6_SRC_DIO_5),
            6 => Val(GPIO6_A::RF_GPIO6_SRC_DIO_6),
            7 => Val(GPIO6_A::RF_GPIO6_SRC_DIO_7),
            8 => Val(GPIO6_A::RF_GPIO6_SRC_DIO_8),
            9 => Val(GPIO6_A::RF_GPIO6_SRC_DIO_9),
            10 => Val(GPIO6_A::RF_GPIO6_SRC_DIO_10),
            11 => Val(GPIO6_A::RF_GPIO6_SRC_DIO_11),
            12 => Val(GPIO6_A::RF_GPIO6_SRC_DIO_12),
            13 => Val(GPIO6_A::RF_GPIO6_SRC_DIO_13),
            14 => Val(GPIO6_A::RF_GPIO6_SRC_DIO_14),
            15 => Val(GPIO6_A::RF_GPIO6_SRC_DIO_15),
            16 => Val(GPIO6_A::RF_GPIO6_SRC_CONST_LOW),
            17 => Val(GPIO6_A::RF_GPIO6_SRC_CONST_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RF_GPIO6_SRC_DIO_0`"]
    #[inline(always)]
    pub fn is_rf_gpio6_src_dio_0(&self) -> bool {
        *self == GPIO6_A::RF_GPIO6_SRC_DIO_0
    }
    #[doc = "Checks if the value of the field is `RF_GPIO6_SRC_DIO_1`"]
    #[inline(always)]
    pub fn is_rf_gpio6_src_dio_1(&self) -> bool {
        *self == GPIO6_A::RF_GPIO6_SRC_DIO_1
    }
    #[doc = "Checks if the value of the field is `RF_GPIO6_SRC_DIO_2`"]
    #[inline(always)]
    pub fn is_rf_gpio6_src_dio_2(&self) -> bool {
        *self == GPIO6_A::RF_GPIO6_SRC_DIO_2
    }
    #[doc = "Checks if the value of the field is `RF_GPIO6_SRC_DIO_3`"]
    #[inline(always)]
    pub fn is_rf_gpio6_src_dio_3(&self) -> bool {
        *self == GPIO6_A::RF_GPIO6_SRC_DIO_3
    }
    #[doc = "Checks if the value of the field is `RF_GPIO6_SRC_DIO_4`"]
    #[inline(always)]
    pub fn is_rf_gpio6_src_dio_4(&self) -> bool {
        *self == GPIO6_A::RF_GPIO6_SRC_DIO_4
    }
    #[doc = "Checks if the value of the field is `RF_GPIO6_SRC_DIO_5`"]
    #[inline(always)]
    pub fn is_rf_gpio6_src_dio_5(&self) -> bool {
        *self == GPIO6_A::RF_GPIO6_SRC_DIO_5
    }
    #[doc = "Checks if the value of the field is `RF_GPIO6_SRC_DIO_6`"]
    #[inline(always)]
    pub fn is_rf_gpio6_src_dio_6(&self) -> bool {
        *self == GPIO6_A::RF_GPIO6_SRC_DIO_6
    }
    #[doc = "Checks if the value of the field is `RF_GPIO6_SRC_DIO_7`"]
    #[inline(always)]
    pub fn is_rf_gpio6_src_dio_7(&self) -> bool {
        *self == GPIO6_A::RF_GPIO6_SRC_DIO_7
    }
    #[doc = "Checks if the value of the field is `RF_GPIO6_SRC_DIO_8`"]
    #[inline(always)]
    pub fn is_rf_gpio6_src_dio_8(&self) -> bool {
        *self == GPIO6_A::RF_GPIO6_SRC_DIO_8
    }
    #[doc = "Checks if the value of the field is `RF_GPIO6_SRC_DIO_9`"]
    #[inline(always)]
    pub fn is_rf_gpio6_src_dio_9(&self) -> bool {
        *self == GPIO6_A::RF_GPIO6_SRC_DIO_9
    }
    #[doc = "Checks if the value of the field is `RF_GPIO6_SRC_DIO_10`"]
    #[inline(always)]
    pub fn is_rf_gpio6_src_dio_10(&self) -> bool {
        *self == GPIO6_A::RF_GPIO6_SRC_DIO_10
    }
    #[doc = "Checks if the value of the field is `RF_GPIO6_SRC_DIO_11`"]
    #[inline(always)]
    pub fn is_rf_gpio6_src_dio_11(&self) -> bool {
        *self == GPIO6_A::RF_GPIO6_SRC_DIO_11
    }
    #[doc = "Checks if the value of the field is `RF_GPIO6_SRC_DIO_12`"]
    #[inline(always)]
    pub fn is_rf_gpio6_src_dio_12(&self) -> bool {
        *self == GPIO6_A::RF_GPIO6_SRC_DIO_12
    }
    #[doc = "Checks if the value of the field is `RF_GPIO6_SRC_DIO_13`"]
    #[inline(always)]
    pub fn is_rf_gpio6_src_dio_13(&self) -> bool {
        *self == GPIO6_A::RF_GPIO6_SRC_DIO_13
    }
    #[doc = "Checks if the value of the field is `RF_GPIO6_SRC_DIO_14`"]
    #[inline(always)]
    pub fn is_rf_gpio6_src_dio_14(&self) -> bool {
        *self == GPIO6_A::RF_GPIO6_SRC_DIO_14
    }
    #[doc = "Checks if the value of the field is `RF_GPIO6_SRC_DIO_15`"]
    #[inline(always)]
    pub fn is_rf_gpio6_src_dio_15(&self) -> bool {
        *self == GPIO6_A::RF_GPIO6_SRC_DIO_15
    }
    #[doc = "Checks if the value of the field is `RF_GPIO6_SRC_CONST_LOW`"]
    #[inline(always)]
    pub fn is_rf_gpio6_src_const_low(&self) -> bool {
        *self == GPIO6_A::RF_GPIO6_SRC_CONST_LOW
    }
    #[doc = "Checks if the value of the field is `RF_GPIO6_SRC_CONST_HIGH`"]
    #[inline(always)]
    pub fn is_rf_gpio6_src_const_high(&self) -> bool {
        *self == GPIO6_A::RF_GPIO6_SRC_CONST_HIGH
    }
}
#[doc = "Write proxy for field `GPIO6`"]
pub struct GPIO6_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio6_src_dio_0(self) -> &'a mut W {
        self.variant(GPIO6_A::RF_GPIO6_SRC_DIO_0)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio6_src_dio_1(self) -> &'a mut W {
        self.variant(GPIO6_A::RF_GPIO6_SRC_DIO_1)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio6_src_dio_2(self) -> &'a mut W {
        self.variant(GPIO6_A::RF_GPIO6_SRC_DIO_2)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio6_src_dio_3(self) -> &'a mut W {
        self.variant(GPIO6_A::RF_GPIO6_SRC_DIO_3)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio6_src_dio_4(self) -> &'a mut W {
        self.variant(GPIO6_A::RF_GPIO6_SRC_DIO_4)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio6_src_dio_5(self) -> &'a mut W {
        self.variant(GPIO6_A::RF_GPIO6_SRC_DIO_5)
    }
    #[doc = "Select DIO\\[6\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio6_src_dio_6(self) -> &'a mut W {
        self.variant(GPIO6_A::RF_GPIO6_SRC_DIO_6)
    }
    #[doc = "Select DIO\\[7\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio6_src_dio_7(self) -> &'a mut W {
        self.variant(GPIO6_A::RF_GPIO6_SRC_DIO_7)
    }
    #[doc = "Select DIO\\[8\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio6_src_dio_8(self) -> &'a mut W {
        self.variant(GPIO6_A::RF_GPIO6_SRC_DIO_8)
    }
    #[doc = "Select DIO\\[9\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio6_src_dio_9(self) -> &'a mut W {
        self.variant(GPIO6_A::RF_GPIO6_SRC_DIO_9)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio6_src_dio_10(self) -> &'a mut W {
        self.variant(GPIO6_A::RF_GPIO6_SRC_DIO_10)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio6_src_dio_11(self) -> &'a mut W {
        self.variant(GPIO6_A::RF_GPIO6_SRC_DIO_11)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio6_src_dio_12(self) -> &'a mut W {
        self.variant(GPIO6_A::RF_GPIO6_SRC_DIO_12)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio6_src_dio_13(self) -> &'a mut W {
        self.variant(GPIO6_A::RF_GPIO6_SRC_DIO_13)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio6_src_dio_14(self) -> &'a mut W {
        self.variant(GPIO6_A::RF_GPIO6_SRC_DIO_14)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio6_src_dio_15(self) -> &'a mut W {
        self.variant(GPIO6_A::RF_GPIO6_SRC_DIO_15)
    }
    #[doc = "Select constant low as source"]
    #[inline(always)]
    pub fn rf_gpio6_src_const_low(self) -> &'a mut W {
        self.variant(GPIO6_A::RF_GPIO6_SRC_CONST_LOW)
    }
    #[doc = "Select constant high as source"]
    #[inline(always)]
    pub fn rf_gpio6_src_const_high(self) -> &'a mut W {
        self.variant(GPIO6_A::RF_GPIO6_SRC_CONST_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "RF front-end GPIO5 input selection\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO5_A {
    #[doc = "0: Select DIO\\[0\\]
as source"]
    RF_GPIO5_SRC_DIO_0 = 0,
    #[doc = "1: Select DIO\\[1\\]
as source"]
    RF_GPIO5_SRC_DIO_1 = 1,
    #[doc = "2: Select DIO\\[5\\]
as source"]
    RF_GPIO5_SRC_DIO_2 = 2,
    #[doc = "3: Select DIO\\[3\\]
as source"]
    RF_GPIO5_SRC_DIO_3 = 3,
    #[doc = "4: Select DIO\\[4\\]
as source"]
    RF_GPIO5_SRC_DIO_4 = 4,
    #[doc = "5: Select DIO\\[5\\]
as source"]
    RF_GPIO5_SRC_DIO_5 = 5,
    #[doc = "6: Select DIO\\[6\\]
as source"]
    RF_GPIO5_SRC_DIO_6 = 6,
    #[doc = "7: Select DIO\\[7\\]
as source"]
    RF_GPIO5_SRC_DIO_7 = 7,
    #[doc = "8: Select DIO\\[8\\]
as source"]
    RF_GPIO5_SRC_DIO_8 = 8,
    #[doc = "9: Select DIO\\[9\\]
as source"]
    RF_GPIO5_SRC_DIO_9 = 9,
    #[doc = "10: Select DIO\\[10\\]
as source"]
    RF_GPIO5_SRC_DIO_10 = 10,
    #[doc = "11: Select DIO\\[11\\]
as source"]
    RF_GPIO5_SRC_DIO_11 = 11,
    #[doc = "12: Select DIO\\[15\\]
as source"]
    RF_GPIO5_SRC_DIO_12 = 12,
    #[doc = "13: Select DIO\\[13\\]
as source"]
    RF_GPIO5_SRC_DIO_13 = 13,
    #[doc = "14: Select DIO\\[14\\]
as source"]
    RF_GPIO5_SRC_DIO_14 = 14,
    #[doc = "15: Select DIO\\[15\\]
as source"]
    RF_GPIO5_SRC_DIO_15 = 15,
    #[doc = "16: Select constant low as source"]
    RF_GPIO5_SRC_CONST_LOW = 16,
    #[doc = "17: Select constant high as source"]
    RF_GPIO5_SRC_CONST_HIGH = 17,
}
impl From<GPIO5_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO5`"]
pub type GPIO5_R = crate::R<u8, GPIO5_A>;
impl GPIO5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GPIO5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO5_A::RF_GPIO5_SRC_DIO_0),
            1 => Val(GPIO5_A::RF_GPIO5_SRC_DIO_1),
            2 => Val(GPIO5_A::RF_GPIO5_SRC_DIO_2),
            3 => Val(GPIO5_A::RF_GPIO5_SRC_DIO_3),
            4 => Val(GPIO5_A::RF_GPIO5_SRC_DIO_4),
            5 => Val(GPIO5_A::RF_GPIO5_SRC_DIO_5),
            6 => Val(GPIO5_A::RF_GPIO5_SRC_DIO_6),
            7 => Val(GPIO5_A::RF_GPIO5_SRC_DIO_7),
            8 => Val(GPIO5_A::RF_GPIO5_SRC_DIO_8),
            9 => Val(GPIO5_A::RF_GPIO5_SRC_DIO_9),
            10 => Val(GPIO5_A::RF_GPIO5_SRC_DIO_10),
            11 => Val(GPIO5_A::RF_GPIO5_SRC_DIO_11),
            12 => Val(GPIO5_A::RF_GPIO5_SRC_DIO_12),
            13 => Val(GPIO5_A::RF_GPIO5_SRC_DIO_13),
            14 => Val(GPIO5_A::RF_GPIO5_SRC_DIO_14),
            15 => Val(GPIO5_A::RF_GPIO5_SRC_DIO_15),
            16 => Val(GPIO5_A::RF_GPIO5_SRC_CONST_LOW),
            17 => Val(GPIO5_A::RF_GPIO5_SRC_CONST_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RF_GPIO5_SRC_DIO_0`"]
    #[inline(always)]
    pub fn is_rf_gpio5_src_dio_0(&self) -> bool {
        *self == GPIO5_A::RF_GPIO5_SRC_DIO_0
    }
    #[doc = "Checks if the value of the field is `RF_GPIO5_SRC_DIO_1`"]
    #[inline(always)]
    pub fn is_rf_gpio5_src_dio_1(&self) -> bool {
        *self == GPIO5_A::RF_GPIO5_SRC_DIO_1
    }
    #[doc = "Checks if the value of the field is `RF_GPIO5_SRC_DIO_2`"]
    #[inline(always)]
    pub fn is_rf_gpio5_src_dio_2(&self) -> bool {
        *self == GPIO5_A::RF_GPIO5_SRC_DIO_2
    }
    #[doc = "Checks if the value of the field is `RF_GPIO5_SRC_DIO_3`"]
    #[inline(always)]
    pub fn is_rf_gpio5_src_dio_3(&self) -> bool {
        *self == GPIO5_A::RF_GPIO5_SRC_DIO_3
    }
    #[doc = "Checks if the value of the field is `RF_GPIO5_SRC_DIO_4`"]
    #[inline(always)]
    pub fn is_rf_gpio5_src_dio_4(&self) -> bool {
        *self == GPIO5_A::RF_GPIO5_SRC_DIO_4
    }
    #[doc = "Checks if the value of the field is `RF_GPIO5_SRC_DIO_5`"]
    #[inline(always)]
    pub fn is_rf_gpio5_src_dio_5(&self) -> bool {
        *self == GPIO5_A::RF_GPIO5_SRC_DIO_5
    }
    #[doc = "Checks if the value of the field is `RF_GPIO5_SRC_DIO_6`"]
    #[inline(always)]
    pub fn is_rf_gpio5_src_dio_6(&self) -> bool {
        *self == GPIO5_A::RF_GPIO5_SRC_DIO_6
    }
    #[doc = "Checks if the value of the field is `RF_GPIO5_SRC_DIO_7`"]
    #[inline(always)]
    pub fn is_rf_gpio5_src_dio_7(&self) -> bool {
        *self == GPIO5_A::RF_GPIO5_SRC_DIO_7
    }
    #[doc = "Checks if the value of the field is `RF_GPIO5_SRC_DIO_8`"]
    #[inline(always)]
    pub fn is_rf_gpio5_src_dio_8(&self) -> bool {
        *self == GPIO5_A::RF_GPIO5_SRC_DIO_8
    }
    #[doc = "Checks if the value of the field is `RF_GPIO5_SRC_DIO_9`"]
    #[inline(always)]
    pub fn is_rf_gpio5_src_dio_9(&self) -> bool {
        *self == GPIO5_A::RF_GPIO5_SRC_DIO_9
    }
    #[doc = "Checks if the value of the field is `RF_GPIO5_SRC_DIO_10`"]
    #[inline(always)]
    pub fn is_rf_gpio5_src_dio_10(&self) -> bool {
        *self == GPIO5_A::RF_GPIO5_SRC_DIO_10
    }
    #[doc = "Checks if the value of the field is `RF_GPIO5_SRC_DIO_11`"]
    #[inline(always)]
    pub fn is_rf_gpio5_src_dio_11(&self) -> bool {
        *self == GPIO5_A::RF_GPIO5_SRC_DIO_11
    }
    #[doc = "Checks if the value of the field is `RF_GPIO5_SRC_DIO_12`"]
    #[inline(always)]
    pub fn is_rf_gpio5_src_dio_12(&self) -> bool {
        *self == GPIO5_A::RF_GPIO5_SRC_DIO_12
    }
    #[doc = "Checks if the value of the field is `RF_GPIO5_SRC_DIO_13`"]
    #[inline(always)]
    pub fn is_rf_gpio5_src_dio_13(&self) -> bool {
        *self == GPIO5_A::RF_GPIO5_SRC_DIO_13
    }
    #[doc = "Checks if the value of the field is `RF_GPIO5_SRC_DIO_14`"]
    #[inline(always)]
    pub fn is_rf_gpio5_src_dio_14(&self) -> bool {
        *self == GPIO5_A::RF_GPIO5_SRC_DIO_14
    }
    #[doc = "Checks if the value of the field is `RF_GPIO5_SRC_DIO_15`"]
    #[inline(always)]
    pub fn is_rf_gpio5_src_dio_15(&self) -> bool {
        *self == GPIO5_A::RF_GPIO5_SRC_DIO_15
    }
    #[doc = "Checks if the value of the field is `RF_GPIO5_SRC_CONST_LOW`"]
    #[inline(always)]
    pub fn is_rf_gpio5_src_const_low(&self) -> bool {
        *self == GPIO5_A::RF_GPIO5_SRC_CONST_LOW
    }
    #[doc = "Checks if the value of the field is `RF_GPIO5_SRC_CONST_HIGH`"]
    #[inline(always)]
    pub fn is_rf_gpio5_src_const_high(&self) -> bool {
        *self == GPIO5_A::RF_GPIO5_SRC_CONST_HIGH
    }
}
#[doc = "Write proxy for field `GPIO5`"]
pub struct GPIO5_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio5_src_dio_0(self) -> &'a mut W {
        self.variant(GPIO5_A::RF_GPIO5_SRC_DIO_0)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio5_src_dio_1(self) -> &'a mut W {
        self.variant(GPIO5_A::RF_GPIO5_SRC_DIO_1)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio5_src_dio_2(self) -> &'a mut W {
        self.variant(GPIO5_A::RF_GPIO5_SRC_DIO_2)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio5_src_dio_3(self) -> &'a mut W {
        self.variant(GPIO5_A::RF_GPIO5_SRC_DIO_3)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio5_src_dio_4(self) -> &'a mut W {
        self.variant(GPIO5_A::RF_GPIO5_SRC_DIO_4)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio5_src_dio_5(self) -> &'a mut W {
        self.variant(GPIO5_A::RF_GPIO5_SRC_DIO_5)
    }
    #[doc = "Select DIO\\[6\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio5_src_dio_6(self) -> &'a mut W {
        self.variant(GPIO5_A::RF_GPIO5_SRC_DIO_6)
    }
    #[doc = "Select DIO\\[7\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio5_src_dio_7(self) -> &'a mut W {
        self.variant(GPIO5_A::RF_GPIO5_SRC_DIO_7)
    }
    #[doc = "Select DIO\\[8\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio5_src_dio_8(self) -> &'a mut W {
        self.variant(GPIO5_A::RF_GPIO5_SRC_DIO_8)
    }
    #[doc = "Select DIO\\[9\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio5_src_dio_9(self) -> &'a mut W {
        self.variant(GPIO5_A::RF_GPIO5_SRC_DIO_9)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio5_src_dio_10(self) -> &'a mut W {
        self.variant(GPIO5_A::RF_GPIO5_SRC_DIO_10)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio5_src_dio_11(self) -> &'a mut W {
        self.variant(GPIO5_A::RF_GPIO5_SRC_DIO_11)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio5_src_dio_12(self) -> &'a mut W {
        self.variant(GPIO5_A::RF_GPIO5_SRC_DIO_12)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio5_src_dio_13(self) -> &'a mut W {
        self.variant(GPIO5_A::RF_GPIO5_SRC_DIO_13)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio5_src_dio_14(self) -> &'a mut W {
        self.variant(GPIO5_A::RF_GPIO5_SRC_DIO_14)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio5_src_dio_15(self) -> &'a mut W {
        self.variant(GPIO5_A::RF_GPIO5_SRC_DIO_15)
    }
    #[doc = "Select constant low as source"]
    #[inline(always)]
    pub fn rf_gpio5_src_const_low(self) -> &'a mut W {
        self.variant(GPIO5_A::RF_GPIO5_SRC_CONST_LOW)
    }
    #[doc = "Select constant high as source"]
    #[inline(always)]
    pub fn rf_gpio5_src_const_high(self) -> &'a mut W {
        self.variant(GPIO5_A::RF_GPIO5_SRC_CONST_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "RE front-end GPIO4 input selection\n\nValue on reset: 18"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO4_A {
    #[doc = "0: Select DIO\\[0\\]
as source"]
    RF_GPIO4_SRC_DIO_0 = 0,
    #[doc = "1: Select DIO\\[1\\]
as source"]
    RF_GPIO4_SRC_DIO_1 = 1,
    #[doc = "2: Select DIO\\[4\\]
as source"]
    RF_GPIO4_SRC_DIO_2 = 2,
    #[doc = "3: Select DIO\\[3\\]
as source"]
    RF_GPIO4_SRC_DIO_3 = 3,
    #[doc = "4: Select DIO\\[4\\]
as source"]
    RF_GPIO4_SRC_DIO_4 = 4,
    #[doc = "5: Select DIO\\[5\\]
as source"]
    RF_GPIO4_SRC_DIO_5 = 5,
    #[doc = "6: Select DIO\\[6\\]
as source"]
    RF_GPIO4_SRC_DIO_6 = 6,
    #[doc = "7: Select DIO\\[7\\]
as source"]
    RF_GPIO4_SRC_DIO_7 = 7,
    #[doc = "8: Select DIO\\[8\\]
as source"]
    RF_GPIO4_SRC_DIO_8 = 8,
    #[doc = "9: Select DIO\\[9\\]
as source"]
    RF_GPIO4_SRC_DIO_9 = 9,
    #[doc = "10: Select DIO\\[10\\]
as source"]
    RF_GPIO4_SRC_DIO_10 = 10,
    #[doc = "11: Select DIO\\[11\\]
as source"]
    RF_GPIO4_SRC_DIO_11 = 11,
    #[doc = "12: Select DIO\\[14\\]
as source"]
    RF_GPIO4_SRC_DIO_12 = 12,
    #[doc = "13: Select DIO\\[13\\]
as source"]
    RF_GPIO4_SRC_DIO_13 = 13,
    #[doc = "14: Select DIO\\[14\\]
as source"]
    RF_GPIO4_SRC_DIO_14 = 14,
    #[doc = "15: Select DIO\\[15\\]
as source"]
    RF_GPIO4_SRC_DIO_15 = 15,
    #[doc = "16: Select constant low as source"]
    RF_GPIO4_SRC_CONST_LOW = 16,
    #[doc = "17: Select constant high as source"]
    RF_GPIO4_SRC_CONST_HIGH = 17,
    #[doc = "18: Select baseband controller TX_DATA_VALID as source"]
    RF_GPIO4_SRC_BB_TX_DATA_VALID = 18,
}
impl From<GPIO4_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO4`"]
pub type GPIO4_R = crate::R<u8, GPIO4_A>;
impl GPIO4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GPIO4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO4_A::RF_GPIO4_SRC_DIO_0),
            1 => Val(GPIO4_A::RF_GPIO4_SRC_DIO_1),
            2 => Val(GPIO4_A::RF_GPIO4_SRC_DIO_2),
            3 => Val(GPIO4_A::RF_GPIO4_SRC_DIO_3),
            4 => Val(GPIO4_A::RF_GPIO4_SRC_DIO_4),
            5 => Val(GPIO4_A::RF_GPIO4_SRC_DIO_5),
            6 => Val(GPIO4_A::RF_GPIO4_SRC_DIO_6),
            7 => Val(GPIO4_A::RF_GPIO4_SRC_DIO_7),
            8 => Val(GPIO4_A::RF_GPIO4_SRC_DIO_8),
            9 => Val(GPIO4_A::RF_GPIO4_SRC_DIO_9),
            10 => Val(GPIO4_A::RF_GPIO4_SRC_DIO_10),
            11 => Val(GPIO4_A::RF_GPIO4_SRC_DIO_11),
            12 => Val(GPIO4_A::RF_GPIO4_SRC_DIO_12),
            13 => Val(GPIO4_A::RF_GPIO4_SRC_DIO_13),
            14 => Val(GPIO4_A::RF_GPIO4_SRC_DIO_14),
            15 => Val(GPIO4_A::RF_GPIO4_SRC_DIO_15),
            16 => Val(GPIO4_A::RF_GPIO4_SRC_CONST_LOW),
            17 => Val(GPIO4_A::RF_GPIO4_SRC_CONST_HIGH),
            18 => Val(GPIO4_A::RF_GPIO4_SRC_BB_TX_DATA_VALID),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_DIO_0`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_dio_0(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_DIO_0
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_DIO_1`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_dio_1(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_DIO_1
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_DIO_2`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_dio_2(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_DIO_2
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_DIO_3`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_dio_3(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_DIO_3
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_DIO_4`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_dio_4(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_DIO_4
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_DIO_5`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_dio_5(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_DIO_5
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_DIO_6`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_dio_6(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_DIO_6
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_DIO_7`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_dio_7(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_DIO_7
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_DIO_8`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_dio_8(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_DIO_8
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_DIO_9`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_dio_9(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_DIO_9
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_DIO_10`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_dio_10(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_DIO_10
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_DIO_11`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_dio_11(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_DIO_11
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_DIO_12`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_dio_12(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_DIO_12
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_DIO_13`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_dio_13(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_DIO_13
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_DIO_14`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_dio_14(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_DIO_14
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_DIO_15`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_dio_15(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_DIO_15
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_CONST_LOW`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_const_low(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_CONST_LOW
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_CONST_HIGH`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_const_high(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_CONST_HIGH
    }
    #[doc = "Checks if the value of the field is `RF_GPIO4_SRC_BB_TX_DATA_VALID`"]
    #[inline(always)]
    pub fn is_rf_gpio4_src_bb_tx_data_valid(&self) -> bool {
        *self == GPIO4_A::RF_GPIO4_SRC_BB_TX_DATA_VALID
    }
}
#[doc = "Write proxy for field `GPIO4`"]
pub struct GPIO4_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_dio_0(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_DIO_0)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_dio_1(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_DIO_1)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_dio_2(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_DIO_2)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_dio_3(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_DIO_3)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_dio_4(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_DIO_4)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_dio_5(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_DIO_5)
    }
    #[doc = "Select DIO\\[6\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_dio_6(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_DIO_6)
    }
    #[doc = "Select DIO\\[7\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_dio_7(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_DIO_7)
    }
    #[doc = "Select DIO\\[8\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_dio_8(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_DIO_8)
    }
    #[doc = "Select DIO\\[9\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_dio_9(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_DIO_9)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_dio_10(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_DIO_10)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_dio_11(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_DIO_11)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_dio_12(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_DIO_12)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_dio_13(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_DIO_13)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_dio_14(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_DIO_14)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_dio_15(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_DIO_15)
    }
    #[doc = "Select constant low as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_const_low(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_CONST_LOW)
    }
    #[doc = "Select constant high as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_const_high(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_CONST_HIGH)
    }
    #[doc = "Select baseband controller TX_DATA_VALID as source"]
    #[inline(always)]
    pub fn rf_gpio4_src_bb_tx_data_valid(self) -> &'a mut W {
        self.variant(GPIO4_A::RF_GPIO4_SRC_BB_TX_DATA_VALID)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:28 - RF front-end GPIO7 input selection"]
    #[inline(always)]
    pub fn gpio7(&self) -> GPIO7_R {
        GPIO7_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - RF front-end GPIO6 input selection"]
    #[inline(always)]
    pub fn gpio6(&self) -> GPIO6_R {
        GPIO6_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - RF front-end GPIO5 input selection"]
    #[inline(always)]
    pub fn gpio5(&self) -> GPIO5_R {
        GPIO5_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - RE front-end GPIO4 input selection"]
    #[inline(always)]
    pub fn gpio4(&self) -> GPIO4_R {
        GPIO4_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28 - RF front-end GPIO7 input selection"]
    #[inline(always)]
    pub fn gpio7(&mut self) -> GPIO7_W {
        GPIO7_W { w: self }
    }
    #[doc = "Bits 16:20 - RF front-end GPIO6 input selection"]
    #[inline(always)]
    pub fn gpio6(&mut self) -> GPIO6_W {
        GPIO6_W { w: self }
    }
    #[doc = "Bits 8:12 - RF front-end GPIO5 input selection"]
    #[inline(always)]
    pub fn gpio5(&mut self) -> GPIO5_W {
        GPIO5_W { w: self }
    }
    #[doc = "Bits 0:4 - RE front-end GPIO4 input selection"]
    #[inline(always)]
    pub fn gpio4(&mut self) -> GPIO4_W {
        GPIO4_W { w: self }
    }
}
