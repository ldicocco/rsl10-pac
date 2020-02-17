#[doc = "Reader of register DIO_AUDIOSINK_SRC"]
pub type R = crate::R<u32, super::DIO_AUDIOSINK_SRC>;
#[doc = "Writer for register DIO_AUDIOSINK_SRC"]
pub type W = crate::W<u32, super::DIO_AUDIOSINK_SRC>;
#[doc = "Register DIO_AUDIOSINK_SRC `reset()`'s with value 0x11"]
impl crate::ResetValue for super::DIO_AUDIOSINK_SRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x11
    }
}
#[doc = "Audio sink clock input selection\n\nValue on reset: 17"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_A {
    #[doc = "0: Select DIO\\[0\\]
as source"]
    AUDIOSINK_CLK_SRC_DIO_0 = 0,
    #[doc = "1: Select DIO\\[1\\]
as source"]
    AUDIOSINK_CLK_SRC_DIO_1 = 1,
    #[doc = "2: Select DIO\\[2\\]
as source"]
    AUDIOSINK_CLK_SRC_DIO_2 = 2,
    #[doc = "3: Select DIO\\[3\\]
as source"]
    AUDIOSINK_CLK_SRC_DIO_3 = 3,
    #[doc = "4: Select DIO\\[4\\]
as source"]
    AUDIOSINK_CLK_SRC_DIO_4 = 4,
    #[doc = "5: Select DIO\\[5\\]
as source"]
    AUDIOSINK_CLK_SRC_DIO_5 = 5,
    #[doc = "6: Select DIO\\[6\\]
as source"]
    AUDIOSINK_CLK_SRC_DIO_6 = 6,
    #[doc = "7: Select DIO\\[7\\]
as source"]
    AUDIOSINK_CLK_SRC_DIO_7 = 7,
    #[doc = "8: Select DIO\\[8\\]
as source"]
    AUDIOSINK_CLK_SRC_DIO_8 = 8,
    #[doc = "9: Select DIO\\[9\\]
as source"]
    AUDIOSINK_CLK_SRC_DIO_9 = 9,
    #[doc = "10: Select DIO\\[10\\]
as source"]
    AUDIOSINK_CLK_SRC_DIO_10 = 10,
    #[doc = "11: Select DIO\\[11\\]
as source"]
    AUDIOSINK_CLK_SRC_DIO_11 = 11,
    #[doc = "12: Select DIO\\[12\\]
as source"]
    AUDIOSINK_CLK_SRC_DIO_12 = 12,
    #[doc = "13: Select DIO\\[13\\]
as source"]
    AUDIOSINK_CLK_SRC_DIO_13 = 13,
    #[doc = "14: Select DIO\\[14\\]
as source"]
    AUDIOSINK_CLK_SRC_DIO_14 = 14,
    #[doc = "15: Select DIO\\[15\\]
