#[doc = "Reader of register RF_CENTER_FREQ"]
pub type R = crate::R<u32, super::RF_CENTER_FREQ>;
#[doc = "Writer for register RF_CENTER_FREQ"]
pub type W = crate::W<u32, super::RF_CENTER_FREQ>;
#[doc = "Register RF_CENTER_FREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_CENTER_FREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "If set to 1, automatically adapt frequency between Tx and Rx.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CENTER_FREQ_ADAPT_CFREQ_A {
    #[doc = "0: `0`"]
    CENTER_FREQ_ADAPT_CFREQ_DEFAULT = 0,
}
impl From<CENTER_FREQ_ADAPT_CFREQ_A> for bool {
    #[inline(always)]
    fn from(variant: CENTER_FREQ_ADAPT_CFREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CENTER_FREQ_ADAPT_CFREQ`"]
pub type CENTER_FREQ_ADAPT_CFREQ_R = crate::R<bool, CENTER_FREQ_ADAPT_CFREQ_A>;
impl CENTER_FREQ_ADAPT_CFREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CENTER_FREQ_ADAPT_CFREQ_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CENTER_FREQ_ADAPT_CFREQ_A::CENTER_FREQ_ADAPT_CFREQ_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CENTER_FREQ_ADAPT_CFREQ_DEFAULT`"]
    #[inline(always)]
    pub fn is_center_freq_adapt_cfreq_default(&self) -> bool {
        *self == CENTER_FREQ_ADAPT_CFREQ_A::CENTER_FREQ_ADAPT_CFREQ_DEFAULT
    }
}
#[doc = "Write proxy for field `CENTER_FREQ_ADAPT_CFREQ`"]
pub struct CENTER_FREQ_ADAPT_CFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CENTER_FREQ_ADAPT_CFREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CENTER_FREQ_ADAPT_CFREQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn center_freq_adapt_cfreq_default(self) -> &'a mut W {
        self.variant(CENTER_FREQ_ADAPT_CFREQ_A::CENTER_FREQ_ADAPT_CFREQ_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "If set to 1, the ratio of the pll reference between Tx and Rx is 5 instead of 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CENTER_FREQ_RX_DIV_5_N6_A {
    #[doc = "0: `0`"]
    CENTER_FREQ_RX_DIV_5_N6_DEFAULT = 0,
}
impl From<CENTER_FREQ_RX_DIV_5_N6_A> for bool {
    #[inline(always)]
    fn from(variant: CENTER_FREQ_RX_DIV_5_N6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CENTER_FREQ_RX_DIV_5_N6`"]
pub type CENTER_FREQ_RX_DIV_5_N6_R = crate::R<bool, CENTER_FREQ_RX_DIV_5_N6_A>;
impl CENTER_FREQ_RX_DIV_5_N6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CENTER_FREQ_RX_DIV_5_N6_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(CENTER_FREQ_RX_DIV_5_N6_A::CENTER_FREQ_RX_DIV_5_N6_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CENTER_FREQ_RX_DIV_5_N6_DEFAULT`"]
    #[inline(always)]
    pub fn is_center_freq_rx_div_5_n6_default(&self) -> bool {
        *self == CENTER_FREQ_RX_DIV_5_N6_A::CENTER_FREQ_RX_DIV_5_N6_DEFAULT
    }
}
#[doc = "Write proxy for field `CENTER_FREQ_RX_DIV_5_N6`"]
pub struct CENTER_FREQ_RX_DIV_5_N6_W<'a> {
    w: &'a mut W,
}
impl<'a> CENTER_FREQ_RX_DIV_5_N6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CENTER_FREQ_RX_DIV_5_N6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn center_freq_rx_div_5_n6_default(self) -> &'a mut W {
        self.variant(CENTER_FREQ_RX_DIV_5_N6_A::CENTER_FREQ_RX_DIV_5_N6_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Set the center frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CENTER_FREQ_CENTER_FREQUENCY_A {
    #[doc = "0: `0`"]
    CENTER_FREQ_CENTER_FREQUENCY_DEFAULT = 0,
}
impl From<CENTER_FREQ_CENTER_FREQUENCY_A> for u32 {
    #[inline(always)]
    fn from(variant: CENTER_FREQ_CENTER_FREQUENCY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CENTER_FREQ_CENTER_FREQUENCY`"]
pub type CENTER_FREQ_CENTER_FREQUENCY_R = crate::R<u32, CENTER_FREQ_CENTER_FREQUENCY_A>;
impl CENTER_FREQ_CENTER_FREQUENCY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CENTER_FREQ_CENTER_FREQUENCY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CENTER_FREQ_CENTER_FREQUENCY_A::CENTER_FREQ_CENTER_FREQUENCY_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CENTER_FREQ_CENTER_FREQUENCY_DEFAULT`"]
    #[inline(always)]
    pub fn is_center_freq_center_frequency_default(&self) -> bool {
        *self == CENTER_FREQ_CENTER_FREQUENCY_A::CENTER_FREQ_CENTER_FREQUENCY_DEFAULT
    }
}
#[doc = "Write proxy for field `CENTER_FREQ_CENTER_FREQUENCY`"]
pub struct CENTER_FREQ_CENTER_FREQUENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> CENTER_FREQ_CENTER_FREQUENCY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CENTER_FREQ_CENTER_FREQUENCY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn center_freq_center_frequency_default(self) -> &'a mut W {
        self.variant(CENTER_FREQ_CENTER_FREQUENCY_A::CENTER_FREQ_CENTER_FREQUENCY_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - If set to 1, automatically adapt frequency between Tx and Rx."]
    #[inline(always)]
    pub fn center_freq_adapt_cfreq(&self) -> CENTER_FREQ_ADAPT_CFREQ_R {
        CENTER_FREQ_ADAPT_CFREQ_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - If set to 1, the ratio of the pll reference between Tx and Rx is 5 instead of 6."]
    #[inline(always)]
    pub fn center_freq_rx_div_5_n6(&self) -> CENTER_FREQ_RX_DIV_5_N6_R {
        CENTER_FREQ_RX_DIV_5_N6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 0:29 - Set the center frequency"]
    #[inline(always)]
    pub fn center_freq_center_frequency(&self) -> CENTER_FREQ_CENTER_FREQUENCY_R {
        CENTER_FREQ_CENTER_FREQUENCY_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - If set to 1, automatically adapt frequency between Tx and Rx."]
    #[inline(always)]
    pub fn center_freq_adapt_cfreq(&mut self) -> CENTER_FREQ_ADAPT_CFREQ_W {
        CENTER_FREQ_ADAPT_CFREQ_W { w: self }
    }
    #[doc = "Bit 30 - If set to 1, the ratio of the pll reference between Tx and Rx is 5 instead of 6."]
    #[inline(always)]
    pub fn center_freq_rx_div_5_n6(&mut self) -> CENTER_FREQ_RX_DIV_5_N6_W {
        CENTER_FREQ_RX_DIV_5_N6_W { w: self }
    }
    #[doc = "Bits 0:29 - Set the center frequency"]
    #[inline(always)]
    pub fn center_freq_center_frequency(&mut self) -> CENTER_FREQ_CENTER_FREQUENCY_W {
        CENTER_FREQ_CENTER_FREQUENCY_W { w: self }
    }
}
