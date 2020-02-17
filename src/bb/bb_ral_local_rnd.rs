#[doc = "Reader of register BB_RAL_LOCAL_RND"]
pub type R = crate::R<u32, super::BB_RAL_LOCAL_RND>;
#[doc = "Writer for register BB_RAL_LOCAL_RND"]
pub type W = crate::W<u32, super::BB_RAL_LOCAL_RND>;
#[doc = "Register BB_RAL_LOCAL_RND `reset()`'s with value 0x003f_0f0f"]
impl crate::ResetValue for super::BB_RAL_LOCAL_RND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x003f_0f0f
    }
}
#[doc = "Writing a 1 initializes of local RPA random number generation LFSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRND_INIT_A {
    #[doc = "0: `0`"]
    LRND_INIT_0 = 0,
}
impl From<LRND_INIT_A> for bool {
    #[inline(always)]
    fn from(variant: LRND_INIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LRND_INIT`"]
pub type LRND_INIT_R = crate::R<bool, LRND_INIT_A>;
impl LRND_INIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, LRND_INIT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(LRND_INIT_A::LRND_INIT_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LRND_INIT_0`"]
    #[inline(always)]
    pub fn is_lrnd_init_0(&self) -> bool {
        *self == LRND_INIT_A::LRND_INIT_0
    }
}
#[doc = "Write proxy for field `LRND_INIT`"]
pub struct LRND_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> LRND_INIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LRND_INIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn lrnd_init_0(self) -> &'a mut W {
        self.variant(LRND_INIT_A::LRND_INIT_0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Initialization value for local RPA random generation when LRDN_INIT is set to 1, else reports the current Local RPA random number LFSR value\n\nValue on reset: 4132623"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum LRND_VAL_A {
    #[doc = "4132623: `1111110000111100001111`"]
    LRND_VAL_4132623 = 4132623,
}
impl From<LRND_VAL_A> for u32 {
    #[inline(always)]
    fn from(variant: LRND_VAL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LRND_VAL`"]
pub type LRND_VAL_R = crate::R<u32, LRND_VAL_A>;
impl LRND_VAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, LRND_VAL_A> {
        use crate::Variant::*;
        match self.bits {
            4132623 => Val(LRND_VAL_A::LRND_VAL_4132623),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LRND_VAL_4132623`"]
    #[inline(always)]
    pub fn is_lrnd_val_4132623(&self) -> bool {
        *self == LRND_VAL_A::LRND_VAL_4132623
    }
}
#[doc = "Write proxy for field `LRND_VAL`"]
pub struct LRND_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LRND_VAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LRND_VAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1111110000111100001111`"]
    #[inline(always)]
    pub fn lrnd_val_4132623(self) -> &'a mut W {
        self.variant(LRND_VAL_A::LRND_VAL_4132623)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Writing a 1 initializes of local RPA random number generation LFSR"]
    #[inline(always)]
    pub fn lrnd_init(&self) -> LRND_INIT_R {
        LRND_INIT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:21 - Initialization value for local RPA random generation when LRDN_INIT is set to 1, else reports the current Local RPA random number LFSR value"]
    #[inline(always)]
    pub fn lrnd_val(&self) -> LRND_VAL_R {
        LRND_VAL_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - Writing a 1 initializes of local RPA random number generation LFSR"]
    #[inline(always)]
    pub fn lrnd_init(&mut self) -> LRND_INIT_W {
        LRND_INIT_W { w: self }
    }
    #[doc = "Bits 0:21 - Initialization value for local RPA random generation when LRDN_INIT is set to 1, else reports the current Local RPA random number LFSR value"]
    #[inline(always)]
    pub fn lrnd_val(&mut self) -> LRND_VAL_W {
        LRND_VAL_W { w: self }
    }
}
