#[doc = "Reader of register SYSCTRL_DSS_CMD"]
pub type R = crate::R<u32, super::SYSCTRL_DSS_CMD>;
#[doc = "Writer for register SYSCTRL_DSS_CMD"]
pub type W = crate::W<u32, super::SYSCTRL_DSS_CMD>;
#[doc = "Register SYSCTRL_DSS_CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCTRL_DSS_CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write a 1 to issue DSS command 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSS_CMD_6_AW {
    #[doc = "1: `1`"]
    DSS_CMD_6 = 1,
}
impl From<DSS_CMD_6_AW> for bool {
    #[inline(always)]
    fn from(variant: DSS_CMD_6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DSS_CMD_6`"]
pub struct DSS_CMD_6_W<'a> {
    w: &'a mut W,
}
impl<'a> DSS_CMD_6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSS_CMD_6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn dss_cmd_6(self) -> &'a mut W {
        self.variant(DSS_CMD_6_AW::DSS_CMD_6)
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
#[doc = "Write a 1 to issue DSS command 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSS_CMD_5_AW {
    #[doc = "1: `1`"]
    DSS_CMD_5 = 1,
}
impl From<DSS_CMD_5_AW> for bool {
    #[inline(always)]
    fn from(variant: DSS_CMD_5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DSS_CMD_5`"]
pub struct DSS_CMD_5_W<'a> {
    w: &'a mut W,
}
impl<'a> DSS_CMD_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSS_CMD_5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn dss_cmd_5(self) -> &'a mut W {
        self.variant(DSS_CMD_5_AW::DSS_CMD_5)
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
#[doc = "Write a 1 to issue DSS command 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSS_CMD_4_AW {
    #[doc = "1: `1`"]
    DSS_CMD_4 = 1,
}
impl From<DSS_CMD_4_AW> for bool {
    #[inline(always)]
    fn from(variant: DSS_CMD_4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DSS_CMD_4`"]
pub struct DSS_CMD_4_W<'a> {
    w: &'a mut W,
}
impl<'a> DSS_CMD_4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSS_CMD_4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn dss_cmd_4(self) -> &'a mut W {
        self.variant(DSS_CMD_4_AW::DSS_CMD_4)
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
#[doc = "Write a 1 to issue DSS command 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSS_CMD_3_AW {
    #[doc = "1: `1`"]
    DSS_CMD_3 = 1,
}
impl From<DSS_CMD_3_AW> for bool {
    #[inline(always)]
    fn from(variant: DSS_CMD_3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DSS_CMD_3`"]
pub struct DSS_CMD_3_W<'a> {
    w: &'a mut W,
}
impl<'a> DSS_CMD_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSS_CMD_3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn dss_cmd_3(self) -> &'a mut W {
        self.variant(DSS_CMD_3_AW::DSS_CMD_3)
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
#[doc = "Write a 1 to issue DSS command 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSS_CMD_2_AW {
    #[doc = "1: `1`"]
    DSS_CMD_2 = 1,
}
impl From<DSS_CMD_2_AW> for bool {
    #[inline(always)]
    fn from(variant: DSS_CMD_2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DSS_CMD_2`"]
pub struct DSS_CMD_2_W<'a> {
    w: &'a mut W,
}
impl<'a> DSS_CMD_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSS_CMD_2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn dss_cmd_2(self) -> &'a mut W {
        self.variant(DSS_CMD_2_AW::DSS_CMD_2)
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
#[doc = "Write a 1 to issue DSS command 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSS_CMD_1_AW {
    #[doc = "1: `1`"]
    DSS_CMD_1 = 1,
}
impl From<DSS_CMD_1_AW> for bool {
    #[inline(always)]
    fn from(variant: DSS_CMD_1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DSS_CMD_1`"]
pub struct DSS_CMD_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DSS_CMD_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSS_CMD_1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn dss_cmd_1(self) -> &'a mut W {
        self.variant(DSS_CMD_1_AW::DSS_CMD_1)
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
#[doc = "Write a 1 to issue DSS command 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSS_CMD_0_AW {
    #[doc = "1: `1`"]
    DSS_CMD_0 = 1,
}
impl From<DSS_CMD_0_AW> for bool {
    #[inline(always)]
    fn from(variant: DSS_CMD_0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DSS_CMD_0`"]
pub struct DSS_CMD_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DSS_CMD_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSS_CMD_0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn dss_cmd_0(self) -> &'a mut W {
        self.variant(DSS_CMD_0_AW::DSS_CMD_0)
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
    #[doc = "Bit 6 - Write a 1 to issue DSS command 6"]
    #[inline(always)]
    pub fn dss_cmd_6(&mut self) -> DSS_CMD_6_W {
        DSS_CMD_6_W { w: self }
    }
    #[doc = "Bit 5 - Write a 1 to issue DSS command 5"]
    #[inline(always)]
    pub fn dss_cmd_5(&mut self) -> DSS_CMD_5_W {
        DSS_CMD_5_W { w: self }
    }
    #[doc = "Bit 4 - Write a 1 to issue DSS command 4"]
    #[inline(always)]
    pub fn dss_cmd_4(&mut self) -> DSS_CMD_4_W {
        DSS_CMD_4_W { w: self }
    }
    #[doc = "Bit 3 - Write a 1 to issue DSS command 3"]
    #[inline(always)]
    pub fn dss_cmd_3(&mut self) -> DSS_CMD_3_W {
        DSS_CMD_3_W { w: self }
    }
    #[doc = "Bit 2 - Write a 1 to issue DSS command 2"]
    #[inline(always)]
    pub fn dss_cmd_2(&mut self) -> DSS_CMD_2_W {
        DSS_CMD_2_W { w: self }
    }
    #[doc = "Bit 1 - Write a 1 to issue DSS command 1"]
    #[inline(always)]
    pub fn dss_cmd_1(&mut self) -> DSS_CMD_1_W {
        DSS_CMD_1_W { w: self }
    }
    #[doc = "Bit 0 - Write a 1 to issue DSS command 0"]
    #[inline(always)]
    pub fn dss_cmd_0(&mut self) -> DSS_CMD_0_W {
        DSS_CMD_0_W { w: self }
    }
}
