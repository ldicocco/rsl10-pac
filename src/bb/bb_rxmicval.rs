#[doc = "Reader of register BB_RXMICVAL"]
pub type R = crate::R<u32, super::BB_RXMICVAL>;
#[doc = "AES-CCM plain MIC value. Valid on once MIC has been extracted from Rx packet\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum RXMICVAL_A {
    #[doc = "0: `0`"]
    RXMICVAL_0 = 0,
}
impl From<RXMICVAL_A> for u32 {
    #[inline(always)]
    fn from(variant: RXMICVAL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXMICVAL`"]
pub type RXMICVAL_R = crate::R<u32, RXMICVAL_A>;
impl RXMICVAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, RXMICVAL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXMICVAL_A::RXMICVAL_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RXMICVAL_0`"]
    #[inline(always)]
    pub fn is_rxmicval_0(&self) -> bool {
        *self == RXMICVAL_A::RXMICVAL_0
    }
}
impl R {
    #[doc = "Bits 0:31 - AES-CCM plain MIC value. Valid on once MIC has been extracted from Rx packet"]
    #[inline(always)]
    pub fn rxmicval(&self) -> RXMICVAL_R {
        RXMICVAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
