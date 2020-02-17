#[doc = "Reader of register RF_REG07"]
pub type R = crate::R<u32, super::RF_REG07>;
#[doc = "Writer for register RF_REG07"]
pub type W = crate::W<u32, super::RF_REG07>;
#[doc = "Register RF_REG07 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG07 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "channel spacing: the formula that determines this value is the same as for the central frequency. v=ch_sp/144e6*2^25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CHANNELS_1_CHANNEL_SPACING_LO_A {
    #[doc = "0: `0`"]
    CHANNELS_1_CHANNEL_SPACING_LO_DEFAULT = 0,
}
impl From<CHANNELS_1_CHANNEL_SPACING_LO_A> for u16 {
    #[inline(always)]
    fn from(variant: CHANNELS_1_CHANNEL_SPACING_LO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHANNELS_1_CHANNEL_SPACING_LO`"]
pub type CHANNELS_1_CHANNEL_SPACING_LO_R = crate::R<u16, CHANNELS_1_CHANNEL_SPACING_LO_A>;
impl CHANNELS_1_CHANNEL_SPACING_LO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, CHANNELS_1_CHANNEL_SPACING_LO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CHANNELS_1_CHANNEL_SPACING_LO_A::CHANNELS_1_CHANNEL_SPACING_LO_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CHANNELS_1_CHANNEL_SPACING_LO_DEFAULT`"]
    #[inline(always)]
    pub fn is_channels_1_channel_spacing_lo_default(&self) -> bool {
        *self == CHANNELS_1_CHANNEL_SPACING_LO_A::CHANNELS_1_CHANNEL_SPACING_LO_DEFAULT
    }
}
#[doc = "Write proxy for field `CHANNELS_1_CHANNEL_SPACING_LO`"]
pub struct CHANNELS_1_CHANNEL_SPACING_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNELS_1_CHANNEL_SPACING_LO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNELS_1_CHANNEL_SPACING_LO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn channels_1_channel_spacing_lo_default(self) -> &'a mut W {
        self.variant(CHANNELS_1_CHANNEL_SPACING_LO_A::CHANNELS_1_CHANNEL_SPACING_LO_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "If set to 1 the clock divider will provide a clock divided by 2 instead of 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_INFO_RX_EN_DIV_2_N3_RX_A {
    #[doc = "0: `0`"]
    MOD_INFO_RX_EN_DIV_2_N3_RX_DEFAULT = 0,
}
impl From<MOD_INFO_RX_EN_DIV_2_N3_RX_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_INFO_RX_EN_DIV_2_N3_RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MOD_INFO_RX_EN_DIV_2_N3_RX`"]
pub type MOD_INFO_RX_EN_DIV_2_N3_RX_R = crate::R<bool, MOD_INFO_RX_EN_DIV_2_N3_RX_A>;
impl MOD_INFO_RX_EN_DIV_2_N3_RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MOD_INFO_RX_EN_DIV_2_N3_RX_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MOD_INFO_RX_EN_DIV_2_N3_RX_A::MOD_INFO_RX_EN_DIV_2_N3_RX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MOD_INFO_RX_EN_DIV_2_N3_RX_DEFAULT`"]
    #[inline(always)]
    pub fn is_mod_info_rx_en_div_2_n3_rx_default(&self) -> bool {
        *self == MOD_INFO_RX_EN_DIV_2_N3_RX_A::MOD_INFO_RX_EN_DIV_2_N3_RX_DEFAULT
    }
}
#[doc = "Write proxy for field `MOD_INFO_RX_EN_DIV_2_N3_RX`"]
pub struct MOD_INFO_RX_EN_DIV_2_N3_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_INFO_RX_EN_DIV_2_N3_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_INFO_RX_EN_DIV_2_N3_RX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mod_info_rx_en_div_2_n3_rx_default(self) -> &'a mut W {
        self.variant(MOD_INFO_RX_EN_DIV_2_N3_RX_A::MOD_INFO_RX_EN_DIV_2_N3_RX_DEFAULT)
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
#[doc = "If set to 1, each symbol is composed by 2 bits (OQPSK or 4FSK)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_INFO_RX_SYMBOL_2BIT_RX_A {
    #[doc = "0: `0`"]
    MOD_INFO_RX_SYMBOL_2BIT_RX_DEFAULT = 0,
}
impl From<MOD_INFO_RX_SYMBOL_2BIT_RX_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_INFO_RX_SYMBOL_2BIT_RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MOD_INFO_RX_SYMBOL_2BIT_RX`"]
pub type MOD_INFO_RX_SYMBOL_2BIT_RX_R = crate::R<bool, MOD_INFO_RX_SYMBOL_2BIT_RX_A>;
impl MOD_INFO_RX_SYMBOL_2BIT_RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MOD_INFO_RX_SYMBOL_2BIT_RX_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MOD_INFO_RX_SYMBOL_2BIT_RX_A::MOD_INFO_RX_SYMBOL_2BIT_RX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MOD_INFO_RX_SYMBOL_2BIT_RX_DEFAULT`"]
    #[inline(always)]
    pub fn is_mod_info_rx_symbol_2bit_rx_default(&self) -> bool {
        *self == MOD_INFO_RX_SYMBOL_2BIT_RX_A::MOD_INFO_RX_SYMBOL_2BIT_RX_DEFAULT
    }
}
#[doc = "Write proxy for field `MOD_INFO_RX_SYMBOL_2BIT_RX`"]
pub struct MOD_INFO_RX_SYMBOL_2BIT_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_INFO_RX_SYMBOL_2BIT_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_INFO_RX_SYMBOL_2BIT_RX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mod_info_rx_symbol_2bit_rx_default(self) -> &'a mut W {
        self.variant(MOD_INFO_RX_SYMBOL_2BIT_RX_A::MOD_INFO_RX_SYMBOL_2BIT_RX_DEFAULT)
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
#[doc = "Unsigned value that determine the oversampling frequency and consequently the data-rate. This frequency is the system frequency (16 or 24 MHz) divided by this value+1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MOD_INFO_RX_DR_M_RX_A {
    #[doc = "0: `0`"]
    MOD_INFO_RX_DR_M_RX_DEFAULT = 0,
}
impl From<MOD_INFO_RX_DR_M_RX_A> for u8 {
    #[inline(always)]
    fn from(variant: MOD_INFO_RX_DR_M_RX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MOD_INFO_RX_DR_M_RX`"]
pub type MOD_INFO_RX_DR_M_RX_R = crate::R<u8, MOD_INFO_RX_DR_M_RX_A>;
impl MOD_INFO_RX_DR_M_RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MOD_INFO_RX_DR_M_RX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MOD_INFO_RX_DR_M_RX_A::MOD_INFO_RX_DR_M_RX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MOD_INFO_RX_DR_M_RX_DEFAULT`"]
    #[inline(always)]
    pub fn is_mod_info_rx_dr_m_rx_default(&self) -> bool {
        *self == MOD_INFO_RX_DR_M_RX_A::MOD_INFO_RX_DR_M_RX_DEFAULT
    }
}
#[doc = "Write proxy for field `MOD_INFO_RX_DR_M_RX`"]
pub struct MOD_INFO_RX_DR_M_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_INFO_RX_DR_M_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_INFO_RX_DR_M_RX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mod_info_rx_dr_m_rx_default(self) -> &'a mut W {
        self.variant(MOD_INFO_RX_DR_M_RX_A::MOD_INFO_RX_DR_M_RX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "If set to 1 the clock divider will provide a clock divided by 2 instead of 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_INFO_TX_EN_DIV_2_N3_TX_A {
    #[doc = "0: `0`"]
    MOD_INFO_TX_EN_DIV_2_N3_TX_DEFAULT = 0,
}
impl From<MOD_INFO_TX_EN_DIV_2_N3_TX_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_INFO_TX_EN_DIV_2_N3_TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MOD_INFO_TX_EN_DIV_2_N3_TX`"]
pub type MOD_INFO_TX_EN_DIV_2_N3_TX_R = crate::R<bool, MOD_INFO_TX_EN_DIV_2_N3_TX_A>;
impl MOD_INFO_TX_EN_DIV_2_N3_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MOD_INFO_TX_EN_DIV_2_N3_TX_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MOD_INFO_TX_EN_DIV_2_N3_TX_A::MOD_INFO_TX_EN_DIV_2_N3_TX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MOD_INFO_TX_EN_DIV_2_N3_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_mod_info_tx_en_div_2_n3_tx_default(&self) -> bool {
        *self == MOD_INFO_TX_EN_DIV_2_N3_TX_A::MOD_INFO_TX_EN_DIV_2_N3_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `MOD_INFO_TX_EN_DIV_2_N3_TX`"]
pub struct MOD_INFO_TX_EN_DIV_2_N3_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_INFO_TX_EN_DIV_2_N3_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_INFO_TX_EN_DIV_2_N3_TX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mod_info_tx_en_div_2_n3_tx_default(self) -> &'a mut W {
        self.variant(MOD_INFO_TX_EN_DIV_2_N3_TX_A::MOD_INFO_TX_EN_DIV_2_N3_TX_DEFAULT)
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
#[doc = "If set to 1, each symbol is composed by 2 bits (OQPSK or 4FSK)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_INFO_TX_SYMBOL_2BIT_TX_A {
    #[doc = "0: `0`"]
    MOD_INFO_TX_SYMBOL_2BIT_TX_DEFAULT = 0,
}
impl From<MOD_INFO_TX_SYMBOL_2BIT_TX_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_INFO_TX_SYMBOL_2BIT_TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MOD_INFO_TX_SYMBOL_2BIT_TX`"]
pub type MOD_INFO_TX_SYMBOL_2BIT_TX_R = crate::R<bool, MOD_INFO_TX_SYMBOL_2BIT_TX_A>;
impl MOD_INFO_TX_SYMBOL_2BIT_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MOD_INFO_TX_SYMBOL_2BIT_TX_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MOD_INFO_TX_SYMBOL_2BIT_TX_A::MOD_INFO_TX_SYMBOL_2BIT_TX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MOD_INFO_TX_SYMBOL_2BIT_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_mod_info_tx_symbol_2bit_tx_default(&self) -> bool {
        *self == MOD_INFO_TX_SYMBOL_2BIT_TX_A::MOD_INFO_TX_SYMBOL_2BIT_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `MOD_INFO_TX_SYMBOL_2BIT_TX`"]
pub struct MOD_INFO_TX_SYMBOL_2BIT_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_INFO_TX_SYMBOL_2BIT_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_INFO_TX_SYMBOL_2BIT_TX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mod_info_tx_symbol_2bit_tx_default(self) -> &'a mut W {
        self.variant(MOD_INFO_TX_SYMBOL_2BIT_TX_A::MOD_INFO_TX_SYMBOL_2BIT_TX_DEFAULT)
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
#[doc = "Unsigned value that determine the oversampling frequency and consequently the data-rate. This frequency is the system frequency (16 or 24 MHz) divided by this value+1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MOD_INFO_TX_DR_M_TX_A {
    #[doc = "0: `0`"]
    MOD_INFO_TX_DR_M_TX_DEFAULT = 0,
}
impl From<MOD_INFO_TX_DR_M_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: MOD_INFO_TX_DR_M_TX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MOD_INFO_TX_DR_M_TX`"]
pub type MOD_INFO_TX_DR_M_TX_R = crate::R<u8, MOD_INFO_TX_DR_M_TX_A>;
impl MOD_INFO_TX_DR_M_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MOD_INFO_TX_DR_M_TX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MOD_INFO_TX_DR_M_TX_A::MOD_INFO_TX_DR_M_TX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MOD_INFO_TX_DR_M_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_mod_info_tx_dr_m_tx_default(&self) -> bool {
        *self == MOD_INFO_TX_DR_M_TX_A::MOD_INFO_TX_DR_M_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `MOD_INFO_TX_DR_M_TX`"]
pub struct MOD_INFO_TX_DR_M_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_INFO_TX_DR_M_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_INFO_TX_DR_M_TX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mod_info_tx_dr_m_tx_default(self) -> &'a mut W {
        self.variant(MOD_INFO_TX_DR_M_TX_A::MOD_INFO_TX_DR_M_TX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - channel spacing: the formula that determines this value is the same as for the central frequency. v=ch_sp/144e6*2^25"]
    #[inline(always)]
    pub fn channels_1_channel_spacing_lo(&self) -> CHANNELS_1_CHANNEL_SPACING_LO_R {
        CHANNELS_1_CHANNEL_SPACING_LO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 14 - If set to 1 the clock divider will provide a clock divided by 2 instead of 3."]
    #[inline(always)]
    pub fn mod_info_rx_en_div_2_n3_rx(&self) -> MOD_INFO_RX_EN_DIV_2_N3_RX_R {
        MOD_INFO_RX_EN_DIV_2_N3_RX_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - If set to 1, each symbol is composed by 2 bits (OQPSK or 4FSK)"]
    #[inline(always)]
    pub fn mod_info_rx_symbol_2bit_rx(&self) -> MOD_INFO_RX_SYMBOL_2BIT_RX_R {
        MOD_INFO_RX_SYMBOL_2BIT_RX_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Unsigned value that determine the oversampling frequency and consequently the data-rate. This frequency is the system frequency (16 or 24 MHz) divided by this value+1."]
    #[inline(always)]
    pub fn mod_info_rx_dr_m_rx(&self) -> MOD_INFO_RX_DR_M_RX_R {
        MOD_INFO_RX_DR_M_RX_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - If set to 1 the clock divider will provide a clock divided by 2 instead of 3."]
    #[inline(always)]
    pub fn mod_info_tx_en_div_2_n3_tx(&self) -> MOD_INFO_TX_EN_DIV_2_N3_TX_R {
        MOD_INFO_TX_EN_DIV_2_N3_TX_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If set to 1, each symbol is composed by 2 bits (OQPSK or 4FSK)"]
    #[inline(always)]
    pub fn mod_info_tx_symbol_2bit_tx(&self) -> MOD_INFO_TX_SYMBOL_2BIT_TX_R {
        MOD_INFO_TX_SYMBOL_2BIT_TX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - Unsigned value that determine the oversampling frequency and consequently the data-rate. This frequency is the system frequency (16 or 24 MHz) divided by this value+1."]
    #[inline(always)]
    pub fn mod_info_tx_dr_m_tx(&self) -> MOD_INFO_TX_DR_M_TX_R {
        MOD_INFO_TX_DR_M_TX_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - channel spacing: the formula that determines this value is the same as for the central frequency. v=ch_sp/144e6*2^25"]
    #[inline(always)]
    pub fn channels_1_channel_spacing_lo(&mut self) -> CHANNELS_1_CHANNEL_SPACING_LO_W {
        CHANNELS_1_CHANNEL_SPACING_LO_W { w: self }
    }
    #[doc = "Bit 14 - If set to 1 the clock divider will provide a clock divided by 2 instead of 3."]
    #[inline(always)]
    pub fn mod_info_rx_en_div_2_n3_rx(&mut self) -> MOD_INFO_RX_EN_DIV_2_N3_RX_W {
        MOD_INFO_RX_EN_DIV_2_N3_RX_W { w: self }
    }
    #[doc = "Bit 13 - If set to 1, each symbol is composed by 2 bits (OQPSK or 4FSK)"]
    #[inline(always)]
    pub fn mod_info_rx_symbol_2bit_rx(&mut self) -> MOD_INFO_RX_SYMBOL_2BIT_RX_W {
        MOD_INFO_RX_SYMBOL_2BIT_RX_W { w: self }
    }
    #[doc = "Bits 8:12 - Unsigned value that determine the oversampling frequency and consequently the data-rate. This frequency is the system frequency (16 or 24 MHz) divided by this value+1."]
    #[inline(always)]
    pub fn mod_info_rx_dr_m_rx(&mut self) -> MOD_INFO_RX_DR_M_RX_W {
        MOD_INFO_RX_DR_M_RX_W { w: self }
    }
    #[doc = "Bit 6 - If set to 1 the clock divider will provide a clock divided by 2 instead of 3."]
    #[inline(always)]
    pub fn mod_info_tx_en_div_2_n3_tx(&mut self) -> MOD_INFO_TX_EN_DIV_2_N3_TX_W {
        MOD_INFO_TX_EN_DIV_2_N3_TX_W { w: self }
    }
    #[doc = "Bit 5 - If set to 1, each symbol is composed by 2 bits (OQPSK or 4FSK)"]
    #[inline(always)]
    pub fn mod_info_tx_symbol_2bit_tx(&mut self) -> MOD_INFO_TX_SYMBOL_2BIT_TX_W {
        MOD_INFO_TX_SYMBOL_2BIT_TX_W { w: self }
    }
    #[doc = "Bits 0:4 - Unsigned value that determine the oversampling frequency and consequently the data-rate. This frequency is the system frequency (16 or 24 MHz) divided by this value+1."]
    #[inline(always)]
    pub fn mod_info_tx_dr_m_tx(&mut self) -> MOD_INFO_TX_DR_M_TX_W {
        MOD_INFO_TX_DR_M_TX_W { w: self }
    }
}
