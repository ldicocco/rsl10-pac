#[doc = "Reader of register BB_BBMPRIO0"]
pub type R = crate::R<u32, super::BB_BBMPRIO0>;
#[doc = "Writer for register BB_BBMPRIO0"]
pub type W = crate::W<u32, super::BB_BBMPRIO0>;
#[doc = "Register BB_BBMPRIO0 `reset()`'s with value 0x3489_adef"]
impl crate::ResetValue for super::BB_BBMPRIO0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3489_adef
    }
}
#[doc = "Set priority value for passive scanning\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLEM7_A {
    #[doc = "3: `11`"]
    BLEM7_3 = 3,
}
impl From<BLEM7_A> for u8 {
    #[inline(always)]
    fn from(variant: BLEM7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLEM7`"]
pub type BLEM7_R = crate::R<u8, BLEM7_A>;
impl BLEM7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BLEM7_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(BLEM7_A::BLEM7_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLEM7_3`"]
    #[inline(always)]
    pub fn is_blem7_3(&self) -> bool {
        *self == BLEM7_A::BLEM7_3
    }
}
#[doc = "Write proxy for field `BLEM7`"]
pub struct BLEM7_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEM7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEM7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn blem7_3(self) -> &'a mut W {
        self.variant(BLEM7_A::BLEM7_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Set priority value for non-connectable advertising\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLEM6_A {
    #[doc = "4: `100`"]
    BLEM6_4 = 4,
}
impl From<BLEM6_A> for u8 {
    #[inline(always)]
    fn from(variant: BLEM6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLEM6`"]
pub type BLEM6_R = crate::R<u8, BLEM6_A>;
impl BLEM6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BLEM6_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(BLEM6_A::BLEM6_4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLEM6_4`"]
    #[inline(always)]
    pub fn is_blem6_4(&self) -> bool {
        *self == BLEM6_A::BLEM6_4
    }
}
#[doc = "Write proxy for field `BLEM6`"]
pub struct BLEM6_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEM6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEM6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn blem6_4(self) -> &'a mut W {
        self.variant(BLEM6_A::BLEM6_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Set priority value for connectable advertising BLE message\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLEM5_A {
    #[doc = "8: `1000`"]
    BLEM5_8 = 8,
}
impl From<BLEM5_A> for u8 {
    #[inline(always)]
    fn from(variant: BLEM5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLEM5`"]
pub type BLEM5_R = crate::R<u8, BLEM5_A>;
impl BLEM5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BLEM5_A> {
        use crate::Variant::*;
        match self.bits {
            8 => Val(BLEM5_A::BLEM5_8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLEM5_8`"]
    #[inline(always)]
    pub fn is_blem5_8(&self) -> bool {
        *self == BLEM5_A::BLEM5_8
    }
}
#[doc = "Write proxy for field `BLEM5`"]
pub struct BLEM5_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEM5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEM5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn blem5_8(self) -> &'a mut W {
        self.variant(BLEM5_A::BLEM5_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Set priority value for active scanning BLE message\n\nValue on reset: 9"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLEM4_A {
    #[doc = "9: `1001`"]
    BLEM4_9 = 9,
}
impl From<BLEM4_A> for u8 {
    #[inline(always)]
    fn from(variant: BLEM4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLEM4`"]
pub type BLEM4_R = crate::R<u8, BLEM4_A>;
impl BLEM4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BLEM4_A> {
        use crate::Variant::*;
        match self.bits {
            9 => Val(BLEM4_A::BLEM4_9),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLEM4_9`"]
    #[inline(always)]
    pub fn is_blem4_9(&self) -> bool {
        *self == BLEM4_A::BLEM4_9
    }
}
#[doc = "Write proxy for field `BLEM4`"]
pub struct BLEM4_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEM4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEM4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn blem4_9(self) -> &'a mut W {
        self.variant(BLEM4_A::BLEM4_9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Set priority value for initiating (scanning) BLE message\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLEM3_A {
    #[doc = "10: `1010`"]
    BLEM3_10 = 10,
}
impl From<BLEM3_A> for u8 {
    #[inline(always)]
    fn from(variant: BLEM3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLEM3`"]
pub type BLEM3_R = crate::R<u8, BLEM3_A>;
impl BLEM3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BLEM3_A> {
        use crate::Variant::*;
        match self.bits {
            10 => Val(BLEM3_A::BLEM3_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLEM3_10`"]
    #[inline(always)]
    pub fn is_blem3_10(&self) -> bool {
        *self == BLEM3_A::BLEM3_10
    }
}
#[doc = "Write proxy for field `BLEM3`"]
pub struct BLEM3_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEM3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn blem3_10(self) -> &'a mut W {
        self.variant(BLEM3_A::BLEM3_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Set priority value for data channel transmission BLE message\n\nValue on reset: 13"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLEM2_A {
    #[doc = "13: `1101`"]
    BLEM2_13 = 13,
}
impl From<BLEM2_A> for u8 {
    #[inline(always)]
    fn from(variant: BLEM2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLEM2`"]
pub type BLEM2_R = crate::R<u8, BLEM2_A>;
impl BLEM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BLEM2_A> {
        use crate::Variant::*;
        match self.bits {
            13 => Val(BLEM2_A::BLEM2_13),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLEM2_13`"]
    #[inline(always)]
    pub fn is_blem2_13(&self) -> bool {
        *self == BLEM2_A::BLEM2_13
    }
}
#[doc = "Write proxy for field `BLEM2`"]
pub struct BLEM2_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEM2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn blem2_13(self) -> &'a mut W {
        self.variant(BLEM2_A::BLEM2_13)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Set priority value for LLCP BLE message\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLEM1_A {
    #[doc = "14: `1110`"]
    BLEM1_14 = 14,
}
impl From<BLEM1_A> for u8 {
    #[inline(always)]
    fn from(variant: BLEM1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLEM1`"]
pub type BLEM1_R = crate::R<u8, BLEM1_A>;
impl BLEM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BLEM1_A> {
        use crate::Variant::*;
        match self.bits {
            14 => Val(BLEM1_A::BLEM1_14),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLEM1_14`"]
    #[inline(always)]
    pub fn is_blem1_14(&self) -> bool {
        *self == BLEM1_A::BLEM1_14
    }
}
#[doc = "Write proxy for field `BLEM1`"]
pub struct BLEM1_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEM1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn blem1_14(self) -> &'a mut W {
        self.variant(BLEM1_A::BLEM1_14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Set priority value for initiating (connection request response) BLE message\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLEM0_A {
    #[doc = "15: `1111`"]
    BLEM0_15 = 15,
}
impl From<BLEM0_A> for u8 {
    #[inline(always)]
    fn from(variant: BLEM0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLEM0`"]
pub type BLEM0_R = crate::R<u8, BLEM0_A>;
impl BLEM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BLEM0_A> {
        use crate::Variant::*;
        match self.bits {
            15 => Val(BLEM0_A::BLEM0_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLEM0_15`"]
    #[inline(always)]
    pub fn is_blem0_15(&self) -> bool {
        *self == BLEM0_A::BLEM0_15
    }
}
#[doc = "Write proxy for field `BLEM0`"]
pub struct BLEM0_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEM0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn blem0_15(self) -> &'a mut W {
        self.variant(BLEM0_A::BLEM0_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - Set priority value for passive scanning"]
    #[inline(always)]
    pub fn blem7(&self) -> BLEM7_R {
        BLEM7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Set priority value for non-connectable advertising"]
    #[inline(always)]
    pub fn blem6(&self) -> BLEM6_R {
        BLEM6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Set priority value for connectable advertising BLE message"]
    #[inline(always)]
    pub fn blem5(&self) -> BLEM5_R {
        BLEM5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Set priority value for active scanning BLE message"]
    #[inline(always)]
    pub fn blem4(&self) -> BLEM4_R {
        BLEM4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Set priority value for initiating (scanning) BLE message"]
    #[inline(always)]
    pub fn blem3(&self) -> BLEM3_R {
        BLEM3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Set priority value for data channel transmission BLE message"]
    #[inline(always)]
    pub fn blem2(&self) -> BLEM2_R {
        BLEM2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Set priority value for LLCP BLE message"]
    #[inline(always)]
    pub fn blem1(&self) -> BLEM1_R {
        BLEM1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Set priority value for initiating (connection request response) BLE message"]
    #[inline(always)]
    pub fn blem0(&self) -> BLEM0_R {
        BLEM0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Set priority value for passive scanning"]
    #[inline(always)]
    pub fn blem7(&mut self) -> BLEM7_W {
        BLEM7_W { w: self }
    }
    #[doc = "Bits 24:27 - Set priority value for non-connectable advertising"]
    #[inline(always)]
    pub fn blem6(&mut self) -> BLEM6_W {
        BLEM6_W { w: self }
    }
    #[doc = "Bits 20:23 - Set priority value for connectable advertising BLE message"]
    #[inline(always)]
    pub fn blem5(&mut self) -> BLEM5_W {
        BLEM5_W { w: self }
    }
    #[doc = "Bits 16:19 - Set priority value for active scanning BLE message"]
    #[inline(always)]
    pub fn blem4(&mut self) -> BLEM4_W {
        BLEM4_W { w: self }
    }
    #[doc = "Bits 12:15 - Set priority value for initiating (scanning) BLE message"]
    #[inline(always)]
    pub fn blem3(&mut self) -> BLEM3_W {
        BLEM3_W { w: self }
    }
    #[doc = "Bits 8:11 - Set priority value for data channel transmission BLE message"]
    #[inline(always)]
    pub fn blem2(&mut self) -> BLEM2_W {
        BLEM2_W { w: self }
    }
    #[doc = "Bits 4:7 - Set priority value for LLCP BLE message"]
    #[inline(always)]
    pub fn blem1(&mut self) -> BLEM1_W {
        BLEM1_W { w: self }
    }
    #[doc = "Bits 0:3 - Set priority value for initiating (connection request response) BLE message"]
    #[inline(always)]
    pub fn blem0(&mut self) -> BLEM0_W {
        BLEM0_W { w: self }
    }
}
