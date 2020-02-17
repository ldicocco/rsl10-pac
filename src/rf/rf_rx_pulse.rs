#[doc = "Reader of register RF_RX_PULSE"]
pub type R = crate::R<u32, super::RF_RX_PULSE>;
#[doc = "Writer for register RF_RX_PULSE"]
pub type W = crate::W<u32, super::RF_RX_PULSE>;
#[doc = "Register RF_RX_PULSE `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_RX_PULSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_PULSE_SHAPE_RX_COEF8_A {
    #[doc = "0: `0`"]
    RX_PULSE_SHAPE_RX_COEF8_DEFAULT = 0,
}
impl From<RX_PULSE_SHAPE_RX_COEF8_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_PULSE_SHAPE_RX_COEF8_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RX_PULSE_SHAPE_RX_COEF8`"]
pub type RX_PULSE_SHAPE_RX_COEF8_R = crate::R<u8, RX_PULSE_SHAPE_RX_COEF8_A>;
impl RX_PULSE_SHAPE_RX_COEF8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RX_PULSE_SHAPE_RX_COEF8_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RX_PULSE_SHAPE_RX_COEF8_A::RX_PULSE_SHAPE_RX_COEF8_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX_PULSE_SHAPE_RX_COEF8_DEFAULT`"]
    #[inline(always)]
    pub fn is_rx_pulse_shape_rx_coef8_default(&self) -> bool {
        *self == RX_PULSE_SHAPE_RX_COEF8_A::RX_PULSE_SHAPE_RX_COEF8_DEFAULT
    }
}
#[doc = "Write proxy for field `RX_PULSE_SHAPE_RX_COEF8`"]
pub struct RX_PULSE_SHAPE_RX_COEF8_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PULSE_SHAPE_RX_COEF8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_PULSE_SHAPE_RX_COEF8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef8_default(self) -> &'a mut W {
        self.variant(RX_PULSE_SHAPE_RX_COEF8_A::RX_PULSE_SHAPE_RX_COEF8_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_PULSE_SHAPE_RX_COEF7_A {
    #[doc = "0: `0`"]
    RX_PULSE_SHAPE_RX_COEF7_DEFAULT = 0,
}
impl From<RX_PULSE_SHAPE_RX_COEF7_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_PULSE_SHAPE_RX_COEF7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RX_PULSE_SHAPE_RX_COEF7`"]
pub type RX_PULSE_SHAPE_RX_COEF7_R = crate::R<u8, RX_PULSE_SHAPE_RX_COEF7_A>;
impl RX_PULSE_SHAPE_RX_COEF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RX_PULSE_SHAPE_RX_COEF7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RX_PULSE_SHAPE_RX_COEF7_A::RX_PULSE_SHAPE_RX_COEF7_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX_PULSE_SHAPE_RX_COEF7_DEFAULT`"]
    #[inline(always)]
    pub fn is_rx_pulse_shape_rx_coef7_default(&self) -> bool {
        *self == RX_PULSE_SHAPE_RX_COEF7_A::RX_PULSE_SHAPE_RX_COEF7_DEFAULT
    }
}
#[doc = "Write proxy for field `RX_PULSE_SHAPE_RX_COEF7`"]
pub struct RX_PULSE_SHAPE_RX_COEF7_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PULSE_SHAPE_RX_COEF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_PULSE_SHAPE_RX_COEF7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef7_default(self) -> &'a mut W {
        self.variant(RX_PULSE_SHAPE_RX_COEF7_A::RX_PULSE_SHAPE_RX_COEF7_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_PULSE_SHAPE_RX_COEF6_A {
    #[doc = "0: `0`"]
    RX_PULSE_SHAPE_RX_COEF6_DEFAULT = 0,
}
impl From<RX_PULSE_SHAPE_RX_COEF6_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_PULSE_SHAPE_RX_COEF6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RX_PULSE_SHAPE_RX_COEF6`"]
pub type RX_PULSE_SHAPE_RX_COEF6_R = crate::R<u8, RX_PULSE_SHAPE_RX_COEF6_A>;
impl RX_PULSE_SHAPE_RX_COEF6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RX_PULSE_SHAPE_RX_COEF6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RX_PULSE_SHAPE_RX_COEF6_A::RX_PULSE_SHAPE_RX_COEF6_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX_PULSE_SHAPE_RX_COEF6_DEFAULT`"]
    #[inline(always)]
    pub fn is_rx_pulse_shape_rx_coef6_default(&self) -> bool {
        *self == RX_PULSE_SHAPE_RX_COEF6_A::RX_PULSE_SHAPE_RX_COEF6_DEFAULT
    }
}
#[doc = "Write proxy for field `RX_PULSE_SHAPE_RX_COEF6`"]
pub struct RX_PULSE_SHAPE_RX_COEF6_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PULSE_SHAPE_RX_COEF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_PULSE_SHAPE_RX_COEF6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef6_default(self) -> &'a mut W {
        self.variant(RX_PULSE_SHAPE_RX_COEF6_A::RX_PULSE_SHAPE_RX_COEF6_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_PULSE_SHAPE_RX_COEF5_A {
    #[doc = "0: `0`"]
    RX_PULSE_SHAPE_RX_COEF5_DEFAULT = 0,
}
impl From<RX_PULSE_SHAPE_RX_COEF5_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_PULSE_SHAPE_RX_COEF5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RX_PULSE_SHAPE_RX_COEF5`"]
pub type RX_PULSE_SHAPE_RX_COEF5_R = crate::R<u8, RX_PULSE_SHAPE_RX_COEF5_A>;
impl RX_PULSE_SHAPE_RX_COEF5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RX_PULSE_SHAPE_RX_COEF5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RX_PULSE_SHAPE_RX_COEF5_A::RX_PULSE_SHAPE_RX_COEF5_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX_PULSE_SHAPE_RX_COEF5_DEFAULT`"]
    #[inline(always)]
    pub fn is_rx_pulse_shape_rx_coef5_default(&self) -> bool {
        *self == RX_PULSE_SHAPE_RX_COEF5_A::RX_PULSE_SHAPE_RX_COEF5_DEFAULT
    }
}
#[doc = "Write proxy for field `RX_PULSE_SHAPE_RX_COEF5`"]
pub struct RX_PULSE_SHAPE_RX_COEF5_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PULSE_SHAPE_RX_COEF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_PULSE_SHAPE_RX_COEF5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef5_default(self) -> &'a mut W {
        self.variant(RX_PULSE_SHAPE_RX_COEF5_A::RX_PULSE_SHAPE_RX_COEF5_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_PULSE_SHAPE_RX_COEF4_A {
    #[doc = "0: `0`"]
    RX_PULSE_SHAPE_RX_COEF4_DEFAULT = 0,
}
impl From<RX_PULSE_SHAPE_RX_COEF4_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_PULSE_SHAPE_RX_COEF4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RX_PULSE_SHAPE_RX_COEF4`"]
pub type RX_PULSE_SHAPE_RX_COEF4_R = crate::R<u8, RX_PULSE_SHAPE_RX_COEF4_A>;
impl RX_PULSE_SHAPE_RX_COEF4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RX_PULSE_SHAPE_RX_COEF4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RX_PULSE_SHAPE_RX_COEF4_A::RX_PULSE_SHAPE_RX_COEF4_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX_PULSE_SHAPE_RX_COEF4_DEFAULT`"]
    #[inline(always)]
    pub fn is_rx_pulse_shape_rx_coef4_default(&self) -> bool {
        *self == RX_PULSE_SHAPE_RX_COEF4_A::RX_PULSE_SHAPE_RX_COEF4_DEFAULT
    }
}
#[doc = "Write proxy for field `RX_PULSE_SHAPE_RX_COEF4`"]
pub struct RX_PULSE_SHAPE_RX_COEF4_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PULSE_SHAPE_RX_COEF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_PULSE_SHAPE_RX_COEF4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef4_default(self) -> &'a mut W {
        self.variant(RX_PULSE_SHAPE_RX_COEF4_A::RX_PULSE_SHAPE_RX_COEF4_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_PULSE_SHAPE_RX_COEF3_A {
    #[doc = "0: `0`"]
    RX_PULSE_SHAPE_RX_COEF3_DEFAULT = 0,
}
impl From<RX_PULSE_SHAPE_RX_COEF3_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_PULSE_SHAPE_RX_COEF3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RX_PULSE_SHAPE_RX_COEF3`"]
pub type RX_PULSE_SHAPE_RX_COEF3_R = crate::R<u8, RX_PULSE_SHAPE_RX_COEF3_A>;
impl RX_PULSE_SHAPE_RX_COEF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RX_PULSE_SHAPE_RX_COEF3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RX_PULSE_SHAPE_RX_COEF3_A::RX_PULSE_SHAPE_RX_COEF3_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX_PULSE_SHAPE_RX_COEF3_DEFAULT`"]
    #[inline(always)]
    pub fn is_rx_pulse_shape_rx_coef3_default(&self) -> bool {
        *self == RX_PULSE_SHAPE_RX_COEF3_A::RX_PULSE_SHAPE_RX_COEF3_DEFAULT
    }
}
#[doc = "Write proxy for field `RX_PULSE_SHAPE_RX_COEF3`"]
pub struct RX_PULSE_SHAPE_RX_COEF3_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PULSE_SHAPE_RX_COEF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_PULSE_SHAPE_RX_COEF3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef3_default(self) -> &'a mut W {
        self.variant(RX_PULSE_SHAPE_RX_COEF3_A::RX_PULSE_SHAPE_RX_COEF3_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_PULSE_SHAPE_RX_COEF2_A {
    #[doc = "0: `0`"]
    RX_PULSE_SHAPE_RX_COEF2_DEFAULT = 0,
}
impl From<RX_PULSE_SHAPE_RX_COEF2_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_PULSE_SHAPE_RX_COEF2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RX_PULSE_SHAPE_RX_COEF2`"]
pub type RX_PULSE_SHAPE_RX_COEF2_R = crate::R<u8, RX_PULSE_SHAPE_RX_COEF2_A>;
impl RX_PULSE_SHAPE_RX_COEF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RX_PULSE_SHAPE_RX_COEF2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RX_PULSE_SHAPE_RX_COEF2_A::RX_PULSE_SHAPE_RX_COEF2_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX_PULSE_SHAPE_RX_COEF2_DEFAULT`"]
    #[inline(always)]
    pub fn is_rx_pulse_shape_rx_coef2_default(&self) -> bool {
        *self == RX_PULSE_SHAPE_RX_COEF2_A::RX_PULSE_SHAPE_RX_COEF2_DEFAULT
    }
}
#[doc = "Write proxy for field `RX_PULSE_SHAPE_RX_COEF2`"]
pub struct RX_PULSE_SHAPE_RX_COEF2_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PULSE_SHAPE_RX_COEF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_PULSE_SHAPE_RX_COEF2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef2_default(self) -> &'a mut W {
        self.variant(RX_PULSE_SHAPE_RX_COEF2_A::RX_PULSE_SHAPE_RX_COEF2_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "These registers specify the Rx pulse shape. The pulse shape is formed by: coef1-coef8-coef8-coef1. Since the oversampling ratio is 8, the pulse shape is 2 symbols long. Coefficients from coef4 to coef8 are unsigned, while coef1 to coef3 are signed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_PULSE_SHAPE_RX_COEF1_A {
    #[doc = "0: `0`"]
    RX_PULSE_SHAPE_RX_COEF1_DEFAULT = 0,
}
impl From<RX_PULSE_SHAPE_RX_COEF1_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_PULSE_SHAPE_RX_COEF1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RX_PULSE_SHAPE_RX_COEF1`"]
pub type RX_PULSE_SHAPE_RX_COEF1_R = crate::R<u8, RX_PULSE_SHAPE_RX_COEF1_A>;
impl RX_PULSE_SHAPE_RX_COEF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RX_PULSE_SHAPE_RX_COEF1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RX_PULSE_SHAPE_RX_COEF1_A::RX_PULSE_SHAPE_RX_COEF1_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX_PULSE_SHAPE_RX_COEF1_DEFAULT`"]
    #[inline(always)]
    pub fn is_rx_pulse_shape_rx_coef1_default(&self) -> bool {
        *self == RX_PULSE_SHAPE_RX_COEF1_A::RX_PULSE_SHAPE_RX_COEF1_DEFAULT
    }
}
#[doc = "Write proxy for field `RX_PULSE_SHAPE_RX_COEF1`"]
pub struct RX_PULSE_SHAPE_RX_COEF1_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PULSE_SHAPE_RX_COEF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_PULSE_SHAPE_RX_COEF1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef1_default(self) -> &'a mut W {
        self.variant(RX_PULSE_SHAPE_RX_COEF1_A::RX_PULSE_SHAPE_RX_COEF1_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef8(&self) -> RX_PULSE_SHAPE_RX_COEF8_R {
        RX_PULSE_SHAPE_RX_COEF8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef7(&self) -> RX_PULSE_SHAPE_RX_COEF7_R {
        RX_PULSE_SHAPE_RX_COEF7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef6(&self) -> RX_PULSE_SHAPE_RX_COEF6_R {
        RX_PULSE_SHAPE_RX_COEF6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef5(&self) -> RX_PULSE_SHAPE_RX_COEF5_R {
        RX_PULSE_SHAPE_RX_COEF5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef4(&self) -> RX_PULSE_SHAPE_RX_COEF4_R {
        RX_PULSE_SHAPE_RX_COEF4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef3(&self) -> RX_PULSE_SHAPE_RX_COEF3_R {
        RX_PULSE_SHAPE_RX_COEF3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef2(&self) -> RX_PULSE_SHAPE_RX_COEF2_R {
        RX_PULSE_SHAPE_RX_COEF2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - These registers specify the Rx pulse shape. The pulse shape is formed by: coef1-coef8-coef8-coef1. Since the oversampling ratio is 8, the pulse shape is 2 symbols long. Coefficients from coef4 to coef8 are unsigned, while coef1 to coef3 are signed."]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef1(&self) -> RX_PULSE_SHAPE_RX_COEF1_R {
        RX_PULSE_SHAPE_RX_COEF1_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef8(&mut self) -> RX_PULSE_SHAPE_RX_COEF8_W {
        RX_PULSE_SHAPE_RX_COEF8_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef7(&mut self) -> RX_PULSE_SHAPE_RX_COEF7_W {
        RX_PULSE_SHAPE_RX_COEF7_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef6(&mut self) -> RX_PULSE_SHAPE_RX_COEF6_W {
        RX_PULSE_SHAPE_RX_COEF6_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef5(&mut self) -> RX_PULSE_SHAPE_RX_COEF5_W {
        RX_PULSE_SHAPE_RX_COEF5_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef4(&mut self) -> RX_PULSE_SHAPE_RX_COEF4_W {
        RX_PULSE_SHAPE_RX_COEF4_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef3(&mut self) -> RX_PULSE_SHAPE_RX_COEF3_W {
        RX_PULSE_SHAPE_RX_COEF3_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef2(&mut self) -> RX_PULSE_SHAPE_RX_COEF2_W {
        RX_PULSE_SHAPE_RX_COEF2_W { w: self }
    }
    #[doc = "Bits 0:3 - These registers specify the Rx pulse shape. The pulse shape is formed by: coef1-coef8-coef8-coef1. Since the oversampling ratio is 8, the pulse shape is 2 symbols long. Coefficients from coef4 to coef8 are unsigned, while coef1 to coef3 are signed."]
    #[inline(always)]
    pub fn rx_pulse_shape_rx_coef1(&mut self) -> RX_PULSE_SHAPE_RX_COEF1_W {
        RX_PULSE_SHAPE_RX_COEF1_W { w: self }
    }
}
