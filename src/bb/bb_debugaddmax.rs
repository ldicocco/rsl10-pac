#[doc = "Reader of register BB_DEBUGADDMAX"]
pub type R = crate::R<u32, super::BB_DEBUGADDMAX>;
#[doc = "Writer for register BB_DEBUGADDMAX"]
pub type W = crate::W<u32, super::BB_DEBUGADDMAX>;
#[doc = "Register BB_DEBUGADDMAX `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_DEBUGADDMAX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Upper limit for the register zone indicated by the reg_inzone flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum REG_ADDMAX_A {
    #[doc = "0: `0`"]
    REG_ADDMAX_0 = 0,
}
impl From<REG_ADDMAX_A> for u16 {
    #[inline(always)]
    fn from(variant: REG_ADDMAX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REG_ADDMAX`"]
pub type REG_ADDMAX_R = crate::R<u16, REG_ADDMAX_A>;
impl REG_ADDMAX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, REG_ADDMAX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REG_ADDMAX_A::REG_ADDMAX_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REG_ADDMAX_0`"]
    #[inline(always)]
    pub fn is_reg_addmax_0(&self) -> bool {
        *self == REG_ADDMAX_A::REG_ADDMAX_0
    }
}
#[doc = "Write proxy for field `REG_ADDMAX`"]
pub struct REG_ADDMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_ADDMAX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_ADDMAX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reg_addmax_0(self) -> &'a mut W {
        self.variant(REG_ADDMAX_A::REG_ADDMAX_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Upper limit for the exchange memory zone indicated by the em_inzone flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum EM_ADDMAX_A {
    #[doc = "0: `0`"]
    EM_ADDMAX_0 = 0,
}
impl From<EM_ADDMAX_A> for u16 {
    #[inline(always)]
    fn from(variant: EM_ADDMAX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EM_ADDMAX`"]
pub type EM_ADDMAX_R = crate::R<u16, EM_ADDMAX_A>;
impl EM_ADDMAX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, EM_ADDMAX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EM_ADDMAX_A::EM_ADDMAX_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EM_ADDMAX_0`"]
    #[inline(always)]
    pub fn is_em_addmax_0(&self) -> bool {
        *self == EM_ADDMAX_A::EM_ADDMAX_0
    }
}
#[doc = "Write proxy for field `EM_ADDMAX`"]
pub struct EM_ADDMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> EM_ADDMAX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM_ADDMAX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn em_addmax_0(self) -> &'a mut W {
        self.variant(EM_ADDMAX_A::EM_ADDMAX_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Upper limit for the register zone indicated by the reg_inzone flag"]
    #[inline(always)]
    pub fn reg_addmax(&self) -> REG_ADDMAX_R {
        REG_ADDMAX_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Upper limit for the exchange memory zone indicated by the em_inzone flag"]
    #[inline(always)]
    pub fn em_addmax(&self) -> EM_ADDMAX_R {
        EM_ADDMAX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Upper limit for the register zone indicated by the reg_inzone flag"]
    #[inline(always)]
    pub fn reg_addmax(&mut self) -> REG_ADDMAX_W {
        REG_ADDMAX_W { w: self }
    }
    #[doc = "Bits 0:15 - Upper limit for the exchange memory zone indicated by the em_inzone flag"]
    #[inline(always)]
    pub fn em_addmax(&mut self) -> EM_ADDMAX_W {
        EM_ADDMAX_W { w: self }
    }
}
