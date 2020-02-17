#[doc = "Reader of register DIO_I2C_SRC"]
pub type R = crate::R<u32, super::DIO_I2C_SRC>;
#[doc = "Writer for register DIO_I2C_SRC"]
pub type W = crate::W<u32, super::DIO_I2C_SRC>;
#[doc = "Register DIO_I2C_SRC `reset()`'s with value 0x1111"]
impl crate::ResetValue for super::DIO_I2C_SRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1111
    }
}
#[doc = "SDA input selection\n\nValue on reset: 17"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDA_A {
    #[doc = "0: Select DIO\\[0\\]
as source"]
    SDA_SRC_DIO_0 = 0,
    #[doc = "1: Select DIO\\[1\\]
as source"]
    SDA_SRC_DIO_1 = 1,
    #[doc = "2: Select DIO\\[2\\]
as source"]
    SDA_SRC_DIO_2 = 2,
    #[doc = "3: Select DIO\\[3\\]
as source"]
    SDA_SRC_DIO_3 = 3,
    #[doc = "4: Select DIO\\[4\\]
as source"]
    SDA_SRC_DIO_4 = 4,
    #[doc = "5: Select DIO\\[5\\]
as source"]
    SDA_SRC_DIO_5 = 5,
    #[doc = "6: Select DIO\\[6\\]
as source"]
    SDA_SRC_DIO_6 = 6,
    #[doc = "7: Select DIO\\[7\\]
as source"]
    SDA_SRC_DIO_7 = 7,
    #[doc = "8: Select DIO\\[8\\]
as source"]
    SDA_SRC_DIO_8 = 8,
    #[doc = "9: Select DIO\\[9\\]
as source"]
    SDA_SRC_DIO_9 = 9,
    #[doc = "10: Select DIO\\[10\\]
as source"]
    SDA_SRC_DIO_10 = 10,
    #[doc = "11: Select DIO\\[11\\]
as source"]
    SDA_SRC_DIO_11 = 11,
    #[doc = "12: Select DIO\\[12\\]
as source"]
    SDA_SRC_DIO_12 = 12,
    #[doc = "13: Select DIO\\[13\\]
as source"]
    SDA_SRC_DIO_13 = 13,
    #[doc = "14: Select DIO\\[14\\]
as source"]
    SDA_SRC_DIO_14 = 14,
    #[doc = "15: Select DIO\\[15\\]
