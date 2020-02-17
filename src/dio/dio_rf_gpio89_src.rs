#[doc = "Reader of register DIO_RF_GPIO89_SRC"]
pub type R = crate::R<u32, super::DIO_RF_GPIO89_SRC>;
#[doc = "Writer for register DIO_RF_GPIO89_SRC"]
pub type W = crate::W<u32, super::DIO_RF_GPIO89_SRC>;
#[doc = "Register DIO_RF_GPIO89_SRC `reset()`'s with value 0x1010"]
impl crate::ResetValue for super::DIO_RF_GPIO89_SRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1010
    }
}
#[doc = "RF front-end GPIO9 input selection\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO9_A {
    #[doc = "0: Select DIO\\[0\\]
as source"]
    RF_GPIO9_SRC_DIO_0 = 0,
    #[doc = "1: Select DIO\\[1\\]
as source"]
    RF_GPIO9_SRC_DIO_1 = 1,
    #[doc = "2: Select DIO\\[5\\]
as source"]
    RF_GPIO9_SRC_DIO_2 = 2,
    #[doc = "3: Select DIO\\[3\\]
as source"]
    RF_GPIO9_SRC_DIO_3 = 3,
    #[doc = "4: Select DIO\\[4\\]
as source"]
    RF_GPIO9_SRC_DIO_4 = 4,
    #[doc = "5: Select DIO\\[5\\]
as source"]
    RF_GPIO9_SRC_DIO_5 = 5,
    #[doc = "6: Select DIO\\[6\\]
as source"]
    RF_GPIO9_SRC_DIO_6 = 6,
    #[doc = "7: Select DIO\\[7\\]
as source"]
    RF_GPIO9_SRC_DIO_7 = 7,
    #[doc = "8: Select DIO\\[8\\]
as source"]
    RF_GPIO9_SRC_DIO_8 = 8,
    #[doc = "9: Select DIO\\[9\\]
as source"]
    RF_GPIO9_SRC_DIO_9 = 9,
    #[doc = "10: Select DIO\\[10\\]
as source"]
    RF_GPIO9_SRC_DIO_10 = 10,
    #[doc = "11: Select DIO\\[11\\]
as source"]
    RF_GPIO9_SRC_DIO_11 = 11,
    #[doc = "12: Select DIO\\[15\\]
as source"]
    RF_GPIO9_SRC_DIO_12 = 12,
    #[doc = "13: Select DIO\\[13\\]
as source"]
    RF_GPIO9_SRC_DIO_13 = 13,
    #[doc = "14: Select DIO\\[14\\]
as source"]
    RF_GPIO9_SRC_DIO_14 = 14,
    #[doc = "15: Select DIO\\[15\\]
