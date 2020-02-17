#[doc = "Reader of register I2C_ADDR_START"]
pub type R = crate::R<u32, super::I2C_ADDR_START>;
#[doc = "Writer for register I2C_ADDR_START"]
pub type W = crate::W<u32, super::I2C_ADDR_START>;
#[doc = "Register I2C_ADDR_START `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_ADDR_START {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ADDRESS`"]
pub struct ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
#[doc = "Select whether a read or a write transaction is started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_WRITE_AW {
    #[doc = "0: Start an I2C write transaction"]
    I2C_START_WRITE = 0,
    #[doc = "1: Start an I2C read transaction"]
    I2C_START_READ = 1,
}
impl From<READ_WRITE_AW> for bool {
    #[inline(always)]
    fn from(variant: READ_WRITE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `READ_WRITE`"]
pub struct READ_WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_WRITE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READ_WRITE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Start an I2C write transaction"]
    #[inline(always)]
    pub fn i2c_start_write(self) -> &'a mut W {
        self.variant(READ_WRITE_AW::I2C_START_WRITE)
    }
    #[doc = "Start an I2C read transaction"]
    #[inline(always)]
    pub fn i2c_start_read(self) -> &'a mut W {
        self.variant(READ_WRITE_AW::I2C_START_READ)
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
impl R {}
impl W {
    #[doc = "Bits 1:7 - I2C address to use for the transaction"]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W {
        ADDRESS_W { w: self }
    }
    #[doc = "Bit 0 - Select whether a read or a write transaction is started"]
    #[inline(always)]
    pub fn read_write(&mut self) -> READ_WRITE_W {
        READ_WRITE_W { w: self }
    }
}
