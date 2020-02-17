#[doc = "Reader of register RF_TX_PULSE0"]
pub type R = crate::R<u32, super::RF_TX_PULSE0>;
#[doc = "Writer for register RF_TX_PULSE0"]
pub type W = crate::W<u32, super::RF_TX_PULSE0>;
#[doc = "Register RF_TX_PULSE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_TX_PULSE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_PULSE_SHAPE_1_TX_COEF4_A {
    #[doc = "0: `0`"]
    TX_PULSE_SHAPE_1_TX_COEF4_DEFAULT = 0,
}
impl From<TX_PULSE_SHAPE_1_TX_COEF4_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PULSE_SHAPE_1_TX_COEF4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_PULSE_SHAPE_1_TX_COEF4`"]
pub type TX_PULSE_SHAPE_1_TX_COEF4_R = crate::R<u8, TX_PULSE_SHAPE_1_TX_COEF4_A>;
impl TX_PULSE_SHAPE_1_TX_COEF4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_PULSE_SHAPE_1_TX_COEF4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_PULSE_SHAPE_1_TX_COEF4_A::TX_PULSE_SHAPE_1_TX_COEF4_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_PULSE_SHAPE_1_TX_COEF4_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_pulse_shape_1_tx_coef4_default(&self) -> bool {
        *self == TX_PULSE_SHAPE_1_TX_COEF4_A::TX_PULSE_SHAPE_1_TX_COEF4_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_PULSE_SHAPE_1_TX_COEF4`"]
pub struct TX_PULSE_SHAPE_1_TX_COEF4_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PULSE_SHAPE_1_TX_COEF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PULSE_SHAPE_1_TX_COEF4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_pulse_shape_1_tx_coef4_default(self) -> &'a mut W {
        self.variant(TX_PULSE_SHAPE_1_TX_COEF4_A::TX_PULSE_SHAPE_1_TX_COEF4_DEFAULT)
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
pub enum TX_PULSE_SHAPE_1_TX_COEF3_A {
    #[doc = "0: `0`"]
    TX_PULSE_SHAPE_1_TX_COEF3_DEFAULT = 0,
}
impl From<TX_PULSE_SHAPE_1_TX_COEF3_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PULSE_SHAPE_1_TX_COEF3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_PULSE_SHAPE_1_TX_COEF3`"]
pub type TX_PULSE_SHAPE_1_TX_COEF3_R = crate::R<u8, TX_PULSE_SHAPE_1_TX_COEF3_A>;
impl TX_PULSE_SHAPE_1_TX_COEF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_PULSE_SHAPE_1_TX_COEF3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_PULSE_SHAPE_1_TX_COEF3_A::TX_PULSE_SHAPE_1_TX_COEF3_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_PULSE_SHAPE_1_TX_COEF3_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_pulse_shape_1_tx_coef3_default(&self) -> bool {
        *self == TX_PULSE_SHAPE_1_TX_COEF3_A::TX_PULSE_SHAPE_1_TX_COEF3_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_PULSE_SHAPE_1_TX_COEF3`"]
pub struct TX_PULSE_SHAPE_1_TX_COEF3_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PULSE_SHAPE_1_TX_COEF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PULSE_SHAPE_1_TX_COEF3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_pulse_shape_1_tx_coef3_default(self) -> &'a mut W {
        self.variant(TX_PULSE_SHAPE_1_TX_COEF3_A::TX_PULSE_SHAPE_1_TX_COEF3_DEFAULT)
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
pub enum TX_PULSE_SHAPE_1_TX_COEF2_A {
    #[doc = "0: `0`"]
    TX_PULSE_SHAPE_1_TX_COEF2_DEFAULT = 0,
}
impl From<TX_PULSE_SHAPE_1_TX_COEF2_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PULSE_SHAPE_1_TX_COEF2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_PULSE_SHAPE_1_TX_COEF2`"]
pub type TX_PULSE_SHAPE_1_TX_COEF2_R = crate::R<u8, TX_PULSE_SHAPE_1_TX_COEF2_A>;
impl TX_PULSE_SHAPE_1_TX_COEF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_PULSE_SHAPE_1_TX_COEF2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_PULSE_SHAPE_1_TX_COEF2_A::TX_PULSE_SHAPE_1_TX_COEF2_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_PULSE_SHAPE_1_TX_COEF2_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_pulse_shape_1_tx_coef2_default(&self) -> bool {
        *self == TX_PULSE_SHAPE_1_TX_COEF2_A::TX_PULSE_SHAPE_1_TX_COEF2_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_PULSE_SHAPE_1_TX_COEF2`"]
