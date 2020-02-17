#[doc = "Reader of register SPI0_CTRL0"]
pub type R = crate::R<u32, super::SPI0_CTRL0>;
#[doc = "Writer for register SPI0_CTRL0"]
pub type W = crate::W<u32, super::SPI0_CTRL0>;
#[doc = "Register SPI0_CTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI0_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable/disable SPI overrun interrupts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_OVERRUN_INT_ENABLE_A {
    #[doc = "0: No interrupts are raised when an overrun occurs on the SPI interface"]
    SPI0_OVERRUN_INT_DISABLE = 0,
    #[doc = "1: An interrupt is raised when an overrun occurs on the SPI interface"]
    SPI0_OVERRUN_INT_ENABLE = 1,
}
impl From<SPI0_OVERRUN_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_OVERRUN_INT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0_OVERRUN_INT_ENABLE`"]
pub type SPI0_OVERRUN_INT_ENABLE_R = crate::R<bool, SPI0_OVERRUN_INT_ENABLE_A>;
impl SPI0_OVERRUN_INT_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_OVERRUN_INT_ENABLE_A {
        match self.bits {
            false => SPI0_OVERRUN_INT_ENABLE_A::SPI0_OVERRUN_INT_DISABLE,
            true => SPI0_OVERRUN_INT_ENABLE_A::SPI0_OVERRUN_INT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `SPI0_OVERRUN_INT_DISABLE`"]
    #[inline(always)]
    pub fn is_spi0_overrun_int_disable(&self) -> bool {
        *self == SPI0_OVERRUN_INT_ENABLE_A::SPI0_OVERRUN_INT_DISABLE
    }
    #[doc = "Checks if the value of the field is `SPI0_OVERRUN_INT_ENABLE`"]
    #[inline(always)]
    pub fn is_spi0_overrun_int_enable(&self) -> bool {
        *self == SPI0_OVERRUN_INT_ENABLE_A::SPI0_OVERRUN_INT_ENABLE
    }
}
#[doc = "Write proxy for field `SPI0_OVERRUN_INT_ENABLE`"]
pub struct SPI0_OVERRUN_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_OVERRUN_INT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_OVERRUN_INT_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupts are raised when an overrun occurs on the SPI interface"]
    #[inline(always)]
    pub fn spi0_overrun_int_disable(self) -> &'a mut W {
        self.variant(SPI0_OVERRUN_INT_ENABLE_A::SPI0_OVERRUN_INT_DISABLE)
    }
    #[doc = "An interrupt is raised when an overrun occurs on the SPI interface"]
    #[inline(always)]
    pub fn spi0_overrun_int_enable(self) -> &'a mut W {
        self.variant(SPI0_OVERRUN_INT_ENABLE_A::SPI0_OVERRUN_INT_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Enable/disable SPI underrun interrupts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_UNDERRUN_INT_ENABLE_A {
    #[doc = "0: No interrupts are raised when an underrun occurs on the SPI interface"]
    SPI0_UNDERRUN_INT_DISABLE = 0,
    #[doc = "1: An interrupt is raised when an underrun occurs on the SPI interface"]
    SPI0_UNDERRUN_INT_ENABLE = 1,
}
impl From<SPI0_UNDERRUN_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_UNDERRUN_INT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0_UNDERRUN_INT_ENABLE`"]
pub type SPI0_UNDERRUN_INT_ENABLE_R = crate::R<bool, SPI0_UNDERRUN_INT_ENABLE_A>;
impl SPI0_UNDERRUN_INT_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_UNDERRUN_INT_ENABLE_A {
        match self.bits {
            false => SPI0_UNDERRUN_INT_ENABLE_A::SPI0_UNDERRUN_INT_DISABLE,
            true => SPI0_UNDERRUN_INT_ENABLE_A::SPI0_UNDERRUN_INT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `SPI0_UNDERRUN_INT_DISABLE`"]
    #[inline(always)]
    pub fn is_spi0_underrun_int_disable(&self) -> bool {
        *self == SPI0_UNDERRUN_INT_ENABLE_A::SPI0_UNDERRUN_INT_DISABLE
    }
    #[doc = "Checks if the value of the field is `SPI0_UNDERRUN_INT_ENABLE`"]
    #[inline(always)]
    pub fn is_spi0_underrun_int_enable(&self) -> bool {
        *self == SPI0_UNDERRUN_INT_ENABLE_A::SPI0_UNDERRUN_INT_ENABLE
    }
}
#[doc = "Write proxy for field `SPI0_UNDERRUN_INT_ENABLE`"]
pub struct SPI0_UNDERRUN_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_UNDERRUN_INT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_UNDERRUN_INT_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupts are raised when an underrun occurs on the SPI interface"]
    #[inline(always)]
    pub fn spi0_underrun_int_disable(self) -> &'a mut W {
        self.variant(SPI0_UNDERRUN_INT_ENABLE_A::SPI0_UNDERRUN_INT_DISABLE)
    }
    #[doc = "An interrupt is raised when an underrun occurs on the SPI interface"]
    #[inline(always)]
    pub fn spi0_underrun_int_enable(self) -> &'a mut W {
        self.variant(SPI0_UNDERRUN_INT_ENABLE_A::SPI0_UNDERRUN_INT_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Select whether data transfer will be controlled by the CM3 or the DMA for SPI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_CONTROLLER_A {
    #[doc = "0: The CM3 controls data transfers using SPI"]
    SPI0_CONTROLLER_CM3 = 0,
    #[doc = "1: The DMA controls data transfers using SPI"]
    SPI0_CONTROLLER_DMA = 1,
}
impl From<SPI0_CONTROLLER_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_CONTROLLER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0_CONTROLLER`"]
pub type SPI0_CONTROLLER_R = crate::R<bool, SPI0_CONTROLLER_A>;
impl SPI0_CONTROLLER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_CONTROLLER_A {
        match self.bits {
            false => SPI0_CONTROLLER_A::SPI0_CONTROLLER_CM3,
            true => SPI0_CONTROLLER_A::SPI0_CONTROLLER_DMA,
        }
    }
    #[doc = "Checks if the value of the field is `SPI0_CONTROLLER_CM3`"]
    #[inline(always)]
    pub fn is_spi0_controller_cm3(&self) -> bool {
        *self == SPI0_CONTROLLER_A::SPI0_CONTROLLER_CM3
    }
    #[doc = "Checks if the value of the field is `SPI0_CONTROLLER_DMA`"]
    #[inline(always)]
    pub fn is_spi0_controller_dma(&self) -> bool {
        *self == SPI0_CONTROLLER_A::SPI0_CONTROLLER_DMA
    }
}
#[doc = "Write proxy for field `SPI0_CONTROLLER`"]
pub struct SPI0_CONTROLLER_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_CONTROLLER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_CONTROLLER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The CM3 controls data transfers using SPI"]
    #[inline(always)]
    pub fn spi0_controller_cm3(self) -> &'a mut W {
        self.variant(SPI0_CONTROLLER_A::SPI0_CONTROLLER_CM3)
    }
    #[doc = "The DMA controls data transfers using SPI"]
    #[inline(always)]
    pub fn spi0_controller_dma(self) -> &'a mut W {
        self.variant(SPI0_CONTROLLER_A::SPI0_CONTROLLER_DMA)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Use the SPI interface as master or slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_SLAVE_A {
    #[doc = "0: Use the SPI interface in master mode"]
    SPI0_SELECT_MASTER = 0,
    #[doc = "1: Use the SPI interface in slave mode"]
    SPI0_SELECT_SLAVE = 1,
}
impl From<SPI0_SLAVE_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_SLAVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0_SLAVE`"]
pub type SPI0_SLAVE_R = crate::R<bool, SPI0_SLAVE_A>;
impl SPI0_SLAVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_SLAVE_A {
        match self.bits {
            false => SPI0_SLAVE_A::SPI0_SELECT_MASTER,
            true => SPI0_SLAVE_A::SPI0_SELECT_SLAVE,
        }
    }
    #[doc = "Checks if the value of the field is `SPI0_SELECT_MASTER`"]
    #[inline(always)]
    pub fn is_spi0_select_master(&self) -> bool {
        *self == SPI0_SLAVE_A::SPI0_SELECT_MASTER
    }
    #[doc = "Checks if the value of the field is `SPI0_SELECT_SLAVE`"]
    #[inline(always)]
    pub fn is_spi0_select_slave(&self) -> bool {
        *self == SPI0_SLAVE_A::SPI0_SELECT_SLAVE
    }
}
#[doc = "Write proxy for field `SPI0_SLAVE`"]
pub struct SPI0_SLAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_SLAVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_SLAVE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use the SPI interface in master mode"]
    #[inline(always)]
    pub fn spi0_select_master(self) -> &'a mut W {
        self.variant(SPI0_SLAVE_A::SPI0_SELECT_MASTER)
    }
    #[doc = "Use the SPI interface in slave mode"]
    #[inline(always)]
    pub fn spi0_select_slave(self) -> &'a mut W {
        self.variant(SPI0_SLAVE_A::SPI0_SELECT_SLAVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Select the polarity of the SPI clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_CLK_POLARITY_A {
    #[doc = "0: In both master and slave modes SERO changes on the falling edge of the SPI clock. The SERI is sampled in slave mode just after and in master mode at the rising edge of the SPI clock"]
    SPI0_CLK_POLARITY_NORMAL = 0,
    #[doc = "1: In both master and slave modes SERO changes on the rising edge of the SPI clock. The SERI is sampled in slave mode just after and in master mode at the falling edge of the SPI clock"]
    SPI0_CLK_POLARITY_INVERSE = 1,
}
impl From<SPI0_CLK_POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_CLK_POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0_CLK_POLARITY`"]
pub type SPI0_CLK_POLARITY_R = crate::R<bool, SPI0_CLK_POLARITY_A>;
impl SPI0_CLK_POLARITY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_CLK_POLARITY_A {
        match self.bits {
            false => SPI0_CLK_POLARITY_A::SPI0_CLK_POLARITY_NORMAL,
            true => SPI0_CLK_POLARITY_A::SPI0_CLK_POLARITY_INVERSE,
        }
    }
    #[doc = "Checks if the value of the field is `SPI0_CLK_POLARITY_NORMAL`"]
    #[inline(always)]
    pub fn is_spi0_clk_polarity_normal(&self) -> bool {
        *self == SPI0_CLK_POLARITY_A::SPI0_CLK_POLARITY_NORMAL
    }
    #[doc = "Checks if the value of the field is `SPI0_CLK_POLARITY_INVERSE`"]
    #[inline(always)]
    pub fn is_spi0_clk_polarity_inverse(&self) -> bool {
        *self == SPI0_CLK_POLARITY_A::SPI0_CLK_POLARITY_INVERSE
    }
}
#[doc = "Write proxy for field `SPI0_CLK_POLARITY`"]
pub struct SPI0_CLK_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_CLK_POLARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_CLK_POLARITY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "In both master and slave modes SERO changes on the falling edge of the SPI clock. The SERI is sampled in slave mode just after and in master mode at the rising edge of the SPI clock"]
    #[inline(always)]
    pub fn spi0_clk_polarity_normal(self) -> &'a mut W {
        self.variant(SPI0_CLK_POLARITY_A::SPI0_CLK_POLARITY_NORMAL)
    }
    #[doc = "In both master and slave modes SERO changes on the rising edge of the SPI clock. The SERI is sampled in slave mode just after and in master mode at the falling edge of the SPI clock"]
    #[inline(always)]
    pub fn spi0_clk_polarity_inverse(self) -> &'a mut W {
        self.variant(SPI0_CLK_POLARITY_A::SPI0_CLK_POLARITY_INVERSE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Select between manual and auto transaction handling modes for SPI master transactions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_MODE_SELECT_A {
    #[doc = "0: Master transfers using the SPI interface do not automatically continue"]
    SPI0_MODE_SELECT_MANUAL = 0,
    #[doc = "1: Automatically continue master transfers using the SPI interface"]
    SPI0_MODE_SELECT_AUTO = 1,
}
impl From<SPI0_MODE_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_MODE_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0_MODE_SELECT`"]
pub type SPI0_MODE_SELECT_R = crate::R<bool, SPI0_MODE_SELECT_A>;
impl SPI0_MODE_SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_MODE_SELECT_A {
        match self.bits {
            false => SPI0_MODE_SELECT_A::SPI0_MODE_SELECT_MANUAL,
            true => SPI0_MODE_SELECT_A::SPI0_MODE_SELECT_AUTO,
        }
    }
    #[doc = "Checks if the value of the field is `SPI0_MODE_SELECT_MANUAL`"]
    #[inline(always)]
    pub fn is_spi0_mode_select_manual(&self) -> bool {
        *self == SPI0_MODE_SELECT_A::SPI0_MODE_SELECT_MANUAL
    }
    #[doc = "Checks if the value of the field is `SPI0_MODE_SELECT_AUTO`"]
    #[inline(always)]
    pub fn is_spi0_mode_select_auto(&self) -> bool {
        *self == SPI0_MODE_SELECT_A::SPI0_MODE_SELECT_AUTO
    }
}
#[doc = "Write proxy for field `SPI0_MODE_SELECT`"]
pub struct SPI0_MODE_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_MODE_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_MODE_SELECT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master transfers using the SPI interface do not automatically continue"]
    #[inline(always)]
    pub fn spi0_mode_select_manual(self) -> &'a mut W {
        self.variant(SPI0_MODE_SELECT_A::SPI0_MODE_SELECT_MANUAL)
    }
    #[doc = "Automatically continue master transfers using the SPI interface"]
    #[inline(always)]
    pub fn spi0_mode_select_auto(self) -> &'a mut W {
        self.variant(SPI0_MODE_SELECT_A::SPI0_MODE_SELECT_AUTO)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Enable/disable the SPI interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_ENABLE_A {
    #[doc = "0: Disable the SPI interface"]
    SPI0_DISABLE = 0,
    #[doc = "1: Enable the SPI interface"]
    SPI0_ENABLE = 1,
}
impl From<SPI0_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0_ENABLE`"]
pub type SPI0_ENABLE_R = crate::R<bool, SPI0_ENABLE_A>;
impl SPI0_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_ENABLE_A {
        match self.bits {
            false => SPI0_ENABLE_A::SPI0_DISABLE,
            true => SPI0_ENABLE_A::SPI0_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `SPI0_DISABLE`"]
    #[inline(always)]
    pub fn is_spi0_disable(&self) -> bool {
        *self == SPI0_ENABLE_A::SPI0_DISABLE
    }
    #[doc = "Checks if the value of the field is `SPI0_ENABLE`"]
    #[inline(always)]
    pub fn is_spi0_enable(&self) -> bool {
        *self == SPI0_ENABLE_A::SPI0_ENABLE
    }
}
#[doc = "Write proxy for field `SPI0_ENABLE`"]
pub struct SPI0_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the SPI interface"]
    #[inline(always)]
    pub fn spi0_disable(self) -> &'a mut W {
        self.variant(SPI0_ENABLE_A::SPI0_DISABLE)
    }
    #[doc = "Enable the SPI interface"]
    #[inline(always)]
    pub fn spi0_enable(self) -> &'a mut W {
        self.variant(SPI0_ENABLE_A::SPI0_ENABLE)
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
#[doc = "Prescale the SPI interface clock for master transfers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPI0_PRESCALE_A {
    #[doc = "0: Prescale the SPI interface clock by 2"]
    SPI0_PRESCALE_2 = 0,
    #[doc = "1: Prescale the SPI interface clock by 4"]
    SPI0_PRESCALE_4 = 1,
    #[doc = "2: Prescale the SPI interface clock by 8"]
    SPI0_PRESCALE_8 = 2,
    #[doc = "3: Prescale the SPI interface clock by 16"]
    SPI0_PRESCALE_16 = 3,
    #[doc = "4: Prescale the SPI interface clock by 32"]
    SPI0_PRESCALE_32 = 4,
    #[doc = "5: Prescale the SPI interface clock by 64"]
    SPI0_PRESCALE_64 = 5,
    #[doc = "6: Prescale the SPI interface clock by 128"]
    SPI0_PRESCALE_128 = 6,
    #[doc = "7: Prescale the SPI interface clock by 256"]
    SPI0_PRESCALE_256 = 7,
    #[doc = "8: Prescale the SPI interface clock by 512"]
    SPI0_PRESCALE_512 = 8,
    #[doc = "9: Prescale the SPI interface clock by 1024"]
    SPI0_PRESCALE_1024 = 9,
}
impl From<SPI0_PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI0_PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPI0_PRESCALE`"]
pub type SPI0_PRESCALE_R = crate::R<u8, SPI0_PRESCALE_A>;
impl SPI0_PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPI0_PRESCALE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SPI0_PRESCALE_A::SPI0_PRESCALE_2),
            1 => Val(SPI0_PRESCALE_A::SPI0_PRESCALE_4),
            2 => Val(SPI0_PRESCALE_A::SPI0_PRESCALE_8),
            3 => Val(SPI0_PRESCALE_A::SPI0_PRESCALE_16),
            4 => Val(SPI0_PRESCALE_A::SPI0_PRESCALE_32),
            5 => Val(SPI0_PRESCALE_A::SPI0_PRESCALE_64),
            6 => Val(SPI0_PRESCALE_A::SPI0_PRESCALE_128),
            7 => Val(SPI0_PRESCALE_A::SPI0_PRESCALE_256),
            8 => Val(SPI0_PRESCALE_A::SPI0_PRESCALE_512),
            9 => Val(SPI0_PRESCALE_A::SPI0_PRESCALE_1024),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SPI0_PRESCALE_2`"]
    #[inline(always)]
    pub fn is_spi0_prescale_2(&self) -> bool {
        *self == SPI0_PRESCALE_A::SPI0_PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `SPI0_PRESCALE_4`"]
    #[inline(always)]
    pub fn is_spi0_prescale_4(&self) -> bool {
        *self == SPI0_PRESCALE_A::SPI0_PRESCALE_4
    }
    #[doc = "Checks if the value of the field is `SPI0_PRESCALE_8`"]
    #[inline(always)]
    pub fn is_spi0_prescale_8(&self) -> bool {
        *self == SPI0_PRESCALE_A::SPI0_PRESCALE_8
    }
    #[doc = "Checks if the value of the field is `SPI0_PRESCALE_16`"]
    #[inline(always)]
    pub fn is_spi0_prescale_16(&self) -> bool {
        *self == SPI0_PRESCALE_A::SPI0_PRESCALE_16
    }
    #[doc = "Checks if the value of the field is `SPI0_PRESCALE_32`"]
    #[inline(always)]
    pub fn is_spi0_prescale_32(&self) -> bool {
        *self == SPI0_PRESCALE_A::SPI0_PRESCALE_32
    }
    #[doc = "Checks if the value of the field is `SPI0_PRESCALE_64`"]
    #[inline(always)]
    pub fn is_spi0_prescale_64(&self) -> bool {
        *self == SPI0_PRESCALE_A::SPI0_PRESCALE_64
    }
    #[doc = "Checks if the value of the field is `SPI0_PRESCALE_128`"]
    #[inline(always)]
    pub fn is_spi0_prescale_128(&self) -> bool {
        *self == SPI0_PRESCALE_A::SPI0_PRESCALE_128
    }
    #[doc = "Checks if the value of the field is `SPI0_PRESCALE_256`"]
    #[inline(always)]
    pub fn is_spi0_prescale_256(&self) -> bool {
        *self == SPI0_PRESCALE_A::SPI0_PRESCALE_256
    }
    #[doc = "Checks if the value of the field is `SPI0_PRESCALE_512`"]
    #[inline(always)]
    pub fn is_spi0_prescale_512(&self) -> bool {
        *self == SPI0_PRESCALE_A::SPI0_PRESCALE_512
    }
    #[doc = "Checks if the value of the field is `SPI0_PRESCALE_1024`"]
    #[inline(always)]
    pub fn is_spi0_prescale_1024(&self) -> bool {
        *self == SPI0_PRESCALE_A::SPI0_PRESCALE_1024
    }
}
#[doc = "Write proxy for field `SPI0_PRESCALE`"]
pub struct SPI0_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_PRESCALE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Prescale the SPI interface clock by 2"]
    #[inline(always)]
    pub fn spi0_prescale_2(self) -> &'a mut W {
        self.variant(SPI0_PRESCALE_A::SPI0_PRESCALE_2)
    }
    #[doc = "Prescale the SPI interface clock by 4"]
    #[inline(always)]
    pub fn spi0_prescale_4(self) -> &'a mut W {
        self.variant(SPI0_PRESCALE_A::SPI0_PRESCALE_4)
    }
    #[doc = "Prescale the SPI interface clock by 8"]
    #[inline(always)]
    pub fn spi0_prescale_8(self) -> &'a mut W {
        self.variant(SPI0_PRESCALE_A::SPI0_PRESCALE_8)
    }
    #[doc = "Prescale the SPI interface clock by 16"]
    #[inline(always)]
    pub fn spi0_prescale_16(self) -> &'a mut W {
        self.variant(SPI0_PRESCALE_A::SPI0_PRESCALE_16)
    }
    #[doc = "Prescale the SPI interface clock by 32"]
    #[inline(always)]
    pub fn spi0_prescale_32(self) -> &'a mut W {
        self.variant(SPI0_PRESCALE_A::SPI0_PRESCALE_32)
    }
    #[doc = "Prescale the SPI interface clock by 64"]
    #[inline(always)]
    pub fn spi0_prescale_64(self) -> &'a mut W {
        self.variant(SPI0_PRESCALE_A::SPI0_PRESCALE_64)
    }
    #[doc = "Prescale the SPI interface clock by 128"]
    #[inline(always)]
    pub fn spi0_prescale_128(self) -> &'a mut W {
        self.variant(SPI0_PRESCALE_A::SPI0_PRESCALE_128)
    }
    #[doc = "Prescale the SPI interface clock by 256"]
    #[inline(always)]
    pub fn spi0_prescale_256(self) -> &'a mut W {
        self.variant(SPI0_PRESCALE_A::SPI0_PRESCALE_256)
    }
    #[doc = "Prescale the SPI interface clock by 512"]
    #[inline(always)]
    pub fn spi0_prescale_512(self) -> &'a mut W {
        self.variant(SPI0_PRESCALE_A::SPI0_PRESCALE_512)
    }
    #[doc = "Prescale the SPI interface clock by 1024"]
    #[inline(always)]
    pub fn spi0_prescale_1024(self) -> &'a mut W {
        self.variant(SPI0_PRESCALE_A::SPI0_PRESCALE_1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 10 - Enable/disable SPI overrun interrupts"]
    #[inline(always)]
    pub fn spi0_overrun_int_enable(&self) -> SPI0_OVERRUN_INT_ENABLE_R {
        SPI0_OVERRUN_INT_ENABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable/disable SPI underrun interrupts"]
    #[inline(always)]
    pub fn spi0_underrun_int_enable(&self) -> SPI0_UNDERRUN_INT_ENABLE_R {
        SPI0_UNDERRUN_INT_ENABLE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Select whether data transfer will be controlled by the CM3 or the DMA for SPI"]
    #[inline(always)]
    pub fn spi0_controller(&self) -> SPI0_CONTROLLER_R {
        SPI0_CONTROLLER_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Use the SPI interface as master or slave"]
    #[inline(always)]
    pub fn spi0_slave(&self) -> SPI0_SLAVE_R {
        SPI0_SLAVE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Select the polarity of the SPI clock"]
    #[inline(always)]
    pub fn spi0_clk_polarity(&self) -> SPI0_CLK_POLARITY_R {
        SPI0_CLK_POLARITY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Select between manual and auto transaction handling modes for SPI master transactions"]
    #[inline(always)]
    pub fn spi0_mode_select(&self) -> SPI0_MODE_SELECT_R {
        SPI0_MODE_SELECT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable/disable the SPI interface"]
    #[inline(always)]
    pub fn spi0_enable(&self) -> SPI0_ENABLE_R {
        SPI0_ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - Prescale the SPI interface clock for master transfers"]
    #[inline(always)]
    pub fn spi0_prescale(&self) -> SPI0_PRESCALE_R {
        SPI0_PRESCALE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 10 - Enable/disable SPI overrun interrupts"]
    #[inline(always)]
    pub fn spi0_overrun_int_enable(&mut self) -> SPI0_OVERRUN_INT_ENABLE_W {
        SPI0_OVERRUN_INT_ENABLE_W { w: self }
    }
    #[doc = "Bit 9 - Enable/disable SPI underrun interrupts"]
    #[inline(always)]
    pub fn spi0_underrun_int_enable(&mut self) -> SPI0_UNDERRUN_INT_ENABLE_W {
        SPI0_UNDERRUN_INT_ENABLE_W { w: self }
    }
    #[doc = "Bit 8 - Select whether data transfer will be controlled by the CM3 or the DMA for SPI"]
    #[inline(always)]
    pub fn spi0_controller(&mut self) -> SPI0_CONTROLLER_W {
        SPI0_CONTROLLER_W { w: self }
    }
    #[doc = "Bit 7 - Use the SPI interface as master or slave"]
    #[inline(always)]
    pub fn spi0_slave(&mut self) -> SPI0_SLAVE_W {
        SPI0_SLAVE_W { w: self }
    }
    #[doc = "Bit 6 - Select the polarity of the SPI clock"]
    #[inline(always)]
    pub fn spi0_clk_polarity(&mut self) -> SPI0_CLK_POLARITY_W {
        SPI0_CLK_POLARITY_W { w: self }
    }
    #[doc = "Bit 5 - Select between manual and auto transaction handling modes for SPI master transactions"]
    #[inline(always)]
    pub fn spi0_mode_select(&mut self) -> SPI0_MODE_SELECT_W {
        SPI0_MODE_SELECT_W { w: self }
    }
    #[doc = "Bit 4 - Enable/disable the SPI interface"]
    #[inline(always)]
    pub fn spi0_enable(&mut self) -> SPI0_ENABLE_W {
        SPI0_ENABLE_W { w: self }
    }
    #[doc = "Bits 0:3 - Prescale the SPI interface clock for master transfers"]
    #[inline(always)]
    pub fn spi0_prescale(&mut self) -> SPI0_PRESCALE_W {
        SPI0_PRESCALE_W { w: self }
    }
}
