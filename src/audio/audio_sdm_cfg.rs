#[doc = "Reader of register AUDIO_SDM_CFG"]
pub type R = crate::R<u32, super::AUDIO_SDM_CFG>;
#[doc = "Writer for register AUDIO_SDM_CFG"]
pub type W = crate::W<u32, super::AUDIO_SDM_CFG>;
#[doc = "Register AUDIO_SDM_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::AUDIO_SDM_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sigma-Delta modulator internal configuration for test purposes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SDM_CFG_A {
    #[doc = "327698: Sigma-Delta modulator configuration for normal operation"]
    SDM_CFG_NORMAL = 327698,
}
impl From<SDM_CFG_A> for u32 {
    #[inline(always)]
    fn from(variant: SDM_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SDM_CFG`"]
pub type SDM_CFG_R = crate::R<u32, SDM_CFG_A>;
impl SDM_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, SDM_CFG_A> {
        use crate::Variant::*;
        match self.bits {
            327698 => Val(SDM_CFG_A::SDM_CFG_NORMAL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SDM_CFG_NORMAL`"]
    #[inline(always)]
    pub fn is_sdm_cfg_normal(&self) -> bool {
        *self == SDM_CFG_A::SDM_CFG_NORMAL
    }
}
#[doc = "Write proxy for field `SDM_CFG`"]
pub struct SDM_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SDM_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDM_CFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Sigma-Delta modulator configuration for normal operation"]
    #[inline(always)]
    pub fn sdm_cfg_normal(self) -> &'a mut W {
        self.variant(SDM_CFG_A::SDM_CFG_NORMAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Sigma-Delta modulator internal configuration for test purposes"]
    #[inline(always)]
    pub fn sdm_cfg(&self) -> SDM_CFG_R {
        SDM_CFG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sigma-Delta modulator internal configuration for test purposes"]
    #[inline(always)]
    pub fn sdm_cfg(&mut self) -> SDM_CFG_W {
        SDM_CFG_W { w: self }
    }
}