as source"]
    RF_GPIO9_SRC_DIO_15 = 15,
    #[doc = "16: Select constant low as source"]
    RF_GPIO9_SRC_CONST_LOW = 16,
    #[doc = "17: Select constant high as source"]
    RF_GPIO9_SRC_CONST_HIGH = 17,
}
impl From<GPIO9_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO9_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO9`"]
pub type GPIO9_R = crate::R<u8, GPIO9_A>;
impl GPIO9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GPIO9_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO9_A::RF_GPIO9_SRC_DIO_0),
            1 => Val(GPIO9_A::RF_GPIO9_SRC_DIO_1),
            2 => Val(GPIO9_A::RF_GPIO9_SRC_DIO_2),
            3 => Val(GPIO9_A::RF_GPIO9_SRC_DIO_3),
            4 => Val(GPIO9_A::RF_GPIO9_SRC_DIO_4),
            5 => Val(GPIO9_A::RF_GPIO9_SRC_DIO_5),
            6 => Val(GPIO9_A::RF_GPIO9_SRC_DIO_6),
            7 => Val(GPIO9_A::RF_GPIO9_SRC_DIO_7),
            8 => Val(GPIO9_A::RF_GPIO9_SRC_DIO_8),
            9 => Val(GPIO9_A::RF_GPIO9_SRC_DIO_9),
            10 => Val(GPIO9_A::RF_GPIO9_SRC_DIO_10),
            11 => Val(GPIO9_A::RF_GPIO9_SRC_DIO_11),
            12 => Val(GPIO9_A::RF_GPIO9_SRC_DIO_12),
            13 => Val(GPIO9_A::RF_GPIO9_SRC_DIO_13),
            14 => Val(GPIO9_A::RF_GPIO9_SRC_DIO_14),
            15 => Val(GPIO9_A::RF_GPIO9_SRC_DIO_15),
            16 => Val(GPIO9_A::RF_GPIO9_SRC_CONST_LOW),
            17 => Val(GPIO9_A::RF_GPIO9_SRC_CONST_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RF_GPIO9_SRC_DIO_0`"]
    #[inline(always)]
    pub fn is_rf_gpio9_src_dio_0(&self) -> bool {
        *self == GPIO9_A::RF_GPIO9_SRC_DIO_0
    }
    #[doc = "Checks if the value of the field is `RF_GPIO9_SRC_DIO_1`"]
    #[inline(always)]
    pub fn is_rf_gpio9_src_dio_1(&self) -> bool {
        *self == GPIO9_A::RF_GPIO9_SRC_DIO_1
    }
    #[doc = "Checks if the value of the field is `RF_GPIO9_SRC_DIO_2`"]
    #[inline(always)]
    pub fn is_rf_gpio9_src_dio_2(&self) -> bool {
        *self == GPIO9_A::RF_GPIO9_SRC_DIO_2
    }
    #[doc = "Checks if the value of the field is `RF_GPIO9_SRC_DIO_3`"]
    #[inline(always)]
    pub fn is_rf_gpio9_src_dio_3(&self) -> bool {
        *self == GPIO9_A::RF_GPIO9_SRC_DIO_3
    }
    #[doc = "Checks if the value of the field is `RF_GPIO9_SRC_DIO_4`"]
    #[inline(always)]
    pub fn is_rf_gpio9_src_dio_4(&self) -> bool {
        *self == GPIO9_A::RF_GPIO9_SRC_DIO_4
    }
    #[doc = "Checks if the value of the field is `RF_GPIO9_SRC_DIO_5`"]
    #[inline(always)]
    pub fn is_rf_gpio9_src_dio_5(&self) -> bool {
        *self == GPIO9_A::RF_GPIO9_SRC_DIO_5
    }
    #[doc = "Checks if the value of the field is `RF_GPIO9_SRC_DIO_6`"]
    #[inline(always)]
    pub fn is_rf_gpio9_src_dio_6(&self) -> bool {
        *self == GPIO9_A::RF_GPIO9_SRC_DIO_6
    }
    #[doc = "Checks if the value of the field is `RF_GPIO9_SRC_DIO_7`"]
    #[inline(always)]
    pub fn is_rf_gpio9_src_dio_7(&self) -> bool {
        *self == GPIO9_A::RF_GPIO9_SRC_DIO_7
    }
    #[doc = "Checks if the value of the field is `RF_GPIO9_SRC_DIO_8`"]
    #[inline(always)]
    pub fn is_rf_gpio9_src_dio_8(&self) -> bool {
        *self == GPIO9_A::RF_GPIO9_SRC_DIO_8
    }
    #[doc = "Checks if the value of the field is `RF_GPIO9_SRC_DIO_9`"]
    #[inline(always)]
    pub fn is_rf_gpio9_src_dio_9(&self) -> bool {
        *self == GPIO9_A::RF_GPIO9_SRC_DIO_9
    }
    #[doc = "Checks if the value of the field is `RF_GPIO9_SRC_DIO_10`"]
    #[inline(always)]
    pub fn is_rf_gpio9_src_dio_10(&self) -> bool {
        *self == GPIO9_A::RF_GPIO9_SRC_DIO_10
    }
    #[doc = "Checks if the value of the field is `RF_GPIO9_SRC_DIO_11`"]
    #[inline(always)]
    pub fn is_rf_gpio9_src_dio_11(&self) -> bool {
        *self == GPIO9_A::RF_GPIO9_SRC_DIO_11
    }
    #[doc = "Checks if the value of the field is `RF_GPIO9_SRC_DIO_12`"]
    #[inline(always)]
    pub fn is_rf_gpio9_src_dio_12(&self) -> bool {
        *self == GPIO9_A::RF_GPIO9_SRC_DIO_12
    }
    #[doc = "Checks if the value of the field is `RF_GPIO9_SRC_DIO_13`"]
    #[inline(always)]
    pub fn is_rf_gpio9_src_dio_13(&self) -> bool {
        *self == GPIO9_A::RF_GPIO9_SRC_DIO_13
    }
    #[doc = "Checks if the value of the field is `RF_GPIO9_SRC_DIO_14`"]
    #[inline(always)]
    pub fn is_rf_gpio9_src_dio_14(&self) -> bool {
        *self == GPIO9_A::RF_GPIO9_SRC_DIO_14
    }
    #[doc = "Checks if the value of the field is `RF_GPIO9_SRC_DIO_15`"]
    #[inline(always)]
    pub fn is_rf_gpio9_src_dio_15(&self) -> bool {
        *self == GPIO9_A::RF_GPIO9_SRC_DIO_15
    }
    #[doc = "Checks if the value of the field is `RF_GPIO9_SRC_CONST_LOW`"]
    #[inline(always)]
    pub fn is_rf_gpio9_src_const_low(&self) -> bool {
        *self == GPIO9_A::RF_GPIO9_SRC_CONST_LOW
    }
    #[doc = "Checks if the value of the field is `RF_GPIO9_SRC_CONST_HIGH`"]
    #[inline(always)]
    pub fn is_rf_gpio9_src_const_high(&self) -> bool {
        *self == GPIO9_A::RF_GPIO9_SRC_CONST_HIGH
    }
}
#[doc = "Write proxy for field `GPIO9`"]
pub struct GPIO9_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio9_src_dio_0(self) -> &'a mut W {
        self.variant(GPIO9_A::RF_GPIO9_SRC_DIO_0)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio9_src_dio_1(self) -> &'a mut W {
        self.variant(GPIO9_A::RF_GPIO9_SRC_DIO_1)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio9_src_dio_2(self) -> &'a mut W {
        self.variant(GPIO9_A::RF_GPIO9_SRC_DIO_2)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio9_src_dio_3(self) -> &'a mut W {
        self.variant(GPIO9_A::RF_GPIO9_SRC_DIO_3)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio9_src_dio_4(self) -> &'a mut W {
        self.variant(GPIO9_A::RF_GPIO9_SRC_DIO_4)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio9_src_dio_5(self) -> &'a mut W {
        self.variant(GPIO9_A::RF_GPIO9_SRC_DIO_5)
    }
    #[doc = "Select DIO\\[6\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio9_src_dio_6(self) -> &'a mut W {
        self.variant(GPIO9_A::RF_GPIO9_SRC_DIO_6)
    }
    #[doc = "Select DIO\\[7\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio9_src_dio_7(self) -> &'a mut W {
        self.variant(GPIO9_A::RF_GPIO9_SRC_DIO_7)
    }
    #[doc = "Select DIO\\[8\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio9_src_dio_8(self) -> &'a mut W {
        self.variant(GPIO9_A::RF_GPIO9_SRC_DIO_8)
    }
    #[doc = "Select DIO\\[9\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio9_src_dio_9(self) -> &'a mut W {
        self.variant(GPIO9_A::RF_GPIO9_SRC_DIO_9)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio9_src_dio_10(self) -> &'a mut W {
        self.variant(GPIO9_A::RF_GPIO9_SRC_DIO_10)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio9_src_dio_11(self) -> &'a mut W {
        self.variant(GPIO9_A::RF_GPIO9_SRC_DIO_11)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio9_src_dio_12(self) -> &'a mut W {
        self.variant(GPIO9_A::RF_GPIO9_SRC_DIO_12)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio9_src_dio_13(self) -> &'a mut W {
        self.variant(GPIO9_A::RF_GPIO9_SRC_DIO_13)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio9_src_dio_14(self) -> &'a mut W {
        self.variant(GPIO9_A::RF_GPIO9_SRC_DIO_14)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio9_src_dio_15(self) -> &'a mut W {
        self.variant(GPIO9_A::RF_GPIO9_SRC_DIO_15)
    }
    #[doc = "Select constant low as source"]
    #[inline(always)]
    pub fn rf_gpio9_src_const_low(self) -> &'a mut W {
        self.variant(GPIO9_A::RF_GPIO9_SRC_CONST_LOW)
    }
    #[doc = "Select constant high as source"]
    #[inline(always)]
    pub fn rf_gpio9_src_const_high(self) -> &'a mut W {
        self.variant(GPIO9_A::RF_GPIO9_SRC_CONST_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "RF front-end GPIO8 input selection\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO8_A {
    #[doc = "0: Select DIO\\[0\\]
as source"]
    RF_GPIO8_SRC_DIO_0 = 0,
    #[doc = "1: Select DIO\\[1\\]
as source"]
    RF_GPIO8_SRC_DIO_1 = 1,
    #[doc = "2: Select DIO\\[5\\]
as source"]
    RF_GPIO8_SRC_DIO_2 = 2,
    #[doc = "3: Select DIO\\[3\\]
as source"]
    RF_GPIO8_SRC_DIO_3 = 3,
    #[doc = "4: Select DIO\\[4\\]
as source"]
    RF_GPIO8_SRC_DIO_4 = 4,
    #[doc = "5: Select DIO\\[5\\]
as source"]
    RF_GPIO8_SRC_DIO_5 = 5,
    #[doc = "6: Select DIO\\[6\\]
as source"]
    RF_GPIO8_SRC_DIO_6 = 6,
    #[doc = "7: Select DIO\\[7\\]
as source"]
    RF_GPIO8_SRC_DIO_7 = 7,
    #[doc = "8: Select DIO\\[8\\]
as source"]
    RF_GPIO8_SRC_DIO_8 = 8,
    #[doc = "9: Select DIO\\[9\\]
as source"]
    RF_GPIO8_SRC_DIO_9 = 9,
    #[doc = "10: Select DIO\\[10\\]
as source"]
    RF_GPIO8_SRC_DIO_10 = 10,
    #[doc = "11: Select DIO\\[11\\]
as source"]
    RF_GPIO8_SRC_DIO_11 = 11,
    #[doc = "12: Select DIO\\[15\\]
as source"]
    RF_GPIO8_SRC_DIO_12 = 12,
    #[doc = "13: Select DIO\\[13\\]
as source"]
    RF_GPIO8_SRC_DIO_13 = 13,
    #[doc = "14: Select DIO\\[14\\]
as source"]
    RF_GPIO8_SRC_DIO_14 = 14,
    #[doc = "15: Select DIO\\[15\\]
as source"]
    RF_GPIO8_SRC_DIO_15 = 15,
    #[doc = "16: Select constant low as source"]
    RF_GPIO8_SRC_CONST_LOW = 16,
    #[doc = "17: Select constant high as source"]
    RF_GPIO8_SRC_CONST_HIGH = 17,
}
impl From<GPIO8_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO8_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO8`"]
pub type GPIO8_R = crate::R<u8, GPIO8_A>;
impl GPIO8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GPIO8_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO8_A::RF_GPIO8_SRC_DIO_0),
            1 => Val(GPIO8_A::RF_GPIO8_SRC_DIO_1),
            2 => Val(GPIO8_A::RF_GPIO8_SRC_DIO_2),
            3 => Val(GPIO8_A::RF_GPIO8_SRC_DIO_3),
            4 => Val(GPIO8_A::RF_GPIO8_SRC_DIO_4),
            5 => Val(GPIO8_A::RF_GPIO8_SRC_DIO_5),
            6 => Val(GPIO8_A::RF_GPIO8_SRC_DIO_6),
            7 => Val(GPIO8_A::RF_GPIO8_SRC_DIO_7),
            8 => Val(GPIO8_A::RF_GPIO8_SRC_DIO_8),
            9 => Val(GPIO8_A::RF_GPIO8_SRC_DIO_9),
            10 => Val(GPIO8_A::RF_GPIO8_SRC_DIO_10),
            11 => Val(GPIO8_A::RF_GPIO8_SRC_DIO_11),
            12 => Val(GPIO8_A::RF_GPIO8_SRC_DIO_12),
            13 => Val(GPIO8_A::RF_GPIO8_SRC_DIO_13),
            14 => Val(GPIO8_A::RF_GPIO8_SRC_DIO_14),
            15 => Val(GPIO8_A::RF_GPIO8_SRC_DIO_15),
            16 => Val(GPIO8_A::RF_GPIO8_SRC_CONST_LOW),
            17 => Val(GPIO8_A::RF_GPIO8_SRC_CONST_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RF_GPIO8_SRC_DIO_0`"]
    #[inline(always)]
    pub fn is_rf_gpio8_src_dio_0(&self) -> bool {
        *self == GPIO8_A::RF_GPIO8_SRC_DIO_0
    }
    #[doc = "Checks if the value of the field is `RF_GPIO8_SRC_DIO_1`"]
    #[inline(always)]
    pub fn is_rf_gpio8_src_dio_1(&self) -> bool {
        *self == GPIO8_A::RF_GPIO8_SRC_DIO_1
    }
    #[doc = "Checks if the value of the field is `RF_GPIO8_SRC_DIO_2`"]
    #[inline(always)]
    pub fn is_rf_gpio8_src_dio_2(&self) -> bool {
        *self == GPIO8_A::RF_GPIO8_SRC_DIO_2
    }
    #[doc = "Checks if the value of the field is `RF_GPIO8_SRC_DIO_3`"]
    #[inline(always)]
    pub fn is_rf_gpio8_src_dio_3(&self) -> bool {
        *self == GPIO8_A::RF_GPIO8_SRC_DIO_3
    }
    #[doc = "Checks if the value of the field is `RF_GPIO8_SRC_DIO_4`"]
    #[inline(always)]
    pub fn is_rf_gpio8_src_dio_4(&self) -> bool {
        *self == GPIO8_A::RF_GPIO8_SRC_DIO_4
    }
    #[doc = "Checks if the value of the field is `RF_GPIO8_SRC_DIO_5`"]
    #[inline(always)]
    pub fn is_rf_gpio8_src_dio_5(&self) -> bool {
        *self == GPIO8_A::RF_GPIO8_SRC_DIO_5
    }
    #[doc = "Checks if the value of the field is `RF_GPIO8_SRC_DIO_6`"]
    #[inline(always)]
    pub fn is_rf_gpio8_src_dio_6(&self) -> bool {
        *self == GPIO8_A::RF_GPIO8_SRC_DIO_6
    }
    #[doc = "Checks if the value of the field is `RF_GPIO8_SRC_DIO_7`"]
    #[inline(always)]
    pub fn is_rf_gpio8_src_dio_7(&self) -> bool {
        *self == GPIO8_A::RF_GPIO8_SRC_DIO_7
    }
    #[doc = "Checks if the value of the field is `RF_GPIO8_SRC_DIO_8`"]
    #[inline(always)]
    pub fn is_rf_gpio8_src_dio_8(&self) -> bool {
        *self == GPIO8_A::RF_GPIO8_SRC_DIO_8
    }
    #[doc = "Checks if the value of the field is `RF_GPIO8_SRC_DIO_9`"]
    #[inline(always)]
    pub fn is_rf_gpio8_src_dio_9(&self) -> bool {
        *self == GPIO8_A::RF_GPIO8_SRC_DIO_9
    }
    #[doc = "Checks if the value of the field is `RF_GPIO8_SRC_DIO_10`"]
    #[inline(always)]
    pub fn is_rf_gpio8_src_dio_10(&self) -> bool {
        *self == GPIO8_A::RF_GPIO8_SRC_DIO_10
    }
    #[doc = "Checks if the value of the field is `RF_GPIO8_SRC_DIO_11`"]
    #[inline(always)]
    pub fn is_rf_gpio8_src_dio_11(&self) -> bool {
        *self == GPIO8_A::RF_GPIO8_SRC_DIO_11
    }
    #[doc = "Checks if the value of the field is `RF_GPIO8_SRC_DIO_12`"]
    #[inline(always)]
    pub fn is_rf_gpio8_src_dio_12(&self) -> bool {
        *self == GPIO8_A::RF_GPIO8_SRC_DIO_12
    }
    #[doc = "Checks if the value of the field is `RF_GPIO8_SRC_DIO_13`"]
    #[inline(always)]
    pub fn is_rf_gpio8_src_dio_13(&self) -> bool {
        *self == GPIO8_A::RF_GPIO8_SRC_DIO_13
    }
    #[doc = "Checks if the value of the field is `RF_GPIO8_SRC_DIO_14`"]
    #[inline(always)]
    pub fn is_rf_gpio8_src_dio_14(&self) -> bool {
        *self == GPIO8_A::RF_GPIO8_SRC_DIO_14
    }
    #[doc = "Checks if the value of the field is `RF_GPIO8_SRC_DIO_15`"]
    #[inline(always)]
    pub fn is_rf_gpio8_src_dio_15(&self) -> bool {
        *self == GPIO8_A::RF_GPIO8_SRC_DIO_15
    }
    #[doc = "Checks if the value of the field is `RF_GPIO8_SRC_CONST_LOW`"]
    #[inline(always)]
    pub fn is_rf_gpio8_src_const_low(&self) -> bool {
        *self == GPIO8_A::RF_GPIO8_SRC_CONST_LOW
    }
    #[doc = "Checks if the value of the field is `RF_GPIO8_SRC_CONST_HIGH`"]
    #[inline(always)]
    pub fn is_rf_gpio8_src_const_high(&self) -> bool {
        *self == GPIO8_A::RF_GPIO8_SRC_CONST_HIGH
    }
}
#[doc = "Write proxy for field `GPIO8`"]
pub struct GPIO8_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio8_src_dio_0(self) -> &'a mut W {
        self.variant(GPIO8_A::RF_GPIO8_SRC_DIO_0)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio8_src_dio_1(self) -> &'a mut W {
        self.variant(GPIO8_A::RF_GPIO8_SRC_DIO_1)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio8_src_dio_2(self) -> &'a mut W {
        self.variant(GPIO8_A::RF_GPIO8_SRC_DIO_2)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio8_src_dio_3(self) -> &'a mut W {
        self.variant(GPIO8_A::RF_GPIO8_SRC_DIO_3)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio8_src_dio_4(self) -> &'a mut W {
        self.variant(GPIO8_A::RF_GPIO8_SRC_DIO_4)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio8_src_dio_5(self) -> &'a mut W {
        self.variant(GPIO8_A::RF_GPIO8_SRC_DIO_5)
    }
    #[doc = "Select DIO\\[6\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio8_src_dio_6(self) -> &'a mut W {
        self.variant(GPIO8_A::RF_GPIO8_SRC_DIO_6)
    }
    #[doc = "Select DIO\\[7\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio8_src_dio_7(self) -> &'a mut W {
        self.variant(GPIO8_A::RF_GPIO8_SRC_DIO_7)
    }
    #[doc = "Select DIO\\[8\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio8_src_dio_8(self) -> &'a mut W {
        self.variant(GPIO8_A::RF_GPIO8_SRC_DIO_8)
    }
    #[doc = "Select DIO\\[9\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio8_src_dio_9(self) -> &'a mut W {
        self.variant(GPIO8_A::RF_GPIO8_SRC_DIO_9)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio8_src_dio_10(self) -> &'a mut W {
        self.variant(GPIO8_A::RF_GPIO8_SRC_DIO_10)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio8_src_dio_11(self) -> &'a mut W {
        self.variant(GPIO8_A::RF_GPIO8_SRC_DIO_11)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio8_src_dio_12(self) -> &'a mut W {
        self.variant(GPIO8_A::RF_GPIO8_SRC_DIO_12)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio8_src_dio_13(self) -> &'a mut W {
        self.variant(GPIO8_A::RF_GPIO8_SRC_DIO_13)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio8_src_dio_14(self) -> &'a mut W {
        self.variant(GPIO8_A::RF_GPIO8_SRC_DIO_14)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio8_src_dio_15(self) -> &'a mut W {
        self.variant(GPIO8_A::RF_GPIO8_SRC_DIO_15)
    }
    #[doc = "Select constant low as source"]
    #[inline(always)]
    pub fn rf_gpio8_src_const_low(self) -> &'a mut W {
        self.variant(GPIO8_A::RF_GPIO8_SRC_CONST_LOW)
    }
    #[doc = "Select constant high as source"]
    #[inline(always)]
    pub fn rf_gpio8_src_const_high(self) -> &'a mut W {
        self.variant(GPIO8_A::RF_GPIO8_SRC_CONST_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:12 - RF front-end GPIO9 input selection"]
    #[inline(always)]
    pub fn gpio9(&self) -> GPIO9_R {
        GPIO9_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - RF front-end GPIO8 input selection"]
    #[inline(always)]
    pub fn gpio8(&self) -> GPIO8_R {
        GPIO8_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - RF front-end GPIO9 input selection"]
    #[inline(always)]
    pub fn gpio9(&mut self) -> GPIO9_W {
        GPIO9_W { w: self }
    }
    #[doc = "Bits 0:4 - RF front-end GPIO8 input selection"]
    #[inline(always)]
    pub fn gpio8(&mut self) -> GPIO8_W {
        GPIO8_W { w: self }
    }
}
