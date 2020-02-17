#[doc = "Reader of register RF_TX_PULSE2"]
pub type R = crate::R<u32, super::RF_TX_PULSE2>;
#[doc = "Writer for register RF_TX_PULSE2"]
pub type W = crate::W<u32, super::RF_TX_PULSE2>;
#[doc = "Register RF_TX_PULSE2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_TX_PULSE2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_PULSE_SHAPE_3_TX_COEF12_A {
    #[doc = "0: `0`"]
    TX_PULSE_SHAPE_3_TX_COEF12_DEFAULT = 0,
}
impl From<TX_PULSE_SHAPE_3_TX_COEF12_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PULSE_SHAPE_3_TX_COEF12_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_PULSE_SHAPE_3_TX_COEF12`"]
pub type TX_PULSE_SHAPE_3_TX_COEF12_R = crate::R<u8, TX_PULSE_SHAPE_3_TX_COEF12_A>;
impl TX_PULSE_SHAPE_3_TX_COEF12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_PULSE_SHAPE_3_TX_COEF12_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_PULSE_SHAPE_3_TX_COEF12_A::TX_PULSE_SHAPE_3_TX_COEF12_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_PULSE_SHAPE_3_TX_COEF12_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_pulse_shape_3_tx_coef12_default(&self) -> bool {
        *self == TX_PULSE_SHAPE_3_TX_COEF12_A::TX_PULSE_SHAPE_3_TX_COEF12_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_PULSE_SHAPE_3_TX_COEF12`"]
pub struct TX_PULSE_SHAPE_3_TX_COEF12_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PULSE_SHAPE_3_TX_COEF12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PULSE_SHAPE_3_TX_COEF12_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_pulse_shape_3_tx_coef12_default(self) -> &'a mut W {
        self.variant(TX_PULSE_SHAPE_3_TX_COEF12_A::TX_PULSE_SHAPE_3_TX_COEF12_DEFAULT)
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
pub enum TX_PULSE_SHAPE_3_TX_COEF11_A {
    #[doc = "0: `0`"]
    TX_PULSE_SHAPE_3_TX_COEF11_DEFAULT = 0,
}
impl From<TX_PULSE_SHAPE_3_TX_COEF11_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PULSE_SHAPE_3_TX_COEF11_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_PULSE_SHAPE_3_TX_COEF11`"]
pub type TX_PULSE_SHAPE_3_TX_COEF11_R = crate::R<u8, TX_PULSE_SHAPE_3_TX_COEF11_A>;
impl TX_PULSE_SHAPE_3_TX_COEF11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_PULSE_SHAPE_3_TX_COEF11_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_PULSE_SHAPE_3_TX_COEF11_A::TX_PULSE_SHAPE_3_TX_COEF11_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_PULSE_SHAPE_3_TX_COEF11_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_pulse_shape_3_tx_coef11_default(&self) -> bool {
        *self == TX_PULSE_SHAPE_3_TX_COEF11_A::TX_PULSE_SHAPE_3_TX_COEF11_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_PULSE_SHAPE_3_TX_COEF11`"]
pub struct TX_PULSE_SHAPE_3_TX_COEF11_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PULSE_SHAPE_3_TX_COEF11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PULSE_SHAPE_3_TX_COEF11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_pulse_shape_3_tx_coef11_default(self) -> &'a mut W {
        self.variant(TX_PULSE_SHAPE_3_TX_COEF11_A::TX_PULSE_SHAPE_3_TX_COEF11_DEFAULT)
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
pub enum TX_PULSE_SHAPE_3_TX_COEF10_A {
    #[doc = "0: `0`"]
    TX_PULSE_SHAPE_3_TX_COEF10_DEFAULT = 0,
}
impl From<TX_PULSE_SHAPE_3_TX_COEF10_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PULSE_SHAPE_3_TX_COEF10_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_PULSE_SHAPE_3_TX_COEF10`"]
pub type TX_PULSE_SHAPE_3_TX_COEF10_R = crate::R<u8, TX_PULSE_SHAPE_3_TX_COEF10_A>;
impl TX_PULSE_SHAPE_3_TX_COEF10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_PULSE_SHAPE_3_TX_COEF10_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_PULSE_SHAPE_3_TX_COEF10_A::TX_PULSE_SHAPE_3_TX_COEF10_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_PULSE_SHAPE_3_TX_COEF10_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_pulse_shape_3_tx_coef10_default(&self) -> bool {
        *self == TX_PULSE_SHAPE_3_TX_COEF10_A::TX_PULSE_SHAPE_3_TX_COEF10_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_PULSE_SHAPE_3_TX_COEF10`"]
