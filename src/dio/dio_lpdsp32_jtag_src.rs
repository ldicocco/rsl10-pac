#[doc = "Reader of register DIO_LPDSP32_JTAG_SRC"]
pub type R = crate::R<u32, super::DIO_LPDSP32_JTAG_SRC>;
#[doc = "Writer for register DIO_LPDSP32_JTAG_SRC"]
pub type W = crate::W<u32, super::DIO_LPDSP32_JTAG_SRC>;
#[doc = "Register DIO_LPDSP32_JTAG_SRC `reset()`'s with value 0x0011_1111"]
impl crate::ResetValue for super::DIO_LPDSP32_JTAG_SRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0011_1111
    }
}
#[doc = "LPDSP32_TDI input selection\n\nValue on reset: 17"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TDI_A {
    #[doc = "0: Select DIO\\[0\\]
as source"]
    LPDSP32_TDI_SRC_DIO_0 = 0,
    #[doc = "1: Select DIO\\[1\\]
as source"]
    LPDSP32_TDI_SRC_DIO_1 = 1,
    #[doc = "2: Select DIO\\[2\\]
as source"]
    LPDSP32_TDI_SRC_DIO_2 = 2,
    #[doc = "3: Select DIO\\[3\\]
as source"]
    LPDSP32_TDI_SRC_DIO_3 = 3,
    #[doc = "4: Select DIO\\[4\\]
as source"]
    LPDSP32_TDI_SRC_DIO_4 = 4,
    #[doc = "5: Select DIO\\[5\\]
as source"]
    LPDSP32_TDI_SRC_DIO_5 = 5,
    #[doc = "6: Select DIO\\[6\\]
as source"]
    LPDSP32_TDI_SRC_DIO_6 = 6,
    #[doc = "7: Select DIO\\[7\\]
as source"]
    LPDSP32_TDI_SRC_DIO_7 = 7,
    #[doc = "8: Select DIO\\[8\\]
as source"]
    LPDSP32_TDI_SRC_DIO_8 = 8,
    #[doc = "9: Select DIO\\[9\\]
as source"]
    LPDSP32_TDI_SRC_DIO_9 = 9,
    #[doc = "10: Select DIO\\[10\\]
as source"]
    LPDSP32_TDI_SRC_DIO_10 = 10,
    #[doc = "11: Select DIO\\[11\\]
as source"]
    LPDSP32_TDI_SRC_DIO_11 = 11,
    #[doc = "12: Select DIO\\[12\\]
as source"]
    LPDSP32_TDI_SRC_DIO_12 = 12,
    #[doc = "13: Select DIO\\[13\\]
as source"]
    LPDSP32_TDI_SRC_DIO_13 = 13,
    #[doc = "14: Select DIO\\[14\\]
as source"]
    LPDSP32_TDI_SRC_DIO_14 = 14,
    #[doc = "15: Select DIO\\[15\\]
