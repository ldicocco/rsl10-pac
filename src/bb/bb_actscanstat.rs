#[doc = "Reader of register BB_ACTSCANSTAT"]
pub type R = crate::R<u32, super::BB_ACTSCANSTAT>;
#[doc = "Active scan mode back-off counter initialization value\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum BACKOFF_A {
    #[doc = "1: `1`"]
    BACKOFF_1 = 1,
}
impl From<BACKOFF_A> for u16 {
    #[inline(always)]
    fn from(variant: BACKOFF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BACKOFF`"]
pub type BACKOFF_R = crate::R<u16, BACKOFF_A>;
impl BACKOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, BACKOFF_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(BACKOFF_A::BACKOFF_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BACKOFF_1`"]
    #[inline(always)]
    pub fn is_backoff_1(&self) -> bool {
        *self == BACKOFF_A::BACKOFF_1
    }
}
#[doc = "Active scan mode upper limit counter value\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum UPPERLIMIT_A {
    #[doc = "1: `1`"]
    UPPERLIMIT_1 = 1,
}
impl From<UPPERLIMIT_A> for u16 {
    #[inline(always)]
    fn from(variant: UPPERLIMIT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UPPERLIMIT`"]
pub type UPPERLIMIT_R = crate::R<u16, UPPERLIMIT_A>;
impl UPPERLIMIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, UPPERLIMIT_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(UPPERLIMIT_A::UPPERLIMIT_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UPPERLIMIT_1`"]
    #[inline(always)]
    pub fn is_upperlimit_1(&self) -> bool {
        *self == UPPERLIMIT_A::UPPERLIMIT_1
    }
}
impl R {
    #[doc = "Bits 16:24 - Active scan mode back-off counter initialization value"]
    #[inline(always)]
    pub fn backoff(&self) -> BACKOFF_R {
        BACKOFF_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 0:8 - Active scan mode upper limit counter value"]
    #[inline(always)]
    pub fn upperlimit(&self) -> UPPERLIMIT_R {
        UPPERLIMIT_R::new((self.bits & 0x01ff) as u16)
    }
}
