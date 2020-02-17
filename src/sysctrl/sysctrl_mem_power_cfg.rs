#[doc = "Reader of register SYSCTRL_MEM_POWER_CFG"]
pub type R = crate::R<u32, super::SYSCTRL_MEM_POWER_CFG>;
#[doc = "Writer for register SYSCTRL_MEM_POWER_CFG"]
pub type W = crate::W<u32, super::SYSCTRL_MEM_POWER_CFG>;
#[doc = "Register SYSCTRL_MEM_POWER_CFG `reset()`'s with value 0x003f_0dfd"]
impl crate::ResetValue for super::SYSCTRL_MEM_POWER_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x003f_0dfd
    }
}
#[doc = "DSP PRAM0 power configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_DRAM5_POWER_A {
    #[doc = "0: DSP DRAM5 power disabled"]
    DSP_DRAM5_POWER_DISABLE = 0,
    #[doc = "1: DSP DRAM5 power enabled"]
    DSP_DRAM5_POWER_ENABLE = 1,
}
impl From<DSP_DRAM5_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_DRAM5_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_DRAM5_POWER`"]
pub type DSP_DRAM5_POWER_R = crate::R<bool, DSP_DRAM5_POWER_A>;
impl DSP_DRAM5_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_DRAM5_POWER_A {
        match self.bits {
            false => DSP_DRAM5_POWER_A::DSP_DRAM5_POWER_DISABLE,
            true => DSP_DRAM5_POWER_A::DSP_DRAM5_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM5_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram5_power_disable(&self) -> bool {
        *self == DSP_DRAM5_POWER_A::DSP_DRAM5_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM5_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram5_power_enable(&self) -> bool {
        *self == DSP_DRAM5_POWER_A::DSP_DRAM5_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_DRAM5_POWER`"]
pub struct DSP_DRAM5_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM5_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM5_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP DRAM5 power disabled"]
    #[inline(always)]
    pub fn dsp_dram5_power_disable(self) -> &'a mut W {
        self.variant(DSP_DRAM5_POWER_A::DSP_DRAM5_POWER_DISABLE)
    }
    #[doc = "DSP DRAM5 power enabled"]
    #[inline(always)]
    pub fn dsp_dram5_power_enable(self) -> &'a mut W {
        self.variant(DSP_DRAM5_POWER_A::DSP_DRAM5_POWER_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "DSP PRAM0 power configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_DRAM4_POWER_A {
    #[doc = "0: DSP DRAM4 power disabled"]
    DSP_DRAM4_POWER_DISABLE = 0,
    #[doc = "1: DSP DRAM4 power enabled"]
    DSP_DRAM4_POWER_ENABLE = 1,
}
impl From<DSP_DRAM4_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_DRAM4_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_DRAM4_POWER`"]
pub type DSP_DRAM4_POWER_R = crate::R<bool, DSP_DRAM4_POWER_A>;
impl DSP_DRAM4_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_DRAM4_POWER_A {
        match self.bits {
            false => DSP_DRAM4_POWER_A::DSP_DRAM4_POWER_DISABLE,
            true => DSP_DRAM4_POWER_A::DSP_DRAM4_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM4_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram4_power_disable(&self) -> bool {
        *self == DSP_DRAM4_POWER_A::DSP_DRAM4_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM4_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram4_power_enable(&self) -> bool {
        *self == DSP_DRAM4_POWER_A::DSP_DRAM4_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_DRAM4_POWER`"]
pub struct DSP_DRAM4_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM4_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM4_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP DRAM4 power disabled"]
    #[inline(always)]
    pub fn dsp_dram4_power_disable(self) -> &'a mut W {
        self.variant(DSP_DRAM4_POWER_A::DSP_DRAM4_POWER_DISABLE)
    }
    #[doc = "DSP DRAM4 power enabled"]
    #[inline(always)]
    pub fn dsp_dram4_power_enable(self) -> &'a mut W {
        self.variant(DSP_DRAM4_POWER_A::DSP_DRAM4_POWER_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "DSP PRAM0 power configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_DRAM3_POWER_A {
    #[doc = "0: DSP DRAM3 power disabled"]
    DSP_DRAM3_POWER_DISABLE = 0,
    #[doc = "1: DSP DRAM3 power enabled"]
    DSP_DRAM3_POWER_ENABLE = 1,
}
impl From<DSP_DRAM3_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_DRAM3_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_DRAM3_POWER`"]
pub type DSP_DRAM3_POWER_R = crate::R<bool, DSP_DRAM3_POWER_A>;
impl DSP_DRAM3_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_DRAM3_POWER_A {
        match self.bits {
            false => DSP_DRAM3_POWER_A::DSP_DRAM3_POWER_DISABLE,
            true => DSP_DRAM3_POWER_A::DSP_DRAM3_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM3_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram3_power_disable(&self) -> bool {
        *self == DSP_DRAM3_POWER_A::DSP_DRAM3_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM3_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram3_power_enable(&self) -> bool {
        *self == DSP_DRAM3_POWER_A::DSP_DRAM3_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_DRAM3_POWER`"]
