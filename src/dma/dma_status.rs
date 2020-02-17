#[doc = "Reader of register DMA_STATUS[%s]"]
pub type R = crate::R<u32, super::DMA_STATUS>;
#[doc = "Writer for register DMA_STATUS[%s]"]
pub type W = crate::W<u32, super::DMA_STATUS>;
#[doc = "Register DMA_STATUS[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clear the state machine error interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_INT_CLEAR_AW {
    #[doc = "1: Clear the state machine error interrupt flag"]
    DMA_ERROR_INT_CLEAR = 1,
}
impl From<ERROR_INT_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: ERROR_INT_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ERROR_INT_CLEAR`"]
pub struct ERROR_INT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_INT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROR_INT_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the state machine error interrupt flag"]
    #[inline(always)]
    pub fn dma_error_int_clear(self) -> &'a mut W {
        self.variant(ERROR_INT_CLEAR_AW::DMA_ERROR_INT_CLEAR)
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
#[doc = "Clear the complete interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPLETE_INT_CLEAR_AW {
    #[doc = "1: Clear the complete interrupt flag"]
    DMA_COMPLETE_INT_CLEAR = 1,
}
impl From<COMPLETE_INT_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPLETE_INT_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `COMPLETE_INT_CLEAR`"]
pub struct COMPLETE_INT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPLETE_INT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPLETE_INT_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the complete interrupt flag"]
    #[inline(always)]
    pub fn dma_complete_int_clear(self) -> &'a mut W {
        self.variant(COMPLETE_INT_CLEAR_AW::DMA_COMPLETE_INT_CLEAR)
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
#[doc = "Clear the counter interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COUNTER_INT_CLEAR_AW {
    #[doc = "1: Clear the counter interrupt flag"]
    DMA_COUNTER_INT_CLEAR = 1,
}
impl From<COUNTER_INT_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: COUNTER_INT_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `COUNTER_INT_CLEAR`"]
pub struct COUNTER_INT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_INT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COUNTER_INT_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the counter interrupt flag"]
    #[inline(always)]
    pub fn dma_counter_int_clear(self) -> &'a mut W {
        self.variant(COUNTER_INT_CLEAR_AW::DMA_COUNTER_INT_CLEAR)
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
#[doc = "Clear the start interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_INT_CLEAR_AW {
    #[doc = "1: Clear the start interrupt flag"]
    DMA_START_INT_CLEAR = 1,
}
impl From<START_INT_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: START_INT_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `START_INT_CLEAR`"]
pub struct START_INT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> START_INT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_INT_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the start interrupt flag"]
    #[inline(always)]
    pub fn dma_start_int_clear(self) -> &'a mut W {
        self.variant(START_INT_CLEAR_AW::DMA_START_INT_CLEAR)
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
#[doc = "Clear the channel disable flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLE_INT_CLEAR_AW {
    #[doc = "1: Clear the channel disable flag"]
    DMA_DISABLE_INT_CLEAR = 1,
}
impl From<DISABLE_INT_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: DISABLE_INT_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DISABLE_INT_CLEAR`"]
pub struct DISABLE_INT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_INT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLE_INT_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the channel disable flag"]
    #[inline(always)]
    pub fn dma_disable_int_clear(self) -> &'a mut W {
        self.variant(DISABLE_INT_CLEAR_AW::DMA_DISABLE_INT_CLEAR)
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
#[doc = "DMA channel state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "0: DMA channel is disabled or waits for both source and destination to be ready"]
    DMA_IDLE = 0,
    #[doc = "1: DMA channel waits for both source and destination to be ready"]
    DMA_WAIT_SRC_DST_RDY = 1,
    #[doc = "2: DMA channel waits for source to be ready"]
    DMA_WAIT_SRC_RDY = 2,
    #[doc = "3: DMA channel waits for read operation to be completed"]
    DMA_WAIT_RD_COMP = 3,
    #[doc = "4: DMA channel checks for destinations to be ready"]
    DMA_CHK_DST = 4,
    #[doc = "5: DMA channel gets disabled"]
    DMA_DIS_CHAN = 5,
    #[doc = "6: DMA channel waits for destination to be ready"]
    DMA_WAIT_DST_RDY = 6,
    #[doc = "7: DMA channel waits for write operation to be completed"]
    DMA_WAIT_WR_COMP = 7,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, STATE_A>;
