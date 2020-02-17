#[doc = "Reader of register SysTick_CTRL"]
pub type R = crate::R<u32, super::SYSTICK_CTRL>;
#[doc = "Writer for register SysTick_CTRL"]
pub type W = crate::W<u32, super::SYSTICK_CTRL>;
#[doc = "Register SysTick_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTICK_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reads as 1 if SYSTICK counter has reached 0 since the last time the timer has reached 0. Clears to 0 on read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COUNTFLAG_A {
    #[doc = "0: SYSTICK counter has not reached zero since last read"]
    SYSTICK_COUNTFLAG_NOT_ZERO = 0,
    #[doc = "1: SYSTICK counter has reached zero since last read"]
    SYSTICK_COUNTFLAG_ZERO = 1,
}
impl From<COUNTFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: COUNTFLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COUNTFLAG`"]
pub type COUNTFLAG_R = crate::R<bool, COUNTFLAG_A>;
impl COUNTFLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COUNTFLAG_A {
        match self.bits {
            false => COUNTFLAG_A::SYSTICK_COUNTFLAG_NOT_ZERO,
            true => COUNTFLAG_A::SYSTICK_COUNTFLAG_ZERO,
        }
    }
    #[doc = "Checks if the value of the field is `SYSTICK_COUNTFLAG_NOT_ZERO`"]
    #[inline(always)]
    pub fn is_systick_countflag_not_zero(&self) -> bool {
        *self == COUNTFLAG_A::SYSTICK_COUNTFLAG_NOT_ZERO
    }
    #[doc = "Checks if the value of the field is `SYSTICK_COUNTFLAG_ZERO`"]
    #[inline(always)]
    pub fn is_systick_countflag_zero(&self) -> bool {
        *self == COUNTFLAG_A::SYSTICK_COUNTFLAG_ZERO
    }
}
#[doc = "SYSTICK timer clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSOURCE_A {
    #[doc = "0: Use external reference clock (STCLK)"]
    SYSTICK_CLKSOURCE_EXTREF_CLK = 0,
    #[doc = "1: Use the core clock"]
    SYSTICK_CLKSOURCE_CORE_CLK = 1,
}
impl From<CLKSOURCE_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSOURCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKSOURCE`"]
pub type CLKSOURCE_R = crate::R<bool, CLKSOURCE_A>;
impl CLKSOURCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSOURCE_A {
        match self.bits {
            false => CLKSOURCE_A::SYSTICK_CLKSOURCE_EXTREF_CLK,
            true => CLKSOURCE_A::SYSTICK_CLKSOURCE_CORE_CLK,
        }
    }
    #[doc = "Checks if the value of the field is `SYSTICK_CLKSOURCE_EXTREF_CLK`"]
    #[inline(always)]
    pub fn is_systick_clksource_extref_clk(&self) -> bool {
        *self == CLKSOURCE_A::SYSTICK_CLKSOURCE_EXTREF_CLK
    }
    #[doc = "Checks if the value of the field is `SYSTICK_CLKSOURCE_CORE_CLK`"]
    #[inline(always)]
    pub fn is_systick_clksource_core_clk(&self) -> bool {
        *self == CLKSOURCE_A::SYSTICK_CLKSOURCE_CORE_CLK
    }
}
#[doc = "Write proxy for field `CLKSOURCE`"]
pub struct CLKSOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSOURCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSOURCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use external reference clock (STCLK)"]
    #[inline(always)]
    pub fn systick_clksource_extref_clk(self) -> &'a mut W {
        self.variant(CLKSOURCE_A::SYSTICK_CLKSOURCE_EXTREF_CLK)
    }
    #[doc = "Use the core clock"]
    #[inline(always)]
    pub fn systick_clksource_core_clk(self) -> &'a mut W {
        self.variant(CLKSOURCE_A::SYSTICK_CLKSOURCE_CORE_CLK)
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
#[doc = "SYSTICK timer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICKINT_A {
    #[doc = "0: Disable interrupt generation when SYSTICK timer reaches 0"]
    SYSTICK_TICKINT_DISABLE = 0,
    #[doc = "1: Enable interrupt generation when SYSTICK timer reaches 0"]
    SYSTICK_TICKINT_ENABLE = 1,
}
impl From<TICKINT_A> for bool {
    #[inline(always)]
    fn from(variant: TICKINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TICKINT`"]
pub type TICKINT_R = crate::R<bool, TICKINT_A>;
impl TICKINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TICKINT_A {
        match self.bits {
            false => TICKINT_A::SYSTICK_TICKINT_DISABLE,
            true => TICKINT_A::SYSTICK_TICKINT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `SYSTICK_TICKINT_DISABLE`"]
    #[inline(always)]
    pub fn is_systick_tickint_disable(&self) -> bool {
        *self == TICKINT_A::SYSTICK_TICKINT_DISABLE
    }
    #[doc = "Checks if the value of the field is `SYSTICK_TICKINT_ENABLE`"]
    #[inline(always)]
    pub fn is_systick_tickint_enable(&self) -> bool {
        *self == TICKINT_A::SYSTICK_TICKINT_ENABLE
    }
}
#[doc = "Write proxy for field `TICKINT`"]
pub struct TICKINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TICKINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TICKINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable interrupt generation when SYSTICK timer reaches 0"]
    #[inline(always)]
    pub fn systick_tickint_disable(self) -> &'a mut W {
        self.variant(TICKINT_A::SYSTICK_TICKINT_DISABLE)
    }
    #[doc = "Enable interrupt generation when SYSTICK timer reaches 0"]
    #[inline(always)]
    pub fn systick_tickint_enable(self) -> &'a mut W {
        self.variant(TICKINT_A::SYSTICK_TICKINT_ENABLE)
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
#[doc = "SYSTICK timer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Disable SYSTICK timer"]
    SYSTICK_DISABLE = 0,
    #[doc = "1: Enable SYSTICK Timer"]
    SYSTICK_ENABLE = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::SYSTICK_DISABLE,
            true => ENABLE_A::SYSTICK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `SYSTICK_DISABLE`"]
    #[inline(always)]
    pub fn is_systick_disable(&self) -> bool {
        *self == ENABLE_A::SYSTICK_DISABLE
    }
    #[doc = "Checks if the value of the field is `SYSTICK_ENABLE`"]
    #[inline(always)]
    pub fn is_systick_enable(&self) -> bool {
        *self == ENABLE_A::SYSTICK_ENABLE
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable SYSTICK timer"]
    #[inline(always)]
    pub fn systick_disable(self) -> &'a mut W {
        self.variant(ENABLE_A::SYSTICK_DISABLE)
    }
    #[doc = "Enable SYSTICK Timer"]
    #[inline(always)]
    pub fn systick_enable(self) -> &'a mut W {
        self.variant(ENABLE_A::SYSTICK_ENABLE)
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
    #[doc = "Bit 16 - Reads as 1 if SYSTICK counter has reached 0 since the last time the timer has reached 0. Clears to 0 on read."]
    #[inline(always)]
    pub fn countflag(&self) -> COUNTFLAG_R {
        COUNTFLAG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SYSTICK timer clock source"]
    #[inline(always)]
    pub fn clksource(&self) -> CLKSOURCE_R {
        CLKSOURCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SYSTICK timer interrupt enable"]
    #[inline(always)]
    pub fn tickint(&self) -> TICKINT_R {
        TICKINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SYSTICK timer enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SYSTICK timer clock source"]
    #[inline(always)]
    pub fn clksource(&mut self) -> CLKSOURCE_W {
        CLKSOURCE_W { w: self }
    }
    #[doc = "Bit 1 - SYSTICK timer interrupt enable"]
    #[inline(always)]
    pub fn tickint(&mut self) -> TICKINT_W {
        TICKINT_W { w: self }
    }
    #[doc = "Bit 0 - SYSTICK timer enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
