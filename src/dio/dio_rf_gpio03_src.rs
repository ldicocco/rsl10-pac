#[doc = "Reader of register DIO_RF_GPIO03_SRC"]
pub type R = crate::R<u32, super::DIO_RF_GPIO03_SRC>;
#[doc = "Writer for register DIO_RF_GPIO03_SRC"]
pub type W = crate::W<u32, super::DIO_RF_GPIO03_SRC>;
#[doc = "Register DIO_RF_GPIO03_SRC `reset()`'s with value 0x1212_1010"]
impl crate::ResetValue for super::DIO_RF_GPIO03_SRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1212_1010
    }
}
#[doc = "RF front-end GPIO3 input selection\n\nValue on reset: 18"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO3_A {
    #[doc = "0: Select DIO\\[0\\]
as source"]
    RF_GPIO3_SRC_DIO_0 = 0,
    #[doc = "1: Select DIO\\[1\\]
as source"]
    RF_GPIO3_SRC_DIO_1 = 1,
    #[doc = "2: Select DIO\\[3\\]
as source"]
    RF_GPIO3_SRC_DIO_2 = 2,
    #[doc = "3: Select DIO\\[3\\]
as source"]
    RF_GPIO3_SRC_DIO_3 = 3,
    #[doc = "4: Select DIO\\[4\\]
as source"]
    RF_GPIO3_SRC_DIO_4 = 4,
    #[doc = "5: Select DIO\\[5\\]
as source"]
    RF_GPIO3_SRC_DIO_5 = 5,
    #[doc = "6: Select DIO\\[6\\]
as source"]
    RF_GPIO3_SRC_DIO_6 = 6,
    #[doc = "7: Select DIO\\[7\\]
as source"]
    RF_GPIO3_SRC_DIO_7 = 7,
    #[doc = "8: Select DIO\\[8\\]
as source"]
    RF_GPIO3_SRC_DIO_8 = 8,
    #[doc = "9: Select DIO\\[9\\]
as source"]
    RF_GPIO3_SRC_DIO_9 = 9,
    #[doc = "10: Select DIO\\[10\\]
as source"]
    RF_GPIO3_SRC_DIO_10 = 10,
    #[doc = "11: Select DIO\\[11\\]
as source"]
    RF_GPIO3_SRC_DIO_11 = 11,
    #[doc = "12: Select DIO\\[13\\]
as source"]
    RF_GPIO3_SRC_DIO_12 = 12,
    #[doc = "13: Select DIO\\[13\\]
as source"]
    RF_GPIO3_SRC_DIO_13 = 13,
    #[doc = "14: Select DIO\\[14\\]
as source"]
    RF_GPIO3_SRC_DIO_14 = 14,
    #[doc = "15: Select DIO\\[15\\]
