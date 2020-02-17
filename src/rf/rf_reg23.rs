#[doc = "Reader of register RF_REG23"]
pub type R = crate::R<u32, super::RF_REG23>;
#[doc = "Writer for register RF_REG23"]
pub type W = crate::W<u32, super::RF_REG23>;
#[doc = "Register RF_REG23 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG23 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PrePA Casc bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_1_IQ_RXTX_3_A {
    #[doc = "0: `0`"]
    BIAS_1_IQ_RXTX_3_DEFAULT = 0,
}
impl From<BIAS_1_IQ_RXTX_3_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_1_IQ_RXTX_3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_1_IQ_RXTX_3`"]
pub type BIAS_1_IQ_RXTX_3_R = crate::R<u8, BIAS_1_IQ_RXTX_3_A>;
impl BIAS_1_IQ_RXTX_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_1_IQ_RXTX_3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_1_IQ_RXTX_3_A::BIAS_1_IQ_RXTX_3_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_1_IQ_RXTX_3_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_1_iq_rxtx_3_default(&self) -> bool {
        *self == BIAS_1_IQ_RXTX_3_A::BIAS_1_IQ_RXTX_3_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_1_IQ_RXTX_3`"]
pub struct BIAS_1_IQ_RXTX_3_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_1_IQ_RXTX_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_1_IQ_RXTX_3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_1_iq_rxtx_3_default(self) -> &'a mut W {
        self.variant(BIAS_1_IQ_RXTX_3_A::BIAS_1_IQ_RXTX_3_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "PrePA In bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_1_IQ_RXTX_2_A {
    #[doc = "0: `0`"]
    BIAS_1_IQ_RXTX_2_DEFAULT = 0,
}
impl From<BIAS_1_IQ_RXTX_2_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_1_IQ_RXTX_2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_1_IQ_RXTX_2`"]
pub type BIAS_1_IQ_RXTX_2_R = crate::R<u8, BIAS_1_IQ_RXTX_2_A>;
impl BIAS_1_IQ_RXTX_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_1_IQ_RXTX_2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_1_IQ_RXTX_2_A::BIAS_1_IQ_RXTX_2_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_1_IQ_RXTX_2_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_1_iq_rxtx_2_default(&self) -> bool {
        *self == BIAS_1_IQ_RXTX_2_A::BIAS_1_IQ_RXTX_2_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_1_IQ_RXTX_2`"]
pub struct BIAS_1_IQ_RXTX_2_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_1_IQ_RXTX_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_1_IQ_RXTX_2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_1_iq_rxtx_2_default(self) -> &'a mut W {
        self.variant(BIAS_1_IQ_RXTX_2_A::BIAS_1_IQ_RXTX_2_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "PA backoff bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_0_IQ_RXTX_1_A {
    #[doc = "0: `0`"]
    BIAS_0_IQ_RXTX_1_DEFAULT = 0,
}
impl From<BIAS_0_IQ_RXTX_1_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_0_IQ_RXTX_1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_0_IQ_RXTX_1`"]
pub type BIAS_0_IQ_RXTX_1_R = crate::R<u8, BIAS_0_IQ_RXTX_1_A>;
impl BIAS_0_IQ_RXTX_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_0_IQ_RXTX_1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_0_IQ_RXTX_1_A::BIAS_0_IQ_RXTX_1_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_0_IQ_RXTX_1_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_0_iq_rxtx_1_default(&self) -> bool {
        *self == BIAS_0_IQ_RXTX_1_A::BIAS_0_IQ_RXTX_1_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_0_IQ_RXTX_1`"]
