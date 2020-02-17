#[doc = "Reader of register FLASH_DELAY_CTRL"]
pub type R = crate::R<u32, super::FLASH_DELAY_CTRL>;
#[doc = "Writer for register FLASH_DELAY_CTRL"]
pub type W = crate::W<u32, super::FLASH_DELAY_CTRL>;
#[doc = "Register FLASH_DELAY_CTRL `reset()`'s with value 0x02"]
impl crate::ResetValue for super::FLASH_DELAY_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Flash Read access time margin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_MARGIN_A {
    #[doc = "0: Used default read margins"]
    DEFAULT_READ_MARGIN = 0,
    #[doc = "1: Used fast read margins"]
    FAST_READ_MARGIN = 1,
}
impl From<READ_MARGIN_A> for bool {
    #[inline(always)]
    fn from(variant: READ_MARGIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READ_MARGIN`"]
pub type READ_MARGIN_R = crate::R<bool, READ_MARGIN_A>;
impl READ_MARGIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_MARGIN_A {
        match self.bits {
            false => READ_MARGIN_A::DEFAULT_READ_MARGIN,
            true => READ_MARGIN_A::FAST_READ_MARGIN,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT_READ_MARGIN`"]
    #[inline(always)]
    pub fn is_default_read_margin(&self) -> bool {
        *self == READ_MARGIN_A::DEFAULT_READ_MARGIN
    }
    #[doc = "Checks if the value of the field is `FAST_READ_MARGIN`"]
    #[inline(always)]
    pub fn is_fast_read_margin(&self) -> bool {
        *self == READ_MARGIN_A::FAST_READ_MARGIN
    }
}
#[doc = "Write proxy for field `READ_MARGIN`"]
pub struct READ_MARGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_MARGIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READ_MARGIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Used default read margins"]
    #[inline(always)]
    pub fn default_read_margin(self) -> &'a mut W {
        self.variant(READ_MARGIN_A::DEFAULT_READ_MARGIN)
    }
    #[doc = "Used fast read margins"]
    #[inline(always)]
    pub fn fast_read_margin(self) -> &'a mut W {
        self.variant(READ_MARGIN_A::FAST_READ_MARGIN)
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
#[doc = "Configure Flash, memory and RF power-up delays\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCLK_FREQ_A {
    #[doc = "0: FLASH_DELAY_CTRLx set for a SYSCLK = 3 MHz"]
    FLASH_DELAY_FOR_SYSCLK_3MHZ = 0,
    #[doc = "1: FLASH_DELAY_CTRLx set for a SYSCLK = 4 MHz"]
    FLASH_DELAY_FOR_SYSCLK_4MHZ = 1,
    #[doc = "2: FLASH_DELAY_CTRLx set for a SYSCLK = 5 MHz"]
    FLASH_DELAY_FOR_SYSCLK_5MHZ = 2,
    #[doc = "3: FLASH_DELAY_CTRLx set for a SYSCLK = 8 MHz"]
    FLASH_DELAY_FOR_SYSCLK_8MHZ = 3,
    #[doc = "4: FLASH_DELAY_CTRLx set for a SYSCLK = 10 MHz"]
    FLASH_DELAY_FOR_SYSCLK_10MHZ = 4,
    #[doc = "5: FLASH_DELAY_CTRLx set for a SYSCLK = 12 MHz"]
    FLASH_DELAY_FOR_SYSCLK_12MHZ = 5,
    #[doc = "6: FLASH_DELAY_CTRLx set for a SYSCLK = 16 MHz"]
    FLASH_DELAY_FOR_SYSCLK_16MHZ = 6,
    #[doc = "7: FLASH_DELAY_CTRLx set for a SYSCLK = 20 MHz"]
    FLASH_DELAY_FOR_SYSCLK_20MHZ = 7,
    #[doc = "8: FLASH_DELAY_CTRLx set for a SYSCLK = 24 MHz"]
    FLASH_DELAY_FOR_SYSCLK_24MHZ = 8,
    #[doc = "9: FLASH_DELAY_CTRLx set for a SYSCLK = 48 MHz"]
    FLASH_DELAY_FOR_SYSCLK_48MHZ = 9,
}
impl From<SYSCLK_FREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCLK_FREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCLK_FREQ`"]
pub type SYSCLK_FREQ_R = crate::R<u8, SYSCLK_FREQ_A>;
impl SYSCLK_FREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSCLK_FREQ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_3MHZ),
            1 => Val(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_4MHZ),
            2 => Val(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_5MHZ),
            3 => Val(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_8MHZ),
            4 => Val(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_10MHZ),
            5 => Val(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_12MHZ),
            6 => Val(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_16MHZ),
            7 => Val(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_20MHZ),
            8 => Val(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_24MHZ),
            9 => Val(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_48MHZ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_DELAY_FOR_SYSCLK_3MHZ`"]
    #[inline(always)]
    pub fn is_flash_delay_for_sysclk_3mhz(&self) -> bool {
        *self == SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_3MHZ
    }
    #[doc = "Checks if the value of the field is `FLASH_DELAY_FOR_SYSCLK_4MHZ`"]
    #[inline(always)]
    pub fn is_flash_delay_for_sysclk_4mhz(&self) -> bool {
        *self == SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_4MHZ
    }
    #[doc = "Checks if the value of the field is `FLASH_DELAY_FOR_SYSCLK_5MHZ`"]
    #[inline(always)]
    pub fn is_flash_delay_for_sysclk_5mhz(&self) -> bool {
        *self == SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_5MHZ
    }
    #[doc = "Checks if the value of the field is `FLASH_DELAY_FOR_SYSCLK_8MHZ`"]
    #[inline(always)]
    pub fn is_flash_delay_for_sysclk_8mhz(&self) -> bool {
        *self == SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_8MHZ
    }
    #[doc = "Checks if the value of the field is `FLASH_DELAY_FOR_SYSCLK_10MHZ`"]
    #[inline(always)]
    pub fn is_flash_delay_for_sysclk_10mhz(&self) -> bool {
        *self == SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_10MHZ
    }
    #[doc = "Checks if the value of the field is `FLASH_DELAY_FOR_SYSCLK_12MHZ`"]
    #[inline(always)]
    pub fn is_flash_delay_for_sysclk_12mhz(&self) -> bool {
        *self == SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_12MHZ
    }
    #[doc = "Checks if the value of the field is `FLASH_DELAY_FOR_SYSCLK_16MHZ`"]
    #[inline(always)]
    pub fn is_flash_delay_for_sysclk_16mhz(&self) -> bool {
        *self == SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_16MHZ
    }
    #[doc = "Checks if the value of the field is `FLASH_DELAY_FOR_SYSCLK_20MHZ`"]
    #[inline(always)]
    pub fn is_flash_delay_for_sysclk_20mhz(&self) -> bool {
        *self == SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_20MHZ
    }
    #[doc = "Checks if the value of the field is `FLASH_DELAY_FOR_SYSCLK_24MHZ`"]
    #[inline(always)]
    pub fn is_flash_delay_for_sysclk_24mhz(&self) -> bool {
        *self == SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_24MHZ
    }
    #[doc = "Checks if the value of the field is `FLASH_DELAY_FOR_SYSCLK_48MHZ`"]
    #[inline(always)]
    pub fn is_flash_delay_for_sysclk_48mhz(&self) -> bool {
        *self == SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_48MHZ
    }
}
#[doc = "Write proxy for field `SYSCLK_FREQ`"]
pub struct SYSCLK_FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCLK_FREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCLK_FREQ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FLASH_DELAY_CTRLx set for a SYSCLK = 3 MHz"]
    #[inline(always)]
    pub fn flash_delay_for_sysclk_3mhz(self) -> &'a mut W {
        self.variant(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_3MHZ)
    }
    #[doc = "FLASH_DELAY_CTRLx set for a SYSCLK = 4 MHz"]
    #[inline(always)]
    pub fn flash_delay_for_sysclk_4mhz(self) -> &'a mut W {
        self.variant(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_4MHZ)
    }
    #[doc = "FLASH_DELAY_CTRLx set for a SYSCLK = 5 MHz"]
    #[inline(always)]
    pub fn flash_delay_for_sysclk_5mhz(self) -> &'a mut W {
        self.variant(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_5MHZ)
    }
    #[doc = "FLASH_DELAY_CTRLx set for a SYSCLK = 8 MHz"]
    #[inline(always)]
    pub fn flash_delay_for_sysclk_8mhz(self) -> &'a mut W {
        self.variant(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_8MHZ)
    }
    #[doc = "FLASH_DELAY_CTRLx set for a SYSCLK = 10 MHz"]
    #[inline(always)]
    pub fn flash_delay_for_sysclk_10mhz(self) -> &'a mut W {
        self.variant(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_10MHZ)
    }
    #[doc = "FLASH_DELAY_CTRLx set for a SYSCLK = 12 MHz"]
    #[inline(always)]
    pub fn flash_delay_for_sysclk_12mhz(self) -> &'a mut W {
        self.variant(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_12MHZ)
    }
    #[doc = "FLASH_DELAY_CTRLx set for a SYSCLK = 16 MHz"]
    #[inline(always)]
    pub fn flash_delay_for_sysclk_16mhz(self) -> &'a mut W {
        self.variant(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_16MHZ)
    }
    #[doc = "FLASH_DELAY_CTRLx set for a SYSCLK = 20 MHz"]
    #[inline(always)]
    pub fn flash_delay_for_sysclk_20mhz(self) -> &'a mut W {
        self.variant(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_20MHZ)
    }
    #[doc = "FLASH_DELAY_CTRLx set for a SYSCLK = 24 MHz"]
    #[inline(always)]
    pub fn flash_delay_for_sysclk_24mhz(self) -> &'a mut W {
        self.variant(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_24MHZ)
    }
    #[doc = "FLASH_DELAY_CTRLx set for a SYSCLK = 48 MHz"]
    #[inline(always)]
    pub fn flash_delay_for_sysclk_48mhz(self) -> &'a mut W {
        self.variant(SYSCLK_FREQ_A::FLASH_DELAY_FOR_SYSCLK_48MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Flash Read access time margin"]
    #[inline(always)]
    pub fn read_margin(&self) -> READ_MARGIN_R {
        READ_MARGIN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - Configure Flash, memory and RF power-up delays"]
    #[inline(always)]
    pub fn sysclk_freq(&self) -> SYSCLK_FREQ_R {
        SYSCLK_FREQ_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Flash Read access time margin"]
    #[inline(always)]
    pub fn read_margin(&mut self) -> READ_MARGIN_W {
        READ_MARGIN_W { w: self }
    }
    #[doc = "Bits 0:3 - Configure Flash, memory and RF power-up delays"]
    #[inline(always)]
    pub fn sysclk_freq(&mut self) -> SYSCLK_FREQ_W {
        SYSCLK_FREQ_W { w: self }
    }
}
