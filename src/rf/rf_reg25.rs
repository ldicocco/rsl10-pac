#[doc = "Reader of register RF_REG25"]
pub type R = crate::R<u32, super::RF_REG25>;
#[doc = "Writer for register RF_REG25"]
pub type W = crate::W<u32, super::RF_REG25>;
#[doc = "Register RF_REG25 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG25 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Peak detector threshold bias 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_9_IQ_BB_6_A {
    #[doc = "0: `0`"]
    BIAS_9_IQ_BB_6_DEFAULT = 0,
}
impl From<BIAS_9_IQ_BB_6_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_9_IQ_BB_6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_9_IQ_BB_6`"]
pub type BIAS_9_IQ_BB_6_R = crate::R<u8, BIAS_9_IQ_BB_6_A>;
impl BIAS_9_IQ_BB_6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_9_IQ_BB_6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_9_IQ_BB_6_A::BIAS_9_IQ_BB_6_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_9_IQ_BB_6_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_9_iq_bb_6_default(&self) -> bool {
        *self == BIAS_9_IQ_BB_6_A::BIAS_9_IQ_BB_6_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_9_IQ_BB_6`"]
pub struct BIAS_9_IQ_BB_6_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_9_IQ_BB_6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_9_IQ_BB_6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_9_iq_bb_6_default(self) -> &'a mut W {
        self.variant(BIAS_9_IQ_BB_6_A::BIAS_9_IQ_BB_6_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Peak detector bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_9_IQ_BB_5_A {
    #[doc = "0: `0`"]
    BIAS_9_IQ_BB_5_DEFAULT = 0,
}
impl From<BIAS_9_IQ_BB_5_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_9_IQ_BB_5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_9_IQ_BB_5`"]
pub type BIAS_9_IQ_BB_5_R = crate::R<u8, BIAS_9_IQ_BB_5_A>;
impl BIAS_9_IQ_BB_5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_9_IQ_BB_5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_9_IQ_BB_5_A::BIAS_9_IQ_BB_5_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_9_IQ_BB_5_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_9_iq_bb_5_default(&self) -> bool {
        *self == BIAS_9_IQ_BB_5_A::BIAS_9_IQ_BB_5_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_9_IQ_BB_5`"]
pub struct BIAS_9_IQ_BB_5_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_9_IQ_BB_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_9_IQ_BB_5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_9_iq_bb_5_default(self) -> &'a mut W {
        self.variant(BIAS_9_IQ_BB_5_A::BIAS_9_IQ_BB_5_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "RSSI_D bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_8_IQ_BB_4_A {
    #[doc = "0: `0`"]
    BIAS_8_IQ_BB_4_DEFAULT = 0,
}
impl From<BIAS_8_IQ_BB_4_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_8_IQ_BB_4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_8_IQ_BB_4`"]
pub type BIAS_8_IQ_BB_4_R = crate::R<u8, BIAS_8_IQ_BB_4_A>;
impl BIAS_8_IQ_BB_4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_8_IQ_BB_4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_8_IQ_BB_4_A::BIAS_8_IQ_BB_4_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_8_IQ_BB_4_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_8_iq_bb_4_default(&self) -> bool {
        *self == BIAS_8_IQ_BB_4_A::BIAS_8_IQ_BB_4_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_8_IQ_BB_4`"]
pub struct BIAS_8_IQ_BB_4_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_8_IQ_BB_4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_8_IQ_BB_4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_8_iq_bb_4_default(self) -> &'a mut W {
        self.variant(BIAS_8_IQ_BB_4_A::BIAS_8_IQ_BB_4_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "RSSI_G bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_8_IQ_BB_3_A {
    #[doc = "0: `0`"]
    BIAS_8_IQ_BB_3_DEFAULT = 0,
}
impl From<BIAS_8_IQ_BB_3_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_8_IQ_BB_3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_8_IQ_BB_3`"]
pub type BIAS_8_IQ_BB_3_R = crate::R<u8, BIAS_8_IQ_BB_3_A>;
impl BIAS_8_IQ_BB_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_8_IQ_BB_3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_8_IQ_BB_3_A::BIAS_8_IQ_BB_3_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_8_IQ_BB_3_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_8_iq_bb_3_default(&self) -> bool {
        *self == BIAS_8_IQ_BB_3_A::BIAS_8_IQ_BB_3_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_8_IQ_BB_3`"]
pub struct BIAS_8_IQ_BB_3_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_8_IQ_BB_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_8_IQ_BB_3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_8_iq_bb_3_default(self) -> &'a mut W {
        self.variant(BIAS_8_IQ_BB_3_A::BIAS_8_IQ_BB_3_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "ACD_L bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_7_IQ_BB_2_A {
    #[doc = "0: `0`"]
    BIAS_7_IQ_BB_2_DEFAULT = 0,
}
impl From<BIAS_7_IQ_BB_2_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_7_IQ_BB_2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_7_IQ_BB_2`"]
pub type BIAS_7_IQ_BB_2_R = crate::R<u8, BIAS_7_IQ_BB_2_A>;
impl BIAS_7_IQ_BB_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_7_IQ_BB_2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_7_IQ_BB_2_A::BIAS_7_IQ_BB_2_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_7_IQ_BB_2_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_7_iq_bb_2_default(&self) -> bool {
        *self == BIAS_7_IQ_BB_2_A::BIAS_7_IQ_BB_2_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_7_IQ_BB_2`"]
pub struct BIAS_7_IQ_BB_2_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_7_IQ_BB_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_7_IQ_BB_2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_7_iq_bb_2_default(self) -> &'a mut W {
        self.variant(BIAS_7_IQ_BB_2_A::BIAS_7_IQ_BB_2_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "ACD_C bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_7_IQ_BB_1_A {
    #[doc = "0: `0`"]
    BIAS_7_IQ_BB_1_DEFAULT = 0,
}
impl From<BIAS_7_IQ_BB_1_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_7_IQ_BB_1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_7_IQ_BB_1`"]
pub type BIAS_7_IQ_BB_1_R = crate::R<u8, BIAS_7_IQ_BB_1_A>;
impl BIAS_7_IQ_BB_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_7_IQ_BB_1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_7_IQ_BB_1_A::BIAS_7_IQ_BB_1_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_7_IQ_BB_1_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_7_iq_bb_1_default(&self) -> bool {
        *self == BIAS_7_IQ_BB_1_A::BIAS_7_IQ_BB_1_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_7_IQ_BB_1`"]
pub struct BIAS_7_IQ_BB_1_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_7_IQ_BB_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_7_IQ_BB_1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_7_iq_bb_1_default(self) -> &'a mut W {
        self.variant(BIAS_7_IQ_BB_1_A::BIAS_7_IQ_BB_1_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "ACD_O bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_6_IQ_BB_0_A {
    #[doc = "0: `0`"]
    BIAS_6_IQ_BB_0_DEFAULT = 0,
}
impl From<BIAS_6_IQ_BB_0_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_6_IQ_BB_0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_6_IQ_BB_0`"]
pub type BIAS_6_IQ_BB_0_R = crate::R<u8, BIAS_6_IQ_BB_0_A>;
impl BIAS_6_IQ_BB_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_6_IQ_BB_0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_6_IQ_BB_0_A::BIAS_6_IQ_BB_0_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_6_IQ_BB_0_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_6_iq_bb_0_default(&self) -> bool {
        *self == BIAS_6_IQ_BB_0_A::BIAS_6_IQ_BB_0_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_6_IQ_BB_0`"]
pub struct BIAS_6_IQ_BB_0_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_6_IQ_BB_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_6_IQ_BB_0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_6_iq_bb_0_default(self) -> &'a mut W {
        self.variant(BIAS_6_IQ_BB_0_A::BIAS_6_IQ_BB_0_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "DLL bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_6_IQ_PLL_3_A {
    #[doc = "0: `0`"]
    BIAS_6_IQ_PLL_3_DEFAULT = 0,
}
impl From<BIAS_6_IQ_PLL_3_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_6_IQ_PLL_3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_6_IQ_PLL_3`"]
pub type BIAS_6_IQ_PLL_3_R = crate::R<u8, BIAS_6_IQ_PLL_3_A>;
impl BIAS_6_IQ_PLL_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_6_IQ_PLL_3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_6_IQ_PLL_3_A::BIAS_6_IQ_PLL_3_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_6_IQ_PLL_3_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_6_iq_pll_3_default(&self) -> bool {
        *self == BIAS_6_IQ_PLL_3_A::BIAS_6_IQ_PLL_3_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_6_IQ_PLL_3`"]
pub struct BIAS_6_IQ_PLL_3_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_6_IQ_PLL_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_6_IQ_PLL_3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_6_iq_pll_3_default(self) -> &'a mut W {
        self.variant(BIAS_6_IQ_PLL_3_A::BIAS_6_IQ_PLL_3_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - Peak detector threshold bias 0"]
    #[inline(always)]
    pub fn bias_9_iq_bb_6(&self) -> BIAS_9_IQ_BB_6_R {
        BIAS_9_IQ_BB_6_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Peak detector bias"]
    #[inline(always)]
    pub fn bias_9_iq_bb_5(&self) -> BIAS_9_IQ_BB_5_R {
        BIAS_9_IQ_BB_5_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - RSSI_D bias"]
    #[inline(always)]
    pub fn bias_8_iq_bb_4(&self) -> BIAS_8_IQ_BB_4_R {
        BIAS_8_IQ_BB_4_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RSSI_G bias"]
    #[inline(always)]
    pub fn bias_8_iq_bb_3(&self) -> BIAS_8_IQ_BB_3_R {
        BIAS_8_IQ_BB_3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - ACD_L bias"]
    #[inline(always)]
    pub fn bias_7_iq_bb_2(&self) -> BIAS_7_IQ_BB_2_R {
        BIAS_7_IQ_BB_2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ACD_C bias"]
    #[inline(always)]
    pub fn bias_7_iq_bb_1(&self) -> BIAS_7_IQ_BB_1_R {
        BIAS_7_IQ_BB_1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ACD_O bias"]
    #[inline(always)]
    pub fn bias_6_iq_bb_0(&self) -> BIAS_6_IQ_BB_0_R {
        BIAS_6_IQ_BB_0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - DLL bias"]
    #[inline(always)]
    pub fn bias_6_iq_pll_3(&self) -> BIAS_6_IQ_PLL_3_R {
        BIAS_6_IQ_PLL_3_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Peak detector threshold bias 0"]
    #[inline(always)]
    pub fn bias_9_iq_bb_6(&mut self) -> BIAS_9_IQ_BB_6_W {
        BIAS_9_IQ_BB_6_W { w: self }
    }
    #[doc = "Bits 24:27 - Peak detector bias"]
    #[inline(always)]
    pub fn bias_9_iq_bb_5(&mut self) -> BIAS_9_IQ_BB_5_W {
        BIAS_9_IQ_BB_5_W { w: self }
    }
    #[doc = "Bits 20:23 - RSSI_D bias"]
    #[inline(always)]
    pub fn bias_8_iq_bb_4(&mut self) -> BIAS_8_IQ_BB_4_W {
        BIAS_8_IQ_BB_4_W { w: self }
    }
    #[doc = "Bits 16:19 - RSSI_G bias"]
    #[inline(always)]
    pub fn bias_8_iq_bb_3(&mut self) -> BIAS_8_IQ_BB_3_W {
        BIAS_8_IQ_BB_3_W { w: self }
    }
    #[doc = "Bits 12:15 - ACD_L bias"]
    #[inline(always)]
    pub fn bias_7_iq_bb_2(&mut self) -> BIAS_7_IQ_BB_2_W {
        BIAS_7_IQ_BB_2_W { w: self }
    }
    #[doc = "Bits 8:11 - ACD_C bias"]
    #[inline(always)]
    pub fn bias_7_iq_bb_1(&mut self) -> BIAS_7_IQ_BB_1_W {
        BIAS_7_IQ_BB_1_W { w: self }
    }
    #[doc = "Bits 4:7 - ACD_O bias"]
    #[inline(always)]
    pub fn bias_6_iq_bb_0(&mut self) -> BIAS_6_IQ_BB_0_W {
        BIAS_6_IQ_BB_0_W { w: self }
    }
    #[doc = "Bits 0:3 - DLL bias"]
    #[inline(always)]
    pub fn bias_6_iq_pll_3(&mut self) -> BIAS_6_IQ_PLL_3_W {
        BIAS_6_IQ_PLL_3_W { w: self }
    }
}
