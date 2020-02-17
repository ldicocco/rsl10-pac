#[doc = "Reader of register RF_TX_PULSE1"]
pub type R = crate::R<u32, super::RF_TX_PULSE1>;
#[doc = "Writer for register RF_TX_PULSE1"]
pub type W = crate::W<u32, super::RF_TX_PULSE1>;
#[doc = "Register RF_TX_PULSE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_TX_PULSE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_PULSE_SHAPE_2_TX_COEF8_A {
    #[doc = "0: `0`"]
    TX_PULSE_SHAPE_2_TX_COEF8_DEFAULT = 0,
}
impl From<TX_PULSE_SHAPE_2_TX_COEF8_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PULSE_SHAPE_2_TX_COEF8_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_PULSE_SHAPE_2_TX_COEF8`"]
pub type TX_PULSE_SHAPE_2_TX_COEF8_R = crate::R<u8, TX_PULSE_SHAPE_2_TX_COEF8_A>;
impl TX_PULSE_SHAPE_2_TX_COEF8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_PULSE_SHAPE_2_TX_COEF8_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_PULSE_SHAPE_2_TX_COEF8_A::TX_PULSE_SHAPE_2_TX_COEF8_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_PULSE_SHAPE_2_TX_COEF8_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_pulse_shape_2_tx_coef8_default(&self) -> bool {
        *self == TX_PULSE_SHAPE_2_TX_COEF8_A::TX_PULSE_SHAPE_2_TX_COEF8_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_PULSE_SHAPE_2_TX_COEF8`"]
pub struct TX_PULSE_SHAPE_2_TX_COEF8_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PULSE_SHAPE_2_TX_COEF8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PULSE_SHAPE_2_TX_COEF8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_pulse_shape_2_tx_coef8_default(self) -> &'a mut W {
        self.variant(TX_PULSE_SHAPE_2_TX_COEF8_A::TX_PULSE_SHAPE_2_TX_COEF8_DEFAULT)
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
pub enum TX_PULSE_SHAPE_2_TX_COEF7_A {
    #[doc = "0: `0`"]
    TX_PULSE_SHAPE_2_TX_COEF7_DEFAULT = 0,
}
impl From<TX_PULSE_SHAPE_2_TX_COEF7_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PULSE_SHAPE_2_TX_COEF7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_PULSE_SHAPE_2_TX_COEF7`"]
pub type TX_PULSE_SHAPE_2_TX_COEF7_R = crate::R<u8, TX_PULSE_SHAPE_2_TX_COEF7_A>;
impl TX_PULSE_SHAPE_2_TX_COEF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_PULSE_SHAPE_2_TX_COEF7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_PULSE_SHAPE_2_TX_COEF7_A::TX_PULSE_SHAPE_2_TX_COEF7_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_PULSE_SHAPE_2_TX_COEF7_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_pulse_shape_2_tx_coef7_default(&self) -> bool {
        *self == TX_PULSE_SHAPE_2_TX_COEF7_A::TX_PULSE_SHAPE_2_TX_COEF7_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_PULSE_SHAPE_2_TX_COEF7`"]
pub struct TX_PULSE_SHAPE_2_TX_COEF7_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PULSE_SHAPE_2_TX_COEF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PULSE_SHAPE_2_TX_COEF7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_pulse_shape_2_tx_coef7_default(self) -> &'a mut W {
        self.variant(TX_PULSE_SHAPE_2_TX_COEF7_A::TX_PULSE_SHAPE_2_TX_COEF7_DEFAULT)
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
pub enum TX_PULSE_SHAPE_2_TX_COEF6_A {
    #[doc = "0: `0`"]
    TX_PULSE_SHAPE_2_TX_COEF6_DEFAULT = 0,
}
impl From<TX_PULSE_SHAPE_2_TX_COEF6_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PULSE_SHAPE_2_TX_COEF6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_PULSE_SHAPE_2_TX_COEF6`"]
pub type TX_PULSE_SHAPE_2_TX_COEF6_R = crate::R<u8, TX_PULSE_SHAPE_2_TX_COEF6_A>;
impl TX_PULSE_SHAPE_2_TX_COEF6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_PULSE_SHAPE_2_TX_COEF6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_PULSE_SHAPE_2_TX_COEF6_A::TX_PULSE_SHAPE_2_TX_COEF6_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_PULSE_SHAPE_2_TX_COEF6_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_pulse_shape_2_tx_coef6_default(&self) -> bool {
        *self == TX_PULSE_SHAPE_2_TX_COEF6_A::TX_PULSE_SHAPE_2_TX_COEF6_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_PULSE_SHAPE_2_TX_COEF6`"]