as source"]
    SDA_SRC_DIO_15 = 15,
    #[doc = "16: Select constant low as source"]
    SDA_SRC_CONST_LOW = 16,
    #[doc = "17: Select constant high as source"]
    SDA_SRC_CONST_HIGH = 17,
}
impl From<SDA_A> for u8 {
    #[inline(always)]
    fn from(variant: SDA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SDA`"]
pub type SDA_R = crate::R<u8, SDA_A>;
impl SDA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SDA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SDA_A::SDA_SRC_DIO_0),
            1 => Val(SDA_A::SDA_SRC_DIO_1),
            2 => Val(SDA_A::SDA_SRC_DIO_2),
            3 => Val(SDA_A::SDA_SRC_DIO_3),
            4 => Val(SDA_A::SDA_SRC_DIO_4),
            5 => Val(SDA_A::SDA_SRC_DIO_5),
            6 => Val(SDA_A::SDA_SRC_DIO_6),
            7 => Val(SDA_A::SDA_SRC_DIO_7),
            8 => Val(SDA_A::SDA_SRC_DIO_8),
            9 => Val(SDA_A::SDA_SRC_DIO_9),
            10 => Val(SDA_A::SDA_SRC_DIO_10),
            11 => Val(SDA_A::SDA_SRC_DIO_11),
            12 => Val(SDA_A::SDA_SRC_DIO_12),
            13 => Val(SDA_A::SDA_SRC_DIO_13),
            14 => Val(SDA_A::SDA_SRC_DIO_14),
            15 => Val(SDA_A::SDA_SRC_DIO_15),
            16 => Val(SDA_A::SDA_SRC_CONST_LOW),
            17 => Val(SDA_A::SDA_SRC_CONST_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SDA_SRC_DIO_0`"]
    #[inline(always)]
    pub fn is_sda_src_dio_0(&self) -> bool {
        *self == SDA_A::SDA_SRC_DIO_0
    }
    #[doc = "Checks if the value of the field is `SDA_SRC_DIO_1`"]
    #[inline(always)]
    pub fn is_sda_src_dio_1(&self) -> bool {
        *self == SDA_A::SDA_SRC_DIO_1
    }
    #[doc = "Checks if the value of the field is `SDA_SRC_DIO_2`"]
    #[inline(always)]
    pub fn is_sda_src_dio_2(&self) -> bool {
        *self == SDA_A::SDA_SRC_DIO_2
    }
    #[doc = "Checks if the value of the field is `SDA_SRC_DIO_3`"]
    #[inline(always)]
    pub fn is_sda_src_dio_3(&self) -> bool {
        *self == SDA_A::SDA_SRC_DIO_3
    }
    #[doc = "Checks if the value of the field is `SDA_SRC_DIO_4`"]
    #[inline(always)]
    pub fn is_sda_src_dio_4(&self) -> bool {
        *self == SDA_A::SDA_SRC_DIO_4
    }
    #[doc = "Checks if the value of the field is `SDA_SRC_DIO_5`"]
    #[inline(always)]
    pub fn is_sda_src_dio_5(&self) -> bool {
        *self == SDA_A::SDA_SRC_DIO_5
    }
    #[doc = "Checks if the value of the field is `SDA_SRC_DIO_6`"]
    #[inline(always)]
    pub fn is_sda_src_dio_6(&self) -> bool {
        *self == SDA_A::SDA_SRC_DIO_6
    }
    #[doc = "Checks if the value of the field is `SDA_SRC_DIO_7`"]
    #[inline(always)]
    pub fn is_sda_src_dio_7(&self) -> bool {
        *self == SDA_A::SDA_SRC_DIO_7
    }
    #[doc = "Checks if the value of the field is `SDA_SRC_DIO_8`"]
    #[inline(always)]
    pub fn is_sda_src_dio_8(&self) -> bool {
        *self == SDA_A::SDA_SRC_DIO_8
    }
    #[doc = "Checks if the value of the field is `SDA_SRC_DIO_9`"]
    #[inline(always)]
    pub fn is_sda_src_dio_9(&self) -> bool {
        *self == SDA_A::SDA_SRC_DIO_9
    }
    #[doc = "Checks if the value of the field is `SDA_SRC_DIO_10`"]
    #[inline(always)]
    pub fn is_sda_src_dio_10(&self) -> bool {
        *self == SDA_A::SDA_SRC_DIO_10
    }
    #[doc = "Checks if the value of the field is `SDA_SRC_DIO_11`"]
    #[inline(always)]
    pub fn is_sda_src_dio_11(&self) -> bool {
        *self == SDA_A::SDA_SRC_DIO_11
    }
    #[doc = "Checks if the value of the field is `SDA_SRC_DIO_12`"]
    #[inline(always)]
    pub fn is_sda_src_dio_12(&self) -> bool {
        *self == SDA_A::SDA_SRC_DIO_12
    }
    #[doc = "Checks if the value of the field is `SDA_SRC_DIO_13`"]
    #[inline(always)]
    pub fn is_sda_src_dio_13(&self) -> bool {
        *self == SDA_A::SDA_SRC_DIO_13
    }
    #[doc = "Checks if the value of the field is `SDA_SRC_DIO_14`"]
    #[inline(always)]
    pub fn is_sda_src_dio_14(&self) -> bool {
        *self == SDA_A::SDA_SRC_DIO_14
    }
    #[doc = "Checks if the value of the field is `SDA_SRC_DIO_15`"]
    #[inline(always)]
    pub fn is_sda_src_dio_15(&self) -> bool {
        *self == SDA_A::SDA_SRC_DIO_15
    }
    #[doc = "Checks if the value of the field is `SDA_SRC_CONST_LOW`"]
    #[inline(always)]
    pub fn is_sda_src_const_low(&self) -> bool {
        *self == SDA_A::SDA_SRC_CONST_LOW
    }
    #[doc = "Checks if the value of the field is `SDA_SRC_CONST_HIGH`"]
    #[inline(always)]
    pub fn is_sda_src_const_high(&self) -> bool {
        *self == SDA_A::SDA_SRC_CONST_HIGH
    }
}
#[doc = "Write proxy for field `SDA`"]
pub struct SDA_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn sda_src_dio_0(self) -> &'a mut W {
        self.variant(SDA_A::SDA_SRC_DIO_0)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn sda_src_dio_1(self) -> &'a mut W {
        self.variant(SDA_A::SDA_SRC_DIO_1)
    }
    #[doc = "Select DIO\\[2\\]
