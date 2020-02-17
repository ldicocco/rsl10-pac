#[doc = "Reader of register RF_CRC_RST"]
pub type R = crate::R<u32, super::RF_CRC_RST>;
#[doc = "Writer for register RF_CRC_RST"]
pub type W = crate::W<u32, super::RF_CRC_RST>;
#[doc = "Register RF_CRC_RST `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_CRC_RST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CRC reset value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CRC_RST_CRC_RST_A {
    #[doc = "0: `0`"]
    CRC_RST_CRC_RST_DEFAULT = 0,
}
impl From<CRC_RST_CRC_RST_A> for u32 {
    #[inline(always)]
    fn from(variant: CRC_RST_CRC_RST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CRC_RST_CRC_RST`"]
pub type CRC_RST_CRC_RST_R = crate::R<u32, CRC_RST_CRC_RST_A>;
impl CRC_RST_CRC_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CRC_RST_CRC_RST_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CRC_RST_CRC_RST_A::CRC_RST_CRC_RST_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CRC_RST_CRC_RST_DEFAULT`"]
    #[inline(always)]
    pub fn is_crc_rst_crc_rst_default(&self) -> bool {
        *self == CRC_RST_CRC_RST_A::CRC_RST_CRC_RST_DEFAULT
    }
}
#[doc = "Write proxy for field `CRC_RST_CRC_RST`"]
pub struct CRC_RST_CRC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_RST_CRC_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_RST_CRC_RST_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn crc_rst_crc_rst_default(self) -> &'a mut W {
        self.variant(CRC_RST_CRC_RST_A::CRC_RST_CRC_RST_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC reset value"]
    #[inline(always)]
    pub fn crc_rst_crc_rst(&self) -> CRC_RST_CRC_RST_R {
        CRC_RST_CRC_RST_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC reset value"]
    #[inline(always)]
    pub fn crc_rst_crc_rst(&mut self) -> CRC_RST_CRC_RST_W {
        CRC_RST_CRC_RST_W { w: self }
    }
}
