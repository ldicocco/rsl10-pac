#[doc = "Reader of register BB_ISOCHANCNTL2"]
pub type R = crate::R<u32, super::BB_ISOCHANCNTL2>;
#[doc = "Writer for register BB_ISOCHANCNTL2"]
pub type W = crate::W<u32, super::BB_ISOCHANCNTL2>;
#[doc = "Register BB_ISOCHANCNTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_ISOCHANCNTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Generate Tx ACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RETXACKEN2_A {
    #[doc = "0: No Tx ACK generation in re-Tx"]
    RETXACKEN2_0 = 0,
    #[doc = "1: Tx ACK generation in re-Tx"]
    RETXACKEN2_1 = 1,
}
impl From<RETXACKEN2_A> for bool {
    #[inline(always)]
    fn from(variant: RETXACKEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RETXACKEN2`"]
pub type RETXACKEN2_R = crate::R<bool, RETXACKEN2_A>;
impl RETXACKEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RETXACKEN2_A {
        match self.bits {
            false => RETXACKEN2_A::RETXACKEN2_0,
            true => RETXACKEN2_A::RETXACKEN2_1,
        }
    }
    #[doc = "Checks if the value of the field is `RETXACKEN2_0`"]
    #[inline(always)]
    pub fn is_retxacken2_0(&self) -> bool {
        *self == RETXACKEN2_A::RETXACKEN2_0
    }
    #[doc = "Checks if the value of the field is `RETXACKEN2_1`"]
    #[inline(always)]
    pub fn is_retxacken2_1(&self) -> bool {
        *self == RETXACKEN2_A::RETXACKEN2_1
    }
}
#[doc = "Write proxy for field `RETXACKEN2`"]
pub struct RETXACKEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> RETXACKEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RETXACKEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Tx ACK generation in re-Tx"]
    #[inline(always)]
    pub fn retxacken2_0(self) -> &'a mut W {
        self.variant(RETXACKEN2_A::RETXACKEN2_0)
    }
    #[doc = "Tx ACK generation in re-Tx"]
    #[inline(always)]
    pub fn retxacken2_1(self) -> &'a mut W {
        self.variant(RETXACKEN2_A::RETXACKEN2_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Enable audio syn_p generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCGEN2_A {
    #[doc = "0: Disable audio2_syn_p generation"]
    SYNCGEN2_0 = 0,
    #[doc = "1: Enable audio2_syn_p generation"]
    SYNCGEN2_1 = 1,
}
impl From<SYNCGEN2_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCGEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNCGEN2`"]
pub type SYNCGEN2_R = crate::R<bool, SYNCGEN2_A>;
impl SYNCGEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCGEN2_A {
        match self.bits {
            false => SYNCGEN2_A::SYNCGEN2_0,
            true => SYNCGEN2_A::SYNCGEN2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCGEN2_0`"]
    #[inline(always)]
    pub fn is_syncgen2_0(&self) -> bool {
        *self == SYNCGEN2_A::SYNCGEN2_0
    }
    #[doc = "Checks if the value of the field is `SYNCGEN2_1`"]
    #[inline(always)]
    pub fn is_syncgen2_1(&self) -> bool {
        *self == SYNCGEN2_A::SYNCGEN2_1
    }
}
#[doc = "Write proxy for field `SYNCGEN2`"]
pub struct SYNCGEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCGEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCGEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable audio2_syn_p generation"]
    #[inline(always)]
    pub fn syncgen2_0(self) -> &'a mut W {
        self.variant(SYNCGEN2_A::SYNCGEN2_0)
    }
    #[doc = "Enable audio2_syn_p generation"]
    #[inline(always)]
    pub fn syncgen2_1(self) -> &'a mut W {
        self.variant(SYNCGEN2_A::SYNCGEN2_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Enable ISO channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISOCHANEN2_A {
    #[doc = "0: Disable ISO Channel (LLID=0 invalid)"]
    ISOCHANEN2_0 = 0,
    #[doc = "1: Enable ISO Channel (LLID=0 valid)"]
    ISOCHANEN2_1 = 1,
}
impl From<ISOCHANEN2_A> for bool {
    #[inline(always)]
    fn from(variant: ISOCHANEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ISOCHANEN2`"]
pub type ISOCHANEN2_R = crate::R<bool, ISOCHANEN2_A>;
impl ISOCHANEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOCHANEN2_A {
        match self.bits {
            false => ISOCHANEN2_A::ISOCHANEN2_0,
            true => ISOCHANEN2_A::ISOCHANEN2_1,
        }
    }
    #[doc = "Checks if the value of the field is `ISOCHANEN2_0`"]
    #[inline(always)]
    pub fn is_isochanen2_0(&self) -> bool {
        *self == ISOCHANEN2_A::ISOCHANEN2_0
    }
    #[doc = "Checks if the value of the field is `ISOCHANEN2_1`"]
    #[inline(always)]
    pub fn is_isochanen2_1(&self) -> bool {
        *self == ISOCHANEN2_A::ISOCHANEN2_1
    }
}
#[doc = "Write proxy for field `ISOCHANEN2`"]
pub struct ISOCHANEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOCHANEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISOCHANEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable ISO Channel (LLID=0 invalid)"]
    #[inline(always)]
    pub fn isochanen2_0(self) -> &'a mut W {
        self.variant(ISOCHANEN2_A::ISOCHANEN2_0)
    }
    #[doc = "Enable ISO Channel (LLID=0 valid)"]
    #[inline(always)]
    pub fn isochanen2_1(self) -> &'a mut W {
        self.variant(ISOCHANEN2_A::ISOCHANEN2_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "ISO Channel Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ISOTYPE2_A {
    #[doc = "0: Audio Mode 0"]
    ISOTYPE2_0 = 0,
    #[doc = "1: Reserved"]
    ISOTYPE2_1 = 1,
    #[doc = "2: Reserved"]
    ISOTYPE2_2 = 2,
    #[doc = "3: Reserved"]
    ISOTYPE2_3 = 3,
}
impl From<ISOTYPE2_A> for u8 {
    #[inline(always)]
    fn from(variant: ISOTYPE2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ISOTYPE2`"]
pub type ISOTYPE2_R = crate::R<u8, ISOTYPE2_A>;
impl ISOTYPE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOTYPE2_A {
        match self.bits {
            0 => ISOTYPE2_A::ISOTYPE2_0,
            1 => ISOTYPE2_A::ISOTYPE2_1,
            2 => ISOTYPE2_A::ISOTYPE2_2,
            3 => ISOTYPE2_A::ISOTYPE2_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ISOTYPE2_0`"]
    #[inline(always)]
    pub fn is_isotype2_0(&self) -> bool {
        *self == ISOTYPE2_A::ISOTYPE2_0
    }
    #[doc = "Checks if the value of the field is `ISOTYPE2_1`"]
    #[inline(always)]
    pub fn is_isotype2_1(&self) -> bool {
        *self == ISOTYPE2_A::ISOTYPE2_1
    }
    #[doc = "Checks if the value of the field is `ISOTYPE2_2`"]
    #[inline(always)]
    pub fn is_isotype2_2(&self) -> bool {
        *self == ISOTYPE2_A::ISOTYPE2_2
    }
    #[doc = "Checks if the value of the field is `ISOTYPE2_3`"]
    #[inline(always)]
    pub fn is_isotype2_3(&self) -> bool {
        *self == ISOTYPE2_A::ISOTYPE2_3
    }
}
#[doc = "Write proxy for field `ISOTYPE2`"]
pub struct ISOTYPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOTYPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISOTYPE2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Audio Mode 0"]
    #[inline(always)]
    pub fn isotype2_0(self) -> &'a mut W {
        self.variant(ISOTYPE2_A::ISOTYPE2_0)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn isotype2_1(self) -> &'a mut W {
        self.variant(ISOTYPE2_A::ISOTYPE2_1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn isotype2_2(self) -> &'a mut W {
        self.variant(ISOTYPE2_A::ISOTYPE2_2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn isotype2_3(self) -> &'a mut W {
        self.variant(ISOTYPE2_A::ISOTYPE2_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Generate Tx ACK"]
    #[inline(always)]
    pub fn retxacken2(&self) -> RETXACKEN2_R {
        RETXACKEN2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable audio syn_p generation"]
    #[inline(always)]
    pub fn syncgen2(&self) -> SYNCGEN2_R {
        SYNCGEN2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable ISO channel"]
    #[inline(always)]
    pub fn isochanen2(&self) -> ISOCHANEN2_R {
        ISOCHANEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - ISO Channel Type"]
    #[inline(always)]
    pub fn isotype2(&self) -> ISOTYPE2_R {
        ISOTYPE2_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Generate Tx ACK"]
    #[inline(always)]
    pub fn retxacken2(&mut self) -> RETXACKEN2_W {
        RETXACKEN2_W { w: self }
    }
    #[doc = "Bit 3 - Enable audio syn_p generation"]
    #[inline(always)]
    pub fn syncgen2(&mut self) -> SYNCGEN2_W {
        SYNCGEN2_W { w: self }
    }
    #[doc = "Bit 2 - Enable ISO channel"]
    #[inline(always)]
    pub fn isochanen2(&mut self) -> ISOCHANEN2_W {
        ISOCHANEN2_W { w: self }
    }
    #[doc = "Bits 0:1 - ISO Channel Type"]
    #[inline(always)]
    pub fn isotype2(&mut self) -> ISOTYPE2_W {
        ISOTYPE2_W { w: self }
    }
}
