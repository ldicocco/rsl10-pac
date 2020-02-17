#[doc = "Reader of register ACS_VDDA_CP_CTRL"]
pub type R = crate::R<u32, super::ACS_VDDA_CP_CTRL>;
#[doc = "Writer for register ACS_VDDA_CP_CTRL"]
pub type W = crate::W<u32, super::ACS_VDDA_CP_CTRL>;
#[doc = "Register ACS_VDDA_CP_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ACS_VDDA_CP_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Output power trimming\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PTRIM_A {
    #[doc = "0: Charge pump max current to 4 mA"]
    VDDA_PTRIM_4MA = 0,
    #[doc = "1: Charge pump max current to 8 mA"]
    VDDA_PTRIM_8MA = 1,
    #[doc = "2: Charge pump max current to 12 mA"]
    VDDA_PTRIM_12MA = 2,
    #[doc = "3: Charge pump max current to 16 mA"]
    VDDA_PTRIM_16MA = 3,
}
impl From<PTRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: PTRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PTRIM`"]
pub type PTRIM_R = crate::R<u8, PTRIM_A>;
impl PTRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTRIM_A {
        match self.bits {
            0 => PTRIM_A::VDDA_PTRIM_4MA,
            1 => PTRIM_A::VDDA_PTRIM_8MA,
            2 => PTRIM_A::VDDA_PTRIM_12MA,
            3 => PTRIM_A::VDDA_PTRIM_16MA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VDDA_PTRIM_4MA`"]
    #[inline(always)]
    pub fn is_vdda_ptrim_4ma(&self) -> bool {
        *self == PTRIM_A::VDDA_PTRIM_4MA
    }
    #[doc = "Checks if the value of the field is `VDDA_PTRIM_8MA`"]
    #[inline(always)]
    pub fn is_vdda_ptrim_8ma(&self) -> bool {
        *self == PTRIM_A::VDDA_PTRIM_8MA
    }
    #[doc = "Checks if the value of the field is `VDDA_PTRIM_12MA`"]
    #[inline(always)]
    pub fn is_vdda_ptrim_12ma(&self) -> bool {
        *self == PTRIM_A::VDDA_PTRIM_12MA
    }
    #[doc = "Checks if the value of the field is `VDDA_PTRIM_16MA`"]
    #[inline(always)]
    pub fn is_vdda_ptrim_16ma(&self) -> bool {
        *self == PTRIM_A::VDDA_PTRIM_16MA
    }
}
#[doc = "Write proxy for field `PTRIM`"]
pub struct PTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PTRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTRIM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Charge pump max current to 4 mA"]
    #[inline(always)]
    pub fn vdda_ptrim_4ma(self) -> &'a mut W {
        self.variant(PTRIM_A::VDDA_PTRIM_4MA)
    }
    #[doc = "Charge pump max current to 8 mA"]
    #[inline(always)]
    pub fn vdda_ptrim_8ma(self) -> &'a mut W {
        self.variant(PTRIM_A::VDDA_PTRIM_8MA)
    }
    #[doc = "Charge pump max current to 12 mA"]
    #[inline(always)]
    pub fn vdda_ptrim_12ma(self) -> &'a mut W {
        self.variant(PTRIM_A::VDDA_PTRIM_12MA)
    }
    #[doc = "Charge pump max current to 16 mA"]
    #[inline(always)]
    pub fn vdda_ptrim_16ma(self) -> &'a mut W {
        self.variant(PTRIM_A::VDDA_PTRIM_16MA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Output power trimming"]
    #[inline(always)]
    pub fn ptrim(&self) -> PTRIM_R {
        PTRIM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Output power trimming"]
    #[inline(always)]
    pub fn ptrim(&mut self) -> PTRIM_W {
        PTRIM_W { w: self }
    }
}