pub struct DSP_DRAM3_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM3_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM3_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP DRAM3 power disabled"]
    #[inline(always)]
    pub fn dsp_dram3_power_disable(self) -> &'a mut W {
        self.variant(DSP_DRAM3_POWER_A::DSP_DRAM3_POWER_DISABLE)
    }
    #[doc = "DSP DRAM3 power enabled"]
    #[inline(always)]
    pub fn dsp_dram3_power_enable(self) -> &'a mut W {
        self.variant(DSP_DRAM3_POWER_A::DSP_DRAM3_POWER_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "DSP PRAM0 power configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_DRAM2_POWER_A {
    #[doc = "0: DSP DRAM2 power disabled"]
    DSP_DRAM2_POWER_DISABLE = 0,
    #[doc = "1: DSP DRAM2 power enabled"]
    DSP_DRAM2_POWER_ENABLE = 1,
}
impl From<DSP_DRAM2_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_DRAM2_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_DRAM2_POWER`"]
pub type DSP_DRAM2_POWER_R = crate::R<bool, DSP_DRAM2_POWER_A>;
impl DSP_DRAM2_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_DRAM2_POWER_A {
        match self.bits {
            false => DSP_DRAM2_POWER_A::DSP_DRAM2_POWER_DISABLE,
            true => DSP_DRAM2_POWER_A::DSP_DRAM2_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM2_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram2_power_disable(&self) -> bool {
        *self == DSP_DRAM2_POWER_A::DSP_DRAM2_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM2_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram2_power_enable(&self) -> bool {
        *self == DSP_DRAM2_POWER_A::DSP_DRAM2_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_DRAM2_POWER`"]
pub struct DSP_DRAM2_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM2_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM2_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP DRAM2 power disabled"]
    #[inline(always)]
    pub fn dsp_dram2_power_disable(self) -> &'a mut W {
        self.variant(DSP_DRAM2_POWER_A::DSP_DRAM2_POWER_DISABLE)
    }
    #[doc = "DSP DRAM2 power enabled"]
    #[inline(always)]
    pub fn dsp_dram2_power_enable(self) -> &'a mut W {
        self.variant(DSP_DRAM2_POWER_A::DSP_DRAM2_POWER_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "DSP PRAM0 power configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_DRAM1_POWER_A {
    #[doc = "0: DSP DRAM1 power disabled"]
    DSP_DRAM1_POWER_DISABLE = 0,
    #[doc = "1: DSP DRAM1 power enabled"]
    DSP_DRAM1_POWER_ENABLE = 1,
}
impl From<DSP_DRAM1_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_DRAM1_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_DRAM1_POWER`"]
pub type DSP_DRAM1_POWER_R = crate::R<bool, DSP_DRAM1_POWER_A>;
impl DSP_DRAM1_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_DRAM1_POWER_A {
        match self.bits {
            false => DSP_DRAM1_POWER_A::DSP_DRAM1_POWER_DISABLE,
            true => DSP_DRAM1_POWER_A::DSP_DRAM1_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM1_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram1_power_disable(&self) -> bool {
        *self == DSP_DRAM1_POWER_A::DSP_DRAM1_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM1_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram1_power_enable(&self) -> bool {
        *self == DSP_DRAM1_POWER_A::DSP_DRAM1_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_DRAM1_POWER`"]
pub struct DSP_DRAM1_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM1_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM1_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP DRAM1 power disabled"]
    #[inline(always)]
    pub fn dsp_dram1_power_disable(self) -> &'a mut W {
        self.variant(DSP_DRAM1_POWER_A::DSP_DRAM1_POWER_DISABLE)
    }
    #[doc = "DSP DRAM1 power enabled"]
    #[inline(always)]
    pub fn dsp_dram1_power_enable(self) -> &'a mut W {
        self.variant(DSP_DRAM1_POWER_A::DSP_DRAM1_POWER_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "DSP PRAM0 power configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_DRAM0_POWER_A {
    #[doc = "0: DSP DRAM0 power disabled"]
    DSP_DRAM0_POWER_DISABLE = 0,
    #[doc = "1: DSP DRAM0 power enabled"]
    DSP_DRAM0_POWER_ENABLE = 1,
}
impl From<DSP_DRAM0_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_DRAM0_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_DRAM0_POWER`"]
pub type DSP_DRAM0_POWER_R = crate::R<bool, DSP_DRAM0_POWER_A>;
impl DSP_DRAM0_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_DRAM0_POWER_A {
        match self.bits {
            false => DSP_DRAM0_POWER_A::DSP_DRAM0_POWER_DISABLE,
            true => DSP_DRAM0_POWER_A::DSP_DRAM0_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM0_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram0_power_disable(&self) -> bool {
        *self == DSP_DRAM0_POWER_A::DSP_DRAM0_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM0_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram0_power_enable(&self) -> bool {
        *self == DSP_DRAM0_POWER_A::DSP_DRAM0_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_DRAM0_POWER`"]
