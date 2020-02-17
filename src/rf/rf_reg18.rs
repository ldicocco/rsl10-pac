#[doc = "Reader of register RF_REG18"]
pub type R = crate::R<u32, super::RF_REG18>;
#[doc = "Writer for register RF_REG18"]
pub type W = crate::W<u32, super::RF_REG18>;
#[doc = "Register RF_REG18 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG18 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Unsigned value that specifies the IF for the Rx mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_A {
    #[doc = "0: `0`"]
    CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_DEFAULT = 0,
}
impl From<CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_A> for u16 {
    #[inline(always)]
    fn from(variant: CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CORRECT_CFREQ_IF_CORRECT_CFREQ_IF`"]
pub type CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_R = crate::R<u16, CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_A>;
impl CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => {
                Val(CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_A::CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_DEFAULT)
            }
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_DEFAULT`"]
    #[inline(always)]
    pub fn is_correct_cfreq_if_correct_cfreq_if_default(&self) -> bool {
        *self == CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_A::CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_DEFAULT
    }
}
#[doc = "Write proxy for field `CORRECT_CFREQ_IF_CORRECT_CFREQ_IF`"]
pub struct CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn correct_cfreq_if_correct_cfreq_if_default(self) -> &'a mut W {
        self.variant(CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_A::CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Speed on the RSSI triangular dithering signal (cf reg RSSI_TUN)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSSI_BANK_RSSI_TRI_CK_DIV_A {
    #[doc = "0: `0`"]
    RSSI_BANK_RSSI_TRI_CK_DIV_DEFAULT = 0,
}
impl From<RSSI_BANK_RSSI_TRI_CK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: RSSI_BANK_RSSI_TRI_CK_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSSI_BANK_RSSI_TRI_CK_DIV`"]
pub type RSSI_BANK_RSSI_TRI_CK_DIV_R = crate::R<u8, RSSI_BANK_RSSI_TRI_CK_DIV_A>;
impl RSSI_BANK_RSSI_TRI_CK_DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSSI_BANK_RSSI_TRI_CK_DIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RSSI_BANK_RSSI_TRI_CK_DIV_A::RSSI_BANK_RSSI_TRI_CK_DIV_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_BANK_RSSI_TRI_CK_DIV_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_bank_rssi_tri_ck_div_default(&self) -> bool {
        *self == RSSI_BANK_RSSI_TRI_CK_DIV_A::RSSI_BANK_RSSI_TRI_CK_DIV_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_BANK_RSSI_TRI_CK_DIV`"]
pub struct RSSI_BANK_RSSI_TRI_CK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_BANK_RSSI_TRI_CK_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_BANK_RSSI_TRI_CK_DIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssi_bank_rssi_tri_ck_div_default(self) -> &'a mut W {
        self.variant(RSSI_BANK_RSSI_TRI_CK_DIV_A::RSSI_BANK_RSSI_TRI_CK_DIV_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "If set to 1, the RSSI filtering is 8x faster\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSI_BANK_FAST_RSSI_A {
    #[doc = "0: `0`"]
    RSSI_BANK_FAST_RSSI_DEFAULT = 0,
}
impl From<RSSI_BANK_FAST_RSSI_A> for bool {
    #[inline(always)]
    fn from(variant: RSSI_BANK_FAST_RSSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSSI_BANK_FAST_RSSI`"]
pub type RSSI_BANK_FAST_RSSI_R = crate::R<bool, RSSI_BANK_FAST_RSSI_A>;
impl RSSI_BANK_FAST_RSSI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RSSI_BANK_FAST_RSSI_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(RSSI_BANK_FAST_RSSI_A::RSSI_BANK_FAST_RSSI_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_BANK_FAST_RSSI_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_bank_fast_rssi_default(&self) -> bool {
        *self == RSSI_BANK_FAST_RSSI_A::RSSI_BANK_FAST_RSSI_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_BANK_FAST_RSSI`"]
pub struct RSSI_BANK_FAST_RSSI_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_BANK_FAST_RSSI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_BANK_FAST_RSSI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssi_bank_fast_rssi_default(self) -> &'a mut W {
        self.variant(RSSI_BANK_FAST_RSSI_A::RSSI_BANK_FAST_RSSI_DEFAULT)
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
#[doc = "If the packet mode is set, indicates to switch the fast modes during the preamble reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSI_BANK_EN_FAST_PRE_SYNC_A {
    #[doc = "0: `0`"]
    RSSI_BANK_EN_FAST_PRE_SYNC_DEFAULT = 0,
}
impl From<RSSI_BANK_EN_FAST_PRE_SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: RSSI_BANK_EN_FAST_PRE_SYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSSI_BANK_EN_FAST_PRE_SYNC`"]
pub type RSSI_BANK_EN_FAST_PRE_SYNC_R = crate::R<bool, RSSI_BANK_EN_FAST_PRE_SYNC_A>;
impl RSSI_BANK_EN_FAST_PRE_SYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RSSI_BANK_EN_FAST_PRE_SYNC_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(RSSI_BANK_EN_FAST_PRE_SYNC_A::RSSI_BANK_EN_FAST_PRE_SYNC_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_BANK_EN_FAST_PRE_SYNC_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_bank_en_fast_pre_sync_default(&self) -> bool {
        *self == RSSI_BANK_EN_FAST_PRE_SYNC_A::RSSI_BANK_EN_FAST_PRE_SYNC_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_BANK_EN_FAST_PRE_SYNC`"]
pub struct RSSI_BANK_EN_FAST_PRE_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_BANK_EN_FAST_PRE_SYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_BANK_EN_FAST_PRE_SYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssi_bank_en_fast_pre_sync_default(self) -> &'a mut W {
        self.variant(RSSI_BANK_EN_FAST_PRE_SYNC_A::RSSI_BANK_EN_FAST_PRE_SYNC_DEFAULT)
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
#[doc = "Time constant of the RSSI filtering block: 0: 4symbols, 1: 8symbols, 2: 16 symbols, 3: 32symbols, 4: 64symbols, 5: 128symbols, 6: 256symbols, 7: 512symbols, 8: 1024symbols\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSSI_BANK_TAU_RSSI_FILTERING_A {
    #[doc = "0: `0`"]
    RSSI_BANK_TAU_RSSI_FILTERING_DEFAULT = 0,
}
impl From<RSSI_BANK_TAU_RSSI_FILTERING_A> for u8 {
    #[inline(always)]
    fn from(variant: RSSI_BANK_TAU_RSSI_FILTERING_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSSI_BANK_TAU_RSSI_FILTERING`"]
pub type RSSI_BANK_TAU_RSSI_FILTERING_R = crate::R<u8, RSSI_BANK_TAU_RSSI_FILTERING_A>;
impl RSSI_BANK_TAU_RSSI_FILTERING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSSI_BANK_TAU_RSSI_FILTERING_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RSSI_BANK_TAU_RSSI_FILTERING_A::RSSI_BANK_TAU_RSSI_FILTERING_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RSSI_BANK_TAU_RSSI_FILTERING_DEFAULT`"]
    #[inline(always)]
    pub fn is_rssi_bank_tau_rssi_filtering_default(&self) -> bool {
        *self == RSSI_BANK_TAU_RSSI_FILTERING_A::RSSI_BANK_TAU_RSSI_FILTERING_DEFAULT
    }
}
#[doc = "Write proxy for field `RSSI_BANK_TAU_RSSI_FILTERING`"]
pub struct RSSI_BANK_TAU_RSSI_FILTERING_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_BANK_TAU_RSSI_FILTERING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSI_BANK_TAU_RSSI_FILTERING_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rssi_bank_tau_rssi_filtering_default(self) -> &'a mut W {
        self.variant(RSSI_BANK_TAU_RSSI_FILTERING_A::RSSI_BANK_TAU_RSSI_FILTERING_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "If set to 1 uses the viterbi soft decoding\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECISION_USE_VIT_SOFT_A {
    #[doc = "0: `0`"]
    DECISION_USE_VIT_SOFT_DEFAULT = 0,
}
impl From<DECISION_USE_VIT_SOFT_A> for bool {
    #[inline(always)]
    fn from(variant: DECISION_USE_VIT_SOFT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DECISION_USE_VIT_SOFT`"]
pub type DECISION_USE_VIT_SOFT_R = crate::R<bool, DECISION_USE_VIT_SOFT_A>;
impl DECISION_USE_VIT_SOFT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DECISION_USE_VIT_SOFT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DECISION_USE_VIT_SOFT_A::DECISION_USE_VIT_SOFT_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DECISION_USE_VIT_SOFT_DEFAULT`"]
    #[inline(always)]
    pub fn is_decision_use_vit_soft_default(&self) -> bool {
        *self == DECISION_USE_VIT_SOFT_A::DECISION_USE_VIT_SOFT_DEFAULT
    }
}
#[doc = "Write proxy for field `DECISION_USE_VIT_SOFT`"]
pub struct DECISION_USE_VIT_SOFT_W<'a> {
    w: &'a mut W,
}
impl<'a> DECISION_USE_VIT_SOFT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DECISION_USE_VIT_SOFT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn decision_use_vit_soft_default(self) -> &'a mut W {
        self.variant(DECISION_USE_VIT_SOFT_A::DECISION_USE_VIT_SOFT_DEFAULT)
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
#[doc = "Sets the Viterbi path length: 00: 1 bit, 01: 2 bits, 10: 4 bits, 11: 8 bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DECISION_VITERBI_LEN_A {
    #[doc = "0: `0`"]
    DECISION_VITERBI_LEN_DEFAULT = 0,
}
impl From<DECISION_VITERBI_LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DECISION_VITERBI_LEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DECISION_VITERBI_LEN`"]
pub type DECISION_VITERBI_LEN_R = crate::R<u8, DECISION_VITERBI_LEN_A>;
impl DECISION_VITERBI_LEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DECISION_VITERBI_LEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DECISION_VITERBI_LEN_A::DECISION_VITERBI_LEN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DECISION_VITERBI_LEN_DEFAULT`"]
    #[inline(always)]
    pub fn is_decision_viterbi_len_default(&self) -> bool {
        *self == DECISION_VITERBI_LEN_A::DECISION_VITERBI_LEN_DEFAULT
    }
}
#[doc = "Write proxy for field `DECISION_VITERBI_LEN`"]
pub struct DECISION_VITERBI_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DECISION_VITERBI_LEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DECISION_VITERBI_LEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn decision_viterbi_len_default(self) -> &'a mut W {
        self.variant(DECISION_VITERBI_LEN_A::DECISION_VITERBI_LEN_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "if set to 1, the Viterbi algorithm uses power instead of amplitude to evaluate the error on the path\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECISION_VITERBI_POW_NLIN_A {
    #[doc = "0: `0`"]
    DECISION_VITERBI_POW_NLIN_DEFAULT = 0,
}
impl From<DECISION_VITERBI_POW_NLIN_A> for bool {
    #[inline(always)]
    fn from(variant: DECISION_VITERBI_POW_NLIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DECISION_VITERBI_POW_NLIN`"]
pub type DECISION_VITERBI_POW_NLIN_R = crate::R<bool, DECISION_VITERBI_POW_NLIN_A>;
impl DECISION_VITERBI_POW_NLIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DECISION_VITERBI_POW_NLIN_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DECISION_VITERBI_POW_NLIN_A::DECISION_VITERBI_POW_NLIN_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DECISION_VITERBI_POW_NLIN_DEFAULT`"]
    #[inline(always)]
    pub fn is_decision_viterbi_pow_nlin_default(&self) -> bool {
        *self == DECISION_VITERBI_POW_NLIN_A::DECISION_VITERBI_POW_NLIN_DEFAULT
    }
}
#[doc = "Write proxy for field `DECISION_VITERBI_POW_NLIN`"]
pub struct DECISION_VITERBI_POW_NLIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DECISION_VITERBI_POW_NLIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DECISION_VITERBI_POW_NLIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn decision_viterbi_pow_nlin_default(self) -> &'a mut W {
        self.variant(DECISION_VITERBI_POW_NLIN_A::DECISION_VITERBI_POW_NLIN_DEFAULT)
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
#[doc = "If set to 1 enables the Viterbi algorithm for the GFSK decoding; this will override the old ISI correction algorithm.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECISION_EN_VITERBI_GFSK_A {
    #[doc = "0: `0`"]
    DECISION_EN_VITERBI_GFSK_DEFAULT = 0,
}
impl From<DECISION_EN_VITERBI_GFSK_A> for bool {
    #[inline(always)]
    fn from(variant: DECISION_EN_VITERBI_GFSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DECISION_EN_VITERBI_GFSK`"]
pub type DECISION_EN_VITERBI_GFSK_R = crate::R<bool, DECISION_EN_VITERBI_GFSK_A>;
impl DECISION_EN_VITERBI_GFSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DECISION_EN_VITERBI_GFSK_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DECISION_EN_VITERBI_GFSK_A::DECISION_EN_VITERBI_GFSK_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DECISION_EN_VITERBI_GFSK_DEFAULT`"]
    #[inline(always)]
    pub fn is_decision_en_viterbi_gfsk_default(&self) -> bool {
        *self == DECISION_EN_VITERBI_GFSK_A::DECISION_EN_VITERBI_GFSK_DEFAULT
    }
}
#[doc = "Write proxy for field `DECISION_EN_VITERBI_GFSK`"]
pub struct DECISION_EN_VITERBI_GFSK_W<'a> {
    w: &'a mut W,
}
impl<'a> DECISION_EN_VITERBI_GFSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DECISION_EN_VITERBI_GFSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn decision_en_viterbi_gfsk_default(self) -> &'a mut W {
        self.variant(DECISION_EN_VITERBI_GFSK_A::DECISION_EN_VITERBI_GFSK_DEFAULT)
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
    #[doc = "Bits 16:31 - Unsigned value that specifies the IF for the Rx mode."]
    #[inline(always)]
    pub fn correct_cfreq_if_correct_cfreq_if(&self) -> CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_R {
        CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 14:15 - Speed on the RSSI triangular dithering signal (cf reg RSSI_TUN)"]
    #[inline(always)]
    pub fn rssi_bank_rssi_tri_ck_div(&self) -> RSSI_BANK_RSSI_TRI_CK_DIV_R {
        RSSI_BANK_RSSI_TRI_CK_DIV_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 13 - If set to 1, the RSSI filtering is 8x faster"]
    #[inline(always)]
    pub fn rssi_bank_fast_rssi(&self) -> RSSI_BANK_FAST_RSSI_R {
        RSSI_BANK_FAST_RSSI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - If the packet mode is set, indicates to switch the fast modes during the preamble reception"]
    #[inline(always)]
    pub fn rssi_bank_en_fast_pre_sync(&self) -> RSSI_BANK_EN_FAST_PRE_SYNC_R {
        RSSI_BANK_EN_FAST_PRE_SYNC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Time constant of the RSSI filtering block: 0: 4symbols, 1: 8symbols, 2: 16 symbols, 3: 32symbols, 4: 64symbols, 5: 128symbols, 6: 256symbols, 7: 512symbols, 8: 1024symbols"]
    #[inline(always)]
    pub fn rssi_bank_tau_rssi_filtering(&self) -> RSSI_BANK_TAU_RSSI_FILTERING_R {
        RSSI_BANK_TAU_RSSI_FILTERING_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 4 - If set to 1 uses the viterbi soft decoding"]
    #[inline(always)]
    pub fn decision_use_vit_soft(&self) -> DECISION_USE_VIT_SOFT_R {
        DECISION_USE_VIT_SOFT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Sets the Viterbi path length: 00: 1 bit, 01: 2 bits, 10: 4 bits, 11: 8 bits"]
    #[inline(always)]
    pub fn decision_viterbi_len(&self) -> DECISION_VITERBI_LEN_R {
        DECISION_VITERBI_LEN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - if set to 1, the Viterbi algorithm uses power instead of amplitude to evaluate the error on the path"]
    #[inline(always)]
    pub fn decision_viterbi_pow_nlin(&self) -> DECISION_VITERBI_POW_NLIN_R {
        DECISION_VITERBI_POW_NLIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - If set to 1 enables the Viterbi algorithm for the GFSK decoding; this will override the old ISI correction algorithm."]
    #[inline(always)]
    pub fn decision_en_viterbi_gfsk(&self) -> DECISION_EN_VITERBI_GFSK_R {
        DECISION_EN_VITERBI_GFSK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - Unsigned value that specifies the IF for the Rx mode."]
    #[inline(always)]
    pub fn correct_cfreq_if_correct_cfreq_if(&mut self) -> CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_W {
        CORRECT_CFREQ_IF_CORRECT_CFREQ_IF_W { w: self }
    }
    #[doc = "Bits 14:15 - Speed on the RSSI triangular dithering signal (cf reg RSSI_TUN)"]
    #[inline(always)]
    pub fn rssi_bank_rssi_tri_ck_div(&mut self) -> RSSI_BANK_RSSI_TRI_CK_DIV_W {
        RSSI_BANK_RSSI_TRI_CK_DIV_W { w: self }
    }
    #[doc = "Bit 13 - If set to 1, the RSSI filtering is 8x faster"]
    #[inline(always)]
    pub fn rssi_bank_fast_rssi(&mut self) -> RSSI_BANK_FAST_RSSI_W {
        RSSI_BANK_FAST_RSSI_W { w: self }
    }
    #[doc = "Bit 12 - If the packet mode is set, indicates to switch the fast modes during the preamble reception"]
    #[inline(always)]
    pub fn rssi_bank_en_fast_pre_sync(&mut self) -> RSSI_BANK_EN_FAST_PRE_SYNC_W {
        RSSI_BANK_EN_FAST_PRE_SYNC_W { w: self }
    }
    #[doc = "Bits 8:11 - Time constant of the RSSI filtering block: 0: 4symbols, 1: 8symbols, 2: 16 symbols, 3: 32symbols, 4: 64symbols, 5: 128symbols, 6: 256symbols, 7: 512symbols, 8: 1024symbols"]
    #[inline(always)]
    pub fn rssi_bank_tau_rssi_filtering(&mut self) -> RSSI_BANK_TAU_RSSI_FILTERING_W {
        RSSI_BANK_TAU_RSSI_FILTERING_W { w: self }
    }
    #[doc = "Bit 4 - If set to 1 uses the viterbi soft decoding"]
    #[inline(always)]
    pub fn decision_use_vit_soft(&mut self) -> DECISION_USE_VIT_SOFT_W {
        DECISION_USE_VIT_SOFT_W { w: self }
    }
    #[doc = "Bits 2:3 - Sets the Viterbi path length: 00: 1 bit, 01: 2 bits, 10: 4 bits, 11: 8 bits"]
    #[inline(always)]
    pub fn decision_viterbi_len(&mut self) -> DECISION_VITERBI_LEN_W {
        DECISION_VITERBI_LEN_W { w: self }
    }
    #[doc = "Bit 1 - if set to 1, the Viterbi algorithm uses power instead of amplitude to evaluate the error on the path"]
    #[inline(always)]
    pub fn decision_viterbi_pow_nlin(&mut self) -> DECISION_VITERBI_POW_NLIN_W {
        DECISION_VITERBI_POW_NLIN_W { w: self }
    }
    #[doc = "Bit 0 - If set to 1 enables the Viterbi algorithm for the GFSK decoding; this will override the old ISI correction algorithm."]
    #[inline(always)]
    pub fn decision_en_viterbi_gfsk(&mut self) -> DECISION_EN_VITERBI_GFSK_W {
        DECISION_EN_VITERBI_GFSK_W { w: self }
    }
}
