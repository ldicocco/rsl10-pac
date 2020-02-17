#[doc = "Reader of register UART_CFG"]
pub type R = crate::R<u32, super::UART_CFG>;
#[doc = "Writer for register UART_CFG"]
pub type W = crate::W<u32, super::UART_CFG>;
#[doc = "Register UART_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRESCALE`"]
pub type PRESCALE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PRESCALE`"]
pub struct PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | (((value as u32) & 0xffff) << 8);
        self.w
    }
}
#[doc = "Enable/disable a fixed prescaler by 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALE_ENABLE_A {
    #[doc = "0: Disable prescaling division by 12"]
    UART_PRESCALE_DISABLE = 0,
    #[doc = "1: Enable prescaling division by 12"]
    UART_PRESCALE_ENABLE = 1,
}
impl From<PRESCALE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PRESCALE_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRESCALE_ENABLE`"]
pub type PRESCALE_ENABLE_R = crate::R<bool, PRESCALE_ENABLE_A>;
impl PRESCALE_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCALE_ENABLE_A {
        match self.bits {
            false => PRESCALE_ENABLE_A::UART_PRESCALE_DISABLE,
            true => PRESCALE_ENABLE_A::UART_PRESCALE_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `UART_PRESCALE_DISABLE`"]
    #[inline(always)]
    pub fn is_uart_prescale_disable(&self) -> bool {
        *self == PRESCALE_ENABLE_A::UART_PRESCALE_DISABLE
    }
    #[doc = "Checks if the value of the field is `UART_PRESCALE_ENABLE`"]
    #[inline(always)]
    pub fn is_uart_prescale_enable(&self) -> bool {
        *self == PRESCALE_ENABLE_A::UART_PRESCALE_ENABLE
    }
}
#[doc = "Write proxy for field `PRESCALE_ENABLE`"]
pub struct PRESCALE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALE_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALE_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable prescaling division by 12"]
    #[inline(always)]
    pub fn uart_prescale_disable(self) -> &'a mut W {
        self.variant(PRESCALE_ENABLE_A::UART_PRESCALE_DISABLE)
    }
    #[doc = "Enable prescaling division by 12"]
    #[inline(always)]
    pub fn uart_prescale_enable(self) -> &'a mut W {
        self.variant(PRESCALE_ENABLE_A::UART_PRESCALE_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "DMA mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ENABLE_A {
    #[doc = "0: Disable the DMA mode"]
    UART_DMA_MODE_DISABLE = 0,
    #[doc = "1: Enable the DMA mode"]
    UART_DMA_MODE_ENABLE = 1,
}
impl From<DMA_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA_ENABLE`"]
pub type DMA_ENABLE_R = crate::R<bool, DMA_ENABLE_A>;
impl DMA_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_ENABLE_A {
        match self.bits {
            false => DMA_ENABLE_A::UART_DMA_MODE_DISABLE,
            true => DMA_ENABLE_A::UART_DMA_MODE_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `UART_DMA_MODE_DISABLE`"]
    #[inline(always)]
    pub fn is_uart_dma_mode_disable(&self) -> bool {
        *self == DMA_ENABLE_A::UART_DMA_MODE_DISABLE
    }
    #[doc = "Checks if the value of the field is `UART_DMA_MODE_ENABLE`"]
    #[inline(always)]
    pub fn is_uart_dma_mode_enable(&self) -> bool {
        *self == DMA_ENABLE_A::UART_DMA_MODE_ENABLE
    }
}
#[doc = "Write proxy for field `DMA_ENABLE`"]
pub struct DMA_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the DMA mode"]
    #[inline(always)]
    pub fn uart_dma_mode_disable(self) -> &'a mut W {
        self.variant(DMA_ENABLE_A::UART_DMA_MODE_DISABLE)
    }
    #[doc = "Enable the DMA mode"]
    #[inline(always)]
    pub fn uart_dma_mode_enable(self) -> &'a mut W {
        self.variant(DMA_ENABLE_A::UART_DMA_MODE_ENABLE)
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
#[doc = "Enable/disable the UART interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Disable the UART interface"]
    UART_DISABLE = 0,
    #[doc = "1: Enable the UART interface"]
    UART_ENABLE = 1,
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
            false => ENABLE_A::UART_DISABLE,
            true => ENABLE_A::UART_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `UART_DISABLE`"]
    #[inline(always)]
    pub fn is_uart_disable(&self) -> bool {
        *self == ENABLE_A::UART_DISABLE
    }
    #[doc = "Checks if the value of the field is `UART_ENABLE`"]
    #[inline(always)]
    pub fn is_uart_enable(&self) -> bool {
        *self == ENABLE_A::UART_ENABLE
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
    #[doc = "Disable the UART interface"]
    #[inline(always)]
    pub fn uart_disable(self) -> &'a mut W {
        self.variant(ENABLE_A::UART_DISABLE)
    }
    #[doc = "Enable the UART interface"]
    #[inline(always)]
    pub fn uart_enable(self) -> &'a mut W {
        self.variant(ENABLE_A::UART_ENABLE)
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
    #[doc = "Bits 8:23 - Prescaling multiplier in baud rate calculation"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bit 4 - Enable/disable a fixed prescaler by 12"]
    #[inline(always)]
    pub fn prescale_enable(&self) -> PRESCALE_ENABLE_R {
        PRESCALE_ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA mode enable"]
    #[inline(always)]
    pub fn dma_enable(&self) -> DMA_ENABLE_R {
        DMA_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable/disable the UART interface"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:23 - Prescaling multiplier in baud rate calculation"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W {
        PRESCALE_W { w: self }
    }
    #[doc = "Bit 4 - Enable/disable a fixed prescaler by 12"]
    #[inline(always)]
    pub fn prescale_enable(&mut self) -> PRESCALE_ENABLE_W {
        PRESCALE_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - DMA mode enable"]
    #[inline(always)]
    pub fn dma_enable(&mut self) -> DMA_ENABLE_W {
        DMA_ENABLE_W { w: self }
    }
    #[doc = "Bit 0 - Enable/disable the UART interface"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
