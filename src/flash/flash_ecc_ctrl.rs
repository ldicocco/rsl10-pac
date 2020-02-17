#[doc = "Reader of register FLASH_ECC_CTRL"]
pub type R = crate::R<u32, super::FLASH_ECC_CTRL>;
#[doc = "Writer for register FLASH_ECC_CTRL"]
pub type W = crate::W<u32, super::FLASH_ECC_CTRL>;
#[doc = "Register FLASH_ECC_CTRL `reset()`'s with value 0x010d"]
impl crate::ResetValue for super::FLASH_ECC_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x010d
    }
}
#[doc = "Select the number of corrected errors before sending a CM3 interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ECC_COR_CNT_INT_THRESHOLD_A {
    #[doc = "0: Interrupt is disabled"]
    FLASH_ECC_COR_INT_THRESHOLD_DISABLED = 0,
    #[doc = "1: Send a CM3 interrupt when one or more correctable errors are detected."]
    FLASH_ECC_COR_INT_THRESHOLD_1 = 1,
    #[doc = "255: Send a CM3 interrupt when 255 or more correctable errors are detected."]
    FLASH_ECC_COR_INT_THRESHOLD_255 = 255,
}
impl From<ECC_COR_CNT_INT_THRESHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: ECC_COR_CNT_INT_THRESHOLD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ECC_COR_CNT_INT_THRESHOLD`"]
pub type ECC_COR_CNT_INT_THRESHOLD_R = crate::R<u8, ECC_COR_CNT_INT_THRESHOLD_A>;
impl ECC_COR_CNT_INT_THRESHOLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ECC_COR_CNT_INT_THRESHOLD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ECC_COR_CNT_INT_THRESHOLD_A::FLASH_ECC_COR_INT_THRESHOLD_DISABLED),
            1 => Val(ECC_COR_CNT_INT_THRESHOLD_A::FLASH_ECC_COR_INT_THRESHOLD_1),
            255 => Val(ECC_COR_CNT_INT_THRESHOLD_A::FLASH_ECC_COR_INT_THRESHOLD_255),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ECC_COR_INT_THRESHOLD_DISABLED`"]
    #[inline(always)]
    pub fn is_flash_ecc_cor_int_threshold_disabled(&self) -> bool {
        *self == ECC_COR_CNT_INT_THRESHOLD_A::FLASH_ECC_COR_INT_THRESHOLD_DISABLED
    }
    #[doc = "Checks if the value of the field is `FLASH_ECC_COR_INT_THRESHOLD_1`"]
    #[inline(always)]
    pub fn is_flash_ecc_cor_int_threshold_1(&self) -> bool {
        *self == ECC_COR_CNT_INT_THRESHOLD_A::FLASH_ECC_COR_INT_THRESHOLD_1
    }
    #[doc = "Checks if the value of the field is `FLASH_ECC_COR_INT_THRESHOLD_255`"]
    #[inline(always)]
    pub fn is_flash_ecc_cor_int_threshold_255(&self) -> bool {
        *self == ECC_COR_CNT_INT_THRESHOLD_A::FLASH_ECC_COR_INT_THRESHOLD_255
    }
}
#[doc = "Write proxy for field `ECC_COR_CNT_INT_THRESHOLD`"]
pub struct ECC_COR_CNT_INT_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_COR_CNT_INT_THRESHOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECC_COR_CNT_INT_THRESHOLD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn flash_ecc_cor_int_threshold_disabled(self) -> &'a mut W {
        self.variant(ECC_COR_CNT_INT_THRESHOLD_A::FLASH_ECC_COR_INT_THRESHOLD_DISABLED)
    }
    #[doc = "Send a CM3 interrupt when one or more correctable errors are detected."]
    #[inline(always)]
    pub fn flash_ecc_cor_int_threshold_1(self) -> &'a mut W {
        self.variant(ECC_COR_CNT_INT_THRESHOLD_A::FLASH_ECC_COR_INT_THRESHOLD_1)
    }
    #[doc = "Send a CM3 interrupt when 255 or more correctable errors are detected."]
    #[inline(always)]
    pub fn flash_ecc_cor_int_threshold_255(self) -> &'a mut W {
        self.variant(ECC_COR_CNT_INT_THRESHOLD_A::FLASH_ECC_COR_INT_THRESHOLD_255)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COPIER_ECC_CTRL_A {
    #[doc = "0: Disables ECC when reading Flash through Flash Copier"]
    FLASH_COPIER_ECC_DISABLE = 0,
    #[doc = "1: Enables ECC when reading Flash through Flash Copier"]
    FLASH_COPIER_ECC_ENABLE = 1,
}
impl From<COPIER_ECC_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: COPIER_ECC_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COPIER_ECC_CTRL`"]
pub type COPIER_ECC_CTRL_R = crate::R<bool, COPIER_ECC_CTRL_A>;
impl COPIER_ECC_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COPIER_ECC_CTRL_A {
        match self.bits {
            false => COPIER_ECC_CTRL_A::FLASH_COPIER_ECC_DISABLE,
            true => COPIER_ECC_CTRL_A::FLASH_COPIER_ECC_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_COPIER_ECC_DISABLE`"]
    #[inline(always)]
    pub fn is_flash_copier_ecc_disable(&self) -> bool {
        *self == COPIER_ECC_CTRL_A::FLASH_COPIER_ECC_DISABLE
    }
    #[doc = "Checks if the value of the field is `FLASH_COPIER_ECC_ENABLE`"]
    #[inline(always)]
    pub fn is_flash_copier_ecc_enable(&self) -> bool {
        *self == COPIER_ECC_CTRL_A::FLASH_COPIER_ECC_ENABLE
    }
}
#[doc = "Write proxy for field `COPIER_ECC_CTRL`"]
pub struct COPIER_ECC_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> COPIER_ECC_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COPIER_ECC_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables ECC when reading Flash through Flash Copier"]
    #[inline(always)]
    pub fn flash_copier_ecc_disable(self) -> &'a mut W {
        self.variant(COPIER_ECC_CTRL_A::FLASH_COPIER_ECC_DISABLE)
    }
    #[doc = "Enables ECC when reading Flash through Flash Copier"]
    #[inline(always)]
    pub fn flash_copier_ecc_enable(self) -> &'a mut W {
        self.variant(COPIER_ECC_CTRL_A::FLASH_COPIER_ECC_ENABLE)
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
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_ECC_CTRL_A {
    #[doc = "0: Disables ECC when reading Flash through Flash mapped register"]
    FLASH_CMD_ECC_DISABLE = 0,
    #[doc = "1: Enables ECC when reading Flash through Flash mapped register"]
    FLASH_CMD_ECC_ENABLE = 1,
}
impl From<CMD_ECC_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_ECC_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMD_ECC_CTRL`"]
pub type CMD_ECC_CTRL_R = crate::R<bool, CMD_ECC_CTRL_A>;
impl CMD_ECC_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_ECC_CTRL_A {
        match self.bits {
            false => CMD_ECC_CTRL_A::FLASH_CMD_ECC_DISABLE,
            true => CMD_ECC_CTRL_A::FLASH_CMD_ECC_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_CMD_ECC_DISABLE`"]
    #[inline(always)]
    pub fn is_flash_cmd_ecc_disable(&self) -> bool {
        *self == CMD_ECC_CTRL_A::FLASH_CMD_ECC_DISABLE
    }
    #[doc = "Checks if the value of the field is `FLASH_CMD_ECC_ENABLE`"]
    #[inline(always)]
    pub fn is_flash_cmd_ecc_enable(&self) -> bool {
        *self == CMD_ECC_CTRL_A::FLASH_CMD_ECC_ENABLE
    }
}
#[doc = "Write proxy for field `CMD_ECC_CTRL`"]
pub struct CMD_ECC_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_ECC_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_ECC_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables ECC when reading Flash through Flash mapped register"]
    #[inline(always)]
    pub fn flash_cmd_ecc_disable(self) -> &'a mut W {
        self.variant(CMD_ECC_CTRL_A::FLASH_CMD_ECC_DISABLE)
    }
    #[doc = "Enables ECC when reading Flash through Flash mapped register"]
    #[inline(always)]
    pub fn flash_cmd_ecc_enable(self) -> &'a mut W {
        self.variant(CMD_ECC_CTRL_A::FLASH_CMD_ECC_ENABLE)
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
#[doc = "Select the operating mode of the Flash ECC\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDBUS_ECC_CTRL_A {
    #[doc = "0: Disables ECC when reading Flash through I-Bus and D-Bus"]
    FLASH_IDBUS_ECC_DISABLE = 0,
    #[doc = "1: Enables ECC when reading Flash through I-Bus and D-Bus"]
    FLASH_IDBUS_ECC_ENABLE = 1,
}
impl From<IDBUS_ECC_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: IDBUS_ECC_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IDBUS_ECC_CTRL`"]
pub type IDBUS_ECC_CTRL_R = crate::R<bool, IDBUS_ECC_CTRL_A>;
impl IDBUS_ECC_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDBUS_ECC_CTRL_A {
        match self.bits {
            false => IDBUS_ECC_CTRL_A::FLASH_IDBUS_ECC_DISABLE,
            true => IDBUS_ECC_CTRL_A::FLASH_IDBUS_ECC_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_IDBUS_ECC_DISABLE`"]
    #[inline(always)]
    pub fn is_flash_idbus_ecc_disable(&self) -> bool {
        *self == IDBUS_ECC_CTRL_A::FLASH_IDBUS_ECC_DISABLE
    }
    #[doc = "Checks if the value of the field is `FLASH_IDBUS_ECC_ENABLE`"]
    #[inline(always)]
    pub fn is_flash_idbus_ecc_enable(&self) -> bool {
        *self == IDBUS_ECC_CTRL_A::FLASH_IDBUS_ECC_ENABLE
    }
}
#[doc = "Write proxy for field `IDBUS_ECC_CTRL`"]
pub struct IDBUS_ECC_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> IDBUS_ECC_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDBUS_ECC_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables ECC when reading Flash through I-Bus and D-Bus"]
    #[inline(always)]
    pub fn flash_idbus_ecc_disable(self) -> &'a mut W {
        self.variant(IDBUS_ECC_CTRL_A::FLASH_IDBUS_ECC_DISABLE)
    }
    #[doc = "Enables ECC when reading Flash through I-Bus and D-Bus"]
    #[inline(always)]
    pub fn flash_idbus_ecc_enable(self) -> &'a mut W {
        self.variant(IDBUS_ECC_CTRL_A::FLASH_IDBUS_ECC_ENABLE)
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
    #[doc = "Bits 8:15 - Select the number of corrected errors before sending a CM3 interrupt"]
    #[inline(always)]
    pub fn ecc_cor_cnt_int_threshold(&self) -> ECC_COR_CNT_INT_THRESHOLD_R {
        ECC_COR_CNT_INT_THRESHOLD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn copier_ecc_ctrl(&self) -> COPIER_ECC_CTRL_R {
        COPIER_ECC_CTRL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cmd_ecc_ctrl(&self) -> CMD_ECC_CTRL_R {
        CMD_ECC_CTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Select the operating mode of the Flash ECC"]
    #[inline(always)]
    pub fn idbus_ecc_ctrl(&self) -> IDBUS_ECC_CTRL_R {
        IDBUS_ECC_CTRL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:15 - Select the number of corrected errors before sending a CM3 interrupt"]
    #[inline(always)]
    pub fn ecc_cor_cnt_int_threshold(&mut self) -> ECC_COR_CNT_INT_THRESHOLD_W {
        ECC_COR_CNT_INT_THRESHOLD_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn copier_ecc_ctrl(&mut self) -> COPIER_ECC_CTRL_W {
        COPIER_ECC_CTRL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cmd_ecc_ctrl(&mut self) -> CMD_ECC_CTRL_W {
        CMD_ECC_CTRL_W { w: self }
    }
    #[doc = "Bit 0 - Select the operating mode of the Flash ECC"]
    #[inline(always)]
    pub fn idbus_ecc_ctrl(&mut self) -> IDBUS_ECC_CTRL_W {
        IDBUS_ECC_CTRL_W { w: self }
    }
}
