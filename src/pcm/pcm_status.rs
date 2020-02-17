#[doc = "Reader of register PCM_STATUS"]
pub type R = crate::R<u32, super::PCM_STATUS>;
#[doc = "Writer for register PCM_STATUS"]
pub type W = crate::W<u32, super::PCM_STATUS>;
#[doc = "Register PCM_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::PCM_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Indicate that PCM data has been sent\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRANSMIT_STATUS_A {
    #[doc = "0: PCM data transmit flag not set"]
    PCM_TRANSMIT_FALSE = 0,
    #[doc = "1: PCM transmit data sent"]
    PCM_TRANSMIT_TRUE = 1,
}
impl From<TRANSMIT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: TRANSMIT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRANSMIT_STATUS`"]
pub type TRANSMIT_STATUS_R = crate::R<bool, TRANSMIT_STATUS_A>;
impl TRANSMIT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRANSMIT_STATUS_A {
        match self.bits {
            false => TRANSMIT_STATUS_A::PCM_TRANSMIT_FALSE,
            true => TRANSMIT_STATUS_A::PCM_TRANSMIT_TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `PCM_TRANSMIT_FALSE`"]
    #[inline(always)]
    pub fn is_pcm_transmit_false(&self) -> bool {
        *self == TRANSMIT_STATUS_A::PCM_TRANSMIT_FALSE
    }
    #[doc = "Checks if the value of the field is `PCM_TRANSMIT_TRUE`"]
    #[inline(always)]
    pub fn is_pcm_transmit_true(&self) -> bool {
        *self == TRANSMIT_STATUS_A::PCM_TRANSMIT_TRUE
    }
}
#[doc = "Write proxy for field `TRANSMIT_STATUS`"]
pub struct TRANSMIT_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSMIT_STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRANSMIT_STATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PCM data transmit flag not set"]
    #[inline(always)]
    pub fn pcm_transmit_false(self) -> &'a mut W {
        self.variant(TRANSMIT_STATUS_A::PCM_TRANSMIT_FALSE)
    }
    #[doc = "PCM transmit data sent"]
    #[inline(always)]
    pub fn pcm_transmit_true(self) -> &'a mut W {
        self.variant(TRANSMIT_STATUS_A::PCM_TRANSMIT_TRUE)
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
#[doc = "Indicate that PCM data has been received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE_STATUS_A {
    #[doc = "0: PCM data receive flag not set"]
    PCM_RECEIVE_FALSE = 0,
    #[doc = "1: PCM data received"]
    PCM_RECEIVE_TRUE = 1,
}
impl From<RECEIVE_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RECEIVE_STATUS`"]
pub type RECEIVE_STATUS_R = crate::R<bool, RECEIVE_STATUS_A>;
impl RECEIVE_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE_STATUS_A {
        match self.bits {
            false => RECEIVE_STATUS_A::PCM_RECEIVE_FALSE,
            true => RECEIVE_STATUS_A::PCM_RECEIVE_TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `PCM_RECEIVE_FALSE`"]
    #[inline(always)]
    pub fn is_pcm_receive_false(&self) -> bool {
        *self == RECEIVE_STATUS_A::PCM_RECEIVE_FALSE
    }
    #[doc = "Checks if the value of the field is `PCM_RECEIVE_TRUE`"]
    #[inline(always)]
    pub fn is_pcm_receive_true(&self) -> bool {
        *self == RECEIVE_STATUS_A::PCM_RECEIVE_TRUE
    }
}
#[doc = "Write proxy for field `RECEIVE_STATUS`"]
pub struct RECEIVE_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE_STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE_STATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PCM data receive flag not set"]
    #[inline(always)]
    pub fn pcm_receive_false(self) -> &'a mut W {
        self.variant(RECEIVE_STATUS_A::PCM_RECEIVE_FALSE)
    }
    #[doc = "PCM data received"]
    #[inline(always)]
    pub fn pcm_receive_true(self) -> &'a mut W {
        self.variant(RECEIVE_STATUS_A::PCM_RECEIVE_TRUE)
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
#[doc = "Indicate that an overrun has occurred when receiving data on the PCM interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERRUN_STATUS_A {
    #[doc = "0: No PCM input overrun detected"]
    PCM_OVERRUN_FALSE = 0,
    #[doc = "1: PCM input overrun detected"]
    PCM_OVERRUN_TRUE = 1,
}
impl From<OVERRUN_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: OVERRUN_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVERRUN_STATUS`"]
pub type OVERRUN_STATUS_R = crate::R<bool, OVERRUN_STATUS_A>;
impl OVERRUN_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERRUN_STATUS_A {
        match self.bits {
            false => OVERRUN_STATUS_A::PCM_OVERRUN_FALSE,
            true => OVERRUN_STATUS_A::PCM_OVERRUN_TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `PCM_OVERRUN_FALSE`"]
    #[inline(always)]
    pub fn is_pcm_overrun_false(&self) -> bool {
        *self == OVERRUN_STATUS_A::PCM_OVERRUN_FALSE
    }
    #[doc = "Checks if the value of the field is `PCM_OVERRUN_TRUE`"]
    #[inline(always)]
    pub fn is_pcm_overrun_true(&self) -> bool {
        *self == OVERRUN_STATUS_A::PCM_OVERRUN_TRUE
    }
}
#[doc = "Write proxy for field `OVERRUN_STATUS`"]
pub struct OVERRUN_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRUN_STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERRUN_STATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No PCM input overrun detected"]
    #[inline(always)]
    pub fn pcm_overrun_false(self) -> &'a mut W {
        self.variant(OVERRUN_STATUS_A::PCM_OVERRUN_FALSE)
    }
    #[doc = "PCM input overrun detected"]
    #[inline(always)]
    pub fn pcm_overrun_true(self) -> &'a mut W {
        self.variant(OVERRUN_STATUS_A::PCM_OVERRUN_TRUE)
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
#[doc = "Indicate that an underrun has occurred when transmitting data on the PCM interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNDERRUN_STATUS_A {
    #[doc = "0: No PCM output underrun detected"]
    PCM_UNDERRUN_FALSE = 0,
    #[doc = "1: PCM output underrun detected"]
    PCM_UNDERRUN_TRUE = 1,
}
impl From<UNDERRUN_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: UNDERRUN_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UNDERRUN_STATUS`"]
pub type UNDERRUN_STATUS_R = crate::R<bool, UNDERRUN_STATUS_A>;
impl UNDERRUN_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNDERRUN_STATUS_A {
        match self.bits {
            false => UNDERRUN_STATUS_A::PCM_UNDERRUN_FALSE,
            true => UNDERRUN_STATUS_A::PCM_UNDERRUN_TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `PCM_UNDERRUN_FALSE`"]
    #[inline(always)]
    pub fn is_pcm_underrun_false(&self) -> bool {
        *self == UNDERRUN_STATUS_A::PCM_UNDERRUN_FALSE
    }
    #[doc = "Checks if the value of the field is `PCM_UNDERRUN_TRUE`"]
    #[inline(always)]
    pub fn is_pcm_underrun_true(&self) -> bool {
        *self == UNDERRUN_STATUS_A::PCM_UNDERRUN_TRUE
    }
}
#[doc = "Write proxy for field `UNDERRUN_STATUS`"]
pub struct UNDERRUN_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERRUN_STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNDERRUN_STATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No PCM output underrun detected"]
    #[inline(always)]
    pub fn pcm_underrun_false(self) -> &'a mut W {
        self.variant(UNDERRUN_STATUS_A::PCM_UNDERRUN_FALSE)
    }
    #[doc = "PCM output underrun detected"]
    #[inline(always)]
    pub fn pcm_underrun_true(self) -> &'a mut W {
        self.variant(UNDERRUN_STATUS_A::PCM_UNDERRUN_TRUE)
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
    #[doc = "Bit 3 - Indicate that PCM data has been sent"]
    #[inline(always)]
    pub fn transmit_status(&self) -> TRANSMIT_STATUS_R {
        TRANSMIT_STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicate that PCM data has been received"]
    #[inline(always)]
    pub fn receive_status(&self) -> RECEIVE_STATUS_R {
        RECEIVE_STATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicate that an overrun has occurred when receiving data on the PCM interface"]
    #[inline(always)]
    pub fn overrun_status(&self) -> OVERRUN_STATUS_R {
        OVERRUN_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indicate that an underrun has occurred when transmitting data on the PCM interface"]
    #[inline(always)]
    pub fn underrun_status(&self) -> UNDERRUN_STATUS_R {
        UNDERRUN_STATUS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Indicate that PCM data has been sent"]
    #[inline(always)]
    pub fn transmit_status(&mut self) -> TRANSMIT_STATUS_W {
        TRANSMIT_STATUS_W { w: self }
    }
    #[doc = "Bit 2 - Indicate that PCM data has been received"]
    #[inline(always)]
    pub fn receive_status(&mut self) -> RECEIVE_STATUS_W {
        RECEIVE_STATUS_W { w: self }
    }
    #[doc = "Bit 1 - Indicate that an overrun has occurred when receiving data on the PCM interface"]
    #[inline(always)]
    pub fn overrun_status(&mut self) -> OVERRUN_STATUS_W {
        OVERRUN_STATUS_W { w: self }
    }
    #[doc = "Bit 0 - Indicate that an underrun has occurred when transmitting data on the PCM interface"]
    #[inline(always)]
    pub fn underrun_status(&mut self) -> UNDERRUN_STATUS_W {
        UNDERRUN_STATUS_W { w: self }
    }
}
