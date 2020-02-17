#[doc = "Reader of register BB_DEBUGADDMIN"]
pub type R = crate::R<u32, super::BB_DEBUGADDMIN>;
#[doc = "Writer for register BB_DEBUGADDMIN"]
pub type W = crate::W<u32, super::BB_DEBUGADDMIN>;
#[doc = "Register BB_DEBUGADDMIN `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_DEBUGADDMIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Lower limit for the register zone indicated by the reg_inzone flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum REG_ADDMIN_A {
    #[doc = "0: `0`"]
    REG_ADDMIN_0 = 0,
}
impl From<REG_ADDMIN_A> for u16 {
    #[inline(always)]
    fn from(variant: REG_ADDMIN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REG_ADDMIN`"]
pub type REG_ADDMIN_R = crate::R<u16, REG_ADDMIN_A>;
impl REG_ADDMIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, REG_ADDMIN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REG_ADDMIN_A::REG_ADDMIN_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REG_ADDMIN_0`"]
    #[inline(always)]
    pub fn is_reg_addmin_0(&self) -> bool {
        *self == REG_ADDMIN_A::REG_ADDMIN_0
    }
}
#[doc = "Write proxy for field `REG_ADDMIN`"]
pub struct REG_ADDMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_ADDMIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_ADDMIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn reg_addmin_0(self) -> &'a mut W {
        self.variant(REG_ADDMIN_A::REG_ADDMIN_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Lower limit for the exchange memory zone indicated by the em_inzone flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum EM_ADDMIN_A {
    #[doc = "0: `0`"]
    EM_ADDMIN_0 = 0,
}
impl From<EM_ADDMIN_A> for u16 {
    #[inline(always)]
    fn from(variant: EM_ADDMIN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EM_ADDMIN`"]
pub type EM_ADDMIN_R = crate::R<u16, EM_ADDMIN_A>;
impl EM_ADDMIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, EM_ADDMIN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EM_ADDMIN_A::EM_ADDMIN_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EM_ADDMIN_0`"]
    #[inline(always)]
    pub fn is_em_addmin_0(&self) -> bool {
        *self == EM_ADDMIN_A::EM_ADDMIN_0
    }
}
#[doc = "Write proxy for field `EM_ADDMIN`"]
pub struct EM_ADDMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EM_ADDMIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM_ADDMIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn em_addmin_0(self) -> &'a mut W {
        self.variant(EM_ADDMIN_A::EM_ADDMIN_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Lower limit for the register zone indicated by the reg_inzone flag"]
    #[inline(always)]
    pub fn reg_addmin(&self) -> REG_ADDMIN_R {
        REG_ADDMIN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Lower limit for the exchange memory zone indicated by the em_inzone flag"]
    #[inline(always)]
    pub fn em_addmin(&self) -> EM_ADDMIN_R {
        EM_ADDMIN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Lower limit for the register zone indicated by the reg_inzone flag"]
    #[inline(always)]
    pub fn reg_addmin(&mut self) -> REG_ADDMIN_W {
        REG_ADDMIN_W { w: self }
    }
    #[doc = "Bits 0:15 - Lower limit for the exchange memory zone indicated by the em_inzone flag"]
    #[inline(always)]
    pub fn em_addmin(&mut self) -> EM_ADDMIN_W {
        EM_ADDMIN_W { w: self }
    }
}
