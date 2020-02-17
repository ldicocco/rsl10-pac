#[doc = "Reader of register ACS_WAKEUP_CFG"]
pub type R = crate::R<u32, super::ACS_WAKEUP_CFG>;
#[doc = "Writer for register ACS_WAKEUP_CFG"]
pub type W = crate::W<u32, super::ACS_WAKEUP_CFG>;
#[doc = "Register ACS_WAKEUP_CFG `reset()`'s with value 0x0005_0000"]
impl crate::ResetValue for super::ACS_WAKEUP_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0005_0000
    }
}
#[doc = "Delay from VDDC ready to digital clock enable (power of 2)\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DELAY_A {
    #[doc = "0: Wait for 1 clock cycle"]
    WAKEUP_DELAY_1 = 0,
    #[doc = "1: Wait for 2 clock cycles"]
    WAKEUP_DELAY_2 = 1,
    #[doc = "2: Wait for 4 clock cycles"]
    WAKEUP_DELAY_4 = 2,
    #[doc = "3: Wait for 8 clock cycles"]
    WAKEUP_DELAY_8 = 3,
    #[doc = "4: Wait for 16 clock cycles"]
    WAKEUP_DELAY_16 = 4,
    #[doc = "5: Wait for 32 clock cycles (typ. 10 us)"]
    WAKEUP_DELAY_32 = 5,
    #[doc = "6: Wait for 64 clock cycles"]
    WAKEUP_DELAY_64 = 6,
    #[doc = "7: Wait for 128 clock cycles"]
    WAKEUP_DELAY_128 = 7,
}
impl From<DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: DELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DELAY`"]
pub type DELAY_R = crate::R<u8, DELAY_A>;
impl DELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DELAY_A {
        match self.bits {
            0 => DELAY_A::WAKEUP_DELAY_1,
            1 => DELAY_A::WAKEUP_DELAY_2,
            2 => DELAY_A::WAKEUP_DELAY_4,
            3 => DELAY_A::WAKEUP_DELAY_8,
            4 => DELAY_A::WAKEUP_DELAY_16,
            5 => DELAY_A::WAKEUP_DELAY_32,
            6 => DELAY_A::WAKEUP_DELAY_64,
            7 => DELAY_A::WAKEUP_DELAY_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DELAY_1`"]
    #[inline(always)]
    pub fn is_wakeup_delay_1(&self) -> bool {
        *self == DELAY_A::WAKEUP_DELAY_1
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DELAY_2`"]
    #[inline(always)]
    pub fn is_wakeup_delay_2(&self) -> bool {
        *self == DELAY_A::WAKEUP_DELAY_2
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DELAY_4`"]
    #[inline(always)]
    pub fn is_wakeup_delay_4(&self) -> bool {
        *self == DELAY_A::WAKEUP_DELAY_4
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DELAY_8`"]
    #[inline(always)]
    pub fn is_wakeup_delay_8(&self) -> bool {
        *self == DELAY_A::WAKEUP_DELAY_8
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DELAY_16`"]
    #[inline(always)]
    pub fn is_wakeup_delay_16(&self) -> bool {
        *self == DELAY_A::WAKEUP_DELAY_16
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DELAY_32`"]
    #[inline(always)]
    pub fn is_wakeup_delay_32(&self) -> bool {
        *self == DELAY_A::WAKEUP_DELAY_32
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DELAY_64`"]
    #[inline(always)]
    pub fn is_wakeup_delay_64(&self) -> bool {
        *self == DELAY_A::WAKEUP_DELAY_64
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DELAY_128`"]
    #[inline(always)]
    pub fn is_wakeup_delay_128(&self) -> bool {
        *self == DELAY_A::WAKEUP_DELAY_128
    }
}
#[doc = "Write proxy for field `DELAY`"]
pub struct DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DELAY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Wait for 1 clock cycle"]
    #[inline(always)]
    pub fn wakeup_delay_1(self) -> &'a mut W {
        self.variant(DELAY_A::WAKEUP_DELAY_1)
    }
    #[doc = "Wait for 2 clock cycles"]
    #[inline(always)]
    pub fn wakeup_delay_2(self) -> &'a mut W {
        self.variant(DELAY_A::WAKEUP_DELAY_2)
    }
    #[doc = "Wait for 4 clock cycles"]
    #[inline(always)]
    pub fn wakeup_delay_4(self) -> &'a mut W {
        self.variant(DELAY_A::WAKEUP_DELAY_4)
    }
    #[doc = "Wait for 8 clock cycles"]
    #[inline(always)]
    pub fn wakeup_delay_8(self) -> &'a mut W {
        self.variant(DELAY_A::WAKEUP_DELAY_8)
    }
    #[doc = "Wait for 16 clock cycles"]
    #[inline(always)]
    pub fn wakeup_delay_16(self) -> &'a mut W {
        self.variant(DELAY_A::WAKEUP_DELAY_16)
    }
    #[doc = "Wait for 32 clock cycles (typ. 10 us)"]
    #[inline(always)]
    pub fn wakeup_delay_32(self) -> &'a mut W {
        self.variant(DELAY_A::WAKEUP_DELAY_32)
    }
    #[doc = "Wait for 64 clock cycles"]
    #[inline(always)]
    pub fn wakeup_delay_64(self) -> &'a mut W {
        self.variant(DELAY_A::WAKEUP_DELAY_64)
    }
    #[doc = "Wait for 128 clock cycles"]
    #[inline(always)]
    pub fn wakeup_delay_128(self) -> &'a mut W {
        self.variant(DELAY_A::WAKEUP_DELAY_128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Enable / Disable the Wake-up functionality on the DCDC overload flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_OVERLOAD_EN_A {
    #[doc = "0: Disable the Wake-up functionality on the DCDC overload flag"]
    WAKEUP_DCDC_OVERLOAD_DISABLE = 0,
    #[doc = "1: Enable the Wake-up functionality on the DCDC overload flag"]
    WAKEUP_DCDC_OVERLOAD_ENABLE = 1,
}
impl From<DCDC_OVERLOAD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DCDC_OVERLOAD_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCDC_OVERLOAD_EN`"]
pub type DCDC_OVERLOAD_EN_R = crate::R<bool, DCDC_OVERLOAD_EN_A>;
impl DCDC_OVERLOAD_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDC_OVERLOAD_EN_A {
        match self.bits {
            false => DCDC_OVERLOAD_EN_A::WAKEUP_DCDC_OVERLOAD_DISABLE,
            true => DCDC_OVERLOAD_EN_A::WAKEUP_DCDC_OVERLOAD_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DCDC_OVERLOAD_DISABLE`"]
    #[inline(always)]
    pub fn is_wakeup_dcdc_overload_disable(&self) -> bool {
        *self == DCDC_OVERLOAD_EN_A::WAKEUP_DCDC_OVERLOAD_DISABLE
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DCDC_OVERLOAD_ENABLE`"]
    #[inline(always)]
    pub fn is_wakeup_dcdc_overload_enable(&self) -> bool {
        *self == DCDC_OVERLOAD_EN_A::WAKEUP_DCDC_OVERLOAD_ENABLE
    }
}
#[doc = "Write proxy for field `DCDC_OVERLOAD_EN`"]
pub struct DCDC_OVERLOAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_OVERLOAD_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCDC_OVERLOAD_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the Wake-up functionality on the DCDC overload flag"]
    #[inline(always)]
    pub fn wakeup_dcdc_overload_disable(self) -> &'a mut W {
        self.variant(DCDC_OVERLOAD_EN_A::WAKEUP_DCDC_OVERLOAD_DISABLE)
    }
    #[doc = "Enable the Wake-up functionality on the DCDC overload flag"]
    #[inline(always)]
    pub fn wakeup_dcdc_overload_enable(self) -> &'a mut W {
        self.variant(DCDC_OVERLOAD_EN_A::WAKEUP_DCDC_OVERLOAD_ENABLE)
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
#[doc = "Wake-up polarity on the WAKEUP pad\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_PAD_POL_A {
    #[doc = "0: Wake-up on the WAKEUP pad rising edge & enable pull-down"]
    WAKEUP_WAKEUP_PAD_RISING = 0,
    #[doc = "1: Wake-up on the WAKEUP pad falling edge & enable pull-up"]
    WAKEUP_WAKEUP_PAD_FALLING = 1,
}
impl From<WAKEUP_PAD_POL_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_PAD_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAKEUP_PAD_POL`"]
pub type WAKEUP_PAD_POL_R = crate::R<bool, WAKEUP_PAD_POL_A>;
impl WAKEUP_PAD_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP_PAD_POL_A {
        match self.bits {
            false => WAKEUP_PAD_POL_A::WAKEUP_WAKEUP_PAD_RISING,
            true => WAKEUP_PAD_POL_A::WAKEUP_WAKEUP_PAD_FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_WAKEUP_PAD_RISING`"]
    #[inline(always)]
    pub fn is_wakeup_wakeup_pad_rising(&self) -> bool {
        *self == WAKEUP_PAD_POL_A::WAKEUP_WAKEUP_PAD_RISING
    }
    #[doc = "Checks if the value of the field is `WAKEUP_WAKEUP_PAD_FALLING`"]
    #[inline(always)]
    pub fn is_wakeup_wakeup_pad_falling(&self) -> bool {
        *self == WAKEUP_PAD_POL_A::WAKEUP_WAKEUP_PAD_FALLING
    }
}
#[doc = "Write proxy for field `WAKEUP_PAD_POL`"]
pub struct WAKEUP_PAD_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_PAD_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUP_PAD_POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up on the WAKEUP pad rising edge & enable pull-down"]
    #[inline(always)]
    pub fn wakeup_wakeup_pad_rising(self) -> &'a mut W {
        self.variant(WAKEUP_PAD_POL_A::WAKEUP_WAKEUP_PAD_RISING)
    }
    #[doc = "Wake-up on the WAKEUP pad falling edge & enable pull-up"]
    #[inline(always)]
    pub fn wakeup_wakeup_pad_falling(self) -> &'a mut W {
        self.variant(WAKEUP_PAD_POL_A::WAKEUP_WAKEUP_PAD_FALLING)
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
#[doc = "Wake-up polarity on the DIO3 pad\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIO3_POL_A {
    #[doc = "0: Wake-up on the DIO3 rising edge"]
    WAKEUP_DIO3_RISING = 0,
    #[doc = "1: Wake-up on the DIO3 falling edge"]
    WAKEUP_DIO3_FALLING = 1,
}
impl From<DIO3_POL_A> for bool {
    #[inline(always)]
    fn from(variant: DIO3_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIO3_POL`"]
pub type DIO3_POL_R = crate::R<bool, DIO3_POL_A>;
impl DIO3_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIO3_POL_A {
        match self.bits {
            false => DIO3_POL_A::WAKEUP_DIO3_RISING,
            true => DIO3_POL_A::WAKEUP_DIO3_FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO3_RISING`"]
    #[inline(always)]
    pub fn is_wakeup_dio3_rising(&self) -> bool {
        *self == DIO3_POL_A::WAKEUP_DIO3_RISING
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO3_FALLING`"]
    #[inline(always)]
    pub fn is_wakeup_dio3_falling(&self) -> bool {
        *self == DIO3_POL_A::WAKEUP_DIO3_FALLING
    }
}
#[doc = "Write proxy for field `DIO3_POL`"]
pub struct DIO3_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO3_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIO3_POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up on the DIO3 rising edge"]
    #[inline(always)]
    pub fn wakeup_dio3_rising(self) -> &'a mut W {
        self.variant(DIO3_POL_A::WAKEUP_DIO3_RISING)
    }
    #[doc = "Wake-up on the DIO3 falling edge"]
    #[inline(always)]
    pub fn wakeup_dio3_falling(self) -> &'a mut W {
        self.variant(DIO3_POL_A::WAKEUP_DIO3_FALLING)
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
#[doc = "Wake-up polarity on the DIO2 pad\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIO2_POL_A {
    #[doc = "0: Wake-up on the DIO2 rising edge"]
    WAKEUP_DIO2_RISING = 0,
    #[doc = "1: Wake-up on the DIO2 falling edge"]
    WAKEUP_DIO2_FALLING = 1,
}
impl From<DIO2_POL_A> for bool {
    #[inline(always)]
    fn from(variant: DIO2_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIO2_POL`"]
pub type DIO2_POL_R = crate::R<bool, DIO2_POL_A>;
impl DIO2_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIO2_POL_A {
        match self.bits {
            false => DIO2_POL_A::WAKEUP_DIO2_RISING,
            true => DIO2_POL_A::WAKEUP_DIO2_FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO2_RISING`"]
    #[inline(always)]
    pub fn is_wakeup_dio2_rising(&self) -> bool {
        *self == DIO2_POL_A::WAKEUP_DIO2_RISING
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO2_FALLING`"]
    #[inline(always)]
    pub fn is_wakeup_dio2_falling(&self) -> bool {
        *self == DIO2_POL_A::WAKEUP_DIO2_FALLING
    }
}
#[doc = "Write proxy for field `DIO2_POL`"]
pub struct DIO2_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO2_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIO2_POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up on the DIO2 rising edge"]
    #[inline(always)]
    pub fn wakeup_dio2_rising(self) -> &'a mut W {
        self.variant(DIO2_POL_A::WAKEUP_DIO2_RISING)
    }
    #[doc = "Wake-up on the DIO2 falling edge"]
    #[inline(always)]
    pub fn wakeup_dio2_falling(self) -> &'a mut W {
        self.variant(DIO2_POL_A::WAKEUP_DIO2_FALLING)
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
#[doc = "Wake-up polarity on the DIO1 pad\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIO1_POL_A {
    #[doc = "0: Wake-up on the DIO1 rising edge"]
    WAKEUP_DIO1_RISING = 0,
    #[doc = "1: Wake-up on the DIO1 falling edge"]
    WAKEUP_DIO1_FALLING = 1,
}
impl From<DIO1_POL_A> for bool {
    #[inline(always)]
    fn from(variant: DIO1_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIO1_POL`"]
pub type DIO1_POL_R = crate::R<bool, DIO1_POL_A>;
impl DIO1_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIO1_POL_A {
        match self.bits {
            false => DIO1_POL_A::WAKEUP_DIO1_RISING,
            true => DIO1_POL_A::WAKEUP_DIO1_FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO1_RISING`"]
    #[inline(always)]
    pub fn is_wakeup_dio1_rising(&self) -> bool {
        *self == DIO1_POL_A::WAKEUP_DIO1_RISING
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO1_FALLING`"]
    #[inline(always)]
    pub fn is_wakeup_dio1_falling(&self) -> bool {
        *self == DIO1_POL_A::WAKEUP_DIO1_FALLING
    }
}
#[doc = "Write proxy for field `DIO1_POL`"]
pub struct DIO1_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO1_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIO1_POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up on the DIO1 rising edge"]
    #[inline(always)]
    pub fn wakeup_dio1_rising(self) -> &'a mut W {
        self.variant(DIO1_POL_A::WAKEUP_DIO1_RISING)
    }
    #[doc = "Wake-up on the DIO1 falling edge"]
    #[inline(always)]
    pub fn wakeup_dio1_falling(self) -> &'a mut W {
        self.variant(DIO1_POL_A::WAKEUP_DIO1_FALLING)
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
#[doc = "Wake-up polarity on the DIO0 pad\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIO0_POL_A {
    #[doc = "0: Wake-up on the DIO0 rising edge"]
    WAKEUP_DIO0_RISING = 0,
    #[doc = "1: Wake-up on the DIO0 falling edge"]
    WAKEUP_DIO0_FALLING = 1,
}
impl From<DIO0_POL_A> for bool {
    #[inline(always)]
    fn from(variant: DIO0_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIO0_POL`"]
pub type DIO0_POL_R = crate::R<bool, DIO0_POL_A>;
impl DIO0_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIO0_POL_A {
        match self.bits {
            false => DIO0_POL_A::WAKEUP_DIO0_RISING,
            true => DIO0_POL_A::WAKEUP_DIO0_FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO0_RISING`"]
    #[inline(always)]
    pub fn is_wakeup_dio0_rising(&self) -> bool {
        *self == DIO0_POL_A::WAKEUP_DIO0_RISING
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO0_FALLING`"]
    #[inline(always)]
    pub fn is_wakeup_dio0_falling(&self) -> bool {
        *self == DIO0_POL_A::WAKEUP_DIO0_FALLING
    }
}
#[doc = "Write proxy for field `DIO0_POL`"]
pub struct DIO0_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO0_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIO0_POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up on the DIO0 rising edge"]
    #[inline(always)]
    pub fn wakeup_dio0_rising(self) -> &'a mut W {
        self.variant(DIO0_POL_A::WAKEUP_DIO0_RISING)
    }
    #[doc = "Wake-up on the DIO0 falling edge"]
    #[inline(always)]
    pub fn wakeup_dio0_falling(self) -> &'a mut W {
        self.variant(DIO0_POL_A::WAKEUP_DIO0_FALLING)
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
#[doc = "Enable / Disable the Wake-up functionality on the DIO3 pad\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIO3_EN_A {
    #[doc = "0: Disable the Wake-up functionality on the DIO3 pad"]
    WAKEUP_DIO3_DISABLE = 0,
    #[doc = "1: Enable the Wake-up functionality on the DIO3 pad"]
    WAKEUP_DIO3_ENABLE = 1,
}
impl From<DIO3_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DIO3_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIO3_EN`"]
pub type DIO3_EN_R = crate::R<bool, DIO3_EN_A>;
impl DIO3_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIO3_EN_A {
        match self.bits {
            false => DIO3_EN_A::WAKEUP_DIO3_DISABLE,
            true => DIO3_EN_A::WAKEUP_DIO3_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO3_DISABLE`"]
    #[inline(always)]
    pub fn is_wakeup_dio3_disable(&self) -> bool {
        *self == DIO3_EN_A::WAKEUP_DIO3_DISABLE
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO3_ENABLE`"]
    #[inline(always)]
    pub fn is_wakeup_dio3_enable(&self) -> bool {
        *self == DIO3_EN_A::WAKEUP_DIO3_ENABLE
    }
}
#[doc = "Write proxy for field `DIO3_EN`"]
pub struct DIO3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO3_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIO3_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the Wake-up functionality on the DIO3 pad"]
    #[inline(always)]
    pub fn wakeup_dio3_disable(self) -> &'a mut W {
        self.variant(DIO3_EN_A::WAKEUP_DIO3_DISABLE)
    }
    #[doc = "Enable the Wake-up functionality on the DIO3 pad"]
    #[inline(always)]
    pub fn wakeup_dio3_enable(self) -> &'a mut W {
        self.variant(DIO3_EN_A::WAKEUP_DIO3_ENABLE)
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
#[doc = "Enable / Disable the Wake-up functionality on the DIO2 pad\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIO2_EN_A {
    #[doc = "0: Disable the Wake-up functionality on the DIO2 pad"]
    WAKEUP_DIO2_DISABLE = 0,
    #[doc = "1: Enable the Wake-up functionality on the DIO2 pad"]
    WAKEUP_DIO2_ENABLE = 1,
}
impl From<DIO2_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DIO2_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIO2_EN`"]
pub type DIO2_EN_R = crate::R<bool, DIO2_EN_A>;
impl DIO2_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIO2_EN_A {
        match self.bits {
            false => DIO2_EN_A::WAKEUP_DIO2_DISABLE,
            true => DIO2_EN_A::WAKEUP_DIO2_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO2_DISABLE`"]
    #[inline(always)]
    pub fn is_wakeup_dio2_disable(&self) -> bool {
        *self == DIO2_EN_A::WAKEUP_DIO2_DISABLE
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO2_ENABLE`"]
    #[inline(always)]
    pub fn is_wakeup_dio2_enable(&self) -> bool {
        *self == DIO2_EN_A::WAKEUP_DIO2_ENABLE
    }
}
#[doc = "Write proxy for field `DIO2_EN`"]
pub struct DIO2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO2_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIO2_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the Wake-up functionality on the DIO2 pad"]
    #[inline(always)]
    pub fn wakeup_dio2_disable(self) -> &'a mut W {
        self.variant(DIO2_EN_A::WAKEUP_DIO2_DISABLE)
    }
    #[doc = "Enable the Wake-up functionality on the DIO2 pad"]
    #[inline(always)]
    pub fn wakeup_dio2_enable(self) -> &'a mut W {
        self.variant(DIO2_EN_A::WAKEUP_DIO2_ENABLE)
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
#[doc = "Enable / Disable the Wake-up functionality on the DIO1 pad\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIO1_EN_A {
    #[doc = "0: Disable the Wake-up functionality on the DIO1 pad"]
    WAKEUP_DIO1_DISABLE = 0,
    #[doc = "1: Enable the Wake-up functionality on the DIO1 pad"]
    WAKEUP_DIO1_ENABLE = 1,
}
impl From<DIO1_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DIO1_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIO1_EN`"]
pub type DIO1_EN_R = crate::R<bool, DIO1_EN_A>;
impl DIO1_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIO1_EN_A {
        match self.bits {
            false => DIO1_EN_A::WAKEUP_DIO1_DISABLE,
            true => DIO1_EN_A::WAKEUP_DIO1_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO1_DISABLE`"]
    #[inline(always)]
    pub fn is_wakeup_dio1_disable(&self) -> bool {
        *self == DIO1_EN_A::WAKEUP_DIO1_DISABLE
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO1_ENABLE`"]
    #[inline(always)]
    pub fn is_wakeup_dio1_enable(&self) -> bool {
        *self == DIO1_EN_A::WAKEUP_DIO1_ENABLE
    }
}
#[doc = "Write proxy for field `DIO1_EN`"]
pub struct DIO1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO1_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIO1_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the Wake-up functionality on the DIO1 pad"]
    #[inline(always)]
    pub fn wakeup_dio1_disable(self) -> &'a mut W {
        self.variant(DIO1_EN_A::WAKEUP_DIO1_DISABLE)
    }
    #[doc = "Enable the Wake-up functionality on the DIO1 pad"]
    #[inline(always)]
    pub fn wakeup_dio1_enable(self) -> &'a mut W {
        self.variant(DIO1_EN_A::WAKEUP_DIO1_ENABLE)
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
#[doc = "Enable / Disable the Wake-up functionality on the DIO0 pad\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIO0_EN_A {
    #[doc = "0: Disable the Wake-up functionality on the DIO0 pad"]
    WAKEUP_DIO0_DISABLE = 0,
    #[doc = "1: Enable the Wake-up functionality on the DIO0 pad"]
    WAKEUP_DIO0_ENABLE = 1,
}
impl From<DIO0_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DIO0_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIO0_EN`"]
pub type DIO0_EN_R = crate::R<bool, DIO0_EN_A>;
impl DIO0_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIO0_EN_A {
        match self.bits {
            false => DIO0_EN_A::WAKEUP_DIO0_DISABLE,
            true => DIO0_EN_A::WAKEUP_DIO0_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO0_DISABLE`"]
    #[inline(always)]
    pub fn is_wakeup_dio0_disable(&self) -> bool {
        *self == DIO0_EN_A::WAKEUP_DIO0_DISABLE
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DIO0_ENABLE`"]
    #[inline(always)]
    pub fn is_wakeup_dio0_enable(&self) -> bool {
        *self == DIO0_EN_A::WAKEUP_DIO0_ENABLE
    }
}
#[doc = "Write proxy for field `DIO0_EN`"]
pub struct DIO0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO0_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIO0_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the Wake-up functionality on the DIO0 pad"]
    #[inline(always)]
    pub fn wakeup_dio0_disable(self) -> &'a mut W {
        self.variant(DIO0_EN_A::WAKEUP_DIO0_DISABLE)
    }
    #[doc = "Enable the Wake-up functionality on the DIO0 pad"]
    #[inline(always)]
    pub fn wakeup_dio0_enable(self) -> &'a mut W {
        self.variant(DIO0_EN_A::WAKEUP_DIO0_ENABLE)
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
    #[doc = "Bits 16:18 - Delay from VDDC ready to digital clock enable (power of 2)"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 9 - Enable / Disable the Wake-up functionality on the DCDC overload flag"]
    #[inline(always)]
    pub fn dcdc_overload_en(&self) -> DCDC_OVERLOAD_EN_R {
        DCDC_OVERLOAD_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Wake-up polarity on the WAKEUP pad"]
    #[inline(always)]
    pub fn wakeup_pad_pol(&self) -> WAKEUP_PAD_POL_R {
        WAKEUP_PAD_POL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wake-up polarity on the DIO3 pad"]
    #[inline(always)]
    pub fn dio3_pol(&self) -> DIO3_POL_R {
        DIO3_POL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wake-up polarity on the DIO2 pad"]
    #[inline(always)]
    pub fn dio2_pol(&self) -> DIO2_POL_R {
        DIO2_POL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wake-up polarity on the DIO1 pad"]
    #[inline(always)]
    pub fn dio1_pol(&self) -> DIO1_POL_R {
        DIO1_POL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wake-up polarity on the DIO0 pad"]
    #[inline(always)]
    pub fn dio0_pol(&self) -> DIO0_POL_R {
        DIO0_POL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable / Disable the Wake-up functionality on the DIO3 pad"]
    #[inline(always)]
    pub fn dio3_en(&self) -> DIO3_EN_R {
        DIO3_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable / Disable the Wake-up functionality on the DIO2 pad"]
    #[inline(always)]
    pub fn dio2_en(&self) -> DIO2_EN_R {
        DIO2_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable / Disable the Wake-up functionality on the DIO1 pad"]
    #[inline(always)]
    pub fn dio1_en(&self) -> DIO1_EN_R {
        DIO1_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable / Disable the Wake-up functionality on the DIO0 pad"]
    #[inline(always)]
    pub fn dio0_en(&self) -> DIO0_EN_R {
        DIO0_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:18 - Delay from VDDC ready to digital clock enable (power of 2)"]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W {
        DELAY_W { w: self }
    }
    #[doc = "Bit 9 - Enable / Disable the Wake-up functionality on the DCDC overload flag"]
    #[inline(always)]
    pub fn dcdc_overload_en(&mut self) -> DCDC_OVERLOAD_EN_W {
        DCDC_OVERLOAD_EN_W { w: self }
    }
    #[doc = "Bit 8 - Wake-up polarity on the WAKEUP pad"]
    #[inline(always)]
    pub fn wakeup_pad_pol(&mut self) -> WAKEUP_PAD_POL_W {
        WAKEUP_PAD_POL_W { w: self }
    }
    #[doc = "Bit 7 - Wake-up polarity on the DIO3 pad"]
    #[inline(always)]
    pub fn dio3_pol(&mut self) -> DIO3_POL_W {
        DIO3_POL_W { w: self }
    }
    #[doc = "Bit 6 - Wake-up polarity on the DIO2 pad"]
    #[inline(always)]
    pub fn dio2_pol(&mut self) -> DIO2_POL_W {
        DIO2_POL_W { w: self }
    }
    #[doc = "Bit 5 - Wake-up polarity on the DIO1 pad"]
    #[inline(always)]
    pub fn dio1_pol(&mut self) -> DIO1_POL_W {
        DIO1_POL_W { w: self }
    }
    #[doc = "Bit 4 - Wake-up polarity on the DIO0 pad"]
    #[inline(always)]
    pub fn dio0_pol(&mut self) -> DIO0_POL_W {
        DIO0_POL_W { w: self }
    }
    #[doc = "Bit 3 - Enable / Disable the Wake-up functionality on the DIO3 pad"]
    #[inline(always)]
    pub fn dio3_en(&mut self) -> DIO3_EN_W {
        DIO3_EN_W { w: self }
    }
    #[doc = "Bit 2 - Enable / Disable the Wake-up functionality on the DIO2 pad"]
    #[inline(always)]
    pub fn dio2_en(&mut self) -> DIO2_EN_W {
        DIO2_EN_W { w: self }
    }
    #[doc = "Bit 1 - Enable / Disable the Wake-up functionality on the DIO1 pad"]
    #[inline(always)]
    pub fn dio1_en(&mut self) -> DIO1_EN_W {
        DIO1_EN_W { w: self }
    }
    #[doc = "Bit 0 - Enable / Disable the Wake-up functionality on the DIO0 pad"]
    #[inline(always)]
    pub fn dio0_en(&mut self) -> DIO0_EN_W {
        DIO0_EN_W { w: self }
    }
}
