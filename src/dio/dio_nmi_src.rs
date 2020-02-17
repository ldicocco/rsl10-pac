#[doc = "Reader of register DIO_NMI_SRC"]
pub type R = crate::R<u32, super::DIO_NMI_SRC>;
#[doc = "Writer for register DIO_NMI_SRC"]
pub type W = crate::W<u32, super::DIO_NMI_SRC>;
#[doc = "Register DIO_NMI_SRC `reset()`'s with value 0x30"]
impl crate::ResetValue for super::DIO_NMI_SRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x30
    }
}
#[doc = "NMI polarity\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMI_POLARITY_A {
    #[doc = "0: NMI active low"]
    NMI_ACTIVE_LOW = 0,
    #[doc = "1: NMI active high"]
    NMI_ACTIVE_HIGH = 1,
}
impl From<NMI_POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: NMI_POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NMI_POLARITY`"]
pub type NMI_POLARITY_R = crate::R<bool, NMI_POLARITY_A>;
impl NMI_POLARITY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMI_POLARITY_A {
        match self.bits {
            false => NMI_POLARITY_A::NMI_ACTIVE_LOW,
            true => NMI_POLARITY_A::NMI_ACTIVE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NMI_ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_nmi_active_low(&self) -> bool {
        *self == NMI_POLARITY_A::NMI_ACTIVE_LOW
    }
    #[doc = "Checks if the value of the field is `NMI_ACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_nmi_active_high(&self) -> bool {
        *self == NMI_POLARITY_A::NMI_ACTIVE_HIGH
    }
}
#[doc = "Write proxy for field `NMI_POLARITY`"]
pub struct NMI_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> NMI_POLARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMI_POLARITY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "NMI active low"]
    #[inline(always)]
    pub fn nmi_active_low(self) -> &'a mut W {
        self.variant(NMI_POLARITY_A::NMI_ACTIVE_LOW)
    }
    #[doc = "NMI active high"]
    #[inline(always)]
    pub fn nmi_active_high(self) -> &'a mut W {
        self.variant(NMI_POLARITY_A::NMI_ACTIVE_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "NMI input selection\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NMI_A {
    #[doc = "0: Select DIO\\[0\\]
as source"]
    NMI_SRC_DIO_0 = 0,
    #[doc = "1: Select DIO\\[1\\]
as source"]
    NMI_SRC_DIO_1 = 1,
    #[doc = "2: Select DIO\\[2\\]
as source"]
    NMI_SRC_DIO_2 = 2,
    #[doc = "3: Select DIO\\[3\\]
as source"]
    NMI_SRC_DIO_3 = 3,
    #[doc = "4: Select DIO\\[4\\]
as source"]
    NMI_SRC_DIO_4 = 4,
    #[doc = "5: Select DIO\\[5\\]
as source"]
    NMI_SRC_DIO_5 = 5,
    #[doc = "6: Select DIO\\[6\\]
as source"]
    NMI_SRC_DIO_6 = 6,
    #[doc = "7: Select DIO\\[7\\]
as source"]
    NMI_SRC_DIO_7 = 7,
    #[doc = "8: Select DIO\\[8\\]
as source"]
    NMI_SRC_DIO_8 = 8,
    #[doc = "9: Select DIO\\[9\\]
as source"]
    NMI_SRC_DIO_9 = 9,
    #[doc = "10: Select DIO\\[10\\]
as source"]
    NMI_SRC_DIO_10 = 10,
    #[doc = "11: Select DIO\\[11\\]
as source"]
    NMI_SRC_DIO_11 = 11,
    #[doc = "12: Select DIO\\[12\\]
as source"]
    NMI_SRC_DIO_12 = 12,
    #[doc = "13: Select DIO\\[13\\]
as source"]
    NMI_SRC_DIO_13 = 13,
    #[doc = "14: Select DIO\\[14\\]
as source"]
    NMI_SRC_DIO_14 = 14,
    #[doc = "15: Select DIO\\[15\\]
as source"]
    NMI_SRC_DIO_15 = 15,
    #[doc = "16: Select constant low as source"]
    NMI_SRC_CONST_LOW = 16,
    #[doc = "17: Select constant high as source"]
    NMI_SRC_CONST_HIGH = 17,
}
impl From<NMI_A> for u8 {
    #[inline(always)]
    fn from(variant: NMI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NMI`"]
pub type NMI_R = crate::R<u8, NMI_A>;
impl NMI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NMI_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NMI_A::NMI_SRC_DIO_0),
            1 => Val(NMI_A::NMI_SRC_DIO_1),
            2 => Val(NMI_A::NMI_SRC_DIO_2),
            3 => Val(NMI_A::NMI_SRC_DIO_3),
            4 => Val(NMI_A::NMI_SRC_DIO_4),
            5 => Val(NMI_A::NMI_SRC_DIO_5),
            6 => Val(NMI_A::NMI_SRC_DIO_6),
            7 => Val(NMI_A::NMI_SRC_DIO_7),
            8 => Val(NMI_A::NMI_SRC_DIO_8),
            9 => Val(NMI_A::NMI_SRC_DIO_9),
            10 => Val(NMI_A::NMI_SRC_DIO_10),
            11 => Val(NMI_A::NMI_SRC_DIO_11),
            12 => Val(NMI_A::NMI_SRC_DIO_12),
            13 => Val(NMI_A::NMI_SRC_DIO_13),
            14 => Val(NMI_A::NMI_SRC_DIO_14),
            15 => Val(NMI_A::NMI_SRC_DIO_15),
            16 => Val(NMI_A::NMI_SRC_CONST_LOW),
            17 => Val(NMI_A::NMI_SRC_CONST_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NMI_SRC_DIO_0`"]
    #[inline(always)]
    pub fn is_nmi_src_dio_0(&self) -> bool {
        *self == NMI_A::NMI_SRC_DIO_0
    }
    #[doc = "Checks if the value of the field is `NMI_SRC_DIO_1`"]
    #[inline(always)]
    pub fn is_nmi_src_dio_1(&self) -> bool {
        *self == NMI_A::NMI_SRC_DIO_1
    }
    #[doc = "Checks if the value of the field is `NMI_SRC_DIO_2`"]
    #[inline(always)]
    pub fn is_nmi_src_dio_2(&self) -> bool {
        *self == NMI_A::NMI_SRC_DIO_2
    }
    #[doc = "Checks if the value of the field is `NMI_SRC_DIO_3`"]
    #[inline(always)]
    pub fn is_nmi_src_dio_3(&self) -> bool {
        *self == NMI_A::NMI_SRC_DIO_3
    }
    #[doc = "Checks if the value of the field is `NMI_SRC_DIO_4`"]
    #[inline(always)]
    pub fn is_nmi_src_dio_4(&self) -> bool {
        *self == NMI_A::NMI_SRC_DIO_4
    }
    #[doc = "Checks if the value of the field is `NMI_SRC_DIO_5`"]
    #[inline(always)]
    pub fn is_nmi_src_dio_5(&self) -> bool {
        *self == NMI_A::NMI_SRC_DIO_5
    }
    #[doc = "Checks if the value of the field is `NMI_SRC_DIO_6`"]
    #[inline(always)]
    pub fn is_nmi_src_dio_6(&self) -> bool {
        *self == NMI_A::NMI_SRC_DIO_6
    }
    #[doc = "Checks if the value of the field is `NMI_SRC_DIO_7`"]
    #[inline(always)]
    pub fn is_nmi_src_dio_7(&self) -> bool {
        *self == NMI_A::NMI_SRC_DIO_7
    }
    #[doc = "Checks if the value of the field is `NMI_SRC_DIO_8`"]
    #[inline(always)]
    pub fn is_nmi_src_dio_8(&self) -> bool {
        *self == NMI_A::NMI_SRC_DIO_8
    }
    #[doc = "Checks if the value of the field is `NMI_SRC_DIO_9`"]
    #[inline(always)]
    pub fn is_nmi_src_dio_9(&self) -> bool {
        *self == NMI_A::NMI_SRC_DIO_9
    }
    #[doc = "Checks if the value of the field is `NMI_SRC_DIO_10`"]
    #[inline(always)]
    pub fn is_nmi_src_dio_10(&self) -> bool {
        *self == NMI_A::NMI_SRC_DIO_10
    }
    #[doc = "Checks if the value of the field is `NMI_SRC_DIO_11`"]
    #[inline(always)]
    pub fn is_nmi_src_dio_11(&self) -> bool {
        *self == NMI_A::NMI_SRC_DIO_11
    }
    #[doc = "Checks if the value of the field is `NMI_SRC_DIO_12`"]
    #[inline(always)]
    pub fn is_nmi_src_dio_12(&self) -> bool {
        *self == NMI_A::NMI_SRC_DIO_12
    }
    #[doc = "Checks if the value of the field is `NMI_SRC_DIO_13`"]
    #[inline(always)]
    pub fn is_nmi_src_dio_13(&self) -> bool {
        *self == NMI_A::NMI_SRC_DIO_13
    }
    #[doc = "Checks if the value of the field is `NMI_SRC_DIO_14`"]
    #[inline(always)]
    pub fn is_nmi_src_dio_14(&self) -> bool {
        *self == NMI_A::NMI_SRC_DIO_14
    }
    #[doc = "Checks if the value of the field is `NMI_SRC_DIO_15`"]
    #[inline(always)]
    pub fn is_nmi_src_dio_15(&self) -> bool {
        *self == NMI_A::NMI_SRC_DIO_15
    }
    #[doc = "Checks if the value of the field is `NMI_SRC_CONST_LOW`"]
    #[inline(always)]
    pub fn is_nmi_src_const_low(&self) -> bool {
        *self == NMI_A::NMI_SRC_CONST_LOW
    }
    #[doc = "Checks if the value of the field is `NMI_SRC_CONST_HIGH`"]
    #[inline(always)]
    pub fn is_nmi_src_const_high(&self) -> bool {
        *self == NMI_A::NMI_SRC_CONST_HIGH
    }
}
#[doc = "Write proxy for field `NMI`"]
pub struct NMI_W<'a> {
    w: &'a mut W,
}
impl<'a> NMI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select DIO\\[0\\]
as source"]
    #[inline(always)]
    pub fn nmi_src_dio_0(self) -> &'a mut W {
        self.variant(NMI_A::NMI_SRC_DIO_0)
    }
    #[doc = "Select DIO\\[1\\]
