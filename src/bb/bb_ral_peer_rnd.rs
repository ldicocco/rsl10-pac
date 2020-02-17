#[doc = "Reader of register BB_RAL_PEER_RND"]
pub type R = crate::R<u32, super::BB_RAL_PEER_RND>;
#[doc = "Writer for register BB_RAL_PEER_RND"]
pub type W = crate::W<u32, super::BB_RAL_PEER_RND>;
#[doc = "Register BB_RAL_PEER_RND `reset()`'s with value 0x0030_f0f0"]
impl crate::ResetValue for super::BB_RAL_PEER_RND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0030_f0f0
    }
}
#[doc = "Writing a 1 initializes of peer RPA random number generation LFSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRND_INIT_A {
    #[doc = "0: `0`"]
    PRND_INIT_0 = 0,
}
impl From<PRND_INIT_A> for bool {
    #[inline(always)]
    fn from(variant: PRND_INIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRND_INIT`"]
pub type PRND_INIT_R = crate::R<bool, PRND_INIT_A>;
impl PRND_INIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PRND_INIT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PRND_INIT_A::PRND_INIT_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRND_INIT_0`"]
    #[inline(always)]
    pub fn is_prnd_init_0(&self) -> bool {
        *self == PRND_INIT_A::PRND_INIT_0
    }
}
#[doc = "Write proxy for field `PRND_INIT`"]
pub struct PRND_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PRND_INIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRND_INIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn prnd_init_0(self) -> &'a mut W {
        self.variant(PRND_INIT_A::PRND_INIT_0)
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
#[doc = "Initialization value for peer RPA random generation when LRDN_INIT is set to 1, else reports the current Local RPA random number LFSR value\n\nValue on reset: 3207408"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PRND_VAL_A {
    #[doc = "3207408: `1100001111000011110000`"]
    PRND_VAL_3207408 = 3207408,
}
impl From<PRND_VAL_A> for u32 {
    #[inline(always)]
    fn from(variant: PRND_VAL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRND_VAL`"]
pub type PRND_VAL_R = crate::R<u32, PRND_VAL_A>;
impl PRND_VAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PRND_VAL_A> {
        use crate::Variant::*;
        match self.bits {
            3207408 => Val(PRND_VAL_A::PRND_VAL_3207408),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRND_VAL_3207408`"]
    #[inline(always)]
    pub fn is_prnd_val_3207408(&self) -> bool {
        *self == PRND_VAL_A::PRND_VAL_3207408
    }
}
#[doc = "Write proxy for field `PRND_VAL`"]
pub struct PRND_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRND_VAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRND_VAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1100001111000011110000`"]
    #[inline(always)]
    pub fn prnd_val_3207408(self) -> &'a mut W {
        self.variant(PRND_VAL_A::PRND_VAL_3207408)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Writing a 1 initializes of peer RPA random number generation LFSR"]
    #[inline(always)]
    pub fn prnd_init(&self) -> PRND_INIT_R {
        PRND_INIT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:21 - Initialization value for peer RPA random generation when LRDN_INIT is set to 1, else reports the current Local RPA random number LFSR value"]
    #[inline(always)]
    pub fn prnd_val(&self) -> PRND_VAL_R {
        PRND_VAL_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - Writing a 1 initializes of peer RPA random number generation LFSR"]
    #[inline(always)]
    pub fn prnd_init(&mut self) -> PRND_INIT_W {
        PRND_INIT_W { w: self }
    }
    #[doc = "Bits 0:21 - Initialization value for peer RPA random generation when LRDN_INIT is set to 1, else reports the current Local RPA random number LFSR value"]
    #[inline(always)]
    pub fn prnd_val(&mut self) -> PRND_VAL_W {
        PRND_VAL_W { w: self }
    }
}
