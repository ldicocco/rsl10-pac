#[doc = "Reader of register RF_REG03"]
pub type R = crate::R<u32, super::RF_REG03>;
#[doc = "Writer for register RF_REG03"]
pub type W = crate::W<u32, super::RF_REG03>;
#[doc = "Register RF_REG03 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REG03 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Configuration of GPIO pad 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD_CONF_2_PAD_3_CONF_A {
    #[doc = "0: `0`"]
    PAD_CONF_2_PAD_3_CONF_DEFAULT = 0,
}
impl From<PAD_CONF_2_PAD_3_CONF_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD_CONF_2_PAD_3_CONF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD_CONF_2_PAD_3_CONF`"]
pub type PAD_CONF_2_PAD_3_CONF_R = crate::R<u8, PAD_CONF_2_PAD_3_CONF_A>;
impl PAD_CONF_2_PAD_3_CONF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD_CONF_2_PAD_3_CONF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD_CONF_2_PAD_3_CONF_A::PAD_CONF_2_PAD_3_CONF_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PAD_CONF_2_PAD_3_CONF_DEFAULT`"]
    #[inline(always)]
    pub fn is_pad_conf_2_pad_3_conf_default(&self) -> bool {
        *self == PAD_CONF_2_PAD_3_CONF_A::PAD_CONF_2_PAD_3_CONF_DEFAULT
    }
}
#[doc = "Write proxy for field `PAD_CONF_2_PAD_3_CONF`"]
pub struct PAD_CONF_2_PAD_3_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CONF_2_PAD_3_CONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD_CONF_2_PAD_3_CONF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pad_conf_2_pad_3_conf_default(self) -> &'a mut W {
        self.variant(PAD_CONF_2_PAD_3_CONF_A::PAD_CONF_2_PAD_3_CONF_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Configuration of GPIO pad 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD_CONF_2_PAD_2_CONF_A {
    #[doc = "0: `0`"]
    PAD_CONF_2_PAD_2_CONF_DEFAULT = 0,
}
impl From<PAD_CONF_2_PAD_2_CONF_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD_CONF_2_PAD_2_CONF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD_CONF_2_PAD_2_CONF`"]
pub type PAD_CONF_2_PAD_2_CONF_R = crate::R<u8, PAD_CONF_2_PAD_2_CONF_A>;
impl PAD_CONF_2_PAD_2_CONF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD_CONF_2_PAD_2_CONF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD_CONF_2_PAD_2_CONF_A::PAD_CONF_2_PAD_2_CONF_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PAD_CONF_2_PAD_2_CONF_DEFAULT`"]
    #[inline(always)]
    pub fn is_pad_conf_2_pad_2_conf_default(&self) -> bool {
        *self == PAD_CONF_2_PAD_2_CONF_A::PAD_CONF_2_PAD_2_CONF_DEFAULT
    }
}
#[doc = "Write proxy for field `PAD_CONF_2_PAD_2_CONF`"]
pub struct PAD_CONF_2_PAD_2_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CONF_2_PAD_2_CONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD_CONF_2_PAD_2_CONF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pad_conf_2_pad_2_conf_default(self) -> &'a mut W {
        self.variant(PAD_CONF_2_PAD_2_CONF_A::PAD_CONF_2_PAD_2_CONF_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Configuration of GPIO pad 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD_CONF_1_PAD_1_CONF_A {
    #[doc = "0: `0`"]
    PAD_CONF_1_PAD_1_CONF_DEFAULT = 0,
}
impl From<PAD_CONF_1_PAD_1_CONF_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD_CONF_1_PAD_1_CONF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD_CONF_1_PAD_1_CONF`"]
pub type PAD_CONF_1_PAD_1_CONF_R = crate::R<u8, PAD_CONF_1_PAD_1_CONF_A>;
impl PAD_CONF_1_PAD_1_CONF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD_CONF_1_PAD_1_CONF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD_CONF_1_PAD_1_CONF_A::PAD_CONF_1_PAD_1_CONF_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PAD_CONF_1_PAD_1_CONF_DEFAULT`"]
    #[inline(always)]
    pub fn is_pad_conf_1_pad_1_conf_default(&self) -> bool {
        *self == PAD_CONF_1_PAD_1_CONF_A::PAD_CONF_1_PAD_1_CONF_DEFAULT
    }
}
#[doc = "Write proxy for field `PAD_CONF_1_PAD_1_CONF`"]
pub struct PAD_CONF_1_PAD_1_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CONF_1_PAD_1_CONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD_CONF_1_PAD_1_CONF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pad_conf_1_pad_1_conf_default(self) -> &'a mut W {
        self.variant(PAD_CONF_1_PAD_1_CONF_A::PAD_CONF_1_PAD_1_CONF_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Configuration of GPIO pad 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD_CONF_1_PAD_0_CONF_A {
    #[doc = "0: `0`"]
    PAD_CONF_1_PAD_0_CONF_DEFAULT = 0,
}
impl From<PAD_CONF_1_PAD_0_CONF_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD_CONF_1_PAD_0_CONF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD_CONF_1_PAD_0_CONF`"]
pub type PAD_CONF_1_PAD_0_CONF_R = crate::R<u8, PAD_CONF_1_PAD_0_CONF_A>;
impl PAD_CONF_1_PAD_0_CONF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD_CONF_1_PAD_0_CONF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD_CONF_1_PAD_0_CONF_A::PAD_CONF_1_PAD_0_CONF_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PAD_CONF_1_PAD_0_CONF_DEFAULT`"]
    #[inline(always)]
    pub fn is_pad_conf_1_pad_0_conf_default(&self) -> bool {
        *self == PAD_CONF_1_PAD_0_CONF_A::PAD_CONF_1_PAD_0_CONF_DEFAULT
    }
}
#[doc = "Write proxy for field `PAD_CONF_1_PAD_0_CONF`"]
pub struct PAD_CONF_1_PAD_0_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_CONF_1_PAD_0_CONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD_CONF_1_PAD_0_CONF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pad_conf_1_pad_0_conf_default(self) -> &'a mut W {
        self.variant(PAD_CONF_1_PAD_0_CONF_A::PAD_CONF_1_PAD_0_CONF_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "If set to 1, the pads are set to High-Z when the IRQ is not active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ_CONF_IRQ_HIGH_Z_A {
    #[doc = "0: `0`"]
    IRQ_CONF_IRQ_HIGH_Z_DEFAULT = 0,
}
impl From<IRQ_CONF_IRQ_HIGH_Z_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ_CONF_IRQ_HIGH_Z_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IRQ_CONF_IRQ_HIGH_Z`"]
pub type IRQ_CONF_IRQ_HIGH_Z_R = crate::R<bool, IRQ_CONF_IRQ_HIGH_Z_A>;
impl IRQ_CONF_IRQ_HIGH_Z_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, IRQ_CONF_IRQ_HIGH_Z_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(IRQ_CONF_IRQ_HIGH_Z_A::IRQ_CONF_IRQ_HIGH_Z_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IRQ_CONF_IRQ_HIGH_Z_DEFAULT`"]
    #[inline(always)]
    pub fn is_irq_conf_irq_high_z_default(&self) -> bool {
        *self == IRQ_CONF_IRQ_HIGH_Z_A::IRQ_CONF_IRQ_HIGH_Z_DEFAULT
    }
}
#[doc = "Write proxy for field `IRQ_CONF_IRQ_HIGH_Z`"]
pub struct IRQ_CONF_IRQ_HIGH_Z_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_CONF_IRQ_HIGH_Z_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQ_CONF_IRQ_HIGH_Z_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn irq_conf_irq_high_z_default(self) -> &'a mut W {
        self.variant(IRQ_CONF_IRQ_HIGH_Z_A::IRQ_CONF_IRQ_HIGH_Z_DEFAULT)
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
#[doc = "If set to 1, the IRQ are active low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ_CONF_IRQ_ACTIVE_LOW_A {
    #[doc = "0: `0`"]
    IRQ_CONF_IRQ_ACTIVE_LOW_DEFAULT = 0,
}
impl From<IRQ_CONF_IRQ_ACTIVE_LOW_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ_CONF_IRQ_ACTIVE_LOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IRQ_CONF_IRQ_ACTIVE_LOW`"]
pub type IRQ_CONF_IRQ_ACTIVE_LOW_R = crate::R<bool, IRQ_CONF_IRQ_ACTIVE_LOW_A>;
impl IRQ_CONF_IRQ_ACTIVE_LOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, IRQ_CONF_IRQ_ACTIVE_LOW_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(IRQ_CONF_IRQ_ACTIVE_LOW_A::IRQ_CONF_IRQ_ACTIVE_LOW_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IRQ_CONF_IRQ_ACTIVE_LOW_DEFAULT`"]
    #[inline(always)]
    pub fn is_irq_conf_irq_active_low_default(&self) -> bool {
        *self == IRQ_CONF_IRQ_ACTIVE_LOW_A::IRQ_CONF_IRQ_ACTIVE_LOW_DEFAULT
    }
}
#[doc = "Write proxy for field `IRQ_CONF_IRQ_ACTIVE_LOW`"]
pub struct IRQ_CONF_IRQ_ACTIVE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_CONF_IRQ_ACTIVE_LOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQ_CONF_IRQ_ACTIVE_LOW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn irq_conf_irq_active_low_default(self) -> &'a mut W {
        self.variant(IRQ_CONF_IRQ_ACTIVE_LOW_A::IRQ_CONF_IRQ_ACTIVE_LOW_DEFAULT)
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
#[doc = "Mask to determine which IRQs are enabled (active high)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IRQ_CONF_IRQS_MASK_A {
    #[doc = "0: `0`"]
    IRQ_CONF_IRQS_MASK_DEFAULT = 0,
}
impl From<IRQ_CONF_IRQS_MASK_A> for u8 {
    #[inline(always)]
    fn from(variant: IRQ_CONF_IRQS_MASK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IRQ_CONF_IRQS_MASK`"]
pub type IRQ_CONF_IRQS_MASK_R = crate::R<u8, IRQ_CONF_IRQS_MASK_A>;
impl IRQ_CONF_IRQS_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IRQ_CONF_IRQS_MASK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IRQ_CONF_IRQS_MASK_A::IRQ_CONF_IRQS_MASK_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IRQ_CONF_IRQS_MASK_DEFAULT`"]
    #[inline(always)]
    pub fn is_irq_conf_irqs_mask_default(&self) -> bool {
        *self == IRQ_CONF_IRQS_MASK_A::IRQ_CONF_IRQS_MASK_DEFAULT
    }
}
#[doc = "Write proxy for field `IRQ_CONF_IRQS_MASK`"]
pub struct IRQ_CONF_IRQS_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_CONF_IRQS_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQ_CONF_IRQS_MASK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn irq_conf_irqs_mask_default(self) -> &'a mut W {
        self.variant(IRQ_CONF_IRQS_MASK_A::IRQ_CONF_IRQS_MASK_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Threshold indicating the 'almost empty' state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FIFO_2_FIFO_THR_TX_A {
    #[doc = "0: `0`"]
    FIFO_2_FIFO_THR_TX_DEFAULT = 0,
}
impl From<FIFO_2_FIFO_THR_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: FIFO_2_FIFO_THR_TX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FIFO_2_FIFO_THR_TX`"]
pub type FIFO_2_FIFO_THR_TX_R = crate::R<u8, FIFO_2_FIFO_THR_TX_A>;
impl FIFO_2_FIFO_THR_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FIFO_2_FIFO_THR_TX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FIFO_2_FIFO_THR_TX_A::FIFO_2_FIFO_THR_TX_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_2_FIFO_THR_TX_DEFAULT`"]
    #[inline(always)]
    pub fn is_fifo_2_fifo_thr_tx_default(&self) -> bool {
        *self == FIFO_2_FIFO_THR_TX_A::FIFO_2_FIFO_THR_TX_DEFAULT
    }
}
#[doc = "Write proxy for field `FIFO_2_FIFO_THR_TX`"]
pub struct FIFO_2_FIFO_THR_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_2_FIFO_THR_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_2_FIFO_THR_TX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fifo_2_fifo_thr_tx_default(self) -> &'a mut W {
        self.variant(FIFO_2_FIFO_THR_TX_A::FIFO_2_FIFO_THR_TX_DEFAULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "If set to 1, the FSM will wait a Tx FIFO write before starting the Tx in case of an empty Tx FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_2_WAIT_TXFIFO_WR_A {
    #[doc = "0: `0`"]
    FIFO_2_WAIT_TXFIFO_WR_DEFAULT = 0,
}
impl From<FIFO_2_WAIT_TXFIFO_WR_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_2_WAIT_TXFIFO_WR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIFO_2_WAIT_TXFIFO_WR`"]
pub type FIFO_2_WAIT_TXFIFO_WR_R = crate::R<bool, FIFO_2_WAIT_TXFIFO_WR_A>;
impl FIFO_2_WAIT_TXFIFO_WR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FIFO_2_WAIT_TXFIFO_WR_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(FIFO_2_WAIT_TXFIFO_WR_A::FIFO_2_WAIT_TXFIFO_WR_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_2_WAIT_TXFIFO_WR_DEFAULT`"]
    #[inline(always)]
    pub fn is_fifo_2_wait_txfifo_wr_default(&self) -> bool {
        *self == FIFO_2_WAIT_TXFIFO_WR_A::FIFO_2_WAIT_TXFIFO_WR_DEFAULT
    }
}
#[doc = "Write proxy for field `FIFO_2_WAIT_TXFIFO_WR`"]
pub struct FIFO_2_WAIT_TXFIFO_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_2_WAIT_TXFIFO_WR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_2_WAIT_TXFIFO_WR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fifo_2_wait_txfifo_wr_default(self) -> &'a mut W {
        self.variant(FIFO_2_WAIT_TXFIFO_WR_A::FIFO_2_WAIT_TXFIFO_WR_DEFAULT)
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
#[doc = "If set to 1, stops the reception in case of a FIFO overflow.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_2_STOP_ON_RXFF_OVFLW_A {
    #[doc = "0: `0`"]
    FIFO_2_STOP_ON_RXFF_OVFLW_DEFAULT = 0,
}
impl From<FIFO_2_STOP_ON_RXFF_OVFLW_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_2_STOP_ON_RXFF_OVFLW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIFO_2_STOP_ON_RXFF_OVFLW`"]
pub type FIFO_2_STOP_ON_RXFF_OVFLW_R = crate::R<bool, FIFO_2_STOP_ON_RXFF_OVFLW_A>;
impl FIFO_2_STOP_ON_RXFF_OVFLW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FIFO_2_STOP_ON_RXFF_OVFLW_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(FIFO_2_STOP_ON_RXFF_OVFLW_A::FIFO_2_STOP_ON_RXFF_OVFLW_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_2_STOP_ON_RXFF_OVFLW_DEFAULT`"]
    #[inline(always)]
    pub fn is_fifo_2_stop_on_rxff_ovflw_default(&self) -> bool {
        *self == FIFO_2_STOP_ON_RXFF_OVFLW_A::FIFO_2_STOP_ON_RXFF_OVFLW_DEFAULT
    }
}
#[doc = "Write proxy for field `FIFO_2_STOP_ON_RXFF_OVFLW`"]
pub struct FIFO_2_STOP_ON_RXFF_OVFLW_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_2_STOP_ON_RXFF_OVFLW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_2_STOP_ON_RXFF_OVFLW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fifo_2_stop_on_rxff_ovflw_default(self) -> &'a mut W {
        self.variant(FIFO_2_STOP_ON_RXFF_OVFLW_A::FIFO_2_STOP_ON_RXFF_OVFLW_DEFAULT)
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
#[doc = "If set to 1, stops the transmission in case of a FIFO underflow.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_2_STOP_ON_TXFF_UNFLW_A {
    #[doc = "0: `0`"]
    FIFO_2_STOP_ON_TXFF_UNFLW_DEFAULT = 0,
}
impl From<FIFO_2_STOP_ON_TXFF_UNFLW_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_2_STOP_ON_TXFF_UNFLW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIFO_2_STOP_ON_TXFF_UNFLW`"]
pub type FIFO_2_STOP_ON_TXFF_UNFLW_R = crate::R<bool, FIFO_2_STOP_ON_TXFF_UNFLW_A>;
impl FIFO_2_STOP_ON_TXFF_UNFLW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FIFO_2_STOP_ON_TXFF_UNFLW_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(FIFO_2_STOP_ON_TXFF_UNFLW_A::FIFO_2_STOP_ON_TXFF_UNFLW_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_2_STOP_ON_TXFF_UNFLW_DEFAULT`"]
    #[inline(always)]
    pub fn is_fifo_2_stop_on_txff_unflw_default(&self) -> bool {
        *self == FIFO_2_STOP_ON_TXFF_UNFLW_A::FIFO_2_STOP_ON_TXFF_UNFLW_DEFAULT
    }
}
#[doc = "Write proxy for field `FIFO_2_STOP_ON_TXFF_UNFLW`"]
pub struct FIFO_2_STOP_ON_TXFF_UNFLW_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_2_STOP_ON_TXFF_UNFLW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_2_STOP_ON_TXFF_UNFLW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fifo_2_stop_on_txff_unflw_default(self) -> &'a mut W {
        self.variant(FIFO_2_STOP_ON_TXFF_UNFLW_A::FIFO_2_STOP_ON_TXFF_UNFLW_DEFAULT)
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
#[doc = "If set to 1, flushes the Rx FIFO when the Rx is enabled, in order to receive a packet with an empty FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_2_RXFF_FLUSH_ON_START_A {
    #[doc = "0: `0`"]
    FIFO_2_RXFF_FLUSH_ON_START_DEFAULT = 0,
}
impl From<FIFO_2_RXFF_FLUSH_ON_START_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_2_RXFF_FLUSH_ON_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIFO_2_RXFF_FLUSH_ON_START`"]
pub type FIFO_2_RXFF_FLUSH_ON_START_R = crate::R<bool, FIFO_2_RXFF_FLUSH_ON_START_A>;
impl FIFO_2_RXFF_FLUSH_ON_START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FIFO_2_RXFF_FLUSH_ON_START_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(FIFO_2_RXFF_FLUSH_ON_START_A::FIFO_2_RXFF_FLUSH_ON_START_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_2_RXFF_FLUSH_ON_START_DEFAULT`"]
    #[inline(always)]
    pub fn is_fifo_2_rxff_flush_on_start_default(&self) -> bool {
        *self == FIFO_2_RXFF_FLUSH_ON_START_A::FIFO_2_RXFF_FLUSH_ON_START_DEFAULT
    }
}
#[doc = "Write proxy for field `FIFO_2_RXFF_FLUSH_ON_START`"]
pub struct FIFO_2_RXFF_FLUSH_ON_START_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_2_RXFF_FLUSH_ON_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_2_RXFF_FLUSH_ON_START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fifo_2_rxff_flush_on_start_default(self) -> &'a mut W {
        self.variant(FIFO_2_RXFF_FLUSH_ON_START_A::FIFO_2_RXFF_FLUSH_ON_START_DEFAULT)
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
#[doc = "If set to 1, flushes the Tx FIFO after the end of a packet transmission in order to have an empty FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_2_TXFF_FLUSH_ON_STOP_A {
    #[doc = "0: `0`"]
    FIFO_2_TXFF_FLUSH_ON_STOP_DEFAULT = 0,
}
impl From<FIFO_2_TXFF_FLUSH_ON_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_2_TXFF_FLUSH_ON_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIFO_2_TXFF_FLUSH_ON_STOP`"]
pub type FIFO_2_TXFF_FLUSH_ON_STOP_R = crate::R<bool, FIFO_2_TXFF_FLUSH_ON_STOP_A>;
impl FIFO_2_TXFF_FLUSH_ON_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FIFO_2_TXFF_FLUSH_ON_STOP_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(FIFO_2_TXFF_FLUSH_ON_STOP_A::FIFO_2_TXFF_FLUSH_ON_STOP_DEFAULT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_2_TXFF_FLUSH_ON_STOP_DEFAULT`"]
    #[inline(always)]
    pub fn is_fifo_2_txff_flush_on_stop_default(&self) -> bool {
        *self == FIFO_2_TXFF_FLUSH_ON_STOP_A::FIFO_2_TXFF_FLUSH_ON_STOP_DEFAULT
    }
}
#[doc = "Write proxy for field `FIFO_2_TXFF_FLUSH_ON_STOP`"]
pub struct FIFO_2_TXFF_FLUSH_ON_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_2_TXFF_FLUSH_ON_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_2_TXFF_FLUSH_ON_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fifo_2_txff_flush_on_stop_default(self) -> &'a mut W {
        self.variant(FIFO_2_TXFF_FLUSH_ON_STOP_A::FIFO_2_TXFF_FLUSH_ON_STOP_DEFAULT)
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
    #[doc = "Bits 28:31 - Configuration of GPIO pad 3"]
    #[inline(always)]
    pub fn pad_conf_2_pad_3_conf(&self) -> PAD_CONF_2_PAD_3_CONF_R {
        PAD_CONF_2_PAD_3_CONF_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Configuration of GPIO pad 2"]
    #[inline(always)]
    pub fn pad_conf_2_pad_2_conf(&self) -> PAD_CONF_2_PAD_2_CONF_R {
        PAD_CONF_2_PAD_2_CONF_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Configuration of GPIO pad 1"]
    #[inline(always)]
    pub fn pad_conf_1_pad_1_conf(&self) -> PAD_CONF_1_PAD_1_CONF_R {
        PAD_CONF_1_PAD_1_CONF_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Configuration of GPIO pad 0"]
    #[inline(always)]
    pub fn pad_conf_1_pad_0_conf(&self) -> PAD_CONF_1_PAD_0_CONF_R {
        PAD_CONF_1_PAD_0_CONF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - If set to 1, the pads are set to High-Z when the IRQ is not active."]
    #[inline(always)]
    pub fn irq_conf_irq_high_z(&self) -> IRQ_CONF_IRQ_HIGH_Z_R {
        IRQ_CONF_IRQ_HIGH_Z_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - If set to 1, the IRQ are active low"]
    #[inline(always)]
    pub fn irq_conf_irq_active_low(&self) -> IRQ_CONF_IRQ_ACTIVE_LOW_R {
        IRQ_CONF_IRQ_ACTIVE_LOW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Mask to determine which IRQs are enabled (active high)"]
    #[inline(always)]
    pub fn irq_conf_irqs_mask(&self) -> IRQ_CONF_IRQS_MASK_R {
        IRQ_CONF_IRQS_MASK_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 5:7 - Threshold indicating the 'almost empty' state"]
    #[inline(always)]
    pub fn fifo_2_fifo_thr_tx(&self) -> FIFO_2_FIFO_THR_TX_R {
        FIFO_2_FIFO_THR_TX_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4 - If set to 1, the FSM will wait a Tx FIFO write before starting the Tx in case of an empty Tx FIFO."]
    #[inline(always)]
    pub fn fifo_2_wait_txfifo_wr(&self) -> FIFO_2_WAIT_TXFIFO_WR_R {
        FIFO_2_WAIT_TXFIFO_WR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If set to 1, stops the reception in case of a FIFO overflow."]
    #[inline(always)]
    pub fn fifo_2_stop_on_rxff_ovflw(&self) -> FIFO_2_STOP_ON_RXFF_OVFLW_R {
        FIFO_2_STOP_ON_RXFF_OVFLW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If set to 1, stops the transmission in case of a FIFO underflow."]
    #[inline(always)]
    pub fn fifo_2_stop_on_txff_unflw(&self) -> FIFO_2_STOP_ON_TXFF_UNFLW_R {
        FIFO_2_STOP_ON_TXFF_UNFLW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - If set to 1, flushes the Rx FIFO when the Rx is enabled, in order to receive a packet with an empty FIFO."]
    #[inline(always)]
    pub fn fifo_2_rxff_flush_on_start(&self) -> FIFO_2_RXFF_FLUSH_ON_START_R {
        FIFO_2_RXFF_FLUSH_ON_START_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - If set to 1, flushes the Tx FIFO after the end of a packet transmission in order to have an empty FIFO."]
    #[inline(always)]
    pub fn fifo_2_txff_flush_on_stop(&self) -> FIFO_2_TXFF_FLUSH_ON_STOP_R {
        FIFO_2_TXFF_FLUSH_ON_STOP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31 - Configuration of GPIO pad 3"]
    #[inline(always)]
    pub fn pad_conf_2_pad_3_conf(&mut self) -> PAD_CONF_2_PAD_3_CONF_W {
        PAD_CONF_2_PAD_3_CONF_W { w: self }
    }
    #[doc = "Bits 24:27 - Configuration of GPIO pad 2"]
    #[inline(always)]
    pub fn pad_conf_2_pad_2_conf(&mut self) -> PAD_CONF_2_PAD_2_CONF_W {
        PAD_CONF_2_PAD_2_CONF_W { w: self }
    }
    #[doc = "Bits 20:23 - Configuration of GPIO pad 1"]
    #[inline(always)]
    pub fn pad_conf_1_pad_1_conf(&mut self) -> PAD_CONF_1_PAD_1_CONF_W {
        PAD_CONF_1_PAD_1_CONF_W { w: self }
    }
    #[doc = "Bits 16:19 - Configuration of GPIO pad 0"]
    #[inline(always)]
    pub fn pad_conf_1_pad_0_conf(&mut self) -> PAD_CONF_1_PAD_0_CONF_W {
        PAD_CONF_1_PAD_0_CONF_W { w: self }
    }
    #[doc = "Bit 15 - If set to 1, the pads are set to High-Z when the IRQ is not active."]
    #[inline(always)]
    pub fn irq_conf_irq_high_z(&mut self) -> IRQ_CONF_IRQ_HIGH_Z_W {
        IRQ_CONF_IRQ_HIGH_Z_W { w: self }
    }
    #[doc = "Bit 14 - If set to 1, the IRQ are active low"]
    #[inline(always)]
    pub fn irq_conf_irq_active_low(&mut self) -> IRQ_CONF_IRQ_ACTIVE_LOW_W {
        IRQ_CONF_IRQ_ACTIVE_LOW_W { w: self }
    }
    #[doc = "Bits 8:13 - Mask to determine which IRQs are enabled (active high)"]
    #[inline(always)]
    pub fn irq_conf_irqs_mask(&mut self) -> IRQ_CONF_IRQS_MASK_W {
        IRQ_CONF_IRQS_MASK_W { w: self }
    }
    #[doc = "Bits 5:7 - Threshold indicating the 'almost empty' state"]
    #[inline(always)]
    pub fn fifo_2_fifo_thr_tx(&mut self) -> FIFO_2_FIFO_THR_TX_W {
        FIFO_2_FIFO_THR_TX_W { w: self }
    }
    #[doc = "Bit 4 - If set to 1, the FSM will wait a Tx FIFO write before starting the Tx in case of an empty Tx FIFO."]
    #[inline(always)]
    pub fn fifo_2_wait_txfifo_wr(&mut self) -> FIFO_2_WAIT_TXFIFO_WR_W {
        FIFO_2_WAIT_TXFIFO_WR_W { w: self }
    }
    #[doc = "Bit 3 - If set to 1, stops the reception in case of a FIFO overflow."]
    #[inline(always)]
    pub fn fifo_2_stop_on_rxff_ovflw(&mut self) -> FIFO_2_STOP_ON_RXFF_OVFLW_W {
        FIFO_2_STOP_ON_RXFF_OVFLW_W { w: self }
    }
    #[doc = "Bit 2 - If set to 1, stops the transmission in case of a FIFO underflow."]
    #[inline(always)]
    pub fn fifo_2_stop_on_txff_unflw(&mut self) -> FIFO_2_STOP_ON_TXFF_UNFLW_W {
        FIFO_2_STOP_ON_TXFF_UNFLW_W { w: self }
    }
    #[doc = "Bit 1 - If set to 1, flushes the Rx FIFO when the Rx is enabled, in order to receive a packet with an empty FIFO."]
    #[inline(always)]
    pub fn fifo_2_rxff_flush_on_start(&mut self) -> FIFO_2_RXFF_FLUSH_ON_START_W {
        FIFO_2_RXFF_FLUSH_ON_START_W { w: self }
    }
    #[doc = "Bit 0 - If set to 1, flushes the Tx FIFO after the end of a packet transmission in order to have an empty FIFO."]
    #[inline(always)]
    pub fn fifo_2_txff_flush_on_stop(&mut self) -> FIFO_2_TXFF_FLUSH_ON_STOP_W {
        FIFO_2_TXFF_FLUSH_ON_STOP_W { w: self }
    }
}
