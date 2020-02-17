#[doc = "Reader of register FLASH_ECC_STATUS"]
pub type R = crate::R<u32, super::FLASH_ECC_STATUS>;
#[doc = "Writer for register FLASH_ECC_STATUS"]
pub type W = crate::W<u32, super::FLASH_ECC_STATUS>;
#[doc = "Register FLASH_ECC_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_ECC_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reset the Flash corrected errors counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECC_COR_ERROR_CNT_CLEAR_AW {
    #[doc = "1: Reset the Flash corrected errors counter"]
    FLASH_ECC_COR_ERROR_CNT_CLEAR = 1,
}
impl From<ECC_COR_ERROR_CNT_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: ECC_COR_ERROR_CNT_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ECC_COR_ERROR_CNT_CLEAR`"]
pub struct ECC_COR_ERROR_CNT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_COR_ERROR_CNT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECC_COR_ERROR_CNT_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the Flash corrected errors counter"]
    #[inline(always)]
    pub fn flash_ecc_cor_error_cnt_clear(self) -> &'a mut W {
        self.variant(ECC_COR_ERROR_CNT_CLEAR_AW::FLASH_ECC_COR_ERROR_CNT_CLEAR)
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
#[doc = "Reset the Flash uncorrected errors counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECC_UNCOR_ERROR_CNT_CLEAR_AW {
    #[doc = "1: Reset the Flash uncorrected errors counter"]
    FLASH_ECC_UNCOR_ERROR_CNT_CLEAR = 1,
}
impl From<ECC_UNCOR_ERROR_CNT_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: ECC_UNCOR_ERROR_CNT_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ECC_UNCOR_ERROR_CNT_CLEAR`"]
pub struct ECC_UNCOR_ERROR_CNT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_UNCOR_ERROR_CNT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECC_UNCOR_ERROR_CNT_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the Flash uncorrected errors counter"]
    #[inline(always)]
    pub fn flash_ecc_uncor_error_cnt_clear(self) -> &'a mut W {
        self.variant(ECC_UNCOR_ERROR_CNT_CLEAR_AW::FLASH_ECC_UNCOR_ERROR_CNT_CLEAR)
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
#[doc = "Reset the Flash address of the last detected error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECC_ERROR_ADDR_CLEAR_AW {
    #[doc = "1: Reset the Flash address of the latest detected error"]
    FLASH_ECC_ERROR_ADDR_CLEAR = 1,
}
impl From<ECC_ERROR_ADDR_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: ECC_ERROR_ADDR_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ECC_ERROR_ADDR_CLEAR`"]
pub struct ECC_ERROR_ADDR_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_ERROR_ADDR_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECC_ERROR_ADDR_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the Flash address of the latest detected error"]
    #[inline(always)]
    pub fn flash_ecc_error_addr_clear(self) -> &'a mut W {
        self.variant(ECC_ERROR_ADDR_CLEAR_AW::FLASH_ECC_ERROR_ADDR_CLEAR)
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
#[doc = "FLASH_ECC_ERROR_COR_CNT status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECC_COR_ERROR_CNT_STATUS_A {
    #[doc = "0: Indicates FLASH_ECC_COR_ERROR_CNT is zero"]
    FLASH_ECC_NO_CORRECTED_ERROR = 0,
    #[doc = "1: Indicates FLASH_ECC_COR_ERROR_CNT is not zero"]
    FLASH_ECC_CORRECTED_ERROR = 1,
}
impl From<ECC_COR_ERROR_CNT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: ECC_COR_ERROR_CNT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ECC_COR_ERROR_CNT_STATUS`"]
pub type ECC_COR_ERROR_CNT_STATUS_R = crate::R<bool, ECC_COR_ERROR_CNT_STATUS_A>;
impl ECC_COR_ERROR_CNT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECC_COR_ERROR_CNT_STATUS_A {
        match self.bits {
            false => ECC_COR_ERROR_CNT_STATUS_A::FLASH_ECC_NO_CORRECTED_ERROR,
            true => ECC_COR_ERROR_CNT_STATUS_A::FLASH_ECC_CORRECTED_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ECC_NO_CORRECTED_ERROR`"]
    #[inline(always)]
    pub fn is_flash_ecc_no_corrected_error(&self) -> bool {
        *self == ECC_COR_ERROR_CNT_STATUS_A::FLASH_ECC_NO_CORRECTED_ERROR
    }
    #[doc = "Checks if the value of the field is `FLASH_ECC_CORRECTED_ERROR`"]
    #[inline(always)]
    pub fn is_flash_ecc_corrected_error(&self) -> bool {
        *self == ECC_COR_ERROR_CNT_STATUS_A::FLASH_ECC_CORRECTED_ERROR
    }
}
#[doc = "FLASH_ECC_ERROR_UNCOR_CNT status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECC_UNCOR_ERROR_CNT_STATUS_A {
    #[doc = "0: Indicates FLASH_ECC_UNCOR_ERROR_CNT is zero"]
    FLASH_ECC_NO_UNCORRECTED_ERROR = 0,
    #[doc = "1: Indicates FLASH_ECC_UNCOR_ERROR_CNT is not zero"]
    FLASH_ECC_UNCORRECTED_ERROR = 1,
}
impl From<ECC_UNCOR_ERROR_CNT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: ECC_UNCOR_ERROR_CNT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ECC_UNCOR_ERROR_CNT_STATUS`"]
pub type ECC_UNCOR_ERROR_CNT_STATUS_R = crate::R<bool, ECC_UNCOR_ERROR_CNT_STATUS_A>;
impl ECC_UNCOR_ERROR_CNT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECC_UNCOR_ERROR_CNT_STATUS_A {
        match self.bits {
            false => ECC_UNCOR_ERROR_CNT_STATUS_A::FLASH_ECC_NO_UNCORRECTED_ERROR,
            true => ECC_UNCOR_ERROR_CNT_STATUS_A::FLASH_ECC_UNCORRECTED_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ECC_NO_UNCORRECTED_ERROR`"]
    #[inline(always)]
    pub fn is_flash_ecc_no_uncorrected_error(&self) -> bool {
        *self == ECC_UNCOR_ERROR_CNT_STATUS_A::FLASH_ECC_NO_UNCORRECTED_ERROR
    }
    #[doc = "Checks if the value of the field is `FLASH_ECC_UNCORRECTED_ERROR`"]
    #[inline(always)]
    pub fn is_flash_ecc_uncorrected_error(&self) -> bool {
        *self == ECC_UNCOR_ERROR_CNT_STATUS_A::FLASH_ECC_UNCORRECTED_ERROR
    }
}
impl R {
    #[doc = "Bit 1 - FLASH_ECC_ERROR_COR_CNT status"]
    #[inline(always)]
    pub fn ecc_cor_error_cnt_status(&self) -> ECC_COR_ERROR_CNT_STATUS_R {
        ECC_COR_ERROR_CNT_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - FLASH_ECC_ERROR_UNCOR_CNT status"]
    #[inline(always)]
    pub fn ecc_uncor_error_cnt_status(&self) -> ECC_UNCOR_ERROR_CNT_STATUS_R {
        ECC_UNCOR_ERROR_CNT_STATUS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Reset the Flash corrected errors counter"]
    #[inline(always)]
    pub fn ecc_cor_error_cnt_clear(&mut self) -> ECC_COR_ERROR_CNT_CLEAR_W {
        ECC_COR_ERROR_CNT_CLEAR_W { w: self }
    }
    #[doc = "Bit 5 - Reset the Flash uncorrected errors counter"]
    #[inline(always)]
    pub fn ecc_uncor_error_cnt_clear(&mut self) -> ECC_UNCOR_ERROR_CNT_CLEAR_W {
        ECC_UNCOR_ERROR_CNT_CLEAR_W { w: self }
    }
    #[doc = "Bit 4 - Reset the Flash address of the last detected error"]
    #[inline(always)]
    pub fn ecc_error_addr_clear(&mut self) -> ECC_ERROR_ADDR_CLEAR_W {
        ECC_ERROR_ADDR_CLEAR_W { w: self }
    }
}