pub struct DSP_DRAM0_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM0_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM0_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP DRAM0 power disabled"]
    #[inline(always)]
    pub fn dsp_dram0_power_disable(self) -> &'a mut W {
        self.variant(DSP_DRAM0_POWER_A::DSP_DRAM0_POWER_DISABLE)
    }
    #[doc = "DSP DRAM0 power enabled"]
    #[inline(always)]
    pub fn dsp_dram0_power_enable(self) -> &'a mut W {
        self.variant(DSP_DRAM0_POWER_A::DSP_DRAM0_POWER_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "DSP PRAM0 power configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_PRAM3_POWER_A {
    #[doc = "0: DSP PRAM3 power disabled"]
    DSP_PRAM3_POWER_DISABLE = 0,
    #[doc = "1: DSP PRAM3 power enabled"]
    DSP_PRAM3_POWER_ENABLE = 1,
}
impl From<DSP_PRAM3_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_PRAM3_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_PRAM3_POWER`"]
pub type DSP_PRAM3_POWER_R = crate::R<bool, DSP_PRAM3_POWER_A>;
impl DSP_PRAM3_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_PRAM3_POWER_A {
        match self.bits {
            false => DSP_PRAM3_POWER_A::DSP_PRAM3_POWER_DISABLE,
            true => DSP_PRAM3_POWER_A::DSP_PRAM3_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM3_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram3_power_disable(&self) -> bool {
        *self == DSP_PRAM3_POWER_A::DSP_PRAM3_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM3_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram3_power_enable(&self) -> bool {
        *self == DSP_PRAM3_POWER_A::DSP_PRAM3_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_PRAM3_POWER`"]
pub struct DSP_PRAM3_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM3_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM3_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP PRAM3 power disabled"]
    #[inline(always)]
    pub fn dsp_pram3_power_disable(self) -> &'a mut W {
        self.variant(DSP_PRAM3_POWER_A::DSP_PRAM3_POWER_DISABLE)
    }
    #[doc = "DSP PRAM3 power enabled"]
    #[inline(always)]
    pub fn dsp_pram3_power_enable(self) -> &'a mut W {
        self.variant(DSP_PRAM3_POWER_A::DSP_PRAM3_POWER_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "DSP PRAM0 power configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_PRAM2_POWER_A {
    #[doc = "0: DSP PRAM2 power disabled"]
    DSP_PRAM2_POWER_DISABLE = 0,
    #[doc = "1: DSP PRAM2 power enabled"]
    DSP_PRAM2_POWER_ENABLE = 1,
}
impl From<DSP_PRAM2_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_PRAM2_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_PRAM2_POWER`"]
pub type DSP_PRAM2_POWER_R = crate::R<bool, DSP_PRAM2_POWER_A>;
impl DSP_PRAM2_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_PRAM2_POWER_A {
        match self.bits {
            false => DSP_PRAM2_POWER_A::DSP_PRAM2_POWER_DISABLE,
            true => DSP_PRAM2_POWER_A::DSP_PRAM2_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM2_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram2_power_disable(&self) -> bool {
        *self == DSP_PRAM2_POWER_A::DSP_PRAM2_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM2_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram2_power_enable(&self) -> bool {
        *self == DSP_PRAM2_POWER_A::DSP_PRAM2_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_PRAM2_POWER`"]
pub struct DSP_PRAM2_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM2_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM2_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP PRAM2 power disabled"]
    #[inline(always)]
    pub fn dsp_pram2_power_disable(self) -> &'a mut W {
        self.variant(DSP_PRAM2_POWER_A::DSP_PRAM2_POWER_DISABLE)
    }
    #[doc = "DSP PRAM2 power enabled"]
    #[inline(always)]
    pub fn dsp_pram2_power_enable(self) -> &'a mut W {
        self.variant(DSP_PRAM2_POWER_A::DSP_PRAM2_POWER_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "DSP PRAM0 power configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_PRAM1_POWER_A {
    #[doc = "0: DSP PRAM1 power disabled"]
    DSP_PRAM1_POWER_DISABLE = 0,
    #[doc = "1: DSP PRAM1 power enabled"]
    DSP_PRAM1_POWER_ENABLE = 1,
}
impl From<DSP_PRAM1_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_PRAM1_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_PRAM1_POWER`"]
pub type DSP_PRAM1_POWER_R = crate::R<bool, DSP_PRAM1_POWER_A>;
impl DSP_PRAM1_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_PRAM1_POWER_A {
        match self.bits {
            false => DSP_PRAM1_POWER_A::DSP_PRAM1_POWER_DISABLE,
            true => DSP_PRAM1_POWER_A::DSP_PRAM1_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM1_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram1_power_disable(&self) -> bool {
        *self == DSP_PRAM1_POWER_A::DSP_PRAM1_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM1_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram1_power_enable(&self) -> bool {
        *self == DSP_PRAM1_POWER_A::DSP_PRAM1_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_PRAM1_POWER`"]