as source"]
    #[inline(always)]
    pub fn sda_src_dio_2(self) -> &'a mut W {
        self.variant(SDA_A::SDA_SRC_DIO_2)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn sda_src_dio_3(self) -> &'a mut W {
        self.variant(SDA_A::SDA_SRC_DIO_3)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn sda_src_dio_4(self) -> &'a mut W {
        self.variant(SDA_A::SDA_SRC_DIO_4)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn sda_src_dio_5(self) -> &'a mut W {
        self.variant(SDA_A::SDA_SRC_DIO_5)
    }
    #[doc = "Select DIO\\[6\\]
as source"]
    #[inline(always)]
    pub fn sda_src_dio_6(self) -> &'a mut W {
        self.variant(SDA_A::SDA_SRC_DIO_6)
    }
    #[doc = "Select DIO\\[7\\]
as source"]
    #[inline(always)]
    pub fn sda_src_dio_7(self) -> &'a mut W {
        self.variant(SDA_A::SDA_SRC_DIO_7)
    }
    #[doc = "Select DIO\\[8\\]
as source"]
    #[inline(always)]
    pub fn sda_src_dio_8(self) -> &'a mut W {
        self.variant(SDA_A::SDA_SRC_DIO_8)
    }
    #[doc = "Select DIO\\[9\\]
as source"]
    #[inline(always)]
    pub fn sda_src_dio_9(self) -> &'a mut W {
        self.variant(SDA_A::SDA_SRC_DIO_9)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn sda_src_dio_10(self) -> &'a mut W {
        self.variant(SDA_A::SDA_SRC_DIO_10)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn sda_src_dio_11(self) -> &'a mut W {
        self.variant(SDA_A::SDA_SRC_DIO_11)
    }
    #[doc = "Select DIO\\[12\\]
