#[doc = "Reader of register SYSCTRL_MEM_TIMING_CFG"]
pub type R = crate::R<u32, super::SYSCTRL_MEM_TIMING_CFG>;
#[doc = "Writer for register SYSCTRL_MEM_TIMING_CFG"]
pub type W = crate::W<u32, super::SYSCTRL_MEM_TIMING_CFG>;
#[doc = "Register SYSCTRL_MEM_TIMING_CFG `reset()`'s with value 0x2d"]
impl crate::ResetValue for super::SYSCTRL_MEM_TIMING_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2d
    }
}
#[doc = "DSP_PRAM extra write margin configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSP_PRAM_EMAW_A {
    #[doc = "0: DSP_PRAM default/minimum extra write margin"]
    DSP_PRAM_EMAW_DEFAULT = 0,
    #[doc = "3: DSP_PRAM maximum extra write margin"]
    DSP_PRAM_EMAW_MAX = 3,
}
impl From<DSP_PRAM_EMAW_A> for u8 {
    #[inline(always)]
    fn from(variant: DSP_PRAM_EMAW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DSP_PRAM_EMAW`"]
pub type DSP_PRAM_EMAW_R = crate::R<u8, DSP_PRAM_EMAW_A>;
impl DSP_PRAM_EMAW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSP_PRAM_EMAW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DSP_PRAM_EMAW_A::DSP_PRAM_EMAW_DEFAULT),
            3 => Val(DSP_PRAM_EMAW_A::DSP_PRAM_EMAW_MAX),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM_EMAW_DEFAULT`"]
    #[inline(always)]
    pub fn is_dsp_pram_emaw_default(&self) -> bool {
        *self == DSP_PRAM_EMAW_A::DSP_PRAM_EMAW_DEFAULT
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM_EMAW_MAX`"]
    #[inline(always)]
    pub fn is_dsp_pram_emaw_max(&self) -> bool {
        *self == DSP_PRAM_EMAW_A::DSP_PRAM_EMAW_MAX
    }
}
#[doc = "Write proxy for field `DSP_PRAM_EMAW`"]
pub struct DSP_PRAM_EMAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM_EMAW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM_EMAW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DSP_PRAM default/minimum extra write margin"]
    #[inline(always)]
    pub fn dsp_pram_emaw_default(self) -> &'a mut W {
        self.variant(DSP_PRAM_EMAW_A::DSP_PRAM_EMAW_DEFAULT)
    }
    #[doc = "DSP_PRAM maximum extra write margin"]
    #[inline(always)]
    pub fn dsp_pram_emaw_max(self) -> &'a mut W {
        self.variant(DSP_PRAM_EMAW_A::DSP_PRAM_EMAW_MAX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "DSP_PRAM extra margin configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSP_PRAM_EMA_A {
    #[doc = "0: DSP_PRAM minimum extra margin"]
    DSP_PRAM_EMA_MIN = 0,
    #[doc = "2: DSP_PRAM default extra margin"]
    DSP_PRAM_EMA_DEFAULT = 2,
    #[doc = "7: DSP_PRAM maximum extra margin"]
    DSP_PRAM_EMA_MAX = 7,
}
impl From<DSP_PRAM_EMA_A> for u8 {
    #[inline(always)]
    fn from(variant: DSP_PRAM_EMA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DSP_PRAM_EMA`"]
pub type DSP_PRAM_EMA_R = crate::R<u8, DSP_PRAM_EMA_A>;
impl DSP_PRAM_EMA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSP_PRAM_EMA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DSP_PRAM_EMA_A::DSP_PRAM_EMA_MIN),
            2 => Val(DSP_PRAM_EMA_A::DSP_PRAM_EMA_DEFAULT),
            7 => Val(DSP_PRAM_EMA_A::DSP_PRAM_EMA_MAX),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM_EMA_MIN`"]
    #[inline(always)]
    pub fn is_dsp_pram_ema_min(&self) -> bool {
        *self == DSP_PRAM_EMA_A::DSP_PRAM_EMA_MIN
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM_EMA_DEFAULT`"]
    #[inline(always)]
    pub fn is_dsp_pram_ema_default(&self) -> bool {
        *self == DSP_PRAM_EMA_A::DSP_PRAM_EMA_DEFAULT
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM_EMA_MAX`"]
    #[inline(always)]
    pub fn is_dsp_pram_ema_max(&self) -> bool {
        *self == DSP_PRAM_EMA_A::DSP_PRAM_EMA_MAX
    }
}
#[doc = "Write proxy for field `DSP_PRAM_EMA`"]
pub struct DSP_PRAM_EMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM_EMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM_EMA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DSP_PRAM minimum extra margin"]
    #[inline(always)]
    pub fn dsp_pram_ema_min(self) -> &'a mut W {
        self.variant(DSP_PRAM_EMA_A::DSP_PRAM_EMA_MIN)
    }
    #[doc = "DSP_PRAM default extra margin"]
    #[inline(always)]
    pub fn dsp_pram_ema_default(self) -> &'a mut W {
        self.variant(DSP_PRAM_EMA_A::DSP_PRAM_EMA_DEFAULT)
    }
    #[doc = "DSP_PRAM maximum extra margin"]
    #[inline(always)]
    pub fn dsp_pram_ema_max(self) -> &'a mut W {
        self.variant(DSP_PRAM_EMA_A::DSP_PRAM_EMA_MAX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "PROM bitlines keeper configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROM_KEN_A {
    #[doc = "0: PROM bitlines keeper enabled"]
    PROM_KEN_ENABLED = 0,
    #[doc = "1: PROM bitlines keeper disabled"]
    PROM_KEN_DISABLED = 1,
}
impl From<PROM_KEN_A> for bool {
    #[inline(always)]
    fn from(variant: PROM_KEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PROM_KEN`"]
pub type PROM_KEN_R = crate::R<bool, PROM_KEN_A>;
impl PROM_KEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROM_KEN_A {
        match self.bits {
            false => PROM_KEN_A::PROM_KEN_ENABLED,
            true => PROM_KEN_A::PROM_KEN_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `PROM_KEN_ENABLED`"]
    #[inline(always)]
    pub fn is_prom_ken_enabled(&self) -> bool {
        *self == PROM_KEN_A::PROM_KEN_ENABLED
    }
    #[doc = "Checks if the value of the field is `PROM_KEN_DISABLED`"]
    #[inline(always)]
    pub fn is_prom_ken_disabled(&self) -> bool {
        *self == PROM_KEN_A::PROM_KEN_DISABLED
    }
}
#[doc = "Write proxy for field `PROM_KEN`"]
pub struct PROM_KEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PROM_KEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROM_KEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PROM bitlines keeper enabled"]
    #[inline(always)]
    pub fn prom_ken_enabled(self) -> &'a mut W {
        self.variant(PROM_KEN_A::PROM_KEN_ENABLED)
    }
    #[doc = "PROM bitlines keeper disabled"]
    #[inline(always)]
    pub fn prom_ken_disabled(self) -> &'a mut W {
        self.variant(PROM_KEN_A::PROM_KEN_DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "PROM extra margin configuration\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PROM_EMA_A {
    #[doc = "0: PROM minimum extra margin"]
    PROM_EMA_MIN = 0,
    #[doc = "5: PROM default extra margin"]
    PROM_EMA_DEFAULT = 5,
    #[doc = "7: PROM maximum extra margin"]
    PROM_EMA_MAX = 7,
}
impl From<PROM_EMA_A> for u8 {
    #[inline(always)]
    fn from(variant: PROM_EMA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PROM_EMA`"]
pub type PROM_EMA_R = crate::R<u8, PROM_EMA_A>;
impl PROM_EMA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PROM_EMA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PROM_EMA_A::PROM_EMA_MIN),
            5 => Val(PROM_EMA_A::PROM_EMA_DEFAULT),
            7 => Val(PROM_EMA_A::PROM_EMA_MAX),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PROM_EMA_MIN`"]
    #[inline(always)]
    pub fn is_prom_ema_min(&self) -> bool {
        *self == PROM_EMA_A::PROM_EMA_MIN
    }
    #[doc = "Checks if the value of the field is `PROM_EMA_DEFAULT`"]
    #[inline(always)]
    pub fn is_prom_ema_default(&self) -> bool {
        *self == PROM_EMA_A::PROM_EMA_DEFAULT
    }
    #[doc = "Checks if the value of the field is `PROM_EMA_MAX`"]
    #[inline(always)]
    pub fn is_prom_ema_max(&self) -> bool {
        *self == PROM_EMA_A::PROM_EMA_MAX
    }
}
#[doc = "Write proxy for field `PROM_EMA`"]
pub struct PROM_EMA_W<'a> {
    w: &'a mut W,
}
impl<'a> PROM_EMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROM_EMA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PROM minimum extra margin"]
    #[inline(always)]
    pub fn prom_ema_min(self) -> &'a mut W {
        self.variant(PROM_EMA_A::PROM_EMA_MIN)
    }
    #[doc = "PROM default extra margin"]
    #[inline(always)]
    pub fn prom_ema_default(self) -> &'a mut W {
        self.variant(PROM_EMA_A::PROM_EMA_DEFAULT)
    }
    #[doc = "PROM maximum extra margin"]
    #[inline(always)]
    pub fn prom_ema_max(self) -> &'a mut W {
        self.variant(PROM_EMA_A::PROM_EMA_MAX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - DSP_PRAM extra write margin configuration"]
    #[inline(always)]
    pub fn dsp_pram_emaw(&self) -> DSP_PRAM_EMAW_R {
        DSP_PRAM_EMAW_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - DSP_PRAM extra margin configuration"]
    #[inline(always)]
    pub fn dsp_pram_ema(&self) -> DSP_PRAM_EMA_R {
        DSP_PRAM_EMA_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - PROM bitlines keeper configuration"]
    #[inline(always)]
    pub fn prom_ken(&self) -> PROM_KEN_R {
        PROM_KEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - PROM extra margin configuration"]
    #[inline(always)]
    pub fn prom_ema(&self) -> PROM_EMA_R {
        PROM_EMA_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - DSP_PRAM extra write margin configuration"]
    #[inline(always)]
    pub fn dsp_pram_emaw(&mut self) -> DSP_PRAM_EMAW_W {
        DSP_PRAM_EMAW_W { w: self }
    }
    #[doc = "Bits 4:6 - DSP_PRAM extra margin configuration"]
    #[inline(always)]
    pub fn dsp_pram_ema(&mut self) -> DSP_PRAM_EMA_W {
        DSP_PRAM_EMA_W { w: self }
    }
    #[doc = "Bit 3 - PROM bitlines keeper configuration"]
    #[inline(always)]
    pub fn prom_ken(&mut self) -> PROM_KEN_W {
        PROM_KEN_W { w: self }
    }
    #[doc = "Bits 0:2 - PROM extra margin configuration"]
    #[inline(always)]
    pub fn prom_ema(&mut self) -> PROM_EMA_W {
        PROM_EMA_W { w: self }
    }
}