pub struct BIAS_0_IQ_RXTX_1_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_0_IQ_RXTX_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_0_IQ_RXTX_1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_0_iq_rxtx_1_default(self) -> &'a mut W {
        self.variant(BIAS_0_IQ_RXTX_1_A::BIAS_0_IQ_RXTX_1_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "PA bias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_0_IQ_RXTX_0_A {
    #[doc = "0: `0`"]
    BIAS_0_IQ_RXTX_0_DEFAULT = 0,
}
impl From<BIAS_0_IQ_RXTX_0_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_0_IQ_RXTX_0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS_0_IQ_RXTX_0`"]
pub type BIAS_0_IQ_RXTX_0_R = crate::R<u8, BIAS_0_IQ_RXTX_0_A>;
impl BIAS_0_IQ_RXTX_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIAS_0_IQ_RXTX_0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIAS_0_IQ_RXTX_0_A::BIAS_0_IQ_RXTX_0_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS_0_IQ_RXTX_0_DEFAULT`"]
    #[inline(always)]
    pub fn is_bias_0_iq_rxtx_0_default(&self) -> bool {
        *self == BIAS_0_IQ_RXTX_0_A::BIAS_0_IQ_RXTX_0_DEFAULT
    }
}
#[doc = "Write proxy for field `BIAS_0_IQ_RXTX_0`"]
pub struct BIAS_0_IQ_RXTX_0_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_0_IQ_RXTX_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_0_IQ_RXTX_0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bias_0_iq_rxtx_0_default(self) -> &'a mut W {
        self.variant(BIAS_0_IQ_RXTX_0_A::BIAS_0_IQ_RXTX_0_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Select the number of wait states during the APB transaction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTERFACE_CONF_APB_WAIT_STATE_A {
    #[doc = "0: `0`"]
    INTERFACE_CONF_APB_WAIT_STATE_DEFAULT = 0,
}
impl From<INTERFACE_CONF_APB_WAIT_STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: INTERFACE_CONF_APB_WAIT_STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INTERFACE_CONF_APB_WAIT_STATE`"]
pub type INTERFACE_CONF_APB_WAIT_STATE_R = crate::R<u8, INTERFACE_CONF_APB_WAIT_STATE_A>;
impl INTERFACE_CONF_APB_WAIT_STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INTERFACE_CONF_APB_WAIT_STATE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INTERFACE_CONF_APB_WAIT_STATE_A::INTERFACE_CONF_APB_WAIT_STATE_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INTERFACE_CONF_APB_WAIT_STATE_DEFAULT`"]
    #[inline(always)]
    pub fn is_interface_conf_apb_wait_state_default(&self) -> bool {
        *self == INTERFACE_CONF_APB_WAIT_STATE_A::INTERFACE_CONF_APB_WAIT_STATE_DEFAULT
    }
}
#[doc = "Write proxy for field `INTERFACE_CONF_APB_WAIT_STATE`"]
pub struct INTERFACE_CONF_APB_WAIT_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERFACE_CONF_APB_WAIT_STATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTERFACE_CONF_APB_WAIT_STATE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn interface_conf_apb_wait_state_default(self) -> &'a mut W {
        self.variant(INTERFACE_CONF_APB_WAIT_STATE_A::INTERFACE_CONF_APB_WAIT_STATE_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Select the spi mode: 00 legacy spi, 01 advanced spi, 10 BLIM4SME spi\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTERFACE_CONF_SPI_SELECT_A {
    #[doc = "0: `0`"]
    INTERFACE_CONF_SPI_SELECT_DEFAULT = 0,
}
impl From<INTERFACE_CONF_SPI_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: INTERFACE_CONF_SPI_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INTERFACE_CONF_SPI_SELECT`"]
pub type INTERFACE_CONF_SPI_SELECT_R = crate::R<u8, INTERFACE_CONF_SPI_SELECT_A>;
impl INTERFACE_CONF_SPI_SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INTERFACE_CONF_SPI_SELECT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INTERFACE_CONF_SPI_SELECT_A::INTERFACE_CONF_SPI_SELECT_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INTERFACE_CONF_SPI_SELECT_DEFAULT`"]
    #[inline(always)]
    pub fn is_interface_conf_spi_select_default(&self) -> bool {
        *self == INTERFACE_CONF_SPI_SELECT_A::INTERFACE_CONF_SPI_SELECT_DEFAULT
    }
}
#[doc = "Write proxy for field `INTERFACE_CONF_SPI_SELECT`"]
pub struct INTERFACE_CONF_SPI_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERFACE_CONF_SPI_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTERFACE_CONF_SPI_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn interface_conf_spi_select_default(self) -> &'a mut W {
        self.variant(INTERFACE_CONF_SPI_SELECT_A::INTERFACE_CONF_SPI_SELECT_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "If set to 1 enables the timeout of the Rx when the system is on FSM mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_EN_RX_TIMEOUT_A {
    #[doc = "0: `0`"]
    TIMEOUT_EN_RX_TIMEOUT_DEFAULT = 0,
}
impl From<TIMEOUT_EN_RX_TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_EN_RX_TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMEOUT_EN_RX_TIMEOUT`"]
pub type TIMEOUT_EN_RX_TIMEOUT_R = crate::R<bool, TIMEOUT_EN_RX_TIMEOUT_A>;
impl TIMEOUT_EN_RX_TIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TIMEOUT_EN_RX_TIMEOUT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(TIMEOUT_EN_RX_TIMEOUT_A::TIMEOUT_EN_RX_TIMEOUT_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMEOUT_EN_RX_TIMEOUT_DEFAULT`"]
    #[inline(always)]
    pub fn is_timeout_en_rx_timeout_default(&self) -> bool {
        *self == TIMEOUT_EN_RX_TIMEOUT_A::TIMEOUT_EN_RX_TIMEOUT_DEFAULT
    }
}
#[doc = "Write proxy for field `TIMEOUT_EN_RX_TIMEOUT`"]
pub struct TIMEOUT_EN_RX_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_EN_RX_TIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUT_EN_RX_TIMEOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn timeout_en_rx_timeout_default(self) -> &'a mut W {
        self.variant(TIMEOUT_EN_RX_TIMEOUT_A::TIMEOUT_EN_RX_TIMEOUT_DEFAULT)
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
#[doc = "Granularity of the timer in timeout Rx mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMEOUT_T_TIMEOUT_GR_A {
    #[doc = "0: `0`"]
    TIMEOUT_T_TIMEOUT_GR_DEFAULT = 0,
}
impl From<TIMEOUT_T_TIMEOUT_GR_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT_T_TIMEOUT_GR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMEOUT_T_TIMEOUT_GR`"]
pub type TIMEOUT_T_TIMEOUT_GR_R = crate::R<u8, TIMEOUT_T_TIMEOUT_GR_A>;
impl TIMEOUT_T_TIMEOUT_GR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMEOUT_T_TIMEOUT_GR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIMEOUT_T_TIMEOUT_GR_A::TIMEOUT_T_TIMEOUT_GR_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMEOUT_T_TIMEOUT_GR_DEFAULT`"]
    #[inline(always)]
    pub fn is_timeout_t_timeout_gr_default(&self) -> bool {
        *self == TIMEOUT_T_TIMEOUT_GR_A::TIMEOUT_T_TIMEOUT_GR_DEFAULT
    }
}
#[doc = "Write proxy for field `TIMEOUT_T_TIMEOUT_GR`"]
pub struct TIMEOUT_T_TIMEOUT_GR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_T_TIMEOUT_GR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUT_T_TIMEOUT_GR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn timeout_t_timeout_gr_default(self) -> &'a mut W {
        self.variant(TIMEOUT_T_TIMEOUT_GR_A::TIMEOUT_T_TIMEOUT_GR_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Time that has to occur before the timeout.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMEOUT_T_RX_TIMEOUT_A {
    #[doc = "0: `0`"]
    TIMEOUT_T_RX_TIMEOUT_DEFAULT = 0,
}
impl From<TIMEOUT_T_RX_TIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT_T_RX_TIMEOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMEOUT_T_RX_TIMEOUT`"]
pub type TIMEOUT_T_RX_TIMEOUT_R = crate::R<u8, TIMEOUT_T_RX_TIMEOUT_A>;
impl TIMEOUT_T_RX_TIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMEOUT_T_RX_TIMEOUT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIMEOUT_T_RX_TIMEOUT_A::TIMEOUT_T_RX_TIMEOUT_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMEOUT_T_RX_TIMEOUT_DEFAULT`"]
    #[inline(always)]
    pub fn is_timeout_t_rx_timeout_default(&self) -> bool {
        *self == TIMEOUT_T_RX_TIMEOUT_A::TIMEOUT_T_RX_TIMEOUT_DEFAULT
    }
}
#[doc = "Write proxy for field `TIMEOUT_T_RX_TIMEOUT`"]
pub struct TIMEOUT_T_RX_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_T_RX_TIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUT_T_RX_TIMEOUT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn timeout_t_rx_timeout_default(self) -> &'a mut W {
        self.variant(TIMEOUT_T_RX_TIMEOUT_A::TIMEOUT_T_RX_TIMEOUT_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - PrePA Casc bias"]
    #[inline(always)]
    pub fn bias_1_iq_rxtx_3(&self) -> BIAS_1_IQ_RXTX_3_R {
        BIAS_1_IQ_RXTX_3_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PrePA In bias"]
    #[inline(always)]
    pub fn bias_1_iq_rxtx_2(&self) -> BIAS_1_IQ_RXTX_2_R {
        BIAS_1_IQ_RXTX_2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PA backoff bias"]
    #[inline(always)]
    pub fn bias_0_iq_rxtx_1(&self) -> BIAS_0_IQ_RXTX_1_R {
        BIAS_0_IQ_RXTX_1_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PA bias"]
    #[inline(always)]
    pub fn bias_0_iq_rxtx_0(&self) -> BIAS_0_IQ_RXTX_0_R {
        BIAS_0_IQ_RXTX_0_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Select the number of wait states during the APB transaction"]
    #[inline(always)]
    pub fn interface_conf_apb_wait_state(&self) -> INTERFACE_CONF_APB_WAIT_STATE_R {
        INTERFACE_CONF_APB_WAIT_STATE_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Select the spi mode: 00 legacy spi, 01 advanced spi, 10 BLIM4SME spi"]
    #[inline(always)]
    pub fn interface_conf_spi_select(&self) -> INTERFACE_CONF_SPI_SELECT_R {
        INTERFACE_CONF_SPI_SELECT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 7 - If set to 1 enables the timeout of the Rx when the system is on FSM mode"]
    #[inline(always)]
    pub fn timeout_en_rx_timeout(&self) -> TIMEOUT_EN_RX_TIMEOUT_R {
        TIMEOUT_EN_RX_TIMEOUT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Granularity of the timer in timeout Rx mode"]
    #[inline(always)]
    pub fn timeout_t_timeout_gr(&self) -> TIMEOUT_T_TIMEOUT_GR_R {
        TIMEOUT_T_TIMEOUT_GR_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:3 - Time that has to occur before the timeout."]
    #[inline(always)]
    pub fn timeout_t_rx_timeout(&self) -> TIMEOUT_T_RX_TIMEOUT_R {
        TIMEOUT_T_RX_TIMEOUT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - PrePA Casc bias"]
    #[inline(always)]
    pub fn bias_1_iq_rxtx_3(&mut self) -> BIAS_1_IQ_RXTX_3_W {
        BIAS_1_IQ_RXTX_3_W { w: self }
    }
    #[doc = "Bits 24:27 - PrePA In bias"]
    #[inline(always)]
    pub fn bias_1_iq_rxtx_2(&mut self) -> BIAS_1_IQ_RXTX_2_W {
        BIAS_1_IQ_RXTX_2_W { w: self }
    }
    #[doc = "Bits 20:23 - PA backoff bias"]
    #[inline(always)]
    pub fn bias_0_iq_rxtx_1(&mut self) -> BIAS_0_IQ_RXTX_1_W {
        BIAS_0_IQ_RXTX_1_W { w: self }
    }
    #[doc = "Bits 16:19 - PA bias"]
    #[inline(always)]
    pub fn bias_0_iq_rxtx_0(&mut self) -> BIAS_0_IQ_RXTX_0_W {
        BIAS_0_IQ_RXTX_0_W { w: self }
    }
    #[doc = "Bits 12:14 - Select the number of wait states during the APB transaction"]
    #[inline(always)]
    pub fn interface_conf_apb_wait_state(&mut self) -> INTERFACE_CONF_APB_WAIT_STATE_W {
        INTERFACE_CONF_APB_WAIT_STATE_W { w: self }
    }
    #[doc = "Bits 8:9 - Select the spi mode: 00 legacy spi, 01 advanced spi, 10 BLIM4SME spi"]
    #[inline(always)]
    pub fn interface_conf_spi_select(&mut self) -> INTERFACE_CONF_SPI_SELECT_W {
        INTERFACE_CONF_SPI_SELECT_W { w: self }
    }
    #[doc = "Bit 7 - If set to 1 enables the timeout of the Rx when the system is on FSM mode"]
    #[inline(always)]
    pub fn timeout_en_rx_timeout(&mut self) -> TIMEOUT_EN_RX_TIMEOUT_W {
        TIMEOUT_EN_RX_TIMEOUT_W { w: self }
    }
    #[doc = "Bits 4:6 - Granularity of the timer in timeout Rx mode"]
    #[inline(always)]
    pub fn timeout_t_timeout_gr(&mut self) -> TIMEOUT_T_TIMEOUT_GR_W {
        TIMEOUT_T_TIMEOUT_GR_W { w: self }
    }
    #[doc = "Bits 0:3 - Time that has to occur before the timeout."]
    #[inline(always)]
    pub fn timeout_t_rx_timeout(&mut self) -> TIMEOUT_T_RX_TIMEOUT_W {
        TIMEOUT_T_RX_TIMEOUT_W { w: self }
    }
}