as source"]
    #[inline(always)]
    pub fn sda_src_dio_12(self) -> &'a mut W {
        self.variant(SDA_A::SDA_SRC_DIO_12)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn sda_src_dio_13(self) -> &'a mut W {
        self.variant(SDA_A::SDA_SRC_DIO_13)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn sda_src_dio_14(self) -> &'a mut W {
        self.variant(SDA_A::SDA_SRC_DIO_14)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn sda_src_dio_15(self) -> &'a mut W {
        self.variant(SDA_A::SDA_SRC_DIO_15)
    }
    #[doc = "Select constant low as source"]
    #[inline(always)]
    pub fn sda_src_const_low(self) -> &'a mut W {
        self.variant(SDA_A::SDA_SRC_CONST_LOW)
    }
    #[doc = "Select constant high as source"]
    #[inline(always)]
    pub fn sda_src_const_high(self) -> &'a mut W {
        self.variant(SDA_A::SDA_SRC_CONST_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "SCL input selection\n\nValue on reset: 17"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCL_A {
    #[doc = "0: Select DIO\\[0\\]
as source"]
    SCL_SRC_DIO_0 = 0,
    #[doc = "1: Select DIO\\[1\\]
as source"]
    SCL_SRC_DIO_1 = 1,
    #[doc = "2: Select DIO\\[2\\]
as source"]
    SCL_SRC_DIO_2 = 2,
    #[doc = "3: Select DIO\\[3\\]
as source"]
    SCL_SRC_DIO_3 = 3,
    #[doc = "4: Select DIO\\[4\\]
as source"]
    SCL_SRC_DIO_4 = 4,
    #[doc = "5: Select DIO\\[5\\]
as source"]
    SCL_SRC_DIO_5 = 5,
    #[doc = "6: Select DIO\\[6\\]
as source"]
    SCL_SRC_DIO_6 = 6,
    #[doc = "7: Select DIO\\[7\\]
as source"]
    SCL_SRC_DIO_7 = 7,
    #[doc = "8: Select DIO\\[8\\]
as source"]
    SCL_SRC_DIO_8 = 8,
    #[doc = "9: Select DIO\\[9\\]
as source"]
    SCL_SRC_DIO_9 = 9,
    #[doc = "10: Select DIO\\[10\\]
as source"]
    SCL_SRC_DIO_10 = 10,
    #[doc = "11: Select DIO\\[11\\]
as source"]
    SCL_SRC_DIO_11 = 11,
    #[doc = "12: Select DIO\\[12\\]
as source"]
    SCL_SRC_DIO_12 = 12,
    #[doc = "13: Select DIO\\[13\\]
as source"]
    SCL_SRC_DIO_13 = 13,
    #[doc = "14: Select DIO\\[14\\]
as source"]
    SCL_SRC_DIO_14 = 14,
    #[doc = "15: Select DIO\\[15\\]
as source"]
    SCL_SRC_DIO_15 = 15,
    #[doc = "16: Select constant low as source"]
    SCL_SRC_CONST_LOW = 16,
    #[doc = "17: Select constant high as source"]
    SCL_SRC_CONST_HIGH = 17,
}
impl From<SCL_A> for u8 {
    #[inline(always)]
    fn from(variant: SCL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SCL`"]
pub type SCL_R = crate::R<u8, SCL_A>;
impl SCL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SCL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SCL_A::SCL_SRC_DIO_0),
            1 => Val(SCL_A::SCL_SRC_DIO_1),
            2 => Val(SCL_A::SCL_SRC_DIO_2),
            3 => Val(SCL_A::SCL_SRC_DIO_3),
            4 => Val(SCL_A::SCL_SRC_DIO_4),
            5 => Val(SCL_A::SCL_SRC_DIO_5),
            6 => Val(SCL_A::SCL_SRC_DIO_6),
            7 => Val(SCL_A::SCL_SRC_DIO_7),
            8 => Val(SCL_A::SCL_SRC_DIO_8),
            9 => Val(SCL_A::SCL_SRC_DIO_9),
            10 => Val(SCL_A::SCL_SRC_DIO_10),
            11 => Val(SCL_A::SCL_SRC_DIO_11),
            12 => Val(SCL_A::SCL_SRC_DIO_12),
            13 => Val(SCL_A::SCL_SRC_DIO_13),
            14 => Val(SCL_A::SCL_SRC_DIO_14),
            15 => Val(SCL_A::SCL_SRC_DIO_15),
            16 => Val(SCL_A::SCL_SRC_CONST_LOW),
            17 => Val(SCL_A::SCL_SRC_CONST_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SCL_SRC_DIO_0`"]
    #[inline(always)]
    pub fn is_scl_src_dio_0(&self) -> bool {
        *self == SCL_A::SCL_SRC_DIO_0
    }
    #[doc = "Checks if the value of the field is `SCL_SRC_DIO_1`"]
    #[inline(always)]
    pub fn is_scl_src_dio_1(&self) -> bool {
        *self == SCL_A::SCL_SRC_DIO_1
    }
    #[doc = "Checks if the value of the field is `SCL_SRC_DIO_2`"]
    #[inline(always)]
    pub fn is_scl_src_dio_2(&self) -> bool {
        *self == SCL_A::SCL_SRC_DIO_2
    }
    #[doc = "Checks if the value of the field is `SCL_SRC_DIO_3`"]
    #[inline(always)]
    pub fn is_scl_src_dio_3(&self) -> bool {
        *self == SCL_A::SCL_SRC_DIO_3
    }
    #[doc = "Checks if the value of the field is `SCL_SRC_DIO_4`"]
    #[inline(always)]
    pub fn is_scl_src_dio_4(&self) -> bool {
        *self == SCL_A::SCL_SRC_DIO_4
    }
    #[doc = "Checks if the value of the field is `SCL_SRC_DIO_5`"]
    #[inline(always)]
    pub fn is_scl_src_dio_5(&self) -> bool {
        *self == SCL_A::SCL_SRC_DIO_5
    }
    #[doc = "Checks if the value of the field is `SCL_SRC_DIO_6`"]
    #[inline(always)]
    pub fn is_scl_src_dio_6(&self) -> bool {
        *self == SCL_A::SCL_SRC_DIO_6
    }
    #[doc = "Checks if the value of the field is `SCL_SRC_DIO_7`"]
    #[inline(always)]
    pub fn is_scl_src_dio_7(&self) -> bool {
        *self == SCL_A::SCL_SRC_DIO_7
    }
    #[doc = "Checks if the value of the field is `SCL_SRC_DIO_8`"]
    #[inline(always)]
    pub fn is_scl_src_dio_8(&self) -> bool {
        *self == SCL_A::SCL_SRC_DIO_8
    }
    #[doc = "Checks if the value of the field is `SCL_SRC_DIO_9`"]
    #[inline(always)]
    pub fn is_scl_src_dio_9(&self) -> bool {
        *self == SCL_A::SCL_SRC_DIO_9
    }
    #[doc = "Checks if the value of the field is `SCL_SRC_DIO_10`"]
    #[inline(always)]
    pub fn is_scl_src_dio_10(&self) -> bool {
        *self == SCL_A::SCL_SRC_DIO_10
    }
    #[doc = "Checks if the value of the field is `SCL_SRC_DIO_11`"]
    #[inline(always)]
    pub fn is_scl_src_dio_11(&self) -> bool {
        *self == SCL_A::SCL_SRC_DIO_11
    }
    #[doc = "Checks if the value of the field is `SCL_SRC_DIO_12`"]
    #[inline(always)]
    pub fn is_scl_src_dio_12(&self) -> bool {
        *self == SCL_A::SCL_SRC_DIO_12
    }
    #[doc = "Checks if the value of the field is `SCL_SRC_DIO_13`"]
    #[inline(always)]
    pub fn is_scl_src_dio_13(&self) -> bool {
        *self == SCL_A::SCL_SRC_DIO_13
    }
    #[doc = "Checks if the value of the field is `SCL_SRC_DIO_14`"]
    #[inline(always)]
    pub fn is_scl_src_dio_14(&self) -> bool {
        *self == SCL_A::SCL_SRC_DIO_14
    }
    #[doc = "Checks if the value of the field is `SCL_SRC_DIO_15`"]
    #[inline(always)]
    pub fn is_scl_src_dio_15(&self) -> bool {
        *self == SCL_A::SCL_SRC_DIO_15
    }
    #[doc = "Checks if the value of the field is `SCL_SRC_CONST_LOW`"]
    #[inline(always)]
    pub fn is_scl_src_const_low(&self) -> bool {
        *self == SCL_A::SCL_SRC_CONST_LOW
    }
    #[doc = "Checks if the value of the field is `SCL_SRC_CONST_HIGH`"]
    #[inline(always)]
    pub fn is_scl_src_const_high(&self) -> bool {
        *self == SCL_A::SCL_SRC_CONST_HIGH
    }
}
#[doc = "Write proxy for field `SCL`"]
pub struct SCL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn scl_src_dio_0(self) -> &'a mut W {
        self.variant(SCL_A::SCL_SRC_DIO_0)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn scl_src_dio_1(self) -> &'a mut W {
        self.variant(SCL_A::SCL_SRC_DIO_1)
    }
    #[doc = "Select DIO\\[2\\]
as source"]
    #[inline(always)]
    pub fn scl_src_dio_2(self) -> &'a mut W {
        self.variant(SCL_A::SCL_SRC_DIO_2)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn scl_src_dio_3(self) -> &'a mut W {
        self.variant(SCL_A::SCL_SRC_DIO_3)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn scl_src_dio_4(self) -> &'a mut W {
        self.variant(SCL_A::SCL_SRC_DIO_4)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn scl_src_dio_5(self) -> &'a mut W {
        self.variant(SCL_A::SCL_SRC_DIO_5)
    }
    #[doc = "Select DIO\\[6\\]
