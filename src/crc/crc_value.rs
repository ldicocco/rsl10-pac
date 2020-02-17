#[doc = "Reader of register CRC_VALUE"]
pub type R = crate::R<u32, super::CRC_VALUE>;
#[doc = "Writer for register CRC_VALUE"]
pub type W = crate::W<u32, super::CRC_VALUE>;
#[doc = "Register CRC_VALUE `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::CRC_VALUE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "CRC generator value: Write 0xFFFFFFF (32) or 0xFFFF (CCITT) to initialize the CRC, read provides the current CRC value.\n\nValue on reset: 65535"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CURRENT_CRC_A {
    #[doc = "65535: Initial value for the CRC CCITT calculation"]
    CRC_CCITT_INIT_VALUE = 65535,
    #[doc = "4294967295: Initial value for the CRC 32 calculation"]
    CRC_32_INIT_VALUE = 4294967295,
}
impl From<CURRENT_CRC_A> for u32 {
    #[inline(always)]
    fn from(variant: CURRENT_CRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CURRENT_CRC`"]
pub type CURRENT_CRC_R = crate::R<u32, CURRENT_CRC_A>;
impl CURRENT_CRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CURRENT_CRC_A> {
        use crate::Variant::*;
        match self.bits {
            65535 => Val(CURRENT_CRC_A::CRC_CCITT_INIT_VALUE),
            4294967295 => Val(CURRENT_CRC_A::CRC_32_INIT_VALUE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CRC_CCITT_INIT_VALUE`"]
    #[inline(always)]
    pub fn is_crc_ccitt_init_value(&self) -> bool {
        *self == CURRENT_CRC_A::CRC_CCITT_INIT_VALUE
    }
    #[doc = "Checks if the value of the field is `CRC_32_INIT_VALUE`"]
    #[inline(always)]
    pub fn is_crc_32_init_value(&self) -> bool {
        *self == CURRENT_CRC_A::CRC_32_INIT_VALUE
    }
}
#[doc = "Write proxy for field `CURRENT_CRC`"]
pub struct CURRENT_CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRENT_CRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CURRENT_CRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Initial value for the CRC CCITT calculation"]
    #[inline(always)]
    pub fn crc_ccitt_init_value(self) -> &'a mut W {
        self.variant(CURRENT_CRC_A::CRC_CCITT_INIT_VALUE)
    }
    #[doc = "Initial value for the CRC 32 calculation"]
    #[inline(always)]
    pub fn crc_32_init_value(self) -> &'a mut W {
        self.variant(CURRENT_CRC_A::CRC_32_INIT_VALUE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC generator value: Write 0xFFFFFFF (32) or 0xFFFF (CCITT) to initialize the CRC, read provides the current CRC value."]
    #[inline(always)]
    pub fn current_crc(&self) -> CURRENT_CRC_R {
        CURRENT_CRC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC generator value: Write 0xFFFFFFF (32) or 0xFFFF (CCITT) to initialize the CRC, read provides the current CRC value."]
    #[inline(always)]
    pub fn current_crc(&mut self) -> CURRENT_CRC_W {
        CURRENT_CRC_W { w: self }
    }
}
