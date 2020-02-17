#[doc = "Reader of register BB_BBMPRIO1"]
pub type R = crate::R<u32, super::BB_BBMPRIO1>;
#[doc = "Writer for register BB_BBMPRIO1"]
pub type W = crate::W<u32, super::BB_BBMPRIO1>;
#[doc = "Register BB_BBMPRIO1 `reset()`'s with value 0x3000_00dc"]
impl crate::ResetValue for super::BB_BBMPRIO1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3000_00dc
    }
}
#[doc = "Set default priority value for other BLE message than those defined above\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLEMDEFAULT_A {
    #[doc = "3: `11`"]
    BLEMDEFAULT_3 = 3,
}
impl From<BLEMDEFAULT_A> for u8 {
    #[inline(always)]
    fn from(variant: BLEMDEFAULT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLEMDEFAULT`"]
pub type BLEMDEFAULT_R = crate::R<u8, BLEMDEFAULT_A>;
impl BLEMDEFAULT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BLEMDEFAULT_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(BLEMDEFAULT_A::BLEMDEFAULT_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLEMDEFAULT_3`"]
    #[inline(always)]
    pub fn is_blemdefault_3(&self) -> bool {
        *self == BLEMDEFAULT_A::BLEMDEFAULT_3
    }
}
#[doc = "Write proxy for field `BLEMDEFAULT`"]
pub struct BLEMDEFAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEMDEFAULT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEMDEFAULT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn blemdefault_3(self) -> &'a mut W {
        self.variant(BLEMDEFAULT_A::BLEMDEFAULT_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Set default priority value for ISO Channel first Tx/Rx attempt\n\nValue on reset: 13"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLEM9_A {
    #[doc = "13: `1101`"]
    BLEM7_13 = 13,
}
impl From<BLEM9_A> for u8 {
    #[inline(always)]
    fn from(variant: BLEM9_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLEM9`"]
pub type BLEM9_R = crate::R<u8, BLEM9_A>;
impl BLEM9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BLEM9_A> {
        use crate::Variant::*;
        match self.bits {
            13 => Val(BLEM9_A::BLEM7_13),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLEM7_13`"]
    #[inline(always)]
    pub fn is_blem7_13(&self) -> bool {
        *self == BLEM9_A::BLEM7_13
    }
}
#[doc = "Write proxy for field `BLEM9`"]
pub struct BLEM9_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEM9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEM9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn blem7_13(self) -> &'a mut W {
        self.variant(BLEM9_A::BLEM7_13)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Set default priority value for ISO Channel subsequent Tx/Rx attempt\n\nValue on reset: 12"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLEM8_A {
    #[doc = "12: `1100`"]
    BLEM6_12 = 12,
}
impl From<BLEM8_A> for u8 {
    #[inline(always)]
    fn from(variant: BLEM8_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLEM8`"]
pub type BLEM8_R = crate::R<u8, BLEM8_A>;
impl BLEM8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BLEM8_A> {
        use crate::Variant::*;
        match self.bits {
            12 => Val(BLEM8_A::BLEM6_12),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLEM6_12`"]
    #[inline(always)]
    pub fn is_blem6_12(&self) -> bool {
        *self == BLEM8_A::BLEM6_12
    }
}
#[doc = "Write proxy for field `BLEM8`"]
pub struct BLEM8_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEM8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLEM8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn blem6_12(self) -> &'a mut W {
        self.variant(BLEM8_A::BLEM6_12)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - Set default priority value for other BLE message than those defined above"]
    #[inline(always)]
    pub fn blemdefault(&self) -> BLEMDEFAULT_R {
        BLEMDEFAULT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Set default priority value for ISO Channel first Tx/Rx attempt"]
    #[inline(always)]
    pub fn blem9(&self) -> BLEM9_R {
        BLEM9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Set default priority value for ISO Channel subsequent Tx/Rx attempt"]
    #[inline(always)]
    pub fn blem8(&self) -> BLEM8_R {
        BLEM8_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Set default priority value for other BLE message than those defined above"]
    #[inline(always)]
    pub fn blemdefault(&mut self) -> BLEMDEFAULT_W {
        BLEMDEFAULT_W { w: self }
    }
    #[doc = "Bits 4:7 - Set default priority value for ISO Channel first Tx/Rx attempt"]
    #[inline(always)]
    pub fn blem9(&mut self) -> BLEM9_W {
        BLEM9_W { w: self }
    }
    #[doc = "Bits 0:3 - Set default priority value for ISO Channel subsequent Tx/Rx attempt"]
    #[inline(always)]
    pub fn blem8(&mut self) -> BLEM8_W {
        BLEM8_W { w: self }
    }
}
