#[doc = "Reader of register SYSCTRL_MEM_RETENTION_CFG"]
pub type R = crate::R<u32, super::SYSCTRL_MEM_RETENTION_CFG>;
#[doc = "Writer for register SYSCTRL_MEM_RETENTION_CFG"]
pub type W = crate::W<u32, super::SYSCTRL_MEM_RETENTION_CFG>;
#[doc = "Register SYSCTRL_MEM_RETENTION_CFG `reset()`'s with value 0x003f_0dbc"]
impl crate::ResetValue for super::SYSCTRL_MEM_RETENTION_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x003f_0dbc
    }
}
#[doc = "DSP PRAM0 retention configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_DRAM5_RETENTION_A {
    #[doc = "0: DSP DRAM5 normal mode"]
    DSP_DRAM5_NORMAL_MODE = 0,
    #[doc = "1: DSP DRAM5 retention mode"]
    DSP_DRAM5_RETENTION_MODE = 1,
}
impl From<DSP_DRAM5_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_DRAM5_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_DRAM5_RETENTION`"]
pub type DSP_DRAM5_RETENTION_R = crate::R<bool, DSP_DRAM5_RETENTION_A>;
impl DSP_DRAM5_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_DRAM5_RETENTION_A {
        match self.bits {
            false => DSP_DRAM5_RETENTION_A::DSP_DRAM5_NORMAL_MODE,
            true => DSP_DRAM5_RETENTION_A::DSP_DRAM5_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM5_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_dsp_dram5_normal_mode(&self) -> bool {
        *self == DSP_DRAM5_RETENTION_A::DSP_DRAM5_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM5_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_dsp_dram5_retention_mode(&self) -> bool {
        *self == DSP_DRAM5_RETENTION_A::DSP_DRAM5_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `DSP_DRAM5_RETENTION`"]
pub struct DSP_DRAM5_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM5_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM5_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP DRAM5 normal mode"]
    #[inline(always)]
    pub fn dsp_dram5_normal_mode(self) -> &'a mut W {
        self.variant(DSP_DRAM5_RETENTION_A::DSP_DRAM5_NORMAL_MODE)
    }
    #[doc = "DSP DRAM5 retention mode"]
    #[inline(always)]
    pub fn dsp_dram5_retention_mode(self) -> &'a mut W {
        self.variant(DSP_DRAM5_RETENTION_A::DSP_DRAM5_RETENTION_MODE)
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
#[doc = "DSP PRAM0 retention configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_DRAM4_RETENTION_A {
    #[doc = "0: DSP DRAM4 normal mode"]
    DSP_DRAM4_NORMAL_MODE = 0,
    #[doc = "1: DSP DRAM4 retention mode"]
    DSP_DRAM4_RETENTION_MODE = 1,
}
impl From<DSP_DRAM4_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_DRAM4_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_DRAM4_RETENTION`"]
pub type DSP_DRAM4_RETENTION_R = crate::R<bool, DSP_DRAM4_RETENTION_A>;
impl DSP_DRAM4_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_DRAM4_RETENTION_A {
        match self.bits {
            false => DSP_DRAM4_RETENTION_A::DSP_DRAM4_NORMAL_MODE,
            true => DSP_DRAM4_RETENTION_A::DSP_DRAM4_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM4_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_dsp_dram4_normal_mode(&self) -> bool {
        *self == DSP_DRAM4_RETENTION_A::DSP_DRAM4_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM4_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_dsp_dram4_retention_mode(&self) -> bool {
        *self == DSP_DRAM4_RETENTION_A::DSP_DRAM4_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `DSP_DRAM4_RETENTION`"]
pub struct DSP_DRAM4_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM4_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM4_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP DRAM4 normal mode"]
    #[inline(always)]
    pub fn dsp_dram4_normal_mode(self) -> &'a mut W {
        self.variant(DSP_DRAM4_RETENTION_A::DSP_DRAM4_NORMAL_MODE)
    }
    #[doc = "DSP DRAM4 retention mode"]
    #[inline(always)]
    pub fn dsp_dram4_retention_mode(self) -> &'a mut W {
        self.variant(DSP_DRAM4_RETENTION_A::DSP_DRAM4_RETENTION_MODE)
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
#[doc = "DSP PRAM0 retention configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_DRAM3_RETENTION_A {
    #[doc = "0: DSP DRAM3 normal mode"]
    DSP_DRAM3_NORMAL_MODE = 0,
    #[doc = "1: DSP DRAM3 retention mode"]
    DSP_DRAM3_RETENTION_MODE = 1,
}
impl From<DSP_DRAM3_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_DRAM3_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_DRAM3_RETENTION`"]
pub type DSP_DRAM3_RETENTION_R = crate::R<bool, DSP_DRAM3_RETENTION_A>;
impl DSP_DRAM3_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_DRAM3_RETENTION_A {
        match self.bits {
            false => DSP_DRAM3_RETENTION_A::DSP_DRAM3_NORMAL_MODE,
            true => DSP_DRAM3_RETENTION_A::DSP_DRAM3_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM3_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_dsp_dram3_normal_mode(&self) -> bool {
        *self == DSP_DRAM3_RETENTION_A::DSP_DRAM3_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM3_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_dsp_dram3_retention_mode(&self) -> bool {
        *self == DSP_DRAM3_RETENTION_A::DSP_DRAM3_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `DSP_DRAM3_RETENTION`"]
pub struct DSP_DRAM3_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM3_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM3_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP DRAM3 normal mode"]
    #[inline(always)]
    pub fn dsp_dram3_normal_mode(self) -> &'a mut W {
        self.variant(DSP_DRAM3_RETENTION_A::DSP_DRAM3_NORMAL_MODE)
    }
    #[doc = "DSP DRAM3 retention mode"]
    #[inline(always)]
    pub fn dsp_dram3_retention_mode(self) -> &'a mut W {
        self.variant(DSP_DRAM3_RETENTION_A::DSP_DRAM3_RETENTION_MODE)
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
#[doc = "DSP PRAM0 retention configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_DRAM2_RETENTION_A {
    #[doc = "0: DSP DRAM2 normal mode"]
    DSP_DRAM2_NORMAL_MODE = 0,
    #[doc = "1: DSP DRAM2 retention mode"]
    DSP_DRAM2_RETENTION_MODE = 1,
}
impl From<DSP_DRAM2_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_DRAM2_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_DRAM2_RETENTION`"]
pub type DSP_DRAM2_RETENTION_R = crate::R<bool, DSP_DRAM2_RETENTION_A>;
impl DSP_DRAM2_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_DRAM2_RETENTION_A {
        match self.bits {
            false => DSP_DRAM2_RETENTION_A::DSP_DRAM2_NORMAL_MODE,
            true => DSP_DRAM2_RETENTION_A::DSP_DRAM2_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM2_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_dsp_dram2_normal_mode(&self) -> bool {
        *self == DSP_DRAM2_RETENTION_A::DSP_DRAM2_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM2_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_dsp_dram2_retention_mode(&self) -> bool {
        *self == DSP_DRAM2_RETENTION_A::DSP_DRAM2_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `DSP_DRAM2_RETENTION`"]
pub struct DSP_DRAM2_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM2_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM2_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP DRAM2 normal mode"]
    #[inline(always)]
    pub fn dsp_dram2_normal_mode(self) -> &'a mut W {
        self.variant(DSP_DRAM2_RETENTION_A::DSP_DRAM2_NORMAL_MODE)
    }
    #[doc = "DSP DRAM2 retention mode"]
    #[inline(always)]
    pub fn dsp_dram2_retention_mode(self) -> &'a mut W {
        self.variant(DSP_DRAM2_RETENTION_A::DSP_DRAM2_RETENTION_MODE)
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
#[doc = "DSP PRAM0 retention configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_DRAM1_RETENTION_A {
    #[doc = "0: DSP DRAM1 normal mode"]
    DSP_DRAM1_NORMAL_MODE = 0,
    #[doc = "1: DSP DRAM1 retention mode"]
    DSP_DRAM1_RETENTION_MODE = 1,
}
impl From<DSP_DRAM1_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_DRAM1_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_DRAM1_RETENTION`"]
pub type DSP_DRAM1_RETENTION_R = crate::R<bool, DSP_DRAM1_RETENTION_A>;
impl DSP_DRAM1_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_DRAM1_RETENTION_A {
        match self.bits {
            false => DSP_DRAM1_RETENTION_A::DSP_DRAM1_NORMAL_MODE,
            true => DSP_DRAM1_RETENTION_A::DSP_DRAM1_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM1_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_dsp_dram1_normal_mode(&self) -> bool {
        *self == DSP_DRAM1_RETENTION_A::DSP_DRAM1_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM1_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_dsp_dram1_retention_mode(&self) -> bool {
        *self == DSP_DRAM1_RETENTION_A::DSP_DRAM1_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `DSP_DRAM1_RETENTION`"]
pub struct DSP_DRAM1_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM1_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM1_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP DRAM1 normal mode"]
    #[inline(always)]
    pub fn dsp_dram1_normal_mode(self) -> &'a mut W {
        self.variant(DSP_DRAM1_RETENTION_A::DSP_DRAM1_NORMAL_MODE)
    }
    #[doc = "DSP DRAM1 retention mode"]
    #[inline(always)]
    pub fn dsp_dram1_retention_mode(self) -> &'a mut W {
        self.variant(DSP_DRAM1_RETENTION_A::DSP_DRAM1_RETENTION_MODE)
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
#[doc = "DSP PRAM0 retention configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_DRAM0_RETENTION_A {
    #[doc = "0: DSP DRAM0 normal mode"]
    DSP_DRAM0_NORMAL_MODE = 0,
    #[doc = "1: DSP DRAM0 retention mode"]
    DSP_DRAM0_RETENTION_MODE = 1,
}
impl From<DSP_DRAM0_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_DRAM0_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_DRAM0_RETENTION`"]
pub type DSP_DRAM0_RETENTION_R = crate::R<bool, DSP_DRAM0_RETENTION_A>;
impl DSP_DRAM0_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_DRAM0_RETENTION_A {
        match self.bits {
            false => DSP_DRAM0_RETENTION_A::DSP_DRAM0_NORMAL_MODE,
            true => DSP_DRAM0_RETENTION_A::DSP_DRAM0_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM0_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_dsp_dram0_normal_mode(&self) -> bool {
        *self == DSP_DRAM0_RETENTION_A::DSP_DRAM0_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM0_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_dsp_dram0_retention_mode(&self) -> bool {
        *self == DSP_DRAM0_RETENTION_A::DSP_DRAM0_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `DSP_DRAM0_RETENTION`"]
pub struct DSP_DRAM0_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM0_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM0_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP DRAM0 normal mode"]
    #[inline(always)]
    pub fn dsp_dram0_normal_mode(self) -> &'a mut W {
        self.variant(DSP_DRAM0_RETENTION_A::DSP_DRAM0_NORMAL_MODE)
    }
    #[doc = "DSP DRAM0 retention mode"]
    #[inline(always)]
    pub fn dsp_dram0_retention_mode(self) -> &'a mut W {
        self.variant(DSP_DRAM0_RETENTION_A::DSP_DRAM0_RETENTION_MODE)
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
#[doc = "DSP PRAM0 retention configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_PRAM3_RETENTION_A {
    #[doc = "0: DSP PRAM3 normal mode"]
    DSP_PRAM3_NORMAL_MODE = 0,
    #[doc = "1: DSP PRAM3 retention mode"]
    DSP_PRAM3_RETENTION_MODE = 1,
}
impl From<DSP_PRAM3_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_PRAM3_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_PRAM3_RETENTION`"]
pub type DSP_PRAM3_RETENTION_R = crate::R<bool, DSP_PRAM3_RETENTION_A>;
impl DSP_PRAM3_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_PRAM3_RETENTION_A {
        match self.bits {
            false => DSP_PRAM3_RETENTION_A::DSP_PRAM3_NORMAL_MODE,
            true => DSP_PRAM3_RETENTION_A::DSP_PRAM3_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM3_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_dsp_pram3_normal_mode(&self) -> bool {
        *self == DSP_PRAM3_RETENTION_A::DSP_PRAM3_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM3_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_dsp_pram3_retention_mode(&self) -> bool {
        *self == DSP_PRAM3_RETENTION_A::DSP_PRAM3_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `DSP_PRAM3_RETENTION`"]
pub struct DSP_PRAM3_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM3_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM3_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP PRAM3 normal mode"]
    #[inline(always)]
    pub fn dsp_pram3_normal_mode(self) -> &'a mut W {
        self.variant(DSP_PRAM3_RETENTION_A::DSP_PRAM3_NORMAL_MODE)
    }
    #[doc = "DSP PRAM3 retention mode"]
    #[inline(always)]
    pub fn dsp_pram3_retention_mode(self) -> &'a mut W {
        self.variant(DSP_PRAM3_RETENTION_A::DSP_PRAM3_RETENTION_MODE)
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
#[doc = "DSP PRAM0 retention configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_PRAM2_RETENTION_A {
    #[doc = "0: DSP PRAM2 normal mode"]
    DSP_PRAM2_NORMAL_MODE = 0,
    #[doc = "1: DSP PRAM2 retention mode"]
    DSP_PRAM2_RETENTION_MODE = 1,
}
impl From<DSP_PRAM2_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_PRAM2_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_PRAM2_RETENTION`"]
pub type DSP_PRAM2_RETENTION_R = crate::R<bool, DSP_PRAM2_RETENTION_A>;
impl DSP_PRAM2_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_PRAM2_RETENTION_A {
        match self.bits {
            false => DSP_PRAM2_RETENTION_A::DSP_PRAM2_NORMAL_MODE,
            true => DSP_PRAM2_RETENTION_A::DSP_PRAM2_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM2_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_dsp_pram2_normal_mode(&self) -> bool {
        *self == DSP_PRAM2_RETENTION_A::DSP_PRAM2_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM2_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_dsp_pram2_retention_mode(&self) -> bool {
        *self == DSP_PRAM2_RETENTION_A::DSP_PRAM2_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `DSP_PRAM2_RETENTION`"]
pub struct DSP_PRAM2_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM2_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM2_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP PRAM2 normal mode"]
    #[inline(always)]
    pub fn dsp_pram2_normal_mode(self) -> &'a mut W {
        self.variant(DSP_PRAM2_RETENTION_A::DSP_PRAM2_NORMAL_MODE)
    }
    #[doc = "DSP PRAM2 retention mode"]
    #[inline(always)]
    pub fn dsp_pram2_retention_mode(self) -> &'a mut W {
        self.variant(DSP_PRAM2_RETENTION_A::DSP_PRAM2_RETENTION_MODE)
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
#[doc = "DSP PRAM0 retention configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_PRAM1_RETENTION_A {
    #[doc = "0: DSP PRAM1 normal mode"]
    DSP_PRAM1_NORMAL_MODE = 0,
    #[doc = "1: DSP PRAM1 retention mode"]
    DSP_PRAM1_RETENTION_MODE = 1,
}
impl From<DSP_PRAM1_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_PRAM1_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_PRAM1_RETENTION`"]
pub type DSP_PRAM1_RETENTION_R = crate::R<bool, DSP_PRAM1_RETENTION_A>;
impl DSP_PRAM1_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_PRAM1_RETENTION_A {
        match self.bits {
            false => DSP_PRAM1_RETENTION_A::DSP_PRAM1_NORMAL_MODE,
            true => DSP_PRAM1_RETENTION_A::DSP_PRAM1_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM1_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_dsp_pram1_normal_mode(&self) -> bool {
        *self == DSP_PRAM1_RETENTION_A::DSP_PRAM1_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM1_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_dsp_pram1_retention_mode(&self) -> bool {
        *self == DSP_PRAM1_RETENTION_A::DSP_PRAM1_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `DSP_PRAM1_RETENTION`"]
pub struct DSP_PRAM1_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM1_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM1_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP PRAM1 normal mode"]
    #[inline(always)]
    pub fn dsp_pram1_normal_mode(self) -> &'a mut W {
        self.variant(DSP_PRAM1_RETENTION_A::DSP_PRAM1_NORMAL_MODE)
    }
    #[doc = "DSP PRAM1 retention mode"]
    #[inline(always)]
    pub fn dsp_pram1_retention_mode(self) -> &'a mut W {
        self.variant(DSP_PRAM1_RETENTION_A::DSP_PRAM1_RETENTION_MODE)
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
#[doc = "DSP PRAM0 retention configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_PRAM0_RETENTION_A {
    #[doc = "0: DSP PRAM0 normal mode"]
    DSP_PRAM0_NORMAL_MODE = 0,
    #[doc = "1: DSP PRAM0 retention mode"]
    DSP_PRAM0_RETENTION_MODE = 1,
}
impl From<DSP_PRAM0_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_PRAM0_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_PRAM0_RETENTION`"]
pub type DSP_PRAM0_RETENTION_R = crate::R<bool, DSP_PRAM0_RETENTION_A>;
impl DSP_PRAM0_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_PRAM0_RETENTION_A {
        match self.bits {
            false => DSP_PRAM0_RETENTION_A::DSP_PRAM0_NORMAL_MODE,
            true => DSP_PRAM0_RETENTION_A::DSP_PRAM0_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM0_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_dsp_pram0_normal_mode(&self) -> bool {
        *self == DSP_PRAM0_RETENTION_A::DSP_PRAM0_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM0_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_dsp_pram0_retention_mode(&self) -> bool {
        *self == DSP_PRAM0_RETENTION_A::DSP_PRAM0_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `DSP_PRAM0_RETENTION`"]
pub struct DSP_PRAM0_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM0_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM0_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP PRAM0 normal mode"]
    #[inline(always)]
    pub fn dsp_pram0_normal_mode(self) -> &'a mut W {
        self.variant(DSP_PRAM0_RETENTION_A::DSP_PRAM0_NORMAL_MODE)
    }
    #[doc = "DSP PRAM0 retention mode"]
    #[inline(always)]
    pub fn dsp_pram0_retention_mode(self) -> &'a mut W {
        self.variant(DSP_PRAM0_RETENTION_A::DSP_PRAM0_RETENTION_MODE)
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
#[doc = "Baseband DRAM1 retention configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_DRAM1_RETENTION_A {
    #[doc = "0: Baseband DRAM1 normal mode"]
    BB_DRAM1_NORMAL_MODE = 0,
    #[doc = "1: Baseband DRAM1 retention mode"]
    BB_DRAM1_RETENTION_MODE = 1,
}
impl From<BB_DRAM1_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: BB_DRAM1_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BB_DRAM1_RETENTION`"]
pub type BB_DRAM1_RETENTION_R = crate::R<bool, BB_DRAM1_RETENTION_A>;
impl BB_DRAM1_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_DRAM1_RETENTION_A {
        match self.bits {
            false => BB_DRAM1_RETENTION_A::BB_DRAM1_NORMAL_MODE,
            true => BB_DRAM1_RETENTION_A::BB_DRAM1_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `BB_DRAM1_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_bb_dram1_normal_mode(&self) -> bool {
        *self == BB_DRAM1_RETENTION_A::BB_DRAM1_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `BB_DRAM1_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_bb_dram1_retention_mode(&self) -> bool {
        *self == BB_DRAM1_RETENTION_A::BB_DRAM1_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `BB_DRAM1_RETENTION`"]
pub struct BB_DRAM1_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_DRAM1_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_DRAM1_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Baseband DRAM1 normal mode"]
    #[inline(always)]
    pub fn bb_dram1_normal_mode(self) -> &'a mut W {
        self.variant(BB_DRAM1_RETENTION_A::BB_DRAM1_NORMAL_MODE)
    }
    #[doc = "Baseband DRAM1 retention mode"]
    #[inline(always)]
    pub fn bb_dram1_retention_mode(self) -> &'a mut W {
        self.variant(BB_DRAM1_RETENTION_A::BB_DRAM1_RETENTION_MODE)
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
#[doc = "Baseband DRAM0 retention configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_DRAM0_RETENTION_A {
    #[doc = "0: Baseband DRAM0 normal mode"]
    BB_DRAM0_NORMAL_MODE = 0,
    #[doc = "1: Baseband DRAM0 retention mode"]
    BB_DRAM0_RETENTION_MODE = 1,
}
impl From<BB_DRAM0_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: BB_DRAM0_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BB_DRAM0_RETENTION`"]
pub type BB_DRAM0_RETENTION_R = crate::R<bool, BB_DRAM0_RETENTION_A>;
impl BB_DRAM0_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_DRAM0_RETENTION_A {
        match self.bits {
            false => BB_DRAM0_RETENTION_A::BB_DRAM0_NORMAL_MODE,
            true => BB_DRAM0_RETENTION_A::BB_DRAM0_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `BB_DRAM0_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_bb_dram0_normal_mode(&self) -> bool {
        *self == BB_DRAM0_RETENTION_A::BB_DRAM0_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `BB_DRAM0_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_bb_dram0_retention_mode(&self) -> bool {
        *self == BB_DRAM0_RETENTION_A::BB_DRAM0_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `BB_DRAM0_RETENTION`"]
pub struct BB_DRAM0_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_DRAM0_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_DRAM0_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Baseband DRAM0 normal mode"]
    #[inline(always)]
    pub fn bb_dram0_normal_mode(self) -> &'a mut W {
        self.variant(BB_DRAM0_RETENTION_A::BB_DRAM0_NORMAL_MODE)
    }
    #[doc = "Baseband DRAM0 retention mode"]
    #[inline(always)]
    pub fn bb_dram0_retention_mode(self) -> &'a mut W {
        self.variant(BB_DRAM0_RETENTION_A::BB_DRAM0_RETENTION_MODE)
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
#[doc = "DRAM2 retention configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRAM2_RETENTION_A {
    #[doc = "0: DRAM2 normal mode"]
    DRAM2_NORMAL_MODE = 0,
    #[doc = "1: DRAM2 retention mode"]
    DRAM2_RETENTION_MODE = 1,
}
impl From<DRAM2_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: DRAM2_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRAM2_RETENTION`"]
pub type DRAM2_RETENTION_R = crate::R<bool, DRAM2_RETENTION_A>;
impl DRAM2_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRAM2_RETENTION_A {
        match self.bits {
            false => DRAM2_RETENTION_A::DRAM2_NORMAL_MODE,
            true => DRAM2_RETENTION_A::DRAM2_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `DRAM2_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_dram2_normal_mode(&self) -> bool {
        *self == DRAM2_RETENTION_A::DRAM2_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `DRAM2_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_dram2_retention_mode(&self) -> bool {
        *self == DRAM2_RETENTION_A::DRAM2_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `DRAM2_RETENTION`"]
pub struct DRAM2_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DRAM2_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRAM2_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DRAM2 normal mode"]
    #[inline(always)]
    pub fn dram2_normal_mode(self) -> &'a mut W {
        self.variant(DRAM2_RETENTION_A::DRAM2_NORMAL_MODE)
    }
    #[doc = "DRAM2 retention mode"]
    #[inline(always)]
    pub fn dram2_retention_mode(self) -> &'a mut W {
        self.variant(DRAM2_RETENTION_A::DRAM2_RETENTION_MODE)
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
#[doc = "DRAM1 retention configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRAM1_RETENTION_A {
    #[doc = "0: DRAM1 normal mode"]
    DRAM1_NORMAL_MODE = 0,
    #[doc = "1: DRAM1 retention mode"]
    DRAM1_RETENTION_MODE = 1,
}
impl From<DRAM1_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: DRAM1_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRAM1_RETENTION`"]
pub type DRAM1_RETENTION_R = crate::R<bool, DRAM1_RETENTION_A>;
impl DRAM1_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRAM1_RETENTION_A {
        match self.bits {
            false => DRAM1_RETENTION_A::DRAM1_NORMAL_MODE,
            true => DRAM1_RETENTION_A::DRAM1_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `DRAM1_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_dram1_normal_mode(&self) -> bool {
        *self == DRAM1_RETENTION_A::DRAM1_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `DRAM1_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_dram1_retention_mode(&self) -> bool {
        *self == DRAM1_RETENTION_A::DRAM1_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `DRAM1_RETENTION`"]
pub struct DRAM1_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DRAM1_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRAM1_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DRAM1 normal mode"]
    #[inline(always)]
    pub fn dram1_normal_mode(self) -> &'a mut W {
        self.variant(DRAM1_RETENTION_A::DRAM1_NORMAL_MODE)
    }
    #[doc = "DRAM1 retention mode"]
    #[inline(always)]
    pub fn dram1_retention_mode(self) -> &'a mut W {
        self.variant(DRAM1_RETENTION_A::DRAM1_RETENTION_MODE)
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
#[doc = "DRAM0 retention configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRAM0_RETENTION_A {
    #[doc = "0: DRAM0 normal mode"]
    DRAM0_NORMAL_MODE = 0,
    #[doc = "1: DRAM0 retention mode"]
    DRAM0_RETENTION_MODE = 1,
}
impl From<DRAM0_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: DRAM0_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRAM0_RETENTION`"]
pub type DRAM0_RETENTION_R = crate::R<bool, DRAM0_RETENTION_A>;
impl DRAM0_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRAM0_RETENTION_A {
        match self.bits {
            false => DRAM0_RETENTION_A::DRAM0_NORMAL_MODE,
            true => DRAM0_RETENTION_A::DRAM0_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `DRAM0_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_dram0_normal_mode(&self) -> bool {
        *self == DRAM0_RETENTION_A::DRAM0_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `DRAM0_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_dram0_retention_mode(&self) -> bool {
        *self == DRAM0_RETENTION_A::DRAM0_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `DRAM0_RETENTION`"]
pub struct DRAM0_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DRAM0_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRAM0_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DRAM0 normal mode"]
    #[inline(always)]
    pub fn dram0_normal_mode(self) -> &'a mut W {
        self.variant(DRAM0_RETENTION_A::DRAM0_NORMAL_MODE)
    }
    #[doc = "DRAM0 retention mode"]
    #[inline(always)]
    pub fn dram0_retention_mode(self) -> &'a mut W {
        self.variant(DRAM0_RETENTION_A::DRAM0_RETENTION_MODE)
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
#[doc = "PRAM3 retention configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRAM3_RETENTION_A {
    #[doc = "0: PRAM3 normal mode"]
    PRAM3_NORMAL_MODE = 0,
    #[doc = "1: PRAM3 retention mode"]
    PRAM3_RETENTION_MODE = 1,
}
impl From<PRAM3_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: PRAM3_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRAM3_RETENTION`"]
pub type PRAM3_RETENTION_R = crate::R<bool, PRAM3_RETENTION_A>;
impl PRAM3_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRAM3_RETENTION_A {
        match self.bits {
            false => PRAM3_RETENTION_A::PRAM3_NORMAL_MODE,
            true => PRAM3_RETENTION_A::PRAM3_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `PRAM3_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_pram3_normal_mode(&self) -> bool {
        *self == PRAM3_RETENTION_A::PRAM3_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `PRAM3_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_pram3_retention_mode(&self) -> bool {
        *self == PRAM3_RETENTION_A::PRAM3_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `PRAM3_RETENTION`"]
pub struct PRAM3_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> PRAM3_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRAM3_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PRAM3 normal mode"]
    #[inline(always)]
    pub fn pram3_normal_mode(self) -> &'a mut W {
        self.variant(PRAM3_RETENTION_A::PRAM3_NORMAL_MODE)
    }
    #[doc = "PRAM3 retention mode"]
    #[inline(always)]
    pub fn pram3_retention_mode(self) -> &'a mut W {
        self.variant(PRAM3_RETENTION_A::PRAM3_RETENTION_MODE)
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
#[doc = "PRAM2 retention configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRAM2_RETENTION_A {
    #[doc = "0: PRAM2 normal mode"]
    PRAM2_NORMAL_MODE = 0,
    #[doc = "1: PRAM2 retention mode"]
    PRAM2_RETENTION_MODE = 1,
}
impl From<PRAM2_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: PRAM2_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRAM2_RETENTION`"]
pub type PRAM2_RETENTION_R = crate::R<bool, PRAM2_RETENTION_A>;
impl PRAM2_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRAM2_RETENTION_A {
        match self.bits {
            false => PRAM2_RETENTION_A::PRAM2_NORMAL_MODE,
            true => PRAM2_RETENTION_A::PRAM2_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `PRAM2_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_pram2_normal_mode(&self) -> bool {
        *self == PRAM2_RETENTION_A::PRAM2_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `PRAM2_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_pram2_retention_mode(&self) -> bool {
        *self == PRAM2_RETENTION_A::PRAM2_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `PRAM2_RETENTION`"]
pub struct PRAM2_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> PRAM2_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRAM2_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PRAM2 normal mode"]
    #[inline(always)]
    pub fn pram2_normal_mode(self) -> &'a mut W {
        self.variant(PRAM2_RETENTION_A::PRAM2_NORMAL_MODE)
    }
    #[doc = "PRAM2 retention mode"]
    #[inline(always)]
    pub fn pram2_retention_mode(self) -> &'a mut W {
        self.variant(PRAM2_RETENTION_A::PRAM2_RETENTION_MODE)
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
#[doc = "PRAM1 retention configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRAM1_RETENTION_A {
    #[doc = "0: PRAM1 normal mode"]
    PRAM1_NORMAL_MODE = 0,
    #[doc = "1: PRAM1 retention mode"]
    PRAM1_RETENTION_MODE = 1,
}
impl From<PRAM1_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: PRAM1_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRAM1_RETENTION`"]
pub type PRAM1_RETENTION_R = crate::R<bool, PRAM1_RETENTION_A>;
impl PRAM1_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRAM1_RETENTION_A {
        match self.bits {
            false => PRAM1_RETENTION_A::PRAM1_NORMAL_MODE,
            true => PRAM1_RETENTION_A::PRAM1_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `PRAM1_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_pram1_normal_mode(&self) -> bool {
        *self == PRAM1_RETENTION_A::PRAM1_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `PRAM1_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_pram1_retention_mode(&self) -> bool {
        *self == PRAM1_RETENTION_A::PRAM1_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `PRAM1_RETENTION`"]
pub struct PRAM1_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> PRAM1_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRAM1_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PRAM1 normal mode"]
    #[inline(always)]
    pub fn pram1_normal_mode(self) -> &'a mut W {
        self.variant(PRAM1_RETENTION_A::PRAM1_NORMAL_MODE)
    }
    #[doc = "PRAM1 retention mode"]
    #[inline(always)]
    pub fn pram1_retention_mode(self) -> &'a mut W {
        self.variant(PRAM1_RETENTION_A::PRAM1_RETENTION_MODE)
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
#[doc = "PRAM0 retention configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRAM0_RETENTION_A {
    #[doc = "0: PRAM0 normal mode"]
    PRAM0_NORMAL_MODE = 0,
    #[doc = "1: PRAM0 retention mode"]
    PRAM0_RETENTION_MODE = 1,
}
impl From<PRAM0_RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: PRAM0_RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRAM0_RETENTION`"]
pub type PRAM0_RETENTION_R = crate::R<bool, PRAM0_RETENTION_A>;
impl PRAM0_RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRAM0_RETENTION_A {
        match self.bits {
            false => PRAM0_RETENTION_A::PRAM0_NORMAL_MODE,
            true => PRAM0_RETENTION_A::PRAM0_RETENTION_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `PRAM0_NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_pram0_normal_mode(&self) -> bool {
        *self == PRAM0_RETENTION_A::PRAM0_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `PRAM0_RETENTION_MODE`"]
    #[inline(always)]
    pub fn is_pram0_retention_mode(&self) -> bool {
        *self == PRAM0_RETENTION_A::PRAM0_RETENTION_MODE
    }
}
#[doc = "Write proxy for field `PRAM0_RETENTION`"]
pub struct PRAM0_RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> PRAM0_RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRAM0_RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PRAM0 normal mode"]
    #[inline(always)]
    pub fn pram0_normal_mode(self) -> &'a mut W {
        self.variant(PRAM0_RETENTION_A::PRAM0_NORMAL_MODE)
    }
    #[doc = "PRAM0 retention mode"]
    #[inline(always)]
    pub fn pram0_retention_mode(self) -> &'a mut W {
        self.variant(PRAM0_RETENTION_A::PRAM0_RETENTION_MODE)
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
impl R {
    #[doc = "Bit 21 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_dram5_retention(&self) -> DSP_DRAM5_RETENTION_R {
        DSP_DRAM5_RETENTION_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_dram4_retention(&self) -> DSP_DRAM4_RETENTION_R {
        DSP_DRAM4_RETENTION_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_dram3_retention(&self) -> DSP_DRAM3_RETENTION_R {
        DSP_DRAM3_RETENTION_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_dram2_retention(&self) -> DSP_DRAM2_RETENTION_R {
        DSP_DRAM2_RETENTION_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_dram1_retention(&self) -> DSP_DRAM1_RETENTION_R {
        DSP_DRAM1_RETENTION_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_dram0_retention(&self) -> DSP_DRAM0_RETENTION_R {
        DSP_DRAM0_RETENTION_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_pram3_retention(&self) -> DSP_PRAM3_RETENTION_R {
        DSP_PRAM3_RETENTION_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_pram2_retention(&self) -> DSP_PRAM2_RETENTION_R {
        DSP_PRAM2_RETENTION_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_pram1_retention(&self) -> DSP_PRAM1_RETENTION_R {
        DSP_PRAM1_RETENTION_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_pram0_retention(&self) -> DSP_PRAM0_RETENTION_R {
        DSP_PRAM0_RETENTION_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Baseband DRAM1 retention configuration"]
    #[inline(always)]
    pub fn bb_dram1_retention(&self) -> BB_DRAM1_RETENTION_R {
        BB_DRAM1_RETENTION_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Baseband DRAM0 retention configuration"]
    #[inline(always)]
    pub fn bb_dram0_retention(&self) -> BB_DRAM0_RETENTION_R {
        BB_DRAM0_RETENTION_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DRAM2 retention configuration"]
    #[inline(always)]
    pub fn dram2_retention(&self) -> DRAM2_RETENTION_R {
        DRAM2_RETENTION_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DRAM1 retention configuration"]
    #[inline(always)]
    pub fn dram1_retention(&self) -> DRAM1_RETENTION_R {
        DRAM1_RETENTION_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DRAM0 retention configuration"]
    #[inline(always)]
    pub fn dram0_retention(&self) -> DRAM0_RETENTION_R {
        DRAM0_RETENTION_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PRAM3 retention configuration"]
    #[inline(always)]
    pub fn pram3_retention(&self) -> PRAM3_RETENTION_R {
        PRAM3_RETENTION_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PRAM2 retention configuration"]
    #[inline(always)]
    pub fn pram2_retention(&self) -> PRAM2_RETENTION_R {
        PRAM2_RETENTION_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PRAM1 retention configuration"]
    #[inline(always)]
    pub fn pram1_retention(&self) -> PRAM1_RETENTION_R {
        PRAM1_RETENTION_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PRAM0 retention configuration"]
    #[inline(always)]
    pub fn pram0_retention(&self) -> PRAM0_RETENTION_R {
        PRAM0_RETENTION_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_dram5_retention(&mut self) -> DSP_DRAM5_RETENTION_W {
        DSP_DRAM5_RETENTION_W { w: self }
    }
    #[doc = "Bit 20 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_dram4_retention(&mut self) -> DSP_DRAM4_RETENTION_W {
        DSP_DRAM4_RETENTION_W { w: self }
    }
    #[doc = "Bit 19 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_dram3_retention(&mut self) -> DSP_DRAM3_RETENTION_W {
        DSP_DRAM3_RETENTION_W { w: self }
    }
    #[doc = "Bit 18 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_dram2_retention(&mut self) -> DSP_DRAM2_RETENTION_W {
        DSP_DRAM2_RETENTION_W { w: self }
    }
    #[doc = "Bit 17 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_dram1_retention(&mut self) -> DSP_DRAM1_RETENTION_W {
        DSP_DRAM1_RETENTION_W { w: self }
    }
    #[doc = "Bit 16 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_dram0_retention(&mut self) -> DSP_DRAM0_RETENTION_W {
        DSP_DRAM0_RETENTION_W { w: self }
    }
    #[doc = "Bit 15 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_pram3_retention(&mut self) -> DSP_PRAM3_RETENTION_W {
        DSP_PRAM3_RETENTION_W { w: self }
    }
    #[doc = "Bit 14 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_pram2_retention(&mut self) -> DSP_PRAM2_RETENTION_W {
        DSP_PRAM2_RETENTION_W { w: self }
    }
    #[doc = "Bit 13 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_pram1_retention(&mut self) -> DSP_PRAM1_RETENTION_W {
        DSP_PRAM1_RETENTION_W { w: self }
    }
    #[doc = "Bit 12 - DSP PRAM0 retention configuration"]
    #[inline(always)]
    pub fn dsp_pram0_retention(&mut self) -> DSP_PRAM0_RETENTION_W {
        DSP_PRAM0_RETENTION_W { w: self }
    }
    #[doc = "Bit 11 - Baseband DRAM1 retention configuration"]
    #[inline(always)]
    pub fn bb_dram1_retention(&mut self) -> BB_DRAM1_RETENTION_W {
        BB_DRAM1_RETENTION_W { w: self }
    }
    #[doc = "Bit 10 - Baseband DRAM0 retention configuration"]
    #[inline(always)]
    pub fn bb_dram0_retention(&mut self) -> BB_DRAM0_RETENTION_W {
        BB_DRAM0_RETENTION_W { w: self }
    }
    #[doc = "Bit 8 - DRAM2 retention configuration"]
    #[inline(always)]
    pub fn dram2_retention(&mut self) -> DRAM2_RETENTION_W {
        DRAM2_RETENTION_W { w: self }
    }
    #[doc = "Bit 7 - DRAM1 retention configuration"]
    #[inline(always)]
    pub fn dram1_retention(&mut self) -> DRAM1_RETENTION_W {
        DRAM1_RETENTION_W { w: self }
    }
    #[doc = "Bit 6 - DRAM0 retention configuration"]
    #[inline(always)]
    pub fn dram0_retention(&mut self) -> DRAM0_RETENTION_W {
        DRAM0_RETENTION_W { w: self }
    }
    #[doc = "Bit 5 - PRAM3 retention configuration"]
    #[inline(always)]
    pub fn pram3_retention(&mut self) -> PRAM3_RETENTION_W {
        PRAM3_RETENTION_W { w: self }
    }
    #[doc = "Bit 4 - PRAM2 retention configuration"]
    #[inline(always)]
    pub fn pram2_retention(&mut self) -> PRAM2_RETENTION_W {
        PRAM2_RETENTION_W { w: self }
    }
    #[doc = "Bit 3 - PRAM1 retention configuration"]
    #[inline(always)]
    pub fn pram1_retention(&mut self) -> PRAM1_RETENTION_W {
        PRAM1_RETENTION_W { w: self }
    }
    #[doc = "Bit 2 - PRAM0 retention configuration"]
    #[inline(always)]
    pub fn pram0_retention(&mut self) -> PRAM0_RETENTION_W {
        PRAM0_RETENTION_W { w: self }
    }
}
