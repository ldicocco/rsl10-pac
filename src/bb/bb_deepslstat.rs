#[doc = "Reader of register BB_DEEPSLSTAT"]
pub type R = crate::R<u32, super::BB_DEEPSLSTAT>;
#[doc = "Actual duration of the last deep sleep phase measured in low_power_clk clock cycle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum DEEPSLDUR_A {
    #[doc = "0: `0`"]
    DEEPSLDUR_0 = 0,
}
impl From<DEEPSLDUR_A> for u32 {
    #[inline(always)]
    fn from(variant: DEEPSLDUR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DEEPSLDUR`"]
pub type DEEPSLDUR_R = crate::R<u32, DEEPSLDUR_A>;
impl DEEPSLDUR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, DEEPSLDUR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DEEPSLDUR_A::DEEPSLDUR_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEEPSLDUR_0`"]
    #[inline(always)]
    pub fn is_deepsldur_0(&self) -> bool {
        *self == DEEPSLDUR_A::DEEPSLDUR_0
    }
}
impl R {
    #[doc = "Bits 0:31 - Actual duration of the last deep sleep phase measured in low_power_clk clock cycle"]
    #[inline(always)]
    pub fn deepsldur(&self) -> DEEPSLDUR_R {
        DEEPSLDUR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