pub struct DSP_PRAM1_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM1_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM1_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP PRAM1 power disabled"]
    #[inline(always)]
    pub fn dsp_pram1_power_disable(self) -> &'a mut W {
        self.variant(DSP_PRAM1_POWER_A::DSP_PRAM1_POWER_DISABLE)
    }
    #[doc = "DSP PRAM1 power enabled"]
    #[inline(always)]
    pub fn dsp_pram1_power_enable(self) -> &'a mut W {
        self.variant(DSP_PRAM1_POWER_A::DSP_PRAM1_POWER_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "DSP PRAM0 power configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_PRAM0_POWER_A {
    #[doc = "0: DSP PRAM0 power disabled"]
    DSP_PRAM0_POWER_DISABLE = 0,
    #[doc = "1: DSP PRAM0 power enabled"]
    DSP_PRAM0_POWER_ENABLE = 1,
}
impl From<DSP_PRAM0_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_PRAM0_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_PRAM0_POWER`"]
pub type DSP_PRAM0_POWER_R = crate::R<bool, DSP_PRAM0_POWER_A>;
impl DSP_PRAM0_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_PRAM0_POWER_A {
        match self.bits {
            false => DSP_PRAM0_POWER_A::DSP_PRAM0_POWER_DISABLE,
            true => DSP_PRAM0_POWER_A::DSP_PRAM0_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM0_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram0_power_disable(&self) -> bool {
        *self == DSP_PRAM0_POWER_A::DSP_PRAM0_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM0_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram0_power_enable(&self) -> bool {
        *self == DSP_PRAM0_POWER_A::DSP_PRAM0_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_PRAM0_POWER`"]
