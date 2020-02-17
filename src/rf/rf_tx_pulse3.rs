#[doc = "Reader of register RF_TX_PULSE3"]
pub type R = crate::R<u32, super::RF_TX_PULSE3>;
#[doc = "Writer for register RF_TX_PULSE3"]
pub type W = crate::W<u32, super::RF_TX_PULSE3>;
#[doc = "Register RF_TX_PULSE3 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_TX_PULSE3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_PULSE_SHAPE_4_TX_COEF16_A {
    #[doc = "0: `0`"]
    TX_PULSE_SHAPE_4_TX_COEF16_DEFAULT = 0,
}
impl From<TX_PULSE_SHAPE_4_TX_COEF16_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PULSE_SHAPE_4_TX_COEF16_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_PULSE_SHAPE_4_TX_COEF16`"]
pub type TX_PULSE_SHAPE_4_TX_COEF16_R = crate::R<u8, TX_PULSE_SHAPE_4_TX_COEF16_A>;
impl TX_PULSE_SHAPE_4_TX_COEF16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_PULSE_SHAPE_4_TX_COEF16_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_PULSE_SHAPE_4_TX_COEF16_A::TX_PULSE_SHAPE_4_TX_COEF16_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_PULSE_SHAPE_4_TX_COEF16_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_pulse_shape_4_tx_coef16_default(&self) -> bool {
        *self == TX_PULSE_SHAPE_4_TX_COEF16_A::TX_PULSE_SHAPE_4_TX_COEF16_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_PULSE_SHAPE_4_TX_COEF16`"]
