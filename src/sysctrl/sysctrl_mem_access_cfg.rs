#[doc = "Reader of register SYSCTRL_MEM_ACCESS_CFG"]
pub type R = crate::R<u32, super::SYSCTRL_MEM_ACCESS_CFG>;
#[doc = "Writer for register SYSCTRL_MEM_ACCESS_CFG"]
pub type W = crate::W<u32, super::SYSCTRL_MEM_ACCESS_CFG>;
#[doc = "Register SYSCTRL_MEM_ACCESS_CFG `reset()`'s with value 0x41"]
impl crate::ResetValue for super::SYSCTRL_MEM_ACCESS_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x41
    }
}
#[doc = "Reader of field `WAKEUP_ADDR_PACKED`"]
pub type WAKEUP_ADDR_PACKED_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAKEUP_ADDR_PACKED`"]
pub struct WAKEUP_ADDR_PACKED_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_ADDR_PACKED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
#[doc = "DSP PRAM0 access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_DRAM5_ACCESS_A {
    #[doc = "0: DSP DRAM5 access disabled"]
    DSP_DRAM5_ACCESS_DISABLE = 0,
    #[doc = "1: DSP DRAM5 access enabled"]
    DSP_DRAM5_ACCESS_ENABLE = 1,
}
impl From<DSP_DRAM5_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_DRAM5_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_DRAM5_ACCESS`"]
pub type DSP_DRAM5_ACCESS_R = crate::R<bool, DSP_DRAM5_ACCESS_A>;
impl DSP_DRAM5_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_DRAM5_ACCESS_A {
        match self.bits {
            false => DSP_DRAM5_ACCESS_A::DSP_DRAM5_ACCESS_DISABLE,
            true => DSP_DRAM5_ACCESS_A::DSP_DRAM5_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM5_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram5_access_disable(&self) -> bool {
        *self == DSP_DRAM5_ACCESS_A::DSP_DRAM5_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM5_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram5_access_enable(&self) -> bool {
        *self == DSP_DRAM5_ACCESS_A::DSP_DRAM5_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_DRAM5_ACCESS`"]
pub struct DSP_DRAM5_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM5_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM5_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP DRAM5 access disabled"]
    #[inline(always)]
    pub fn dsp_dram5_access_disable(self) -> &'a mut W {
        self.variant(DSP_DRAM5_ACCESS_A::DSP_DRAM5_ACCESS_DISABLE)
    }
    #[doc = "DSP DRAM5 access enabled"]
    #[inline(always)]
    pub fn dsp_dram5_access_enable(self) -> &'a mut W {
        self.variant(DSP_DRAM5_ACCESS_A::DSP_DRAM5_ACCESS_ENABLE)
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
#[doc = "DSP PRAM0 access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_DRAM4_ACCESS_A {
    #[doc = "0: DSP DRAM4 access disabled"]
    DSP_DRAM4_ACCESS_DISABLE = 0,
    #[doc = "1: DSP DRAM4 access enabled"]
    DSP_DRAM4_ACCESS_ENABLE = 1,
}
impl From<DSP_DRAM4_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_DRAM4_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_DRAM4_ACCESS`"]
pub type DSP_DRAM4_ACCESS_R = crate::R<bool, DSP_DRAM4_ACCESS_A>;
impl DSP_DRAM4_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_DRAM4_ACCESS_A {
        match self.bits {
            false => DSP_DRAM4_ACCESS_A::DSP_DRAM4_ACCESS_DISABLE,
            true => DSP_DRAM4_ACCESS_A::DSP_DRAM4_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM4_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram4_access_disable(&self) -> bool {
        *self == DSP_DRAM4_ACCESS_A::DSP_DRAM4_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM4_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram4_access_enable(&self) -> bool {
        *self == DSP_DRAM4_ACCESS_A::DSP_DRAM4_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_DRAM4_ACCESS`"]
pub struct DSP_DRAM4_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM4_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM4_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP DRAM4 access disabled"]
    #[inline(always)]
    pub fn dsp_dram4_access_disable(self) -> &'a mut W {
        self.variant(DSP_DRAM4_ACCESS_A::DSP_DRAM4_ACCESS_DISABLE)
    }
    #[doc = "DSP DRAM4 access enabled"]
    #[inline(always)]
    pub fn dsp_dram4_access_enable(self) -> &'a mut W {
        self.variant(DSP_DRAM4_ACCESS_A::DSP_DRAM4_ACCESS_ENABLE)
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
#[doc = "DSP PRAM0 access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_DRAM3_ACCESS_A {
    #[doc = "0: DSP DRAM3 access disabled"]
    DSP_DRAM3_ACCESS_DISABLE = 0,
    #[doc = "1: DSP DRAM3 access enabled"]
    DSP_DRAM3_ACCESS_ENABLE = 1,
}
impl From<DSP_DRAM3_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_DRAM3_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_DRAM3_ACCESS`"]
pub type DSP_DRAM3_ACCESS_R = crate::R<bool, DSP_DRAM3_ACCESS_A>;
impl DSP_DRAM3_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_DRAM3_ACCESS_A {
        match self.bits {
            false => DSP_DRAM3_ACCESS_A::DSP_DRAM3_ACCESS_DISABLE,
            true => DSP_DRAM3_ACCESS_A::DSP_DRAM3_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM3_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram3_access_disable(&self) -> bool {
        *self == DSP_DRAM3_ACCESS_A::DSP_DRAM3_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM3_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram3_access_enable(&self) -> bool {
        *self == DSP_DRAM3_ACCESS_A::DSP_DRAM3_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_DRAM3_ACCESS`"]
pub struct DSP_DRAM3_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM3_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM3_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP DRAM3 access disabled"]
    #[inline(always)]
    pub fn dsp_dram3_access_disable(self) -> &'a mut W {
        self.variant(DSP_DRAM3_ACCESS_A::DSP_DRAM3_ACCESS_DISABLE)
    }
    #[doc = "DSP DRAM3 access enabled"]
    #[inline(always)]
    pub fn dsp_dram3_access_enable(self) -> &'a mut W {
        self.variant(DSP_DRAM3_ACCESS_A::DSP_DRAM3_ACCESS_ENABLE)
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
#[doc = "DSP PRAM0 access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_DRAM2_ACCESS_A {
    #[doc = "0: DSP DRAM2 access disabled"]
    DSP_DRAM2_ACCESS_DISABLE = 0,
    #[doc = "1: DSP DRAM2 access enabled"]
    DSP_DRAM2_ACCESS_ENABLE = 1,
}
impl From<DSP_DRAM2_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_DRAM2_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_DRAM2_ACCESS`"]
pub type DSP_DRAM2_ACCESS_R = crate::R<bool, DSP_DRAM2_ACCESS_A>;
impl DSP_DRAM2_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_DRAM2_ACCESS_A {
        match self.bits {
            false => DSP_DRAM2_ACCESS_A::DSP_DRAM2_ACCESS_DISABLE,
            true => DSP_DRAM2_ACCESS_A::DSP_DRAM2_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM2_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram2_access_disable(&self) -> bool {
        *self == DSP_DRAM2_ACCESS_A::DSP_DRAM2_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM2_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram2_access_enable(&self) -> bool {
        *self == DSP_DRAM2_ACCESS_A::DSP_DRAM2_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_DRAM2_ACCESS`"]
pub struct DSP_DRAM2_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM2_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM2_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP DRAM2 access disabled"]
    #[inline(always)]
    pub fn dsp_dram2_access_disable(self) -> &'a mut W {
        self.variant(DSP_DRAM2_ACCESS_A::DSP_DRAM2_ACCESS_DISABLE)
    }
    #[doc = "DSP DRAM2 access enabled"]
    #[inline(always)]
    pub fn dsp_dram2_access_enable(self) -> &'a mut W {
        self.variant(DSP_DRAM2_ACCESS_A::DSP_DRAM2_ACCESS_ENABLE)
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
#[doc = "DSP PRAM0 access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_DRAM1_ACCESS_A {
    #[doc = "0: DSP DRAM1 access disabled"]
    DSP_DRAM1_ACCESS_DISABLE = 0,
    #[doc = "1: DSP DRAM1 access enabled"]
    DSP_DRAM1_ACCESS_ENABLE = 1,
}
impl From<DSP_DRAM1_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_DRAM1_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_DRAM1_ACCESS`"]
pub type DSP_DRAM1_ACCESS_R = crate::R<bool, DSP_DRAM1_ACCESS_A>;
impl DSP_DRAM1_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_DRAM1_ACCESS_A {
        match self.bits {
            false => DSP_DRAM1_ACCESS_A::DSP_DRAM1_ACCESS_DISABLE,
            true => DSP_DRAM1_ACCESS_A::DSP_DRAM1_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM1_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram1_access_disable(&self) -> bool {
        *self == DSP_DRAM1_ACCESS_A::DSP_DRAM1_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM1_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram1_access_enable(&self) -> bool {
        *self == DSP_DRAM1_ACCESS_A::DSP_DRAM1_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_DRAM1_ACCESS`"]
pub struct DSP_DRAM1_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM1_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM1_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP DRAM1 access disabled"]
    #[inline(always)]
    pub fn dsp_dram1_access_disable(self) -> &'a mut W {
        self.variant(DSP_DRAM1_ACCESS_A::DSP_DRAM1_ACCESS_DISABLE)
    }
    #[doc = "DSP DRAM1 access enabled"]
    #[inline(always)]
    pub fn dsp_dram1_access_enable(self) -> &'a mut W {
        self.variant(DSP_DRAM1_ACCESS_A::DSP_DRAM1_ACCESS_ENABLE)
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
#[doc = "DSP PRAM0 access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_DRAM0_ACCESS_A {
    #[doc = "0: DSP DRAM0 access disabled"]
    DSP_DRAM0_ACCESS_DISABLE = 0,
    #[doc = "1: DSP DRAM0 access enabled"]
    DSP_DRAM0_ACCESS_ENABLE = 1,
}
impl From<DSP_DRAM0_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_DRAM0_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_DRAM0_ACCESS`"]
pub type DSP_DRAM0_ACCESS_R = crate::R<bool, DSP_DRAM0_ACCESS_A>;
impl DSP_DRAM0_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_DRAM0_ACCESS_A {
        match self.bits {
            false => DSP_DRAM0_ACCESS_A::DSP_DRAM0_ACCESS_DISABLE,
            true => DSP_DRAM0_ACCESS_A::DSP_DRAM0_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM0_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram0_access_disable(&self) -> bool {
        *self == DSP_DRAM0_ACCESS_A::DSP_DRAM0_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_DRAM0_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_dram0_access_enable(&self) -> bool {
        *self == DSP_DRAM0_ACCESS_A::DSP_DRAM0_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_DRAM0_ACCESS`"]
pub struct DSP_DRAM0_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_DRAM0_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_DRAM0_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP DRAM0 access disabled"]
    #[inline(always)]
    pub fn dsp_dram0_access_disable(self) -> &'a mut W {
        self.variant(DSP_DRAM0_ACCESS_A::DSP_DRAM0_ACCESS_DISABLE)
    }
    #[doc = "DSP DRAM0 access enabled"]
    #[inline(always)]
    pub fn dsp_dram0_access_enable(self) -> &'a mut W {
        self.variant(DSP_DRAM0_ACCESS_A::DSP_DRAM0_ACCESS_ENABLE)
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
#[doc = "DSP PRAM0 access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_PRAM3_ACCESS_A {
    #[doc = "0: DSP PRAM3 access disabled"]
    DSP_PRAM3_ACCESS_DISABLE = 0,
    #[doc = "1: DSP PRAM3 access enabled"]
    DSP_PRAM3_ACCESS_ENABLE = 1,
}
impl From<DSP_PRAM3_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_PRAM3_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_PRAM3_ACCESS`"]
pub type DSP_PRAM3_ACCESS_R = crate::R<bool, DSP_PRAM3_ACCESS_A>;
impl DSP_PRAM3_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_PRAM3_ACCESS_A {
        match self.bits {
            false => DSP_PRAM3_ACCESS_A::DSP_PRAM3_ACCESS_DISABLE,
            true => DSP_PRAM3_ACCESS_A::DSP_PRAM3_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM3_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram3_access_disable(&self) -> bool {
        *self == DSP_PRAM3_ACCESS_A::DSP_PRAM3_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM3_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram3_access_enable(&self) -> bool {
        *self == DSP_PRAM3_ACCESS_A::DSP_PRAM3_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_PRAM3_ACCESS`"]
pub struct DSP_PRAM3_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM3_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM3_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP PRAM3 access disabled"]
    #[inline(always)]
    pub fn dsp_pram3_access_disable(self) -> &'a mut W {
        self.variant(DSP_PRAM3_ACCESS_A::DSP_PRAM3_ACCESS_DISABLE)
    }
    #[doc = "DSP PRAM3 access enabled"]
    #[inline(always)]
    pub fn dsp_pram3_access_enable(self) -> &'a mut W {
        self.variant(DSP_PRAM3_ACCESS_A::DSP_PRAM3_ACCESS_ENABLE)
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
#[doc = "DSP PRAM0 access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_PRAM2_ACCESS_A {
    #[doc = "0: DSP PRAM2 access disabled"]
    DSP_PRAM2_ACCESS_DISABLE = 0,
    #[doc = "1: DSP PRAM2 access enabled"]
    DSP_PRAM2_ACCESS_ENABLE = 1,
}
impl From<DSP_PRAM2_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_PRAM2_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_PRAM2_ACCESS`"]
pub type DSP_PRAM2_ACCESS_R = crate::R<bool, DSP_PRAM2_ACCESS_A>;
impl DSP_PRAM2_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_PRAM2_ACCESS_A {
        match self.bits {
            false => DSP_PRAM2_ACCESS_A::DSP_PRAM2_ACCESS_DISABLE,
            true => DSP_PRAM2_ACCESS_A::DSP_PRAM2_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM2_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram2_access_disable(&self) -> bool {
        *self == DSP_PRAM2_ACCESS_A::DSP_PRAM2_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM2_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram2_access_enable(&self) -> bool {
        *self == DSP_PRAM2_ACCESS_A::DSP_PRAM2_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_PRAM2_ACCESS`"]
pub struct DSP_PRAM2_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM2_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM2_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP PRAM2 access disabled"]
    #[inline(always)]
    pub fn dsp_pram2_access_disable(self) -> &'a mut W {
        self.variant(DSP_PRAM2_ACCESS_A::DSP_PRAM2_ACCESS_DISABLE)
    }
    #[doc = "DSP PRAM2 access enabled"]
    #[inline(always)]
    pub fn dsp_pram2_access_enable(self) -> &'a mut W {
        self.variant(DSP_PRAM2_ACCESS_A::DSP_PRAM2_ACCESS_ENABLE)
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
#[doc = "DSP PRAM0 access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_PRAM1_ACCESS_A {
    #[doc = "0: DSP PRAM1 access disabled"]
    DSP_PRAM1_ACCESS_DISABLE = 0,
    #[doc = "1: DSP PRAM1 access enabled"]
    DSP_PRAM1_ACCESS_ENABLE = 1,
}
impl From<DSP_PRAM1_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_PRAM1_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_PRAM1_ACCESS`"]
pub type DSP_PRAM1_ACCESS_R = crate::R<bool, DSP_PRAM1_ACCESS_A>;
impl DSP_PRAM1_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_PRAM1_ACCESS_A {
        match self.bits {
            false => DSP_PRAM1_ACCESS_A::DSP_PRAM1_ACCESS_DISABLE,
            true => DSP_PRAM1_ACCESS_A::DSP_PRAM1_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM1_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram1_access_disable(&self) -> bool {
        *self == DSP_PRAM1_ACCESS_A::DSP_PRAM1_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM1_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram1_access_enable(&self) -> bool {
        *self == DSP_PRAM1_ACCESS_A::DSP_PRAM1_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_PRAM1_ACCESS`"]
pub struct DSP_PRAM1_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM1_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM1_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP PRAM1 access disabled"]
    #[inline(always)]
    pub fn dsp_pram1_access_disable(self) -> &'a mut W {
        self.variant(DSP_PRAM1_ACCESS_A::DSP_PRAM1_ACCESS_DISABLE)
    }
    #[doc = "DSP PRAM1 access enabled"]
    #[inline(always)]
    pub fn dsp_pram1_access_enable(self) -> &'a mut W {
        self.variant(DSP_PRAM1_ACCESS_A::DSP_PRAM1_ACCESS_ENABLE)
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
#[doc = "DSP PRAM0 access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSP_PRAM0_ACCESS_A {
    #[doc = "0: DSP PRAM0 access disabled"]
    DSP_PRAM0_ACCESS_DISABLE = 0,
    #[doc = "1: DSP PRAM0 access enabled"]
    DSP_PRAM0_ACCESS_ENABLE = 1,
}
impl From<DSP_PRAM0_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_PRAM0_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSP_PRAM0_ACCESS`"]
pub type DSP_PRAM0_ACCESS_R = crate::R<bool, DSP_PRAM0_ACCESS_A>;
impl DSP_PRAM0_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_PRAM0_ACCESS_A {
        match self.bits {
            false => DSP_PRAM0_ACCESS_A::DSP_PRAM0_ACCESS_DISABLE,
            true => DSP_PRAM0_ACCESS_A::DSP_PRAM0_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM0_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram0_access_disable(&self) -> bool {
        *self == DSP_PRAM0_ACCESS_A::DSP_PRAM0_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `DSP_PRAM0_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_dsp_pram0_access_enable(&self) -> bool {
        *self == DSP_PRAM0_ACCESS_A::DSP_PRAM0_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `DSP_PRAM0_ACCESS`"]
pub struct DSP_PRAM0_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_PRAM0_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSP_PRAM0_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP PRAM0 access disabled"]
    #[inline(always)]
    pub fn dsp_pram0_access_disable(self) -> &'a mut W {
        self.variant(DSP_PRAM0_ACCESS_A::DSP_PRAM0_ACCESS_DISABLE)
    }
    #[doc = "DSP PRAM0 access enabled"]
    #[inline(always)]
    pub fn dsp_pram0_access_enable(self) -> &'a mut W {
        self.variant(DSP_PRAM0_ACCESS_A::DSP_PRAM0_ACCESS_ENABLE)
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
#[doc = "Baseband DRAM1 access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_DRAM1_ACCESS_A {
    #[doc = "0: Baseband DRAM1 access disabled"]
    BB_DRAM1_ACCESS_DISABLE = 0,
    #[doc = "1: Baseband DRAM1 access enabled"]
    BB_DRAM1_ACCESS_ENABLE = 1,
}
impl From<BB_DRAM1_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: BB_DRAM1_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BB_DRAM1_ACCESS`"]
pub type BB_DRAM1_ACCESS_R = crate::R<bool, BB_DRAM1_ACCESS_A>;
impl BB_DRAM1_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_DRAM1_ACCESS_A {
        match self.bits {
            false => BB_DRAM1_ACCESS_A::BB_DRAM1_ACCESS_DISABLE,
            true => BB_DRAM1_ACCESS_A::BB_DRAM1_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BB_DRAM1_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_bb_dram1_access_disable(&self) -> bool {
        *self == BB_DRAM1_ACCESS_A::BB_DRAM1_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `BB_DRAM1_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_bb_dram1_access_enable(&self) -> bool {
        *self == BB_DRAM1_ACCESS_A::BB_DRAM1_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `BB_DRAM1_ACCESS`"]
pub struct BB_DRAM1_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_DRAM1_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_DRAM1_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Baseband DRAM1 access disabled"]
    #[inline(always)]
    pub fn bb_dram1_access_disable(self) -> &'a mut W {
        self.variant(BB_DRAM1_ACCESS_A::BB_DRAM1_ACCESS_DISABLE)
    }
    #[doc = "Baseband DRAM1 access enabled"]
    #[inline(always)]
    pub fn bb_dram1_access_enable(self) -> &'a mut W {
        self.variant(BB_DRAM1_ACCESS_A::BB_DRAM1_ACCESS_ENABLE)
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
#[doc = "Baseband DRAM0 access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_DRAM0_ACCESS_A {
    #[doc = "0: Baseband DRAM0 access disabled"]
    BB_DRAM0_ACCESS_DISABLE = 0,
    #[doc = "1: Baseband DRAM0 access enabled"]
    BB_DRAM0_ACCESS_ENABLE = 1,
}
impl From<BB_DRAM0_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: BB_DRAM0_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BB_DRAM0_ACCESS`"]
pub type BB_DRAM0_ACCESS_R = crate::R<bool, BB_DRAM0_ACCESS_A>;
impl BB_DRAM0_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_DRAM0_ACCESS_A {
        match self.bits {
            false => BB_DRAM0_ACCESS_A::BB_DRAM0_ACCESS_DISABLE,
            true => BB_DRAM0_ACCESS_A::BB_DRAM0_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BB_DRAM0_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_bb_dram0_access_disable(&self) -> bool {
        *self == BB_DRAM0_ACCESS_A::BB_DRAM0_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `BB_DRAM0_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_bb_dram0_access_enable(&self) -> bool {
        *self == BB_DRAM0_ACCESS_A::BB_DRAM0_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `BB_DRAM0_ACCESS`"]
pub struct BB_DRAM0_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_DRAM0_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_DRAM0_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Baseband DRAM0 access disabled"]
    #[inline(always)]
    pub fn bb_dram0_access_disable(self) -> &'a mut W {
        self.variant(BB_DRAM0_ACCESS_A::BB_DRAM0_ACCESS_DISABLE)
    }
    #[doc = "Baseband DRAM0 access enabled"]
    #[inline(always)]
    pub fn bb_dram0_access_enable(self) -> &'a mut W {
        self.variant(BB_DRAM0_ACCESS_A::BB_DRAM0_ACCESS_ENABLE)
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
#[doc = "DRAM2 access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRAM2_ACCESS_A {
    #[doc = "0: DRAM2 access disabled"]
    DRAM2_ACCESS_DISABLE = 0,
    #[doc = "1: DRAM2 access enabled"]
    DRAM2_ACCESS_ENABLE = 1,
}
impl From<DRAM2_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: DRAM2_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRAM2_ACCESS`"]
pub type DRAM2_ACCESS_R = crate::R<bool, DRAM2_ACCESS_A>;
impl DRAM2_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRAM2_ACCESS_A {
        match self.bits {
            false => DRAM2_ACCESS_A::DRAM2_ACCESS_DISABLE,
            true => DRAM2_ACCESS_A::DRAM2_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DRAM2_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_dram2_access_disable(&self) -> bool {
        *self == DRAM2_ACCESS_A::DRAM2_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `DRAM2_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_dram2_access_enable(&self) -> bool {
        *self == DRAM2_ACCESS_A::DRAM2_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `DRAM2_ACCESS`"]
pub struct DRAM2_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> DRAM2_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRAM2_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DRAM2 access disabled"]
    #[inline(always)]
    pub fn dram2_access_disable(self) -> &'a mut W {
        self.variant(DRAM2_ACCESS_A::DRAM2_ACCESS_DISABLE)
    }
    #[doc = "DRAM2 access enabled"]
    #[inline(always)]
    pub fn dram2_access_enable(self) -> &'a mut W {
        self.variant(DRAM2_ACCESS_A::DRAM2_ACCESS_ENABLE)
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
#[doc = "DRAM1 access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRAM1_ACCESS_A {
    #[doc = "0: DRAM1 access disabled"]
    DRAM1_ACCESS_DISABLE = 0,
    #[doc = "1: DRAM1 access enabled"]
    DRAM1_ACCESS_ENABLE = 1,
}
impl From<DRAM1_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: DRAM1_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRAM1_ACCESS`"]
pub type DRAM1_ACCESS_R = crate::R<bool, DRAM1_ACCESS_A>;
impl DRAM1_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRAM1_ACCESS_A {
        match self.bits {
            false => DRAM1_ACCESS_A::DRAM1_ACCESS_DISABLE,
            true => DRAM1_ACCESS_A::DRAM1_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DRAM1_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_dram1_access_disable(&self) -> bool {
        *self == DRAM1_ACCESS_A::DRAM1_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `DRAM1_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_dram1_access_enable(&self) -> bool {
        *self == DRAM1_ACCESS_A::DRAM1_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `DRAM1_ACCESS`"]
pub struct DRAM1_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> DRAM1_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRAM1_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DRAM1 access disabled"]
    #[inline(always)]
    pub fn dram1_access_disable(self) -> &'a mut W {
        self.variant(DRAM1_ACCESS_A::DRAM1_ACCESS_DISABLE)
    }
    #[doc = "DRAM1 access enabled"]
    #[inline(always)]
    pub fn dram1_access_enable(self) -> &'a mut W {
        self.variant(DRAM1_ACCESS_A::DRAM1_ACCESS_ENABLE)
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
#[doc = "DRAM0 access configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRAM0_ACCESS_A {
    #[doc = "0: DRAM0 access disabled"]
    DRAM0_ACCESS_DISABLE = 0,
    #[doc = "1: DRAM0 access enabled"]
    DRAM0_ACCESS_ENABLE = 1,
}
impl From<DRAM0_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: DRAM0_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRAM0_ACCESS`"]
pub type DRAM0_ACCESS_R = crate::R<bool, DRAM0_ACCESS_A>;
impl DRAM0_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRAM0_ACCESS_A {
        match self.bits {
            false => DRAM0_ACCESS_A::DRAM0_ACCESS_DISABLE,
            true => DRAM0_ACCESS_A::DRAM0_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DRAM0_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_dram0_access_disable(&self) -> bool {
        *self == DRAM0_ACCESS_A::DRAM0_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `DRAM0_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_dram0_access_enable(&self) -> bool {
        *self == DRAM0_ACCESS_A::DRAM0_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `DRAM0_ACCESS`"]
pub struct DRAM0_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> DRAM0_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRAM0_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DRAM0 access disabled"]
    #[inline(always)]
    pub fn dram0_access_disable(self) -> &'a mut W {
        self.variant(DRAM0_ACCESS_A::DRAM0_ACCESS_DISABLE)
    }
    #[doc = "DRAM0 access enabled"]
    #[inline(always)]
    pub fn dram0_access_enable(self) -> &'a mut W {
        self.variant(DRAM0_ACCESS_A::DRAM0_ACCESS_ENABLE)
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
#[doc = "PRAM3 access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRAM3_ACCESS_A {
    #[doc = "0: PRAM3 access disabled"]
    PRAM3_ACCESS_DISABLE = 0,
    #[doc = "1: PRAM3 access enabled"]
    PRAM3_ACCESS_ENABLE = 1,
}
impl From<PRAM3_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: PRAM3_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRAM3_ACCESS`"]
pub type PRAM3_ACCESS_R = crate::R<bool, PRAM3_ACCESS_A>;
impl PRAM3_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRAM3_ACCESS_A {
        match self.bits {
            false => PRAM3_ACCESS_A::PRAM3_ACCESS_DISABLE,
            true => PRAM3_ACCESS_A::PRAM3_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PRAM3_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_pram3_access_disable(&self) -> bool {
        *self == PRAM3_ACCESS_A::PRAM3_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `PRAM3_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_pram3_access_enable(&self) -> bool {
        *self == PRAM3_ACCESS_A::PRAM3_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `PRAM3_ACCESS`"]
pub struct PRAM3_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRAM3_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRAM3_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PRAM3 access disabled"]
    #[inline(always)]
    pub fn pram3_access_disable(self) -> &'a mut W {
        self.variant(PRAM3_ACCESS_A::PRAM3_ACCESS_DISABLE)
    }
    #[doc = "PRAM3 access enabled"]
    #[inline(always)]
    pub fn pram3_access_enable(self) -> &'a mut W {
        self.variant(PRAM3_ACCESS_A::PRAM3_ACCESS_ENABLE)
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
#[doc = "PRAM2 access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRAM2_ACCESS_A {
    #[doc = "0: PRAM2 access disabled"]
    PRAM2_ACCESS_DISABLE = 0,
    #[doc = "1: PRAM2 access enabled"]
    PRAM2_ACCESS_ENABLE = 1,
}
impl From<PRAM2_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: PRAM2_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRAM2_ACCESS`"]
pub type PRAM2_ACCESS_R = crate::R<bool, PRAM2_ACCESS_A>;
impl PRAM2_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRAM2_ACCESS_A {
        match self.bits {
            false => PRAM2_ACCESS_A::PRAM2_ACCESS_DISABLE,
            true => PRAM2_ACCESS_A::PRAM2_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PRAM2_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_pram2_access_disable(&self) -> bool {
        *self == PRAM2_ACCESS_A::PRAM2_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `PRAM2_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_pram2_access_enable(&self) -> bool {
        *self == PRAM2_ACCESS_A::PRAM2_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `PRAM2_ACCESS`"]
pub struct PRAM2_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRAM2_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRAM2_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PRAM2 access disabled"]
    #[inline(always)]
    pub fn pram2_access_disable(self) -> &'a mut W {
        self.variant(PRAM2_ACCESS_A::PRAM2_ACCESS_DISABLE)
    }
    #[doc = "PRAM2 access enabled"]
    #[inline(always)]
    pub fn pram2_access_enable(self) -> &'a mut W {
        self.variant(PRAM2_ACCESS_A::PRAM2_ACCESS_ENABLE)
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
#[doc = "PRAM1 access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRAM1_ACCESS_A {
    #[doc = "0: PRAM1 access disabled"]
    PRAM1_ACCESS_DISABLE = 0,
    #[doc = "1: PRAM1 access enabled"]
    PRAM1_ACCESS_ENABLE = 1,
}
impl From<PRAM1_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: PRAM1_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRAM1_ACCESS`"]
pub type PRAM1_ACCESS_R = crate::R<bool, PRAM1_ACCESS_A>;
impl PRAM1_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRAM1_ACCESS_A {
        match self.bits {
            false => PRAM1_ACCESS_A::PRAM1_ACCESS_DISABLE,
            true => PRAM1_ACCESS_A::PRAM1_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PRAM1_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_pram1_access_disable(&self) -> bool {
        *self == PRAM1_ACCESS_A::PRAM1_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `PRAM1_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_pram1_access_enable(&self) -> bool {
        *self == PRAM1_ACCESS_A::PRAM1_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `PRAM1_ACCESS`"]
pub struct PRAM1_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRAM1_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRAM1_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PRAM1 access disabled"]
    #[inline(always)]
    pub fn pram1_access_disable(self) -> &'a mut W {
        self.variant(PRAM1_ACCESS_A::PRAM1_ACCESS_DISABLE)
    }
    #[doc = "PRAM1 access enabled"]
    #[inline(always)]
    pub fn pram1_access_enable(self) -> &'a mut W {
        self.variant(PRAM1_ACCESS_A::PRAM1_ACCESS_ENABLE)
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
#[doc = "PRAM0 access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRAM0_ACCESS_A {
    #[doc = "0: PRAM0 access disabled"]
    PRAM0_ACCESS_DISABLE = 0,
    #[doc = "1: PRAM0 access enabled"]
    PRAM0_ACCESS_ENABLE = 1,
}
impl From<PRAM0_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: PRAM0_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRAM0_ACCESS`"]
pub type PRAM0_ACCESS_R = crate::R<bool, PRAM0_ACCESS_A>;
impl PRAM0_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRAM0_ACCESS_A {
        match self.bits {
            false => PRAM0_ACCESS_A::PRAM0_ACCESS_DISABLE,
            true => PRAM0_ACCESS_A::PRAM0_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PRAM0_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_pram0_access_disable(&self) -> bool {
        *self == PRAM0_ACCESS_A::PRAM0_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `PRAM0_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_pram0_access_enable(&self) -> bool {
        *self == PRAM0_ACCESS_A::PRAM0_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `PRAM0_ACCESS`"]
pub struct PRAM0_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRAM0_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRAM0_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PRAM0 access disabled"]
    #[inline(always)]
    pub fn pram0_access_disable(self) -> &'a mut W {
        self.variant(PRAM0_ACCESS_A::PRAM0_ACCESS_DISABLE)
    }
    #[doc = "PRAM0 access enabled"]
    #[inline(always)]
    pub fn pram0_access_enable(self) -> &'a mut W {
        self.variant(PRAM0_ACCESS_A::PRAM0_ACCESS_ENABLE)
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
#[doc = "Flash access configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_ACCESS_A {
    #[doc = "0: Flash access disabled"]
    FLASH_ACCESS_DISABLE = 0,
    #[doc = "1: Flash access enabled"]
    FLASH_ACCESS_ENABLE = 1,
}
impl From<FLASH_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLASH_ACCESS`"]
pub type FLASH_ACCESS_R = crate::R<bool, FLASH_ACCESS_A>;
impl FLASH_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_ACCESS_A {
        match self.bits {
            false => FLASH_ACCESS_A::FLASH_ACCESS_DISABLE,
            true => FLASH_ACCESS_A::FLASH_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_flash_access_disable(&self) -> bool {
        *self == FLASH_ACCESS_A::FLASH_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `FLASH_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_flash_access_enable(&self) -> bool {
        *self == FLASH_ACCESS_A::FLASH_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `FLASH_ACCESS`"]
pub struct FLASH_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash access disabled"]
    #[inline(always)]
    pub fn flash_access_disable(self) -> &'a mut W {
        self.variant(FLASH_ACCESS_A::FLASH_ACCESS_DISABLE)
    }
    #[doc = "Flash access enabled"]
    #[inline(always)]
    pub fn flash_access_enable(self) -> &'a mut W {
        self.variant(FLASH_ACCESS_A::FLASH_ACCESS_ENABLE)
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
#[doc = "PROM access configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROM_ACCESS_A {
    #[doc = "0: PROM access disabled"]
    PROM_ACCESS_DISABLE = 0,
    #[doc = "1: PROM access enabled"]
    PROM_ACCESS_ENABLE = 1,
}
impl From<PROM_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: PROM_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PROM_ACCESS`"]
pub type PROM_ACCESS_R = crate::R<bool, PROM_ACCESS_A>;
impl PROM_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROM_ACCESS_A {
        match self.bits {
            false => PROM_ACCESS_A::PROM_ACCESS_DISABLE,
            true => PROM_ACCESS_A::PROM_ACCESS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PROM_ACCESS_DISABLE`"]
    #[inline(always)]
    pub fn is_prom_access_disable(&self) -> bool {
        *self == PROM_ACCESS_A::PROM_ACCESS_DISABLE
    }
    #[doc = "Checks if the value of the field is `PROM_ACCESS_ENABLE`"]
    #[inline(always)]
    pub fn is_prom_access_enable(&self) -> bool {
        *self == PROM_ACCESS_A::PROM_ACCESS_ENABLE
    }
}
#[doc = "Write proxy for field `PROM_ACCESS`"]
pub struct PROM_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> PROM_ACCESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROM_ACCESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PROM access disabled"]
    #[inline(always)]
    pub fn prom_access_disable(self) -> &'a mut W {
        self.variant(PROM_ACCESS_A::PROM_ACCESS_DISABLE)
    }
    #[doc = "PROM access enabled"]
    #[inline(always)]
    pub fn prom_access_enable(self) -> &'a mut W {
        self.variant(PROM_ACCESS_A::PROM_ACCESS_ENABLE)
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
    #[doc = "Bits 24:30 - Wakeup restore address in packed 7-bit format. When written, SYSCTRL_WAKEUP_ADDR is updated. This field reads back as zero when SYSCTRL_WAKEUP_ADDR does not point to an enabled RAM instance."]
    #[inline(always)]
    pub fn wakeup_addr_packed(&self) -> WAKEUP_ADDR_PACKED_R {
        WAKEUP_ADDR_PACKED_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 21 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_dram5_access(&self) -> DSP_DRAM5_ACCESS_R {
        DSP_DRAM5_ACCESS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_dram4_access(&self) -> DSP_DRAM4_ACCESS_R {
        DSP_DRAM4_ACCESS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_dram3_access(&self) -> DSP_DRAM3_ACCESS_R {
        DSP_DRAM3_ACCESS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_dram2_access(&self) -> DSP_DRAM2_ACCESS_R {
        DSP_DRAM2_ACCESS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_dram1_access(&self) -> DSP_DRAM1_ACCESS_R {
        DSP_DRAM1_ACCESS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_dram0_access(&self) -> DSP_DRAM0_ACCESS_R {
        DSP_DRAM0_ACCESS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_pram3_access(&self) -> DSP_PRAM3_ACCESS_R {
        DSP_PRAM3_ACCESS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_pram2_access(&self) -> DSP_PRAM2_ACCESS_R {
        DSP_PRAM2_ACCESS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_pram1_access(&self) -> DSP_PRAM1_ACCESS_R {
        DSP_PRAM1_ACCESS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_pram0_access(&self) -> DSP_PRAM0_ACCESS_R {
        DSP_PRAM0_ACCESS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Baseband DRAM1 access configuration"]
    #[inline(always)]
    pub fn bb_dram1_access(&self) -> BB_DRAM1_ACCESS_R {
        BB_DRAM1_ACCESS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Baseband DRAM0 access configuration"]
    #[inline(always)]
    pub fn bb_dram0_access(&self) -> BB_DRAM0_ACCESS_R {
        BB_DRAM0_ACCESS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DRAM2 access configuration"]
    #[inline(always)]
    pub fn dram2_access(&self) -> DRAM2_ACCESS_R {
        DRAM2_ACCESS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DRAM1 access configuration"]
    #[inline(always)]
    pub fn dram1_access(&self) -> DRAM1_ACCESS_R {
        DRAM1_ACCESS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DRAM0 access configuration"]
    #[inline(always)]
    pub fn dram0_access(&self) -> DRAM0_ACCESS_R {
        DRAM0_ACCESS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PRAM3 access configuration"]
    #[inline(always)]
    pub fn pram3_access(&self) -> PRAM3_ACCESS_R {
        PRAM3_ACCESS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PRAM2 access configuration"]
    #[inline(always)]
    pub fn pram2_access(&self) -> PRAM2_ACCESS_R {
        PRAM2_ACCESS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PRAM1 access configuration"]
    #[inline(always)]
    pub fn pram1_access(&self) -> PRAM1_ACCESS_R {
        PRAM1_ACCESS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PRAM0 access configuration"]
    #[inline(always)]
    pub fn pram0_access(&self) -> PRAM0_ACCESS_R {
        PRAM0_ACCESS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Flash access configuration"]
    #[inline(always)]
    pub fn flash_access(&self) -> FLASH_ACCESS_R {
        FLASH_ACCESS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PROM access configuration"]
    #[inline(always)]
    pub fn prom_access(&self) -> PROM_ACCESS_R {
        PROM_ACCESS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:30 - Wakeup restore address in packed 7-bit format. When written, SYSCTRL_WAKEUP_ADDR is updated. This field reads back as zero when SYSCTRL_WAKEUP_ADDR does not point to an enabled RAM instance."]
    #[inline(always)]
    pub fn wakeup_addr_packed(&mut self) -> WAKEUP_ADDR_PACKED_W {
        WAKEUP_ADDR_PACKED_W { w: self }
    }
    #[doc = "Bit 21 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_dram5_access(&mut self) -> DSP_DRAM5_ACCESS_W {
        DSP_DRAM5_ACCESS_W { w: self }
    }
    #[doc = "Bit 20 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_dram4_access(&mut self) -> DSP_DRAM4_ACCESS_W {
        DSP_DRAM4_ACCESS_W { w: self }
    }
    #[doc = "Bit 19 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_dram3_access(&mut self) -> DSP_DRAM3_ACCESS_W {
        DSP_DRAM3_ACCESS_W { w: self }
    }
    #[doc = "Bit 18 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_dram2_access(&mut self) -> DSP_DRAM2_ACCESS_W {
        DSP_DRAM2_ACCESS_W { w: self }
    }
    #[doc = "Bit 17 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_dram1_access(&mut self) -> DSP_DRAM1_ACCESS_W {
        DSP_DRAM1_ACCESS_W { w: self }
    }
    #[doc = "Bit 16 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_dram0_access(&mut self) -> DSP_DRAM0_ACCESS_W {
        DSP_DRAM0_ACCESS_W { w: self }
    }
    #[doc = "Bit 15 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_pram3_access(&mut self) -> DSP_PRAM3_ACCESS_W {
        DSP_PRAM3_ACCESS_W { w: self }
    }
    #[doc = "Bit 14 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_pram2_access(&mut self) -> DSP_PRAM2_ACCESS_W {
        DSP_PRAM2_ACCESS_W { w: self }
    }
    #[doc = "Bit 13 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_pram1_access(&mut self) -> DSP_PRAM1_ACCESS_W {
        DSP_PRAM1_ACCESS_W { w: self }
    }
    #[doc = "Bit 12 - DSP PRAM0 access configuration"]
    #[inline(always)]
    pub fn dsp_pram0_access(&mut self) -> DSP_PRAM0_ACCESS_W {
        DSP_PRAM0_ACCESS_W { w: self }
    }
    #[doc = "Bit 11 - Baseband DRAM1 access configuration"]
    #[inline(always)]
    pub fn bb_dram1_access(&mut self) -> BB_DRAM1_ACCESS_W {
        BB_DRAM1_ACCESS_W { w: self }
    }
    #[doc = "Bit 10 - Baseband DRAM0 access configuration"]
    #[inline(always)]
    pub fn bb_dram0_access(&mut self) -> BB_DRAM0_ACCESS_W {
        BB_DRAM0_ACCESS_W { w: self }
    }
    #[doc = "Bit 8 - DRAM2 access configuration"]
    #[inline(always)]
    pub fn dram2_access(&mut self) -> DRAM2_ACCESS_W {
        DRAM2_ACCESS_W { w: self }
    }
    #[doc = "Bit 7 - DRAM1 access configuration"]
    #[inline(always)]
    pub fn dram1_access(&mut self) -> DRAM1_ACCESS_W {
        DRAM1_ACCESS_W { w: self }
    }
    #[doc = "Bit 6 - DRAM0 access configuration"]
    #[inline(always)]
    pub fn dram0_access(&mut self) -> DRAM0_ACCESS_W {
        DRAM0_ACCESS_W { w: self }
    }
    #[doc = "Bit 5 - PRAM3 access configuration"]
    #[inline(always)]
    pub fn pram3_access(&mut self) -> PRAM3_ACCESS_W {
        PRAM3_ACCESS_W { w: self }
    }
    #[doc = "Bit 4 - PRAM2 access configuration"]
    #[inline(always)]
    pub fn pram2_access(&mut self) -> PRAM2_ACCESS_W {
        PRAM2_ACCESS_W { w: self }
    }
    #[doc = "Bit 3 - PRAM1 access configuration"]
    #[inline(always)]
    pub fn pram1_access(&mut self) -> PRAM1_ACCESS_W {
        PRAM1_ACCESS_W { w: self }
    }
    #[doc = "Bit 2 - PRAM0 access configuration"]
    #[inline(always)]
    pub fn pram0_access(&mut self) -> PRAM0_ACCESS_W {
        PRAM0_ACCESS_W { w: self }
    }
    #[doc = "Bit 1 - Flash access configuration"]
    #[inline(always)]
    pub fn flash_access(&mut self) -> FLASH_ACCESS_W {
        FLASH_ACCESS_W { w: self }
    }
    #[doc = "Bit 0 - PROM access configuration"]
    #[inline(always)]
    pub fn prom_access(&mut self) -> PROM_ACCESS_W {
        PROM_ACCESS_W { w: self }
    }
}
