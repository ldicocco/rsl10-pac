#[doc = "Reader of register SYSCTRL_MEM_ARBITER_CFG"]
pub type R = crate::R<u32, super::SYSCTRL_MEM_ARBITER_CFG>;
#[doc = "Writer for register SYSCTRL_MEM_ARBITER_CFG"]
pub type W = crate::W<u32, super::SYSCTRL_MEM_ARBITER_CFG>;
#[doc = "Register SYSCTRL_MEM_ARBITER_CFG `reset()`'s with value 0x0f00"]
impl crate::ResetValue for super::SYSCTRL_MEM_ARBITER_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f00
    }
}
#[doc = "DSP DRAM4 and DRAM5 arbiter configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSP_DRAM45_ARBITER_A {
    #[doc = "0: DSP has priority access to the DSP DRAM4 and DRAM5 (above CM3 and DMA)"]
    DSP_DRAM45_DSP_PRIORITY = 0,
    #[doc = "1: CM3 has priority access to the DSP DRAM4 and DRAM5 (above DSP and DMA)"]
    DSP_DRAM45_CM3_PRIORITY = 1,
    #[doc = "2: Round robin priority access to the DSP DRAM4 and DRAM5"]
    DSP_DRAM45_ROUND_ROBIN_PRIORITY = 2,
}
impl From<DSP_DRAM45_ARBITER_A> for u8 {
    #[inline(always)]
    fn from(variant: DSP_DRAM45_ARBITER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DSP_DRAM45_ARBITER`"]
pub type DSP_DRAM45_ARBITER_R = crate::R<u8, DSP_DRAM45_ARBITER_A>;
impl DSP_DRAM45_ARBITER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSP_DRAM45_ARBITER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DSP_DRAM45_ARBITER_A::DSP_DRAM45_DSP_PRIORITY),
            1 => Val(DSP_DRAM45_ARBITER_A::DSP_DRAM45_CM3_PRIORITY),
            2 => Val(DSP_DRAM45_ARBITER_A::DSP_DRAM45_ROUND_ROBIN_PRIORITY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM45_DSP_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_dram45_dsp_priority(&self) -> bool {
        *self == DSP_DRAM45_ARBITER_A::DSP_DRAM45_DSP_PRIORITY
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM45_CM3_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_dram45_cm3_priority(&self) -> bool {
        *self == DSP_DRAM45_ARBITER_A::DSP_DRAM45_CM3_PRIORITY
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM45_ROUND_ROBIN_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_dram45_round_robin_priority(&self) -> bool {
        *self == DSP_DRAM45_ARBITER_A::DSP_DRAM45_ROUND_ROBIN_PRIORITY
    }
}
#[doc = "Write proxy for field `DSP_DRAM45_ARBITER`"]
pub struct DSP_DRAM45_ARBITER_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM45_ARBITER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM45_ARBITER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DSP has priority access to the DSP DRAM4 and DRAM5 (above CM3 and DMA)"]
    #[inline(always)]
    pub fn dsp_dram45_dsp_priority(self) -> &'a mut W {
        self.variant(DSP_DRAM45_ARBITER_A::DSP_DRAM45_DSP_PRIORITY)
    }
    #[doc = "CM3 has priority access to the DSP DRAM4 and DRAM5 (above DSP and DMA)"]
    #[inline(always)]
    pub fn dsp_dram45_cm3_priority(self) -> &'a mut W {
        self.variant(DSP_DRAM45_ARBITER_A::DSP_DRAM45_CM3_PRIORITY)
    }
    #[doc = "Round robin priority access to the DSP DRAM4 and DRAM5"]
    #[inline(always)]
    pub fn dsp_dram45_round_robin_priority(self) -> &'a mut W {
        self.variant(DSP_DRAM45_ARBITER_A::DSP_DRAM45_ROUND_ROBIN_PRIORITY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "DSP DRAM2 and DRAM3 arbiter configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSP_DRAM23_ARBITER_A {
    #[doc = "0: DSP has priority access to the DSP DRAM2 and DRAM3 (above CM3 and DMA)"]
    DSP_DRAM23_DSP_PRIORITY = 0,
    #[doc = "1: CM3 has priority access to the DSP DRAM2 and DRAM3 (above DSP and DMA)"]
    DSP_DRAM23_CM3_PRIORITY = 1,
    #[doc = "2: Round robin priority access to the DSP DRAM2 and DRAM3"]
    DSP_DRAM23_ROUND_ROBIN_PRIORITY = 2,
}
impl From<DSP_DRAM23_ARBITER_A> for u8 {
    #[inline(always)]
    fn from(variant: DSP_DRAM23_ARBITER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DSP_DRAM23_ARBITER`"]
pub type DSP_DRAM23_ARBITER_R = crate::R<u8, DSP_DRAM23_ARBITER_A>;
impl DSP_DRAM23_ARBITER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSP_DRAM23_ARBITER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DSP_DRAM23_ARBITER_A::DSP_DRAM23_DSP_PRIORITY),
            1 => Val(DSP_DRAM23_ARBITER_A::DSP_DRAM23_CM3_PRIORITY),
            2 => Val(DSP_DRAM23_ARBITER_A::DSP_DRAM23_ROUND_ROBIN_PRIORITY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM23_DSP_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_dram23_dsp_priority(&self) -> bool {
        *self == DSP_DRAM23_ARBITER_A::DSP_DRAM23_DSP_PRIORITY
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM23_CM3_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_dram23_cm3_priority(&self) -> bool {
        *self == DSP_DRAM23_ARBITER_A::DSP_DRAM23_CM3_PRIORITY
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM23_ROUND_ROBIN_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_dram23_round_robin_priority(&self) -> bool {
        *self == DSP_DRAM23_ARBITER_A::DSP_DRAM23_ROUND_ROBIN_PRIORITY
    }
}
#[doc = "Write proxy for field `DSP_DRAM23_ARBITER`"]
pub struct DSP_DRAM23_ARBITER_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM23_ARBITER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM23_ARBITER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DSP has priority access to the DSP DRAM2 and DRAM3 (above CM3 and DMA)"]
    #[inline(always)]
    pub fn dsp_dram23_dsp_priority(self) -> &'a mut W {
        self.variant(DSP_DRAM23_ARBITER_A::DSP_DRAM23_DSP_PRIORITY)
    }
    #[doc = "CM3 has priority access to the DSP DRAM2 and DRAM3 (above DSP and DMA)"]
    #[inline(always)]
    pub fn dsp_dram23_cm3_priority(self) -> &'a mut W {
        self.variant(DSP_DRAM23_ARBITER_A::DSP_DRAM23_CM3_PRIORITY)
    }
    #[doc = "Round robin priority access to the DSP DRAM2 and DRAM3"]
    #[inline(always)]
    pub fn dsp_dram23_round_robin_priority(self) -> &'a mut W {
        self.variant(DSP_DRAM23_ARBITER_A::DSP_DRAM23_ROUND_ROBIN_PRIORITY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "DSP DRAM0 and DRAM1 arbiter configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSP_DRAM01_ARBITER_A {
    #[doc = "0: DSP has priority access to the DSP DRAM0 and DRAM1 (above CM3 and DMA)"]
    DSP_DRAM01_DSP_PRIORITY = 0,
    #[doc = "1: CM3 has priority access to the DSP DRAM0 and DRAM1 (above DSP and DMA)"]
    DSP_DRAM01_CM3_PRIORITY = 1,
    #[doc = "2: Round robin priority access to the DSP DRAM0 and DRAM1"]
    DSP_DRAM01_ROUND_ROBIN_PRIORITY = 2,
}
impl From<DSP_DRAM01_ARBITER_A> for u8 {
    #[inline(always)]
    fn from(variant: DSP_DRAM01_ARBITER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DSP_DRAM01_ARBITER`"]
pub type DSP_DRAM01_ARBITER_R = crate::R<u8, DSP_DRAM01_ARBITER_A>;
impl DSP_DRAM01_ARBITER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSP_DRAM01_ARBITER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DSP_DRAM01_ARBITER_A::DSP_DRAM01_DSP_PRIORITY),
            1 => Val(DSP_DRAM01_ARBITER_A::DSP_DRAM01_CM3_PRIORITY),
            2 => Val(DSP_DRAM01_ARBITER_A::DSP_DRAM01_ROUND_ROBIN_PRIORITY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM01_DSP_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_dram01_dsp_priority(&self) -> bool {
        *self == DSP_DRAM01_ARBITER_A::DSP_DRAM01_DSP_PRIORITY
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM01_CM3_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_dram01_cm3_priority(&self) -> bool {
        *self == DSP_DRAM01_ARBITER_A::DSP_DRAM01_CM3_PRIORITY
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM01_ROUND_ROBIN_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_dram01_round_robin_priority(&self) -> bool {
        *self == DSP_DRAM01_ARBITER_A::DSP_DRAM01_ROUND_ROBIN_PRIORITY
    }
}
#[doc = "Write proxy for field `DSP_DRAM01_ARBITER`"]
pub struct DSP_DRAM01_ARBITER_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM01_ARBITER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM01_ARBITER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DSP has priority access to the DSP DRAM0 and DRAM1 (above CM3 and DMA)"]
    #[inline(always)]
    pub fn dsp_dram01_dsp_priority(self) -> &'a mut W {
        self.variant(DSP_DRAM01_ARBITER_A::DSP_DRAM01_DSP_PRIORITY)
    }
    #[doc = "CM3 has priority access to the DSP DRAM0 and DRAM1 (above DSP and DMA)"]
    #[inline(always)]
    pub fn dsp_dram01_cm3_priority(self) -> &'a mut W {
        self.variant(DSP_DRAM01_ARBITER_A::DSP_DRAM01_CM3_PRIORITY)
    }
    #[doc = "Round robin priority access to the DSP DRAM0 and DRAM1"]
    #[inline(always)]
    pub fn dsp_dram01_round_robin_priority(self) -> &'a mut W {
        self.variant(DSP_DRAM01_ARBITER_A::DSP_DRAM01_ROUND_ROBIN_PRIORITY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "DSP PRAM3 arbiter configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSP_PRAM3_ARBITER_A {
    #[doc = "0: DSP has priority access to the DSP PRAM3 (above CM3 and DMA)"]
    DSP_PRAM3_DSP_PRIORITY = 0,
    #[doc = "1: CM3 has priority access to the DSP PRAM3 (above DSP and DMA)"]
    DSP_PRAM3_CM3_PRIORITY = 1,
    #[doc = "2: Round robin priority access to the DSP PRAM3"]
    DSP_PRAM3_ROUND_ROBIN_PRIORITY = 2,
}
impl From<DSP_PRAM3_ARBITER_A> for u8 {
    #[inline(always)]
    fn from(variant: DSP_PRAM3_ARBITER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DSP_PRAM3_ARBITER`"]
pub type DSP_PRAM3_ARBITER_R = crate::R<u8, DSP_PRAM3_ARBITER_A>;
impl DSP_PRAM3_ARBITER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSP_PRAM3_ARBITER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DSP_PRAM3_ARBITER_A::DSP_PRAM3_DSP_PRIORITY),
            1 => Val(DSP_PRAM3_ARBITER_A::DSP_PRAM3_CM3_PRIORITY),
            2 => Val(DSP_PRAM3_ARBITER_A::DSP_PRAM3_ROUND_ROBIN_PRIORITY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM3_DSP_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_pram3_dsp_priority(&self) -> bool {
        *self == DSP_PRAM3_ARBITER_A::DSP_PRAM3_DSP_PRIORITY
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM3_CM3_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_pram3_cm3_priority(&self) -> bool {
        *self == DSP_PRAM3_ARBITER_A::DSP_PRAM3_CM3_PRIORITY
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM3_ROUND_ROBIN_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_pram3_round_robin_priority(&self) -> bool {
        *self == DSP_PRAM3_ARBITER_A::DSP_PRAM3_ROUND_ROBIN_PRIORITY
    }
}
#[doc = "Write proxy for field `DSP_PRAM3_ARBITER`"]
pub struct DSP_PRAM3_ARBITER_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM3_ARBITER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM3_ARBITER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DSP has priority access to the DSP PRAM3 (above CM3 and DMA)"]
    #[inline(always)]
    pub fn dsp_pram3_dsp_priority(self) -> &'a mut W {
        self.variant(DSP_PRAM3_ARBITER_A::DSP_PRAM3_DSP_PRIORITY)
    }
    #[doc = "CM3 has priority access to the DSP PRAM3 (above DSP and DMA)"]
    #[inline(always)]
    pub fn dsp_pram3_cm3_priority(self) -> &'a mut W {
        self.variant(DSP_PRAM3_ARBITER_A::DSP_PRAM3_CM3_PRIORITY)
    }
    #[doc = "Round robin priority access to the DSP PRAM3"]
    #[inline(always)]
    pub fn dsp_pram3_round_robin_priority(self) -> &'a mut W {
        self.variant(DSP_PRAM3_ARBITER_A::DSP_PRAM3_ROUND_ROBIN_PRIORITY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "DSP PRAM2 arbiter configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSP_PRAM2_ARBITER_A {
    #[doc = "0: DSP has priority access to the DSP PRAM2 (above CM3 and DMA)"]
    DSP_PRAM2_DSP_PRIORITY = 0,
    #[doc = "1: CM3 has priority access to the DSP PRAM2 (above DSP and DMA)"]
    DSP_PRAM2_CM3_PRIORITY = 1,
    #[doc = "2: Round robin priority access to the DSP PRAM2"]
    DSP_PRAM2_ROUND_ROBIN_PRIORITY = 2,
}
impl From<DSP_PRAM2_ARBITER_A> for u8 {
    #[inline(always)]
    fn from(variant: DSP_PRAM2_ARBITER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DSP_PRAM2_ARBITER`"]
pub type DSP_PRAM2_ARBITER_R = crate::R<u8, DSP_PRAM2_ARBITER_A>;
impl DSP_PRAM2_ARBITER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSP_PRAM2_ARBITER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DSP_PRAM2_ARBITER_A::DSP_PRAM2_DSP_PRIORITY),
            1 => Val(DSP_PRAM2_ARBITER_A::DSP_PRAM2_CM3_PRIORITY),
            2 => Val(DSP_PRAM2_ARBITER_A::DSP_PRAM2_ROUND_ROBIN_PRIORITY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM2_DSP_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_pram2_dsp_priority(&self) -> bool {
        *self == DSP_PRAM2_ARBITER_A::DSP_PRAM2_DSP_PRIORITY
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM2_CM3_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_pram2_cm3_priority(&self) -> bool {
        *self == DSP_PRAM2_ARBITER_A::DSP_PRAM2_CM3_PRIORITY
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM2_ROUND_ROBIN_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_pram2_round_robin_priority(&self) -> bool {
        *self == DSP_PRAM2_ARBITER_A::DSP_PRAM2_ROUND_ROBIN_PRIORITY
    }
}
#[doc = "Write proxy for field `DSP_PRAM2_ARBITER`"]
pub struct DSP_PRAM2_ARBITER_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM2_ARBITER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM2_ARBITER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DSP has priority access to the DSP PRAM2 (above CM3 and DMA)"]
    #[inline(always)]
    pub fn dsp_pram2_dsp_priority(self) -> &'a mut W {
        self.variant(DSP_PRAM2_ARBITER_A::DSP_PRAM2_DSP_PRIORITY)
    }
    #[doc = "CM3 has priority access to the DSP PRAM2 (above DSP and DMA)"]
    #[inline(always)]
    pub fn dsp_pram2_cm3_priority(self) -> &'a mut W {
        self.variant(DSP_PRAM2_ARBITER_A::DSP_PRAM2_CM3_PRIORITY)
    }
    #[doc = "Round robin priority access to the DSP PRAM2"]
    #[inline(always)]
    pub fn dsp_pram2_round_robin_priority(self) -> &'a mut W {
        self.variant(DSP_PRAM2_ARBITER_A::DSP_PRAM2_ROUND_ROBIN_PRIORITY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "DSP PRAM1 arbiter configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSP_PRAM1_ARBITER_A {
    #[doc = "0: DSP has priority access to the DSP PRAM1 (above CM3 and DMA)"]
    DSP_PRAM1_DSP_PRIORITY = 0,
    #[doc = "1: CM3 has priority access to the DSP PRAM1 (above DSP and DMA)"]
    DSP_PRAM1_CM3_PRIORITY = 1,
    #[doc = "2: Round robin priority access to the DSP PRAM1"]
    DSP_PRAM1_ROUND_ROBIN_PRIORITY = 2,
}
impl From<DSP_PRAM1_ARBITER_A> for u8 {
    #[inline(always)]
    fn from(variant: DSP_PRAM1_ARBITER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DSP_PRAM1_ARBITER`"]
pub type DSP_PRAM1_ARBITER_R = crate::R<u8, DSP_PRAM1_ARBITER_A>;
impl DSP_PRAM1_ARBITER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSP_PRAM1_ARBITER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DSP_PRAM1_ARBITER_A::DSP_PRAM1_DSP_PRIORITY),
            1 => Val(DSP_PRAM1_ARBITER_A::DSP_PRAM1_CM3_PRIORITY),
            2 => Val(DSP_PRAM1_ARBITER_A::DSP_PRAM1_ROUND_ROBIN_PRIORITY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM1_DSP_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_pram1_dsp_priority(&self) -> bool {
        *self == DSP_PRAM1_ARBITER_A::DSP_PRAM1_DSP_PRIORITY
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM1_CM3_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_pram1_cm3_priority(&self) -> bool {
        *self == DSP_PRAM1_ARBITER_A::DSP_PRAM1_CM3_PRIORITY
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM1_ROUND_ROBIN_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_pram1_round_robin_priority(&self) -> bool {
        *self == DSP_PRAM1_ARBITER_A::DSP_PRAM1_ROUND_ROBIN_PRIORITY
    }
}
#[doc = "Write proxy for field `DSP_PRAM1_ARBITER`"]
pub struct DSP_PRAM1_ARBITER_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM1_ARBITER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM1_ARBITER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DSP has priority access to the DSP PRAM1 (above CM3 and DMA)"]
    #[inline(always)]
    pub fn dsp_pram1_dsp_priority(self) -> &'a mut W {
        self.variant(DSP_PRAM1_ARBITER_A::DSP_PRAM1_DSP_PRIORITY)
    }
    #[doc = "CM3 has priority access to the DSP PRAM1 (above DSP and DMA)"]
    #[inline(always)]
    pub fn dsp_pram1_cm3_priority(self) -> &'a mut W {
        self.variant(DSP_PRAM1_ARBITER_A::DSP_PRAM1_CM3_PRIORITY)
    }
    #[doc = "Round robin priority access to the DSP PRAM1"]
    #[inline(always)]
    pub fn dsp_pram1_round_robin_priority(self) -> &'a mut W {
        self.variant(DSP_PRAM1_ARBITER_A::DSP_PRAM1_ROUND_ROBIN_PRIORITY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "DSP PRAM0 arbiter configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSP_PRAM0_ARBITER_A {
    #[doc = "0: DSP has priority access to the DSP PRAM0 (above CM3 and DMA)"]
    DSP_PRAM0_DSP_PRIORITY = 0,
    #[doc = "1: CM3 has priority access to the DSP PRAM0 (above DSP and DMA)"]
    DSP_PRAM0_CM3_PRIORITY = 1,
    #[doc = "2: Round robin priority access to the DSP PRAM0"]
    DSP_PRAM0_ROUND_ROBIN_PRIORITY = 2,
}
impl From<DSP_PRAM0_ARBITER_A> for u8 {
    #[inline(always)]
    fn from(variant: DSP_PRAM0_ARBITER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DSP_PRAM0_ARBITER`"]
pub type DSP_PRAM0_ARBITER_R = crate::R<u8, DSP_PRAM0_ARBITER_A>;
impl DSP_PRAM0_ARBITER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSP_PRAM0_ARBITER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DSP_PRAM0_ARBITER_A::DSP_PRAM0_DSP_PRIORITY),
            1 => Val(DSP_PRAM0_ARBITER_A::DSP_PRAM0_CM3_PRIORITY),
            2 => Val(DSP_PRAM0_ARBITER_A::DSP_PRAM0_ROUND_ROBIN_PRIORITY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM0_DSP_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_pram0_dsp_priority(&self) -> bool {
        *self == DSP_PRAM0_ARBITER_A::DSP_PRAM0_DSP_PRIORITY
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM0_CM3_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_pram0_cm3_priority(&self) -> bool {
        *self == DSP_PRAM0_ARBITER_A::DSP_PRAM0_CM3_PRIORITY
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM0_ROUND_ROBIN_PRIORITY`"]
    #[inline(always)]
    pub fn is_dsp_pram0_round_robin_priority(&self) -> bool {
        *self == DSP_PRAM0_ARBITER_A::DSP_PRAM0_ROUND_ROBIN_PRIORITY
    }
}
#[doc = "Write proxy for field `DSP_PRAM0_ARBITER`"]
pub struct DSP_PRAM0_ARBITER_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM0_ARBITER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM0_ARBITER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DSP has priority access to the DSP PRAM0 (above CM3 and DMA)"]
    #[inline(always)]
    pub fn dsp_pram0_dsp_priority(self) -> &'a mut W {
        self.variant(DSP_PRAM0_ARBITER_A::DSP_PRAM0_DSP_PRIORITY)
    }
    #[doc = "CM3 has priority access to the DSP PRAM0 (above DSP and DMA)"]
    #[inline(always)]
    pub fn dsp_pram0_cm3_priority(self) -> &'a mut W {
        self.variant(DSP_PRAM0_ARBITER_A::DSP_PRAM0_CM3_PRIORITY)
    }
    #[doc = "Round robin priority access to the DSP PRAM0"]
    #[inline(always)]
    pub fn dsp_pram0_round_robin_priority(self) -> &'a mut W {
        self.variant(DSP_PRAM0_ARBITER_A::DSP_PRAM0_ROUND_ROBIN_PRIORITY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Baseband DRAM1 arbiter configuration\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BB_DRAM1_ARBITER_A {
    #[doc = "0: Baseband controller has priority access to the BB DRAM1 (above CM3 and DMA)"]
    BB_DRAM1_BB_PRIORITY = 0,
    #[doc = "1: CM3 has priority access to the BB DRAM1 (above baseband and DMA)"]
    BB_DRAM1_CM3_PRIORITY = 1,
    #[doc = "2: Round robin priority access to the BB DRAM1"]
    BB_DRAM1_ROUND_ROBIN_PRIORITY = 2,
    #[doc = "3: Smart priority access to the BB DRAM1"]
    BB_DRAM1_SMART_PRIORITY = 3,
}
impl From<BB_DRAM1_ARBITER_A> for u8 {
    #[inline(always)]
    fn from(variant: BB_DRAM1_ARBITER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BB_DRAM1_ARBITER`"]
pub type BB_DRAM1_ARBITER_R = crate::R<u8, BB_DRAM1_ARBITER_A>;
impl BB_DRAM1_ARBITER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_DRAM1_ARBITER_A {
        match self.bits {
            0 => BB_DRAM1_ARBITER_A::BB_DRAM1_BB_PRIORITY,
            1 => BB_DRAM1_ARBITER_A::BB_DRAM1_CM3_PRIORITY,
            2 => BB_DRAM1_ARBITER_A::BB_DRAM1_ROUND_ROBIN_PRIORITY,
            3 => BB_DRAM1_ARBITER_A::BB_DRAM1_SMART_PRIORITY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BB_DRAM1_BB_PRIORITY`"]
    #[inline(always)]
    pub fn is_bb_dram1_bb_priority(&self) -> bool {
        *self == BB_DRAM1_ARBITER_A::BB_DRAM1_BB_PRIORITY
    }
    #[doc = "Checks if the value of the field is `BB_DRAM1_CM3_PRIORITY`"]
    #[inline(always)]
    pub fn is_bb_dram1_cm3_priority(&self) -> bool {
        *self == BB_DRAM1_ARBITER_A::BB_DRAM1_CM3_PRIORITY
    }
    #[doc = "Checks if the value of the field is `BB_DRAM1_ROUND_ROBIN_PRIORITY`"]
    #[inline(always)]
    pub fn is_bb_dram1_round_robin_priority(&self) -> bool {
        *self == BB_DRAM1_ARBITER_A::BB_DRAM1_ROUND_ROBIN_PRIORITY
    }
    #[doc = "Checks if the value of the field is `BB_DRAM1_SMART_PRIORITY`"]
    #[inline(always)]
    pub fn is_bb_dram1_smart_priority(&self) -> bool {
        *self == BB_DRAM1_ARBITER_A::BB_DRAM1_SMART_PRIORITY
    }
}
#[doc = "Write proxy for field `BB_DRAM1_ARBITER`"]
pub struct BB_DRAM1_ARBITER_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_DRAM1_ARBITER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_DRAM1_ARBITER_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Baseband controller has priority access to the BB DRAM1 (above CM3 and DMA)"]
    #[inline(always)]
    pub fn bb_dram1_bb_priority(self) -> &'a mut W {
        self.variant(BB_DRAM1_ARBITER_A::BB_DRAM1_BB_PRIORITY)
    }
    #[doc = "CM3 has priority access to the BB DRAM1 (above baseband and DMA)"]
    #[inline(always)]
    pub fn bb_dram1_cm3_priority(self) -> &'a mut W {
        self.variant(BB_DRAM1_ARBITER_A::BB_DRAM1_CM3_PRIORITY)
    }
    #[doc = "Round robin priority access to the BB DRAM1"]
    #[inline(always)]
    pub fn bb_dram1_round_robin_priority(self) -> &'a mut W {
        self.variant(BB_DRAM1_ARBITER_A::BB_DRAM1_ROUND_ROBIN_PRIORITY)
    }
    #[doc = "Smart priority access to the BB DRAM1"]
    #[inline(always)]
    pub fn bb_dram1_smart_priority(self) -> &'a mut W {
        self.variant(BB_DRAM1_ARBITER_A::BB_DRAM1_SMART_PRIORITY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Baseband DRAM0 arbiter configuration\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BB_DRAM0_ARBITER_A {
    #[doc = "0: Baseband controller has priority access to the BB DRAM0 (above CM3 and DMA)"]
    BB_DRAM0_BB_PRIORITY = 0,
    #[doc = "1: CM3 has priority access to the BB DRAM0 (above baseband and DMA)"]
    BB_DRAM0_CM3_PRIORITY = 1,
    #[doc = "2: Round robin priority access to the BB DRAM0"]
    BB_DRAM0_ROUND_ROBIN_PRIORITY = 2,
    #[doc = "3: Smart priority access to the BB DRAM0"]
    BB_DRAM0_SMART_PRIORITY = 3,
}
impl From<BB_DRAM0_ARBITER_A> for u8 {
    #[inline(always)]
    fn from(variant: BB_DRAM0_ARBITER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BB_DRAM0_ARBITER`"]
pub type BB_DRAM0_ARBITER_R = crate::R<u8, BB_DRAM0_ARBITER_A>;
impl BB_DRAM0_ARBITER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_DRAM0_ARBITER_A {
        match self.bits {
            0 => BB_DRAM0_ARBITER_A::BB_DRAM0_BB_PRIORITY,
            1 => BB_DRAM0_ARBITER_A::BB_DRAM0_CM3_PRIORITY,
            2 => BB_DRAM0_ARBITER_A::BB_DRAM0_ROUND_ROBIN_PRIORITY,
            3 => BB_DRAM0_ARBITER_A::BB_DRAM0_SMART_PRIORITY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BB_DRAM0_BB_PRIORITY`"]
    #[inline(always)]
    pub fn is_bb_dram0_bb_priority(&self) -> bool {
        *self == BB_DRAM0_ARBITER_A::BB_DRAM0_BB_PRIORITY
    }
    #[doc = "Checks if the value of the field is `BB_DRAM0_CM3_PRIORITY`"]
    #[inline(always)]
    pub fn is_bb_dram0_cm3_priority(&self) -> bool {
        *self == BB_DRAM0_ARBITER_A::BB_DRAM0_CM3_PRIORITY
    }
    #[doc = "Checks if the value of the field is `BB_DRAM0_ROUND_ROBIN_PRIORITY`"]
    #[inline(always)]
    pub fn is_bb_dram0_round_robin_priority(&self) -> bool {
        *self == BB_DRAM0_ARBITER_A::BB_DRAM0_ROUND_ROBIN_PRIORITY
    }
    #[doc = "Checks if the value of the field is `BB_DRAM0_SMART_PRIORITY`"]
    #[inline(always)]
    pub fn is_bb_dram0_smart_priority(&self) -> bool {
        *self == BB_DRAM0_ARBITER_A::BB_DRAM0_SMART_PRIORITY
    }
}
#[doc = "Write proxy for field `BB_DRAM0_ARBITER`"]
pub struct BB_DRAM0_ARBITER_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_DRAM0_ARBITER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_DRAM0_ARBITER_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Baseband controller has priority access to the BB DRAM0 (above CM3 and DMA)"]
    #[inline(always)]
    pub fn bb_dram0_bb_priority(self) -> &'a mut W {
        self.variant(BB_DRAM0_ARBITER_A::BB_DRAM0_BB_PRIORITY)
    }
    #[doc = "CM3 has priority access to the BB DRAM0 (above baseband and DMA)"]
    #[inline(always)]
    pub fn bb_dram0_cm3_priority(self) -> &'a mut W {
        self.variant(BB_DRAM0_ARBITER_A::BB_DRAM0_CM3_PRIORITY)
    }
    #[doc = "Round robin priority access to the BB DRAM0"]
    #[inline(always)]
    pub fn bb_dram0_round_robin_priority(self) -> &'a mut W {
        self.variant(BB_DRAM0_ARBITER_A::BB_DRAM0_ROUND_ROBIN_PRIORITY)
    }
    #[doc = "Smart priority access to the BB DRAM0"]
    #[inline(always)]
    pub fn bb_dram0_smart_priority(self) -> &'a mut W {
        self.variant(BB_DRAM0_ARBITER_A::BB_DRAM0_SMART_PRIORITY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "DRAM1 and DRAM2 arbiter configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRAM12_ARBITER_A {
    #[doc = "0: CM3 has priority access to the DRAM1 and DRAM2 (above DSP and DMA)"]
    DRAM12_CM3_PRIORITY = 0,
    #[doc = "1: DSP has priority access to the DRAM1 and DRAM2 (above CM3 and DMA)"]
    DRAM12_DSP_PRIORITY = 1,
    #[doc = "2: Round robin priority access to the DRAM1 and DRAM2"]
    DRAM12_ROUND_ROBIN_PRIORITY = 2,
}
impl From<DRAM12_ARBITER_A> for u8 {
    #[inline(always)]
    fn from(variant: DRAM12_ARBITER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DRAM12_ARBITER`"]
pub type DRAM12_ARBITER_R = crate::R<u8, DRAM12_ARBITER_A>;
impl DRAM12_ARBITER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DRAM12_ARBITER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DRAM12_ARBITER_A::DRAM12_CM3_PRIORITY),
            1 => Val(DRAM12_ARBITER_A::DRAM12_DSP_PRIORITY),
            2 => Val(DRAM12_ARBITER_A::DRAM12_ROUND_ROBIN_PRIORITY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DRAM12_CM3_PRIORITY`"]
    #[inline(always)]
    pub fn is_dram12_cm3_priority(&self) -> bool {
        *self == DRAM12_ARBITER_A::DRAM12_CM3_PRIORITY
    }
    #[doc = "Checks if the value of the field is `DRAM12_DSP_PRIORITY`"]
    #[inline(always)]
    pub fn is_dram12_dsp_priority(&self) -> bool {
        *self == DRAM12_ARBITER_A::DRAM12_DSP_PRIORITY
    }
    #[doc = "Checks if the value of the field is `DRAM12_ROUND_ROBIN_PRIORITY`"]
    #[inline(always)]
    pub fn is_dram12_round_robin_priority(&self) -> bool {
        *self == DRAM12_ARBITER_A::DRAM12_ROUND_ROBIN_PRIORITY
    }
}
#[doc = "Write proxy for field `DRAM12_ARBITER`"]
pub struct DRAM12_ARBITER_W<'a> {
    w: &'a mut W,
}
impl<'a> DRAM12_ARBITER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRAM12_ARBITER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CM3 has priority access to the DRAM1 and DRAM2 (above DSP and DMA)"]
    #[inline(always)]
    pub fn dram12_cm3_priority(self) -> &'a mut W {
        self.variant(DRAM12_ARBITER_A::DRAM12_CM3_PRIORITY)
    }
    #[doc = "DSP has priority access to the DRAM1 and DRAM2 (above CM3 and DMA)"]
    #[inline(always)]
    pub fn dram12_dsp_priority(self) -> &'a mut W {
        self.variant(DRAM12_ARBITER_A::DRAM12_DSP_PRIORITY)
    }
    #[doc = "Round robin priority access to the DRAM1 and DRAM2"]
    #[inline(always)]
    pub fn dram12_round_robin_priority(self) -> &'a mut W {
        self.variant(DRAM12_ARBITER_A::DRAM12_ROUND_ROBIN_PRIORITY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "DRAM0 arbiter configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRAM0_ARBITER_A {
    #[doc = "0: CM3 has priority access to the DRAM0 (above DMA)"]
    DRAM0_CM3_PRIORITY = 0,
    #[doc = "1: Round robin priority access to the DRAM0"]
    DRAM0_ROUND_ROBIN_PRIORITY = 1,
}
impl From<DRAM0_ARBITER_A> for bool {
    #[inline(always)]
    fn from(variant: DRAM0_ARBITER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRAM0_ARBITER`"]
pub type DRAM0_ARBITER_R = crate::R<bool, DRAM0_ARBITER_A>;
impl DRAM0_ARBITER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRAM0_ARBITER_A {
        match self.bits {
            false => DRAM0_ARBITER_A::DRAM0_CM3_PRIORITY,
            true => DRAM0_ARBITER_A::DRAM0_ROUND_ROBIN_PRIORITY,
        }
    }
    #[doc = "Checks if the value of the field is `DRAM0_CM3_PRIORITY`"]
    #[inline(always)]
    pub fn is_dram0_cm3_priority(&self) -> bool {
        *self == DRAM0_ARBITER_A::DRAM0_CM3_PRIORITY
    }
    #[doc = "Checks if the value of the field is `DRAM0_ROUND_ROBIN_PRIORITY`"]
    #[inline(always)]
    pub fn is_dram0_round_robin_priority(&self) -> bool {
        *self == DRAM0_ARBITER_A::DRAM0_ROUND_ROBIN_PRIORITY
    }
}
#[doc = "Write proxy for field `DRAM0_ARBITER`"]
pub struct DRAM0_ARBITER_W<'a> {
    w: &'a mut W,
}
impl<'a> DRAM0_ARBITER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRAM0_ARBITER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CM3 has priority access to the DRAM0 (above DMA)"]
    #[inline(always)]
    pub fn dram0_cm3_priority(self) -> &'a mut W {
        self.variant(DRAM0_ARBITER_A::DRAM0_CM3_PRIORITY)
    }
    #[doc = "Round robin priority access to the DRAM0"]
    #[inline(always)]
    pub fn dram0_round_robin_priority(self) -> &'a mut W {
        self.variant(DRAM0_ARBITER_A::DRAM0_ROUND_ROBIN_PRIORITY)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "PRAM0 to PRAM3 arbiter configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRAM_ARBITER_A {
    #[doc = "0: CM3 has priority access to the PRAM0 to PRAM3 (above DMA)"]
    PRAM_CM3_PRIORITY = 0,
    #[doc = "1: Round robin priority access to the PRAM0 to PRAM3"]
    PRAM_ROUND_ROBIN_PRIORITY = 1,
}
impl From<PRAM_ARBITER_A> for bool {
    #[inline(always)]
    fn from(variant: PRAM_ARBITER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRAM_ARBITER`"]
pub type PRAM_ARBITER_R = crate::R<bool, PRAM_ARBITER_A>;
impl PRAM_ARBITER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRAM_ARBITER_A {
        match self.bits {
            false => PRAM_ARBITER_A::PRAM_CM3_PRIORITY,
            true => PRAM_ARBITER_A::PRAM_ROUND_ROBIN_PRIORITY,
        }
    }
    #[doc = "Checks if the value of the field is `PRAM_CM3_PRIORITY`"]
    #[inline(always)]
    pub fn is_pram_cm3_priority(&self) -> bool {
        *self == PRAM_ARBITER_A::PRAM_CM3_PRIORITY
    }
    #[doc = "Checks if the value of the field is `PRAM_ROUND_ROBIN_PRIORITY`"]
    #[inline(always)]
    pub fn is_pram_round_robin_priority(&self) -> bool {
        *self == PRAM_ARBITER_A::PRAM_ROUND_ROBIN_PRIORITY
    }
}
#[doc = "Write proxy for field `PRAM_ARBITER`"]
pub struct PRAM_ARBITER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRAM_ARBITER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRAM_ARBITER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CM3 has priority access to the PRAM0 to PRAM3 (above DMA)"]
    #[inline(always)]
    pub fn pram_cm3_priority(self) -> &'a mut W {
        self.variant(PRAM_ARBITER_A::PRAM_CM3_PRIORITY)
    }
    #[doc = "Round robin priority access to the PRAM0 to PRAM3"]
    #[inline(always)]
    pub fn pram_round_robin_priority(self) -> &'a mut W {
        self.variant(PRAM_ARBITER_A::PRAM_ROUND_ROBIN_PRIORITY)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Round-robin token generation configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROUND_ROBIN_TOKEN_A {
    #[doc = "0: Real-time DMA priority mode: After 7 cycles a pending high priority DMA access automatically gets the token"]
    REALTIME_DMA_MODE = 0,
    #[doc = "1: Continuous round-robin mode: Token is rotating every SYSCLK cycle"]
    ROUND_ROBIN_MODE = 1,
}
impl From<ROUND_ROBIN_TOKEN_A> for bool {
    #[inline(always)]
    fn from(variant: ROUND_ROBIN_TOKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ROUND_ROBIN_TOKEN`"]
pub type ROUND_ROBIN_TOKEN_R = crate::R<bool, ROUND_ROBIN_TOKEN_A>;
impl ROUND_ROBIN_TOKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROUND_ROBIN_TOKEN_A {
        match self.bits {
            false => ROUND_ROBIN_TOKEN_A::REALTIME_DMA_MODE,
            true => ROUND_ROBIN_TOKEN_A::ROUND_ROBIN_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `REALTIME_DMA_MODE`"]
    #[inline(always)]
    pub fn is_realtime_dma_mode(&self) -> bool {
        *self == ROUND_ROBIN_TOKEN_A::REALTIME_DMA_MODE
    }
    #[doc = "Checks if the value of the field is `ROUND_ROBIN_MODE`"]
    #[inline(always)]
    pub fn is_round_robin_mode(&self) -> bool {
        *self == ROUND_ROBIN_TOKEN_A::ROUND_ROBIN_MODE
    }
}
#[doc = "Write proxy for field `ROUND_ROBIN_TOKEN`"]
pub struct ROUND_ROBIN_TOKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROUND_ROBIN_TOKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROUND_ROBIN_TOKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Real-time DMA priority mode: After 7 cycles a pending high priority DMA access automatically gets the token"]
    #[inline(always)]
    pub fn realtime_dma_mode(self) -> &'a mut W {
        self.variant(ROUND_ROBIN_TOKEN_A::REALTIME_DMA_MODE)
    }
    #[doc = "Continuous round-robin mode: Token is rotating every SYSCLK cycle"]
    #[inline(always)]
    pub fn round_robin_mode(self) -> &'a mut W {
        self.variant(ROUND_ROBIN_TOKEN_A::ROUND_ROBIN_MODE)
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
    #[doc = "Bits 28:29 - DSP DRAM4 and DRAM5 arbiter configuration"]
    #[inline(always)]
    pub fn dsp_dram45_arbiter(&self) -> DSP_DRAM45_ARBITER_R {
        DSP_DRAM45_ARBITER_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - DSP DRAM2 and DRAM3 arbiter configuration"]
    #[inline(always)]
    pub fn dsp_dram23_arbiter(&self) -> DSP_DRAM23_ARBITER_R {
        DSP_DRAM23_ARBITER_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - DSP DRAM0 and DRAM1 arbiter configuration"]
    #[inline(always)]
    pub fn dsp_dram01_arbiter(&self) -> DSP_DRAM01_ARBITER_R {
        DSP_DRAM01_ARBITER_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - DSP PRAM3 arbiter configuration"]
    #[inline(always)]
    pub fn dsp_pram3_arbiter(&self) -> DSP_PRAM3_ARBITER_R {
        DSP_PRAM3_ARBITER_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - DSP PRAM2 arbiter configuration"]
    #[inline(always)]
    pub fn dsp_pram2_arbiter(&self) -> DSP_PRAM2_ARBITER_R {
        DSP_PRAM2_ARBITER_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - DSP PRAM1 arbiter configuration"]
    #[inline(always)]
    pub fn dsp_pram1_arbiter(&self) -> DSP_PRAM1_ARBITER_R {
        DSP_PRAM1_ARBITER_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - DSP PRAM0 arbiter configuration"]
    #[inline(always)]
    pub fn dsp_pram0_arbiter(&self) -> DSP_PRAM0_ARBITER_R {
        DSP_PRAM0_ARBITER_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Baseband DRAM1 arbiter configuration"]
    #[inline(always)]
    pub fn bb_dram1_arbiter(&self) -> BB_DRAM1_ARBITER_R {
        BB_DRAM1_ARBITER_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Baseband DRAM0 arbiter configuration"]
    #[inline(always)]
    pub fn bb_dram0_arbiter(&self) -> BB_DRAM0_ARBITER_R {
        BB_DRAM0_ARBITER_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - DRAM1 and DRAM2 arbiter configuration"]
    #[inline(always)]
    pub fn dram12_arbiter(&self) -> DRAM12_ARBITER_R {
        DRAM12_ARBITER_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 2 - DRAM0 arbiter configuration"]
    #[inline(always)]
    pub fn dram0_arbiter(&self) -> DRAM0_ARBITER_R {
        DRAM0_ARBITER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - PRAM0 to PRAM3 arbiter configuration"]
    #[inline(always)]
    pub fn pram_arbiter(&self) -> PRAM_ARBITER_R {
        PRAM_ARBITER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Round-robin token generation configuration"]
    #[inline(always)]
    pub fn round_robin_token(&self) -> ROUND_ROBIN_TOKEN_R {
        ROUND_ROBIN_TOKEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:29 - DSP DRAM4 and DRAM5 arbiter configuration"]
    #[inline(always)]
    pub fn dsp_dram45_arbiter(&mut self) -> DSP_DRAM45_ARBITER_W {
        DSP_DRAM45_ARBITER_W { w: self }
    }
    #[doc = "Bits 26:27 - DSP DRAM2 and DRAM3 arbiter configuration"]
    #[inline(always)]
    pub fn dsp_dram23_arbiter(&mut self) -> DSP_DRAM23_ARBITER_W {
        DSP_DRAM23_ARBITER_W { w: self }
    }
    #[doc = "Bits 24:25 - DSP DRAM0 and DRAM1 arbiter configuration"]
    #[inline(always)]
    pub fn dsp_dram01_arbiter(&mut self) -> DSP_DRAM01_ARBITER_W {
        DSP_DRAM01_ARBITER_W { w: self }
    }
    #[doc = "Bits 22:23 - DSP PRAM3 arbiter configuration"]
    #[inline(always)]
    pub fn dsp_pram3_arbiter(&mut self) -> DSP_PRAM3_ARBITER_W {
        DSP_PRAM3_ARBITER_W { w: self }
    }
    #[doc = "Bits 20:21 - DSP PRAM2 arbiter configuration"]
    #[inline(always)]
    pub fn dsp_pram2_arbiter(&mut self) -> DSP_PRAM2_ARBITER_W {
        DSP_PRAM2_ARBITER_W { w: self }
    }
    #[doc = "Bits 18:19 - DSP PRAM1 arbiter configuration"]
    #[inline(always)]
    pub fn dsp_pram1_arbiter(&mut self) -> DSP_PRAM1_ARBITER_W {
        DSP_PRAM1_ARBITER_W { w: self }
    }
    #[doc = "Bits 16:17 - DSP PRAM0 arbiter configuration"]
    #[inline(always)]
    pub fn dsp_pram0_arbiter(&mut self) -> DSP_PRAM0_ARBITER_W {
        DSP_PRAM0_ARBITER_W { w: self }
    }
    #[doc = "Bits 10:11 - Baseband DRAM1 arbiter configuration"]
    #[inline(always)]
    pub fn bb_dram1_arbiter(&mut self) -> BB_DRAM1_ARBITER_W {
        BB_DRAM1_ARBITER_W { w: self }
    }
    #[doc = "Bits 8:9 - Baseband DRAM0 arbiter configuration"]
    #[inline(always)]
    pub fn bb_dram0_arbiter(&mut self) -> BB_DRAM0_ARBITER_W {
        BB_DRAM0_ARBITER_W { w: self }
    }
    #[doc = "Bits 4:5 - DRAM1 and DRAM2 arbiter configuration"]
    #[inline(always)]
    pub fn dram12_arbiter(&mut self) -> DRAM12_ARBITER_W {
        DRAM12_ARBITER_W { w: self }
    }
    #[doc = "Bit 2 - DRAM0 arbiter configuration"]
    #[inline(always)]
    pub fn dram0_arbiter(&mut self) -> DRAM0_ARBITER_W {
        DRAM0_ARBITER_W { w: self }
    }
    #[doc = "Bit 1 - PRAM0 to PRAM3 arbiter configuration"]
    #[inline(always)]
    pub fn pram_arbiter(&mut self) -> PRAM_ARBITER_W {
        PRAM_ARBITER_W { w: self }
    }
    #[doc = "Bit 0 - Round-robin token generation configuration"]
    #[inline(always)]
    pub fn round_robin_token(&mut self) -> ROUND_ROBIN_TOKEN_W {
        ROUND_ROBIN_TOKEN_W { w: self }
    }
}
