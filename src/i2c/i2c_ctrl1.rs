#[doc = "Reader of register I2C_CTRL1"]
pub type R = crate::R<u32, super::I2C_CTRL1>;
#[doc = "Writer for register I2C_CTRL1"]
pub type W = crate::W<u32, super::I2C_CTRL1>;
#[doc = "Register I2C_CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reset the I2C interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_AW {
    #[doc = "1: Reset the I2C interface"]
    I2C_RESET = 1,
}
impl From<RESET_AW> for bool {
    #[inline(always)]
    fn from(variant: RESET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RESET`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESET_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the I2C interface"]
    #[inline(always)]
    pub fn i2c_reset(self) -> &'a mut W {
        self.variant(RESET_AW::I2C_RESET)
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
#[doc = "Indicate that the current data is the last byte of a data transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LAST_DATA_AW {
    #[doc = "1: Indicate that the current data is the last byte of a data transfer"]
    I2C_LAST_DATA = 1,
}
impl From<LAST_DATA_AW> for bool {
    #[inline(always)]
    fn from(variant: LAST_DATA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `LAST_DATA`"]
pub struct LAST_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_DATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LAST_DATA_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Indicate that the current data is the last byte of a data transfer"]
    #[inline(always)]
    pub fn i2c_last_data(self) -> &'a mut W {
        self.variant(LAST_DATA_AW::I2C_LAST_DATA)
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
#[doc = "Issue a stop condition on the I2C interface bus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_AW {
    #[doc = "1: Issue a stop condition on the I2C interface bus"]
    I2C_STOP = 1,
}
impl From<STOP_AW> for bool {
    #[inline(always)]
    fn from(variant: STOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `STOP`"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Issue a stop condition on the I2C interface bus"]
    #[inline(always)]
    pub fn i2c_stop(self) -> &'a mut W {
        self.variant(STOP_AW::I2C_STOP)
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
#[doc = "Issue a not acknowledge on the I2C interface bus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACK_AW {
    #[doc = "1: Issue a not acknowledge on the I2C interface bus"]
    I2C_NACK = 1,
}
impl From<NACK_AW> for bool {
    #[inline(always)]
    fn from(variant: NACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `NACK`"]
pub struct NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> NACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NACK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Issue a not acknowledge on the I2C interface bus"]
    #[inline(always)]
    pub fn i2c_nack(self) -> &'a mut W {
        self.variant(NACK_AW::I2C_NACK)
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
#[doc = "Issue an acknowledge on the I2C interface bus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACK_AW {
    #[doc = "1: Issue an acknowledge on the I2C interface bus"]
    I2C_ACK = 1,
}
impl From<ACK_AW> for bool {
    #[inline(always)]
    fn from(variant: ACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ACK`"]
pub struct ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Issue an acknowledge on the I2C interface bus"]
    #[inline(always)]
    pub fn i2c_ack(self) -> &'a mut W {
        self.variant(ACK_AW::I2C_ACK)
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
    #[doc = "Bit 5 - Reset the I2C interface"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 4 - Indicate that the current data is the last byte of a data transfer"]
    #[inline(always)]
    pub fn last_data(&mut self) -> LAST_DATA_W {
        LAST_DATA_W { w: self }
    }
    #[doc = "Bit 3 - Issue a stop condition on the I2C interface bus"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 1 - Issue a not acknowledge on the I2C interface bus"]
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W {
        NACK_W { w: self }
    }
    #[doc = "Bit 0 - Issue an acknowledge on the I2C interface bus"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W { w: self }
    }
}