as source"]
    AUDIOSINK_CLK_SRC_DIO_15 = 15,
    #[doc = "16: Select constant low as source"]
    AUDIOSINK_CLK_SRC_CONST_LOW = 16,
    #[doc = "17: Select constant high as source"]
    AUDIOSINK_CLK_SRC_CONST_HIGH = 17,
    #[doc = "18: Select STANDBYCLK as source"]
    AUDIOSINK_CLK_SRC_STANDBYCLK = 18,
    #[doc = "19: Select DMIC & OD sample synchronization pulse as source"]
    AUDIOSINK_CLK_SRC_DMIC_OD = 19,
}
impl From<CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLK`"]
pub type CLK_R = crate::R<u8, CLK_A>;
impl CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLK_A::AUDIOSINK_CLK_SRC_DIO_0),
            1 => Val(CLK_A::AUDIOSINK_CLK_SRC_DIO_1),
            2 => Val(CLK_A::AUDIOSINK_CLK_SRC_DIO_2),
            3 => Val(CLK_A::AUDIOSINK_CLK_SRC_DIO_3),
            4 => Val(CLK_A::AUDIOSINK_CLK_SRC_DIO_4),
            5 => Val(CLK_A::AUDIOSINK_CLK_SRC_DIO_5),
            6 => Val(CLK_A::AUDIOSINK_CLK_SRC_DIO_6),
            7 => Val(CLK_A::AUDIOSINK_CLK_SRC_DIO_7),
            8 => Val(CLK_A::AUDIOSINK_CLK_SRC_DIO_8),
            9 => Val(CLK_A::AUDIOSINK_CLK_SRC_DIO_9),
            10 => Val(CLK_A::AUDIOSINK_CLK_SRC_DIO_10),
            11 => Val(CLK_A::AUDIOSINK_CLK_SRC_DIO_11),
            12 => Val(CLK_A::AUDIOSINK_CLK_SRC_DIO_12),
            13 => Val(CLK_A::AUDIOSINK_CLK_SRC_DIO_13),
            14 => Val(CLK_A::AUDIOSINK_CLK_SRC_DIO_14),
            15 => Val(CLK_A::AUDIOSINK_CLK_SRC_DIO_15),
            16 => Val(CLK_A::AUDIOSINK_CLK_SRC_CONST_LOW),
            17 => Val(CLK_A::AUDIOSINK_CLK_SRC_CONST_HIGH),
            18 => Val(CLK_A::AUDIOSINK_CLK_SRC_STANDBYCLK),
            19 => Val(CLK_A::AUDIOSINK_CLK_SRC_DMIC_OD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_DIO_0`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_dio_0(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_DIO_0
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_DIO_1`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_dio_1(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_DIO_1
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_DIO_2`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_dio_2(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_DIO_2
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_DIO_3`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_dio_3(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_DIO_3
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_DIO_4`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_dio_4(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_DIO_4
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_DIO_5`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_dio_5(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_DIO_5
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_DIO_6`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_dio_6(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_DIO_6
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_DIO_7`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_dio_7(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_DIO_7
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_DIO_8`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_dio_8(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_DIO_8
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_DIO_9`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_dio_9(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_DIO_9
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_DIO_10`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_dio_10(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_DIO_10
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_DIO_11`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_dio_11(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_DIO_11
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_DIO_12`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_dio_12(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_DIO_12
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_DIO_13`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_dio_13(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_DIO_13
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_DIO_14`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_dio_14(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_DIO_14
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_DIO_15`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_dio_15(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_DIO_15
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_CONST_LOW`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_const_low(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_CONST_LOW
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_CONST_HIGH`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_const_high(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_CONST_HIGH
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_STANDBYCLK`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_standbyclk(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_STANDBYCLK
    }
    #[doc = "Checks if the value of the field is `AUDIOSINK_CLK_SRC_DMIC_OD`"]
    #[inline(always)]
    pub fn is_audiosink_clk_src_dmic_od(&self) -> bool {
        *self == CLK_A::AUDIOSINK_CLK_SRC_DMIC_OD
    }
}
#[doc = "Write proxy for field `CLK`"]
pub struct CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_dio_0(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_DIO_0)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_dio_1(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_DIO_1)
    }
    #[doc = "Select DIO\\[2\\]
as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_dio_2(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_DIO_2)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_dio_3(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_DIO_3)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_dio_4(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_DIO_4)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_dio_5(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_DIO_5)
    }
    #[doc = "Select DIO\\[6\\]
as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_dio_6(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_DIO_6)
    }
    #[doc = "Select DIO\\[7\\]
as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_dio_7(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_DIO_7)
    }
    #[doc = "Select DIO\\[8\\]
as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_dio_8(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_DIO_8)
    }
    #[doc = "Select DIO\\[9\\]
as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_dio_9(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_DIO_9)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_dio_10(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_DIO_10)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_dio_11(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_DIO_11)
    }
    #[doc = "Select DIO\\[12\\]
as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_dio_12(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_DIO_12)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_dio_13(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_DIO_13)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_dio_14(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_DIO_14)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_dio_15(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_DIO_15)
    }
    #[doc = "Select constant low as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_const_low(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_CONST_LOW)
    }
    #[doc = "Select constant high as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_const_high(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_CONST_HIGH)
    }
    #[doc = "Select STANDBYCLK as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_standbyclk(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_STANDBYCLK)
    }
    #[doc = "Select DMIC & OD sample synchronization pulse as source"]
    #[inline(always)]
    pub fn audiosink_clk_src_dmic_od(self) -> &'a mut W {
        self.variant(CLK_A::AUDIOSINK_CLK_SRC_DMIC_OD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Audio sink clock input selection"]
    #[inline(always)]
    pub fn clk(&self) -> CLK_R {
        CLK_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Audio sink clock input selection"]
    #[inline(always)]
    pub fn clk(&mut self) -> CLK_W {
        CLK_W { w: self }
    }
}
