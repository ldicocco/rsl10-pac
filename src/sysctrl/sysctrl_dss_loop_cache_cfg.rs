#[doc = "Reader of register SYSCTRL_DSS_LOOP_CACHE_CFG"]
pub type R = crate::R<u32, super::SYSCTRL_DSS_LOOP_CACHE_CFG>;
#[doc = "Writer for register SYSCTRL_DSS_LOOP_CACHE_CFG"]
pub type W = crate::W<u32, super::SYSCTRL_DSS_LOOP_CACHE_CFG>;
#[doc = "Register SYSCTRL_DSS_LOOP_CACHE_CFG `reset()`'s with value 0x01"]
impl crate::ResetValue for super::SYSCTRL_DSS_LOOP_CACHE_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "DSS loop cache enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSS_LOOP_CACHE_ENABLE_A {
    #[doc = "0: DSS loop cache disabled"]
    DSS_LOOP_CACHE_DISABLE = 0,
    #[doc = "1: DSS loop cache enabled"]
    DSS_LOOP_CACHE_ENABLE = 1,
}
impl From<DSS_LOOP_CACHE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DSS_LOOP_CACHE_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSS_LOOP_CACHE_ENABLE`"]
pub type DSS_LOOP_CACHE_ENABLE_R = crate::R<bool, DSS_LOOP_CACHE_ENABLE_A>;
impl DSS_LOOP_CACHE_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSS_LOOP_CACHE_ENABLE_A {
        match self.bits {
            false => DSS_LOOP_CACHE_ENABLE_A::DSS_LOOP_CACHE_DISABLE,
            true => DSS_LOOP_CACHE_ENABLE_A::DSS_LOOP_CACHE_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSS_LOOP_CACHE_DISABLE`"]
    #[inline(always)]
    pub fn is_dss_loop_cache_disable(&self) -> bool {
        *self == DSS_LOOP_CACHE_ENABLE_A::DSS_LOOP_CACHE_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSS_LOOP_CACHE_ENABLE`"]
    #[inline(always)]
    pub fn is_dss_loop_cache_enable(&self) -> bool {
        *self == DSS_LOOP_CACHE_ENABLE_A::DSS_LOOP_CACHE_ENABLE
    }
}
#[doc = "Write proxy for field `DSS_LOOP_CACHE_ENABLE`"]
pub struct DSS_LOOP_CACHE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSS_LOOP_CACHE_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSS_LOOP_CACHE_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSS loop cache disabled"]
    #[inline(always)]
    pub fn dss_loop_cache_disable(self) -> &'a mut W {
        self.variant(DSS_LOOP_CACHE_ENABLE_A::DSS_LOOP_CACHE_DISABLE)
    }
    #[doc = "DSS loop cache enabled"]
    #[inline(always)]
    pub fn dss_loop_cache_enable(self) -> &'a mut W {
        self.variant(DSS_LOOP_CACHE_ENABLE_A::DSS_LOOP_CACHE_ENABLE)
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
    #[doc = "Bit 0 - DSS loop cache enable"]
    #[inline(always)]
    pub fn dss_loop_cache_enable(&self) -> DSS_LOOP_CACHE_ENABLE_R {
        DSS_LOOP_CACHE_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DSS loop cache enable"]
    #[inline(always)]
    pub fn dss_loop_cache_enable(&mut self) -> DSS_LOOP_CACHE_ENABLE_W {
        DSS_LOOP_CACHE_ENABLE_W { w: self }
    }
}
