#[doc = "Reader of register RF_REG0A"]
pub type R = crate::R<u32, super::RF_REG0A>;
#[doc = "Writer for register RF_REG0A"]
pub type W = crate::W<u32, super::RF_REG0A>;
#[doc = "Register RF_REG0A `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG0A {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Broadcast address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ADDRESS_BROADCAST_ADDRESS_BR_A {
    #[doc = "0: `0`"]
    ADDRESS_BROADCAST_ADDRESS_BR_DEFAULT = 0,
}
impl From<ADDRESS_BROADCAST_ADDRESS_BR_A> for u16 {
    #[inline(always)]
    fn from(variant: ADDRESS_BROADCAST_ADDRESS_BR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADDRESS_BROADCAST_ADDRESS_BR`"]
pub type ADDRESS_BROADCAST_ADDRESS_BR_R = crate::R<u16, ADDRESS_BROADCAST_ADDRESS_BR_A>;
impl ADDRESS_BROADCAST_ADDRESS_BR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ADDRESS_BROADCAST_ADDRESS_BR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADDRESS_BROADCAST_ADDRESS_BR_A::ADDRESS_BROADCAST_ADDRESS_BR_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADDRESS_BROADCAST_ADDRESS_BR_DEFAULT`"]
    #[inline(always)]
    pub fn is_address_broadcast_address_br_default(&self) -> bool {
        *self == ADDRESS_BROADCAST_ADDRESS_BR_A::ADDRESS_BROADCAST_ADDRESS_BR_DEFAULT
    }
}
#[doc = "Write proxy for field `ADDRESS_BROADCAST_ADDRESS_BR`"]
pub struct ADDRESS_BROADCAST_ADDRESS_BR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_BROADCAST_ADDRESS_BR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRESS_BROADCAST_ADDRESS_BR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn address_broadcast_address_br_default(self) -> &'a mut W {
        self.variant(ADDRESS_BROADCAST_ADDRESS_BR_A::ADDRESS_BROADCAST_ADDRESS_BR_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Address of the node\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ADDRESS_ADDRESS_A {
    #[doc = "0: `0`"]
    ADDRESS_ADDRESS_DEFAULT = 0,
}
impl From<ADDRESS_ADDRESS_A> for u16 {
    #[inline(always)]
    fn from(variant: ADDRESS_ADDRESS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADDRESS_ADDRESS`"]
pub type ADDRESS_ADDRESS_R = crate::R<u16, ADDRESS_ADDRESS_A>;
impl ADDRESS_ADDRESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ADDRESS_ADDRESS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADDRESS_ADDRESS_A::ADDRESS_ADDRESS_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADDRESS_ADDRESS_DEFAULT`"]
    #[inline(always)]
    pub fn is_address_address_default(&self) -> bool {
        *self == ADDRESS_ADDRESS_A::ADDRESS_ADDRESS_DEFAULT
    }
}
#[doc = "Write proxy for field `ADDRESS_ADDRESS`"]
pub struct ADDRESS_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_ADDRESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRESS_ADDRESS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn address_address_default(self) -> &'a mut W {
        self.variant(ADDRESS_ADDRESS_A::ADDRESS_ADDRESS_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Broadcast address"]
    #[inline(always)]
    pub fn address_broadcast_address_br(&self) -> ADDRESS_BROADCAST_ADDRESS_BR_R {
        ADDRESS_BROADCAST_ADDRESS_BR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Address of the node"]
    #[inline(always)]
    pub fn address_address(&self) -> ADDRESS_ADDRESS_R {
        ADDRESS_ADDRESS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Broadcast address"]
    #[inline(always)]
    pub fn address_broadcast_address_br(&mut self) -> ADDRESS_BROADCAST_ADDRESS_BR_W {
        ADDRESS_BROADCAST_ADDRESS_BR_W { w: self }
    }
    #[doc = "Bits 0:15 - Address of the node"]
    #[inline(always)]
    pub fn address_address(&mut self) -> ADDRESS_ADDRESS_W {
        ADDRESS_ADDRESS_W { w: self }
    }
}