as source"]
    LPDSP32_TDI_SRC_DIO_15 = 15,
    #[doc = "16: Select constant low as source"]
    LPDSP32_TDI_SRC_CONST_LOW = 16,
    #[doc = "17: Select constant high as source"]
    LPDSP32_TDI_SRC_CONST_HIGH = 17,
}
impl From<TDI_A> for u8 {
    #[inline(always)]
    fn from(variant: TDI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TDI`"]
pub type TDI_R = crate::R<u8, TDI_A>;
impl TDI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TDI_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TDI_A::LPDSP32_TDI_SRC_DIO_0),
            1 => Val(TDI_A::LPDSP32_TDI_SRC_DIO_1),
            2 => Val(TDI_A::LPDSP32_TDI_SRC_DIO_2),
            3 => Val(TDI_A::LPDSP32_TDI_SRC_DIO_3),
            4 => Val(TDI_A::LPDSP32_TDI_SRC_DIO_4),
            5 => Val(TDI_A::LPDSP32_TDI_SRC_DIO_5),
            6 => Val(TDI_A::LPDSP32_TDI_SRC_DIO_6),
            7 => Val(TDI_A::LPDSP32_TDI_SRC_DIO_7),
            8 => Val(TDI_A::LPDSP32_TDI_SRC_DIO_8),
            9 => Val(TDI_A::LPDSP32_TDI_SRC_DIO_9),
            10 => Val(TDI_A::LPDSP32_TDI_SRC_DIO_10),
            11 => Val(TDI_A::LPDSP32_TDI_SRC_DIO_11),
            12 => Val(TDI_A::LPDSP32_TDI_SRC_DIO_12),
            13 => Val(TDI_A::LPDSP32_TDI_SRC_DIO_13),
            14 => Val(TDI_A::LPDSP32_TDI_SRC_DIO_14),
            15 => Val(TDI_A::LPDSP32_TDI_SRC_DIO_15),
            16 => Val(TDI_A::LPDSP32_TDI_SRC_CONST_LOW),
            17 => Val(TDI_A::LPDSP32_TDI_SRC_CONST_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TDI_SRC_DIO_0`"]
    #[inline(always)]
    pub fn is_lpdsp32_tdi_src_dio_0(&self) -> bool {
        *self == TDI_A::LPDSP32_TDI_SRC_DIO_0
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TDI_SRC_DIO_1`"]
    #[inline(always)]
    pub fn is_lpdsp32_tdi_src_dio_1(&self) -> bool {
        *self == TDI_A::LPDSP32_TDI_SRC_DIO_1
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TDI_SRC_DIO_2`"]
    #[inline(always)]
    pub fn is_lpdsp32_tdi_src_dio_2(&self) -> bool {
        *self == TDI_A::LPDSP32_TDI_SRC_DIO_2
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TDI_SRC_DIO_3`"]
    #[inline(always)]
    pub fn is_lpdsp32_tdi_src_dio_3(&self) -> bool {
        *self == TDI_A::LPDSP32_TDI_SRC_DIO_3
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TDI_SRC_DIO_4`"]
    #[inline(always)]
    pub fn is_lpdsp32_tdi_src_dio_4(&self) -> bool {
        *self == TDI_A::LPDSP32_TDI_SRC_DIO_4
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TDI_SRC_DIO_5`"]
    #[inline(always)]
    pub fn is_lpdsp32_tdi_src_dio_5(&self) -> bool {
        *self == TDI_A::LPDSP32_TDI_SRC_DIO_5
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TDI_SRC_DIO_6`"]
    #[inline(always)]
    pub fn is_lpdsp32_tdi_src_dio_6(&self) -> bool {
        *self == TDI_A::LPDSP32_TDI_SRC_DIO_6
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TDI_SRC_DIO_7`"]
    #[inline(always)]
    pub fn is_lpdsp32_tdi_src_dio_7(&self) -> bool {
        *self == TDI_A::LPDSP32_TDI_SRC_DIO_7
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TDI_SRC_DIO_8`"]
    #[inline(always)]
    pub fn is_lpdsp32_tdi_src_dio_8(&self) -> bool {
        *self == TDI_A::LPDSP32_TDI_SRC_DIO_8
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TDI_SRC_DIO_9`"]
    #[inline(always)]
    pub fn is_lpdsp32_tdi_src_dio_9(&self) -> bool {
        *self == TDI_A::LPDSP32_TDI_SRC_DIO_9
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TDI_SRC_DIO_10`"]
    #[inline(always)]
    pub fn is_lpdsp32_tdi_src_dio_10(&self) -> bool {
        *self == TDI_A::LPDSP32_TDI_SRC_DIO_10
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TDI_SRC_DIO_11`"]
    #[inline(always)]
    pub fn is_lpdsp32_tdi_src_dio_11(&self) -> bool {
        *self == TDI_A::LPDSP32_TDI_SRC_DIO_11
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TDI_SRC_DIO_12`"]
    #[inline(always)]
    pub fn is_lpdsp32_tdi_src_dio_12(&self) -> bool {
        *self == TDI_A::LPDSP32_TDI_SRC_DIO_12
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TDI_SRC_DIO_13`"]
    #[inline(always)]
    pub fn is_lpdsp32_tdi_src_dio_13(&self) -> bool {
        *self == TDI_A::LPDSP32_TDI_SRC_DIO_13
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TDI_SRC_DIO_14`"]
    #[inline(always)]
    pub fn is_lpdsp32_tdi_src_dio_14(&self) -> bool {
        *self == TDI_A::LPDSP32_TDI_SRC_DIO_14
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TDI_SRC_DIO_15`"]
    #[inline(always)]
    pub fn is_lpdsp32_tdi_src_dio_15(&self) -> bool {
        *self == TDI_A::LPDSP32_TDI_SRC_DIO_15
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TDI_SRC_CONST_LOW`"]
    #[inline(always)]
    pub fn is_lpdsp32_tdi_src_const_low(&self) -> bool {
        *self == TDI_A::LPDSP32_TDI_SRC_CONST_LOW
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TDI_SRC_CONST_HIGH`"]
    #[inline(always)]
    pub fn is_lpdsp32_tdi_src_const_high(&self) -> bool {
        *self == TDI_A::LPDSP32_TDI_SRC_CONST_HIGH
    }
}
#[doc = "Write proxy for field `TDI`"]
pub struct TDI_W<'a> {
    w: &'a mut W,
}
impl<'a> TDI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tdi_src_dio_0(self) -> &'a mut W {
        self.variant(TDI_A::LPDSP32_TDI_SRC_DIO_0)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tdi_src_dio_1(self) -> &'a mut W {
        self.variant(TDI_A::LPDSP32_TDI_SRC_DIO_1)
    }
    #[doc = "Select DIO\\[2\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tdi_src_dio_2(self) -> &'a mut W {
        self.variant(TDI_A::LPDSP32_TDI_SRC_DIO_2)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tdi_src_dio_3(self) -> &'a mut W {
        self.variant(TDI_A::LPDSP32_TDI_SRC_DIO_3)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tdi_src_dio_4(self) -> &'a mut W {
        self.variant(TDI_A::LPDSP32_TDI_SRC_DIO_4)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tdi_src_dio_5(self) -> &'a mut W {
        self.variant(TDI_A::LPDSP32_TDI_SRC_DIO_5)
    }
    #[doc = "Select DIO\\[6\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tdi_src_dio_6(self) -> &'a mut W {
        self.variant(TDI_A::LPDSP32_TDI_SRC_DIO_6)
    }
    #[doc = "Select DIO\\[7\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tdi_src_dio_7(self) -> &'a mut W {
        self.variant(TDI_A::LPDSP32_TDI_SRC_DIO_7)
    }
    #[doc = "Select DIO\\[8\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tdi_src_dio_8(self) -> &'a mut W {
        self.variant(TDI_A::LPDSP32_TDI_SRC_DIO_8)
    }
    #[doc = "Select DIO\\[9\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tdi_src_dio_9(self) -> &'a mut W {
        self.variant(TDI_A::LPDSP32_TDI_SRC_DIO_9)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tdi_src_dio_10(self) -> &'a mut W {
        self.variant(TDI_A::LPDSP32_TDI_SRC_DIO_10)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tdi_src_dio_11(self) -> &'a mut W {
        self.variant(TDI_A::LPDSP32_TDI_SRC_DIO_11)
    }
    #[doc = "Select DIO\\[12\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tdi_src_dio_12(self) -> &'a mut W {
        self.variant(TDI_A::LPDSP32_TDI_SRC_DIO_12)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tdi_src_dio_13(self) -> &'a mut W {
        self.variant(TDI_A::LPDSP32_TDI_SRC_DIO_13)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tdi_src_dio_14(self) -> &'a mut W {
        self.variant(TDI_A::LPDSP32_TDI_SRC_DIO_14)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tdi_src_dio_15(self) -> &'a mut W {
        self.variant(TDI_A::LPDSP32_TDI_SRC_DIO_15)
    }
    #[doc = "Select constant low as source"]
    #[inline(always)]
    pub fn lpdsp32_tdi_src_const_low(self) -> &'a mut W {
        self.variant(TDI_A::LPDSP32_TDI_SRC_CONST_LOW)
    }
    #[doc = "Select constant high as source"]
    #[inline(always)]
    pub fn lpdsp32_tdi_src_const_high(self) -> &'a mut W {
        self.variant(TDI_A::LPDSP32_TDI_SRC_CONST_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "LPDSP32_TMS input selection\n\nValue on reset: 17"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMS_A {
    #[doc = "0: Select DIO\\[0\\]
as source"]
    LPDSP32_TMS_SRC_DIO_0 = 0,
    #[doc = "1: Select DIO\\[1\\]
as source"]
    LPDSP32_TMS_SRC_DIO_1 = 1,
    #[doc = "2: Select DIO\\[2\\]
as source"]
    LPDSP32_TMS_SRC_DIO_2 = 2,
    #[doc = "3: Select DIO\\[3\\]
as source"]
    LPDSP32_TMS_SRC_DIO_3 = 3,
    #[doc = "4: Select DIO\\[4\\]
as source"]
    LPDSP32_TMS_SRC_DIO_4 = 4,
    #[doc = "5: Select DIO\\[5\\]
as source"]
    LPDSP32_TMS_SRC_DIO_5 = 5,
    #[doc = "6: Select DIO\\[6\\]
as source"]
    LPDSP32_TMS_SRC_DIO_6 = 6,
    #[doc = "7: Select DIO\\[7\\]
as source"]
    LPDSP32_TMS_SRC_DIO_7 = 7,
    #[doc = "8: Select DIO\\[8\\]
as source"]
    LPDSP32_TMS_SRC_DIO_8 = 8,
    #[doc = "9: Select DIO\\[9\\]
as source"]
    LPDSP32_TMS_SRC_DIO_9 = 9,
    #[doc = "10: Select DIO\\[10\\]
as source"]
    LPDSP32_TMS_SRC_DIO_10 = 10,
    #[doc = "11: Select DIO\\[11\\]
as source"]
    LPDSP32_TMS_SRC_DIO_11 = 11,
    #[doc = "12: Select DIO\\[12\\]
as source"]
    LPDSP32_TMS_SRC_DIO_12 = 12,
    #[doc = "13: Select DIO\\[13\\]
as source"]
    LPDSP32_TMS_SRC_DIO_13 = 13,
    #[doc = "14: Select DIO\\[14\\]
as source"]
    LPDSP32_TMS_SRC_DIO_14 = 14,
    #[doc = "15: Select DIO\\[15\\]
as source"]
    LPDSP32_TMS_SRC_DIO_15 = 15,
    #[doc = "16: Select constant low as source"]
    LPDSP32_TMS_SRC_CONST_LOW = 16,
    #[doc = "17: Select constant high as source"]
    LPDSP32_TMS_SRC_CONST_HIGH = 17,
}
impl From<TMS_A> for u8 {
    #[inline(always)]
    fn from(variant: TMS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMS`"]
pub type TMS_R = crate::R<u8, TMS_A>;
impl TMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMS_A::LPDSP32_TMS_SRC_DIO_0),
            1 => Val(TMS_A::LPDSP32_TMS_SRC_DIO_1),
            2 => Val(TMS_A::LPDSP32_TMS_SRC_DIO_2),
            3 => Val(TMS_A::LPDSP32_TMS_SRC_DIO_3),
            4 => Val(TMS_A::LPDSP32_TMS_SRC_DIO_4),
            5 => Val(TMS_A::LPDSP32_TMS_SRC_DIO_5),
            6 => Val(TMS_A::LPDSP32_TMS_SRC_DIO_6),
            7 => Val(TMS_A::LPDSP32_TMS_SRC_DIO_7),
            8 => Val(TMS_A::LPDSP32_TMS_SRC_DIO_8),
            9 => Val(TMS_A::LPDSP32_TMS_SRC_DIO_9),
            10 => Val(TMS_A::LPDSP32_TMS_SRC_DIO_10),
            11 => Val(TMS_A::LPDSP32_TMS_SRC_DIO_11),
            12 => Val(TMS_A::LPDSP32_TMS_SRC_DIO_12),
            13 => Val(TMS_A::LPDSP32_TMS_SRC_DIO_13),
            14 => Val(TMS_A::LPDSP32_TMS_SRC_DIO_14),
            15 => Val(TMS_A::LPDSP32_TMS_SRC_DIO_15),
            16 => Val(TMS_A::LPDSP32_TMS_SRC_CONST_LOW),
            17 => Val(TMS_A::LPDSP32_TMS_SRC_CONST_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TMS_SRC_DIO_0`"]
    #[inline(always)]
    pub fn is_lpdsp32_tms_src_dio_0(&self) -> bool {
        *self == TMS_A::LPDSP32_TMS_SRC_DIO_0
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TMS_SRC_DIO_1`"]
    #[inline(always)]
    pub fn is_lpdsp32_tms_src_dio_1(&self) -> bool {
        *self == TMS_A::LPDSP32_TMS_SRC_DIO_1
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TMS_SRC_DIO_2`"]
    #[inline(always)]
    pub fn is_lpdsp32_tms_src_dio_2(&self) -> bool {
        *self == TMS_A::LPDSP32_TMS_SRC_DIO_2
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TMS_SRC_DIO_3`"]
    #[inline(always)]
    pub fn is_lpdsp32_tms_src_dio_3(&self) -> bool {
        *self == TMS_A::LPDSP32_TMS_SRC_DIO_3
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TMS_SRC_DIO_4`"]
    #[inline(always)]
    pub fn is_lpdsp32_tms_src_dio_4(&self) -> bool {
        *self == TMS_A::LPDSP32_TMS_SRC_DIO_4
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TMS_SRC_DIO_5`"]
    #[inline(always)]
    pub fn is_lpdsp32_tms_src_dio_5(&self) -> bool {
        *self == TMS_A::LPDSP32_TMS_SRC_DIO_5
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TMS_SRC_DIO_6`"]
    #[inline(always)]
    pub fn is_lpdsp32_tms_src_dio_6(&self) -> bool {
        *self == TMS_A::LPDSP32_TMS_SRC_DIO_6
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TMS_SRC_DIO_7`"]
    #[inline(always)]
    pub fn is_lpdsp32_tms_src_dio_7(&self) -> bool {
        *self == TMS_A::LPDSP32_TMS_SRC_DIO_7
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TMS_SRC_DIO_8`"]
    #[inline(always)]
    pub fn is_lpdsp32_tms_src_dio_8(&self) -> bool {
        *self == TMS_A::LPDSP32_TMS_SRC_DIO_8
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TMS_SRC_DIO_9`"]
    #[inline(always)]
    pub fn is_lpdsp32_tms_src_dio_9(&self) -> bool {
        *self == TMS_A::LPDSP32_TMS_SRC_DIO_9
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TMS_SRC_DIO_10`"]
    #[inline(always)]
    pub fn is_lpdsp32_tms_src_dio_10(&self) -> bool {
        *self == TMS_A::LPDSP32_TMS_SRC_DIO_10
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TMS_SRC_DIO_11`"]
    #[inline(always)]
    pub fn is_lpdsp32_tms_src_dio_11(&self) -> bool {
        *self == TMS_A::LPDSP32_TMS_SRC_DIO_11
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TMS_SRC_DIO_12`"]
    #[inline(always)]
    pub fn is_lpdsp32_tms_src_dio_12(&self) -> bool {
        *self == TMS_A::LPDSP32_TMS_SRC_DIO_12
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TMS_SRC_DIO_13`"]
    #[inline(always)]
    pub fn is_lpdsp32_tms_src_dio_13(&self) -> bool {
        *self == TMS_A::LPDSP32_TMS_SRC_DIO_13
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TMS_SRC_DIO_14`"]
    #[inline(always)]
    pub fn is_lpdsp32_tms_src_dio_14(&self) -> bool {
        *self == TMS_A::LPDSP32_TMS_SRC_DIO_14
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TMS_SRC_DIO_15`"]
    #[inline(always)]
    pub fn is_lpdsp32_tms_src_dio_15(&self) -> bool {
        *self == TMS_A::LPDSP32_TMS_SRC_DIO_15
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TMS_SRC_CONST_LOW`"]
    #[inline(always)]
    pub fn is_lpdsp32_tms_src_const_low(&self) -> bool {
        *self == TMS_A::LPDSP32_TMS_SRC_CONST_LOW
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TMS_SRC_CONST_HIGH`"]
    #[inline(always)]
    pub fn is_lpdsp32_tms_src_const_high(&self) -> bool {
        *self == TMS_A::LPDSP32_TMS_SRC_CONST_HIGH
    }
}
#[doc = "Write proxy for field `TMS`"]
pub struct TMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tms_src_dio_0(self) -> &'a mut W {
        self.variant(TMS_A::LPDSP32_TMS_SRC_DIO_0)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tms_src_dio_1(self) -> &'a mut W {
        self.variant(TMS_A::LPDSP32_TMS_SRC_DIO_1)
    }
    #[doc = "Select DIO\\[2\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tms_src_dio_2(self) -> &'a mut W {
        self.variant(TMS_A::LPDSP32_TMS_SRC_DIO_2)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tms_src_dio_3(self) -> &'a mut W {
        self.variant(TMS_A::LPDSP32_TMS_SRC_DIO_3)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tms_src_dio_4(self) -> &'a mut W {
        self.variant(TMS_A::LPDSP32_TMS_SRC_DIO_4)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tms_src_dio_5(self) -> &'a mut W {
        self.variant(TMS_A::LPDSP32_TMS_SRC_DIO_5)
    }
    #[doc = "Select DIO\\[6\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tms_src_dio_6(self) -> &'a mut W {
        self.variant(TMS_A::LPDSP32_TMS_SRC_DIO_6)
    }
    #[doc = "Select DIO\\[7\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tms_src_dio_7(self) -> &'a mut W {
        self.variant(TMS_A::LPDSP32_TMS_SRC_DIO_7)
    }
    #[doc = "Select DIO\\[8\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tms_src_dio_8(self) -> &'a mut W {
        self.variant(TMS_A::LPDSP32_TMS_SRC_DIO_8)
    }
    #[doc = "Select DIO\\[9\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tms_src_dio_9(self) -> &'a mut W {
        self.variant(TMS_A::LPDSP32_TMS_SRC_DIO_9)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tms_src_dio_10(self) -> &'a mut W {
        self.variant(TMS_A::LPDSP32_TMS_SRC_DIO_10)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tms_src_dio_11(self) -> &'a mut W {
        self.variant(TMS_A::LPDSP32_TMS_SRC_DIO_11)
    }
    #[doc = "Select DIO\\[12\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tms_src_dio_12(self) -> &'a mut W {
        self.variant(TMS_A::LPDSP32_TMS_SRC_DIO_12)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tms_src_dio_13(self) -> &'a mut W {
        self.variant(TMS_A::LPDSP32_TMS_SRC_DIO_13)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tms_src_dio_14(self) -> &'a mut W {
        self.variant(TMS_A::LPDSP32_TMS_SRC_DIO_14)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tms_src_dio_15(self) -> &'a mut W {
        self.variant(TMS_A::LPDSP32_TMS_SRC_DIO_15)
    }
    #[doc = "Select constant low as source"]
    #[inline(always)]
    pub fn lpdsp32_tms_src_const_low(self) -> &'a mut W {
        self.variant(TMS_A::LPDSP32_TMS_SRC_CONST_LOW)
    }
    #[doc = "Select constant high as source"]
    #[inline(always)]
    pub fn lpdsp32_tms_src_const_high(self) -> &'a mut W {
        self.variant(TMS_A::LPDSP32_TMS_SRC_CONST_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "LPDSP32_TCK input selection\n\nValue on reset: 17"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TCK_A {
    #[doc = "0: Select DIO\\[0\\]
as source"]
    LPDSP32_TCK_SRC_DIO_0 = 0,
    #[doc = "1: Select DIO\\[1\\]
as source"]
    LPDSP32_TCK_SRC_DIO_1 = 1,
    #[doc = "2: Select DIO\\[2\\]
as source"]
    LPDSP32_TCK_SRC_DIO_2 = 2,
    #[doc = "3: Select DIO\\[3\\]
as source"]
    LPDSP32_TCK_SRC_DIO_3 = 3,
    #[doc = "4: Select DIO\\[4\\]
as source"]
    LPDSP32_TCK_SRC_DIO_4 = 4,
    #[doc = "5: Select DIO\\[5\\]
as source"]
    LPDSP32_TCK_SRC_DIO_5 = 5,
    #[doc = "6: Select DIO\\[6\\]
as source"]
    LPDSP32_TCK_SRC_DIO_6 = 6,
    #[doc = "7: Select DIO\\[7\\]
as source"]
    LPDSP32_TCK_SRC_DIO_7 = 7,
    #[doc = "8: Select DIO\\[8\\]
as source"]
    LPDSP32_TCK_SRC_DIO_8 = 8,
    #[doc = "9: Select DIO\\[9\\]
as source"]
    LPDSP32_TCK_SRC_DIO_9 = 9,
    #[doc = "10: Select DIO\\[10\\]
as source"]
    LPDSP32_TCK_SRC_DIO_10 = 10,
    #[doc = "11: Select DIO\\[11\\]
as source"]
    LPDSP32_TCK_SRC_DIO_11 = 11,
    #[doc = "12: Select DIO\\[12\\]
as source"]
    LPDSP32_TCK_SRC_DIO_12 = 12,
    #[doc = "13: Select DIO\\[13\\]
as source"]
    LPDSP32_TCK_SRC_DIO_13 = 13,
    #[doc = "14: Select DIO\\[14\\]
as source"]
    LPDSP32_TCK_SRC_DIO_14 = 14,
    #[doc = "15: Select DIO\\[15\\]
as source"]
    LPDSP32_TCK_SRC_DIO_15 = 15,
    #[doc = "16: Select constant low as source"]
    LPDSP32_TCK_SRC_CONST_LOW = 16,
    #[doc = "17: Select constant high as source"]
    LPDSP32_TCK_SRC_CONST_HIGH = 17,
}
impl From<TCK_A> for u8 {
    #[inline(always)]
    fn from(variant: TCK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TCK`"]
pub type TCK_R = crate::R<u8, TCK_A>;
impl TCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TCK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TCK_A::LPDSP32_TCK_SRC_DIO_0),
            1 => Val(TCK_A::LPDSP32_TCK_SRC_DIO_1),
            2 => Val(TCK_A::LPDSP32_TCK_SRC_DIO_2),
            3 => Val(TCK_A::LPDSP32_TCK_SRC_DIO_3),
            4 => Val(TCK_A::LPDSP32_TCK_SRC_DIO_4),
            5 => Val(TCK_A::LPDSP32_TCK_SRC_DIO_5),
            6 => Val(TCK_A::LPDSP32_TCK_SRC_DIO_6),
            7 => Val(TCK_A::LPDSP32_TCK_SRC_DIO_7),
            8 => Val(TCK_A::LPDSP32_TCK_SRC_DIO_8),
            9 => Val(TCK_A::LPDSP32_TCK_SRC_DIO_9),
            10 => Val(TCK_A::LPDSP32_TCK_SRC_DIO_10),
            11 => Val(TCK_A::LPDSP32_TCK_SRC_DIO_11),
            12 => Val(TCK_A::LPDSP32_TCK_SRC_DIO_12),
            13 => Val(TCK_A::LPDSP32_TCK_SRC_DIO_13),
            14 => Val(TCK_A::LPDSP32_TCK_SRC_DIO_14),
            15 => Val(TCK_A::LPDSP32_TCK_SRC_DIO_15),
            16 => Val(TCK_A::LPDSP32_TCK_SRC_CONST_LOW),
            17 => Val(TCK_A::LPDSP32_TCK_SRC_CONST_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TCK_SRC_DIO_0`"]
    #[inline(always)]
    pub fn is_lpdsp32_tck_src_dio_0(&self) -> bool {
        *self == TCK_A::LPDSP32_TCK_SRC_DIO_0
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TCK_SRC_DIO_1`"]
    #[inline(always)]
    pub fn is_lpdsp32_tck_src_dio_1(&self) -> bool {
        *self == TCK_A::LPDSP32_TCK_SRC_DIO_1
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TCK_SRC_DIO_2`"]
    #[inline(always)]
    pub fn is_lpdsp32_tck_src_dio_2(&self) -> bool {
        *self == TCK_A::LPDSP32_TCK_SRC_DIO_2
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TCK_SRC_DIO_3`"]
    #[inline(always)]
    pub fn is_lpdsp32_tck_src_dio_3(&self) -> bool {
        *self == TCK_A::LPDSP32_TCK_SRC_DIO_3
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TCK_SRC_DIO_4`"]
    #[inline(always)]
    pub fn is_lpdsp32_tck_src_dio_4(&self) -> bool {
        *self == TCK_A::LPDSP32_TCK_SRC_DIO_4
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TCK_SRC_DIO_5`"]
    #[inline(always)]
    pub fn is_lpdsp32_tck_src_dio_5(&self) -> bool {
        *self == TCK_A::LPDSP32_TCK_SRC_DIO_5
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TCK_SRC_DIO_6`"]
    #[inline(always)]
    pub fn is_lpdsp32_tck_src_dio_6(&self) -> bool {
        *self == TCK_A::LPDSP32_TCK_SRC_DIO_6
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TCK_SRC_DIO_7`"]
    #[inline(always)]
    pub fn is_lpdsp32_tck_src_dio_7(&self) -> bool {
        *self == TCK_A::LPDSP32_TCK_SRC_DIO_7
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TCK_SRC_DIO_8`"]
    #[inline(always)]
    pub fn is_lpdsp32_tck_src_dio_8(&self) -> bool {
        *self == TCK_A::LPDSP32_TCK_SRC_DIO_8
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TCK_SRC_DIO_9`"]
    #[inline(always)]
    pub fn is_lpdsp32_tck_src_dio_9(&self) -> bool {
        *self == TCK_A::LPDSP32_TCK_SRC_DIO_9
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TCK_SRC_DIO_10`"]
    #[inline(always)]
    pub fn is_lpdsp32_tck_src_dio_10(&self) -> bool {
        *self == TCK_A::LPDSP32_TCK_SRC_DIO_10
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TCK_SRC_DIO_11`"]
    #[inline(always)]
    pub fn is_lpdsp32_tck_src_dio_11(&self) -> bool {
        *self == TCK_A::LPDSP32_TCK_SRC_DIO_11
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TCK_SRC_DIO_12`"]
    #[inline(always)]
    pub fn is_lpdsp32_tck_src_dio_12(&self) -> bool {
        *self == TCK_A::LPDSP32_TCK_SRC_DIO_12
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TCK_SRC_DIO_13`"]
    #[inline(always)]
    pub fn is_lpdsp32_tck_src_dio_13(&self) -> bool {
        *self == TCK_A::LPDSP32_TCK_SRC_DIO_13
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TCK_SRC_DIO_14`"]
    #[inline(always)]
    pub fn is_lpdsp32_tck_src_dio_14(&self) -> bool {
        *self == TCK_A::LPDSP32_TCK_SRC_DIO_14
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TCK_SRC_DIO_15`"]
    #[inline(always)]
    pub fn is_lpdsp32_tck_src_dio_15(&self) -> bool {
        *self == TCK_A::LPDSP32_TCK_SRC_DIO_15
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TCK_SRC_CONST_LOW`"]
    #[inline(always)]
    pub fn is_lpdsp32_tck_src_const_low(&self) -> bool {
        *self == TCK_A::LPDSP32_TCK_SRC_CONST_LOW
    }
    #[doc = "Checks if the value of the field is `LPDSP32_TCK_SRC_CONST_HIGH`"]
    #[inline(always)]
    pub fn is_lpdsp32_tck_src_const_high(&self) -> bool {
        *self == TCK_A::LPDSP32_TCK_SRC_CONST_HIGH
    }
}
#[doc = "Write proxy for field `TCK`"]
pub struct TCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tck_src_dio_0(self) -> &'a mut W {
        self.variant(TCK_A::LPDSP32_TCK_SRC_DIO_0)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tck_src_dio_1(self) -> &'a mut W {
        self.variant(TCK_A::LPDSP32_TCK_SRC_DIO_1)
    }
    #[doc = "Select DIO\\[2\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tck_src_dio_2(self) -> &'a mut W {
        self.variant(TCK_A::LPDSP32_TCK_SRC_DIO_2)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tck_src_dio_3(self) -> &'a mut W {
        self.variant(TCK_A::LPDSP32_TCK_SRC_DIO_3)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tck_src_dio_4(self) -> &'a mut W {
        self.variant(TCK_A::LPDSP32_TCK_SRC_DIO_4)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tck_src_dio_5(self) -> &'a mut W {
        self.variant(TCK_A::LPDSP32_TCK_SRC_DIO_5)
    }
    #[doc = "Select DIO\\[6\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tck_src_dio_6(self) -> &'a mut W {
        self.variant(TCK_A::LPDSP32_TCK_SRC_DIO_6)
    }
    #[doc = "Select DIO\\[7\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tck_src_dio_7(self) -> &'a mut W {
        self.variant(TCK_A::LPDSP32_TCK_SRC_DIO_7)
    }
    #[doc = "Select DIO\\[8\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tck_src_dio_8(self) -> &'a mut W {
        self.variant(TCK_A::LPDSP32_TCK_SRC_DIO_8)
    }
    #[doc = "Select DIO\\[9\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tck_src_dio_9(self) -> &'a mut W {
        self.variant(TCK_A::LPDSP32_TCK_SRC_DIO_9)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tck_src_dio_10(self) -> &'a mut W {
        self.variant(TCK_A::LPDSP32_TCK_SRC_DIO_10)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tck_src_dio_11(self) -> &'a mut W {
        self.variant(TCK_A::LPDSP32_TCK_SRC_DIO_11)
    }
    #[doc = "Select DIO\\[12\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tck_src_dio_12(self) -> &'a mut W {
        self.variant(TCK_A::LPDSP32_TCK_SRC_DIO_12)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tck_src_dio_13(self) -> &'a mut W {
        self.variant(TCK_A::LPDSP32_TCK_SRC_DIO_13)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tck_src_dio_14(self) -> &'a mut W {
        self.variant(TCK_A::LPDSP32_TCK_SRC_DIO_14)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn lpdsp32_tck_src_dio_15(self) -> &'a mut W {
        self.variant(TCK_A::LPDSP32_TCK_SRC_DIO_15)
    }
    #[doc = "Select constant low as source"]
    #[inline(always)]
    pub fn lpdsp32_tck_src_const_low(self) -> &'a mut W {
        self.variant(TCK_A::LPDSP32_TCK_SRC_CONST_LOW)
    }
    #[doc = "Select constant high as source"]
    #[inline(always)]
    pub fn lpdsp32_tck_src_const_high(self) -> &'a mut W {
        self.variant(TCK_A::LPDSP32_TCK_SRC_CONST_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:20 - LPDSP32_TDI input selection"]
    #[inline(always)]
    pub fn tdi(&self) -> TDI_R {
        TDI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - LPDSP32_TMS input selection"]
    #[inline(always)]
    pub fn tms(&self) -> TMS_R {
        TMS_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - LPDSP32_TCK input selection"]
    #[inline(always)]
    pub fn tck(&self) -> TCK_R {
        TCK_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:20 - LPDSP32_TDI input selection"]
    #[inline(always)]
    pub fn tdi(&mut self) -> TDI_W {
        TDI_W { w: self }
    }
    #[doc = "Bits 8:12 - LPDSP32_TMS input selection"]
    #[inline(always)]
    pub fn tms(&mut self) -> TMS_W {
        TMS_W { w: self }
    }
    #[doc = "Bits 0:4 - LPDSP32_TCK input selection"]
    #[inline(always)]
    pub fn tck(&mut self) -> TCK_W {
        TCK_W { w: self }
    }
}
