#[doc = "Reader of register BB_FINETIMECNT"]
pub type R = crate::R<u32, super::BB_FINETIMECNT>;
#[doc = "Value of the current us fine time reference counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum FINECNT_A {
    #[doc = "0: `0`"]
    FINECNT_0 = 0,
}
impl From<FINECNT_A> for u16 {
    #[inline(always)]
    fn from(variant: FINECNT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FINECNT`"]
pub type FINECNT_R = crate::R<u16, FINECNT_A>;
impl FINECNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, FINECNT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FINECNT_A::FINECNT_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FINECNT_0`"]
    #[inline(always)]
    pub fn is_finecnt_0(&self) -> bool {
        *self == FINECNT_A::FINECNT_0
    }
}
impl R {
    #[doc = "Bits 0:9 - Value of the current us fine time reference counter"]
    #[inline(always)]
    pub fn finecnt(&self) -> FINECNT_R {
        FINECNT_R::new((self.bits & 0x03ff) as u16)
    }
}