as source"]
    #[inline(always)]
    pub fn nmi_src_dio_1(self) -> &'a mut W {
        self.variant(NMI_A::NMI_SRC_DIO_1)
    }
    #[doc = "Select DIO\\[2\\]
as source"]
    #[inline(always)]
    pub fn nmi_src_dio_2(self) -> &'a mut W {
        self.variant(NMI_A::NMI_SRC_DIO_2)
    }
    #[doc = "Select DIO\\[3\\]
as source"]
    #[inline(always)]
    pub fn nmi_src_dio_3(self) -> &'a mut W {
        self.variant(NMI_A::NMI_SRC_DIO_3)
    }
    #[doc = "Select DIO\\[4\\]
as source"]
    #[inline(always)]
    pub fn nmi_src_dio_4(self) -> &'a mut W {
        self.variant(NMI_A::NMI_SRC_DIO_4)
    }
    #[doc = "Select DIO\\[5\\]
as source"]
    #[inline(always)]
    pub fn nmi_src_dio_5(self) -> &'a mut W {
        self.variant(NMI_A::NMI_SRC_DIO_5)
    }
    #[doc = "Select DIO\\[6\\]
as source"]
    #[inline(always)]
    pub fn nmi_src_dio_6(self) -> &'a mut W {
        self.variant(NMI_A::NMI_SRC_DIO_6)
    }
    #[doc = "Select DIO\\[7\\]
