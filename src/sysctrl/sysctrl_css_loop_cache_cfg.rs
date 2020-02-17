#[doc = "Reader of register SYSCTRL_CSS_LOOP_CACHE_CFG"]
pub type R = crate::R<u32, super::SYSCTRL_CSS_LOOP_CACHE_CFG>;
#[doc = "Writer for register SYSCTRL_CSS_LOOP_CACHE_CFG"]
pub type W = crate::W<u32, super::SYSCTRL_CSS_LOOP_CACHE_CFG>;
#[doc = "Register SYSCTRL_CSS_LOOP_CACHE_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCTRL_CSS_LOOP_CACHE_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CSS loop cache enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSS_LOOP_CACHE_ENABLE_A {
    #[doc = "0: CSS loop cache disabled"]
    CSS_LOOP_CACHE_DISABLE = 0,
    #[doc = "1: CSS loop cache enabled"]
    CSS_LOOP_CACHE_ENABLE = 1,
}
impl From<CSS_LOOP_CACHE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CSS_LOOP_CACHE_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSS_LOOP_CACHE_ENABLE`"]
pub type CSS_LOOP_CACHE_ENABLE_R = crate::R<bool, CSS_LOOP_CACHE_ENABLE_A>;
impl CSS_LOOP_CACHE_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSS_LOOP_CACHE_ENABLE_A {
        match self.bits {
            false => CSS_LOOP_CACHE_ENABLE_A::CSS_LOOP_CACHE_DISABLE,
            true => CSS_LOOP_CACHE_ENABLE_A::CSS_LOOP_CACHE_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CSS_LOOP_CACHE_DISABLE`"]
    #[inline(always)]
    pub fn is_css_loop_cache_disable(&self) -> bool {
        *self == CSS_LOOP_CACHE_ENABLE_A::CSS_LOOP_CACHE_DISABLE
    }
    #[doc = "Checks if the value of the field is `CSS_LOOP_CACHE_ENABLE`"]
    #[inline(always)]
    pub fn is_css_loop_cache_enable(&self) -> bool {
        *self == CSS_LOOP_CACHE_ENABLE_A::CSS_LOOP_CACHE_ENABLE
    }
}
#[doc = "Write proxy for field `CSS_LOOP_CACHE_ENABLE`"]
pub struct CSS_LOOP_CACHE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSS_LOOP_CACHE_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSS_LOOP_CACHE_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CSS loop cache disabled"]
    #[inline(always)]
    pub fn css_loop_cache_disable(self) -> &'a mut W {
        self.variant(CSS_LOOP_CACHE_ENABLE_A::CSS_LOOP_CACHE_DISABLE)
    }
    #[doc = "CSS loop cache enabled"]
    #[inline(always)]
    pub fn css_loop_cache_enable(self) -> &'a mut W {
        self.variant(CSS_LOOP_CACHE_ENABLE_A::CSS_LOOP_CACHE_ENABLE)
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
    #[doc = "Bit 0 - CSS loop cache enable"]
    #[inline(always)]
    pub fn css_loop_cache_enable(&self) -> CSS_LOOP_CACHE_ENABLE_R {
        CSS_LOOP_CACHE_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSS loop cache enable"]
    #[inline(always)]
    pub fn css_loop_cache_enable(&mut self) -> CSS_LOOP_CACHE_ENABLE_W {
        CSS_LOOP_CACHE_ENABLE_W { w: self }
    }
}