pub struct TX_PULSE_SHAPE_4_TX_COEF16_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PULSE_SHAPE_4_TX_COEF16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PULSE_SHAPE_4_TX_COEF16_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_pulse_shape_4_tx_coef16_default(self) -> &'a mut W {
        self.variant(TX_PULSE_SHAPE_4_TX_COEF16_A::TX_PULSE_SHAPE_4_TX_COEF16_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_PULSE_SHAPE_4_TX_COEF15_A {
    #[doc = "0: `0`"]
    TX_PULSE_SHAPE_4_TX_COEF15_DEFAULT = 0,
}
impl From<TX_PULSE_SHAPE_4_TX_COEF15_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PULSE_SHAPE_4_TX_COEF15_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_PULSE_SHAPE_4_TX_COEF15`"]
pub type TX_PULSE_SHAPE_4_TX_COEF15_R = crate::R<u8, TX_PULSE_SHAPE_4_TX_COEF15_A>;
impl TX_PULSE_SHAPE_4_TX_COEF15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_PULSE_SHAPE_4_TX_COEF15_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_PULSE_SHAPE_4_TX_COEF15_A::TX_PULSE_SHAPE_4_TX_COEF15_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_PULSE_SHAPE_4_TX_COEF15_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_pulse_shape_4_tx_coef15_default(&self) -> bool {
        *self == TX_PULSE_SHAPE_4_TX_COEF15_A::TX_PULSE_SHAPE_4_TX_COEF15_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_PULSE_SHAPE_4_TX_COEF15`"]
pub struct TX_PULSE_SHAPE_4_TX_COEF15_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PULSE_SHAPE_4_TX_COEF15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PULSE_SHAPE_4_TX_COEF15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_pulse_shape_4_tx_coef15_default(self) -> &'a mut W {
        self.variant(TX_PULSE_SHAPE_4_TX_COEF15_A::TX_PULSE_SHAPE_4_TX_COEF15_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_PULSE_SHAPE_4_TX_COEF14_A {
    #[doc = "0: `0`"]
    TX_PULSE_SHAPE_4_TX_COEF14_DEFAULT = 0,
}
impl From<TX_PULSE_SHAPE_4_TX_COEF14_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PULSE_SHAPE_4_TX_COEF14_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_PULSE_SHAPE_4_TX_COEF14`"]
pub type TX_PULSE_SHAPE_4_TX_COEF14_R = crate::R<u8, TX_PULSE_SHAPE_4_TX_COEF14_A>;
impl TX_PULSE_SHAPE_4_TX_COEF14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_PULSE_SHAPE_4_TX_COEF14_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_PULSE_SHAPE_4_TX_COEF14_A::TX_PULSE_SHAPE_4_TX_COEF14_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_PULSE_SHAPE_4_TX_COEF14_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_pulse_shape_4_tx_coef14_default(&self) -> bool {
        *self == TX_PULSE_SHAPE_4_TX_COEF14_A::TX_PULSE_SHAPE_4_TX_COEF14_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_PULSE_SHAPE_4_TX_COEF14`"]
pub struct TX_PULSE_SHAPE_4_TX_COEF14_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PULSE_SHAPE_4_TX_COEF14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PULSE_SHAPE_4_TX_COEF14_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_pulse_shape_4_tx_coef14_default(self) -> &'a mut W {
        self.variant(TX_PULSE_SHAPE_4_TX_COEF14_A::TX_PULSE_SHAPE_4_TX_COEF14_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_PULSE_SHAPE_4_TX_COEF13_A {
    #[doc = "0: `0`"]
    TX_PULSE_SHAPE_4_TX_COEF13_DEFAULT = 0,
}
impl From<TX_PULSE_SHAPE_4_TX_COEF13_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PULSE_SHAPE_4_TX_COEF13_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_PULSE_SHAPE_4_TX_COEF13`"]
pub type TX_PULSE_SHAPE_4_TX_COEF13_R = crate::R<u8, TX_PULSE_SHAPE_4_TX_COEF13_A>;
impl TX_PULSE_SHAPE_4_TX_COEF13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_PULSE_SHAPE_4_TX_COEF13_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_PULSE_SHAPE_4_TX_COEF13_A::TX_PULSE_SHAPE_4_TX_COEF13_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_PULSE_SHAPE_4_TX_COEF13_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_pulse_shape_4_tx_coef13_default(&self) -> bool {
        *self == TX_PULSE_SHAPE_4_TX_COEF13_A::TX_PULSE_SHAPE_4_TX_COEF13_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_PULSE_SHAPE_4_TX_COEF13`"]
pub struct TX_PULSE_SHAPE_4_TX_COEF13_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PULSE_SHAPE_4_TX_COEF13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PULSE_SHAPE_4_TX_COEF13_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_pulse_shape_4_tx_coef13_default(self) -> &'a mut W {
        self.variant(TX_PULSE_SHAPE_4_TX_COEF13_A::TX_PULSE_SHAPE_4_TX_COEF13_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn tx_pulse_shape_4_tx_coef16(&self) -> TX_PULSE_SHAPE_4_TX_COEF16_R {
        TX_PULSE_SHAPE_4_TX_COEF16_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn tx_pulse_shape_4_tx_coef15(&self) -> TX_PULSE_SHAPE_4_TX_COEF15_R {
        TX_PULSE_SHAPE_4_TX_COEF15_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tx_pulse_shape_4_tx_coef14(&self) -> TX_PULSE_SHAPE_4_TX_COEF14_R {
        TX_PULSE_SHAPE_4_TX_COEF14_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tx_pulse_shape_4_tx_coef13(&self) -> TX_PULSE_SHAPE_4_TX_COEF13_R {
        TX_PULSE_SHAPE_4_TX_COEF13_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn tx_pulse_shape_4_tx_coef16(&mut self) -> TX_PULSE_SHAPE_4_TX_COEF16_W {
        TX_PULSE_SHAPE_4_TX_COEF16_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn tx_pulse_shape_4_tx_coef15(&mut self) -> TX_PULSE_SHAPE_4_TX_COEF15_W {
        TX_PULSE_SHAPE_4_TX_COEF15_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tx_pulse_shape_4_tx_coef14(&mut self) -> TX_PULSE_SHAPE_4_TX_COEF14_W {
        TX_PULSE_SHAPE_4_TX_COEF14_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tx_pulse_shape_4_tx_coef13(&mut self) -> TX_PULSE_SHAPE_4_TX_COEF13_W {
        TX_PULSE_SHAPE_4_TX_COEF13_W { w: self }
    }
}
