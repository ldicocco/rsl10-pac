#[doc = "Reader of register BB_TXMICVAL"]
pub type R = crate::R<u32, super::BB_TXMICVAL>;
#[doc = "AES-CCM plain MIC value. Valid on when MIC has been calculated (in Tx)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum TXMICVAL_A {
    #[doc = "0: `0`"]
    TXMICVAL_0 = 0,
}
impl From<TXMICVAL_A> for u32 {
    #[inline(always)]
    fn from(variant: TXMICVAL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXMICVAL`"]
pub type TXMICVAL_R = crate::R<u32, TXMICVAL_A>;
impl TXMICVAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, TXMICVAL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXMICVAL_A::TXMICVAL_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXMICVAL_0`"]
    #[inline(always)]
    pub fn is_txmicval_0(&self) -> bool {
        *self == TXMICVAL_A::TXMICVAL_0
    }
}
impl R {
    #[doc = "Bits 0:31 - AES-CCM plain MIC value. Valid on when MIC has been calculated (in Tx)"]
    #[inline(always)]
    pub fn txmicval(&self) -> TXMICVAL_R {
        TXMICVAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