impl STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATE_A {
        match self.bits {
            0 => STATE_A::DMA_IDLE,
            1 => STATE_A::DMA_WAIT_SRC_DST_RDY,
            2 => STATE_A::DMA_WAIT_SRC_RDY,
            3 => STATE_A::DMA_WAIT_RD_COMP,
            4 => STATE_A::DMA_CHK_DST,
            5 => STATE_A::DMA_DIS_CHAN,
            6 => STATE_A::DMA_WAIT_DST_RDY,
            7 => STATE_A::DMA_WAIT_WR_COMP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_IDLE`"]
    #[inline(always)]
    pub fn is_dma_idle(&self) -> bool {
        *self == STATE_A::DMA_IDLE
    }
    #[doc = "Checks if the value of the field is `DMA_WAIT_SRC_DST_RDY`"]
    #[inline(always)]
    pub fn is_dma_wait_src_dst_rdy(&self) -> bool {
        *self == STATE_A::DMA_WAIT_SRC_DST_RDY
    }
    #[doc = "Checks if the value of the field is `DMA_WAIT_SRC_RDY`"]
    #[inline(always)]
    pub fn is_dma_wait_src_rdy(&self) -> bool {
        *self == STATE_A::DMA_WAIT_SRC_RDY
    }
    #[doc = "Checks if the value of the field is `DMA_WAIT_RD_COMP`"]
    #[inline(always)]
    pub fn is_dma_wait_rd_comp(&self) -> bool {
        *self == STATE_A::DMA_WAIT_RD_COMP
    }
    #[doc = "Checks if the value of the field is `DMA_CHK_DST`"]
    #[inline(always)]
    pub fn is_dma_chk_dst(&self) -> bool {
        *self == STATE_A::DMA_CHK_DST
    }
    #[doc = "Checks if the value of the field is `DMA_DIS_CHAN`"]
    #[inline(always)]
    pub fn is_dma_dis_chan(&self) -> bool {
        *self == STATE_A::DMA_DIS_CHAN
    }
    #[doc = "Checks if the value of the field is `DMA_WAIT_DST_RDY`"]
    #[inline(always)]
    pub fn is_dma_wait_dst_rdy(&self) -> bool {
        *self == STATE_A::DMA_WAIT_DST_RDY
    }
    #[doc = "Checks if the value of the field is `DMA_WAIT_WR_COMP`"]
    #[inline(always)]
    pub fn is_dma_wait_wr_comp(&self) -> bool {
        *self == STATE_A::DMA_WAIT_WR_COMP
    }
}
#[doc = "Indicate if a state machine error interrupt has occurred on DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_INT_STATUS_A {
    #[doc = "1: Indicate that a state machine error interrupt has occurred"]
    DMA_ERROR_INT_STATUS = 1,
}
impl From<ERROR_INT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_INT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERROR_INT_STATUS`"]
pub type ERROR_INT_STATUS_R = crate::R<bool, ERROR_INT_STATUS_A>;
impl ERROR_INT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ERROR_INT_STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(ERROR_INT_STATUS_A::DMA_ERROR_INT_STATUS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_ERROR_INT_STATUS`"]
    #[inline(always)]
    pub fn is_dma_error_int_status(&self) -> bool {
        *self == ERROR_INT_STATUS_A::DMA_ERROR_INT_STATUS
    }
}
#[doc = "Indicate if a complete interrupt has occurred on DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPLETE_INT_STATUS_A {
    #[doc = "1: Indicate that a complete interrupt has occurred"]
    DMA_COMPLETE_INT_STATUS = 1,
}
impl From<COMPLETE_INT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: COMPLETE_INT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPLETE_INT_STATUS`"]
pub type COMPLETE_INT_STATUS_R = crate::R<bool, COMPLETE_INT_STATUS_A>;
impl COMPLETE_INT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, COMPLETE_INT_STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(COMPLETE_INT_STATUS_A::DMA_COMPLETE_INT_STATUS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_COMPLETE_INT_STATUS`"]
    #[inline(always)]
    pub fn is_dma_complete_int_status(&self) -> bool {
        *self == COMPLETE_INT_STATUS_A::DMA_COMPLETE_INT_STATUS
    }
}
#[doc = "Indicate if a counter interrupt has occurred on DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COUNTER_INT_STATUS_A {
    #[doc = "1: Indicate that a counter interrupt has occurred"]
    DMA_COUNTER_INT_STATUS = 1,
}
impl From<COUNTER_INT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: COUNTER_INT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COUNTER_INT_STATUS`"]
pub type COUNTER_INT_STATUS_R = crate::R<bool, COUNTER_INT_STATUS_A>;
impl COUNTER_INT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, COUNTER_INT_STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(COUNTER_INT_STATUS_A::DMA_COUNTER_INT_STATUS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_COUNTER_INT_STATUS`"]
    #[inline(always)]
    pub fn is_dma_counter_int_status(&self) -> bool {
        *self == COUNTER_INT_STATUS_A::DMA_COUNTER_INT_STATUS
    }
}
#[doc = "Indicate if a start interrupt has occurred on DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_INT_STATUS_A {
    #[doc = "1: Indicate that a start interrupt has occurred"]
    DMA_START_INT_STATUS = 1,
}
impl From<START_INT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: START_INT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `START_INT_STATUS`"]
pub type START_INT_STATUS_R = crate::R<bool, START_INT_STATUS_A>;
impl START_INT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, START_INT_STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(START_INT_STATUS_A::DMA_START_INT_STATUS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_START_INT_STATUS`"]
    #[inline(always)]
    pub fn is_dma_start_int_status(&self) -> bool {
        *self == START_INT_STATUS_A::DMA_START_INT_STATUS
    }
}
#[doc = "Indicate if a channel disable interrupt has occurred on DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLE_INT_STATUS_A {
    #[doc = "1: Indicate that a channel disable interrupt has occurred"]
    DMA_DISABLE_INT_STATUS = 1,
}
impl From<DISABLE_INT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLE_INT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISABLE_INT_STATUS`"]
pub type DISABLE_INT_STATUS_R = crate::R<bool, DISABLE_INT_STATUS_A>;
impl DISABLE_INT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DISABLE_INT_STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(DISABLE_INT_STATUS_A::DMA_DISABLE_INT_STATUS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_DISABLE_INT_STATUS`"]
    #[inline(always)]
    pub fn is_dma_disable_int_status(&self) -> bool {
        *self == DISABLE_INT_STATUS_A::DMA_DISABLE_INT_STATUS
    }
}
impl R {
    #[doc = "Bits 5:7 - DMA channel state"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4 - Indicate if a state machine error interrupt has occurred on DMA channel"]
    #[inline(always)]
    pub fn error_int_status(&self) -> ERROR_INT_STATUS_R {
        ERROR_INT_STATUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicate if a complete interrupt has occurred on DMA channel"]
    #[inline(always)]
    pub fn complete_int_status(&self) -> COMPLETE_INT_STATUS_R {
        COMPLETE_INT_STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicate if a counter interrupt has occurred on DMA channel"]
    #[inline(always)]
    pub fn counter_int_status(&self) -> COUNTER_INT_STATUS_R {
        COUNTER_INT_STATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicate if a start interrupt has occurred on DMA channel"]
    #[inline(always)]
    pub fn start_int_status(&self) -> START_INT_STATUS_R {
        START_INT_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indicate if a channel disable interrupt has occurred on DMA channel"]
    #[inline(always)]
    pub fn disable_int_status(&self) -> DISABLE_INT_STATUS_R {
        DISABLE_INT_STATUS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Clear the state machine error interrupt flag"]
    #[inline(always)]
    pub fn error_int_clear(&mut self) -> ERROR_INT_CLEAR_W {
        ERROR_INT_CLEAR_W { w: self }
    }
    #[doc = "Bit 11 - Clear the complete interrupt flag"]
    #[inline(always)]
    pub fn complete_int_clear(&mut self) -> COMPLETE_INT_CLEAR_W {
        COMPLETE_INT_CLEAR_W { w: self }
    }
    #[doc = "Bit 10 - Clear the counter interrupt flag"]
    #[inline(always)]
    pub fn counter_int_clear(&mut self) -> COUNTER_INT_CLEAR_W {
        COUNTER_INT_CLEAR_W { w: self }
    }
    #[doc = "Bit 9 - Clear the start interrupt flag"]
    #[inline(always)]
    pub fn start_int_clear(&mut self) -> START_INT_CLEAR_W {
        START_INT_CLEAR_W { w: self }
    }
    #[doc = "Bit 8 - Clear the channel disable flag"]
    #[inline(always)]
    pub fn disable_int_clear(&mut self) -> DISABLE_INT_CLEAR_W {
        DISABLE_INT_CLEAR_W { w: self }
    }
}