as source"]
    #[inline(always)]
    pub fn scl_src_dio_6(self) -> &'a mut W {
        self.variant(SCL_A::SCL_SRC_DIO_6)
    }
    #[doc = "Select DIO\\[7\\]
as source"]
    #[inline(always)]
    pub fn scl_src_dio_7(self) -> &'a mut W {
        self.variant(SCL_A::SCL_SRC_DIO_7)
    }
    #[doc = "Select DIO\\[8\\]
as source"]
    #[inline(always)]
    pub fn scl_src_dio_8(self) -> &'a mut W {
        self.variant(SCL_A::SCL_SRC_DIO_8)
    }
    #[doc = "Select DIO\\[9\\]
as source"]
    #[inline(always)]
    pub fn scl_src_dio_9(self) -> &'a mut W {
        self.variant(SCL_A::SCL_SRC_DIO_9)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn scl_src_dio_10(self) -> &'a mut W {
        self.variant(SCL_A::SCL_SRC_DIO_10)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn scl_src_dio_11(self) -> &'a mut W {
        self.variant(SCL_A::SCL_SRC_DIO_11)
    }
    #[doc = "Select DIO\\[12\\]
as source"]
    #[inline(always)]
    pub fn scl_src_dio_12(self) -> &'a mut W {
        self.variant(SCL_A::SCL_SRC_DIO_12)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn scl_src_dio_13(self) -> &'a mut W {
        self.variant(SCL_A::SCL_SRC_DIO_13)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn scl_src_dio_14(self) -> &'a mut W {
        self.variant(SCL_A::SCL_SRC_DIO_14)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn scl_src_dio_15(self) -> &'a mut W {
        self.variant(SCL_A::SCL_SRC_DIO_15)
    }
    #[doc = "Select constant low as source"]
    #[inline(always)]
    pub fn scl_src_const_low(self) -> &'a mut W {
        self.variant(SCL_A::SCL_SRC_CONST_LOW)
    }
    #[doc = "Select constant high as source"]
    #[inline(always)]
    pub fn scl_src_const_high(self) -> &'a mut W {
        self.variant(SCL_A::SCL_SRC_CONST_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:12 - SDA input selection"]
    #[inline(always)]
    pub fn sda(&self) -> SDA_R {
        SDA_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - SCL input selection"]
    #[inline(always)]
    pub fn scl(&self) -> SCL_R {
        SCL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - SDA input selection"]
    #[inline(always)]
    pub fn sda(&mut self) -> SDA_W {
        SDA_W { w: self }
    }
    #[doc = "Bits 0:4 - SCL input selection"]
    #[inline(always)]
    pub fn scl(&mut self) -> SCL_W {
        SCL_W { w: self }
    }
}
