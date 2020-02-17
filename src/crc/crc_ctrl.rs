#[doc = "Reader of register CRC_CTRL"]
pub type R = crate::R<u32, super::CRC_CTRL>;
#[doc = "Writer for register CRC_CTRL"]
pub type W = crate::W<u32, super::CRC_CTRL>;
#[doc = "Register CRC_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects the final CRC XOR mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FINAL_CRC_XOR_A {
    #[doc = "0: Final CRC XOR is done according to the standard (CRC-CCITT: no XOR; CRC-32: XOR with 0xFFFFFFFF)"]
    CRC_FINAL_XOR_STANDARD = 0,
    #[doc = "1: Final CRC XOR is done in opposite of the standard"]
    CRC_FINAL_XOR_NON_STANDARD = 1,
}
impl From<FINAL_CRC_XOR_A> for bool {
    #[inline(always)]
    fn from(variant: FINAL_CRC_XOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FINAL_CRC_XOR`"]
pub type FINAL_CRC_XOR_R = crate::R<bool, FINAL_CRC_XOR_A>;
impl FINAL_CRC_XOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FINAL_CRC_XOR_A {
        match self.bits {
            false => FINAL_CRC_XOR_A::CRC_FINAL_XOR_STANDARD,
            true => FINAL_CRC_XOR_A::CRC_FINAL_XOR_NON_STANDARD,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_FINAL_XOR_STANDARD`"]
    #[inline(always)]
    pub fn is_crc_final_xor_standard(&self) -> bool {
        *self == FINAL_CRC_XOR_A::CRC_FINAL_XOR_STANDARD
    }
    #[doc = "Checks if the value of the field is `CRC_FINAL_XOR_NON_STANDARD`"]
    #[inline(always)]
    pub fn is_crc_final_xor_non_standard(&self) -> bool {
        *self == FINAL_CRC_XOR_A::CRC_FINAL_XOR_NON_STANDARD
    }
}
#[doc = "Write proxy for field `FINAL_CRC_XOR`"]
pub struct FINAL_CRC_XOR_W<'a> {
    w: &'a mut W,
}
impl<'a> FINAL_CRC_XOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FINAL_CRC_XOR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Final CRC XOR is done according to the standard (CRC-CCITT: no XOR; CRC-32: XOR with 0xFFFFFFFF)"]
    #[inline(always)]
    pub fn crc_final_xor_standard(self) -> &'a mut W {
        self.variant(FINAL_CRC_XOR_A::CRC_FINAL_XOR_STANDARD)
    }
    #[doc = "Final CRC XOR is done in opposite of the standard"]
    #[inline(always)]
    pub fn crc_final_xor_non_standard(self) -> &'a mut W {
        self.variant(FINAL_CRC_XOR_A::CRC_FINAL_XOR_NON_STANDARD)
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
#[doc = "Selects the final CRC reversal mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FINAL_CRC_REVERSE_A {
    #[doc = "0: Final CRC reversal is done according to the standard (CRC-CCITT: normal; CRC-32 reversed)"]
    CRC_FINAL_REVERSE_STANDARD = 0,
    #[doc = "1: Final CRC reversal is done in opposite of the standard"]
    CRC_FINAL_REVERSE_NON_STANDARD = 1,
}
impl From<FINAL_CRC_REVERSE_A> for bool {
    #[inline(always)]
    fn from(variant: FINAL_CRC_REVERSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FINAL_CRC_REVERSE`"]
pub type FINAL_CRC_REVERSE_R = crate::R<bool, FINAL_CRC_REVERSE_A>;
impl FINAL_CRC_REVERSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FINAL_CRC_REVERSE_A {
        match self.bits {
            false => FINAL_CRC_REVERSE_A::CRC_FINAL_REVERSE_STANDARD,
            true => FINAL_CRC_REVERSE_A::CRC_FINAL_REVERSE_NON_STANDARD,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_FINAL_REVERSE_STANDARD`"]
    #[inline(always)]
    pub fn is_crc_final_reverse_standard(&self) -> bool {
        *self == FINAL_CRC_REVERSE_A::CRC_FINAL_REVERSE_STANDARD
    }
    #[doc = "Checks if the value of the field is `CRC_FINAL_REVERSE_NON_STANDARD`"]
    #[inline(always)]
    pub fn is_crc_final_reverse_non_standard(&self) -> bool {
        *self == FINAL_CRC_REVERSE_A::CRC_FINAL_REVERSE_NON_STANDARD
    }
}
#[doc = "Write proxy for field `FINAL_CRC_REVERSE`"]
pub struct FINAL_CRC_REVERSE_W<'a> {
    w: &'a mut W,
}
impl<'a> FINAL_CRC_REVERSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FINAL_CRC_REVERSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Final CRC reversal is done according to the standard (CRC-CCITT: normal; CRC-32 reversed)"]
    #[inline(always)]
    pub fn crc_final_reverse_standard(self) -> &'a mut W {
        self.variant(FINAL_CRC_REVERSE_A::CRC_FINAL_REVERSE_STANDARD)
    }
    #[doc = "Final CRC reversal is done in opposite of the standard"]
    #[inline(always)]
    pub fn crc_final_reverse_non_standard(self) -> &'a mut W {
        self.variant(FINAL_CRC_REVERSE_A::CRC_FINAL_REVERSE_NON_STANDARD)
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
#[doc = "Selects the bit order for bytes added to the CRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT_ORDER_A {
    #[doc = "0: Bit order is as defined by the standard (CRC-CCITT: normal; CRC-32 reversed)"]
    CRC_BIT_ORDER_STANDARD = 0,
    #[doc = "1: Bit order is opposite of the standard"]
    CRC_BIT_ORDER_NON_STANDARD = 1,
}
impl From<BIT_ORDER_A> for bool {
    #[inline(always)]
    fn from(variant: BIT_ORDER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BIT_ORDER`"]
pub type BIT_ORDER_R = crate::R<bool, BIT_ORDER_A>;
impl BIT_ORDER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT_ORDER_A {
        match self.bits {
            false => BIT_ORDER_A::CRC_BIT_ORDER_STANDARD,
            true => BIT_ORDER_A::CRC_BIT_ORDER_NON_STANDARD,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_BIT_ORDER_STANDARD`"]
    #[inline(always)]
    pub fn is_crc_bit_order_standard(&self) -> bool {
        *self == BIT_ORDER_A::CRC_BIT_ORDER_STANDARD
    }
    #[doc = "Checks if the value of the field is `CRC_BIT_ORDER_NON_STANDARD`"]
    #[inline(always)]
    pub fn is_crc_bit_order_non_standard(&self) -> bool {
        *self == BIT_ORDER_A::CRC_BIT_ORDER_NON_STANDARD
    }
}
#[doc = "Write proxy for field `BIT_ORDER`"]
pub struct BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_ORDER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIT_ORDER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bit order is as defined by the standard (CRC-CCITT: normal; CRC-32 reversed)"]
    #[inline(always)]
    pub fn crc_bit_order_standard(self) -> &'a mut W {
        self.variant(BIT_ORDER_A::CRC_BIT_ORDER_STANDARD)
    }
    #[doc = "Bit order is opposite of the standard"]
    #[inline(always)]
    pub fn crc_bit_order_non_standard(self) -> &'a mut W {
        self.variant(BIT_ORDER_A::CRC_BIT_ORDER_NON_STANDARD)
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
#[doc = "Selects the CRC type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_TYPE_A {
    #[doc = "0: CRC-CCITT algorithm selected"]
    CRC_CCITT = 0,
    #[doc = "1: CRC-32 (IEEE 802.3) algorithm selected"]
    CRC_32 = 1,
}
impl From<CRC_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRC_TYPE`"]
pub type CRC_TYPE_R = crate::R<bool, CRC_TYPE_A>;
impl CRC_TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_TYPE_A {
        match self.bits {
            false => CRC_TYPE_A::CRC_CCITT,
            true => CRC_TYPE_A::CRC_32,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_CCITT`"]
    #[inline(always)]
    pub fn is_crc_ccitt(&self) -> bool {
        *self == CRC_TYPE_A::CRC_CCITT
    }
    #[doc = "Checks if the value of the field is `CRC_32`"]
    #[inline(always)]
    pub fn is_crc_32(&self) -> bool {
        *self == CRC_TYPE_A::CRC_32
    }
}
#[doc = "Write proxy for field `CRC_TYPE`"]
pub struct CRC_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_TYPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CRC-CCITT algorithm selected"]
    #[inline(always)]
    pub fn crc_ccitt(self) -> &'a mut W {
        self.variant(CRC_TYPE_A::CRC_CCITT)
    }
    #[doc = "CRC-32 (IEEE 802.3) algorithm selected"]
    #[inline(always)]
    pub fn crc_32(self) -> &'a mut W {
        self.variant(CRC_TYPE_A::CRC_32)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Selects the endianness for bytes added to the CRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYTE_ORDER_A {
    #[doc = "0: Bytes are added to the CRC in big-endian order"]
    CRC_BIG_ENDIAN = 0,
    #[doc = "1: Bytes are added to the CRC in little-endian order"]
    CRC_LITTLE_ENDIAN = 1,
}
impl From<BYTE_ORDER_A> for bool {
    #[inline(always)]
    fn from(variant: BYTE_ORDER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BYTE_ORDER`"]
pub type BYTE_ORDER_R = crate::R<bool, BYTE_ORDER_A>;
impl BYTE_ORDER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYTE_ORDER_A {
        match self.bits {
            false => BYTE_ORDER_A::CRC_BIG_ENDIAN,
            true => BYTE_ORDER_A::CRC_LITTLE_ENDIAN,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_BIG_ENDIAN`"]
    #[inline(always)]
    pub fn is_crc_big_endian(&self) -> bool {
        *self == BYTE_ORDER_A::CRC_BIG_ENDIAN
    }
    #[doc = "Checks if the value of the field is `CRC_LITTLE_ENDIAN`"]
    #[inline(always)]
    pub fn is_crc_little_endian(&self) -> bool {
        *self == BYTE_ORDER_A::CRC_LITTLE_ENDIAN
    }
}
#[doc = "Write proxy for field `BYTE_ORDER`"]
pub struct BYTE_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_ORDER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYTE_ORDER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bytes are added to the CRC in big-endian order"]
    #[inline(always)]
    pub fn crc_big_endian(self) -> &'a mut W {
        self.variant(BYTE_ORDER_A::CRC_BIG_ENDIAN)
    }
    #[doc = "Bytes are added to the CRC in little-endian order"]
    #[inline(always)]
    pub fn crc_little_endian(self) -> &'a mut W {
        self.variant(BYTE_ORDER_A::CRC_LITTLE_ENDIAN)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Selects the final CRC XOR mode"]
    #[inline(always)]
    pub fn final_crc_xor(&self) -> FINAL_CRC_XOR_R {
        FINAL_CRC_XOR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Selects the final CRC reversal mode"]
    #[inline(always)]
    pub fn final_crc_reverse(&self) -> FINAL_CRC_REVERSE_R {
        FINAL_CRC_REVERSE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects the bit order for bytes added to the CRC"]
    #[inline(always)]
    pub fn bit_order(&self) -> BIT_ORDER_R {
        BIT_ORDER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects the CRC type"]
    #[inline(always)]
    pub fn crc_type(&self) -> CRC_TYPE_R {
        CRC_TYPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Selects the endianness for bytes added to the CRC"]
    #[inline(always)]
    pub fn byte_order(&self) -> BYTE_ORDER_R {
        BYTE_ORDER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Selects the final CRC XOR mode"]
    #[inline(always)]
    pub fn final_crc_xor(&mut self) -> FINAL_CRC_XOR_W {
        FINAL_CRC_XOR_W { w: self }
    }
    #[doc = "Bit 3 - Selects the final CRC reversal mode"]
    #[inline(always)]
    pub fn final_crc_reverse(&mut self) -> FINAL_CRC_REVERSE_W {
        FINAL_CRC_REVERSE_W { w: self }
    }
    #[doc = "Bit 2 - Selects the bit order for bytes added to the CRC"]
    #[inline(always)]
    pub fn bit_order(&mut self) -> BIT_ORDER_W {
        BIT_ORDER_W { w: self }
    }
    #[doc = "Bit 1 - Selects the CRC type"]
    #[inline(always)]
    pub fn crc_type(&mut self) -> CRC_TYPE_W {
        CRC_TYPE_W { w: self }
    }
    #[doc = "Bit 0 - Selects the endianness for bytes added to the CRC"]
    #[inline(always)]
    pub fn byte_order(&mut self) -> BYTE_ORDER_W {
        BYTE_ORDER_W { w: self }
    }
}