as source"]
    #[inline(always)]
    pub fn nmi_src_dio_7(self) -> &'a mut W {
        self.variant(NMI_A::NMI_SRC_DIO_7)
    }
    #[doc = "Select DIO\\[8\\]
as source"]
    #[inline(always)]
    pub fn nmi_src_dio_8(self) -> &'a mut W {
        self.variant(NMI_A::NMI_SRC_DIO_8)
    }
    #[doc = "Select DIO\\[9\\]
as source"]
    #[inline(always)]
    pub fn nmi_src_dio_9(self) -> &'a mut W {
        self.variant(NMI_A::NMI_SRC_DIO_9)
    }
    #[doc = "Select DIO\\[10\\]
as source"]
    #[inline(always)]
    pub fn nmi_src_dio_10(self) -> &'a mut W {
        self.variant(NMI_A::NMI_SRC_DIO_10)
    }
    #[doc = "Select DIO\\[11\\]
as source"]
    #[inline(always)]
    pub fn nmi_src_dio_11(self) -> &'a mut W {
        self.variant(NMI_A::NMI_SRC_DIO_11)
    }
    #[doc = "Select DIO\\[12\\]
as source"]
    #[inline(always)]
    pub fn nmi_src_dio_12(self) -> &'a mut W {
        self.variant(NMI_A::NMI_SRC_DIO_12)
    }
    #[doc = "Select DIO\\[13\\]
as source"]
    #[inline(always)]
    pub fn nmi_src_dio_13(self) -> &'a mut W {
        self.variant(NMI_A::NMI_SRC_DIO_13)
    }
    #[doc = "Select DIO\\[14\\]
as source"]
    #[inline(always)]
    pub fn nmi_src_dio_14(self) -> &'a mut W {
        self.variant(NMI_A::NMI_SRC_DIO_14)
    }
    #[doc = "Select DIO\\[15\\]
as source"]
    #[inline(always)]
    pub fn nmi_src_dio_15(self) -> &'a mut W {
        self.variant(NMI_A::NMI_SRC_DIO_15)
    }
    #[doc = "Select constant low as source"]
    #[inline(always)]
    pub fn nmi_src_const_low(self) -> &'a mut W {
        self.variant(NMI_A::NMI_SRC_CONST_LOW)
    }
    #[doc = "Select constant high as source"]
    #[inline(always)]
    pub fn nmi_src_const_high(self) -> &'a mut W {
        self.variant(NMI_A::NMI_SRC_CONST_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - NMI polarity"]
    #[inline(always)]
    pub fn nmi_polarity(&self) -> NMI_POLARITY_R {
        NMI_POLARITY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - NMI input selection"]
    #[inline(always)]
    pub fn nmi(&self) -> NMI_R {
        NMI_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - NMI polarity"]
    #[inline(always)]
    pub fn nmi_polarity(&mut self) -> NMI_POLARITY_W {
        NMI_POLARITY_W { w: self }
    }
    #[doc = "Bits 0:4 - NMI input selection"]
    #[inline(always)]
    pub fn nmi(&mut self) -> NMI_W {
        NMI_W { w: self }
    }
}
