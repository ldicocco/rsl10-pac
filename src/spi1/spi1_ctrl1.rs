#[doc = "Reader of register SPI1_CTRL1"]
pub type R = crate::R<u32, super::SPI1_CTRL1>;
#[doc = "Writer for register SPI1_CTRL1"]
pub type W = crate::W<u32, super::SPI1_CTRL1>;
#[doc = "Register SPI1_CTRL1 `reset()`'s with value 0x20"]
impl crate::ResetValue for super::SPI1_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Start an SPI data transfer and indicate if a transfer is in progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_START_BUSY_A {
    #[doc = "0: Stop a transfer or indicate that the SPI interface is idle"]
    SPI1_IDLE = 0,
    #[doc = "1: Start a transfer on the SPI interface (master mode only)"]
    SPI1_START = 1,
}
impl From<SPI1_START_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_START_BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI1_START_BUSY`"]
pub type SPI1_START_BUSY_R = crate::R<bool, SPI1_START_BUSY_A>;
impl SPI1_START_BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_START_BUSY_A {
        match self.bits {
            false => SPI1_START_BUSY_A::SPI1_IDLE,
            true => SPI1_START_BUSY_A::SPI1_START,
        }
    }
    #[doc = "Checks if the value of the field is `SPI1_IDLE`"]
    #[inline(always)]
    pub fn is_spi1_idle(&self) -> bool {
        *self == SPI1_START_BUSY_A::SPI1_IDLE
    }
    #[doc = "Checks if the value of the field is `SPI1_START`"]
    #[inline(always)]
    pub fn is_spi1_start(&self) -> bool {
        *self == SPI1_START_BUSY_A::SPI1_START
    }
}
#[doc = "Write proxy for field `SPI1_START_BUSY`"]
pub struct SPI1_START_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_START_BUSY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_START_BUSY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop a transfer or indicate that the SPI interface is idle"]
    #[inline(always)]
    pub fn spi1_idle(self) -> &'a mut W {
        self.variant(SPI1_START_BUSY_A::SPI1_IDLE)
    }
    #[doc = "Start a transfer on the SPI interface (master mode only)"]
    #[inline(always)]
    pub fn spi1_start(self) -> &'a mut W {
        self.variant(SPI1_START_BUSY_A::SPI1_START)
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
#[doc = "Issue a read command or write command to the SPI interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPI1_RW_CMD_A {
    #[doc = "0: No operation"]
    SPI1_NOP = 0,
    #[doc = "1: Write data using the SPI interface"]
    SPI1_WRITE_DATA = 1,
    #[doc = "2: Read data using the SPI interface"]
    SPI1_READ_DATA = 2,
    #[doc = "3: Read and write data using the SPI interface"]
    SPI1_RW_DATA = 3,
}
impl From<SPI1_RW_CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI1_RW_CMD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPI1_RW_CMD`"]
pub type SPI1_RW_CMD_R = crate::R<u8, SPI1_RW_CMD_A>;
impl SPI1_RW_CMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_RW_CMD_A {
        match self.bits {
            0 => SPI1_RW_CMD_A::SPI1_NOP,
            1 => SPI1_RW_CMD_A::SPI1_WRITE_DATA,
            2 => SPI1_RW_CMD_A::SPI1_READ_DATA,
            3 => SPI1_RW_CMD_A::SPI1_RW_DATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SPI1_NOP`"]
    #[inline(always)]
    pub fn is_spi1_nop(&self) -> bool {
        *self == SPI1_RW_CMD_A::SPI1_NOP
    }
    #[doc = "Checks if the value of the field is `SPI1_WRITE_DATA`"]
    #[inline(always)]
    pub fn is_spi1_write_data(&self) -> bool {
        *self == SPI1_RW_CMD_A::SPI1_WRITE_DATA
    }
    #[doc = "Checks if the value of the field is `SPI1_READ_DATA`"]
    #[inline(always)]
    pub fn is_spi1_read_data(&self) -> bool {
        *self == SPI1_RW_CMD_A::SPI1_READ_DATA
    }
    #[doc = "Checks if the value of the field is `SPI1_RW_DATA`"]
    #[inline(always)]
    pub fn is_spi1_rw_data(&self) -> bool {
        *self == SPI1_RW_CMD_A::SPI1_RW_DATA
    }
}
#[doc = "Write proxy for field `SPI1_RW_CMD`"]
pub struct SPI1_RW_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_RW_CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_RW_CMD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn spi1_nop(self) -> &'a mut W {
        self.variant(SPI1_RW_CMD_A::SPI1_NOP)
    }
    #[doc = "Write data using the SPI interface"]
    #[inline(always)]
    pub fn spi1_write_data(self) -> &'a mut W {
        self.variant(SPI1_RW_CMD_A::SPI1_WRITE_DATA)
    }
    #[doc = "Read data using the SPI interface"]
    #[inline(always)]
    pub fn spi1_read_data(self) -> &'a mut W {
        self.variant(SPI1_RW_CMD_A::SPI1_READ_DATA)
    }
    #[doc = "Read and write data using the SPI interface"]
    #[inline(always)]
    pub fn spi1_rw_data(self) -> &'a mut W {
        self.variant(SPI1_RW_CMD_A::SPI1_RW_DATA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Set the chip-select line for SPI (master mode), read the chip-select line for SPI (slave mode)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_CS_A {
    #[doc = "0: Set the SPI CS signal low"]
    SPI1_CS_0 = 0,
    #[doc = "1: Set the SPI CS signal high"]
    SPI1_CS_1 = 1,
}
impl From<SPI1_CS_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_CS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI1_CS`"]
pub type SPI1_CS_R = crate::R<bool, SPI1_CS_A>;
impl SPI1_CS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_CS_A {
        match self.bits {
            false => SPI1_CS_A::SPI1_CS_0,
            true => SPI1_CS_A::SPI1_CS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SPI1_CS_0`"]
    #[inline(always)]
    pub fn is_spi1_cs_0(&self) -> bool {
        *self == SPI1_CS_A::SPI1_CS_0
    }
    #[doc = "Checks if the value of the field is `SPI1_CS_1`"]
    #[inline(always)]
    pub fn is_spi1_cs_1(&self) -> bool {
        *self == SPI1_CS_A::SPI1_CS_1
    }
}
#[doc = "Write proxy for field `SPI1_CS`"]
pub struct SPI1_CS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_CS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_CS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set the SPI CS signal low"]
    #[inline(always)]
    pub fn spi1_cs_0(self) -> &'a mut W {
        self.variant(SPI1_CS_A::SPI1_CS_0)
    }
    #[doc = "Set the SPI CS signal high"]
    #[inline(always)]
    pub fn spi1_cs_1(self) -> &'a mut W {
        self.variant(SPI1_CS_A::SPI1_CS_1)
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
#[doc = "Select the word size used by the SPI interface (word size = SPI1_WORD_SIZE + 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPI1_WORD_SIZE_A {
    #[doc = "0: SPI transfers use 1-bit words"]
    SPI1_WORD_SIZE_1 = 0,
    #[doc = "7: SPI transfers use 8-bit words"]
    SPI1_WORD_SIZE_8 = 7,
    #[doc = "15: SPI transfers use 16-bit words"]
    SPI1_WORD_SIZE_16 = 15,
    #[doc = "23: SPI transfers use 24-bit words"]
    SPI1_WORD_SIZE_24 = 23,
    #[doc = "31: SPI transfers use 32-bit words"]
    SPI1_WORD_SIZE_32 = 31,
}
impl From<SPI1_WORD_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI1_WORD_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPI1_WORD_SIZE`"]
pub type SPI1_WORD_SIZE_R = crate::R<u8, SPI1_WORD_SIZE_A>;
impl SPI1_WORD_SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPI1_WORD_SIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SPI1_WORD_SIZE_A::SPI1_WORD_SIZE_1),
            7 => Val(SPI1_WORD_SIZE_A::SPI1_WORD_SIZE_8),
            15 => Val(SPI1_WORD_SIZE_A::SPI1_WORD_SIZE_16),
            23 => Val(SPI1_WORD_SIZE_A::SPI1_WORD_SIZE_24),
            31 => Val(SPI1_WORD_SIZE_A::SPI1_WORD_SIZE_32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SPI1_WORD_SIZE_1`"]
    #[inline(always)]
    pub fn is_spi1_word_size_1(&self) -> bool {
        *self == SPI1_WORD_SIZE_A::SPI1_WORD_SIZE_1
    }
    #[doc = "Checks if the value of the field is `SPI1_WORD_SIZE_8`"]
    #[inline(always)]
    pub fn is_spi1_word_size_8(&self) -> bool {
        *self == SPI1_WORD_SIZE_A::SPI1_WORD_SIZE_8
    }
    #[doc = "Checks if the value of the field is `SPI1_WORD_SIZE_16`"]
    #[inline(always)]
    pub fn is_spi1_word_size_16(&self) -> bool {
        *self == SPI1_WORD_SIZE_A::SPI1_WORD_SIZE_16
    }
    #[doc = "Checks if the value of the field is `SPI1_WORD_SIZE_24`"]
    #[inline(always)]
    pub fn is_spi1_word_size_24(&self) -> bool {
        *self == SPI1_WORD_SIZE_A::SPI1_WORD_SIZE_24
    }
    #[doc = "Checks if the value of the field is `SPI1_WORD_SIZE_32`"]
    #[inline(always)]
    pub fn is_spi1_word_size_32(&self) -> bool {
        *self == SPI1_WORD_SIZE_A::SPI1_WORD_SIZE_32
    }
}
#[doc = "Write proxy for field `SPI1_WORD_SIZE`"]
pub struct SPI1_WORD_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_WORD_SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_WORD_SIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SPI transfers use 1-bit words"]
    #[inline(always)]
    pub fn spi1_word_size_1(self) -> &'a mut W {
        self.variant(SPI1_WORD_SIZE_A::SPI1_WORD_SIZE_1)
    }
    #[doc = "SPI transfers use 8-bit words"]
    #[inline(always)]
    pub fn spi1_word_size_8(self) -> &'a mut W {
        self.variant(SPI1_WORD_SIZE_A::SPI1_WORD_SIZE_8)
    }
    #[doc = "SPI transfers use 16-bit words"]
    #[inline(always)]
    pub fn spi1_word_size_16(self) -> &'a mut W {
        self.variant(SPI1_WORD_SIZE_A::SPI1_WORD_SIZE_16)
    }
    #[doc = "SPI transfers use 24-bit words"]
    #[inline(always)]
    pub fn spi1_word_size_24(self) -> &'a mut W {
        self.variant(SPI1_WORD_SIZE_A::SPI1_WORD_SIZE_24)
    }
    #[doc = "SPI transfers use 32-bit words"]
    #[inline(always)]
    pub fn spi1_word_size_32(self) -> &'a mut W {
        self.variant(SPI1_WORD_SIZE_A::SPI1_WORD_SIZE_32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - Start an SPI data transfer and indicate if a transfer is in progress"]
    #[inline(always)]
    pub fn spi1_start_busy(&self) -> SPI1_START_BUSY_R {
        SPI1_START_BUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Issue a read command or write command to the SPI interface"]
    #[inline(always)]
    pub fn spi1_rw_cmd(&self) -> SPI1_RW_CMD_R {
        SPI1_RW_CMD_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Set the chip-select line for SPI (master mode), read the chip-select line for SPI (slave mode)"]
    #[inline(always)]
    pub fn spi1_cs(&self) -> SPI1_CS_R {
        SPI1_CS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - Select the word size used by the SPI interface (word size = SPI1_WORD_SIZE + 1)"]
    #[inline(always)]
    pub fn spi1_word_size(&self) -> SPI1_WORD_SIZE_R {
        SPI1_WORD_SIZE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Start an SPI data transfer and indicate if a transfer is in progress"]
    #[inline(always)]
    pub fn spi1_start_busy(&mut self) -> SPI1_START_BUSY_W {
        SPI1_START_BUSY_W { w: self }
    }
    #[doc = "Bits 6:7 - Issue a read command or write command to the SPI interface"]
    #[inline(always)]
    pub fn spi1_rw_cmd(&mut self) -> SPI1_RW_CMD_W {
        SPI1_RW_CMD_W { w: self }
    }
    #[doc = "Bit 5 - Set the chip-select line for SPI (master mode), read the chip-select line for SPI (slave mode)"]
    #[inline(always)]
    pub fn spi1_cs(&mut self) -> SPI1_CS_W {
        SPI1_CS_W { w: self }
    }
    #[doc = "Bits 0:4 - Select the word size used by the SPI interface (word size = SPI1_WORD_SIZE + 1)"]
    #[inline(always)]
    pub fn spi1_word_size(&mut self) -> SPI1_WORD_SIZE_W {
        SPI1_WORD_SIZE_W { w: self }
    }
}