as source"]
    RF_GPIO3_SRC_DIO_15 = 15,
    #[doc = "16: Select constant low as source"]
    RF_GPIO3_SRC_CONST_LOW = 16,
    #[doc = "17: Select constant high as source"]
    RF_GPIO3_SRC_CONST_HIGH = 17,
    #[doc = "18: Select baseband controller TX_DATA as source"]
    RF_GPIO3_SRC_BB_TX_DATA = 18,
}
impl From<GPIO3_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO3`"]
pub type GPIO3_R = crate::R<u8, GPIO3_A>;
impl GPIO3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GPIO3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO3_A::RF_GPIO3_SRC_DIO_0),
            1 => Val(GPIO3_A::RF_GPIO3_SRC_DIO_1),
            2 => Val(GPIO3_A::RF_GPIO3_SRC_DIO_2),
            3 => Val(GPIO3_A::RF_GPIO3_SRC_DIO_3),
            4 => Val(GPIO3_A::RF_GPIO3_SRC_DIO_4),
            5 => Val(GPIO3_A::RF_GPIO3_SRC_DIO_5),
            6 => Val(GPIO3_A::RF_GPIO3_SRC_DIO_6),
            7 => Val(GPIO3_A::RF_GPIO3_SRC_DIO_7),
            8 => Val(GPIO3_A::RF_GPIO3_SRC_DIO_8),
            9 => Val(GPIO3_A::RF_GPIO3_SRC_DIO_9),
            10 => Val(GPIO3_A::RF_GPIO3_SRC_DIO_10),
            11 => Val(GPIO3_A::RF_GPIO3_SRC_DIO_11),
            12 => Val(GPIO3_A::RF_GPIO3_SRC_DIO_12),
            13 => Val(GPIO3_A::RF_GPIO3_SRC_DIO_13),
            14 => Val(GPIO3_A::RF_GPIO3_SRC_DIO_14),
            15 => Val(GPIO3_A::RF_GPIO3_SRC_DIO_15),
            16 => Val(GPIO3_A::RF_GPIO3_SRC_CONST_LOW),
            17 => Val(GPIO3_A::RF_GPIO3_SRC_CONST_HIGH),
            18 => Val(GPIO3_A::RF_GPIO3_SRC_BB_TX_DATA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_DIO_0`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_dio_0(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_DIO_0
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_DIO_1`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_dio_1(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_DIO_1
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_DIO_2`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_dio_2(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_DIO_2
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_DIO_3`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_dio_3(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_DIO_3
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_DIO_4`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_dio_4(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_DIO_4
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_DIO_5`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_dio_5(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_DIO_5
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_DIO_6`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_dio_6(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_DIO_6
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_DIO_7`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_dio_7(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_DIO_7
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_DIO_8`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_dio_8(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_DIO_8
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_DIO_9`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_dio_9(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_DIO_9
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_DIO_10`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_dio_10(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_DIO_10
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_DIO_11`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_dio_11(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_DIO_11
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_DIO_12`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_dio_12(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_DIO_12
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_DIO_13`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_dio_13(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_DIO_13
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_DIO_14`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_dio_14(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_DIO_14
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_DIO_15`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_dio_15(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_DIO_15
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_CONST_LOW`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_const_low(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_CONST_LOW
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_CONST_HIGH`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_const_high(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_CONST_HIGH
    }
    #[doc = "Checks if the value of the field is `RF_GPIO3_SRC_BB_TX_DATA`"]
    #[inline(always)]
    pub fn is_rf_gpio3_src_bb_tx_data(&self) -> bool {
        *self == GPIO3_A::RF_GPIO3_SRC_BB_TX_DATA
    }
}
#[doc = "Write proxy for field `GPIO3`"]
pub struct GPIO3_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_dio_0(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_DIO_0)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_dio_1(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_DIO_1)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_dio_2(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_DIO_2)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_dio_3(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_DIO_3)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_dio_4(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_DIO_4)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_dio_5(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_DIO_5)
    }
    #[doc = "Select DIO\\[6\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_dio_6(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_DIO_6)
    }
    #[doc = "Select DIO\\[7\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_dio_7(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_DIO_7)
    }
    #[doc = "Select DIO\\[8\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_dio_8(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_DIO_8)
    }
    #[doc = "Select DIO\\[9\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_dio_9(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_DIO_9)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_dio_10(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_DIO_10)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_dio_11(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_DIO_11)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_dio_12(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_DIO_12)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_dio_13(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_DIO_13)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_dio_14(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_DIO_14)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_dio_15(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_DIO_15)
    }
    #[doc = "Select constant low as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_const_low(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_CONST_LOW)
    }
    #[doc = "Select constant high as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_const_high(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_CONST_HIGH)
    }
    #[doc = "Select baseband controller TX_DATA as source"]
    #[inline(always)]
    pub fn rf_gpio3_src_bb_tx_data(self) -> &'a mut W {
        self.variant(GPIO3_A::RF_GPIO3_SRC_BB_TX_DATA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "RF front-end GPIO2 input selection\n\nValue on reset: 18"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO2_A {
    #[doc = "0: Select DIO\\[0\\]
as source"]
    RF_GPIO2_SRC_DIO_0 = 0,
    #[doc = "1: Select DIO\\[1\\]
as source"]
    RF_GPIO2_SRC_DIO_1 = 1,
    #[doc = "2: Select DIO\\[2\\]
as source"]
    RF_GPIO2_SRC_DIO_2 = 2,
    #[doc = "3: Select DIO\\[3\\]
as source"]
    RF_GPIO2_SRC_DIO_3 = 3,
    #[doc = "4: Select DIO\\[4\\]
as source"]
    RF_GPIO2_SRC_DIO_4 = 4,
    #[doc = "5: Select DIO\\[5\\]
as source"]
    RF_GPIO2_SRC_DIO_5 = 5,
    #[doc = "6: Select DIO\\[6\\]
as source"]
    RF_GPIO2_SRC_DIO_6 = 6,
    #[doc = "7: Select DIO\\[7\\]
as source"]
    RF_GPIO2_SRC_DIO_7 = 7,
    #[doc = "8: Select DIO\\[8\\]
as source"]
    RF_GPIO2_SRC_DIO_8 = 8,
    #[doc = "9: Select DIO\\[9\\]
as source"]
    RF_GPIO2_SRC_DIO_9 = 9,
    #[doc = "10: Select DIO\\[10\\]
as source"]
    RF_GPIO2_SRC_DIO_10 = 10,
    #[doc = "11: Select DIO\\[11\\]
as source"]
    RF_GPIO2_SRC_DIO_11 = 11,
    #[doc = "12: Select DIO\\[12\\]
as source"]
    RF_GPIO2_SRC_DIO_12 = 12,
    #[doc = "13: Select DIO\\[13\\]
as source"]
    RF_GPIO2_SRC_DIO_13 = 13,
    #[doc = "14: Select DIO\\[14\\]
as source"]
    RF_GPIO2_SRC_DIO_14 = 14,
    #[doc = "15: Select DIO\\[15\\]
as source"]
    RF_GPIO2_SRC_DIO_15 = 15,
    #[doc = "16: Select constant low as source"]
    RF_GPIO2_SRC_CONST_LOW = 16,
    #[doc = "17: Select constant high as source"]
    RF_GPIO2_SRC_CONST_HIGH = 17,
    #[doc = "18: Select baseband controller SYNC_P as source"]
    RF_GPIO2_SRC_BB_SYNC_P = 18,
}
impl From<GPIO2_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO2`"]
pub type GPIO2_R = crate::R<u8, GPIO2_A>;
impl GPIO2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GPIO2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO2_A::RF_GPIO2_SRC_DIO_0),
            1 => Val(GPIO2_A::RF_GPIO2_SRC_DIO_1),
            2 => Val(GPIO2_A::RF_GPIO2_SRC_DIO_2),
            3 => Val(GPIO2_A::RF_GPIO2_SRC_DIO_3),
            4 => Val(GPIO2_A::RF_GPIO2_SRC_DIO_4),
            5 => Val(GPIO2_A::RF_GPIO2_SRC_DIO_5),
            6 => Val(GPIO2_A::RF_GPIO2_SRC_DIO_6),
            7 => Val(GPIO2_A::RF_GPIO2_SRC_DIO_7),
            8 => Val(GPIO2_A::RF_GPIO2_SRC_DIO_8),
            9 => Val(GPIO2_A::RF_GPIO2_SRC_DIO_9),
            10 => Val(GPIO2_A::RF_GPIO2_SRC_DIO_10),
            11 => Val(GPIO2_A::RF_GPIO2_SRC_DIO_11),
            12 => Val(GPIO2_A::RF_GPIO2_SRC_DIO_12),
            13 => Val(GPIO2_A::RF_GPIO2_SRC_DIO_13),
            14 => Val(GPIO2_A::RF_GPIO2_SRC_DIO_14),
            15 => Val(GPIO2_A::RF_GPIO2_SRC_DIO_15),
            16 => Val(GPIO2_A::RF_GPIO2_SRC_CONST_LOW),
            17 => Val(GPIO2_A::RF_GPIO2_SRC_CONST_HIGH),
            18 => Val(GPIO2_A::RF_GPIO2_SRC_BB_SYNC_P),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_DIO_0`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_dio_0(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_DIO_0
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_DIO_1`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_dio_1(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_DIO_1
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_DIO_2`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_dio_2(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_DIO_2
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_DIO_3`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_dio_3(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_DIO_3
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_DIO_4`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_dio_4(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_DIO_4
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_DIO_5`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_dio_5(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_DIO_5
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_DIO_6`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_dio_6(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_DIO_6
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_DIO_7`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_dio_7(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_DIO_7
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_DIO_8`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_dio_8(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_DIO_8
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_DIO_9`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_dio_9(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_DIO_9
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_DIO_10`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_dio_10(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_DIO_10
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_DIO_11`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_dio_11(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_DIO_11
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_DIO_12`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_dio_12(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_DIO_12
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_DIO_13`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_dio_13(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_DIO_13
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_DIO_14`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_dio_14(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_DIO_14
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_DIO_15`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_dio_15(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_DIO_15
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_CONST_LOW`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_const_low(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_CONST_LOW
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_CONST_HIGH`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_const_high(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_CONST_HIGH
    }
    #[doc = "Checks if the value of the field is `RF_GPIO2_SRC_BB_SYNC_P`"]
    #[inline(always)]
    pub fn is_rf_gpio2_src_bb_sync_p(&self) -> bool {
        *self == GPIO2_A::RF_GPIO2_SRC_BB_SYNC_P
    }
}
#[doc = "Write proxy for field `GPIO2`"]
pub struct GPIO2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_dio_0(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_DIO_0)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_dio_1(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_DIO_1)
    }
    #[doc = "Select DIO\\[2\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_dio_2(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_DIO_2)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_dio_3(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_DIO_3)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_dio_4(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_DIO_4)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_dio_5(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_DIO_5)
    }
    #[doc = "Select DIO\\[6\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_dio_6(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_DIO_6)
    }
    #[doc = "Select DIO\\[7\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_dio_7(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_DIO_7)
    }
    #[doc = "Select DIO\\[8\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_dio_8(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_DIO_8)
    }
    #[doc = "Select DIO\\[9\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_dio_9(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_DIO_9)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_dio_10(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_DIO_10)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_dio_11(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_DIO_11)
    }
    #[doc = "Select DIO\\[12\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_dio_12(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_DIO_12)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_dio_13(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_DIO_13)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_dio_14(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_DIO_14)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_dio_15(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_DIO_15)
    }
    #[doc = "Select constant low as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_const_low(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_CONST_LOW)
    }
    #[doc = "Select constant high as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_const_high(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_CONST_HIGH)
    }
    #[doc = "Select baseband controller SYNC_P as source"]
    #[inline(always)]
    pub fn rf_gpio2_src_bb_sync_p(self) -> &'a mut W {
        self.variant(GPIO2_A::RF_GPIO2_SRC_BB_SYNC_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "RF front-end GPIO1 input selection\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO1_A {
    #[doc = "0: Select DIO\\[0\\]
as source"]
    RF_GPIO1_SRC_DIO_0 = 0,
    #[doc = "1: Select DIO\\[1\\]
as source"]
    RF_GPIO1_SRC_DIO_1 = 1,
    #[doc = "2: Select DIO\\[1\\]
as source"]
    RF_GPIO1_SRC_DIO_2 = 2,
    #[doc = "3: Select DIO\\[3\\]
as source"]
    RF_GPIO1_SRC_DIO_3 = 3,
    #[doc = "4: Select DIO\\[4\\]
as source"]
    RF_GPIO1_SRC_DIO_4 = 4,
    #[doc = "5: Select DIO\\[5\\]
as source"]
    RF_GPIO1_SRC_DIO_5 = 5,
    #[doc = "6: Select DIO\\[6\\]
as source"]
    RF_GPIO1_SRC_DIO_6 = 6,
    #[doc = "7: Select DIO\\[7\\]
as source"]
    RF_GPIO1_SRC_DIO_7 = 7,
    #[doc = "8: Select DIO\\[8\\]
as source"]
    RF_GPIO1_SRC_DIO_8 = 8,
    #[doc = "9: Select DIO\\[9\\]
as source"]
    RF_GPIO1_SRC_DIO_9 = 9,
    #[doc = "10: Select DIO\\[10\\]
as source"]
    RF_GPIO1_SRC_DIO_10 = 10,
    #[doc = "11: Select DIO\\[11\\]
as source"]
    RF_GPIO1_SRC_DIO_11 = 11,
    #[doc = "12: Select DIO\\[11\\]
as source"]
    RF_GPIO1_SRC_DIO_12 = 12,
    #[doc = "13: Select DIO\\[13\\]
as source"]
    RF_GPIO1_SRC_DIO_13 = 13,
    #[doc = "14: Select DIO\\[14\\]
as source"]
    RF_GPIO1_SRC_DIO_14 = 14,
    #[doc = "15: Select DIO\\[15\\]
as source"]
    RF_GPIO1_SRC_DIO_15 = 15,
    #[doc = "16: Select constant low as source"]
    RF_GPIO1_SRC_CONST_LOW = 16,
    #[doc = "17: Select constant high as source"]
    RF_GPIO1_SRC_CONST_HIGH = 17,
}
impl From<GPIO1_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO1`"]
pub type GPIO1_R = crate::R<u8, GPIO1_A>;
impl GPIO1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GPIO1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO1_A::RF_GPIO1_SRC_DIO_0),
            1 => Val(GPIO1_A::RF_GPIO1_SRC_DIO_1),
            2 => Val(GPIO1_A::RF_GPIO1_SRC_DIO_2),
            3 => Val(GPIO1_A::RF_GPIO1_SRC_DIO_3),
            4 => Val(GPIO1_A::RF_GPIO1_SRC_DIO_4),
            5 => Val(GPIO1_A::RF_GPIO1_SRC_DIO_5),
            6 => Val(GPIO1_A::RF_GPIO1_SRC_DIO_6),
            7 => Val(GPIO1_A::RF_GPIO1_SRC_DIO_7),
            8 => Val(GPIO1_A::RF_GPIO1_SRC_DIO_8),
            9 => Val(GPIO1_A::RF_GPIO1_SRC_DIO_9),
            10 => Val(GPIO1_A::RF_GPIO1_SRC_DIO_10),
            11 => Val(GPIO1_A::RF_GPIO1_SRC_DIO_11),
            12 => Val(GPIO1_A::RF_GPIO1_SRC_DIO_12),
            13 => Val(GPIO1_A::RF_GPIO1_SRC_DIO_13),
            14 => Val(GPIO1_A::RF_GPIO1_SRC_DIO_14),
            15 => Val(GPIO1_A::RF_GPIO1_SRC_DIO_15),
            16 => Val(GPIO1_A::RF_GPIO1_SRC_CONST_LOW),
            17 => Val(GPIO1_A::RF_GPIO1_SRC_CONST_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RF_GPIO1_SRC_DIO_0`"]
    #[inline(always)]
    pub fn is_rf_gpio1_src_dio_0(&self) -> bool {
        *self == GPIO1_A::RF_GPIO1_SRC_DIO_0
    }
    #[doc = "Checks if the value of the field is `RF_GPIO1_SRC_DIO_1`"]
    #[inline(always)]
    pub fn is_rf_gpio1_src_dio_1(&self) -> bool {
        *self == GPIO1_A::RF_GPIO1_SRC_DIO_1
    }
    #[doc = "Checks if the value of the field is `RF_GPIO1_SRC_DIO_2`"]
    #[inline(always)]
    pub fn is_rf_gpio1_src_dio_2(&self) -> bool {
        *self == GPIO1_A::RF_GPIO1_SRC_DIO_2
    }
    #[doc = "Checks if the value of the field is `RF_GPIO1_SRC_DIO_3`"]
    #[inline(always)]
    pub fn is_rf_gpio1_src_dio_3(&self) -> bool {
        *self == GPIO1_A::RF_GPIO1_SRC_DIO_3
    }
    #[doc = "Checks if the value of the field is `RF_GPIO1_SRC_DIO_4`"]
    #[inline(always)]
    pub fn is_rf_gpio1_src_dio_4(&self) -> bool {
        *self == GPIO1_A::RF_GPIO1_SRC_DIO_4
    }
    #[doc = "Checks if the value of the field is `RF_GPIO1_SRC_DIO_5`"]
    #[inline(always)]
    pub fn is_rf_gpio1_src_dio_5(&self) -> bool {
        *self == GPIO1_A::RF_GPIO1_SRC_DIO_5
    }
    #[doc = "Checks if the value of the field is `RF_GPIO1_SRC_DIO_6`"]
    #[inline(always)]
    pub fn is_rf_gpio1_src_dio_6(&self) -> bool {
        *self == GPIO1_A::RF_GPIO1_SRC_DIO_6
    }
    #[doc = "Checks if the value of the field is `RF_GPIO1_SRC_DIO_7`"]
    #[inline(always)]
    pub fn is_rf_gpio1_src_dio_7(&self) -> bool {
        *self == GPIO1_A::RF_GPIO1_SRC_DIO_7
    }
    #[doc = "Checks if the value of the field is `RF_GPIO1_SRC_DIO_8`"]
    #[inline(always)]
    pub fn is_rf_gpio1_src_dio_8(&self) -> bool {
        *self == GPIO1_A::RF_GPIO1_SRC_DIO_8
    }
    #[doc = "Checks if the value of the field is `RF_GPIO1_SRC_DIO_9`"]
    #[inline(always)]
    pub fn is_rf_gpio1_src_dio_9(&self) -> bool {
        *self == GPIO1_A::RF_GPIO1_SRC_DIO_9
    }
    #[doc = "Checks if the value of the field is `RF_GPIO1_SRC_DIO_10`"]
    #[inline(always)]
    pub fn is_rf_gpio1_src_dio_10(&self) -> bool {
        *self == GPIO1_A::RF_GPIO1_SRC_DIO_10
    }
    #[doc = "Checks if the value of the field is `RF_GPIO1_SRC_DIO_11`"]
    #[inline(always)]
    pub fn is_rf_gpio1_src_dio_11(&self) -> bool {
        *self == GPIO1_A::RF_GPIO1_SRC_DIO_11
    }
    #[doc = "Checks if the value of the field is `RF_GPIO1_SRC_DIO_12`"]
    #[inline(always)]
    pub fn is_rf_gpio1_src_dio_12(&self) -> bool {
        *self == GPIO1_A::RF_GPIO1_SRC_DIO_12
    }
    #[doc = "Checks if the value of the field is `RF_GPIO1_SRC_DIO_13`"]
    #[inline(always)]
    pub fn is_rf_gpio1_src_dio_13(&self) -> bool {
        *self == GPIO1_A::RF_GPIO1_SRC_DIO_13
    }
    #[doc = "Checks if the value of the field is `RF_GPIO1_SRC_DIO_14`"]
    #[inline(always)]
    pub fn is_rf_gpio1_src_dio_14(&self) -> bool {
        *self == GPIO1_A::RF_GPIO1_SRC_DIO_14
    }
    #[doc = "Checks if the value of the field is `RF_GPIO1_SRC_DIO_15`"]
    #[inline(always)]
    pub fn is_rf_gpio1_src_dio_15(&self) -> bool {
        *self == GPIO1_A::RF_GPIO1_SRC_DIO_15
    }
    #[doc = "Checks if the value of the field is `RF_GPIO1_SRC_CONST_LOW`"]
    #[inline(always)]
    pub fn is_rf_gpio1_src_const_low(&self) -> bool {
        *self == GPIO1_A::RF_GPIO1_SRC_CONST_LOW
    }
    #[doc = "Checks if the value of the field is `RF_GPIO1_SRC_CONST_HIGH`"]
    #[inline(always)]
    pub fn is_rf_gpio1_src_const_high(&self) -> bool {
        *self == GPIO1_A::RF_GPIO1_SRC_CONST_HIGH
    }
}
#[doc = "Write proxy for field `GPIO1`"]
pub struct GPIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio1_src_dio_0(self) -> &'a mut W {
        self.variant(GPIO1_A::RF_GPIO1_SRC_DIO_0)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio1_src_dio_1(self) -> &'a mut W {
        self.variant(GPIO1_A::RF_GPIO1_SRC_DIO_1)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio1_src_dio_2(self) -> &'a mut W {
        self.variant(GPIO1_A::RF_GPIO1_SRC_DIO_2)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio1_src_dio_3(self) -> &'a mut W {
        self.variant(GPIO1_A::RF_GPIO1_SRC_DIO_3)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio1_src_dio_4(self) -> &'a mut W {
        self.variant(GPIO1_A::RF_GPIO1_SRC_DIO_4)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio1_src_dio_5(self) -> &'a mut W {
        self.variant(GPIO1_A::RF_GPIO1_SRC_DIO_5)
    }
    #[doc = "Select DIO\\[6\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio1_src_dio_6(self) -> &'a mut W {
        self.variant(GPIO1_A::RF_GPIO1_SRC_DIO_6)
    }
    #[doc = "Select DIO\\[7\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio1_src_dio_7(self) -> &'a mut W {
        self.variant(GPIO1_A::RF_GPIO1_SRC_DIO_7)
    }
    #[doc = "Select DIO\\[8\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio1_src_dio_8(self) -> &'a mut W {
        self.variant(GPIO1_A::RF_GPIO1_SRC_DIO_8)
    }
    #[doc = "Select DIO\\[9\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio1_src_dio_9(self) -> &'a mut W {
        self.variant(GPIO1_A::RF_GPIO1_SRC_DIO_9)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio1_src_dio_10(self) -> &'a mut W {
        self.variant(GPIO1_A::RF_GPIO1_SRC_DIO_10)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio1_src_dio_11(self) -> &'a mut W {
        self.variant(GPIO1_A::RF_GPIO1_SRC_DIO_11)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio1_src_dio_12(self) -> &'a mut W {
        self.variant(GPIO1_A::RF_GPIO1_SRC_DIO_12)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio1_src_dio_13(self) -> &'a mut W {
        self.variant(GPIO1_A::RF_GPIO1_SRC_DIO_13)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio1_src_dio_14(self) -> &'a mut W {
        self.variant(GPIO1_A::RF_GPIO1_SRC_DIO_14)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio1_src_dio_15(self) -> &'a mut W {
        self.variant(GPIO1_A::RF_GPIO1_SRC_DIO_15)
    }
    #[doc = "Select constant low as source"]
    #[inline(always)]
    pub fn rf_gpio1_src_const_low(self) -> &'a mut W {
        self.variant(GPIO1_A::RF_GPIO1_SRC_CONST_LOW)
    }
    #[doc = "Select constant high as source"]
    #[inline(always)]
    pub fn rf_gpio1_src_const_high(self) -> &'a mut W {
        self.variant(GPIO1_A::RF_GPIO1_SRC_CONST_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "RF front-end GPIO0 input selection\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO0_A {
    #[doc = "0: Select DIO\\[0\\]
as source"]
    RF_GPIO0_SRC_DIO_0 = 0,
    #[doc = "1: Select DIO\\[1\\]
as source"]
    RF_GPIO0_SRC_DIO_1 = 1,
    #[doc = "2: Select DIO\\[0\\]
as source"]
    RF_GPIO0_SRC_DIO_2 = 2,
    #[doc = "3: Select DIO\\[3\\]
as source"]
    RF_GPIO0_SRC_DIO_3 = 3,
    #[doc = "4: Select DIO\\[4\\]
as source"]
    RF_GPIO0_SRC_DIO_4 = 4,
    #[doc = "5: Select DIO\\[5\\]
as source"]
    RF_GPIO0_SRC_DIO_5 = 5,
    #[doc = "6: Select DIO\\[6\\]
as source"]
    RF_GPIO0_SRC_DIO_6 = 6,
    #[doc = "7: Select DIO\\[7\\]
as source"]
    RF_GPIO0_SRC_DIO_7 = 7,
    #[doc = "8: Select DIO\\[8\\]
as source"]
    RF_GPIO0_SRC_DIO_8 = 8,
    #[doc = "9: Select DIO\\[9\\]
as source"]
    RF_GPIO0_SRC_DIO_9 = 9,
    #[doc = "10: Select DIO\\[10\\]
as source"]
    RF_GPIO0_SRC_DIO_10 = 10,
    #[doc = "11: Select DIO\\[11\\]
as source"]
    RF_GPIO0_SRC_DIO_11 = 11,
    #[doc = "12: Select DIO\\[10\\]
as source"]
    RF_GPIO0_SRC_DIO_12 = 12,
    #[doc = "13: Select DIO\\[13\\]
as source"]
    RF_GPIO0_SRC_DIO_13 = 13,
    #[doc = "14: Select DIO\\[14\\]
as source"]
    RF_GPIO0_SRC_DIO_14 = 14,
    #[doc = "15: Select DIO\\[15\\]
as source"]
    RF_GPIO0_SRC_DIO_15 = 15,
    #[doc = "16: Select constant low as source"]
    RF_GPIO0_SRC_CONST_LOW = 16,
    #[doc = "17: Select constant high as source"]
    RF_GPIO0_SRC_CONST_HIGH = 17,
}
impl From<GPIO0_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO0`"]
pub type GPIO0_R = crate::R<u8, GPIO0_A>;
impl GPIO0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GPIO0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO0_A::RF_GPIO0_SRC_DIO_0),
            1 => Val(GPIO0_A::RF_GPIO0_SRC_DIO_1),
            2 => Val(GPIO0_A::RF_GPIO0_SRC_DIO_2),
            3 => Val(GPIO0_A::RF_GPIO0_SRC_DIO_3),
            4 => Val(GPIO0_A::RF_GPIO0_SRC_DIO_4),
            5 => Val(GPIO0_A::RF_GPIO0_SRC_DIO_5),
            6 => Val(GPIO0_A::RF_GPIO0_SRC_DIO_6),
            7 => Val(GPIO0_A::RF_GPIO0_SRC_DIO_7),
            8 => Val(GPIO0_A::RF_GPIO0_SRC_DIO_8),
            9 => Val(GPIO0_A::RF_GPIO0_SRC_DIO_9),
            10 => Val(GPIO0_A::RF_GPIO0_SRC_DIO_10),
            11 => Val(GPIO0_A::RF_GPIO0_SRC_DIO_11),
            12 => Val(GPIO0_A::RF_GPIO0_SRC_DIO_12),
            13 => Val(GPIO0_A::RF_GPIO0_SRC_DIO_13),
            14 => Val(GPIO0_A::RF_GPIO0_SRC_DIO_14),
            15 => Val(GPIO0_A::RF_GPIO0_SRC_DIO_15),
            16 => Val(GPIO0_A::RF_GPIO0_SRC_CONST_LOW),
            17 => Val(GPIO0_A::RF_GPIO0_SRC_CONST_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RF_GPIO0_SRC_DIO_0`"]
    #[inline(always)]
    pub fn is_rf_gpio0_src_dio_0(&self) -> bool {
        *self == GPIO0_A::RF_GPIO0_SRC_DIO_0
    }
    #[doc = "Checks if the value of the field is `RF_GPIO0_SRC_DIO_1`"]
    #[inline(always)]
    pub fn is_rf_gpio0_src_dio_1(&self) -> bool {
        *self == GPIO0_A::RF_GPIO0_SRC_DIO_1
    }
    #[doc = "Checks if the value of the field is `RF_GPIO0_SRC_DIO_2`"]
    #[inline(always)]
    pub fn is_rf_gpio0_src_dio_2(&self) -> bool {
        *self == GPIO0_A::RF_GPIO0_SRC_DIO_2
    }
    #[doc = "Checks if the value of the field is `RF_GPIO0_SRC_DIO_3`"]
    #[inline(always)]
    pub fn is_rf_gpio0_src_dio_3(&self) -> bool {
        *self == GPIO0_A::RF_GPIO0_SRC_DIO_3
    }
    #[doc = "Checks if the value of the field is `RF_GPIO0_SRC_DIO_4`"]
    #[inline(always)]
    pub fn is_rf_gpio0_src_dio_4(&self) -> bool {
        *self == GPIO0_A::RF_GPIO0_SRC_DIO_4
    }
    #[doc = "Checks if the value of the field is `RF_GPIO0_SRC_DIO_5`"]
    #[inline(always)]
    pub fn is_rf_gpio0_src_dio_5(&self) -> bool {
        *self == GPIO0_A::RF_GPIO0_SRC_DIO_5
    }
    #[doc = "Checks if the value of the field is `RF_GPIO0_SRC_DIO_6`"]
    #[inline(always)]
    pub fn is_rf_gpio0_src_dio_6(&self) -> bool {
        *self == GPIO0_A::RF_GPIO0_SRC_DIO_6
    }
    #[doc = "Checks if the value of the field is `RF_GPIO0_SRC_DIO_7`"]
    #[inline(always)]
    pub fn is_rf_gpio0_src_dio_7(&self) -> bool {
        *self == GPIO0_A::RF_GPIO0_SRC_DIO_7
    }
    #[doc = "Checks if the value of the field is `RF_GPIO0_SRC_DIO_8`"]
    #[inline(always)]
    pub fn is_rf_gpio0_src_dio_8(&self) -> bool {
        *self == GPIO0_A::RF_GPIO0_SRC_DIO_8
    }
    #[doc = "Checks if the value of the field is `RF_GPIO0_SRC_DIO_9`"]
    #[inline(always)]
    pub fn is_rf_gpio0_src_dio_9(&self) -> bool {
        *self == GPIO0_A::RF_GPIO0_SRC_DIO_9
    }
    #[doc = "Checks if the value of the field is `RF_GPIO0_SRC_DIO_10`"]
    #[inline(always)]
    pub fn is_rf_gpio0_src_dio_10(&self) -> bool {
        *self == GPIO0_A::RF_GPIO0_SRC_DIO_10
    }
    #[doc = "Checks if the value of the field is `RF_GPIO0_SRC_DIO_11`"]
    #[inline(always)]
    pub fn is_rf_gpio0_src_dio_11(&self) -> bool {
        *self == GPIO0_A::RF_GPIO0_SRC_DIO_11
    }
    #[doc = "Checks if the value of the field is `RF_GPIO0_SRC_DIO_12`"]
    #[inline(always)]
    pub fn is_rf_gpio0_src_dio_12(&self) -> bool {
        *self == GPIO0_A::RF_GPIO0_SRC_DIO_12
    }
    #[doc = "Checks if the value of the field is `RF_GPIO0_SRC_DIO_13`"]
    #[inline(always)]
    pub fn is_rf_gpio0_src_dio_13(&self) -> bool {
        *self == GPIO0_A::RF_GPIO0_SRC_DIO_13
    }
    #[doc = "Checks if the value of the field is `RF_GPIO0_SRC_DIO_14`"]
    #[inline(always)]
    pub fn is_rf_gpio0_src_dio_14(&self) -> bool {
        *self == GPIO0_A::RF_GPIO0_SRC_DIO_14
    }
    #[doc = "Checks if the value of the field is `RF_GPIO0_SRC_DIO_15`"]
    #[inline(always)]
    pub fn is_rf_gpio0_src_dio_15(&self) -> bool {
        *self == GPIO0_A::RF_GPIO0_SRC_DIO_15
    }
    #[doc = "Checks if the value of the field is `RF_GPIO0_SRC_CONST_LOW`"]
    #[inline(always)]
    pub fn is_rf_gpio0_src_const_low(&self) -> bool {
        *self == GPIO0_A::RF_GPIO0_SRC_CONST_LOW
    }
    #[doc = "Checks if the value of the field is `RF_GPIO0_SRC_CONST_HIGH`"]
    #[inline(always)]
    pub fn is_rf_gpio0_src_const_high(&self) -> bool {
        *self == GPIO0_A::RF_GPIO0_SRC_CONST_HIGH
    }
}
#[doc = "Write proxy for field `GPIO0`"]
pub struct GPIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio0_src_dio_0(self) -> &'a mut W {
        self.variant(GPIO0_A::RF_GPIO0_SRC_DIO_0)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio0_src_dio_1(self) -> &'a mut W {
        self.variant(GPIO0_A::RF_GPIO0_SRC_DIO_1)
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio0_src_dio_2(self) -> &'a mut W {
        self.variant(GPIO0_A::RF_GPIO0_SRC_DIO_2)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio0_src_dio_3(self) -> &'a mut W {
        self.variant(GPIO0_A::RF_GPIO0_SRC_DIO_3)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio0_src_dio_4(self) -> &'a mut W {
        self.variant(GPIO0_A::RF_GPIO0_SRC_DIO_4)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio0_src_dio_5(self) -> &'a mut W {
        self.variant(GPIO0_A::RF_GPIO0_SRC_DIO_5)
    }
    #[doc = "Select DIO\\[6\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio0_src_dio_6(self) -> &'a mut W {
        self.variant(GPIO0_A::RF_GPIO0_SRC_DIO_6)
    }
    #[doc = "Select DIO\\[7\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio0_src_dio_7(self) -> &'a mut W {
        self.variant(GPIO0_A::RF_GPIO0_SRC_DIO_7)
    }
    #[doc = "Select DIO\\[8\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio0_src_dio_8(self) -> &'a mut W {
        self.variant(GPIO0_A::RF_GPIO0_SRC_DIO_8)
    }
    #[doc = "Select DIO\\[9\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio0_src_dio_9(self) -> &'a mut W {
        self.variant(GPIO0_A::RF_GPIO0_SRC_DIO_9)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio0_src_dio_10(self) -> &'a mut W {
        self.variant(GPIO0_A::RF_GPIO0_SRC_DIO_10)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio0_src_dio_11(self) -> &'a mut W {
        self.variant(GPIO0_A::RF_GPIO0_SRC_DIO_11)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio0_src_dio_12(self) -> &'a mut W {
        self.variant(GPIO0_A::RF_GPIO0_SRC_DIO_12)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio0_src_dio_13(self) -> &'a mut W {
        self.variant(GPIO0_A::RF_GPIO0_SRC_DIO_13)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio0_src_dio_14(self) -> &'a mut W {
        self.variant(GPIO0_A::RF_GPIO0_SRC_DIO_14)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn rf_gpio0_src_dio_15(self) -> &'a mut W {
        self.variant(GPIO0_A::RF_GPIO0_SRC_DIO_15)
    }
    #[doc = "Select constant low as source"]
    #[inline(always)]
    pub fn rf_gpio0_src_const_low(self) -> &'a mut W {
        self.variant(GPIO0_A::RF_GPIO0_SRC_CONST_LOW)
    }
    #[doc = "Select constant high as source"]
    #[inline(always)]
    pub fn rf_gpio0_src_const_high(self) -> &'a mut W {
        self.variant(GPIO0_A::RF_GPIO0_SRC_CONST_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:28 - RF front-end GPIO3 input selection"]
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO3_R {
        GPIO3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - RF front-end GPIO2 input selection"]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO2_R {
        GPIO2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - RF front-end GPIO1 input selection"]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - RF front-end GPIO0 input selection"]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28 - RF front-end GPIO3 input selection"]
    #[inline(always)]
    pub fn gpio3(&mut self) -> GPIO3_W {
        GPIO3_W { w: self }
    }
    #[doc = "Bits 16:20 - RF front-end GPIO2 input selection"]
    #[inline(always)]
    pub fn gpio2(&mut self) -> GPIO2_W {
        GPIO2_W { w: self }
    }
    #[doc = "Bits 8:12 - RF front-end GPIO1 input selection"]
    #[inline(always)]
    pub fn gpio1(&mut self) -> GPIO1_W {
        GPIO1_W { w: self }
    }
    #[doc = "Bits 0:4 - RF front-end GPIO0 input selection"]
    #[inline(always)]
    pub fn gpio0(&mut self) -> GPIO0_W {
        GPIO0_W { w: self }
    }
}
