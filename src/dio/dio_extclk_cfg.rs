#[doc = "Reader of register DIO_EXTCLK_CFG"]
pub type R = crate::R<u32, super::DIO_EXTCLK_CFG>;
#[doc = "Writer for register DIO_EXTCLK_CFG"]
pub type W = crate::W<u32, super::DIO_EXTCLK_CFG>;
#[doc = "Register DIO_EXTCLK_CFG `reset()`'s with value 0x01"]
impl crate::ResetValue for super::DIO_EXTCLK_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Low Pass Filter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPF_A {
    #[doc = "0: Disable low pass filter"]
    EXTCLK_LPF_DISABLE = 0,
    #[doc = "1: Enable low pass filter"]
    EXTCLK_LPF_ENABLE = 1,
}
impl From<LPF_A> for bool {
    #[inline(always)]
    fn from(variant: LPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPF`"]
pub type LPF_R = crate::R<bool, LPF_A>;
impl LPF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPF_A {
        match self.bits {
            false => LPF_A::EXTCLK_LPF_DISABLE,
            true => LPF_A::EXTCLK_LPF_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `EXTCLK_LPF_DISABLE`"]
    #[inline(always)]
    pub fn is_extclk_lpf_disable(&self) -> bool {
        *self == LPF_A::EXTCLK_LPF_DISABLE
    }
    #[doc = "Checks if the value of the field is `EXTCLK_LPF_ENABLE`"]
    #[inline(always)]
    pub fn is_extclk_lpf_enable(&self) -> bool {
        *self == LPF_A::EXTCLK_LPF_ENABLE
    }
}
#[doc = "Write proxy for field `LPF`"]
pub struct LPF_W<'a> {
    w: &'a mut W,
}
impl<'a> LPF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable low pass filter"]
    #[inline(always)]
    pub fn extclk_lpf_disable(self) -> &'a mut W {
        self.variant(LPF_A::EXTCLK_LPF_DISABLE)
    }
    #[doc = "Enable low pass filter"]
    #[inline(always)]
    pub fn extclk_lpf_enable(self) -> &'a mut W {
        self.variant(LPF_A::EXTCLK_LPF_ENABLE)
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
#[doc = "Pull Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PULL_CTRL_A {
    #[doc = "0: No pull selected"]
    EXTCLK_NO_PULL = 0,
    #[doc = "1: Weak pull-up selected"]
    EXTCLK_WEAK_PULL_UP = 1,
    #[doc = "2: Weak pull-down selected"]
    EXTCLK_WEAK_PULL_DOWN = 2,
    #[doc = "3: Strong pull-down selected"]
    EXTCLK_STRONG_PULL_UP = 3,
}
impl From<PULL_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: PULL_CTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PULL_CTRL`"]
pub type PULL_CTRL_R = crate::R<u8, PULL_CTRL_A>;
impl PULL_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PULL_CTRL_A {
        match self.bits {
            0 => PULL_CTRL_A::EXTCLK_NO_PULL,
            1 => PULL_CTRL_A::EXTCLK_WEAK_PULL_UP,
            2 => PULL_CTRL_A::EXTCLK_WEAK_PULL_DOWN,
            3 => PULL_CTRL_A::EXTCLK_STRONG_PULL_UP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTCLK_NO_PULL`"]
    #[inline(always)]
    pub fn is_extclk_no_pull(&self) -> bool {
        *self == PULL_CTRL_A::EXTCLK_NO_PULL
    }
    #[doc = "Checks if the value of the field is `EXTCLK_WEAK_PULL_UP`"]
    #[inline(always)]
    pub fn is_extclk_weak_pull_up(&self) -> bool {
        *self == PULL_CTRL_A::EXTCLK_WEAK_PULL_UP
    }
    #[doc = "Checks if the value of the field is `EXTCLK_WEAK_PULL_DOWN`"]
    #[inline(always)]
    pub fn is_extclk_weak_pull_down(&self) -> bool {
        *self == PULL_CTRL_A::EXTCLK_WEAK_PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `EXTCLK_STRONG_PULL_UP`"]
    #[inline(always)]
    pub fn is_extclk_strong_pull_up(&self) -> bool {
        *self == PULL_CTRL_A::EXTCLK_STRONG_PULL_UP
    }
}
#[doc = "Write proxy for field `PULL_CTRL`"]
pub struct PULL_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PULL_CTRL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No pull selected"]
    #[inline(always)]
    pub fn extclk_no_pull(self) -> &'a mut W {
        self.variant(PULL_CTRL_A::EXTCLK_NO_PULL)
    }
    #[doc = "Weak pull-up selected"]
    #[inline(always)]
    pub fn extclk_weak_pull_up(self) -> &'a mut W {
        self.variant(PULL_CTRL_A::EXTCLK_WEAK_PULL_UP)
    }
    #[doc = "Weak pull-down selected"]
    #[inline(always)]
    pub fn extclk_weak_pull_down(self) -> &'a mut W {
        self.variant(PULL_CTRL_A::EXTCLK_WEAK_PULL_DOWN)
    }
    #[doc = "Strong pull-down selected"]
    #[inline(always)]
    pub fn extclk_strong_pull_up(self) -> &'a mut W {
        self.variant(PULL_CTRL_A::EXTCLK_STRONG_PULL_UP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Low Pass Filter enable"]
    #[inline(always)]
    pub fn lpf(&self) -> LPF_R {
        LPF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - Pull Selection"]
    #[inline(always)]
    pub fn pull_ctrl(&self) -> PULL_CTRL_R {
        PULL_CTRL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Low Pass Filter enable"]
    #[inline(always)]
    pub fn lpf(&mut self) -> LPF_W {
        LPF_W { w: self }
    }
    #[doc = "Bits 0:1 - Pull Selection"]
    #[inline(always)]
    pub fn pull_ctrl(&mut self) -> PULL_CTRL_W {
        PULL_CTRL_W { w: self }
    }
}