pub struct TX_PULSE_SHAPE_2_TX_COEF6_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PULSE_SHAPE_2_TX_COEF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PULSE_SHAPE_2_TX_COEF6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_pulse_shape_2_tx_coef6_default(self) -> &'a mut W {
        self.variant(TX_PULSE_SHAPE_2_TX_COEF6_A::TX_PULSE_SHAPE_2_TX_COEF6_DEFAULT)
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
pub enum TX_PULSE_SHAPE_2_TX_COEF5_A {
    #[doc = "0: `0`"]
    TX_PULSE_SHAPE_2_TX_COEF5_DEFAULT = 0,
}
impl From<TX_PULSE_SHAPE_2_TX_COEF5_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PULSE_SHAPE_2_TX_COEF5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_PULSE_SHAPE_2_TX_COEF5`"]
pub type TX_PULSE_SHAPE_2_TX_COEF5_R = crate::R<u8, TX_PULSE_SHAPE_2_TX_COEF5_A>;
impl TX_PULSE_SHAPE_2_TX_COEF5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_PULSE_SHAPE_2_TX_COEF5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_PULSE_SHAPE_2_TX_COEF5_A::TX_PULSE_SHAPE_2_TX_COEF5_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_PULSE_SHAPE_2_TX_COEF5_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_pulse_shape_2_tx_coef5_default(&self) -> bool {
        *self == TX_PULSE_SHAPE_2_TX_COEF5_A::TX_PULSE_SHAPE_2_TX_COEF5_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_PULSE_SHAPE_2_TX_COEF5`"]
pub struct TX_PULSE_SHAPE_2_TX_COEF5_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PULSE_SHAPE_2_TX_COEF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PULSE_SHAPE_2_TX_COEF5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_pulse_shape_2_tx_coef5_default(self) -> &'a mut W {
        self.variant(TX_PULSE_SHAPE_2_TX_COEF5_A::TX_PULSE_SHAPE_2_TX_COEF5_DEFAULT)
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
    pub fn tx_pulse_shape_2_tx_coef8(&self) -> TX_PULSE_SHAPE_2_TX_COEF8_R {
        TX_PULSE_SHAPE_2_TX_COEF8_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn tx_pulse_shape_2_tx_coef7(&self) -> TX_PULSE_SHAPE_2_TX_COEF7_R {
        TX_PULSE_SHAPE_2_TX_COEF7_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tx_pulse_shape_2_tx_coef6(&self) -> TX_PULSE_SHAPE_2_TX_COEF6_R {
        TX_PULSE_SHAPE_2_TX_COEF6_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tx_pulse_shape_2_tx_coef5(&self) -> TX_PULSE_SHAPE_2_TX_COEF5_R {
        TX_PULSE_SHAPE_2_TX_COEF5_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn tx_pulse_shape_2_tx_coef8(&mut self) -> TX_PULSE_SHAPE_2_TX_COEF8_W {
        TX_PULSE_SHAPE_2_TX_COEF8_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn tx_pulse_shape_2_tx_coef7(&mut self) -> TX_PULSE_SHAPE_2_TX_COEF7_W {
        TX_PULSE_SHAPE_2_TX_COEF7_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tx_pulse_shape_2_tx_coef6(&mut self) -> TX_PULSE_SHAPE_2_TX_COEF6_W {
        TX_PULSE_SHAPE_2_TX_COEF6_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tx_pulse_shape_2_tx_coef5(&mut self) -> TX_PULSE_SHAPE_2_TX_COEF5_W {
        TX_PULSE_SHAPE_2_TX_COEF5_W { w: self }
    }
}
