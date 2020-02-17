#[doc = "Reader of register BB_ISOMUTECNTL0"]
pub type R = crate::R<u32, super::BB_ISOMUTECNTL0>;
#[doc = "Writer for register BB_ISOMUTECNTL0"]
pub type W = crate::W<u32, super::BB_ISOMUTECNTL0>;
#[doc = "Register BB_ISOMUTECNTL0 `reset()`'s with value 0x0003_0000"]
impl crate::ResetValue for super::BB_ISOMUTECNTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0003_0000
    }
}
#[doc = "Indicates which buffer is in use (direct copy of ET-ISOBUFSEL)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOGO0_A {
    #[doc = "0: Use buffer 0"]
    TOGO0_0 = 0,
    #[doc = "1: Use buffer 1"]
    TOGO0_1 = 1,
}
impl From<TOGO0_A> for bool {
    #[inline(always)]
    fn from(variant: TOGO0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TOGO0`"]
pub type TOGO0_R = crate::R<bool, TOGO0_A>;
impl TOGO0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOGO0_A {
        match self.bits {
            false => TOGO0_A::TOGO0_0,
            true => TOGO0_A::TOGO0_1,
        }
    }
    #[doc = "Checks if the value of the field is `TOGO0_0`"]
    #[inline(always)]
    pub fn is_togo0_0(&self) -> bool {
        *self == TOGO0_A::TOGO0_0
    }
    #[doc = "Checks if the value of the field is `TOGO0_1`"]
    #[inline(always)]
    pub fn is_togo0_1(&self) -> bool {
        *self == TOGO0_A::TOGO0_1
    }
}
#[doc = "Write proxy for field `TOGO0`"]
pub struct TOGO0_W<'a> {
    w: &'a mut W,
}
impl<'a> TOGO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOGO0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use buffer 0"]
    #[inline(always)]
    pub fn togo0_0(self) -> &'a mut W {
        self.variant(TOGO0_A::TOGO0_0)
    }
    #[doc = "Use buffer 1"]
    #[inline(always)]
    pub fn togo0_1(self) -> &'a mut W {
        self.variant(TOGO0_A::TOGO0_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "HW mute control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTE_SINK0_A {
    #[doc = "0: Do not mute on bad reception of an ISO packet"]
    MUTE_SINK0_0 = 0,
    #[doc = "1: Mute after data or bad reception, with the pattern stored in MUTE_PATTERN0"]
    MUTE_SINK0_1 = 1,
}
impl From<MUTE_SINK0_A> for bool {
    #[inline(always)]
    fn from(variant: MUTE_SINK0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MUTE_SINK0`"]
pub type MUTE_SINK0_R = crate::R<bool, MUTE_SINK0_A>;
impl MUTE_SINK0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUTE_SINK0_A {
        match self.bits {
            false => MUTE_SINK0_A::MUTE_SINK0_0,
            true => MUTE_SINK0_A::MUTE_SINK0_1,
        }
    }
    #[doc = "Checks if the value of the field is `MUTE_SINK0_0`"]
    #[inline(always)]
    pub fn is_mute_sink0_0(&self) -> bool {
        *self == MUTE_SINK0_A::MUTE_SINK0_0
    }
    #[doc = "Checks if the value of the field is `MUTE_SINK0_1`"]
    #[inline(always)]
    pub fn is_mute_sink0_1(&self) -> bool {
        *self == MUTE_SINK0_A::MUTE_SINK0_1
    }
}
#[doc = "Write proxy for field `MUTE_SINK0`"]
pub struct MUTE_SINK0_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTE_SINK0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUTE_SINK0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not mute on bad reception of an ISO packet"]
    #[inline(always)]
    pub fn mute_sink0_0(self) -> &'a mut W {
        self.variant(MUTE_SINK0_A::MUTE_SINK0_0)
    }
    #[doc = "Mute after data or bad reception, with the pattern stored in MUTE_PATTERN0"]
    #[inline(always)]
    pub fn mute_sink0_1(self) -> &'a mut W {
        self.variant(MUTE_SINK0_A::MUTE_SINK0_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "HW mute control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTE_SOURCE0_A {
    #[doc = "0: Provides Source buffer to the Packet Controller for Tx operations"]
    MUTE_SOURCE0_0 = 0,
    #[doc = "1: Forces null length packet to be sent as a replacement of ISO Packets"]
    MUTE_SOURCE0_1 = 1,
}
impl From<MUTE_SOURCE0_A> for bool {
    #[inline(always)]
    fn from(variant: MUTE_SOURCE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MUTE_SOURCE0`"]
pub type MUTE_SOURCE0_R = crate::R<bool, MUTE_SOURCE0_A>;
impl MUTE_SOURCE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUTE_SOURCE0_A {
        match self.bits {
            false => MUTE_SOURCE0_A::MUTE_SOURCE0_0,
            true => MUTE_SOURCE0_A::MUTE_SOURCE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `MUTE_SOURCE0_0`"]
    #[inline(always)]
    pub fn is_mute_source0_0(&self) -> bool {
        *self == MUTE_SOURCE0_A::MUTE_SOURCE0_0
    }
    #[doc = "Checks if the value of the field is `MUTE_SOURCE0_1`"]
    #[inline(always)]
    pub fn is_mute_source0_1(&self) -> bool {
        *self == MUTE_SOURCE0_A::MUTE_SOURCE0_1
    }
}
#[doc = "Write proxy for field `MUTE_SOURCE0`"]
pub struct MUTE_SOURCE0_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTE_SOURCE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUTE_SOURCE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Provides Source buffer to the Packet Controller for Tx operations"]
    #[inline(always)]
    pub fn mute_source0_0(self) -> &'a mut W {
        self.variant(MUTE_SOURCE0_A::MUTE_SOURCE0_0)
    }
    #[doc = "Forces null length packet to be sent as a replacement of ISO Packets"]
    #[inline(always)]
    pub fn mute_source0_1(self) -> &'a mut W {
        self.variant(MUTE_SOURCE0_A::MUTE_SOURCE0_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "SW mute status for ISO buffer 1 (i.e updated when ET-ISOBUFSEL = 0)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVL0_1_A {
    #[doc = "0: Do not mute current ISO buffer"]
    INVL0_1_0 = 0,
    #[doc = "1: Current ISO buffer is invalid, mute"]
    INVL0_1_1 = 1,
}
impl From<INVL0_1_A> for bool {
    #[inline(always)]
    fn from(variant: INVL0_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INVL0_1`"]
pub type INVL0_1_R = crate::R<bool, INVL0_1_A>;
impl INVL0_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVL0_1_A {
        match self.bits {
            false => INVL0_1_A::INVL0_1_0,
            true => INVL0_1_A::INVL0_1_1,
        }
    }
    #[doc = "Checks if the value of the field is `INVL0_1_0`"]
    #[inline(always)]
    pub fn is_invl0_1_0(&self) -> bool {
        *self == INVL0_1_A::INVL0_1_0
    }
    #[doc = "Checks if the value of the field is `INVL0_1_1`"]
    #[inline(always)]
    pub fn is_invl0_1_1(&self) -> bool {
        *self == INVL0_1_A::INVL0_1_1
    }
}
#[doc = "Write proxy for field `INVL0_1`"]
pub struct INVL0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> INVL0_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVL0_1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not mute current ISO buffer"]
    #[inline(always)]
    pub fn invl0_1_0(self) -> &'a mut W {
        self.variant(INVL0_1_A::INVL0_1_0)
    }
    #[doc = "Current ISO buffer is invalid, mute"]
    #[inline(always)]
    pub fn invl0_1_1(self) -> &'a mut W {
        self.variant(INVL0_1_A::INVL0_1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "SW mute status for ISO buffer 0 (i.e updated when ET-ISOBUFSEL = 1)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVL0_0_A {
    #[doc = "0: Do not mute current ISO buffer"]
    INVL0_0_0 = 0,
    #[doc = "1: Current ISO buffer is invalid, mute"]
    INVL0_0_1 = 1,
}
impl From<INVL0_0_A> for bool {
    #[inline(always)]
    fn from(variant: INVL0_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INVL0_0`"]
pub type INVL0_0_R = crate::R<bool, INVL0_0_A>;
impl INVL0_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVL0_0_A {
        match self.bits {
            false => INVL0_0_A::INVL0_0_0,
            true => INVL0_0_A::INVL0_0_1,
        }
    }
    #[doc = "Checks if the value of the field is `INVL0_0_0`"]
    #[inline(always)]
    pub fn is_invl0_0_0(&self) -> bool {
        *self == INVL0_0_A::INVL0_0_0
    }
    #[doc = "Checks if the value of the field is `INVL0_0_1`"]
    #[inline(always)]
    pub fn is_invl0_0_1(&self) -> bool {
        *self == INVL0_0_A::INVL0_0_1
    }
}
#[doc = "Write proxy for field `INVL0_0`"]
pub struct INVL0_0_W<'a> {
    w: &'a mut W,
}
impl<'a> INVL0_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVL0_0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not mute current ISO buffer"]
    #[inline(always)]
    pub fn invl0_0_0(self) -> &'a mut W {
        self.variant(INVL0_0_A::INVL0_0_0)
    }
    #[doc = "Current ISO buffer is invalid, mute"]
    #[inline(always)]
    pub fn invl0_0_1(self) -> &'a mut W {
        self.variant(INVL0_0_A::INVL0_0_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Value of the ISO channel 0 Mute Pattern to be used when HW muting is enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUTE_PATTERN0_A {
    #[doc = "0: Value of the ISO channel 0 Mute Pattern to be used when HW muting is enabled"]
    MUTE_PATTERN0_0 = 0,
}
impl From<MUTE_PATTERN0_A> for u8 {
    #[inline(always)]
    fn from(variant: MUTE_PATTERN0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MUTE_PATTERN0`"]
pub type MUTE_PATTERN0_R = crate::R<u8, MUTE_PATTERN0_A>;
impl MUTE_PATTERN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MUTE_PATTERN0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MUTE_PATTERN0_A::MUTE_PATTERN0_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MUTE_PATTERN0_0`"]
    #[inline(always)]
    pub fn is_mute_pattern0_0(&self) -> bool {
        *self == MUTE_PATTERN0_A::MUTE_PATTERN0_0
    }
}
#[doc = "Write proxy for field `MUTE_PATTERN0`"]
pub struct MUTE_PATTERN0_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTE_PATTERN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUTE_PATTERN0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Value of the ISO channel 0 Mute Pattern to be used when HW muting is enabled"]
    #[inline(always)]
    pub fn mute_pattern0_0(self) -> &'a mut W {
        self.variant(MUTE_PATTERN0_A::MUTE_PATTERN0_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Indicates which buffer is in use (direct copy of ET-ISOBUFSEL)"]
    #[inline(always)]
    pub fn togo0(&self) -> TOGO0_R {
        TOGO0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 19 - HW mute control"]
    #[inline(always)]
    pub fn mute_sink0(&self) -> MUTE_SINK0_R {
        MUTE_SINK0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - HW mute control"]
    #[inline(always)]
    pub fn mute_source0(&self) -> MUTE_SOURCE0_R {
        MUTE_SOURCE0_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SW mute status for ISO buffer 1 (i.e updated when ET-ISOBUFSEL = 0)"]
    #[inline(always)]
    pub fn invl0_1(&self) -> INVL0_1_R {
        INVL0_1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SW mute status for ISO buffer 0 (i.e updated when ET-ISOBUFSEL = 1)"]
    #[inline(always)]
    pub fn invl0_0(&self) -> INVL0_0_R {
        INVL0_0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - Value of the ISO channel 0 Mute Pattern to be used when HW muting is enabled"]
    #[inline(always)]
    pub fn mute_pattern0(&self) -> MUTE_PATTERN0_R {
        MUTE_PATTERN0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Indicates which buffer is in use (direct copy of ET-ISOBUFSEL)"]
    #[inline(always)]
    pub fn togo0(&mut self) -> TOGO0_W {
        TOGO0_W { w: self }
    }
    #[doc = "Bit 19 - HW mute control"]
    #[inline(always)]
    pub fn mute_sink0(&mut self) -> MUTE_SINK0_W {
        MUTE_SINK0_W { w: self }
    }
    #[doc = "Bit 18 - HW mute control"]
    #[inline(always)]
    pub fn mute_source0(&mut self) -> MUTE_SOURCE0_W {
        MUTE_SOURCE0_W { w: self }
    }
    #[doc = "Bit 17 - SW mute status for ISO buffer 1 (i.e updated when ET-ISOBUFSEL = 0)"]
    #[inline(always)]
    pub fn invl0_1(&mut self) -> INVL0_1_W {
        INVL0_1_W { w: self }
    }
    #[doc = "Bit 16 - SW mute status for ISO buffer 0 (i.e updated when ET-ISOBUFSEL = 1)"]
    #[inline(always)]
    pub fn invl0_0(&mut self) -> INVL0_0_W {
        INVL0_0_W { w: self }
    }
    #[doc = "Bits 0:7 - Value of the ISO channel 0 Mute Pattern to be used when HW muting is enabled"]
    #[inline(always)]
    pub fn mute_pattern0(&mut self) -> MUTE_PATTERN0_W {
        MUTE_PATTERN0_W { w: self }
    }
}
