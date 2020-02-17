#[doc = "Reader of register BB_ISOCHANCNTL0"]
pub type R = crate::R<u32, super::BB_ISOCHANCNTL0>;
#[doc = "Writer for register BB_ISOCHANCNTL0"]
pub type W = crate::W<u32, super::BB_ISOCHANCNTL0>;
#[doc = "Register BB_ISOCHANCNTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_ISOCHANCNTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Generate Tx ACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RETXACKEN0_A {
    #[doc = "0: No Tx ACK generation in re-Tx"]
    RETXACKEN0_0 = 0,
    #[doc = "1: Tx ACK generation in re-Tx"]
    RETXACKEN0_1 = 1,
}
impl From<RETXACKEN0_A> for bool {
    #[inline(always)]
    fn from(variant: RETXACKEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RETXACKEN0`"]
pub type RETXACKEN0_R = crate::R<bool, RETXACKEN0_A>;
impl RETXACKEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RETXACKEN0_A {
        match self.bits {
            false => RETXACKEN0_A::RETXACKEN0_0,
            true => RETXACKEN0_A::RETXACKEN0_1,
        }
    }
    #[doc = "Checks if the value of the field is `RETXACKEN0_0`"]
    #[inline(always)]
    pub fn is_retxacken0_0(&self) -> bool {
        *self == RETXACKEN0_A::RETXACKEN0_0
    }
    #[doc = "Checks if the value of the field is `RETXACKEN0_1`"]
    #[inline(always)]
    pub fn is_retxacken0_1(&self) -> bool {
        *self == RETXACKEN0_A::RETXACKEN0_1
    }
}
#[doc = "Write proxy for field `RETXACKEN0`"]
pub struct RETXACKEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> RETXACKEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RETXACKEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Tx ACK generation in re-Tx"]
    #[inline(always)]
    pub fn retxacken0_0(self) -> &'a mut W {
        self.variant(RETXACKEN0_A::RETXACKEN0_0)
    }
    #[doc = "Tx ACK generation in re-Tx"]
    #[inline(always)]
    pub fn retxacken0_1(self) -> &'a mut W {
        self.variant(RETXACKEN0_A::RETXACKEN0_1)
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
pub enum SYNCGEN0_A {
    #[doc = "0: Disable audio0_syn_p generation"]
    SYNCGEN0_0 = 0,
    #[doc = "1: Enable audio0_syn_p generation"]
    SYNCGEN0_1 = 1,
}
impl From<SYNCGEN0_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCGEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNCGEN0`"]
pub type SYNCGEN0_R = crate::R<bool, SYNCGEN0_A>;
impl SYNCGEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCGEN0_A {
        match self.bits {
            false => SYNCGEN0_A::SYNCGEN0_0,
            true => SYNCGEN0_A::SYNCGEN0_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCGEN0_0`"]
    #[inline(always)]
    pub fn is_syncgen0_0(&self) -> bool {
        *self == SYNCGEN0_A::SYNCGEN0_0
    }
    #[doc = "Checks if the value of the field is `SYNCGEN0_1`"]
    #[inline(always)]
    pub fn is_syncgen0_1(&self) -> bool {
        *self == SYNCGEN0_A::SYNCGEN0_1
    }
}
#[doc = "Write proxy for field `SYNCGEN0`"]
pub struct SYNCGEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCGEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCGEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable audio0_syn_p generation"]
    #[inline(always)]
    pub fn syncgen0_0(self) -> &'a mut W {
        self.variant(SYNCGEN0_A::SYNCGEN0_0)
    }
    #[doc = "Enable audio0_syn_p generation"]
    #[inline(always)]
    pub fn syncgen0_1(self) -> &'a mut W {
        self.variant(SYNCGEN0_A::SYNCGEN0_1)
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
pub enum ISOCHANEN0_A {
    #[doc = "0: Disable ISO Channel (LLID=0 invalid)"]
    ISOCHANEN0_0 = 0,
    #[doc = "1: Enable ISO Channel (LLID=0 valid)"]
    ISOCHANEN0_1 = 1,
}
impl From<ISOCHANEN0_A> for bool {
    #[inline(always)]
    fn from(variant: ISOCHANEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ISOCHANEN0`"]
pub type ISOCHANEN0_R = crate::R<bool, ISOCHANEN0_A>;
impl ISOCHANEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOCHANEN0_A {
        match self.bits {
            false => ISOCHANEN0_A::ISOCHANEN0_0,
            true => ISOCHANEN0_A::ISOCHANEN0_1,
        }
    }
    #[doc = "Checks if the value of the field is `ISOCHANEN0_0`"]
    #[inline(always)]
    pub fn is_isochanen0_0(&self) -> bool {
        *self == ISOCHANEN0_A::ISOCHANEN0_0
    }
    #[doc = "Checks if the value of the field is `ISOCHANEN0_1`"]
    #[inline(always)]
    pub fn is_isochanen0_1(&self) -> bool {
        *self == ISOCHANEN0_A::ISOCHANEN0_1
    }
}
#[doc = "Write proxy for field `ISOCHANEN0`"]
pub struct ISOCHANEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOCHANEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISOCHANEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable ISO Channel (LLID=0 invalid)"]
    #[inline(always)]
    pub fn isochanen0_0(self) -> &'a mut W {
        self.variant(ISOCHANEN0_A::ISOCHANEN0_0)
    }
    #[doc = "Enable ISO Channel (LLID=0 valid)"]
    #[inline(always)]
    pub fn isochanen0_1(self) -> &'a mut W {
        self.variant(ISOCHANEN0_A::ISOCHANEN0_1)
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
pub enum ISOTYPE0_A {
    #[doc = "0: Audio Mode 0"]
    ISOTYPE0_0 = 0,
    #[doc = "1: Reserved"]
    ISOTYPE0_1 = 1,
    #[doc = "2: Reserved"]
    ISOTYPE0_2 = 2,
    #[doc = "3: Reserved"]
    ISOTYPE0_3 = 3,
}
impl From<ISOTYPE0_A> for u8 {
    #[inline(always)]
    fn from(variant: ISOTYPE0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ISOTYPE0`"]
pub type ISOTYPE0_R = crate::R<u8, ISOTYPE0_A>;
impl ISOTYPE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOTYPE0_A {
        match self.bits {
            0 => ISOTYPE0_A::ISOTYPE0_0,
            1 => ISOTYPE0_A::ISOTYPE0_1,
            2 => ISOTYPE0_A::ISOTYPE0_2,
            3 => ISOTYPE0_A::ISOTYPE0_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ISOTYPE0_0`"]
    #[inline(always)]
    pub fn is_isotype0_0(&self) -> bool {
        *self == ISOTYPE0_A::ISOTYPE0_0
    }
    #[doc = "Checks if the value of the field is `ISOTYPE0_1`"]
    #[inline(always)]
    pub fn is_isotype0_1(&self) -> bool {
        *self == ISOTYPE0_A::ISOTYPE0_1
    }
    #[doc = "Checks if the value of the field is `ISOTYPE0_2`"]
    #[inline(always)]
    pub fn is_isotype0_2(&self) -> bool {
        *self == ISOTYPE0_A::ISOTYPE0_2
    }
    #[doc = "Checks if the value of the field is `ISOTYPE0_3`"]
    #[inline(always)]
    pub fn is_isotype0_3(&self) -> bool {
        *self == ISOTYPE0_A::ISOTYPE0_3
    }
}
#[doc = "Write proxy for field `ISOTYPE0`"]
pub struct ISOTYPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOTYPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISOTYPE0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Audio Mode 0"]
    #[inline(always)]
    pub fn isotype0_0(self) -> &'a mut W {
        self.variant(ISOTYPE0_A::ISOTYPE0_0)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn isotype0_1(self) -> &'a mut W {
        self.variant(ISOTYPE0_A::ISOTYPE0_1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn isotype0_2(self) -> &'a mut W {
        self.variant(ISOTYPE0_A::ISOTYPE0_2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn isotype0_3(self) -> &'a mut W {
        self.variant(ISOTYPE0_A::ISOTYPE0_3)
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
    pub fn retxacken0(&self) -> RETXACKEN0_R {
        RETXACKEN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable audio syn_p generation"]
    #[inline(always)]
    pub fn syncgen0(&self) -> SYNCGEN0_R {
        SYNCGEN0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable ISO channel"]
    #[inline(always)]
    pub fn isochanen0(&self) -> ISOCHANEN0_R {
        ISOCHANEN0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - ISO Channel Type"]
    #[inline(always)]
    pub fn isotype0(&self) -> ISOTYPE0_R {
        ISOTYPE0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Generate Tx ACK"]
    #[inline(always)]
    pub fn retxacken0(&mut self) -> RETXACKEN0_W {
        RETXACKEN0_W { w: self }
    }
    #[doc = "Bit 3 - Enable audio syn_p generation"]
    #[inline(always)]
    pub fn syncgen0(&mut self) -> SYNCGEN0_W {
        SYNCGEN0_W { w: self }
    }
    #[doc = "Bit 2 - Enable ISO channel"]
    #[inline(always)]
    pub fn isochanen0(&mut self) -> ISOCHANEN0_W {
        ISOCHANEN0_W { w: self }
    }
    #[doc = "Bits 0:1 - ISO Channel Type"]
    #[inline(always)]
    pub fn isotype0(&mut self) -> ISOTYPE0_W {
        ISOTYPE0_W { w: self }
    }
}