pub struct TX_PULSE_SHAPE_1_TX_COEF2_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PULSE_SHAPE_1_TX_COEF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PULSE_SHAPE_1_TX_COEF2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_pulse_shape_1_tx_coef2_default(self) -> &'a mut W {
        self.variant(TX_PULSE_SHAPE_1_TX_COEF2_A::TX_PULSE_SHAPE_1_TX_COEF2_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "These registers specify the Tx pulse shape. The pulse shape is formed by: coef1-coef16-coef16-coef1. Since the oversampling ratio is 8, the pulse shape is 4 symbols long. Every coefficient is an 8 bits signed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_PULSE_SHAPE_1_TX_COEF1_A {
    #[doc = "0: `0`"]
    TX_PULSE_SHAPE_1_TX_COEF1_DEFAULT = 0,
}
impl From<TX_PULSE_SHAPE_1_TX_COEF1_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PULSE_SHAPE_1_TX_COEF1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_PULSE_SHAPE_1_TX_COEF1`"]
pub type TX_PULSE_SHAPE_1_TX_COEF1_R = crate::R<u8, TX_PULSE_SHAPE_1_TX_COEF1_A>;
impl TX_PULSE_SHAPE_1_TX_COEF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_PULSE_SHAPE_1_TX_COEF1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_PULSE_SHAPE_1_TX_COEF1_A::TX_PULSE_SHAPE_1_TX_COEF1_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_PULSE_SHAPE_1_TX_COEF1_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_pulse_shape_1_tx_coef1_default(&self) -> bool {
        *self == TX_PULSE_SHAPE_1_TX_COEF1_A::TX_PULSE_SHAPE_1_TX_COEF1_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_PULSE_SHAPE_1_TX_COEF1`"]
pub struct TX_PULSE_SHAPE_1_TX_COEF1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PULSE_SHAPE_1_TX_COEF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PULSE_SHAPE_1_TX_COEF1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_pulse_shape_1_tx_coef1_default(self) -> &'a mut W {
        self.variant(TX_PULSE_SHAPE_1_TX_COEF1_A::TX_PULSE_SHAPE_1_TX_COEF1_DEFAULT)
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
    pub fn tx_pulse_shape_1_tx_coef4(&self) -> TX_PULSE_SHAPE_1_TX_COEF4_R {
        TX_PULSE_SHAPE_1_TX_COEF4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn tx_pulse_shape_1_tx_coef3(&self) -> TX_PULSE_SHAPE_1_TX_COEF3_R {
        TX_PULSE_SHAPE_1_TX_COEF3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tx_pulse_shape_1_tx_coef2(&self) -> TX_PULSE_SHAPE_1_TX_COEF2_R {
        TX_PULSE_SHAPE_1_TX_COEF2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - These registers specify the Tx pulse shape. The pulse shape is formed by: coef1-coef16-coef16-coef1. Since the oversampling ratio is 8, the pulse shape is 4 symbols long. Every coefficient is an 8 bits signed."]
    #[inline(always)]
    pub fn tx_pulse_shape_1_tx_coef1(&self) -> TX_PULSE_SHAPE_1_TX_COEF1_R {
        TX_PULSE_SHAPE_1_TX_COEF1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn tx_pulse_shape_1_tx_coef4(&mut self) -> TX_PULSE_SHAPE_1_TX_COEF4_W {
        TX_PULSE_SHAPE_1_TX_COEF4_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn tx_pulse_shape_1_tx_coef3(&mut self) -> TX_PULSE_SHAPE_1_TX_COEF3_W {
        TX_PULSE_SHAPE_1_TX_COEF3_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tx_pulse_shape_1_tx_coef2(&mut self) -> TX_PULSE_SHAPE_1_TX_COEF2_W {
        TX_PULSE_SHAPE_1_TX_COEF2_W { w: self }
    }
    #[doc = "Bits 0:7 - These registers specify the Tx pulse shape. The pulse shape is formed by: coef1-coef16-coef16-coef1. Since the oversampling ratio is 8, the pulse shape is 4 symbols long. Every coefficient is an 8 bits signed."]
    #[inline(always)]
    pub fn tx_pulse_shape_1_tx_coef1(&mut self) -> TX_PULSE_SHAPE_1_TX_COEF1_W {
        TX_PULSE_SHAPE_1_TX_COEF1_W { w: self }
    }
}
