#[doc = "Reader of register RF_REG2F"]
pub type R = crate::R<u32, super::RF_REG2F>;
#[doc = "Writer for register RF_REG2F"]
pub type W = crate::W<u32, super::RF_REG2F>;
#[doc = "Register RF_REG2F `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG2F {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock division factor for ck_div_1_6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CK_DIV_1_6_CK_DIV_1_6_A {
    #[doc = "0: This means that no clock is generated"]
    CK_DIV_1_6_NO_CLOCK = 0,
    #[doc = "1: `1`"]
    CK_DIV_1_6_PRESCALE_1 = 1,
    #[doc = "2: `10`"]
    CK_DIV_1_6_PRESCALE_2 = 2,
    #[doc = "3: `11`"]
    CK_DIV_1_6_PRESCALE_3 = 3,
    #[doc = "4: `100`"]
    CK_DIV_1_6_PRESCALE_4 = 4,
    #[doc = "5: `101`"]
    CK_DIV_1_6_PRESCALE_5 = 5,
    #[doc = "6: `110`"]
    CK_DIV_1_6_PRESCALE_6 = 6,
    #[doc = "7: `111`"]
    CK_DIV_1_6_PRESCALE_7 = 7,
}
impl From<CK_DIV_1_6_CK_DIV_1_6_A> for u8 {
    #[inline(always)]
    fn from(variant: CK_DIV_1_6_CK_DIV_1_6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CK_DIV_1_6_CK_DIV_1_6`"]
pub type CK_DIV_1_6_CK_DIV_1_6_R = crate::R<u8, CK_DIV_1_6_CK_DIV_1_6_A>;
impl CK_DIV_1_6_CK_DIV_1_6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CK_DIV_1_6_CK_DIV_1_6_A {
        match self.bits {
            0 => CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_NO_CLOCK,
            1 => CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_1,
            2 => CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_2,
            3 => CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_3,
            4 => CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_4,
            5 => CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_5,
            6 => CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_6,
            7 => CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CK_DIV_1_6_NO_CLOCK`"]
    #[inline(always)]
    pub fn is_ck_div_1_6_no_clock(&self) -> bool {
        *self == CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_NO_CLOCK
    }
    #[doc = "Checks if the value of the field is `CK_DIV_1_6_PRESCALE_1`"]
    #[inline(always)]
    pub fn is_ck_div_1_6_prescale_1(&self) -> bool {
        *self == CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `CK_DIV_1_6_PRESCALE_2`"]
    #[inline(always)]
    pub fn is_ck_div_1_6_prescale_2(&self) -> bool {
        *self == CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `CK_DIV_1_6_PRESCALE_3`"]
    #[inline(always)]
    pub fn is_ck_div_1_6_prescale_3(&self) -> bool {
        *self == CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_3
    }
    #[doc = "Checks if the value of the field is `CK_DIV_1_6_PRESCALE_4`"]
    #[inline(always)]
    pub fn is_ck_div_1_6_prescale_4(&self) -> bool {
        *self == CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_4
    }
    #[doc = "Checks if the value of the field is `CK_DIV_1_6_PRESCALE_5`"]
    #[inline(always)]
    pub fn is_ck_div_1_6_prescale_5(&self) -> bool {
        *self == CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_5
    }
    #[doc = "Checks if the value of the field is `CK_DIV_1_6_PRESCALE_6`"]
    #[inline(always)]
    pub fn is_ck_div_1_6_prescale_6(&self) -> bool {
        *self == CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_6
    }
    #[doc = "Checks if the value of the field is `CK_DIV_1_6_PRESCALE_7`"]
    #[inline(always)]
    pub fn is_ck_div_1_6_prescale_7(&self) -> bool {
        *self == CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_7
    }
}
#[doc = "Write proxy for field `CK_DIV_1_6_CK_DIV_1_6`"]
pub struct CK_DIV_1_6_CK_DIV_1_6_W<'a> {
    w: &'a mut W,
}
impl<'a> CK_DIV_1_6_CK_DIV_1_6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CK_DIV_1_6_CK_DIV_1_6_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "This means that no clock is generated"]
    #[inline(always)]
    pub fn ck_div_1_6_no_clock(self) -> &'a mut W {
        self.variant(CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_NO_CLOCK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ck_div_1_6_prescale_1(self) -> &'a mut W {
        self.variant(CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ck_div_1_6_prescale_2(self) -> &'a mut W {
        self.variant(CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn ck_div_1_6_prescale_3(self) -> &'a mut W {
        self.variant(CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ck_div_1_6_prescale_4(self) -> &'a mut W {
        self.variant(CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_4)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn ck_div_1_6_prescale_5(self) -> &'a mut W {
        self.variant(CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_5)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn ck_div_1_6_prescale_6(self) -> &'a mut W {
        self.variant(CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_6)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn ck_div_1_6_prescale_7(self) -> &'a mut W {
        self.variant(CK_DIV_1_6_CK_DIV_1_6_A::CK_DIV_1_6_PRESCALE_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "If set to 1 enables the increased drive strength of the digital pads\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PADS_PE_DS_GPIO_DS_A {
    #[doc = "0: `0`"]
    PADS_PE_DS_GPIO_DS_DEFAULT = 0,
}
impl From<PADS_PE_DS_GPIO_DS_A> for bool {
    #[inline(always)]
    fn from(variant: PADS_PE_DS_GPIO_DS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PADS_PE_DS_GPIO_DS`"]
pub type PADS_PE_DS_GPIO_DS_R = crate::R<bool, PADS_PE_DS_GPIO_DS_A>;
impl PADS_PE_DS_GPIO_DS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PADS_PE_DS_GPIO_DS_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PADS_PE_DS_GPIO_DS_A::PADS_PE_DS_GPIO_DS_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PADS_PE_DS_GPIO_DS_DEFAULT`"]
    #[inline(always)]
    pub fn is_pads_pe_ds_gpio_ds_default(&self) -> bool {
        *self == PADS_PE_DS_GPIO_DS_A::PADS_PE_DS_GPIO_DS_DEFAULT
    }
}
#[doc = "Write proxy for field `PADS_PE_DS_GPIO_DS`"]
pub struct PADS_PE_DS_GPIO_DS_W<'a> {
    w: &'a mut W,
}
impl<'a> PADS_PE_DS_GPIO_DS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PADS_PE_DS_GPIO_DS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pads_pe_ds_gpio_ds_default(self) -> &'a mut W {
        self.variant(PADS_PE_DS_GPIO_DS_A::PADS_PE_DS_GPIO_DS_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "If set to 1 enables the pull-up of the GPIO pads\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PADS_PE_DS_GPIO_PE_A {
    #[doc = "0: `0`"]
    PADS_PE_DS_GPIO_PE_DEFAULT = 0,
}
impl From<PADS_PE_DS_GPIO_PE_A> for bool {
    #[inline(always)]
    fn from(variant: PADS_PE_DS_GPIO_PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PADS_PE_DS_GPIO_PE`"]
pub type PADS_PE_DS_GPIO_PE_R = crate::R<bool, PADS_PE_DS_GPIO_PE_A>;
impl PADS_PE_DS_GPIO_PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PADS_PE_DS_GPIO_PE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PADS_PE_DS_GPIO_PE_A::PADS_PE_DS_GPIO_PE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PADS_PE_DS_GPIO_PE_DEFAULT`"]
    #[inline(always)]
    pub fn is_pads_pe_ds_gpio_pe_default(&self) -> bool {
        *self == PADS_PE_DS_GPIO_PE_A::PADS_PE_DS_GPIO_PE_DEFAULT
    }
}
#[doc = "Write proxy for field `PADS_PE_DS_GPIO_PE`"]
pub struct PADS_PE_DS_GPIO_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PADS_PE_DS_GPIO_PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PADS_PE_DS_GPIO_PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pads_pe_ds_gpio_pe_default(self) -> &'a mut W {
        self.variant(PADS_PE_DS_GPIO_PE_A::PADS_PE_DS_GPIO_PE_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "If set to 1 enables the pull-up of the nreset pad\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PADS_PE_DS_NRESET_PE_A {
    #[doc = "0: `0`"]
    PADS_PE_DS_NRESET_PE_DEFAULT = 0,
}
impl From<PADS_PE_DS_NRESET_PE_A> for bool {
    #[inline(always)]
    fn from(variant: PADS_PE_DS_NRESET_PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PADS_PE_DS_NRESET_PE`"]
pub type PADS_PE_DS_NRESET_PE_R = crate::R<bool, PADS_PE_DS_NRESET_PE_A>;
impl PADS_PE_DS_NRESET_PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PADS_PE_DS_NRESET_PE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PADS_PE_DS_NRESET_PE_A::PADS_PE_DS_NRESET_PE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PADS_PE_DS_NRESET_PE_DEFAULT`"]
    #[inline(always)]
    pub fn is_pads_pe_ds_nreset_pe_default(&self) -> bool {
        *self == PADS_PE_DS_NRESET_PE_A::PADS_PE_DS_NRESET_PE_DEFAULT
    }
}
#[doc = "Write proxy for field `PADS_PE_DS_NRESET_PE`"]
pub struct PADS_PE_DS_NRESET_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PADS_PE_DS_NRESET_PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PADS_PE_DS_NRESET_PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pads_pe_ds_nreset_pe_default(self) -> &'a mut W {
        self.variant(PADS_PE_DS_NRESET_PE_A::PADS_PE_DS_NRESET_PE_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "If set to 1 enables the pull-up of the MISO SPI pad\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PADS_PE_DS_SPI_MISO_PE_A {
    #[doc = "0: `0`"]
    PADS_PE_DS_SPI_MISO_PE_DEFAULT = 0,
}
impl From<PADS_PE_DS_SPI_MISO_PE_A> for bool {
    #[inline(always)]
    fn from(variant: PADS_PE_DS_SPI_MISO_PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PADS_PE_DS_SPI_MISO_PE`"]
pub type PADS_PE_DS_SPI_MISO_PE_R = crate::R<bool, PADS_PE_DS_SPI_MISO_PE_A>;
impl PADS_PE_DS_SPI_MISO_PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PADS_PE_DS_SPI_MISO_PE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PADS_PE_DS_SPI_MISO_PE_A::PADS_PE_DS_SPI_MISO_PE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PADS_PE_DS_SPI_MISO_PE_DEFAULT`"]
    #[inline(always)]
    pub fn is_pads_pe_ds_spi_miso_pe_default(&self) -> bool {
        *self == PADS_PE_DS_SPI_MISO_PE_A::PADS_PE_DS_SPI_MISO_PE_DEFAULT
    }
}
#[doc = "Write proxy for field `PADS_PE_DS_SPI_MISO_PE`"]
pub struct PADS_PE_DS_SPI_MISO_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PADS_PE_DS_SPI_MISO_PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PADS_PE_DS_SPI_MISO_PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pads_pe_ds_spi_miso_pe_default(self) -> &'a mut W {
        self.variant(PADS_PE_DS_SPI_MISO_PE_A::PADS_PE_DS_SPI_MISO_PE_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "If set to 1 enables the pull-up of the MOSI SPI pad\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PADS_PE_DS_SPI_MOSI_PE_A {
    #[doc = "0: `0`"]
    PADS_PE_DS_SPI_MOSI_PE_DEFAULT = 0,
}
impl From<PADS_PE_DS_SPI_MOSI_PE_A> for bool {
    #[inline(always)]
    fn from(variant: PADS_PE_DS_SPI_MOSI_PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PADS_PE_DS_SPI_MOSI_PE`"]
pub type PADS_PE_DS_SPI_MOSI_PE_R = crate::R<bool, PADS_PE_DS_SPI_MOSI_PE_A>;
impl PADS_PE_DS_SPI_MOSI_PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PADS_PE_DS_SPI_MOSI_PE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PADS_PE_DS_SPI_MOSI_PE_A::PADS_PE_DS_SPI_MOSI_PE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PADS_PE_DS_SPI_MOSI_PE_DEFAULT`"]
    #[inline(always)]
    pub fn is_pads_pe_ds_spi_mosi_pe_default(&self) -> bool {
        *self == PADS_PE_DS_SPI_MOSI_PE_A::PADS_PE_DS_SPI_MOSI_PE_DEFAULT
    }
}
#[doc = "Write proxy for field `PADS_PE_DS_SPI_MOSI_PE`"]
pub struct PADS_PE_DS_SPI_MOSI_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PADS_PE_DS_SPI_MOSI_PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PADS_PE_DS_SPI_MOSI_PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pads_pe_ds_spi_mosi_pe_default(self) -> &'a mut W {
        self.variant(PADS_PE_DS_SPI_MOSI_PE_A::PADS_PE_DS_SPI_MOSI_PE_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "If set to 1 enables the pull-up of the SCLK SPI pad\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PADS_PE_DS_SPI_SCLK_PE_A {
    #[doc = "0: `0`"]
    PADS_PE_DS_SPI_SCLK_PE_DEFAULT = 0,
}
impl From<PADS_PE_DS_SPI_SCLK_PE_A> for bool {
    #[inline(always)]
    fn from(variant: PADS_PE_DS_SPI_SCLK_PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PADS_PE_DS_SPI_SCLK_PE`"]
pub type PADS_PE_DS_SPI_SCLK_PE_R = crate::R<bool, PADS_PE_DS_SPI_SCLK_PE_A>;
impl PADS_PE_DS_SPI_SCLK_PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PADS_PE_DS_SPI_SCLK_PE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PADS_PE_DS_SPI_SCLK_PE_A::PADS_PE_DS_SPI_SCLK_PE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PADS_PE_DS_SPI_SCLK_PE_DEFAULT`"]
    #[inline(always)]
    pub fn is_pads_pe_ds_spi_sclk_pe_default(&self) -> bool {
        *self == PADS_PE_DS_SPI_SCLK_PE_A::PADS_PE_DS_SPI_SCLK_PE_DEFAULT
    }
}
#[doc = "Write proxy for field `PADS_PE_DS_SPI_SCLK_PE`"]
pub struct PADS_PE_DS_SPI_SCLK_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PADS_PE_DS_SPI_SCLK_PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PADS_PE_DS_SPI_SCLK_PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pads_pe_ds_spi_sclk_pe_default(self) -> &'a mut W {
        self.variant(PADS_PE_DS_SPI_SCLK_PE_A::PADS_PE_DS_SPI_SCLK_PE_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "If set to 1 enables the pull-up of the CSN SPI pad\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PADS_PE_DS_SPI_CS_N_PE_A {
    #[doc = "0: `0`"]
    PADS_PE_DS_SPI_CS_N_PE_DEFAULT = 0,
}
impl From<PADS_PE_DS_SPI_CS_N_PE_A> for bool {
    #[inline(always)]
    fn from(variant: PADS_PE_DS_SPI_CS_N_PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PADS_PE_DS_SPI_CS_N_PE`"]
pub type PADS_PE_DS_SPI_CS_N_PE_R = crate::R<bool, PADS_PE_DS_SPI_CS_N_PE_A>;
impl PADS_PE_DS_SPI_CS_N_PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PADS_PE_DS_SPI_CS_N_PE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(PADS_PE_DS_SPI_CS_N_PE_A::PADS_PE_DS_SPI_CS_N_PE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PADS_PE_DS_SPI_CS_N_PE_DEFAULT`"]
    #[inline(always)]
    pub fn is_pads_pe_ds_spi_cs_n_pe_default(&self) -> bool {
        *self == PADS_PE_DS_SPI_CS_N_PE_A::PADS_PE_DS_SPI_CS_N_PE_DEFAULT
    }
}
#[doc = "Write proxy for field `PADS_PE_DS_SPI_CS_N_PE`"]
pub struct PADS_PE_DS_SPI_CS_N_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PADS_PE_DS_SPI_CS_N_PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PADS_PE_DS_SPI_CS_N_PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pads_pe_ds_spi_cs_n_pe_default(self) -> &'a mut W {
        self.variant(PADS_PE_DS_SPI_CS_N_PE_A::PADS_PE_DS_SPI_CS_N_PE_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Select the dithering: 00 no dithering, 01 PN9 positive, 10 PN10 negative, PN9+PN10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBBAND_FLL_SB_FLL_DITHER_A {
    #[doc = "0: `0`"]
    SUBBAND_FLL_SB_FLL_DITHER_DEFAULT = 0,
}
impl From<SUBBAND_FLL_SB_FLL_DITHER_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBBAND_FLL_SB_FLL_DITHER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SUBBAND_FLL_SB_FLL_DITHER`"]
pub type SUBBAND_FLL_SB_FLL_DITHER_R = crate::R<u8, SUBBAND_FLL_SB_FLL_DITHER_A>;
impl SUBBAND_FLL_SB_FLL_DITHER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUBBAND_FLL_SB_FLL_DITHER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SUBBAND_FLL_SB_FLL_DITHER_A::SUBBAND_FLL_SB_FLL_DITHER_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUBBAND_FLL_SB_FLL_DITHER_DEFAULT`"]
    #[inline(always)]
    pub fn is_subband_fll_sb_fll_dither_default(&self) -> bool {
        *self == SUBBAND_FLL_SB_FLL_DITHER_A::SUBBAND_FLL_SB_FLL_DITHER_DEFAULT
    }
}
#[doc = "Write proxy for field `SUBBAND_FLL_SB_FLL_DITHER`"]
pub struct SUBBAND_FLL_SB_FLL_DITHER_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBBAND_FLL_SB_FLL_DITHER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUBBAND_FLL_SB_FLL_DITHER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn subband_fll_sb_fll_dither_default(self) -> &'a mut W {
        self.variant(SUBBAND_FLL_SB_FLL_DITHER_A::SUBBAND_FLL_SB_FLL_DITHER_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Set the CIC decimator factor: 00 => 16, 01 => 32, 10 => 64, 11 => 128\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBBAND_FLL_SB_FLL_CIC_TAU_A {
    #[doc = "0: `0`"]
    SUBBAND_FLL_SB_FLL_CIC_TAU_DEFAULT = 0,
}
impl From<SUBBAND_FLL_SB_FLL_CIC_TAU_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBBAND_FLL_SB_FLL_CIC_TAU_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SUBBAND_FLL_SB_FLL_CIC_TAU`"]
pub type SUBBAND_FLL_SB_FLL_CIC_TAU_R = crate::R<u8, SUBBAND_FLL_SB_FLL_CIC_TAU_A>;
impl SUBBAND_FLL_SB_FLL_CIC_TAU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUBBAND_FLL_SB_FLL_CIC_TAU_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SUBBAND_FLL_SB_FLL_CIC_TAU_A::SUBBAND_FLL_SB_FLL_CIC_TAU_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUBBAND_FLL_SB_FLL_CIC_TAU_DEFAULT`"]
    #[inline(always)]
    pub fn is_subband_fll_sb_fll_cic_tau_default(&self) -> bool {
        *self == SUBBAND_FLL_SB_FLL_CIC_TAU_A::SUBBAND_FLL_SB_FLL_CIC_TAU_DEFAULT
    }
}
#[doc = "Write proxy for field `SUBBAND_FLL_SB_FLL_CIC_TAU`"]
pub struct SUBBAND_FLL_SB_FLL_CIC_TAU_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBBAND_FLL_SB_FLL_CIC_TAU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUBBAND_FLL_SB_FLL_CIC_TAU_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn subband_fll_sb_fll_cic_tau_default(self) -> &'a mut W {
        self.variant(SUBBAND_FLL_SB_FLL_CIC_TAU_A::SUBBAND_FLL_SB_FLL_CIC_TAU_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "If set to 1, it uses only 4 phases in the frequency detector. Default 8 phases\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUBBAND_FLL_SB_FLL_PH_4_N8_A {
    #[doc = "0: `0`"]
    SUBBAND_FLL_SB_FLL_PH_4_N8_DEFAULT = 0,
}
impl From<SUBBAND_FLL_SB_FLL_PH_4_N8_A> for bool {
    #[inline(always)]
    fn from(variant: SUBBAND_FLL_SB_FLL_PH_4_N8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SUBBAND_FLL_SB_FLL_PH_4_N8`"]
pub type SUBBAND_FLL_SB_FLL_PH_4_N8_R = crate::R<bool, SUBBAND_FLL_SB_FLL_PH_4_N8_A>;
impl SUBBAND_FLL_SB_FLL_PH_4_N8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SUBBAND_FLL_SB_FLL_PH_4_N8_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(SUBBAND_FLL_SB_FLL_PH_4_N8_A::SUBBAND_FLL_SB_FLL_PH_4_N8_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUBBAND_FLL_SB_FLL_PH_4_N8_DEFAULT`"]
    #[inline(always)]
    pub fn is_subband_fll_sb_fll_ph_4_n8_default(&self) -> bool {
        *self == SUBBAND_FLL_SB_FLL_PH_4_N8_A::SUBBAND_FLL_SB_FLL_PH_4_N8_DEFAULT
    }
}
#[doc = "Write proxy for field `SUBBAND_FLL_SB_FLL_PH_4_N8`"]
pub struct SUBBAND_FLL_SB_FLL_PH_4_N8_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBBAND_FLL_SB_FLL_PH_4_N8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUBBAND_FLL_SB_FLL_PH_4_N8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn subband_fll_sb_fll_ph_4_n8_default(self) -> &'a mut W {
        self.variant(SUBBAND_FLL_SB_FLL_PH_4_N8_A::SUBBAND_FLL_SB_FLL_PH_4_N8_DEFAULT)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Set the number of CIC samples before stopping the FLL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBBAND_FLL_SB_FLL_WAIT_A {
    #[doc = "0: `0`"]
    SUBBAND_FLL_SB_FLL_WAIT_DEFAULT = 0,
}
impl From<SUBBAND_FLL_SB_FLL_WAIT_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBBAND_FLL_SB_FLL_WAIT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SUBBAND_FLL_SB_FLL_WAIT`"]
pub type SUBBAND_FLL_SB_FLL_WAIT_R = crate::R<u8, SUBBAND_FLL_SB_FLL_WAIT_A>;
impl SUBBAND_FLL_SB_FLL_WAIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUBBAND_FLL_SB_FLL_WAIT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SUBBAND_FLL_SB_FLL_WAIT_A::SUBBAND_FLL_SB_FLL_WAIT_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUBBAND_FLL_SB_FLL_WAIT_DEFAULT`"]
    #[inline(always)]
    pub fn is_subband_fll_sb_fll_wait_default(&self) -> bool {
        *self == SUBBAND_FLL_SB_FLL_WAIT_A::SUBBAND_FLL_SB_FLL_WAIT_DEFAULT
    }
}
#[doc = "Write proxy for field `SUBBAND_FLL_SB_FLL_WAIT`"]
pub struct SUBBAND_FLL_SB_FLL_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBBAND_FLL_SB_FLL_WAIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUBBAND_FLL_SB_FLL_WAIT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn subband_fll_sb_fll_wait_default(self) -> &'a mut W {
        self.variant(SUBBAND_FLL_SB_FLL_WAIT_A::SUBBAND_FLL_SB_FLL_WAIT_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "If set to 1 enable the sync word bias correction with RSSI detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_WORD_CORR_EN_SYNC_WORD_CORR_A {
    #[doc = "0: `0`"]
    SYNC_WORD_CORR_EN_SYNC_WORD_CORR_DEFAULT = 0,
}
impl From<SYNC_WORD_CORR_EN_SYNC_WORD_CORR_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_WORD_CORR_EN_SYNC_WORD_CORR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNC_WORD_CORR_EN_SYNC_WORD_CORR`"]
pub type SYNC_WORD_CORR_EN_SYNC_WORD_CORR_R = crate::R<bool, SYNC_WORD_CORR_EN_SYNC_WORD_CORR_A>;
impl SYNC_WORD_CORR_EN_SYNC_WORD_CORR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SYNC_WORD_CORR_EN_SYNC_WORD_CORR_A> {
        use crate::Variant::*;
        match self.bits {
            false => {
                Val(SYNC_WORD_CORR_EN_SYNC_WORD_CORR_A::SYNC_WORD_CORR_EN_SYNC_WORD_CORR_DEFAULT)
            }
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYNC_WORD_CORR_EN_SYNC_WORD_CORR_DEFAULT`"]
    #[inline(always)]
    pub fn is_sync_word_corr_en_sync_word_corr_default(&self) -> bool {
        *self == SYNC_WORD_CORR_EN_SYNC_WORD_CORR_A::SYNC_WORD_CORR_EN_SYNC_WORD_CORR_DEFAULT
    }
}
#[doc = "Write proxy for field `SYNC_WORD_CORR_EN_SYNC_WORD_CORR`"]
pub struct SYNC_WORD_CORR_EN_SYNC_WORD_CORR_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_WORD_CORR_EN_SYNC_WORD_CORR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_WORD_CORR_EN_SYNC_WORD_CORR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sync_word_corr_en_sync_word_corr_default(self) -> &'a mut W {
        self.variant(SYNC_WORD_CORR_EN_SYNC_WORD_CORR_A::SYNC_WORD_CORR_EN_SYNC_WORD_CORR_DEFAULT)
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
#[doc = "set the sync word bias. Without the phADC rescaler, it's 8*mod_idx.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNC_WORD_CORR_SYNC_WORD_BIAS_A {
    #[doc = "0: `0`"]
    SYNC_WORD_CORR_SYNC_WORD_BIAS_DEFAULT = 0,
}
impl From<SYNC_WORD_CORR_SYNC_WORD_BIAS_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNC_WORD_CORR_SYNC_WORD_BIAS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNC_WORD_CORR_SYNC_WORD_BIAS`"]
pub type SYNC_WORD_CORR_SYNC_WORD_BIAS_R = crate::R<u8, SYNC_WORD_CORR_SYNC_WORD_BIAS_A>;
impl SYNC_WORD_CORR_SYNC_WORD_BIAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYNC_WORD_CORR_SYNC_WORD_BIAS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYNC_WORD_CORR_SYNC_WORD_BIAS_A::SYNC_WORD_CORR_SYNC_WORD_BIAS_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYNC_WORD_CORR_SYNC_WORD_BIAS_DEFAULT`"]
    #[inline(always)]
    pub fn is_sync_word_corr_sync_word_bias_default(&self) -> bool {
        *self == SYNC_WORD_CORR_SYNC_WORD_BIAS_A::SYNC_WORD_CORR_SYNC_WORD_BIAS_DEFAULT
    }
}
#[doc = "Write proxy for field `SYNC_WORD_CORR_SYNC_WORD_BIAS`"]
pub struct SYNC_WORD_CORR_SYNC_WORD_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_WORD_CORR_SYNC_WORD_BIAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_WORD_CORR_SYNC_WORD_BIAS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sync_word_corr_sync_word_bias_default(self) -> &'a mut W {
        self.variant(SYNC_WORD_CORR_SYNC_WORD_BIAS_A::SYNC_WORD_CORR_SYNC_WORD_BIAS_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:26 - Clock division factor for ck_div_1_6"]
    #[inline(always)]
    pub fn ck_div_1_6_ck_div_1_6(&self) -> CK_DIV_1_6_CK_DIV_1_6_R {
        CK_DIV_1_6_CK_DIV_1_6_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 22 - If set to 1 enables the increased drive strength of the digital pads"]
    #[inline(always)]
    pub fn pads_pe_ds_gpio_ds(&self) -> PADS_PE_DS_GPIO_DS_R {
        PADS_PE_DS_GPIO_DS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - If set to 1 enables the pull-up of the GPIO pads"]
    #[inline(always)]
    pub fn pads_pe_ds_gpio_pe(&self) -> PADS_PE_DS_GPIO_PE_R {
        PADS_PE_DS_GPIO_PE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - If set to 1 enables the pull-up of the nreset pad"]
    #[inline(always)]
    pub fn pads_pe_ds_nreset_pe(&self) -> PADS_PE_DS_NRESET_PE_R {
        PADS_PE_DS_NRESET_PE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - If set to 1 enables the pull-up of the MISO SPI pad"]
    #[inline(always)]
    pub fn pads_pe_ds_spi_miso_pe(&self) -> PADS_PE_DS_SPI_MISO_PE_R {
        PADS_PE_DS_SPI_MISO_PE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - If set to 1 enables the pull-up of the MOSI SPI pad"]
    #[inline(always)]
    pub fn pads_pe_ds_spi_mosi_pe(&self) -> PADS_PE_DS_SPI_MOSI_PE_R {
        PADS_PE_DS_SPI_MOSI_PE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - If set to 1 enables the pull-up of the SCLK SPI pad"]
    #[inline(always)]
    pub fn pads_pe_ds_spi_sclk_pe(&self) -> PADS_PE_DS_SPI_SCLK_PE_R {
        PADS_PE_DS_SPI_SCLK_PE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - If set to 1 enables the pull-up of the CSN SPI pad"]
    #[inline(always)]
    pub fn pads_pe_ds_spi_cs_n_pe(&self) -> PADS_PE_DS_SPI_CS_N_PE_R {
        PADS_PE_DS_SPI_CS_N_PE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Select the dithering: 00 no dithering, 01 PN9 positive, 10 PN10 negative, PN9+PN10"]
    #[inline(always)]
    pub fn subband_fll_sb_fll_dither(&self) -> SUBBAND_FLL_SB_FLL_DITHER_R {
        SUBBAND_FLL_SB_FLL_DITHER_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Set the CIC decimator factor: 00 => 16, 01 => 32, 10 => 64, 11 => 128"]
    #[inline(always)]
    pub fn subband_fll_sb_fll_cic_tau(&self) -> SUBBAND_FLL_SB_FLL_CIC_TAU_R {
        SUBBAND_FLL_SB_FLL_CIC_TAU_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11 - If set to 1, it uses only 4 phases in the frequency detector. Default 8 phases"]
    #[inline(always)]
    pub fn subband_fll_sb_fll_ph_4_n8(&self) -> SUBBAND_FLL_SB_FLL_PH_4_N8_R {
        SUBBAND_FLL_SB_FLL_PH_4_N8_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Set the number of CIC samples before stopping the FLL"]
    #[inline(always)]
    pub fn subband_fll_sb_fll_wait(&self) -> SUBBAND_FLL_SB_FLL_WAIT_R {
        SUBBAND_FLL_SB_FLL_WAIT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - If set to 1 enable the sync word bias correction with RSSI detection"]
    #[inline(always)]
    pub fn sync_word_corr_en_sync_word_corr(&self) -> SYNC_WORD_CORR_EN_SYNC_WORD_CORR_R {
        SYNC_WORD_CORR_EN_SYNC_WORD_CORR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - set the sync word bias. Without the phADC rescaler, it's 8*mod_idx."]
    #[inline(always)]
    pub fn sync_word_corr_sync_word_bias(&self) -> SYNC_WORD_CORR_SYNC_WORD_BIAS_R {
        SYNC_WORD_CORR_SYNC_WORD_BIAS_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - Clock division factor for ck_div_1_6"]
    #[inline(always)]
    pub fn ck_div_1_6_ck_div_1_6(&mut self) -> CK_DIV_1_6_CK_DIV_1_6_W {
        CK_DIV_1_6_CK_DIV_1_6_W { w: self }
    }
    #[doc = "Bit 22 - If set to 1 enables the increased drive strength of the digital pads"]
    #[inline(always)]
    pub fn pads_pe_ds_gpio_ds(&mut self) -> PADS_PE_DS_GPIO_DS_W {
        PADS_PE_DS_GPIO_DS_W { w: self }
    }
    #[doc = "Bit 21 - If set to 1 enables the pull-up of the GPIO pads"]
    #[inline(always)]
    pub fn pads_pe_ds_gpio_pe(&mut self) -> PADS_PE_DS_GPIO_PE_W {
        PADS_PE_DS_GPIO_PE_W { w: self }
    }
    #[doc = "Bit 20 - If set to 1 enables the pull-up of the nreset pad"]
    #[inline(always)]
    pub fn pads_pe_ds_nreset_pe(&mut self) -> PADS_PE_DS_NRESET_PE_W {
        PADS_PE_DS_NRESET_PE_W { w: self }
    }
    #[doc = "Bit 19 - If set to 1 enables the pull-up of the MISO SPI pad"]
    #[inline(always)]
    pub fn pads_pe_ds_spi_miso_pe(&mut self) -> PADS_PE_DS_SPI_MISO_PE_W {
        PADS_PE_DS_SPI_MISO_PE_W { w: self }
    }
    #[doc = "Bit 18 - If set to 1 enables the pull-up of the MOSI SPI pad"]
    #[inline(always)]
    pub fn pads_pe_ds_spi_mosi_pe(&mut self) -> PADS_PE_DS_SPI_MOSI_PE_W {
        PADS_PE_DS_SPI_MOSI_PE_W { w: self }
    }
    #[doc = "Bit 17 - If set to 1 enables the pull-up of the SCLK SPI pad"]
    #[inline(always)]
    pub fn pads_pe_ds_spi_sclk_pe(&mut self) -> PADS_PE_DS_SPI_SCLK_PE_W {
        PADS_PE_DS_SPI_SCLK_PE_W { w: self }
    }
    #[doc = "Bit 16 - If set to 1 enables the pull-up of the CSN SPI pad"]
    #[inline(always)]
    pub fn pads_pe_ds_spi_cs_n_pe(&mut self) -> PADS_PE_DS_SPI_CS_N_PE_W {
        PADS_PE_DS_SPI_CS_N_PE_W { w: self }
    }
    #[doc = "Bits 14:15 - Select the dithering: 00 no dithering, 01 PN9 positive, 10 PN10 negative, PN9+PN10"]
    #[inline(always)]
    pub fn subband_fll_sb_fll_dither(&mut self) -> SUBBAND_FLL_SB_FLL_DITHER_W {
        SUBBAND_FLL_SB_FLL_DITHER_W { w: self }
    }
    #[doc = "Bits 12:13 - Set the CIC decimator factor: 00 => 16, 01 => 32, 10 => 64, 11 => 128"]
    #[inline(always)]
    pub fn subband_fll_sb_fll_cic_tau(&mut self) -> SUBBAND_FLL_SB_FLL_CIC_TAU_W {
        SUBBAND_FLL_SB_FLL_CIC_TAU_W { w: self }
    }
    #[doc = "Bit 11 - If set to 1, it uses only 4 phases in the frequency detector. Default 8 phases"]
    #[inline(always)]
    pub fn subband_fll_sb_fll_ph_4_n8(&mut self) -> SUBBAND_FLL_SB_FLL_PH_4_N8_W {
        SUBBAND_FLL_SB_FLL_PH_4_N8_W { w: self }
    }
    #[doc = "Bits 8:10 - Set the number of CIC samples before stopping the FLL"]
    #[inline(always)]
    pub fn subband_fll_sb_fll_wait(&mut self) -> SUBBAND_FLL_SB_FLL_WAIT_W {
        SUBBAND_FLL_SB_FLL_WAIT_W { w: self }
    }
    #[doc = "Bit 7 - If set to 1 enable the sync word bias correction with RSSI detection"]
    #[inline(always)]
    pub fn sync_word_corr_en_sync_word_corr(&mut self) -> SYNC_WORD_CORR_EN_SYNC_WORD_CORR_W {
        SYNC_WORD_CORR_EN_SYNC_WORD_CORR_W { w: self }
    }
    #[doc = "Bits 0:5 - set the sync word bias. Without the phADC rescaler, it's 8*mod_idx."]
    #[inline(always)]
    pub fn sync_word_corr_sync_word_bias(&mut self) -> SYNC_WORD_CORR_SYNC_WORD_BIAS_W {
        SYNC_WORD_CORR_SYNC_WORD_BIAS_W { w: self }
    }
}
