#[doc = "Reader of register RF_CRC_POLYNOMIAL"]
pub type R = crate::R<u32, super::RF_CRC_POLYNOMIAL>;
#[doc = "Writer for register RF_CRC_POLYNOMIAL"]
pub type W = crate::W<u32, super::RF_CRC_POLYNOMIAL>;
#[doc = "Register RF_CRC_POLYNOMIAL `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_CRC_POLYNOMIAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CRC polynomial. It is coded using the Koopman notation, i.e. the nth bit codes the (n+1) coefficient. Example: x^16+x^12+x^5+1 => 0x8810\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CRC_POLYNOMIAL_CRC_POLY_A {
    #[doc = "0: `0`"]
    CRC_POLYNOMIAL_CRC_POLY_DEFAULT = 0,
}
impl From<CRC_POLYNOMIAL_CRC_POLY_A> for u32 {
    #[inline(always)]
    fn from(variant: CRC_POLYNOMIAL_CRC_POLY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CRC_POLYNOMIAL_CRC_POLY`"]
pub type CRC_POLYNOMIAL_CRC_POLY_R = crate::R<u32, CRC_POLYNOMIAL_CRC_POLY_A>;
impl CRC_POLYNOMIAL_CRC_POLY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CRC_POLYNOMIAL_CRC_POLY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CRC_POLYNOMIAL_CRC_POLY_A::CRC_POLYNOMIAL_CRC_POLY_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CRC_POLYNOMIAL_CRC_POLY_DEFAULT`"]
    #[inline(always)]
    pub fn is_crc_polynomial_crc_poly_default(&self) -> bool {
        *self == CRC_POLYNOMIAL_CRC_POLY_A::CRC_POLYNOMIAL_CRC_POLY_DEFAULT
    }
}
#[doc = "Write proxy for field `CRC_POLYNOMIAL_CRC_POLY`"]
pub struct CRC_POLYNOMIAL_CRC_POLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_POLYNOMIAL_CRC_POLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_POLYNOMIAL_CRC_POLY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn crc_polynomial_crc_poly_default(self) -> &'a mut W {
        self.variant(CRC_POLYNOMIAL_CRC_POLY_A::CRC_POLYNOMIAL_CRC_POLY_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC polynomial. It is coded using the Koopman notation, i.e. the nth bit codes the (n+1) coefficient. Example: x^16+x^12+x^5+1 => 0x8810"]
    #[inline(always)]
    pub fn crc_polynomial_crc_poly(&self) -> CRC_POLYNOMIAL_CRC_POLY_R {
        CRC_POLYNOMIAL_CRC_POLY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC polynomial. It is coded using the Koopman notation, i.e. the nth bit codes the (n+1) coefficient. Example: x^16+x^12+x^5+1 => 0x8810"]
    #[inline(always)]
    pub fn crc_polynomial_crc_poly(&mut self) -> CRC_POLYNOMIAL_CRC_POLY_W {
        CRC_POLYNOMIAL_CRC_POLY_W { w: self }
    }
}
