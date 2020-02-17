#[doc = "Reader of register BB_RADIOCNTL0"]
pub type R = crate::R<u32, super::BB_RADIOCNTL0>;
#[doc = "Writer for register BB_RADIOCNTL0"]
pub type W = crate::W<u32, super::BB_RADIOCNTL0>;
#[doc = "Register BB_RADIOCNTL0 `reset()`'s with value 0x02"]
impl crate::ResetValue for super::BB_RADIOCNTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Pointer to the buffer containing data to be transferred to or received from the SPI port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SPIPTR_A {
    #[doc = "0: SPI pointer"]
    SPIPTR_0 = 0,
}
impl From<SPIPTR_A> for u16 {
    #[inline(always)]
    fn from(variant: SPIPTR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPIPTR`"]
pub type SPIPTR_R = crate::R<u16, SPIPTR_A>;
impl SPIPTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SPIPTR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SPIPTR_A::SPIPTR_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SPIPTR_0`"]
    #[inline(always)]
    pub fn is_spiptr_0(&self) -> bool {
        *self == SPIPTR_A::SPIPTR_0
    }
}
#[doc = "Write proxy for field `SPIPTR`"]
pub struct SPIPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIPTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIPTR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SPI pointer"]
    #[inline(always)]
    pub fn spiptr_0(self) -> &'a mut W {
        self.variant(SPIPTR_A::SPIPTR_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "SPI clock frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPIFREQ_A {
    #[doc = "0: SPI clock is master1_gclk / 3"]
    SPIFREQ_0 = 0,
    #[doc = "1: NA"]
    SPIFREQ_1 = 1,
    #[doc = "2: NA"]
    SPIFREQ_2 = 2,
    #[doc = "3: NA"]
    SPIFREQ_3 = 3,
}
impl From<SPIFREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: SPIFREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPIFREQ`"]
pub type SPIFREQ_R = crate::R<u8, SPIFREQ_A>;
impl SPIFREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIFREQ_A {
        match self.bits {
            0 => SPIFREQ_A::SPIFREQ_0,
            1 => SPIFREQ_A::SPIFREQ_1,
            2 => SPIFREQ_A::SPIFREQ_2,
            3 => SPIFREQ_A::SPIFREQ_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SPIFREQ_0`"]
    #[inline(always)]
    pub fn is_spifreq_0(&self) -> bool {
        *self == SPIFREQ_A::SPIFREQ_0
    }
    #[doc = "Checks if the value of the field is `SPIFREQ_1`"]
    #[inline(always)]
    pub fn is_spifreq_1(&self) -> bool {
        *self == SPIFREQ_A::SPIFREQ_1
    }
    #[doc = "Checks if the value of the field is `SPIFREQ_2`"]
    #[inline(always)]
    pub fn is_spifreq_2(&self) -> bool {
        *self == SPIFREQ_A::SPIFREQ_2
    }
    #[doc = "Checks if the value of the field is `SPIFREQ_3`"]
    #[inline(always)]
    pub fn is_spifreq_3(&self) -> bool {
        *self == SPIFREQ_A::SPIFREQ_3
    }
}
#[doc = "Write proxy for field `SPIFREQ`"]
pub struct SPIFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIFREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIFREQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SPI clock is master1_gclk / 3"]
    #[inline(always)]
    pub fn spifreq_0(self) -> &'a mut W {
        self.variant(SPIFREQ_A::SPIFREQ_0)
    }
    #[doc = "NA"]
    #[inline(always)]
    pub fn spifreq_1(self) -> &'a mut W {
        self.variant(SPIFREQ_A::SPIFREQ_1)
    }
    #[doc = "NA"]
    #[inline(always)]
    pub fn spifreq_2(self) -> &'a mut W {
        self.variant(SPIFREQ_A::SPIFREQ_2)
    }
    #[doc = "NA"]
    #[inline(always)]
    pub fn spifreq_3(self) -> &'a mut W {
        self.variant(SPIFREQ_A::SPIFREQ_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "SPI transfer status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPICOMP_A {
    #[doc = "0: Indicates SPI transfer in progress"]
    SPICOMP_0 = 0,
    #[doc = "1: Indicates SPI transfer is completed. RW-BLE core is ready to start a new transfer"]
    SPICOMP_1 = 1,
}
impl From<SPICOMP_A> for bool {
    #[inline(always)]
    fn from(variant: SPICOMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPICOMP`"]
pub type SPICOMP_R = crate::R<bool, SPICOMP_A>;
impl SPICOMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPICOMP_A {
        match self.bits {
            false => SPICOMP_A::SPICOMP_0,
            true => SPICOMP_A::SPICOMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `SPICOMP_0`"]
    #[inline(always)]
    pub fn is_spicomp_0(&self) -> bool {
        *self == SPICOMP_A::SPICOMP_0
    }
    #[doc = "Checks if the value of the field is `SPICOMP_1`"]
    #[inline(always)]
    pub fn is_spicomp_1(&self) -> bool {
        *self == SPICOMP_A::SPICOMP_1
    }
}
#[doc = "Start SPI transfer when writing a 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIGO_A {
    #[doc = "0: `0`"]
    SPIGO_0 = 0,
    #[doc = "1: Triggers the SPI transfer"]
    SPIGO_1 = 1,
}
impl From<SPIGO_A> for bool {
    #[inline(always)]
    fn from(variant: SPIGO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPIGO`"]
pub type SPIGO_R = crate::R<bool, SPIGO_A>;
impl SPIGO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIGO_A {
        match self.bits {
            false => SPIGO_A::SPIGO_0,
            true => SPIGO_A::SPIGO_1,
        }
    }
    #[doc = "Checks if the value of the field is `SPIGO_0`"]
    #[inline(always)]
    pub fn is_spigo_0(&self) -> bool {
        *self == SPIGO_A::SPIGO_0
    }
    #[doc = "Checks if the value of the field is `SPIGO_1`"]
    #[inline(always)]
    pub fn is_spigo_1(&self) -> bool {
        *self == SPIGO_A::SPIGO_1
    }
}
#[doc = "Write proxy for field `SPIGO`"]
pub struct SPIGO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIGO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIGO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn spigo_0(self) -> &'a mut W {
        self.variant(SPIGO_A::SPIGO_0)
    }
    #[doc = "Triggers the SPI transfer"]
    #[inline(always)]
    pub fn spigo_1(self) -> &'a mut W {
        self.variant(SPIGO_A::SPIGO_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Pointer to the buffer containing data to be transferred to or received from the SPI port"]
    #[inline(always)]
    pub fn spiptr(&self) -> SPIPTR_R {
        SPIPTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 4:5 - SPI clock frequency"]
    #[inline(always)]
    pub fn spifreq(&self) -> SPIFREQ_R {
        SPIFREQ_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 1 - SPI transfer status"]
    #[inline(always)]
    pub fn spicomp(&self) -> SPICOMP_R {
        SPICOMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Start SPI transfer when writing a 1"]
    #[inline(always)]
    pub fn spigo(&self) -> SPIGO_R {
        SPIGO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - Pointer to the buffer containing data to be transferred to or received from the SPI port"]
    #[inline(always)]
    pub fn spiptr(&mut self) -> SPIPTR_W {
        SPIPTR_W { w: self }
    }
    #[doc = "Bits 4:5 - SPI clock frequency"]
    #[inline(always)]
    pub fn spifreq(&mut self) -> SPIFREQ_W {
        SPIFREQ_W { w: self }
    }
    #[doc = "Bit 0 - Start SPI transfer when writing a 1"]
    #[inline(always)]
    pub fn spigo(&mut self) -> SPIGO_W {
        SPIGO_W { w: self }
    }
}
