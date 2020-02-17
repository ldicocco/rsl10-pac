#[doc = "Reader of register BB_RWBBCNTL"]
pub type R = crate::R<u32, super::BB_RWBBCNTL>;
#[doc = "Writer for register BB_RWBBCNTL"]
pub type W = crate::W<u32, super::BB_RWBBCNTL>;
#[doc = "Register BB_RWBBCNTL `reset()`'s with value 0"]
impl crate::ResetValue for super::BB_RWBBCNTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reset the complete system except registers and timing generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTER_SOFT_RST_A {
    #[doc = "0: No action happens if it is written with 0"]
    MASTER_SOFT_RST_0 = 0,
    #[doc = "1: Resets the complete system at 0"]
    MASTER_SOFT_RST_1 = 1,
}
impl From<MASTER_SOFT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_SOFT_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MASTER_SOFT_RST`"]
pub type MASTER_SOFT_RST_R = crate::R<bool, MASTER_SOFT_RST_A>;
impl MASTER_SOFT_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTER_SOFT_RST_A {
        match self.bits {
            false => MASTER_SOFT_RST_A::MASTER_SOFT_RST_0,
            true => MASTER_SOFT_RST_A::MASTER_SOFT_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASTER_SOFT_RST_0`"]
    #[inline(always)]
    pub fn is_master_soft_rst_0(&self) -> bool {
        *self == MASTER_SOFT_RST_A::MASTER_SOFT_RST_0
    }
    #[doc = "Checks if the value of the field is `MASTER_SOFT_RST_1`"]
    #[inline(always)]
    pub fn is_master_soft_rst_1(&self) -> bool {
        *self == MASTER_SOFT_RST_A::MASTER_SOFT_RST_1
    }
}
#[doc = "Write proxy for field `MASTER_SOFT_RST`"]
pub struct MASTER_SOFT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_SOFT_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASTER_SOFT_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action happens if it is written with 0"]
    #[inline(always)]
    pub fn master_soft_rst_0(self) -> &'a mut W {
        self.variant(MASTER_SOFT_RST_A::MASTER_SOFT_RST_0)
    }
    #[doc = "Resets the complete system at 0"]
    #[inline(always)]
    pub fn master_soft_rst_1(self) -> &'a mut W {
        self.variant(MASTER_SOFT_RST_A::MASTER_SOFT_RST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reset the timing generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTER_TGSOFT_RST_A {
    #[doc = "0: No action happens if it is written with 0"]
    MASTER_TGSOFT_RST_0 = 0,
    #[doc = "1: Resets the timing generator at 0"]
    MASTER_TGSOFT_RST_1 = 1,
}
impl From<MASTER_TGSOFT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_TGSOFT_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MASTER_TGSOFT_RST`"]
pub type MASTER_TGSOFT_RST_R = crate::R<bool, MASTER_TGSOFT_RST_A>;
impl MASTER_TGSOFT_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTER_TGSOFT_RST_A {
        match self.bits {
            false => MASTER_TGSOFT_RST_A::MASTER_TGSOFT_RST_0,
            true => MASTER_TGSOFT_RST_A::MASTER_TGSOFT_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASTER_TGSOFT_RST_0`"]
    #[inline(always)]
    pub fn is_master_tgsoft_rst_0(&self) -> bool {
        *self == MASTER_TGSOFT_RST_A::MASTER_TGSOFT_RST_0
    }
    #[doc = "Checks if the value of the field is `MASTER_TGSOFT_RST_1`"]
    #[inline(always)]
    pub fn is_master_tgsoft_rst_1(&self) -> bool {
        *self == MASTER_TGSOFT_RST_A::MASTER_TGSOFT_RST_1
    }
}
#[doc = "Write proxy for field `MASTER_TGSOFT_RST`"]
pub struct MASTER_TGSOFT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_TGSOFT_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASTER_TGSOFT_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action happens if it is written with 0"]
    #[inline(always)]
    pub fn master_tgsoft_rst_0(self) -> &'a mut W {
        self.variant(MASTER_TGSOFT_RST_A::MASTER_TGSOFT_RST_0)
    }
    #[doc = "Resets the timing generator at 0"]
    #[inline(always)]
    pub fn master_tgsoft_rst_1(self) -> &'a mut W {
        self.variant(MASTER_TGSOFT_RST_A::MASTER_TGSOFT_RST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reset the complete register block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_SOFT_RST_A {
    #[doc = "0: No action happens if it is written with 0"]
    REG_SOFT_RST_0 = 0,
    #[doc = "1: Resets the complete register block at 0"]
    REG_SOFT_RST_1 = 1,
}
impl From<REG_SOFT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: REG_SOFT_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REG_SOFT_RST`"]
pub type REG_SOFT_RST_R = crate::R<bool, REG_SOFT_RST_A>;
impl REG_SOFT_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_SOFT_RST_A {
        match self.bits {
            false => REG_SOFT_RST_A::REG_SOFT_RST_0,
            true => REG_SOFT_RST_A::REG_SOFT_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `REG_SOFT_RST_0`"]
    #[inline(always)]
    pub fn is_reg_soft_rst_0(&self) -> bool {
        *self == REG_SOFT_RST_A::REG_SOFT_RST_0
    }
    #[doc = "Checks if the value of the field is `REG_SOFT_RST_1`"]
    #[inline(always)]
    pub fn is_reg_soft_rst_1(&self) -> bool {
        *self == REG_SOFT_RST_A::REG_SOFT_RST_1
    }
}
#[doc = "Write proxy for field `REG_SOFT_RST`"]
pub struct REG_SOFT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_SOFT_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_SOFT_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action happens if it is written with 0"]
    #[inline(always)]
    pub fn reg_soft_rst_0(self) -> &'a mut W {
        self.variant(REG_SOFT_RST_A::REG_SOFT_RST_0)
    }
    #[doc = "Resets the complete register block at 0"]
    #[inline(always)]
    pub fn reg_soft_rst_1(self) -> &'a mut W {
        self.variant(REG_SOFT_RST_A::REG_SOFT_RST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Forces the generation of ble_sw_irq\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWINT_REQ_A {
    #[doc = "0: No action happens if it is written with 0"]
    SWINT_REQ_0 = 0,
    #[doc = "1: When written with a 1 and proper masking is set, resets at 0"]
    SWINT_REQ_1 = 1,
}
impl From<SWINT_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: SWINT_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWINT_REQ`"]
pub type SWINT_REQ_R = crate::R<bool, SWINT_REQ_A>;
impl SWINT_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWINT_REQ_A {
        match self.bits {
            false => SWINT_REQ_A::SWINT_REQ_0,
            true => SWINT_REQ_A::SWINT_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWINT_REQ_0`"]
    #[inline(always)]
    pub fn is_swint_req_0(&self) -> bool {
        *self == SWINT_REQ_A::SWINT_REQ_0
    }
    #[doc = "Checks if the value of the field is `SWINT_REQ_1`"]
    #[inline(always)]
    pub fn is_swint_req_1(&self) -> bool {
        *self == SWINT_REQ_A::SWINT_REQ_1
    }
}
#[doc = "Write proxy for field `SWINT_REQ`"]
pub struct SWINT_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SWINT_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWINT_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action happens if it is written with 0"]
    #[inline(always)]
    pub fn swint_req_0(self) -> &'a mut W {
        self.variant(SWINT_REQ_A::SWINT_REQ_0)
    }
    #[doc = "When written with a 1 and proper masking is set, resets at 0"]
    #[inline(always)]
    pub fn swint_req_1(self) -> &'a mut W {
        self.variant(SWINT_REQ_A::SWINT_REQ_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Abort the current RF testing defined as per CS-FORMAT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFTEST_ABORT_A {
    #[doc = "0: No action happens if it is written with 0"]
    RFTEST_ABORT_0 = 0,
    #[doc = "1: Abort the current RF testing"]
    RFTEST_ABORT_1 = 1,
}
impl From<RFTEST_ABORT_A> for bool {
    #[inline(always)]
    fn from(variant: RFTEST_ABORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RFTEST_ABORT`"]
pub type RFTEST_ABORT_R = crate::R<bool, RFTEST_ABORT_A>;
impl RFTEST_ABORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFTEST_ABORT_A {
        match self.bits {
            false => RFTEST_ABORT_A::RFTEST_ABORT_0,
            true => RFTEST_ABORT_A::RFTEST_ABORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `RFTEST_ABORT_0`"]
    #[inline(always)]
    pub fn is_rftest_abort_0(&self) -> bool {
        *self == RFTEST_ABORT_A::RFTEST_ABORT_0
    }
    #[doc = "Checks if the value of the field is `RFTEST_ABORT_1`"]
    #[inline(always)]
    pub fn is_rftest_abort_1(&self) -> bool {
        *self == RFTEST_ABORT_A::RFTEST_ABORT_1
    }
}
#[doc = "Write proxy for field `RFTEST_ABORT`"]
pub struct RFTEST_ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> RFTEST_ABORT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFTEST_ABORT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action happens if it is written with 0"]
    #[inline(always)]
    pub fn rftest_abort_0(self) -> &'a mut W {
        self.variant(RFTEST_ABORT_A::RFTEST_ABORT_0)
    }
    #[doc = "Abort the current RF testing"]
    #[inline(always)]
    pub fn rftest_abort_1(self) -> &'a mut W {
        self.variant(RFTEST_ABORT_A::RFTEST_ABORT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Abort the current scan window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADVERT_ABORT_A {
    #[doc = "0: No action happens if it is written with 0"]
    ADVERT_ABORT_0 = 0,
    #[doc = "1: Abort the current scan window"]
    ADVERT_ABORT_1 = 1,
}
impl From<ADVERT_ABORT_A> for bool {
    #[inline(always)]
    fn from(variant: ADVERT_ABORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADVERT_ABORT`"]
pub type ADVERT_ABORT_R = crate::R<bool, ADVERT_ABORT_A>;
impl ADVERT_ABORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADVERT_ABORT_A {
        match self.bits {
            false => ADVERT_ABORT_A::ADVERT_ABORT_0,
            true => ADVERT_ABORT_A::ADVERT_ABORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADVERT_ABORT_0`"]
    #[inline(always)]
    pub fn is_advert_abort_0(&self) -> bool {
        *self == ADVERT_ABORT_A::ADVERT_ABORT_0
    }
    #[doc = "Checks if the value of the field is `ADVERT_ABORT_1`"]
    #[inline(always)]
    pub fn is_advert_abort_1(&self) -> bool {
        *self == ADVERT_ABORT_A::ADVERT_ABORT_1
    }
}
#[doc = "Write proxy for field `ADVERT_ABORT`"]
pub struct ADVERT_ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVERT_ABORT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADVERT_ABORT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action happens if it is written with 0"]
    #[inline(always)]
    pub fn advert_abort_0(self) -> &'a mut W {
        self.variant(ADVERT_ABORT_A::ADVERT_ABORT_0)
    }
    #[doc = "Abort the current scan window"]
    #[inline(always)]
    pub fn advert_abort_1(self) -> &'a mut W {
        self.variant(ADVERT_ABORT_A::ADVERT_ABORT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Abort the current advertising event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCAN_ABORT_A {
    #[doc = "0: No action happens if it is written with 0"]
    SCAN_ABORT_0 = 0,
    #[doc = "1: Abort the current advertising event"]
    SCAN_ABORT_1 = 1,
}
impl From<SCAN_ABORT_A> for bool {
    #[inline(always)]
    fn from(variant: SCAN_ABORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCAN_ABORT`"]
pub type SCAN_ABORT_R = crate::R<bool, SCAN_ABORT_A>;
impl SCAN_ABORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCAN_ABORT_A {
        match self.bits {
            false => SCAN_ABORT_A::SCAN_ABORT_0,
            true => SCAN_ABORT_A::SCAN_ABORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SCAN_ABORT_0`"]
    #[inline(always)]
    pub fn is_scan_abort_0(&self) -> bool {
        *self == SCAN_ABORT_A::SCAN_ABORT_0
    }
    #[doc = "Checks if the value of the field is `SCAN_ABORT_1`"]
    #[inline(always)]
    pub fn is_scan_abort_1(&self) -> bool {
        *self == SCAN_ABORT_A::SCAN_ABORT_1
    }
}
#[doc = "Write proxy for field `SCAN_ABORT`"]
pub struct SCAN_ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_ABORT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCAN_ABORT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action happens if it is written with 0"]
    #[inline(always)]
    pub fn scan_abort_0(self) -> &'a mut W {
        self.variant(SCAN_ABORT_A::SCAN_ABORT_0)
    }
    #[doc = "Abort the current advertising event"]
    #[inline(always)]
    pub fn scan_abort_1(self) -> &'a mut W {
        self.variant(SCAN_ABORT_A::SCAN_ABORT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Allow a single Tx/Rx exchange whatever the MD bits are\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MD_DSB_A {
    #[doc = "0: Normal operation of MD bits management"]
    MD_DSB_0 = 0,
    #[doc = "1: Allow a single Tx/Rx exchange whatever the MD bits are."]
    MD_DSB_1 = 1,
}
impl From<MD_DSB_A> for bool {
    #[inline(always)]
    fn from(variant: MD_DSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MD_DSB`"]
pub type MD_DSB_R = crate::R<bool, MD_DSB_A>;
impl MD_DSB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MD_DSB_A {
        match self.bits {
            false => MD_DSB_A::MD_DSB_0,
            true => MD_DSB_A::MD_DSB_1,
        }
    }
    #[doc = "Checks if the value of the field is `MD_DSB_0`"]
    #[inline(always)]
    pub fn is_md_dsb_0(&self) -> bool {
        *self == MD_DSB_A::MD_DSB_0
    }
    #[doc = "Checks if the value of the field is `MD_DSB_1`"]
    #[inline(always)]
    pub fn is_md_dsb_1(&self) -> bool {
        *self == MD_DSB_A::MD_DSB_1
    }
}
#[doc = "Write proxy for field `MD_DSB`"]
pub struct MD_DSB_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_DSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MD_DSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation of MD bits management"]
    #[inline(always)]
    pub fn md_dsb_0(self) -> &'a mut W {
        self.variant(MD_DSB_A::MD_DSB_0)
    }
    #[doc = "Allow a single Tx/Rx exchange whatever the MD bits are."]
    #[inline(always)]
    pub fn md_dsb_1(self) -> &'a mut W {
        self.variant(MD_DSB_A::MD_DSB_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Disable sequence number management\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SN_DSB_A {
    #[doc = "0: Normal operation of sequence number"]
    SN_DSB_0 = 0,
    #[doc = "1: Sequence number management disabled"]
    SN_DSB_1 = 1,
}
impl From<SN_DSB_A> for bool {
    #[inline(always)]
    fn from(variant: SN_DSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SN_DSB`"]
pub type SN_DSB_R = crate::R<bool, SN_DSB_A>;
impl SN_DSB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SN_DSB_A {
        match self.bits {
            false => SN_DSB_A::SN_DSB_0,
            true => SN_DSB_A::SN_DSB_1,
        }
    }
    #[doc = "Checks if the value of the field is `SN_DSB_0`"]
    #[inline(always)]
    pub fn is_sn_dsb_0(&self) -> bool {
        *self == SN_DSB_A::SN_DSB_0
    }
    #[doc = "Checks if the value of the field is `SN_DSB_1`"]
    #[inline(always)]
    pub fn is_sn_dsb_1(&self) -> bool {
        *self == SN_DSB_A::SN_DSB_1
    }
}
#[doc = "Write proxy for field `SN_DSB`"]
pub struct SN_DSB_W<'a> {
    w: &'a mut W,
}
impl<'a> SN_DSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SN_DSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation of sequence number"]
    #[inline(always)]
    pub fn sn_dsb_0(self) -> &'a mut W {
        self.variant(SN_DSB_A::SN_DSB_0)
    }
    #[doc = "Sequence number management disabled"]
    #[inline(always)]
    pub fn sn_dsb_1(self) -> &'a mut W {
        self.variant(SN_DSB_A::SN_DSB_1)
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
#[doc = "Disable acknowledge scheme\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NESN_DSB_A {
    #[doc = "0: Normal operation of acknowledge"]
    NESN_DSB_0 = 0,
    #[doc = "1: Acknowledge scheme disabled"]
    NESN_DSB_1 = 1,
}
impl From<NESN_DSB_A> for bool {
    #[inline(always)]
    fn from(variant: NESN_DSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NESN_DSB`"]
pub type NESN_DSB_R = crate::R<bool, NESN_DSB_A>;
impl NESN_DSB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NESN_DSB_A {
        match self.bits {
            false => NESN_DSB_A::NESN_DSB_0,
            true => NESN_DSB_A::NESN_DSB_1,
        }
    }
    #[doc = "Checks if the value of the field is `NESN_DSB_0`"]
    #[inline(always)]
    pub fn is_nesn_dsb_0(&self) -> bool {
        *self == NESN_DSB_A::NESN_DSB_0
    }
    #[doc = "Checks if the value of the field is `NESN_DSB_1`"]
    #[inline(always)]
    pub fn is_nesn_dsb_1(&self) -> bool {
        *self == NESN_DSB_A::NESN_DSB_1
    }
}
#[doc = "Write proxy for field `NESN_DSB`"]
pub struct NESN_DSB_W<'a> {
    w: &'a mut W,
}
impl<'a> NESN_DSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NESN_DSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation of acknowledge"]
    #[inline(always)]
    pub fn nesn_dsb_0(self) -> &'a mut W {
        self.variant(NESN_DSB_A::NESN_DSB_0)
    }
    #[doc = "Acknowledge scheme disabled"]
    #[inline(always)]
    pub fn nesn_dsb_1(self) -> &'a mut W {
        self.variant(NESN_DSB_A::NESN_DSB_1)
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
#[doc = "Disable encryption / decryption\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYPT_DSB_A {
    #[doc = "0: Normal operation (encryption / decryption enabled)"]
    CRYPT_DSB_0 = 0,
    #[doc = "1: Encryption / decryption disabled"]
    CRYPT_DSB_1 = 1,
}
impl From<CRYPT_DSB_A> for bool {
    #[inline(always)]
    fn from(variant: CRYPT_DSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRYPT_DSB`"]
pub type CRYPT_DSB_R = crate::R<bool, CRYPT_DSB_A>;
impl CRYPT_DSB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRYPT_DSB_A {
        match self.bits {
            false => CRYPT_DSB_A::CRYPT_DSB_0,
            true => CRYPT_DSB_A::CRYPT_DSB_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRYPT_DSB_0`"]
    #[inline(always)]
    pub fn is_crypt_dsb_0(&self) -> bool {
        *self == CRYPT_DSB_A::CRYPT_DSB_0
    }
    #[doc = "Checks if the value of the field is `CRYPT_DSB_1`"]
    #[inline(always)]
    pub fn is_crypt_dsb_1(&self) -> bool {
        *self == CRYPT_DSB_A::CRYPT_DSB_1
    }
}
#[doc = "Write proxy for field `CRYPT_DSB`"]
pub struct CRYPT_DSB_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPT_DSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYPT_DSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation (encryption / decryption enabled)"]
    #[inline(always)]
    pub fn crypt_dsb_0(self) -> &'a mut W {
        self.variant(CRYPT_DSB_A::CRYPT_DSB_0)
    }
    #[doc = "Encryption / decryption disabled"]
    #[inline(always)]
    pub fn crypt_dsb_1(self) -> &'a mut W {
        self.variant(CRYPT_DSB_A::CRYPT_DSB_1)
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
#[doc = "Disable whitening\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WHIT_DSB_A {
    #[doc = "0: Normal operation (whitening enabled)"]
    WHIT_DSB_0 = 0,
    #[doc = "1: Whitening disabled"]
    WHIT_DSB_1 = 1,
}
impl From<WHIT_DSB_A> for bool {
    #[inline(always)]
    fn from(variant: WHIT_DSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WHIT_DSB`"]
pub type WHIT_DSB_R = crate::R<bool, WHIT_DSB_A>;
impl WHIT_DSB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WHIT_DSB_A {
        match self.bits {
            false => WHIT_DSB_A::WHIT_DSB_0,
            true => WHIT_DSB_A::WHIT_DSB_1,
        }
    }
    #[doc = "Checks if the value of the field is `WHIT_DSB_0`"]
    #[inline(always)]
    pub fn is_whit_dsb_0(&self) -> bool {
        *self == WHIT_DSB_A::WHIT_DSB_0
    }
    #[doc = "Checks if the value of the field is `WHIT_DSB_1`"]
    #[inline(always)]
    pub fn is_whit_dsb_1(&self) -> bool {
        *self == WHIT_DSB_A::WHIT_DSB_1
    }
}
#[doc = "Write proxy for field `WHIT_DSB`"]
pub struct WHIT_DSB_W<'a> {
    w: &'a mut W,
}
impl<'a> WHIT_DSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WHIT_DSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation (whitening enabled)"]
    #[inline(always)]
    pub fn whit_dsb_0(self) -> &'a mut W {
        self.variant(WHIT_DSB_A::WHIT_DSB_0)
    }
    #[doc = "Whitening disabled"]
    #[inline(always)]
    pub fn whit_dsb_1(self) -> &'a mut W {
        self.variant(WHIT_DSB_A::WHIT_DSB_1)
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
#[doc = "Disable CRC stripping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_DSB_A {
    #[doc = "0: Normal operation (CRC removed from data stream)"]
    CRC_DSB_0 = 0,
    #[doc = "1: CRC stripping disabled on Rx packets, CRC replaced by 0x000 in Tx"]
    CRC_DSB_1 = 1,
}
impl From<CRC_DSB_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_DSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRC_DSB`"]
pub type CRC_DSB_R = crate::R<bool, CRC_DSB_A>;
impl CRC_DSB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_DSB_A {
        match self.bits {
            false => CRC_DSB_A::CRC_DSB_0,
            true => CRC_DSB_A::CRC_DSB_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_DSB_0`"]
    #[inline(always)]
    pub fn is_crc_dsb_0(&self) -> bool {
        *self == CRC_DSB_A::CRC_DSB_0
    }
    #[doc = "Checks if the value of the field is `CRC_DSB_1`"]
    #[inline(always)]
    pub fn is_crc_dsb_1(&self) -> bool {
        *self == CRC_DSB_A::CRC_DSB_1
    }
}
#[doc = "Write proxy for field `CRC_DSB`"]
pub struct CRC_DSB_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_DSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_DSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation (CRC removed from data stream)"]
    #[inline(always)]
    pub fn crc_dsb_0(self) -> &'a mut W {
        self.variant(CRC_DSB_A::CRC_DSB_0)
    }
    #[doc = "CRC stripping disabled on Rx packets, CRC replaced by 0x000 in Tx"]
    #[inline(always)]
    pub fn crc_dsb_1(self) -> &'a mut W {
        self.variant(CRC_DSB_A::CRC_DSB_1)
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
#[doc = "Disable frequency hopping remapping algorithm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOP_REMAP_DSB_A {
    #[doc = "0: Normal operation (requency hopping remapping algorithm enabled)"]
    HOP_REMAP_DSB_0 = 0,
    #[doc = "1: Frequency hopping remapping algorithm disabled"]
    HOP_REMAP_DSB_1 = 1,
}
impl From<HOP_REMAP_DSB_A> for bool {
    #[inline(always)]
    fn from(variant: HOP_REMAP_DSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HOP_REMAP_DSB`"]
pub type HOP_REMAP_DSB_R = crate::R<bool, HOP_REMAP_DSB_A>;
impl HOP_REMAP_DSB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOP_REMAP_DSB_A {
        match self.bits {
            false => HOP_REMAP_DSB_A::HOP_REMAP_DSB_0,
            true => HOP_REMAP_DSB_A::HOP_REMAP_DSB_1,
        }
    }
    #[doc = "Checks if the value of the field is `HOP_REMAP_DSB_0`"]
    #[inline(always)]
    pub fn is_hop_remap_dsb_0(&self) -> bool {
        *self == HOP_REMAP_DSB_A::HOP_REMAP_DSB_0
    }
    #[doc = "Checks if the value of the field is `HOP_REMAP_DSB_1`"]
    #[inline(always)]
    pub fn is_hop_remap_dsb_1(&self) -> bool {
        *self == HOP_REMAP_DSB_A::HOP_REMAP_DSB_1
    }
}
#[doc = "Write proxy for field `HOP_REMAP_DSB`"]
pub struct HOP_REMAP_DSB_W<'a> {
    w: &'a mut W,
}
impl<'a> HOP_REMAP_DSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HOP_REMAP_DSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation (requency hopping remapping algorithm enabled)"]
    #[inline(always)]
    pub fn hop_remap_dsb_0(self) -> &'a mut W {
        self.variant(HOP_REMAP_DSB_A::HOP_REMAP_DSB_0)
    }
    #[doc = "Frequency hopping remapping algorithm disabled"]
    #[inline(always)]
    pub fn hop_remap_dsb_1(self) -> &'a mut W {
        self.variant(HOP_REMAP_DSB_A::HOP_REMAP_DSB_1)
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
#[doc = "Advertising channels error filtering enable control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADVERTFILT_EN_A {
    #[doc = "0: RW-BLE core reports all errors to RW-BLE software"]
    ADVERTFILT_EN_0 = 0,
    #[doc = "1: RW-BLE core reports only correctly received packet, without error to RW-BLE software"]
    ADVERTFILT_EN_1 = 1,
}
impl From<ADVERTFILT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADVERTFILT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADVERTFILT_EN`"]
pub type ADVERTFILT_EN_R = crate::R<bool, ADVERTFILT_EN_A>;
impl ADVERTFILT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADVERTFILT_EN_A {
        match self.bits {
            false => ADVERTFILT_EN_A::ADVERTFILT_EN_0,
            true => ADVERTFILT_EN_A::ADVERTFILT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADVERTFILT_EN_0`"]
    #[inline(always)]
    pub fn is_advertfilt_en_0(&self) -> bool {
        *self == ADVERTFILT_EN_A::ADVERTFILT_EN_0
    }
    #[doc = "Checks if the value of the field is `ADVERTFILT_EN_1`"]
    #[inline(always)]
    pub fn is_advertfilt_en_1(&self) -> bool {
        *self == ADVERTFILT_EN_A::ADVERTFILT_EN_1
    }
}
#[doc = "Write proxy for field `ADVERTFILT_EN`"]
pub struct ADVERTFILT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVERTFILT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADVERTFILT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RW-BLE core reports all errors to RW-BLE software"]
    #[inline(always)]
    pub fn advertfilt_en_0(self) -> &'a mut W {
        self.variant(ADVERTFILT_EN_A::ADVERTFILT_EN_0)
    }
    #[doc = "RW-BLE core reports only correctly received packet, without error to RW-BLE software"]
    #[inline(always)]
    pub fn advertfilt_en_1(self) -> &'a mut W {
        self.variant(ADVERTFILT_EN_A::ADVERTFILT_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Enable RW-BLE core exchange table pre-fetch mechanism\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWBLE_EN_A {
    #[doc = "0: Disable RW-BLE core exchange table pre-fetch mechanism"]
    RWBLE_EN_0 = 0,
    #[doc = "1: Enable RW-BLE core exchange table pre-fetch mechanism"]
    RWBLE_EN_1 = 1,
}
impl From<RWBLE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RWBLE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RWBLE_EN`"]
pub type RWBLE_EN_R = crate::R<bool, RWBLE_EN_A>;
impl RWBLE_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWBLE_EN_A {
        match self.bits {
            false => RWBLE_EN_A::RWBLE_EN_0,
            true => RWBLE_EN_A::RWBLE_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RWBLE_EN_0`"]
    #[inline(always)]
    pub fn is_rwble_en_0(&self) -> bool {
        *self == RWBLE_EN_A::RWBLE_EN_0
    }
    #[doc = "Checks if the value of the field is `RWBLE_EN_1`"]
    #[inline(always)]
    pub fn is_rwble_en_1(&self) -> bool {
        *self == RWBLE_EN_A::RWBLE_EN_1
    }
}
#[doc = "Write proxy for field `RWBLE_EN`"]
pub struct RWBLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RWBLE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWBLE_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable RW-BLE core exchange table pre-fetch mechanism"]
    #[inline(always)]
    pub fn rwble_en_0(self) -> &'a mut W {
        self.variant(RWBLE_EN_A::RWBLE_EN_0)
    }
    #[doc = "Enable RW-BLE core exchange table pre-fetch mechanism"]
    #[inline(always)]
    pub fn rwble_en_1(self) -> &'a mut W {
        self.variant(RWBLE_EN_A::RWBLE_EN_1)
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
#[doc = "Default Rx Window size in us (used when device is master connected or performs its second receipt)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXWINSZDEF_A {
    #[doc = "0: `0`"]
    RXWINSZDEF_0 = 0,
}
impl From<RXWINSZDEF_A> for u8 {
    #[inline(always)]
    fn from(variant: RXWINSZDEF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXWINSZDEF`"]
pub type RXWINSZDEF_R = crate::R<u8, RXWINSZDEF_A>;
impl RXWINSZDEF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXWINSZDEF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXWINSZDEF_A::RXWINSZDEF_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RXWINSZDEF_0`"]
    #[inline(always)]
    pub fn is_rxwinszdef_0(&self) -> bool {
        *self == RXWINSZDEF_A::RXWINSZDEF_0
    }
}
#[doc = "Write proxy for field `RXWINSZDEF`"]
pub struct RXWINSZDEF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXWINSZDEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXWINSZDEF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rxwinszdef_0(self) -> &'a mut W {
        self.variant(RXWINSZDEF_A::RXWINSZDEF_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Indicates the maximum number of errors allowed to recognize the synchronization word\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCERR_A {
    #[doc = "0: `0`"]
    SYNCERR_0 = 0,
}
impl From<SYNCERR_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCERR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNCERR`"]
pub type SYNCERR_R = crate::R<u8, SYNCERR_A>;
impl SYNCERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYNCERR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYNCERR_A::SYNCERR_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYNCERR_0`"]
    #[inline(always)]
    pub fn is_syncerr_0(&self) -> bool {
        *self == SYNCERR_A::SYNCERR_0
    }
}
#[doc = "Write proxy for field `SYNCERR`"]
pub struct SYNCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCERR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn syncerr_0(self) -> &'a mut W {
        self.variant(SYNCERR_A::SYNCERR_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Reset the complete system except registers and timing generator"]
    #[inline(always)]
    pub fn master_soft_rst(&self) -> MASTER_SOFT_RST_R {
        MASTER_SOFT_RST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Reset the timing generator"]
    #[inline(always)]
    pub fn master_tgsoft_rst(&self) -> MASTER_TGSOFT_RST_R {
        MASTER_TGSOFT_RST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Reset the complete register block"]
    #[inline(always)]
    pub fn reg_soft_rst(&self) -> REG_SOFT_RST_R {
        REG_SOFT_RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Forces the generation of ble_sw_irq"]
    #[inline(always)]
    pub fn swint_req(&self) -> SWINT_REQ_R {
        SWINT_REQ_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Abort the current RF testing defined as per CS-FORMAT"]
    #[inline(always)]
    pub fn rftest_abort(&self) -> RFTEST_ABORT_R {
        RFTEST_ABORT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Abort the current scan window"]
    #[inline(always)]
    pub fn advert_abort(&self) -> ADVERT_ABORT_R {
        ADVERT_ABORT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Abort the current advertising event"]
    #[inline(always)]
    pub fn scan_abort(&self) -> SCAN_ABORT_R {
        SCAN_ABORT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Allow a single Tx/Rx exchange whatever the MD bits are"]
    #[inline(always)]
    pub fn md_dsb(&self) -> MD_DSB_R {
        MD_DSB_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Disable sequence number management"]
    #[inline(always)]
    pub fn sn_dsb(&self) -> SN_DSB_R {
        SN_DSB_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Disable acknowledge scheme"]
    #[inline(always)]
    pub fn nesn_dsb(&self) -> NESN_DSB_R {
        NESN_DSB_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Disable encryption / decryption"]
    #[inline(always)]
    pub fn crypt_dsb(&self) -> CRYPT_DSB_R {
        CRYPT_DSB_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Disable whitening"]
    #[inline(always)]
    pub fn whit_dsb(&self) -> WHIT_DSB_R {
        WHIT_DSB_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Disable CRC stripping"]
    #[inline(always)]
    pub fn crc_dsb(&self) -> CRC_DSB_R {
        CRC_DSB_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Disable frequency hopping remapping algorithm"]
    #[inline(always)]
    pub fn hop_remap_dsb(&self) -> HOP_REMAP_DSB_R {
        HOP_REMAP_DSB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Advertising channels error filtering enable control"]
    #[inline(always)]
    pub fn advertfilt_en(&self) -> ADVERTFILT_EN_R {
        ADVERTFILT_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable RW-BLE core exchange table pre-fetch mechanism"]
    #[inline(always)]
    pub fn rwble_en(&self) -> RWBLE_EN_R {
        RWBLE_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Default Rx Window size in us (used when device is master connected or performs its second receipt)"]
    #[inline(always)]
    pub fn rxwinszdef(&self) -> RXWINSZDEF_R {
        RXWINSZDEF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:2 - Indicates the maximum number of errors allowed to recognize the synchronization word"]
    #[inline(always)]
    pub fn syncerr(&self) -> SYNCERR_R {
        SYNCERR_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Reset the complete system except registers and timing generator"]
    #[inline(always)]
    pub fn master_soft_rst(&mut self) -> MASTER_SOFT_RST_W {
        MASTER_SOFT_RST_W { w: self }
    }
    #[doc = "Bit 30 - Reset the timing generator"]
    #[inline(always)]
    pub fn master_tgsoft_rst(&mut self) -> MASTER_TGSOFT_RST_W {
        MASTER_TGSOFT_RST_W { w: self }
    }
    #[doc = "Bit 29 - Reset the complete register block"]
    #[inline(always)]
    pub fn reg_soft_rst(&mut self) -> REG_SOFT_RST_W {
        REG_SOFT_RST_W { w: self }
    }
    #[doc = "Bit 28 - Forces the generation of ble_sw_irq"]
    #[inline(always)]
    pub fn swint_req(&mut self) -> SWINT_REQ_W {
        SWINT_REQ_W { w: self }
    }
    #[doc = "Bit 26 - Abort the current RF testing defined as per CS-FORMAT"]
    #[inline(always)]
    pub fn rftest_abort(&mut self) -> RFTEST_ABORT_W {
        RFTEST_ABORT_W { w: self }
    }
    #[doc = "Bit 25 - Abort the current scan window"]
    #[inline(always)]
    pub fn advert_abort(&mut self) -> ADVERT_ABORT_W {
        ADVERT_ABORT_W { w: self }
    }
    #[doc = "Bit 24 - Abort the current advertising event"]
    #[inline(always)]
    pub fn scan_abort(&mut self) -> SCAN_ABORT_W {
        SCAN_ABORT_W { w: self }
    }
    #[doc = "Bit 22 - Allow a single Tx/Rx exchange whatever the MD bits are"]
    #[inline(always)]
    pub fn md_dsb(&mut self) -> MD_DSB_W {
        MD_DSB_W { w: self }
    }
    #[doc = "Bit 21 - Disable sequence number management"]
    #[inline(always)]
    pub fn sn_dsb(&mut self) -> SN_DSB_W {
        SN_DSB_W { w: self }
    }
    #[doc = "Bit 20 - Disable acknowledge scheme"]
    #[inline(always)]
    pub fn nesn_dsb(&mut self) -> NESN_DSB_W {
        NESN_DSB_W { w: self }
    }
    #[doc = "Bit 19 - Disable encryption / decryption"]
    #[inline(always)]
    pub fn crypt_dsb(&mut self) -> CRYPT_DSB_W {
        CRYPT_DSB_W { w: self }
    }
    #[doc = "Bit 18 - Disable whitening"]
    #[inline(always)]
    pub fn whit_dsb(&mut self) -> WHIT_DSB_W {
        WHIT_DSB_W { w: self }
    }
    #[doc = "Bit 17 - Disable CRC stripping"]
    #[inline(always)]
    pub fn crc_dsb(&mut self) -> CRC_DSB_W {
        CRC_DSB_W { w: self }
    }
    #[doc = "Bit 16 - Disable frequency hopping remapping algorithm"]
    #[inline(always)]
    pub fn hop_remap_dsb(&mut self) -> HOP_REMAP_DSB_W {
        HOP_REMAP_DSB_W { w: self }
    }
    #[doc = "Bit 9 - Advertising channels error filtering enable control"]
    #[inline(always)]
    pub fn advertfilt_en(&mut self) -> ADVERTFILT_EN_W {
        ADVERTFILT_EN_W { w: self }
    }
    #[doc = "Bit 8 - Enable RW-BLE core exchange table pre-fetch mechanism"]
    #[inline(always)]
    pub fn rwble_en(&mut self) -> RWBLE_EN_W {
        RWBLE_EN_W { w: self }
    }
    #[doc = "Bits 4:7 - Default Rx Window size in us (used when device is master connected or performs its second receipt)"]
    #[inline(always)]
    pub fn rxwinszdef(&mut self) -> RXWINSZDEF_W {
        RXWINSZDEF_W { w: self }
    }
    #[doc = "Bits 0:2 - Indicates the maximum number of errors allowed to recognize the synchronization word"]
    #[inline(always)]
    pub fn syncerr(&mut self) -> SYNCERR_W {
        SYNCERR_W { w: self }
    }
}
