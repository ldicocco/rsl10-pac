#[doc = "Reader of register ASRC_INT_ENABLE"]
pub type R = crate::R<u32, super::ASRC_INT_ENABLE>;
#[doc = "Writer for register ASRC_INT_ENABLE"]
pub type W = crate::W<u32, super::ASRC_INT_ENABLE>;
#[doc = "Register ASRC_INT_ENABLE `reset()`'s with value 0x08"]
impl crate::ResetValue for super::ASRC_INT_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "The ASRC state/configuration update error interrupt mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASRC_UPDATE_ERR_A {
    #[doc = "0: This source can not set an interrupt"]
    INT_DIS_ASRC_UPDATE_ERR = 0,
    #[doc = "1: This source can set the interrupt line"]
    INT_EBL_ASRC_UPDATE_ERR = 1,
}
impl From<ASRC_UPDATE_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ASRC_UPDATE_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASRC_UPDATE_ERR`"]
pub type ASRC_UPDATE_ERR_R = crate::R<bool, ASRC_UPDATE_ERR_A>;
impl ASRC_UPDATE_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASRC_UPDATE_ERR_A {
        match self.bits {
            false => ASRC_UPDATE_ERR_A::INT_DIS_ASRC_UPDATE_ERR,
            true => ASRC_UPDATE_ERR_A::INT_EBL_ASRC_UPDATE_ERR,
        }
    }
    #[doc = "Checks if the value of the field is `INT_DIS_ASRC_UPDATE_ERR`"]
    #[inline(always)]
    pub fn is_int_dis_asrc_update_err(&self) -> bool {
        *self == ASRC_UPDATE_ERR_A::INT_DIS_ASRC_UPDATE_ERR
    }
    #[doc = "Checks if the value of the field is `INT_EBL_ASRC_UPDATE_ERR`"]
    #[inline(always)]
    pub fn is_int_ebl_asrc_update_err(&self) -> bool {
        *self == ASRC_UPDATE_ERR_A::INT_EBL_ASRC_UPDATE_ERR
    }
}
#[doc = "Write proxy for field `ASRC_UPDATE_ERR`"]
pub struct ASRC_UPDATE_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ASRC_UPDATE_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASRC_UPDATE_ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This source can not set an interrupt"]
    #[inline(always)]
    pub fn int_dis_asrc_update_err(self) -> &'a mut W {
        self.variant(ASRC_UPDATE_ERR_A::INT_DIS_ASRC_UPDATE_ERR)
    }
    #[doc = "This source can set the interrupt line"]
    #[inline(always)]
    pub fn int_ebl_asrc_update_err(self) -> &'a mut W {
        self.variant(ASRC_UPDATE_ERR_A::INT_EBL_ASRC_UPDATE_ERR)
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
#[doc = "The ASRC input interface error interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASRC_IN_ERR_A {
    #[doc = "0: This source can not set an interrupt"]
    INT_DIS_ASRC_IN_ERR = 0,
    #[doc = "1: This source can set the interrupt line"]
    INT_EBL_ASRC_IN_ERR = 1,
}
impl From<ASRC_IN_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ASRC_IN_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASRC_IN_ERR`"]
pub type ASRC_IN_ERR_R = crate::R<bool, ASRC_IN_ERR_A>;
impl ASRC_IN_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASRC_IN_ERR_A {
        match self.bits {
            false => ASRC_IN_ERR_A::INT_DIS_ASRC_IN_ERR,
            true => ASRC_IN_ERR_A::INT_EBL_ASRC_IN_ERR,
        }
    }
    #[doc = "Checks if the value of the field is `INT_DIS_ASRC_IN_ERR`"]
    #[inline(always)]
    pub fn is_int_dis_asrc_in_err(&self) -> bool {
        *self == ASRC_IN_ERR_A::INT_DIS_ASRC_IN_ERR
    }
    #[doc = "Checks if the value of the field is `INT_EBL_ASRC_IN_ERR`"]
    #[inline(always)]
    pub fn is_int_ebl_asrc_in_err(&self) -> bool {
        *self == ASRC_IN_ERR_A::INT_EBL_ASRC_IN_ERR
    }
}
#[doc = "Write proxy for field `ASRC_IN_ERR`"]
pub struct ASRC_IN_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ASRC_IN_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASRC_IN_ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This source can not set an interrupt"]
    #[inline(always)]
    pub fn int_dis_asrc_in_err(self) -> &'a mut W {
        self.variant(ASRC_IN_ERR_A::INT_DIS_ASRC_IN_ERR)
    }
    #[doc = "This source can set the interrupt line"]
    #[inline(always)]
    pub fn int_ebl_asrc_in_err(self) -> &'a mut W {
        self.variant(ASRC_IN_ERR_A::INT_EBL_ASRC_IN_ERR)
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
#[doc = "The ASRC_OUT register interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASRC_OUT_REQ_A {
    #[doc = "0: This source can not set an interrupt"]
    INT_DIS_ASRC_OUT = 0,
    #[doc = "1: This source can set the interrupt line"]
    INT_EBL_ASRC_OUT = 1,
}
impl From<ASRC_OUT_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: ASRC_OUT_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASRC_OUT_REQ`"]
pub type ASRC_OUT_REQ_R = crate::R<bool, ASRC_OUT_REQ_A>;
impl ASRC_OUT_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASRC_OUT_REQ_A {
        match self.bits {
            false => ASRC_OUT_REQ_A::INT_DIS_ASRC_OUT,
            true => ASRC_OUT_REQ_A::INT_EBL_ASRC_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `INT_DIS_ASRC_OUT`"]
    #[inline(always)]
    pub fn is_int_dis_asrc_out(&self) -> bool {
        *self == ASRC_OUT_REQ_A::INT_DIS_ASRC_OUT
    }
    #[doc = "Checks if the value of the field is `INT_EBL_ASRC_OUT`"]
    #[inline(always)]
    pub fn is_int_ebl_asrc_out(&self) -> bool {
        *self == ASRC_OUT_REQ_A::INT_EBL_ASRC_OUT
    }
}
#[doc = "Write proxy for field `ASRC_OUT_REQ`"]
pub struct ASRC_OUT_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ASRC_OUT_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASRC_OUT_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This source can not set an interrupt"]
    #[inline(always)]
    pub fn int_dis_asrc_out(self) -> &'a mut W {
        self.variant(ASRC_OUT_REQ_A::INT_DIS_ASRC_OUT)
    }
    #[doc = "This source can set the interrupt line"]
    #[inline(always)]
    pub fn int_ebl_asrc_out(self) -> &'a mut W {
        self.variant(ASRC_OUT_REQ_A::INT_EBL_ASRC_OUT)
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
#[doc = "The ASRC_IN register interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASRC_IN_REQ_A {
    #[doc = "0: This source can not set an interrupt"]
    INT_DIS_ASRC_IN = 0,
    #[doc = "1: This source can set the interrupt line"]
    INT_EBL_ASRC_IN = 1,
}
impl From<ASRC_IN_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: ASRC_IN_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASRC_IN_REQ`"]
pub type ASRC_IN_REQ_R = crate::R<bool, ASRC_IN_REQ_A>;
impl ASRC_IN_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASRC_IN_REQ_A {
        match self.bits {
            false => ASRC_IN_REQ_A::INT_DIS_ASRC_IN,
            true => ASRC_IN_REQ_A::INT_EBL_ASRC_IN,
        }
    }
    #[doc = "Checks if the value of the field is `INT_DIS_ASRC_IN`"]
    #[inline(always)]
    pub fn is_int_dis_asrc_in(&self) -> bool {
        *self == ASRC_IN_REQ_A::INT_DIS_ASRC_IN
    }
    #[doc = "Checks if the value of the field is `INT_EBL_ASRC_IN`"]
    #[inline(always)]
    pub fn is_int_ebl_asrc_in(&self) -> bool {
        *self == ASRC_IN_REQ_A::INT_EBL_ASRC_IN
    }
}
#[doc = "Write proxy for field `ASRC_IN_REQ`"]
pub struct ASRC_IN_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ASRC_IN_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASRC_IN_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This source can not set an interrupt"]
    #[inline(always)]
    pub fn int_dis_asrc_in(self) -> &'a mut W {
        self.variant(ASRC_IN_REQ_A::INT_DIS_ASRC_IN)
    }
    #[doc = "This source can set the interrupt line"]
    #[inline(always)]
    pub fn int_ebl_asrc_in(self) -> &'a mut W {
        self.variant(ASRC_IN_REQ_A::INT_EBL_ASRC_IN)
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
    #[doc = "Bit 3 - The ASRC state/configuration update error interrupt mask"]
    #[inline(always)]
    pub fn asrc_update_err(&self) -> ASRC_UPDATE_ERR_R {
        ASRC_UPDATE_ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The ASRC input interface error interrupt mask"]
    #[inline(always)]
    pub fn asrc_in_err(&self) -> ASRC_IN_ERR_R {
        ASRC_IN_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - The ASRC_OUT register interrupt status"]
    #[inline(always)]
    pub fn asrc_out_req(&self) -> ASRC_OUT_REQ_R {
        ASRC_OUT_REQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The ASRC_IN register interrupt status"]
    #[inline(always)]
    pub fn asrc_in_req(&self) -> ASRC_IN_REQ_R {
        ASRC_IN_REQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - The ASRC state/configuration update error interrupt mask"]
    #[inline(always)]
    pub fn asrc_update_err(&mut self) -> ASRC_UPDATE_ERR_W {
        ASRC_UPDATE_ERR_W { w: self }
    }
    #[doc = "Bit 2 - The ASRC input interface error interrupt mask"]
    #[inline(always)]
    pub fn asrc_in_err(&mut self) -> ASRC_IN_ERR_W {
        ASRC_IN_ERR_W { w: self }
    }
    #[doc = "Bit 1 - The ASRC_OUT register interrupt status"]
    #[inline(always)]
    pub fn asrc_out_req(&mut self) -> ASRC_OUT_REQ_W {
        ASRC_OUT_REQ_W { w: self }
    }
    #[doc = "Bit 0 - The ASRC_IN register interrupt status"]
    #[inline(always)]
    pub fn asrc_in_req(&mut self) -> ASRC_IN_REQ_W {
        ASRC_IN_REQ_W { w: self }
    }
}
