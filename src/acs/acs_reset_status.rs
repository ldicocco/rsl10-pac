#[doc = "Reader of register ACS_RESET_STATUS"]
pub type R = crate::R<u32, super::ACS_RESET_STATUS>;
#[doc = "Writer for register ACS_RESET_STATUS"]
pub type W = crate::W<u32, super::ACS_RESET_STATUS>;
#[doc = "Register ACS_RESET_STATUS `reset()`'s with value 0x1d00"]
impl crate::ResetValue for super::ACS_RESET_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1d00
    }
}
#[doc = "Sticky flag that detects that a timeout in the power up sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_RESET_FLAG_A {
    #[doc = "0: The timeout reset has not triggered at least once"]
    TIMEOUT_RESET_FLAG_NOT_SET = 0,
    #[doc = "1: The timeout reset was triggered at least once since this status bit was last cleared"]
    TIMEOUT_RESET_FLAG_SET = 1,
}
impl From<TIMEOUT_RESET_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_RESET_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMEOUT_RESET_FLAG`"]
pub type TIMEOUT_RESET_FLAG_R = crate::R<bool, TIMEOUT_RESET_FLAG_A>;
impl TIMEOUT_RESET_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUT_RESET_FLAG_A {
        match self.bits {
            false => TIMEOUT_RESET_FLAG_A::TIMEOUT_RESET_FLAG_NOT_SET,
            true => TIMEOUT_RESET_FLAG_A::TIMEOUT_RESET_FLAG_SET,
        }
    }
    #[doc = "Checks if the value of the field is `TIMEOUT_RESET_FLAG_NOT_SET`"]
    #[inline(always)]
    pub fn is_timeout_reset_flag_not_set(&self) -> bool {
        *self == TIMEOUT_RESET_FLAG_A::TIMEOUT_RESET_FLAG_NOT_SET
    }
    #[doc = "Checks if the value of the field is `TIMEOUT_RESET_FLAG_SET`"]
    #[inline(always)]
    pub fn is_timeout_reset_flag_set(&self) -> bool {
        *self == TIMEOUT_RESET_FLAG_A::TIMEOUT_RESET_FLAG_SET
    }
}
#[doc = "Sticky flag that detects that a clock detector reset occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_DET_RESET_FLAG_A {
    #[doc = "0: The clock detector reset has not triggered at least once"]
    CLK_DET_RESET_FLAG_NOT_SET = 0,
    #[doc = "1: The clock detector reset was triggered at least once since this status bit was last cleared"]
    CLK_DET_RESET_FLAG_SET = 1,
}
impl From<CLK_DET_RESET_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_DET_RESET_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLK_DET_RESET_FLAG`"]
pub type CLK_DET_RESET_FLAG_R = crate::R<bool, CLK_DET_RESET_FLAG_A>;
impl CLK_DET_RESET_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_DET_RESET_FLAG_A {
        match self.bits {
            false => CLK_DET_RESET_FLAG_A::CLK_DET_RESET_FLAG_NOT_SET,
            true => CLK_DET_RESET_FLAG_A::CLK_DET_RESET_FLAG_SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DET_RESET_FLAG_NOT_SET`"]
    #[inline(always)]
    pub fn is_clk_det_reset_flag_not_set(&self) -> bool {
        *self == CLK_DET_RESET_FLAG_A::CLK_DET_RESET_FLAG_NOT_SET
    }
    #[doc = "Checks if the value of the field is `CLK_DET_RESET_FLAG_SET`"]
    #[inline(always)]
    pub fn is_clk_det_reset_flag_set(&self) -> bool {
        *self == CLK_DET_RESET_FLAG_A::CLK_DET_RESET_FLAG_SET
    }
}
#[doc = "Sticky flag that detects that a VDDA reset occurred (triggered by vdda_ready = 0)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDA_RESET_FLAG_A {
    #[doc = "0: The VDDA reset has not triggered at least once"]
    VDDA_RESET_FLAG_NOT_SET = 0,
    #[doc = "1: The VDDA reset was triggered at least once since this status bit was last cleared"]
    VDDA_RESET_FLAG_SET = 1,
}
impl From<VDDA_RESET_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: VDDA_RESET_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDDA_RESET_FLAG`"]
pub type VDDA_RESET_FLAG_R = crate::R<bool, VDDA_RESET_FLAG_A>;
impl VDDA_RESET_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDA_RESET_FLAG_A {
        match self.bits {
            false => VDDA_RESET_FLAG_A::VDDA_RESET_FLAG_NOT_SET,
            true => VDDA_RESET_FLAG_A::VDDA_RESET_FLAG_SET,
        }
    }
    #[doc = "Checks if the value of the field is `VDDA_RESET_FLAG_NOT_SET`"]
    #[inline(always)]
    pub fn is_vdda_reset_flag_not_set(&self) -> bool {
        *self == VDDA_RESET_FLAG_A::VDDA_RESET_FLAG_NOT_SET
    }
    #[doc = "Checks if the value of the field is `VDDA_RESET_FLAG_SET`"]
    #[inline(always)]
    pub fn is_vdda_reset_flag_set(&self) -> bool {
        *self == VDDA_RESET_FLAG_A::VDDA_RESET_FLAG_SET
    }
}
#[doc = "Sticky flag that detects that a VDDM reset occurred (triggered by vddm_ready = 0)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDM_RESET_FLAG_A {
    #[doc = "0: The VDDM reset has not triggered at least once"]
    VDDM_RESET_FLAG_NOT_SET = 0,
    #[doc = "1: The VDDM reset was triggered at least once since this status bit was last cleared"]
    VDDM_RESET_FLAG_SET = 1,
}
impl From<VDDM_RESET_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: VDDM_RESET_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDDM_RESET_FLAG`"]
pub type VDDM_RESET_FLAG_R = crate::R<bool, VDDM_RESET_FLAG_A>;
impl VDDM_RESET_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDM_RESET_FLAG_A {
        match self.bits {
            false => VDDM_RESET_FLAG_A::VDDM_RESET_FLAG_NOT_SET,
            true => VDDM_RESET_FLAG_A::VDDM_RESET_FLAG_SET,
        }
    }
    #[doc = "Checks if the value of the field is `VDDM_RESET_FLAG_NOT_SET`"]
    #[inline(always)]
    pub fn is_vddm_reset_flag_not_set(&self) -> bool {
        *self == VDDM_RESET_FLAG_A::VDDM_RESET_FLAG_NOT_SET
    }
    #[doc = "Checks if the value of the field is `VDDM_RESET_FLAG_SET`"]
    #[inline(always)]
    pub fn is_vddm_reset_flag_set(&self) -> bool {
        *self == VDDM_RESET_FLAG_A::VDDM_RESET_FLAG_SET
    }
}
#[doc = "Sticky flag that detects that a VDDC reset occurred (triggered by vddc_ready = 0)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDC_RESET_FLAG_A {
    #[doc = "0: The VDDC reset has not triggered at least once"]
    VDDC_RESET_FLAG_NOT_SET = 0,
    #[doc = "1: The VDDC reset was triggered at least once since this status bit was last cleared"]
    VDDC_RESET_FLAG_SET = 1,
}
impl From<VDDC_RESET_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: VDDC_RESET_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDDC_RESET_FLAG`"]
pub type VDDC_RESET_FLAG_R = crate::R<bool, VDDC_RESET_FLAG_A>;
impl VDDC_RESET_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDC_RESET_FLAG_A {
        match self.bits {
            false => VDDC_RESET_FLAG_A::VDDC_RESET_FLAG_NOT_SET,
            true => VDDC_RESET_FLAG_A::VDDC_RESET_FLAG_SET,
        }
    }
    #[doc = "Checks if the value of the field is `VDDC_RESET_FLAG_NOT_SET`"]
    #[inline(always)]
    pub fn is_vddc_reset_flag_not_set(&self) -> bool {
        *self == VDDC_RESET_FLAG_A::VDDC_RESET_FLAG_NOT_SET
    }
    #[doc = "Checks if the value of the field is `VDDC_RESET_FLAG_SET`"]
    #[inline(always)]
    pub fn is_vddc_reset_flag_set(&self) -> bool {
        *self == VDDC_RESET_FLAG_A::VDDC_RESET_FLAG_SET
    }
}
#[doc = "Sticky flag that detects that a reset occurred due to pad NRESET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD_RESET_FLAG_A {
    #[doc = "0: The NRESET pad reset has not triggered at least once"]
    PAD_RESET_FLAG_NOT_SET = 0,
    #[doc = "1: The NRESET pad reset was triggered at least once since this status bit was last cleared"]
    PAD_RESET_FLAG_SET = 1,
}
impl From<PAD_RESET_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD_RESET_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD_RESET_FLAG`"]
pub type PAD_RESET_FLAG_R = crate::R<bool, PAD_RESET_FLAG_A>;
impl PAD_RESET_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD_RESET_FLAG_A {
        match self.bits {
            false => PAD_RESET_FLAG_A::PAD_RESET_FLAG_NOT_SET,
            true => PAD_RESET_FLAG_A::PAD_RESET_FLAG_SET,
        }
    }
    #[doc = "Checks if the value of the field is `PAD_RESET_FLAG_NOT_SET`"]
    #[inline(always)]
    pub fn is_pad_reset_flag_not_set(&self) -> bool {
        *self == PAD_RESET_FLAG_A::PAD_RESET_FLAG_NOT_SET
    }
    #[doc = "Checks if the value of the field is `PAD_RESET_FLAG_SET`"]
    #[inline(always)]
    pub fn is_pad_reset_flag_set(&self) -> bool {
        *self == PAD_RESET_FLAG_A::PAD_RESET_FLAG_SET
    }
}
#[doc = "Sticky flag that detects that a POR reset occurred\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POR_RESET_FLAG_A {
    #[doc = "0: The POR reset has not triggered at least once"]
    POR_RESET_FLAG_NOT_SET = 0,
    #[doc = "1: The POR reset was triggered at least once since this status bit was last cleared"]
    POR_RESET_FLAG_SET = 1,
}
impl From<POR_RESET_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: POR_RESET_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POR_RESET_FLAG`"]
pub type POR_RESET_FLAG_R = crate::R<bool, POR_RESET_FLAG_A>;
impl POR_RESET_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POR_RESET_FLAG_A {
        match self.bits {
            false => POR_RESET_FLAG_A::POR_RESET_FLAG_NOT_SET,
            true => POR_RESET_FLAG_A::POR_RESET_FLAG_SET,
        }
    }
    #[doc = "Checks if the value of the field is `POR_RESET_FLAG_NOT_SET`"]
    #[inline(always)]
    pub fn is_por_reset_flag_not_set(&self) -> bool {
        *self == POR_RESET_FLAG_A::POR_RESET_FLAG_NOT_SET
    }
    #[doc = "Checks if the value of the field is `POR_RESET_FLAG_SET`"]
    #[inline(always)]
    pub fn is_por_reset_flag_set(&self) -> bool {
        *self == POR_RESET_FLAG_A::POR_RESET_FLAG_SET
    }
}
#[doc = "Reset the sticky TIMEOUT_RESET flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_RESET_FLAG_CLEAR_AW {
    #[doc = "1: Reset the sticky TIMEOUT_RESET flag."]
    TIMEOUT_RESET_FLAG_CLEAR = 1,
}
impl From<TIMEOUT_RESET_FLAG_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_RESET_FLAG_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TIMEOUT_RESET_FLAG_CLEAR`"]
pub struct TIMEOUT_RESET_FLAG_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_RESET_FLAG_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUT_RESET_FLAG_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky TIMEOUT_RESET flag."]
    #[inline(always)]
    pub fn timeout_reset_flag_clear(self) -> &'a mut W {
        self.variant(TIMEOUT_RESET_FLAG_CLEAR_AW::TIMEOUT_RESET_FLAG_CLEAR)
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
#[doc = "Reset the sticky CLK_DET_RESET flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_DET_RESET_FLAG_CLEAR_AW {
    #[doc = "1: Reset the sticky CLK_DET_RESET flag."]
    CLK_DET_RESET_FLAG_CLEAR = 1,
}
impl From<CLK_DET_RESET_FLAG_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: CLK_DET_RESET_FLAG_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CLK_DET_RESET_FLAG_CLEAR`"]
pub struct CLK_DET_RESET_FLAG_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DET_RESET_FLAG_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_DET_RESET_FLAG_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky CLK_DET_RESET flag."]
    #[inline(always)]
    pub fn clk_det_reset_flag_clear(self) -> &'a mut W {
        self.variant(CLK_DET_RESET_FLAG_CLEAR_AW::CLK_DET_RESET_FLAG_CLEAR)
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
#[doc = "Reset the sticky VDDA_RESET flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDA_RESET_FLAG_CLEAR_AW {
    #[doc = "1: Reset the sticky VDDA_RESET flag."]
    VDDA_RESET_FLAG_CLEAR = 1,
}
impl From<VDDA_RESET_FLAG_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: VDDA_RESET_FLAG_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `VDDA_RESET_FLAG_CLEAR`"]
pub struct VDDA_RESET_FLAG_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDA_RESET_FLAG_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDA_RESET_FLAG_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky VDDA_RESET flag."]
    #[inline(always)]
    pub fn vdda_reset_flag_clear(self) -> &'a mut W {
        self.variant(VDDA_RESET_FLAG_CLEAR_AW::VDDA_RESET_FLAG_CLEAR)
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
#[doc = "Reset the sticky VDDM_RESET flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDM_RESET_FLAG_CLEAR_AW {
    #[doc = "1: Reset the sticky VDDM_RESET flag."]
    VDDM_RESET_FLAG_CLEAR = 1,
}
impl From<VDDM_RESET_FLAG_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: VDDM_RESET_FLAG_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `VDDM_RESET_FLAG_CLEAR`"]
pub struct VDDM_RESET_FLAG_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDM_RESET_FLAG_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDM_RESET_FLAG_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky VDDM_RESET flag."]
    #[inline(always)]
    pub fn vddm_reset_flag_clear(self) -> &'a mut W {
        self.variant(VDDM_RESET_FLAG_CLEAR_AW::VDDM_RESET_FLAG_CLEAR)
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
#[doc = "Reset the sticky VDDC_RESET flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDC_RESET_FLAG_CLEAR_AW {
    #[doc = "1: Reset the sticky VDDC_RESET flag."]
    VDDC_RESET_FLAG_CLEAR = 1,
}
impl From<VDDC_RESET_FLAG_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: VDDC_RESET_FLAG_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `VDDC_RESET_FLAG_CLEAR`"]
pub struct VDDC_RESET_FLAG_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDC_RESET_FLAG_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDC_RESET_FLAG_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky VDDC_RESET flag."]
    #[inline(always)]
    pub fn vddc_reset_flag_clear(self) -> &'a mut W {
        self.variant(VDDC_RESET_FLAG_CLEAR_AW::VDDC_RESET_FLAG_CLEAR)
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
#[doc = "Reset the sticky PAD_RESET flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD_RESET_FLAG_CLEAR_AW {
    #[doc = "1: Reset the sticky PAD_RESET flag."]
    PAD_RESET_FLAG_CLEAR = 1,
}
impl From<PAD_RESET_FLAG_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: PAD_RESET_FLAG_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PAD_RESET_FLAG_CLEAR`"]
pub struct PAD_RESET_FLAG_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_RESET_FLAG_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD_RESET_FLAG_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky PAD_RESET flag."]
    #[inline(always)]
    pub fn pad_reset_flag_clear(self) -> &'a mut W {
        self.variant(PAD_RESET_FLAG_CLEAR_AW::PAD_RESET_FLAG_CLEAR)
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
#[doc = "Reset the sticky POR_RESET flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POR_RESET_FLAG_CLEAR_AW {
    #[doc = "1: Reset the sticky POR_RESET flag."]
    POR_RESET_FLAG_CLEAR = 1,
}
impl From<POR_RESET_FLAG_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: POR_RESET_FLAG_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `POR_RESET_FLAG_CLEAR`"]
pub struct POR_RESET_FLAG_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_RESET_FLAG_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POR_RESET_FLAG_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the sticky POR_RESET flag."]
    #[inline(always)]
    pub fn por_reset_flag_clear(self) -> &'a mut W {
        self.variant(POR_RESET_FLAG_CLEAR_AW::POR_RESET_FLAG_CLEAR)
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
    #[doc = "Bit 14 - Sticky flag that detects that a timeout in the power up sequence"]
    #[inline(always)]
    pub fn timeout_reset_flag(&self) -> TIMEOUT_RESET_FLAG_R {
        TIMEOUT_RESET_FLAG_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Sticky flag that detects that a clock detector reset occurred"]
    #[inline(always)]
    pub fn clk_det_reset_flag(&self) -> CLK_DET_RESET_FLAG_R {
        CLK_DET_RESET_FLAG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Sticky flag that detects that a VDDA reset occurred (triggered by vdda_ready = 0)"]
    #[inline(always)]
    pub fn vdda_reset_flag(&self) -> VDDA_RESET_FLAG_R {
        VDDA_RESET_FLAG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Sticky flag that detects that a VDDM reset occurred (triggered by vddm_ready = 0)"]
    #[inline(always)]
    pub fn vddm_reset_flag(&self) -> VDDM_RESET_FLAG_R {
        VDDM_RESET_FLAG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Sticky flag that detects that a VDDC reset occurred (triggered by vddc_ready = 0)"]
    #[inline(always)]
    pub fn vddc_reset_flag(&self) -> VDDC_RESET_FLAG_R {
        VDDC_RESET_FLAG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Sticky flag that detects that a reset occurred due to pad NRESET"]
    #[inline(always)]
    pub fn pad_reset_flag(&self) -> PAD_RESET_FLAG_R {
        PAD_RESET_FLAG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sticky flag that detects that a POR reset occurred"]
    #[inline(always)]
    pub fn por_reset_flag(&self) -> POR_RESET_FLAG_R {
        POR_RESET_FLAG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Reset the sticky TIMEOUT_RESET flag."]
    #[inline(always)]
    pub fn timeout_reset_flag_clear(&mut self) -> TIMEOUT_RESET_FLAG_CLEAR_W {
        TIMEOUT_RESET_FLAG_CLEAR_W { w: self }
    }
    #[doc = "Bit 5 - Reset the sticky CLK_DET_RESET flag."]
    #[inline(always)]
    pub fn clk_det_reset_flag_clear(&mut self) -> CLK_DET_RESET_FLAG_CLEAR_W {
        CLK_DET_RESET_FLAG_CLEAR_W { w: self }
    }
    #[doc = "Bit 4 - Reset the sticky VDDA_RESET flag."]
    #[inline(always)]
    pub fn vdda_reset_flag_clear(&mut self) -> VDDA_RESET_FLAG_CLEAR_W {
        VDDA_RESET_FLAG_CLEAR_W { w: self }
    }
    #[doc = "Bit 3 - Reset the sticky VDDM_RESET flag."]
    #[inline(always)]
    pub fn vddm_reset_flag_clear(&mut self) -> VDDM_RESET_FLAG_CLEAR_W {
        VDDM_RESET_FLAG_CLEAR_W { w: self }
    }
    #[doc = "Bit 2 - Reset the sticky VDDC_RESET flag."]
    #[inline(always)]
    pub fn vddc_reset_flag_clear(&mut self) -> VDDC_RESET_FLAG_CLEAR_W {
        VDDC_RESET_FLAG_CLEAR_W { w: self }
    }
    #[doc = "Bit 1 - Reset the sticky PAD_RESET flag."]
    #[inline(always)]
    pub fn pad_reset_flag_clear(&mut self) -> PAD_RESET_FLAG_CLEAR_W {
        PAD_RESET_FLAG_CLEAR_W { w: self }
    }
    #[doc = "Bit 0 - Reset the sticky POR_RESET flag."]
    #[inline(always)]
    pub fn por_reset_flag_clear(&mut self) -> POR_RESET_FLAG_CLEAR_W {
        POR_RESET_FLAG_CLEAR_W { w: self }
    }
}
