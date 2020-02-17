#[doc = "Reader of register FLASH_NVR_CTRL"]
pub type R = crate::R<u32, super::FLASH_NVR_CTRL>;
#[doc = "Writer for register FLASH_NVR_CTRL"]
pub type W = crate::W<u32, super::FLASH_NVR_CTRL>;
#[doc = "Register FLASH_NVR_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_NVR_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Authorize Write access to the Flash NVR3 sector through the FLASH_IF registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NVR3_W_EN_A {
    #[doc = "0: The Flash NVR3 block is protected against write access."]
    NVR3_WRITE_DISABLE = 0,
    #[doc = "1: The Flash NVR3 block can be written."]
    NVR3_WRITE_ENABLE = 1,
}
impl From<NVR3_W_EN_A> for bool {
    #[inline(always)]
    fn from(variant: NVR3_W_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NVR3_W_EN`"]
pub type NVR3_W_EN_R = crate::R<bool, NVR3_W_EN_A>;
impl NVR3_W_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NVR3_W_EN_A {
        match self.bits {
            false => NVR3_W_EN_A::NVR3_WRITE_DISABLE,
            true => NVR3_W_EN_A::NVR3_WRITE_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NVR3_WRITE_DISABLE`"]
    #[inline(always)]
    pub fn is_nvr3_write_disable(&self) -> bool {
        *self == NVR3_W_EN_A::NVR3_WRITE_DISABLE
    }
    #[doc = "Checks if the value of the field is `NVR3_WRITE_ENABLE`"]
    #[inline(always)]
    pub fn is_nvr3_write_enable(&self) -> bool {
        *self == NVR3_W_EN_A::NVR3_WRITE_ENABLE
    }
}
#[doc = "Write proxy for field `NVR3_W_EN`"]
pub struct NVR3_W_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> NVR3_W_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NVR3_W_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The Flash NVR3 block is protected against write access."]
    #[inline(always)]
    pub fn nvr3_write_disable(self) -> &'a mut W {
        self.variant(NVR3_W_EN_A::NVR3_WRITE_DISABLE)
    }
    #[doc = "The Flash NVR3 block can be written."]
    #[inline(always)]
    pub fn nvr3_write_enable(self) -> &'a mut W {
        self.variant(NVR3_W_EN_A::NVR3_WRITE_ENABLE)
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
#[doc = "Authorize Write access to the Flash NVR2 sector through the FLASH_IF registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NVR2_W_EN_A {
    #[doc = "0: The Flash NVR2 block is protected against write access."]
    NVR2_WRITE_DISABLE = 0,
    #[doc = "1: The Flash NVR2 block can be written."]
    NVR2_WRITE_ENABLE = 1,
}
impl From<NVR2_W_EN_A> for bool {
    #[inline(always)]
    fn from(variant: NVR2_W_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NVR2_W_EN`"]
pub type NVR2_W_EN_R = crate::R<bool, NVR2_W_EN_A>;
impl NVR2_W_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NVR2_W_EN_A {
        match self.bits {
            false => NVR2_W_EN_A::NVR2_WRITE_DISABLE,
            true => NVR2_W_EN_A::NVR2_WRITE_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NVR2_WRITE_DISABLE`"]
    #[inline(always)]
    pub fn is_nvr2_write_disable(&self) -> bool {
        *self == NVR2_W_EN_A::NVR2_WRITE_DISABLE
    }
    #[doc = "Checks if the value of the field is `NVR2_WRITE_ENABLE`"]
    #[inline(always)]
    pub fn is_nvr2_write_enable(&self) -> bool {
        *self == NVR2_W_EN_A::NVR2_WRITE_ENABLE
    }
}
#[doc = "Write proxy for field `NVR2_W_EN`"]
pub struct NVR2_W_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> NVR2_W_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NVR2_W_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The Flash NVR2 block is protected against write access."]
    #[inline(always)]
    pub fn nvr2_write_disable(self) -> &'a mut W {
        self.variant(NVR2_W_EN_A::NVR2_WRITE_DISABLE)
    }
    #[doc = "The Flash NVR2 block can be written."]
    #[inline(always)]
    pub fn nvr2_write_enable(self) -> &'a mut W {
        self.variant(NVR2_W_EN_A::NVR2_WRITE_ENABLE)
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
#[doc = "Authorize Write access to the Flash NVR1 sector through the FLASH_IF registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NVR1_W_EN_A {
    #[doc = "0: The Flash NVR1 block is protected against write access."]
    NVR1_WRITE_DISABLE = 0,
    #[doc = "1: The Flash NVR1 block can be written."]
    NVR1_WRITE_ENABLE = 1,
}
impl From<NVR1_W_EN_A> for bool {
    #[inline(always)]
    fn from(variant: NVR1_W_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NVR1_W_EN`"]
pub type NVR1_W_EN_R = crate::R<bool, NVR1_W_EN_A>;
impl NVR1_W_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NVR1_W_EN_A {
        match self.bits {
            false => NVR1_W_EN_A::NVR1_WRITE_DISABLE,
            true => NVR1_W_EN_A::NVR1_WRITE_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NVR1_WRITE_DISABLE`"]
    #[inline(always)]
    pub fn is_nvr1_write_disable(&self) -> bool {
        *self == NVR1_W_EN_A::NVR1_WRITE_DISABLE
    }
    #[doc = "Checks if the value of the field is `NVR1_WRITE_ENABLE`"]
    #[inline(always)]
    pub fn is_nvr1_write_enable(&self) -> bool {
        *self == NVR1_W_EN_A::NVR1_WRITE_ENABLE
    }
}
#[doc = "Write proxy for field `NVR1_W_EN`"]
pub struct NVR1_W_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> NVR1_W_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NVR1_W_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The Flash NVR1 block is protected against write access."]
    #[inline(always)]
    pub fn nvr1_write_disable(self) -> &'a mut W {
        self.variant(NVR1_W_EN_A::NVR1_WRITE_DISABLE)
    }
    #[doc = "The Flash NVR1 block can be written."]
    #[inline(always)]
    pub fn nvr1_write_enable(self) -> &'a mut W {
        self.variant(NVR1_W_EN_A::NVR1_WRITE_ENABLE)
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
impl R {
    #[doc = "Bit 3 - Authorize Write access to the Flash NVR3 sector through the FLASH_IF registers."]
    #[inline(always)]
    pub fn nvr3_w_en(&self) -> NVR3_W_EN_R {
        NVR3_W_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Authorize Write access to the Flash NVR2 sector through the FLASH_IF registers."]
    #[inline(always)]
    pub fn nvr2_w_en(&self) -> NVR2_W_EN_R {
        NVR2_W_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Authorize Write access to the Flash NVR1 sector through the FLASH_IF registers."]
    #[inline(always)]
    pub fn nvr1_w_en(&self) -> NVR1_W_EN_R {
        NVR1_W_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Authorize Write access to the Flash NVR3 sector through the FLASH_IF registers."]
    #[inline(always)]
    pub fn nvr3_w_en(&mut self) -> NVR3_W_EN_W {
        NVR3_W_EN_W { w: self }
    }
    #[doc = "Bit 2 - Authorize Write access to the Flash NVR2 sector through the FLASH_IF registers."]
    #[inline(always)]
    pub fn nvr2_w_en(&mut self) -> NVR2_W_EN_W {
        NVR2_W_EN_W { w: self }
    }
    #[doc = "Bit 1 - Authorize Write access to the Flash NVR1 sector through the FLASH_IF registers."]
    #[inline(always)]
    pub fn nvr1_w_en(&mut self) -> NVR1_W_EN_W {
        NVR1_W_EN_W { w: self }
    }
}