pub struct TX_PULSE_SHAPE_3_TX_COEF10_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PULSE_SHAPE_3_TX_COEF10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PULSE_SHAPE_3_TX_COEF10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_pulse_shape_3_tx_coef10_default(self) -> &'a mut W {
        self.variant(TX_PULSE_SHAPE_3_TX_COEF10_A::TX_PULSE_SHAPE_3_TX_COEF10_DEFAULT)
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
pub enum TX_PULSE_SHAPE_3_TX_COEF9_A {
    #[doc = "0: `0`"]
    TX_PULSE_SHAPE_3_TX_COEF9_DEFAULT = 0,
}
impl From<TX_PULSE_SHAPE_3_TX_COEF9_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PULSE_SHAPE_3_TX_COEF9_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_PULSE_SHAPE_3_TX_COEF9`"]
pub type TX_PULSE_SHAPE_3_TX_COEF9_R = crate::R<u8, TX_PULSE_SHAPE_3_TX_COEF9_A>;
impl TX_PULSE_SHAPE_3_TX_COEF9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_PULSE_SHAPE_3_TX_COEF9_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_PULSE_SHAPE_3_TX_COEF9_A::TX_PULSE_SHAPE_3_TX_COEF9_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_PULSE_SHAPE_3_TX_COEF9_DEFAULT`"]
    #[inline(always)]
    pub fn is_tx_pulse_shape_3_tx_coef9_default(&self) -> bool {
        *self == TX_PULSE_SHAPE_3_TX_COEF9_A::TX_PULSE_SHAPE_3_TX_COEF9_DEFAULT
    }
}
#[doc = "Write proxy for field `TX_PULSE_SHAPE_3_TX_COEF9`"]
pub struct TX_PULSE_SHAPE_3_TX_COEF9_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PULSE_SHAPE_3_TX_COEF9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PULSE_SHAPE_3_TX_COEF9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tx_pulse_shape_3_tx_coef9_default(self) -> &'a mut W {
        self.variant(TX_PULSE_SHAPE_3_TX_COEF9_A::TX_PULSE_SHAPE_3_TX_COEF9_DEFAULT)
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
    pub fn tx_pulse_shape_3_tx_coef12(&self) -> TX_PULSE_SHAPE_3_TX_COEF12_R {
        TX_PULSE_SHAPE_3_TX_COEF12_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn tx_pulse_shape_3_tx_coef11(&self) -> TX_PULSE_SHAPE_3_TX_COEF11_R {
        TX_PULSE_SHAPE_3_TX_COEF11_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tx_pulse_shape_3_tx_coef10(&self) -> TX_PULSE_SHAPE_3_TX_COEF10_R {
        TX_PULSE_SHAPE_3_TX_COEF10_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tx_pulse_shape_3_tx_coef9(&self) -> TX_PULSE_SHAPE_3_TX_COEF9_R {
        TX_PULSE_SHAPE_3_TX_COEF9_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn tx_pulse_shape_3_tx_coef12(&mut self) -> TX_PULSE_SHAPE_3_TX_COEF12_W {
        TX_PULSE_SHAPE_3_TX_COEF12_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn tx_pulse_shape_3_tx_coef11(&mut self) -> TX_PULSE_SHAPE_3_TX_COEF11_W {
        TX_PULSE_SHAPE_3_TX_COEF11_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tx_pulse_shape_3_tx_coef10(&mut self) -> TX_PULSE_SHAPE_3_TX_COEF10_W {
        TX_PULSE_SHAPE_3_TX_COEF10_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tx_pulse_shape_3_tx_coef9(&mut self) -> TX_PULSE_SHAPE_3_TX_COEF9_W {
        TX_PULSE_SHAPE_3_TX_COEF9_W { w: self }
    }
}
