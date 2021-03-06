#[doc = "Reader of register SPI0_STATUS"]
pub type R = crate::R<u32, super::SPI0_STATUS>;
#[doc = "Writer for register SPI0_STATUS"]
pub type W = crate::W<u32, super::SPI0_STATUS>;
#[doc = "Register SPI0_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI0_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Indicate that the transmission of the data is completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_TRANSMIT_STATUS_A {
    #[doc = "0: SPI data transmit flag not set"]
    SPI0_TRANSMIT_FALSE = 0,
    #[doc = "1: SPI transmit data sent"]
    SPI0_TRANSMIT_TRUE = 1,
}
impl From<SPI0_TRANSMIT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_TRANSMIT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0_TRANSMIT_STATUS`"]
pub type SPI0_TRANSMIT_STATUS_R = crate::R<bool, SPI0_TRANSMIT_STATUS_A>;
impl SPI0_TRANSMIT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_TRANSMIT_STATUS_A {
        match self.bits {
            false => SPI0_TRANSMIT_STATUS_A::SPI0_TRANSMIT_FALSE,
            true => SPI0_TRANSMIT_STATUS_A::SPI0_TRANSMIT_TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `SPI0_TRANSMIT_FALSE`"]
    #[inline(always)]
    pub fn is_spi0_transmit_false(&self) -> bool {
        *self == SPI0_TRANSMIT_STATUS_A::SPI0_TRANSMIT_FALSE
    }
    #[doc = "Checks if the value of the field is `SPI0_TRANSMIT_TRUE`"]
    #[inline(always)]
    pub fn is_spi0_transmit_true(&self) -> bool {
        *self == SPI0_TRANSMIT_STATUS_A::SPI0_TRANSMIT_TRUE
    }
}
#[doc = "Write proxy for field `SPI0_TRANSMIT_STATUS`"]
pub struct SPI0_TRANSMIT_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_TRANSMIT_STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_TRANSMIT_STATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SPI data transmit flag not set"]
    #[inline(always)]
    pub fn spi0_transmit_false(self) -> &'a mut W {
        self.variant(SPI0_TRANSMIT_STATUS_A::SPI0_TRANSMIT_FALSE)
    }
    #[doc = "SPI transmit data sent"]
    #[inline(always)]
    pub fn spi0_transmit_true(self) -> &'a mut W {
        self.variant(SPI0_TRANSMIT_STATUS_A::SPI0_TRANSMIT_TRUE)
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
#[doc = "Indicate that new data has been received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_RECEIVE_STATUS_A {
    #[doc = "0: SPI data receive flag not set"]
    SPI0_RECEIVE_FALSE = 0,
    #[doc = "1: SPI data received"]
    SPI0_RECEIVE_TRUE = 1,
}
impl From<SPI0_RECEIVE_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_RECEIVE_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0_RECEIVE_STATUS`"]
pub type SPI0_RECEIVE_STATUS_R = crate::R<bool, SPI0_RECEIVE_STATUS_A>;
impl SPI0_RECEIVE_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_RECEIVE_STATUS_A {
        match self.bits {
            false => SPI0_RECEIVE_STATUS_A::SPI0_RECEIVE_FALSE,
            true => SPI0_RECEIVE_STATUS_A::SPI0_RECEIVE_TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `SPI0_RECEIVE_FALSE`"]
    #[inline(always)]
    pub fn is_spi0_receive_false(&self) -> bool {
        *self == SPI0_RECEIVE_STATUS_A::SPI0_RECEIVE_FALSE
    }
    #[doc = "Checks if the value of the field is `SPI0_RECEIVE_TRUE`"]
    #[inline(always)]
    pub fn is_spi0_receive_true(&self) -> bool {
        *self == SPI0_RECEIVE_STATUS_A::SPI0_RECEIVE_TRUE
    }
}
#[doc = "Write proxy for field `SPI0_RECEIVE_STATUS`"]
pub struct SPI0_RECEIVE_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_RECEIVE_STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_RECEIVE_STATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SPI data receive flag not set"]
    #[inline(always)]
    pub fn spi0_receive_false(self) -> &'a mut W {
        self.variant(SPI0_RECEIVE_STATUS_A::SPI0_RECEIVE_FALSE)
    }
    #[doc = "SPI data received"]
    #[inline(always)]
    pub fn spi0_receive_true(self) -> &'a mut W {
        self.variant(SPI0_RECEIVE_STATUS_A::SPI0_RECEIVE_TRUE)
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
#[doc = "Indicate that an overrun has occurred when receiving data on the SPI interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_OVERRUN_STATUS_A {
    #[doc = "0: No SPI input overrun detected"]
    SPI0_OVERRUN_FALSE = 0,
    #[doc = "1: SPI input overrun detected"]
    SPI0_OVERRUN_TRUE = 1,
}
impl From<SPI0_OVERRUN_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_OVERRUN_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0_OVERRUN_STATUS`"]
pub type SPI0_OVERRUN_STATUS_R = crate::R<bool, SPI0_OVERRUN_STATUS_A>;
impl SPI0_OVERRUN_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_OVERRUN_STATUS_A {
        match self.bits {
            false => SPI0_OVERRUN_STATUS_A::SPI0_OVERRUN_FALSE,
            true => SPI0_OVERRUN_STATUS_A::SPI0_OVERRUN_TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `SPI0_OVERRUN_FALSE`"]
    #[inline(always)]
    pub fn is_spi0_overrun_false(&self) -> bool {
        *self == SPI0_OVERRUN_STATUS_A::SPI0_OVERRUN_FALSE
    }
    #[doc = "Checks if the value of the field is `SPI0_OVERRUN_TRUE`"]
    #[inline(always)]
    pub fn is_spi0_overrun_true(&self) -> bool {
        *self == SPI0_OVERRUN_STATUS_A::SPI0_OVERRUN_TRUE
    }
}
#[doc = "Write proxy for field `SPI0_OVERRUN_STATUS`"]
pub struct SPI0_OVERRUN_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_OVERRUN_STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_OVERRUN_STATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No SPI input overrun detected"]
    #[inline(always)]
    pub fn spi0_overrun_false(self) -> &'a mut W {
        self.variant(SPI0_OVERRUN_STATUS_A::SPI0_OVERRUN_FALSE)
    }
    #[doc = "SPI input overrun detected"]
    #[inline(always)]
    pub fn spi0_overrun_true(self) -> &'a mut W {
        self.variant(SPI0_OVERRUN_STATUS_A::SPI0_OVERRUN_TRUE)
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
#[doc = "Indicate that an underrun has occurred when transmitting data on the SPI interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_UNDERRUN_STATUS_A {
    #[doc = "0: No SPI input underrun detected"]
    SPI0_UNDERRUN_FALSE = 0,
    #[doc = "1: SPI input underrun detected"]
    SPI0_UNDERRUN_TRUE = 1,
}
impl From<SPI0_UNDERRUN_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_UNDERRUN_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0_UNDERRUN_STATUS`"]
pub type SPI0_UNDERRUN_STATUS_R = crate::R<bool, SPI0_UNDERRUN_STATUS_A>;
impl SPI0_UNDERRUN_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_UNDERRUN_STATUS_A {
        match self.bits {
            false => SPI0_UNDERRUN_STATUS_A::SPI0_UNDERRUN_FALSE,
            true => SPI0_UNDERRUN_STATUS_A::SPI0_UNDERRUN_TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `SPI0_UNDERRUN_FALSE`"]
    #[inline(always)]
    pub fn is_spi0_underrun_false(&self) -> bool {
        *self == SPI0_UNDERRUN_STATUS_A::SPI0_UNDERRUN_FALSE
    }
    #[doc = "Checks if the value of the field is `SPI0_UNDERRUN_TRUE`"]
    #[inline(always)]
    pub fn is_spi0_underrun_true(&self) -> bool {
        *self == SPI0_UNDERRUN_STATUS_A::SPI0_UNDERRUN_TRUE
    }
}
#[doc = "Write proxy for field `SPI0_UNDERRUN_STATUS`"]
pub struct SPI0_UNDERRUN_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_UNDERRUN_STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_UNDERRUN_STATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No SPI input underrun detected"]
    #[inline(always)]
    pub fn spi0_underrun_false(self) -> &'a mut W {
        self.variant(SPI0_UNDERRUN_STATUS_A::SPI0_UNDERRUN_FALSE)
    }
    #[doc = "SPI input underrun detected"]
    #[inline(always)]
    pub fn spi0_underrun_true(self) -> &'a mut W {
        self.variant(SPI0_UNDERRUN_STATUS_A::SPI0_UNDERRUN_TRUE)
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
    #[doc = "Bit 3 - Indicate that the transmission of the data is completed"]
    #[inline(always)]
    pub fn spi0_transmit_status(&self) -> SPI0_TRANSMIT_STATUS_R {
        SPI0_TRANSMIT_STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicate that new data has been received"]
    #[inline(always)]
    pub fn spi0_receive_status(&self) -> SPI0_RECEIVE_STATUS_R {
        SPI0_RECEIVE_STATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicate that an overrun has occurred when receiving data on the SPI interface"]
    #[inline(always)]
    pub fn spi0_overrun_status(&self) -> SPI0_OVERRUN_STATUS_R {
        SPI0_OVERRUN_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indicate that an underrun has occurred when transmitting data on the SPI interface"]
    #[inline(always)]
    pub fn spi0_underrun_status(&self) -> SPI0_UNDERRUN_STATUS_R {
        SPI0_UNDERRUN_STATUS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Indicate that the transmission of the data is completed"]
    #[inline(always)]
    pub fn spi0_transmit_status(&mut self) -> SPI0_TRANSMIT_STATUS_W {
        SPI0_TRANSMIT_STATUS_W { w: self }
    }
    #[doc = "Bit 2 - Indicate that new data has been received"]
    #[inline(always)]
    pub fn spi0_receive_status(&mut self) -> SPI0_RECEIVE_STATUS_W {
        SPI0_RECEIVE_STATUS_W { w: self }
    }
    #[doc = "Bit 1 - Indicate that an overrun has occurred when receiving data on the SPI interface"]
    #[inline(always)]
    pub fn spi0_overrun_status(&mut self) -> SPI0_OVERRUN_STATUS_W {
        SPI0_OVERRUN_STATUS_W { w: self }
    }
    #[doc = "Bit 0 - Indicate that an underrun has occurred when transmitting data on the SPI interface"]
    #[inline(always)]
    pub fn spi0_underrun_status(&mut self) -> SPI0_UNDERRUN_STATUS_W {
        SPI0_UNDERRUN_STATUS_W { w: self }
    }
}
