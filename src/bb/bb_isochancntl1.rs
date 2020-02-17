#[doc = "Reader of register BB_ISOCHANCNTL1"]
pub type R = crate::R<u32, super::BB_ISOCHANCNTL1>;
#[doc = "Writer for register BB_ISOCHANCNTL1"]
pub type W = crate::W<u32, super::BB_ISOCHANCNTL1>;
#[doc = "Register BB_ISOCHANCNTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_ISOCHANCNTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Generate Tx ACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RETXACKEN1_A {
    #[doc = "0: No Tx ACK generation in re-Tx"]
    RETXACKEN1_0 = 0,
    #[doc = "1: Tx ACK generation in re-Tx"]
    RETXACKEN1_1 = 1,
}
impl From<RETXACKEN1_A> for bool {
    #[inline(always)]
    fn from(variant: RETXACKEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RETXACKEN1`"]
pub type RETXACKEN1_R = crate::R<bool, RETXACKEN1_A>;
impl RETXACKEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RETXACKEN1_A {
        match self.bits {
            false => RETXACKEN1_A::RETXACKEN1_0,
            true => RETXACKEN1_A::RETXACKEN1_1,
        }
    }
    #[doc = "Checks if the value of the field is `RETXACKEN1_0`"]
    #[inline(always)]
    pub fn is_retxacken1_0(&self) -> bool {
        *self == RETXACKEN1_A::RETXACKEN1_0
    }
    #[doc = "Checks if the value of the field is `RETXACKEN1_1`"]
    #[inline(always)]
    pub fn is_retxacken1_1(&self) -> bool {
        *self == RETXACKEN1_A::RETXACKEN1_1
    }
}
#[doc = "Write proxy for field `RETXACKEN1`"]
pub struct RETXACKEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> RETXACKEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RETXACKEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Tx ACK generation in re-Tx"]
    #[inline(always)]
    pub fn retxacken1_0(self) -> &'a mut W {
        self.variant(RETXACKEN1_A::RETXACKEN1_0)
    }
    #[doc = "Tx ACK generation in re-Tx"]
    #[inline(always)]
    pub fn retxacken1_1(self) -> &'a mut W {
        self.variant(RETXACKEN1_A::RETXACKEN1_1)
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
pub enum SYNCGEN1_A {
    #[doc = "0: Disable audio0_syn_p generation"]
    SYNCGEN1_0 = 0,
    #[doc = "1: Enable audio0_syn_p generation"]
    SYNCGEN1_1 = 1,
}
impl From<SYNCGEN1_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCGEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNCGEN1`"]
pub type SYNCGEN1_R = crate::R<bool, SYNCGEN1_A>;
impl SYNCGEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCGEN1_A {
        match self.bits {
            false => SYNCGEN1_A::SYNCGEN1_0,
            true => SYNCGEN1_A::SYNCGEN1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCGEN1_0`"]
    #[inline(always)]
    pub fn is_syncgen1_0(&self) -> bool {
        *self == SYNCGEN1_A::SYNCGEN1_0
    }
    #[doc = "Checks if the value of the field is `SYNCGEN1_1`"]
    #[inline(always)]
    pub fn is_syncgen1_1(&self) -> bool {
        *self == SYNCGEN1_A::SYNCGEN1_1
    }
}
#[doc = "Write proxy for field `SYNCGEN1`"]
pub struct SYNCGEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCGEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCGEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable audio0_syn_p generation"]
    #[inline(always)]
    pub fn syncgen1_0(self) -> &'a mut W {
        self.variant(SYNCGEN1_A::SYNCGEN1_0)
    }
    #[doc = "Enable audio0_syn_p generation"]
    #[inline(always)]
    pub fn syncgen1_1(self) -> &'a mut W {
        self.variant(SYNCGEN1_A::SYNCGEN1_1)
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
pub enum ISOCHANEN1_A {
    #[doc = "0: Disable ISO Channel (LLID=0 invalid)"]
    ISOCHANEN1_0 = 0,
    #[doc = "1: Enable ISO Channel (LLID=0 valid)"]
    ISOCHANEN1_1 = 1,
}
impl From<ISOCHANEN1_A> for bool {
    #[inline(always)]
    fn from(variant: ISOCHANEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ISOCHANEN1`"]
pub type ISOCHANEN1_R = crate::R<bool, ISOCHANEN1_A>;
impl ISOCHANEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOCHANEN1_A {
        match self.bits {
            false => ISOCHANEN1_A::ISOCHANEN1_0,
            true => ISOCHANEN1_A::ISOCHANEN1_1,
        }
    }
    #[doc = "Checks if the value of the field is `ISOCHANEN1_0`"]
    #[inline(always)]
    pub fn is_isochanen1_0(&self) -> bool {
        *self == ISOCHANEN1_A::ISOCHANEN1_0
    }
    #[doc = "Checks if the value of the field is `ISOCHANEN1_1`"]
    #[inline(always)]
    pub fn is_isochanen1_1(&self) -> bool {
        *self == ISOCHANEN1_A::ISOCHANEN1_1
    }
}
#[doc = "Write proxy for field `ISOCHANEN1`"]
pub struct ISOCHANEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOCHANEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISOCHANEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable ISO Channel (LLID=0 invalid)"]
    #[inline(always)]
    pub fn isochanen1_0(self) -> &'a mut W {
        self.variant(ISOCHANEN1_A::ISOCHANEN1_0)
    }
    #[doc = "Enable ISO Channel (LLID=0 valid)"]
    #[inline(always)]
    pub fn isochanen1_1(self) -> &'a mut W {
        self.variant(ISOCHANEN1_A::ISOCHANEN1_1)
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
pub enum ISOTYPE1_A {
    #[doc = "0: Audio Mode 0"]
    ISOTYPE1_0 = 0,
    #[doc = "1: Reserved"]
    ISOTYPE1_1 = 1,
    #[doc = "2: Reserved"]
    ISOTYPE1_2 = 2,
    #[doc = "3: Reserved"]
    ISOTYPE1_3 = 3,
}
impl From<ISOTYPE1_A> for u8 {
    #[inline(always)]
    fn from(variant: ISOTYPE1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ISOTYPE1`"]
pub type ISOTYPE1_R = crate::R<u8, ISOTYPE1_A>;
impl ISOTYPE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOTYPE1_A {
        match self.bits {
            0 => ISOTYPE1_A::ISOTYPE1_0,
            1 => ISOTYPE1_A::ISOTYPE1_1,
            2 => ISOTYPE1_A::ISOTYPE1_2,
            3 => ISOTYPE1_A::ISOTYPE1_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ISOTYPE1_0`"]
    #[inline(always)]
    pub fn is_isotype1_0(&self) -> bool {
        *self == ISOTYPE1_A::ISOTYPE1_0
    }
    #[doc = "Checks if the value of the field is `ISOTYPE1_1`"]
    #[inline(always)]
    pub fn is_isotype1_1(&self) -> bool {
        *self == ISOTYPE1_A::ISOTYPE1_1
    }
    #[doc = "Checks if the value of the field is `ISOTYPE1_2`"]
    #[inline(always)]
    pub fn is_isotype1_2(&self) -> bool {
        *self == ISOTYPE1_A::ISOTYPE1_2
    }
    #[doc = "Checks if the value of the field is `ISOTYPE1_3`"]
    #[inline(always)]
    pub fn is_isotype1_3(&self) -> bool {
        *self == ISOTYPE1_A::ISOTYPE1_3
    }
}
#[doc = "Write proxy for field `ISOTYPE1`"]
pub struct ISOTYPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOTYPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISOTYPE1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Audio Mode 0"]
    #[inline(always)]
    pub fn isotype1_0(self) -> &'a mut W {
        self.variant(ISOTYPE1_A::ISOTYPE1_0)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn isotype1_1(self) -> &'a mut W {
        self.variant(ISOTYPE1_A::ISOTYPE1_1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn isotype1_2(self) -> &'a mut W {
        self.variant(ISOTYPE1_A::ISOTYPE1_2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn isotype1_3(self) -> &'a mut W {
        self.variant(ISOTYPE1_A::ISOTYPE1_3)
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
    pub fn retxacken1(&self) -> RETXACKEN1_R {
        RETXACKEN1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable audio syn_p generation"]
    #[inline(always)]
    pub fn syncgen1(&self) -> SYNCGEN1_R {
        SYNCGEN1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable ISO channel"]
    #[inline(always)]
    pub fn isochanen1(&self) -> ISOCHANEN1_R {
        ISOCHANEN1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - ISO Channel Type"]
    #[inline(always)]
    pub fn isotype1(&self) -> ISOTYPE1_R {
        ISOTYPE1_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Generate Tx ACK"]
    #[inline(always)]
    pub fn retxacken1(&mut self) -> RETXACKEN1_W {
        RETXACKEN1_W { w: self }
    }
    #[doc = "Bit 3 - Enable audio syn_p generation"]
    #[inline(always)]
    pub fn syncgen1(&mut self) -> SYNCGEN1_W {
        SYNCGEN1_W { w: self }
    }
    #[doc = "Bit 2 - Enable ISO channel"]
    #[inline(always)]
    pub fn isochanen1(&mut self) -> ISOCHANEN1_W {
        ISOCHANEN1_W { w: self }
    }
    #[doc = "Bits 0:1 - ISO Channel Type"]
    #[inline(always)]
    pub fn isotype1(&mut self) -> ISOTYPE1_W {
        ISOTYPE1_W { w: self }
    }
}
