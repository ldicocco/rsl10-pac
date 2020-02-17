#[doc = "Reader of register BB_ISOMUTECNTL1"]
pub type R = crate::R<u32, super::BB_ISOMUTECNTL1>;
#[doc = "Writer for register BB_ISOMUTECNTL1"]
pub type W = crate::W<u32, super::BB_ISOMUTECNTL1>;
#[doc = "Register BB_ISOMUTECNTL1 `reset()`'s with value 0x0003_0000"]
impl crate::ResetValue for super::BB_ISOMUTECNTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0003_0000
    }
}
#[doc = "Indicates which buffer is in use (direct copy of ET-ISOBUFSEL)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOGO1_A {
    #[doc = "0: Use buffer 0"]
    TOGO1_0 = 0,
    #[doc = "1: Use buffer 1"]
    TOGO1_1 = 1,
}
impl From<TOGO1_A> for bool {
    #[inline(always)]
    fn from(variant: TOGO1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TOGO1`"]
pub type TOGO1_R = crate::R<bool, TOGO1_A>;
impl TOGO1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOGO1_A {
        match self.bits {
            false => TOGO1_A::TOGO1_0,
            true => TOGO1_A::TOGO1_1,
        }
    }
    #[doc = "Checks if the value of the field is `TOGO1_0`"]
    #[inline(always)]
    pub fn is_togo1_0(&self) -> bool {
        *self == TOGO1_A::TOGO1_0
    }
    #[doc = "Checks if the value of the field is `TOGO1_1`"]
    #[inline(always)]
    pub fn is_togo1_1(&self) -> bool {
        *self == TOGO1_A::TOGO1_1
    }
}
#[doc = "Write proxy for field `TOGO1`"]
pub struct TOGO1_W<'a> {
    w: &'a mut W,
}
impl<'a> TOGO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOGO1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use buffer 0"]
    #[inline(always)]
    pub fn togo1_0(self) -> &'a mut W {
        self.variant(TOGO1_A::TOGO1_0)
    }
    #[doc = "Use buffer 1"]
    #[inline(always)]
    pub fn togo1_1(self) -> &'a mut W {
        self.variant(TOGO1_A::TOGO1_1)
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
pub enum MUTE_SINK1_A {
    #[doc = "0: Do not mute on bad reception of an ISO packet"]
    MUTE_SINK1_0 = 0,
    #[doc = "1: Mute after data or bad reception, with the pattern stored in MUTE_PATTERN0"]
    MUTE_SINK1_1 = 1,
}
impl From<MUTE_SINK1_A> for bool {
    #[inline(always)]
    fn from(variant: MUTE_SINK1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MUTE_SINK1`"]
pub type MUTE_SINK1_R = crate::R<bool, MUTE_SINK1_A>;
impl MUTE_SINK1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUTE_SINK1_A {
        match self.bits {
            false => MUTE_SINK1_A::MUTE_SINK1_0,
            true => MUTE_SINK1_A::MUTE_SINK1_1,
        }
    }
    #[doc = "Checks if the value of the field is `MUTE_SINK1_0`"]
    #[inline(always)]
    pub fn is_mute_sink1_0(&self) -> bool {
        *self == MUTE_SINK1_A::MUTE_SINK1_0
    }
    #[doc = "Checks if the value of the field is `MUTE_SINK1_1`"]
    #[inline(always)]
    pub fn is_mute_sink1_1(&self) -> bool {
        *self == MUTE_SINK1_A::MUTE_SINK1_1
    }
}
#[doc = "Write proxy for field `MUTE_SINK1`"]
pub struct MUTE_SINK1_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTE_SINK1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUTE_SINK1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not mute on bad reception of an ISO packet"]
    #[inline(always)]
    pub fn mute_sink1_0(self) -> &'a mut W {
        self.variant(MUTE_SINK1_A::MUTE_SINK1_0)
    }
    #[doc = "Mute after data or bad reception, with the pattern stored in MUTE_PATTERN0"]
    #[inline(always)]
    pub fn mute_sink1_1(self) -> &'a mut W {
        self.variant(MUTE_SINK1_A::MUTE_SINK1_1)
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
pub enum MUTE_SOURCE1_A {
    #[doc = "0: Provides Source buffer to the Packet Controller for Tx operations"]
    MUTE_SOURCE1_0 = 0,
    #[doc = "1: Forces null length packet to be sent as a replacement of ISO Packets"]
    MUTE_SOURCE1_1 = 1,
}
impl From<MUTE_SOURCE1_A> for bool {
    #[inline(always)]
    fn from(variant: MUTE_SOURCE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MUTE_SOURCE1`"]
pub type MUTE_SOURCE1_R = crate::R<bool, MUTE_SOURCE1_A>;
impl MUTE_SOURCE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUTE_SOURCE1_A {
        match self.bits {
            false => MUTE_SOURCE1_A::MUTE_SOURCE1_0,
            true => MUTE_SOURCE1_A::MUTE_SOURCE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `MUTE_SOURCE1_0`"]
    #[inline(always)]
    pub fn is_mute_source1_0(&self) -> bool {
        *self == MUTE_SOURCE1_A::MUTE_SOURCE1_0
    }
    #[doc = "Checks if the value of the field is `MUTE_SOURCE1_1`"]
    #[inline(always)]
    pub fn is_mute_source1_1(&self) -> bool {
        *self == MUTE_SOURCE1_A::MUTE_SOURCE1_1
    }
}
#[doc = "Write proxy for field `MUTE_SOURCE1`"]
pub struct MUTE_SOURCE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTE_SOURCE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUTE_SOURCE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Provides Source buffer to the Packet Controller for Tx operations"]
    #[inline(always)]
    pub fn mute_source1_0(self) -> &'a mut W {
        self.variant(MUTE_SOURCE1_A::MUTE_SOURCE1_0)
    }
    #[doc = "Forces null length packet to be sent as a replacement of ISO Packets"]
    #[inline(always)]
    pub fn mute_source1_1(self) -> &'a mut W {
        self.variant(MUTE_SOURCE1_A::MUTE_SOURCE1_1)
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
pub enum INVL1_1_A {
    #[doc = "0: Do not mute current ISO buffer"]
    INVL1_1_0 = 0,
    #[doc = "1: Current ISO buffer is invalid, mute"]
    INVL1_1_1 = 1,
}
impl From<INVL1_1_A> for bool {
    #[inline(always)]
    fn from(variant: INVL1_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INVL1_1`"]
pub type INVL1_1_R = crate::R<bool, INVL1_1_A>;
impl INVL1_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVL1_1_A {
        match self.bits {
            false => INVL1_1_A::INVL1_1_0,
            true => INVL1_1_A::INVL1_1_1,
        }
    }
    #[doc = "Checks if the value of the field is `INVL1_1_0`"]
    #[inline(always)]
    pub fn is_invl1_1_0(&self) -> bool {
        *self == INVL1_1_A::INVL1_1_0
    }
    #[doc = "Checks if the value of the field is `INVL1_1_1`"]
    #[inline(always)]
    pub fn is_invl1_1_1(&self) -> bool {
        *self == INVL1_1_A::INVL1_1_1
    }
}
#[doc = "Write proxy for field `INVL1_1`"]
pub struct INVL1_1_W<'a> {
    w: &'a mut W,
}
impl<'a> INVL1_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVL1_1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not mute current ISO buffer"]
    #[inline(always)]
    pub fn invl1_1_0(self) -> &'a mut W {
        self.variant(INVL1_1_A::INVL1_1_0)
    }
    #[doc = "Current ISO buffer is invalid, mute"]
    #[inline(always)]
    pub fn invl1_1_1(self) -> &'a mut W {
        self.variant(INVL1_1_A::INVL1_1_1)
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
pub enum INVL1_0_A {
    #[doc = "0: Do not mute current ISO buffer"]
    INVL1_0_0 = 0,
    #[doc = "1: Current ISO buffer is invalid, mute"]
    INVL1_0_1 = 1,
}
impl From<INVL1_0_A> for bool {
    #[inline(always)]
    fn from(variant: INVL1_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INVL1_0`"]
pub type INVL1_0_R = crate::R<bool, INVL1_0_A>;
impl INVL1_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVL1_0_A {
        match self.bits {
            false => INVL1_0_A::INVL1_0_0,
            true => INVL1_0_A::INVL1_0_1,
        }
    }
    #[doc = "Checks if the value of the field is `INVL1_0_0`"]
    #[inline(always)]
    pub fn is_invl1_0_0(&self) -> bool {
        *self == INVL1_0_A::INVL1_0_0
    }
    #[doc = "Checks if the value of the field is `INVL1_0_1`"]
    #[inline(always)]
    pub fn is_invl1_0_1(&self) -> bool {
        *self == INVL1_0_A::INVL1_0_1
    }
}
#[doc = "Write proxy for field `INVL1_0`"]
pub struct INVL1_0_W<'a> {
    w: &'a mut W,
}
impl<'a> INVL1_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVL1_0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not mute current ISO buffer"]
    #[inline(always)]
    pub fn invl1_0_0(self) -> &'a mut W {
        self.variant(INVL1_0_A::INVL1_0_0)
    }
    #[doc = "Current ISO buffer is invalid, mute"]
    #[inline(always)]
    pub fn invl1_0_1(self) -> &'a mut W {
        self.variant(INVL1_0_A::INVL1_0_1)
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
pub enum MUTE_PATTERN1_A {
    #[doc = "0: Value of the ISO channel 0 Mute Pattern to be used when HW muting is enabled"]
    MUTE_PATTERN1_0 = 0,
}
impl From<MUTE_PATTERN1_A> for u8 {
    #[inline(always)]
    fn from(variant: MUTE_PATTERN1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MUTE_PATTERN1`"]
pub type MUTE_PATTERN1_R = crate::R<u8, MUTE_PATTERN1_A>;
impl MUTE_PATTERN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MUTE_PATTERN1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MUTE_PATTERN1_A::MUTE_PATTERN1_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MUTE_PATTERN1_0`"]
    #[inline(always)]
    pub fn is_mute_pattern1_0(&self) -> bool {
        *self == MUTE_PATTERN1_A::MUTE_PATTERN1_0
    }
}
#[doc = "Write proxy for field `MUTE_PATTERN1`"]
pub struct MUTE_PATTERN1_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTE_PATTERN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUTE_PATTERN1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Value of the ISO channel 0 Mute Pattern to be used when HW muting is enabled"]
    #[inline(always)]
    pub fn mute_pattern1_0(self) -> &'a mut W {
        self.variant(MUTE_PATTERN1_A::MUTE_PATTERN1_0)
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
    pub fn togo1(&self) -> TOGO1_R {
        TOGO1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 19 - HW mute control"]
    #[inline(always)]
    pub fn mute_sink1(&self) -> MUTE_SINK1_R {
        MUTE_SINK1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - HW mute control"]
    #[inline(always)]
    pub fn mute_source1(&self) -> MUTE_SOURCE1_R {
        MUTE_SOURCE1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SW mute status for ISO buffer 1 (i.e updated when ET-ISOBUFSEL = 0)"]
    #[inline(always)]
    pub fn invl1_1(&self) -> INVL1_1_R {
        INVL1_1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SW mute status for ISO buffer 0 (i.e updated when ET-ISOBUFSEL = 1)"]
    #[inline(always)]
    pub fn invl1_0(&self) -> INVL1_0_R {
        INVL1_0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - Value of the ISO channel 0 Mute Pattern to be used when HW muting is enabled"]
    #[inline(always)]
    pub fn mute_pattern1(&self) -> MUTE_PATTERN1_R {
        MUTE_PATTERN1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Indicates which buffer is in use (direct copy of ET-ISOBUFSEL)"]
    #[inline(always)]
    pub fn togo1(&mut self) -> TOGO1_W {
        TOGO1_W { w: self }
    }
    #[doc = "Bit 19 - HW mute control"]
    #[inline(always)]
    pub fn mute_sink1(&mut self) -> MUTE_SINK1_W {
        MUTE_SINK1_W { w: self }
    }
    #[doc = "Bit 18 - HW mute control"]
    #[inline(always)]
    pub fn mute_source1(&mut self) -> MUTE_SOURCE1_W {
        MUTE_SOURCE1_W { w: self }
    }
    #[doc = "Bit 17 - SW mute status for ISO buffer 1 (i.e updated when ET-ISOBUFSEL = 0)"]
    #[inline(always)]
    pub fn invl1_1(&mut self) -> INVL1_1_W {
        INVL1_1_W { w: self }
    }
    #[doc = "Bit 16 - SW mute status for ISO buffer 0 (i.e updated when ET-ISOBUFSEL = 1)"]
    #[inline(always)]
    pub fn invl1_0(&mut self) -> INVL1_0_W {
        INVL1_0_W { w: self }
    }
    #[doc = "Bits 0:7 - Value of the ISO channel 0 Mute Pattern to be used when HW muting is enabled"]
    #[inline(always)]
    pub fn mute_pattern1(&mut self) -> MUTE_PATTERN1_W {
        MUTE_PATTERN1_W { w: self }
    }
}