pub struct DSP_PRAM0_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM0_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM0_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP PRAM0 power disabled"]
    #[inline(always)]
    pub fn dsp_pram0_power_disable(self) -> &'a mut W {
        self.variant(DSP_PRAM0_POWER_A::DSP_PRAM0_POWER_DISABLE)
    }
    #[doc = "DSP PRAM0 power enabled"]
    #[inline(always)]
    pub fn dsp_pram0_power_enable(self) -> &'a mut W {
        self.variant(DSP_PRAM0_POWER_A::DSP_PRAM0_POWER_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Baseband DRAM1 power configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_DRAM1_POWER_A {
    #[doc = "0: Baseband DRAM1 power disabled"]
    BB_DRAM1_POWER_DISABLE = 0,
    #[doc = "1: Baseband DRAM1 power enabled"]
    BB_DRAM1_POWER_ENABLE = 1,
}
impl From<BB_DRAM1_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: BB_DRAM1_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BB_DRAM1_POWER`"]
pub type BB_DRAM1_POWER_R = crate::R<bool, BB_DRAM1_POWER_A>;
impl BB_DRAM1_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_DRAM1_POWER_A {
        match self.bits {
            false => BB_DRAM1_POWER_A::BB_DRAM1_POWER_DISABLE,
            true => BB_DRAM1_POWER_A::BB_DRAM1_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BB_DRAM1_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_bb_dram1_power_disable(&self) -> bool {
        *self == BB_DRAM1_POWER_A::BB_DRAM1_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `BB_DRAM1_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_bb_dram1_power_enable(&self) -> bool {
        *self == BB_DRAM1_POWER_A::BB_DRAM1_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `BB_DRAM1_POWER`"]
pub struct BB_DRAM1_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_DRAM1_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_DRAM1_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Baseband DRAM1 power disabled"]
    #[inline(always)]
    pub fn bb_dram1_power_disable(self) -> &'a mut W {
        self.variant(BB_DRAM1_POWER_A::BB_DRAM1_POWER_DISABLE)
    }
    #[doc = "Baseband DRAM1 power enabled"]
    #[inline(always)]
    pub fn bb_dram1_power_enable(self) -> &'a mut W {
        self.variant(BB_DRAM1_POWER_A::BB_DRAM1_POWER_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Baseband DRAM0 power configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_DRAM0_POWER_A {
    #[doc = "0: Baseband DRAM0 power disabled"]
    BB_DRAM0_POWER_DISABLE = 0,
    #[doc = "1: Baseband DRAM0 power enabled"]
    BB_DRAM0_POWER_ENABLE = 1,
}
impl From<BB_DRAM0_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: BB_DRAM0_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BB_DRAM0_POWER`"]
pub type BB_DRAM0_POWER_R = crate::R<bool, BB_DRAM0_POWER_A>;
impl BB_DRAM0_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_DRAM0_POWER_A {
        match self.bits {
            false => BB_DRAM0_POWER_A::BB_DRAM0_POWER_DISABLE,
            true => BB_DRAM0_POWER_A::BB_DRAM0_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BB_DRAM0_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_bb_dram0_power_disable(&self) -> bool {
        *self == BB_DRAM0_POWER_A::BB_DRAM0_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `BB_DRAM0_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_bb_dram0_power_enable(&self) -> bool {
        *self == BB_DRAM0_POWER_A::BB_DRAM0_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `BB_DRAM0_POWER`"]
pub struct BB_DRAM0_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_DRAM0_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_DRAM0_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Baseband DRAM0 power disabled"]
    #[inline(always)]
    pub fn bb_dram0_power_disable(self) -> &'a mut W {
        self.variant(BB_DRAM0_POWER_A::BB_DRAM0_POWER_DISABLE)
    }
    #[doc = "Baseband DRAM0 power enabled"]
    #[inline(always)]
    pub fn bb_dram0_power_enable(self) -> &'a mut W {
        self.variant(BB_DRAM0_POWER_A::BB_DRAM0_POWER_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "DRAM2 power configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRAM2_POWER_A {
    #[doc = "0: DRAM2 power disabled"]
    DRAM2_POWER_DISABLE = 0,
    #[doc = "1: DRAM2 power enabled"]
    DRAM2_POWER_ENABLE = 1,
}
impl From<DRAM2_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: DRAM2_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRAM2_POWER`"]
pub type DRAM2_POWER_R = crate::R<bool, DRAM2_POWER_A>;
impl DRAM2_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRAM2_POWER_A {
        match self.bits {
            false => DRAM2_POWER_A::DRAM2_POWER_DISABLE,
            true => DRAM2_POWER_A::DRAM2_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DRAM2_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_dram2_power_disable(&self) -> bool {
        *self == DRAM2_POWER_A::DRAM2_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `DRAM2_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_dram2_power_enable(&self) -> bool {
        *self == DRAM2_POWER_A::DRAM2_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `DRAM2_POWER`"]
pub struct DRAM2_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> DRAM2_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRAM2_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DRAM2 power disabled"]
    #[inline(always)]
    pub fn dram2_power_disable(self) -> &'a mut W {
        self.variant(DRAM2_POWER_A::DRAM2_POWER_DISABLE)
    }
    #[doc = "DRAM2 power enabled"]
    #[inline(always)]
    pub fn dram2_power_enable(self) -> &'a mut W {
        self.variant(DRAM2_POWER_A::DRAM2_POWER_ENABLE)
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
#[doc = "DRAM1 power configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRAM1_POWER_A {
    #[doc = "0: DRAM1 power disabled"]
    DRAM1_POWER_DISABLE = 0,
    #[doc = "1: DRAM1 power enabled"]
    DRAM1_POWER_ENABLE = 1,
}
impl From<DRAM1_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: DRAM1_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRAM1_POWER`"]
pub type DRAM1_POWER_R = crate::R<bool, DRAM1_POWER_A>;
impl DRAM1_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRAM1_POWER_A {
        match self.bits {
            false => DRAM1_POWER_A::DRAM1_POWER_DISABLE,
            true => DRAM1_POWER_A::DRAM1_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DRAM1_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_dram1_power_disable(&self) -> bool {
        *self == DRAM1_POWER_A::DRAM1_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `DRAM1_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_dram1_power_enable(&self) -> bool {
        *self == DRAM1_POWER_A::DRAM1_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `DRAM1_POWER`"]
pub struct DRAM1_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> DRAM1_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRAM1_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DRAM1 power disabled"]
    #[inline(always)]
    pub fn dram1_power_disable(self) -> &'a mut W {
        self.variant(DRAM1_POWER_A::DRAM1_POWER_DISABLE)
    }
    #[doc = "DRAM1 power enabled"]
    #[inline(always)]
    pub fn dram1_power_enable(self) -> &'a mut W {
        self.variant(DRAM1_POWER_A::DRAM1_POWER_ENABLE)
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
#[doc = "DRAM0 power configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRAM0_POWER_A {
    #[doc = "0: DRAM0 power disabled"]
    DRAM0_POWER_DISABLE = 0,
    #[doc = "1: DRAM0 power enabled"]
    DRAM0_POWER_ENABLE = 1,
}
impl From<DRAM0_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: DRAM0_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRAM0_POWER`"]
pub type DRAM0_POWER_R = crate::R<bool, DRAM0_POWER_A>;
impl DRAM0_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRAM0_POWER_A {
        match self.bits {
            false => DRAM0_POWER_A::DRAM0_POWER_DISABLE,
            true => DRAM0_POWER_A::DRAM0_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DRAM0_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_dram0_power_disable(&self) -> bool {
        *self == DRAM0_POWER_A::DRAM0_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `DRAM0_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_dram0_power_enable(&self) -> bool {
        *self == DRAM0_POWER_A::DRAM0_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `DRAM0_POWER`"]
pub struct DRAM0_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> DRAM0_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRAM0_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DRAM0 power disabled"]
    #[inline(always)]
    pub fn dram0_power_disable(self) -> &'a mut W {
        self.variant(DRAM0_POWER_A::DRAM0_POWER_DISABLE)
    }
    #[doc = "DRAM0 power enabled"]
    #[inline(always)]
    pub fn dram0_power_enable(self) -> &'a mut W {
        self.variant(DRAM0_POWER_A::DRAM0_POWER_ENABLE)
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
#[doc = "PRAM3 power configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRAM3_POWER_A {
    #[doc = "0: PRAM3 power disabled"]
    PRAM3_POWER_DISABLE = 0,
    #[doc = "1: PRAM3 power enabled"]
    PRAM3_POWER_ENABLE = 1,
}
impl From<PRAM3_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: PRAM3_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRAM3_POWER`"]
pub type PRAM3_POWER_R = crate::R<bool, PRAM3_POWER_A>;
impl PRAM3_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRAM3_POWER_A {
        match self.bits {
            false => PRAM3_POWER_A::PRAM3_POWER_DISABLE,
            true => PRAM3_POWER_A::PRAM3_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PRAM3_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_pram3_power_disable(&self) -> bool {
        *self == PRAM3_POWER_A::PRAM3_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `PRAM3_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_pram3_power_enable(&self) -> bool {
        *self == PRAM3_POWER_A::PRAM3_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `PRAM3_POWER`"]
pub struct PRAM3_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRAM3_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRAM3_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PRAM3 power disabled"]
    #[inline(always)]
    pub fn pram3_power_disable(self) -> &'a mut W {
        self.variant(PRAM3_POWER_A::PRAM3_POWER_DISABLE)
    }
    #[doc = "PRAM3 power enabled"]
    #[inline(always)]
    pub fn pram3_power_enable(self) -> &'a mut W {
        self.variant(PRAM3_POWER_A::PRAM3_POWER_ENABLE)
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
#[doc = "PRAM2 power configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRAM2_POWER_A {
    #[doc = "0: PRAM2 power disabled"]
    PRAM2_POWER_DISABLE = 0,
    #[doc = "1: PRAM2 power enabled"]
    PRAM2_POWER_ENABLE = 1,
}
impl From<PRAM2_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: PRAM2_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRAM2_POWER`"]
pub type PRAM2_POWER_R = crate::R<bool, PRAM2_POWER_A>;
impl PRAM2_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRAM2_POWER_A {
        match self.bits {
            false => PRAM2_POWER_A::PRAM2_POWER_DISABLE,
            true => PRAM2_POWER_A::PRAM2_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PRAM2_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_pram2_power_disable(&self) -> bool {
        *self == PRAM2_POWER_A::PRAM2_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `PRAM2_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_pram2_power_enable(&self) -> bool {
        *self == PRAM2_POWER_A::PRAM2_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `PRAM2_POWER`"]
pub struct PRAM2_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRAM2_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRAM2_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PRAM2 power disabled"]
    #[inline(always)]
    pub fn pram2_power_disable(self) -> &'a mut W {
        self.variant(PRAM2_POWER_A::PRAM2_POWER_DISABLE)
    }
    #[doc = "PRAM2 power enabled"]
    #[inline(always)]
    pub fn pram2_power_enable(self) -> &'a mut W {
        self.variant(PRAM2_POWER_A::PRAM2_POWER_ENABLE)
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
#[doc = "PRAM1 power configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRAM1_POWER_A {
    #[doc = "0: PRAM1 power disabled"]
    PRAM1_POWER_DISABLE = 0,
    #[doc = "1: PRAM1 power enabled"]
    PRAM1_POWER_ENABLE = 1,
}
impl From<PRAM1_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: PRAM1_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRAM1_POWER`"]
pub type PRAM1_POWER_R = crate::R<bool, PRAM1_POWER_A>;
impl PRAM1_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRAM1_POWER_A {
        match self.bits {
            false => PRAM1_POWER_A::PRAM1_POWER_DISABLE,
            true => PRAM1_POWER_A::PRAM1_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PRAM1_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_pram1_power_disable(&self) -> bool {
        *self == PRAM1_POWER_A::PRAM1_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `PRAM1_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_pram1_power_enable(&self) -> bool {
        *self == PRAM1_POWER_A::PRAM1_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `PRAM1_POWER`"]
pub struct PRAM1_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRAM1_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRAM1_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PRAM1 power disabled"]
    #[inline(always)]
    pub fn pram1_power_disable(self) -> &'a mut W {
        self.variant(PRAM1_POWER_A::PRAM1_POWER_DISABLE)
    }
    #[doc = "PRAM1 power enabled"]
    #[inline(always)]
    pub fn pram1_power_enable(self) -> &'a mut W {
        self.variant(PRAM1_POWER_A::PRAM1_POWER_ENABLE)
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
#[doc = "PRAM0 power configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRAM0_POWER_A {
    #[doc = "0: PRAM0 power disabled"]
    PRAM0_POWER_DISABLE = 0,
    #[doc = "1: PRAM0 power enabled"]
    PRAM0_POWER_ENABLE = 1,
}
impl From<PRAM0_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: PRAM0_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRAM0_POWER`"]
pub type PRAM0_POWER_R = crate::R<bool, PRAM0_POWER_A>;
impl PRAM0_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRAM0_POWER_A {
        match self.bits {
            false => PRAM0_POWER_A::PRAM0_POWER_DISABLE,
            true => PRAM0_POWER_A::PRAM0_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PRAM0_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_pram0_power_disable(&self) -> bool {
        *self == PRAM0_POWER_A::PRAM0_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `PRAM0_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_pram0_power_enable(&self) -> bool {
        *self == PRAM0_POWER_A::PRAM0_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `PRAM0_POWER`"]
pub struct PRAM0_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRAM0_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRAM0_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PRAM0 power disabled"]
    #[inline(always)]
    pub fn pram0_power_disable(self) -> &'a mut W {
        self.variant(PRAM0_POWER_A::PRAM0_POWER_DISABLE)
    }
    #[doc = "PRAM0 power enabled"]
    #[inline(always)]
    pub fn pram0_power_enable(self) -> &'a mut W {
        self.variant(PRAM0_POWER_A::PRAM0_POWER_ENABLE)
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
#[doc = "Flash power configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_POWER_A {
    #[doc = "0: Flash power disabled"]
    FLASH_POWER_DISABLE = 0,
    #[doc = "1: Flash power enabled"]
    FLASH_POWER_ENABLE = 1,
}
impl From<FLASH_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLASH_POWER`"]
pub type FLASH_POWER_R = crate::R<bool, FLASH_POWER_A>;
impl FLASH_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_POWER_A {
        match self.bits {
            false => FLASH_POWER_A::FLASH_POWER_DISABLE,
            true => FLASH_POWER_A::FLASH_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_flash_power_disable(&self) -> bool {
        *self == FLASH_POWER_A::FLASH_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `FLASH_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_flash_power_enable(&self) -> bool {
        *self == FLASH_POWER_A::FLASH_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `FLASH_POWER`"]
pub struct FLASH_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash power disabled"]
    #[inline(always)]
    pub fn flash_power_disable(self) -> &'a mut W {
        self.variant(FLASH_POWER_A::FLASH_POWER_DISABLE)
    }
    #[doc = "Flash power enabled"]
    #[inline(always)]
    pub fn flash_power_enable(self) -> &'a mut W {
        self.variant(FLASH_POWER_A::FLASH_POWER_ENABLE)
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
#[doc = "PROM power configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROM_POWER_A {
    #[doc = "0: PROM power disabled"]
    PROM_POWER_DISABLE = 0,
    #[doc = "1: PROM power enabled"]
    PROM_POWER_ENABLE = 1,
}
impl From<PROM_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: PROM_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PROM_POWER`"]
pub type PROM_POWER_R = crate::R<bool, PROM_POWER_A>;
impl PROM_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROM_POWER_A {
        match self.bits {
            false => PROM_POWER_A::PROM_POWER_DISABLE,
            true => PROM_POWER_A::PROM_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PROM_POWER_DISABLE`"]
    #[inline(always)]
    pub fn is_prom_power_disable(&self) -> bool {
        *self == PROM_POWER_A::PROM_POWER_DISABLE
    }
    #[doc = "Checks if the value of the field is `PROM_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_prom_power_enable(&self) -> bool {
        *self == PROM_POWER_A::PROM_POWER_ENABLE
    }
}
#[doc = "Write proxy for field `PROM_POWER`"]
pub struct PROM_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> PROM_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROM_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PROM power disabled"]
    #[inline(always)]
    pub fn prom_power_disable(self) -> &'a mut W {
        self.variant(PROM_POWER_A::PROM_POWER_DISABLE)
    }
    #[doc = "PROM power enabled"]
    #[inline(always)]
    pub fn prom_power_enable(self) -> &'a mut W {
        self.variant(PROM_POWER_A::PROM_POWER_ENABLE)
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
    #[doc = "Bit 21 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_dram5_power(&self) -> DSP_DRAM5_POWER_R {
        DSP_DRAM5_POWER_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_dram4_power(&self) -> DSP_DRAM4_POWER_R {
        DSP_DRAM4_POWER_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_dram3_power(&self) -> DSP_DRAM3_POWER_R {
        DSP_DRAM3_POWER_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_dram2_power(&self) -> DSP_DRAM2_POWER_R {
        DSP_DRAM2_POWER_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_dram1_power(&self) -> DSP_DRAM1_POWER_R {
        DSP_DRAM1_POWER_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_dram0_power(&self) -> DSP_DRAM0_POWER_R {
        DSP_DRAM0_POWER_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_pram3_power(&self) -> DSP_PRAM3_POWER_R {
        DSP_PRAM3_POWER_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_pram2_power(&self) -> DSP_PRAM2_POWER_R {
        DSP_PRAM2_POWER_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_pram1_power(&self) -> DSP_PRAM1_POWER_R {
        DSP_PRAM1_POWER_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_pram0_power(&self) -> DSP_PRAM0_POWER_R {
        DSP_PRAM0_POWER_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Baseband DRAM1 power configuration"]
    #[inline(always)]
    pub fn bb_dram1_power(&self) -> BB_DRAM1_POWER_R {
        BB_DRAM1_POWER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Baseband DRAM0 power configuration"]
    #[inline(always)]
    pub fn bb_dram0_power(&self) -> BB_DRAM0_POWER_R {
        BB_DRAM0_POWER_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DRAM2 power configuration"]
    #[inline(always)]
    pub fn dram2_power(&self) -> DRAM2_POWER_R {
        DRAM2_POWER_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DRAM1 power configuration"]
    #[inline(always)]
    pub fn dram1_power(&self) -> DRAM1_POWER_R {
        DRAM1_POWER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DRAM0 power configuration"]
    #[inline(always)]
    pub fn dram0_power(&self) -> DRAM0_POWER_R {
        DRAM0_POWER_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PRAM3 power configuration"]
    #[inline(always)]
    pub fn pram3_power(&self) -> PRAM3_POWER_R {
        PRAM3_POWER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PRAM2 power configuration"]
    #[inline(always)]
    pub fn pram2_power(&self) -> PRAM2_POWER_R {
        PRAM2_POWER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PRAM1 power configuration"]
    #[inline(always)]
    pub fn pram1_power(&self) -> PRAM1_POWER_R {
        PRAM1_POWER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PRAM0 power configuration"]
    #[inline(always)]
    pub fn pram0_power(&self) -> PRAM0_POWER_R {
        PRAM0_POWER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Flash power configuration"]
    #[inline(always)]
    pub fn flash_power(&self) -> FLASH_POWER_R {
        FLASH_POWER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PROM power configuration"]
    #[inline(always)]
    pub fn prom_power(&self) -> PROM_POWER_R {
        PROM_POWER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_dram5_power(&mut self) -> DSP_DRAM5_POWER_W {
        DSP_DRAM5_POWER_W { w: self }
    }
    #[doc = "Bit 20 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_dram4_power(&mut self) -> DSP_DRAM4_POWER_W {
        DSP_DRAM4_POWER_W { w: self }
    }
    #[doc = "Bit 19 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_dram3_power(&mut self) -> DSP_DRAM3_POWER_W {
        DSP_DRAM3_POWER_W { w: self }
    }
    #[doc = "Bit 18 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_dram2_power(&mut self) -> DSP_DRAM2_POWER_W {
        DSP_DRAM2_POWER_W { w: self }
    }
    #[doc = "Bit 17 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_dram1_power(&mut self) -> DSP_DRAM1_POWER_W {
        DSP_DRAM1_POWER_W { w: self }
    }
    #[doc = "Bit 16 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_dram0_power(&mut self) -> DSP_DRAM0_POWER_W {
        DSP_DRAM0_POWER_W { w: self }
    }
    #[doc = "Bit 15 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_pram3_power(&mut self) -> DSP_PRAM3_POWER_W {
        DSP_PRAM3_POWER_W { w: self }
    }
    #[doc = "Bit 14 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_pram2_power(&mut self) -> DSP_PRAM2_POWER_W {
        DSP_PRAM2_POWER_W { w: self }
    }
    #[doc = "Bit 13 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_pram1_power(&mut self) -> DSP_PRAM1_POWER_W {
        DSP_PRAM1_POWER_W { w: self }
    }
    #[doc = "Bit 12 - DSP PRAM0 power configuration"]
    #[inline(always)]
    pub fn dsp_pram0_power(&mut self) -> DSP_PRAM0_POWER_W {
        DSP_PRAM0_POWER_W { w: self }
    }
    #[doc = "Bit 11 - Baseband DRAM1 power configuration"]
    #[inline(always)]
    pub fn bb_dram1_power(&mut self) -> BB_DRAM1_POWER_W {
        BB_DRAM1_POWER_W { w: self }
    }
    #[doc = "Bit 10 - Baseband DRAM0 power configuration"]
    #[inline(always)]
    pub fn bb_dram0_power(&mut self) -> BB_DRAM0_POWER_W {
        BB_DRAM0_POWER_W { w: self }
    }
    #[doc = "Bit 8 - DRAM2 power configuration"]
    #[inline(always)]
    pub fn dram2_power(&mut self) -> DRAM2_POWER_W {
        DRAM2_POWER_W { w: self }
    }
    #[doc = "Bit 7 - DRAM1 power configuration"]
    #[inline(always)]
    pub fn dram1_power(&mut self) -> DRAM1_POWER_W {
        DRAM1_POWER_W { w: self }
    }
    #[doc = "Bit 6 - DRAM0 power configuration"]
    #[inline(always)]
    pub fn dram0_power(&mut self) -> DRAM0_POWER_W {
        DRAM0_POWER_W { w: self }
    }
    #[doc = "Bit 5 - PRAM3 power configuration"]
    #[inline(always)]
    pub fn pram3_power(&mut self) -> PRAM3_POWER_W {
        PRAM3_POWER_W { w: self }
    }
    #[doc = "Bit 4 - PRAM2 power configuration"]
    #[inline(always)]
    pub fn pram2_power(&mut self) -> PRAM2_POWER_W {
        PRAM2_POWER_W { w: self }
    }
    #[doc = "Bit 3 - PRAM1 power configuration"]
    #[inline(always)]
    pub fn pram1_power(&mut self) -> PRAM1_POWER_W {
        PRAM1_POWER_W { w: self }
    }
    #[doc = "Bit 2 - PRAM0 power configuration"]
    #[inline(always)]
    pub fn pram0_power(&mut self) -> PRAM0_POWER_W {
        PRAM0_POWER_W { w: self }
    }
    #[doc = "Bit 1 - Flash power configuration"]
    #[inline(always)]
    pub fn flash_power(&mut self) -> FLASH_POWER_W {
        FLASH_POWER_W { w: self }
    }
    #[doc = "Bit 0 - PROM power configuration"]
    #[inline(always)]
    pub fn prom_power(&mut self) -> PROM_POWER_W {
        PROM_POWER_W { w: self }
    }
}
