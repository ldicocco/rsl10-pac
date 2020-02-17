#[doc = "Reader of register I2C_DATA_M"]
pub type R = crate::R<u32, super::I2C_DATA_M>;
#[doc = "Writer for register I2C_DATA_M"]
pub type W = crate::W<u32, super::I2C_DATA_M>;
#[doc = "Register I2C_DATA_M `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_DATA_M {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_DATA_M`"]
pub type I2C_DATA_M_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_DATA_M`"]
pub struct I2C_DATA_M_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_DATA_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Mirror of the single byte buffer for data transmitted and received over the I2C interface"]
    #[inline(always)]
    pub fn i2c_data_m(&self) -> I2C_DATA_M_R {
        I2C_DATA_M_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Mirror of the single byte buffer for data transmitted and received over the I2C interface"]
    #[inline(always)]
    pub fn i2c_data_m(&mut self) -> I2C_DATA_M_W {
        I2C_DATA_M_W { w: self }
    }
}
