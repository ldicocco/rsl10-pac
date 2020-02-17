#[doc = "Reader of register SCnSCB_ICTR"]
pub type R = crate::R<u32, super::SCNSCB_ICTR>;
#[doc = "Number of interrupt inputs in steps of 32\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTLINESNUM_A {
    #[doc = "0: NVIC_INTLINESNUM_1_32"]
    NVIC_INTLINESNUM_1_32 = 0,
    #[doc = "1: NVIC_INTLINESNUM_33_64"]
    NVIC_INTLINESNUM_33_64 = 1,
    #[doc = "2: NVIC_INTLINESNUM_65_96"]
    NVIC_INTLINESNUM_65_96 = 2,
}
impl From<INTLINESNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: INTLINESNUM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INTLINESNUM`"]
pub type INTLINESNUM_R = crate::R<u8, INTLINESNUM_A>;
impl INTLINESNUM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INTLINESNUM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INTLINESNUM_A::NVIC_INTLINESNUM_1_32),
            1 => Val(INTLINESNUM_A::NVIC_INTLINESNUM_33_64),
            2 => Val(INTLINESNUM_A::NVIC_INTLINESNUM_65_96),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NVIC_INTLINESNUM_1_32`"]
    #[inline(always)]
    pub fn is_nvic_intlinesnum_1_32(&self) -> bool {
        *self == INTLINESNUM_A::NVIC_INTLINESNUM_1_32
    }
    #[doc = "Checks if the value of the field is `NVIC_INTLINESNUM_33_64`"]
    #[inline(always)]
    pub fn is_nvic_intlinesnum_33_64(&self) -> bool {
        *self == INTLINESNUM_A::NVIC_INTLINESNUM_33_64
    }
    #[doc = "Checks if the value of the field is `NVIC_INTLINESNUM_65_96`"]
    #[inline(always)]
    pub fn is_nvic_intlinesnum_65_96(&self) -> bool {
        *self == INTLINESNUM_A::NVIC_INTLINESNUM_65_96
    }
}
impl R {
    #[doc = "Bits 0:4 - Number of interrupt inputs in steps of 32"]
    #[inline(always)]
    pub fn intlinesnum(&self) -> INTLINESNUM_R {
        INTLINESNUM_R::new((self.bits & 0x1f) as u8)
    }
}
