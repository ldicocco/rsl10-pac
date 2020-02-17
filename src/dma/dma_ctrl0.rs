#[doc = "Reader of register DMA_CTRL0[%s]"]
pub type R = crate::R<u32, super::DMA_CTRL0>;
#[doc = "Writer for register DMA_CTRL0[%s]"]
pub type W = crate::W<u32, super::DMA_CTRL0>;
#[doc = "Register DMA_CTRL0[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select the destination address step size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEST_ADDR_STEP_SIZE_A {
    #[doc = "0: Set the step size of DMA channel to 1"]
    DMA_DEST_ADDR_STEP_SIZE_1 = 0,
    #[doc = "1: Set the step size of DMA channel to 2"]
    DMA_DEST_ADDR_STEP_SIZE_2 = 1,
    #[doc = "2: Set the step size of DMA channel to 3"]
    DMA_DEST_ADDR_STEP_SIZE_3 = 2,
    #[doc = "3: Set the step size of DMA channel to 4"]
    DMA_DEST_ADDR_STEP_SIZE_4 = 3,
}
impl From<DEST_ADDR_STEP_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DEST_ADDR_STEP_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DEST_ADDR_STEP_SIZE`"]
pub type DEST_ADDR_STEP_SIZE_R = crate::R<u8, DEST_ADDR_STEP_SIZE_A>;
impl DEST_ADDR_STEP_SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEST_ADDR_STEP_SIZE_A {
        match self.bits {
            0 => DEST_ADDR_STEP_SIZE_A::DMA_DEST_ADDR_STEP_SIZE_1,
            1 => DEST_ADDR_STEP_SIZE_A::DMA_DEST_ADDR_STEP_SIZE_2,
            2 => DEST_ADDR_STEP_SIZE_A::DMA_DEST_ADDR_STEP_SIZE_3,
            3 => DEST_ADDR_STEP_SIZE_A::DMA_DEST_ADDR_STEP_SIZE_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_ADDR_STEP_SIZE_1`"]
    #[inline(always)]
    pub fn is_dma_dest_addr_step_size_1(&self) -> bool {
        *self == DEST_ADDR_STEP_SIZE_A::DMA_DEST_ADDR_STEP_SIZE_1
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_ADDR_STEP_SIZE_2`"]
    #[inline(always)]
    pub fn is_dma_dest_addr_step_size_2(&self) -> bool {
        *self == DEST_ADDR_STEP_SIZE_A::DMA_DEST_ADDR_STEP_SIZE_2
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_ADDR_STEP_SIZE_3`"]
    #[inline(always)]
    pub fn is_dma_dest_addr_step_size_3(&self) -> bool {
        *self == DEST_ADDR_STEP_SIZE_A::DMA_DEST_ADDR_STEP_SIZE_3
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_ADDR_STEP_SIZE_4`"]
    #[inline(always)]
    pub fn is_dma_dest_addr_step_size_4(&self) -> bool {
        *self == DEST_ADDR_STEP_SIZE_A::DMA_DEST_ADDR_STEP_SIZE_4
    }
}
#[doc = "Write proxy for field `DEST_ADDR_STEP_SIZE`"]
pub struct DEST_ADDR_STEP_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEST_ADDR_STEP_SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEST_ADDR_STEP_SIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Set the step size of DMA channel to 1"]
    #[inline(always)]
    pub fn dma_dest_addr_step_size_1(self) -> &'a mut W {
        self.variant(DEST_ADDR_STEP_SIZE_A::DMA_DEST_ADDR_STEP_SIZE_1)
    }
    #[doc = "Set the step size of DMA channel to 2"]
    #[inline(always)]
    pub fn dma_dest_addr_step_size_2(self) -> &'a mut W {
        self.variant(DEST_ADDR_STEP_SIZE_A::DMA_DEST_ADDR_STEP_SIZE_2)
    }
    #[doc = "Set the step size of DMA channel to 3"]
    #[inline(always)]
    pub fn dma_dest_addr_step_size_3(self) -> &'a mut W {
        self.variant(DEST_ADDR_STEP_SIZE_A::DMA_DEST_ADDR_STEP_SIZE_3)
    }
    #[doc = "Set the step size of DMA channel to 4"]
    #[inline(always)]
    pub fn dma_dest_addr_step_size_4(self) -> &'a mut W {
        self.variant(DEST_ADDR_STEP_SIZE_A::DMA_DEST_ADDR_STEP_SIZE_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Select the source address step size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_ADDR_STEP_SIZE_A {
    #[doc = "0: Set the step size of DMA channel to 1"]
    DMA_SRC_ADDR_STEP_SIZE_1 = 0,
    #[doc = "1: Set the step size of DMA channel to 2"]
    DMA_SRC_ADDR_STEP_SIZE_2 = 1,
    #[doc = "2: Set the step size of DMA channel to 3"]
    DMA_SRC_ADDR_STEP_SIZE_3 = 2,
    #[doc = "3: Set the step size of DMA channel to 4"]
    DMA_SRC_ADDR_STEP_SIZE_4 = 3,
}
impl From<SRC_ADDR_STEP_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_ADDR_STEP_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC_ADDR_STEP_SIZE`"]
pub type SRC_ADDR_STEP_SIZE_R = crate::R<u8, SRC_ADDR_STEP_SIZE_A>;
impl SRC_ADDR_STEP_SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_ADDR_STEP_SIZE_A {
        match self.bits {
            0 => SRC_ADDR_STEP_SIZE_A::DMA_SRC_ADDR_STEP_SIZE_1,
            1 => SRC_ADDR_STEP_SIZE_A::DMA_SRC_ADDR_STEP_SIZE_2,
            2 => SRC_ADDR_STEP_SIZE_A::DMA_SRC_ADDR_STEP_SIZE_3,
            3 => SRC_ADDR_STEP_SIZE_A::DMA_SRC_ADDR_STEP_SIZE_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_ADDR_STEP_SIZE_1`"]
    #[inline(always)]
    pub fn is_dma_src_addr_step_size_1(&self) -> bool {
        *self == SRC_ADDR_STEP_SIZE_A::DMA_SRC_ADDR_STEP_SIZE_1
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_ADDR_STEP_SIZE_2`"]
    #[inline(always)]
    pub fn is_dma_src_addr_step_size_2(&self) -> bool {
        *self == SRC_ADDR_STEP_SIZE_A::DMA_SRC_ADDR_STEP_SIZE_2
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_ADDR_STEP_SIZE_3`"]
    #[inline(always)]
    pub fn is_dma_src_addr_step_size_3(&self) -> bool {
        *self == SRC_ADDR_STEP_SIZE_A::DMA_SRC_ADDR_STEP_SIZE_3
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_ADDR_STEP_SIZE_4`"]
    #[inline(always)]
    pub fn is_dma_src_addr_step_size_4(&self) -> bool {
        *self == SRC_ADDR_STEP_SIZE_A::DMA_SRC_ADDR_STEP_SIZE_4
    }
}
#[doc = "Write proxy for field `SRC_ADDR_STEP_SIZE`"]
pub struct SRC_ADDR_STEP_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_ADDR_STEP_SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_ADDR_STEP_SIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Set the step size of DMA channel to 1"]
    #[inline(always)]
    pub fn dma_src_addr_step_size_1(self) -> &'a mut W {
        self.variant(SRC_ADDR_STEP_SIZE_A::DMA_SRC_ADDR_STEP_SIZE_1)
    }
    #[doc = "Set the step size of DMA channel to 2"]
    #[inline(always)]
    pub fn dma_src_addr_step_size_2(self) -> &'a mut W {
        self.variant(SRC_ADDR_STEP_SIZE_A::DMA_SRC_ADDR_STEP_SIZE_2)
    }
    #[doc = "Set the step size of DMA channel to 3"]
    #[inline(always)]
    pub fn dma_src_addr_step_size_3(self) -> &'a mut W {
        self.variant(SRC_ADDR_STEP_SIZE_A::DMA_SRC_ADDR_STEP_SIZE_3)
    }
    #[doc = "Set the step size of DMA channel to 4"]
    #[inline(always)]
    pub fn dma_src_addr_step_size_4(self) -> &'a mut W {
        self.variant(SRC_ADDR_STEP_SIZE_A::DMA_SRC_ADDR_STEP_SIZE_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Configure the destination address to either increment or decrement\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEST_ADDR_STEP_MODE_A {
    #[doc = "0: Increment the destination address used by DMA channel"]
    DMA_DEST_ADDR_POS = 0,
    #[doc = "1: Decrement destination address used by DMA channel"]
    DMA_DEST_ADDR_NEG = 1,
}
impl From<DEST_ADDR_STEP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: DEST_ADDR_STEP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEST_ADDR_STEP_MODE`"]
pub type DEST_ADDR_STEP_MODE_R = crate::R<bool, DEST_ADDR_STEP_MODE_A>;
impl DEST_ADDR_STEP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEST_ADDR_STEP_MODE_A {
        match self.bits {
            false => DEST_ADDR_STEP_MODE_A::DMA_DEST_ADDR_POS,
            true => DEST_ADDR_STEP_MODE_A::DMA_DEST_ADDR_NEG,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_ADDR_POS`"]
    #[inline(always)]
    pub fn is_dma_dest_addr_pos(&self) -> bool {
        *self == DEST_ADDR_STEP_MODE_A::DMA_DEST_ADDR_POS
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_ADDR_NEG`"]
    #[inline(always)]
    pub fn is_dma_dest_addr_neg(&self) -> bool {
        *self == DEST_ADDR_STEP_MODE_A::DMA_DEST_ADDR_NEG
    }
}
#[doc = "Write proxy for field `DEST_ADDR_STEP_MODE`"]
pub struct DEST_ADDR_STEP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEST_ADDR_STEP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEST_ADDR_STEP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Increment the destination address used by DMA channel"]
    #[inline(always)]
    pub fn dma_dest_addr_pos(self) -> &'a mut W {
        self.variant(DEST_ADDR_STEP_MODE_A::DMA_DEST_ADDR_POS)
    }
    #[doc = "Decrement destination address used by DMA channel"]
    #[inline(always)]
    pub fn dma_dest_addr_neg(self) -> &'a mut W {
        self.variant(DEST_ADDR_STEP_MODE_A::DMA_DEST_ADDR_NEG)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Configure the source address to either increment or decrement\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_ADDR_STEP_MODE_A {
    #[doc = "0: Increment the source address used by DMA channel"]
    DMA_SRC_ADDR_POS = 0,
    #[doc = "1: Decrement source address used by DMA channel"]
    DMA_SRC_ADDR_NEG = 1,
}
impl From<SRC_ADDR_STEP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_ADDR_STEP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRC_ADDR_STEP_MODE`"]
pub type SRC_ADDR_STEP_MODE_R = crate::R<bool, SRC_ADDR_STEP_MODE_A>;
impl SRC_ADDR_STEP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_ADDR_STEP_MODE_A {
        match self.bits {
            false => SRC_ADDR_STEP_MODE_A::DMA_SRC_ADDR_POS,
            true => SRC_ADDR_STEP_MODE_A::DMA_SRC_ADDR_NEG,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_ADDR_POS`"]
    #[inline(always)]
    pub fn is_dma_src_addr_pos(&self) -> bool {
        *self == SRC_ADDR_STEP_MODE_A::DMA_SRC_ADDR_POS
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_ADDR_NEG`"]
    #[inline(always)]
    pub fn is_dma_src_addr_neg(&self) -> bool {
        *self == SRC_ADDR_STEP_MODE_A::DMA_SRC_ADDR_NEG
    }
}
#[doc = "Write proxy for field `SRC_ADDR_STEP_MODE`"]
pub struct SRC_ADDR_STEP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_ADDR_STEP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_ADDR_STEP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Increment the source address used by DMA channel"]
    #[inline(always)]
    pub fn dma_src_addr_pos(self) -> &'a mut W {
        self.variant(SRC_ADDR_STEP_MODE_A::DMA_SRC_ADDR_POS)
    }
    #[doc = "Decrement source address used by DMA channel"]
    #[inline(always)]
    pub fn dma_src_addr_neg(self) -> &'a mut W {
        self.variant(SRC_ADDR_STEP_MODE_A::DMA_SRC_ADDR_NEG)
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
#[doc = "Select the byte ordering for the DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYTE_ORDER_A {
    #[doc = "0: DMA channel uses little endian mode"]
    DMA_LITTLE_ENDIAN = 0,
    #[doc = "1: DMA channel uses big endian mode"]
    DMA_BIG_ENDIAN = 1,
}
impl From<BYTE_ORDER_A> for bool {
    #[inline(always)]
    fn from(variant: BYTE_ORDER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BYTE_ORDER`"]
pub type BYTE_ORDER_R = crate::R<bool, BYTE_ORDER_A>;
impl BYTE_ORDER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYTE_ORDER_A {
        match self.bits {
            false => BYTE_ORDER_A::DMA_LITTLE_ENDIAN,
            true => BYTE_ORDER_A::DMA_BIG_ENDIAN,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_LITTLE_ENDIAN`"]
    #[inline(always)]
    pub fn is_dma_little_endian(&self) -> bool {
        *self == BYTE_ORDER_A::DMA_LITTLE_ENDIAN
    }
    #[doc = "Checks if the value of the field is `DMA_BIG_ENDIAN`"]
    #[inline(always)]
    pub fn is_dma_big_endian(&self) -> bool {
        *self == BYTE_ORDER_A::DMA_BIG_ENDIAN
    }
}
#[doc = "Write proxy for field `BYTE_ORDER`"]
pub struct BYTE_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_ORDER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYTE_ORDER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA channel uses little endian mode"]
    #[inline(always)]
    pub fn dma_little_endian(self) -> &'a mut W {
        self.variant(BYTE_ORDER_A::DMA_LITTLE_ENDIAN)
    }
    #[doc = "DMA channel uses big endian mode"]
    #[inline(always)]
    pub fn dma_big_endian(self) -> &'a mut W {
        self.variant(BYTE_ORDER_A::DMA_BIG_ENDIAN)
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
#[doc = "Raise an interrupt when the DMA channel is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLE_INT_ENABLE_A {
    #[doc = "0: Disable channel disable indicator interrupts for DMA channel"]
    DMA_DISABLE_INT_DISABLE = 0,
    #[doc = "1: Enable channel disable indicator interrupts for DMA channel"]
    DMA_DISABLE_INT_ENABLE = 1,
}
impl From<DISABLE_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLE_INT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISABLE_INT_ENABLE`"]
pub type DISABLE_INT_ENABLE_R = crate::R<bool, DISABLE_INT_ENABLE_A>;
impl DISABLE_INT_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLE_INT_ENABLE_A {
        match self.bits {
            false => DISABLE_INT_ENABLE_A::DMA_DISABLE_INT_DISABLE,
            true => DISABLE_INT_ENABLE_A::DMA_DISABLE_INT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_DISABLE_INT_DISABLE`"]
    #[inline(always)]
    pub fn is_dma_disable_int_disable(&self) -> bool {
        *self == DISABLE_INT_ENABLE_A::DMA_DISABLE_INT_DISABLE
    }
    #[doc = "Checks if the value of the field is `DMA_DISABLE_INT_ENABLE`"]
    #[inline(always)]
    pub fn is_dma_disable_int_enable(&self) -> bool {
        *self == DISABLE_INT_ENABLE_A::DMA_DISABLE_INT_ENABLE
    }
}
#[doc = "Write proxy for field `DISABLE_INT_ENABLE`"]
pub struct DISABLE_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_INT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLE_INT_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable channel disable indicator interrupts for DMA channel"]
    #[inline(always)]
    pub fn dma_disable_int_disable(self) -> &'a mut W {
        self.variant(DISABLE_INT_ENABLE_A::DMA_DISABLE_INT_DISABLE)
    }
    #[doc = "Enable channel disable indicator interrupts for DMA channel"]
    #[inline(always)]
    pub fn dma_disable_int_enable(self) -> &'a mut W {
        self.variant(DISABLE_INT_ENABLE_A::DMA_DISABLE_INT_ENABLE)
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
#[doc = "Raise an interrupt when a state machine error occurs during a DMA transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_INT_ENABLE_A {
    #[doc = "0: Disable error indicator interrupts for DMA channel"]
    DMA_ERROR_INT_DISABLE = 0,
    #[doc = "1: Enable error indicator interrupts for DMA channel"]
    DMA_ERROR_INT_ENABLE = 1,
}
impl From<ERROR_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_INT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERROR_INT_ENABLE`"]
pub type ERROR_INT_ENABLE_R = crate::R<bool, ERROR_INT_ENABLE_A>;
impl ERROR_INT_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_INT_ENABLE_A {
        match self.bits {
            false => ERROR_INT_ENABLE_A::DMA_ERROR_INT_DISABLE,
            true => ERROR_INT_ENABLE_A::DMA_ERROR_INT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_ERROR_INT_DISABLE`"]
    #[inline(always)]
    pub fn is_dma_error_int_disable(&self) -> bool {
        *self == ERROR_INT_ENABLE_A::DMA_ERROR_INT_DISABLE
    }
    #[doc = "Checks if the value of the field is `DMA_ERROR_INT_ENABLE`"]
    #[inline(always)]
    pub fn is_dma_error_int_enable(&self) -> bool {
        *self == ERROR_INT_ENABLE_A::DMA_ERROR_INT_ENABLE
    }
}
#[doc = "Write proxy for field `ERROR_INT_ENABLE`"]
pub struct ERROR_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_INT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROR_INT_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable error indicator interrupts for DMA channel"]
    #[inline(always)]
    pub fn dma_error_int_disable(self) -> &'a mut W {
        self.variant(ERROR_INT_ENABLE_A::DMA_ERROR_INT_DISABLE)
    }
    #[doc = "Enable error indicator interrupts for DMA channel"]
    #[inline(always)]
    pub fn dma_error_int_enable(self) -> &'a mut W {
        self.variant(ERROR_INT_ENABLE_A::DMA_ERROR_INT_ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Raise an interrupt when the DMA transfer completes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPLETE_INT_ENABLE_A {
    #[doc = "0: Disable completion interrupts for DMA channel"]
    DMA_COMPLETE_INT_DISABLE = 0,
    #[doc = "1: Enable completion interrupts for DMA channel"]
    DMA_COMPLETE_INT_ENABLE = 1,
}
impl From<COMPLETE_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: COMPLETE_INT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPLETE_INT_ENABLE`"]
pub type COMPLETE_INT_ENABLE_R = crate::R<bool, COMPLETE_INT_ENABLE_A>;
impl COMPLETE_INT_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPLETE_INT_ENABLE_A {
        match self.bits {
            false => COMPLETE_INT_ENABLE_A::DMA_COMPLETE_INT_DISABLE,
            true => COMPLETE_INT_ENABLE_A::DMA_COMPLETE_INT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_COMPLETE_INT_DISABLE`"]
    #[inline(always)]
    pub fn is_dma_complete_int_disable(&self) -> bool {
        *self == COMPLETE_INT_ENABLE_A::DMA_COMPLETE_INT_DISABLE
    }
    #[doc = "Checks if the value of the field is `DMA_COMPLETE_INT_ENABLE`"]
    #[inline(always)]
    pub fn is_dma_complete_int_enable(&self) -> bool {
        *self == COMPLETE_INT_ENABLE_A::DMA_COMPLETE_INT_ENABLE
    }
}
#[doc = "Write proxy for field `COMPLETE_INT_ENABLE`"]
pub struct COMPLETE_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPLETE_INT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPLETE_INT_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable completion interrupts for DMA channel"]
    #[inline(always)]
    pub fn dma_complete_int_disable(self) -> &'a mut W {
        self.variant(COMPLETE_INT_ENABLE_A::DMA_COMPLETE_INT_DISABLE)
    }
    #[doc = "Enable completion interrupts for DMA channel"]
    #[inline(always)]
    pub fn dma_complete_int_enable(self) -> &'a mut W {
        self.variant(COMPLETE_INT_ENABLE_A::DMA_COMPLETE_INT_ENABLE)
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
#[doc = "Raise an interrupt when the DMA transfer reaches the counter value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COUNTER_INT_ENABLE_A {
    #[doc = "0: Disable counter interrupts for DMA channel"]
    DMA_COUNTER_INT_DISABLE = 0,
    #[doc = "1: Enable counter interrupts for DMA channel"]
    DMA_COUNTER_INT_ENABLE = 1,
}
impl From<COUNTER_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: COUNTER_INT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COUNTER_INT_ENABLE`"]
pub type COUNTER_INT_ENABLE_R = crate::R<bool, COUNTER_INT_ENABLE_A>;
impl COUNTER_INT_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COUNTER_INT_ENABLE_A {
        match self.bits {
            false => COUNTER_INT_ENABLE_A::DMA_COUNTER_INT_DISABLE,
            true => COUNTER_INT_ENABLE_A::DMA_COUNTER_INT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_COUNTER_INT_DISABLE`"]
    #[inline(always)]
    pub fn is_dma_counter_int_disable(&self) -> bool {
        *self == COUNTER_INT_ENABLE_A::DMA_COUNTER_INT_DISABLE
    }
    #[doc = "Checks if the value of the field is `DMA_COUNTER_INT_ENABLE`"]
    #[inline(always)]
    pub fn is_dma_counter_int_enable(&self) -> bool {
        *self == COUNTER_INT_ENABLE_A::DMA_COUNTER_INT_ENABLE
    }
}
#[doc = "Write proxy for field `COUNTER_INT_ENABLE`"]
pub struct COUNTER_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_INT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COUNTER_INT_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable counter interrupts for DMA channel"]
    #[inline(always)]
    pub fn dma_counter_int_disable(self) -> &'a mut W {
        self.variant(COUNTER_INT_ENABLE_A::DMA_COUNTER_INT_DISABLE)
    }
    #[doc = "Enable counter interrupts for DMA channel"]
    #[inline(always)]
    pub fn dma_counter_int_enable(self) -> &'a mut W {
        self.variant(COUNTER_INT_ENABLE_A::DMA_COUNTER_INT_ENABLE)
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
#[doc = "Raise an interrupt when the DMA transfer starts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_INT_ENABLE_A {
    #[doc = "0: Disable start interrupts for DMA channel"]
    DMA_START_INT_DISABLE = 0,
    #[doc = "1: Enable start interrupts for DMA channel"]
    DMA_START_INT_ENABLE = 1,
}
impl From<START_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: START_INT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `START_INT_ENABLE`"]
pub type START_INT_ENABLE_R = crate::R<bool, START_INT_ENABLE_A>;
impl START_INT_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_INT_ENABLE_A {
        match self.bits {
            false => START_INT_ENABLE_A::DMA_START_INT_DISABLE,
            true => START_INT_ENABLE_A::DMA_START_INT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_START_INT_DISABLE`"]
    #[inline(always)]
    pub fn is_dma_start_int_disable(&self) -> bool {
        *self == START_INT_ENABLE_A::DMA_START_INT_DISABLE
    }
    #[doc = "Checks if the value of the field is `DMA_START_INT_ENABLE`"]
    #[inline(always)]
    pub fn is_dma_start_int_enable(&self) -> bool {
        *self == START_INT_ENABLE_A::DMA_START_INT_ENABLE
    }
}
#[doc = "Write proxy for field `START_INT_ENABLE`"]
pub struct START_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> START_INT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_INT_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable start interrupts for DMA channel"]
    #[inline(always)]
    pub fn dma_start_int_disable(self) -> &'a mut W {
        self.variant(START_INT_ENABLE_A::DMA_START_INT_DISABLE)
    }
    #[doc = "Enable start interrupts for DMA channel"]
    #[inline(always)]
    pub fn dma_start_int_enable(self) -> &'a mut W {
        self.variant(START_INT_ENABLE_A::DMA_START_INT_ENABLE)
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
#[doc = "Select the destination word size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEST_WORD_SIZE_A {
    #[doc = "0: Destination data uses 8-bit words"]
    DMA_DEST_WORD_SIZE_8 = 0,
    #[doc = "1: Destination data uses 16-bit words"]
    DMA_DEST_WORD_SIZE_16 = 1,
    #[doc = "2: Destination data uses 32-bit words"]
    DMA_DEST_WORD_SIZE_32 = 2,
    #[doc = "3: Destination data uses 4-bit words"]
    DMA_DEST_WORD_SIZE_4 = 3,
}
impl From<DEST_WORD_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DEST_WORD_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DEST_WORD_SIZE`"]
pub type DEST_WORD_SIZE_R = crate::R<u8, DEST_WORD_SIZE_A>;
impl DEST_WORD_SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEST_WORD_SIZE_A {
        match self.bits {
            0 => DEST_WORD_SIZE_A::DMA_DEST_WORD_SIZE_8,
            1 => DEST_WORD_SIZE_A::DMA_DEST_WORD_SIZE_16,
            2 => DEST_WORD_SIZE_A::DMA_DEST_WORD_SIZE_32,
            3 => DEST_WORD_SIZE_A::DMA_DEST_WORD_SIZE_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_WORD_SIZE_8`"]
    #[inline(always)]
    pub fn is_dma_dest_word_size_8(&self) -> bool {
        *self == DEST_WORD_SIZE_A::DMA_DEST_WORD_SIZE_8
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_WORD_SIZE_16`"]
    #[inline(always)]
    pub fn is_dma_dest_word_size_16(&self) -> bool {
        *self == DEST_WORD_SIZE_A::DMA_DEST_WORD_SIZE_16
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_WORD_SIZE_32`"]
    #[inline(always)]
    pub fn is_dma_dest_word_size_32(&self) -> bool {
        *self == DEST_WORD_SIZE_A::DMA_DEST_WORD_SIZE_32
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_WORD_SIZE_4`"]
    #[inline(always)]
    pub fn is_dma_dest_word_size_4(&self) -> bool {
        *self == DEST_WORD_SIZE_A::DMA_DEST_WORD_SIZE_4
    }
}
#[doc = "Write proxy for field `DEST_WORD_SIZE`"]
pub struct DEST_WORD_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEST_WORD_SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEST_WORD_SIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Destination data uses 8-bit words"]
    #[inline(always)]
    pub fn dma_dest_word_size_8(self) -> &'a mut W {
        self.variant(DEST_WORD_SIZE_A::DMA_DEST_WORD_SIZE_8)
    }
    #[doc = "Destination data uses 16-bit words"]
    #[inline(always)]
    pub fn dma_dest_word_size_16(self) -> &'a mut W {
        self.variant(DEST_WORD_SIZE_A::DMA_DEST_WORD_SIZE_16)
    }
    #[doc = "Destination data uses 32-bit words"]
    #[inline(always)]
    pub fn dma_dest_word_size_32(self) -> &'a mut W {
        self.variant(DEST_WORD_SIZE_A::DMA_DEST_WORD_SIZE_32)
    }
    #[doc = "Destination data uses 4-bit words"]
    #[inline(always)]
    pub fn dma_dest_word_size_4(self) -> &'a mut W {
        self.variant(DEST_WORD_SIZE_A::DMA_DEST_WORD_SIZE_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Select the source word size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_WORD_SIZE_A {
    #[doc = "0: Source data uses 8-bit words"]
    DMA_SRC_WORD_SIZE_8 = 0,
    #[doc = "1: Source data uses 16-bit words"]
    DMA_SRC_WORD_SIZE_16 = 1,
    #[doc = "2: Source data uses 32-bit words"]
    DMA_SRC_WORD_SIZE_32 = 2,
    #[doc = "3: Source data uses 4-bit words"]
    DMA_SRC_WORD_SIZE_4 = 3,
}
impl From<SRC_WORD_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_WORD_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC_WORD_SIZE`"]
pub type SRC_WORD_SIZE_R = crate::R<u8, SRC_WORD_SIZE_A>;
impl SRC_WORD_SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_WORD_SIZE_A {
        match self.bits {
            0 => SRC_WORD_SIZE_A::DMA_SRC_WORD_SIZE_8,
            1 => SRC_WORD_SIZE_A::DMA_SRC_WORD_SIZE_16,
            2 => SRC_WORD_SIZE_A::DMA_SRC_WORD_SIZE_32,
            3 => SRC_WORD_SIZE_A::DMA_SRC_WORD_SIZE_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_WORD_SIZE_8`"]
    #[inline(always)]
    pub fn is_dma_src_word_size_8(&self) -> bool {
        *self == SRC_WORD_SIZE_A::DMA_SRC_WORD_SIZE_8
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_WORD_SIZE_16`"]
    #[inline(always)]
    pub fn is_dma_src_word_size_16(&self) -> bool {
        *self == SRC_WORD_SIZE_A::DMA_SRC_WORD_SIZE_16
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_WORD_SIZE_32`"]
    #[inline(always)]
    pub fn is_dma_src_word_size_32(&self) -> bool {
        *self == SRC_WORD_SIZE_A::DMA_SRC_WORD_SIZE_32
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_WORD_SIZE_4`"]
    #[inline(always)]
    pub fn is_dma_src_word_size_4(&self) -> bool {
        *self == SRC_WORD_SIZE_A::DMA_SRC_WORD_SIZE_4
    }
}
#[doc = "Write proxy for field `SRC_WORD_SIZE`"]
pub struct SRC_WORD_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_WORD_SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_WORD_SIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Source data uses 8-bit words"]
    #[inline(always)]
    pub fn dma_src_word_size_8(self) -> &'a mut W {
        self.variant(SRC_WORD_SIZE_A::DMA_SRC_WORD_SIZE_8)
    }
    #[doc = "Source data uses 16-bit words"]
    #[inline(always)]
    pub fn dma_src_word_size_16(self) -> &'a mut W {
        self.variant(SRC_WORD_SIZE_A::DMA_SRC_WORD_SIZE_16)
    }
    #[doc = "Source data uses 32-bit words"]
    #[inline(always)]
    pub fn dma_src_word_size_32(self) -> &'a mut W {
        self.variant(SRC_WORD_SIZE_A::DMA_SRC_WORD_SIZE_32)
    }
    #[doc = "Source data uses 4-bit words"]
    #[inline(always)]
    pub fn dma_src_word_size_4(self) -> &'a mut W {
        self.variant(SRC_WORD_SIZE_A::DMA_SRC_WORD_SIZE_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Select the request line for the destination\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEST_SELECT_A {
    #[doc = "0: Data writes are triggered by the I2C request line"]
    DMA_DEST_I2C = 0,
    #[doc = "1: Data writes are triggered by the SPI0 request line"]
    DMA_DEST_SPI0 = 1,
    #[doc = "2: Data writes are triggered by the SPI1 request line"]
    DMA_DEST_SPI1 = 2,
    #[doc = "3: Data writes are triggered by the PCM request line"]
    DMA_DEST_PCM = 3,
    #[doc = "4: Data writes are triggered by the UART request line"]
    DMA_DEST_UART = 4,
    #[doc = "5: Data writes are triggered by the ASRC request line"]
    DMA_DEST_ASRC = 5,
    #[doc = "6: Data writes are triggered by the P bus request line"]
    DMA_DEST_PBUS = 6,
    #[doc = "7: Data writes are triggered by the OD request line"]
    DMA_DEST_OD = 7,
}
impl From<DEST_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DEST_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DEST_SELECT`"]
pub type DEST_SELECT_R = crate::R<u8, DEST_SELECT_A>;
impl DEST_SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEST_SELECT_A {
        match self.bits {
            0 => DEST_SELECT_A::DMA_DEST_I2C,
            1 => DEST_SELECT_A::DMA_DEST_SPI0,
            2 => DEST_SELECT_A::DMA_DEST_SPI1,
            3 => DEST_SELECT_A::DMA_DEST_PCM,
            4 => DEST_SELECT_A::DMA_DEST_UART,
            5 => DEST_SELECT_A::DMA_DEST_ASRC,
            6 => DEST_SELECT_A::DMA_DEST_PBUS,
            7 => DEST_SELECT_A::DMA_DEST_OD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_I2C`"]
    #[inline(always)]
    pub fn is_dma_dest_i2c(&self) -> bool {
        *self == DEST_SELECT_A::DMA_DEST_I2C
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_SPI0`"]
    #[inline(always)]
    pub fn is_dma_dest_spi0(&self) -> bool {
        *self == DEST_SELECT_A::DMA_DEST_SPI0
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_SPI1`"]
    #[inline(always)]
    pub fn is_dma_dest_spi1(&self) -> bool {
        *self == DEST_SELECT_A::DMA_DEST_SPI1
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_PCM`"]
    #[inline(always)]
    pub fn is_dma_dest_pcm(&self) -> bool {
        *self == DEST_SELECT_A::DMA_DEST_PCM
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_UART`"]
    #[inline(always)]
    pub fn is_dma_dest_uart(&self) -> bool {
        *self == DEST_SELECT_A::DMA_DEST_UART
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_ASRC`"]
    #[inline(always)]
    pub fn is_dma_dest_asrc(&self) -> bool {
        *self == DEST_SELECT_A::DMA_DEST_ASRC
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_PBUS`"]
    #[inline(always)]
    pub fn is_dma_dest_pbus(&self) -> bool {
        *self == DEST_SELECT_A::DMA_DEST_PBUS
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_OD`"]
    #[inline(always)]
    pub fn is_dma_dest_od(&self) -> bool {
        *self == DEST_SELECT_A::DMA_DEST_OD
    }
}
#[doc = "Write proxy for field `DEST_SELECT`"]
pub struct DEST_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEST_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEST_SELECT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Data writes are triggered by the I2C request line"]
    #[inline(always)]
    pub fn dma_dest_i2c(self) -> &'a mut W {
        self.variant(DEST_SELECT_A::DMA_DEST_I2C)
    }
    #[doc = "Data writes are triggered by the SPI0 request line"]
    #[inline(always)]
    pub fn dma_dest_spi0(self) -> &'a mut W {
        self.variant(DEST_SELECT_A::DMA_DEST_SPI0)
    }
    #[doc = "Data writes are triggered by the SPI1 request line"]
    #[inline(always)]
    pub fn dma_dest_spi1(self) -> &'a mut W {
        self.variant(DEST_SELECT_A::DMA_DEST_SPI1)
    }
    #[doc = "Data writes are triggered by the PCM request line"]
    #[inline(always)]
    pub fn dma_dest_pcm(self) -> &'a mut W {
        self.variant(DEST_SELECT_A::DMA_DEST_PCM)
    }
    #[doc = "Data writes are triggered by the UART request line"]
    #[inline(always)]
    pub fn dma_dest_uart(self) -> &'a mut W {
        self.variant(DEST_SELECT_A::DMA_DEST_UART)
    }
    #[doc = "Data writes are triggered by the ASRC request line"]
    #[inline(always)]
    pub fn dma_dest_asrc(self) -> &'a mut W {
        self.variant(DEST_SELECT_A::DMA_DEST_ASRC)
    }
    #[doc = "Data writes are triggered by the P bus request line"]
    #[inline(always)]
    pub fn dma_dest_pbus(self) -> &'a mut W {
        self.variant(DEST_SELECT_A::DMA_DEST_PBUS)
    }
    #[doc = "Data writes are triggered by the OD request line"]
    #[inline(always)]
    pub fn dma_dest_od(self) -> &'a mut W {
        self.variant(DEST_SELECT_A::DMA_DEST_OD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Select the request line for the source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_SELECT_A {
    #[doc = "0: Data reads are triggered by the I2C request line"]
    DMA_SRC_I2C = 0,
    #[doc = "1: Data reads are triggered by the SPI0 request line"]
    DMA_SRC_SPI0 = 1,
    #[doc = "2: Data reads are triggered by the SPI1 request line"]
    DMA_SRC_SPI1 = 2,
    #[doc = "3: Data reads are triggered by the PCM request line"]
    DMA_SRC_PCM = 3,
    #[doc = "4: Data reads are triggered by the UART request line"]
    DMA_SRC_UART = 4,
    #[doc = "5: Data reads are triggered by the ASRC request line"]
    DMA_SRC_ASRC = 5,
    #[doc = "6: Data reads are triggered by the P Bus request line"]
    DMA_SRC_PBUS = 6,
    #[doc = "7: Data reads are triggered by the DMIC request line"]
    DMA_SRC_DMIC = 7,
}
impl From<SRC_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC_SELECT`"]
pub type SRC_SELECT_R = crate::R<u8, SRC_SELECT_A>;
impl SRC_SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_SELECT_A {
        match self.bits {
            0 => SRC_SELECT_A::DMA_SRC_I2C,
            1 => SRC_SELECT_A::DMA_SRC_SPI0,
            2 => SRC_SELECT_A::DMA_SRC_SPI1,
            3 => SRC_SELECT_A::DMA_SRC_PCM,
            4 => SRC_SELECT_A::DMA_SRC_UART,
            5 => SRC_SELECT_A::DMA_SRC_ASRC,
            6 => SRC_SELECT_A::DMA_SRC_PBUS,
            7 => SRC_SELECT_A::DMA_SRC_DMIC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_I2C`"]
    #[inline(always)]
    pub fn is_dma_src_i2c(&self) -> bool {
        *self == SRC_SELECT_A::DMA_SRC_I2C
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_SPI0`"]
    #[inline(always)]
    pub fn is_dma_src_spi0(&self) -> bool {
        *self == SRC_SELECT_A::DMA_SRC_SPI0
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_SPI1`"]
    #[inline(always)]
    pub fn is_dma_src_spi1(&self) -> bool {
        *self == SRC_SELECT_A::DMA_SRC_SPI1
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_PCM`"]
    #[inline(always)]
    pub fn is_dma_src_pcm(&self) -> bool {
        *self == SRC_SELECT_A::DMA_SRC_PCM
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_UART`"]
    #[inline(always)]
    pub fn is_dma_src_uart(&self) -> bool {
        *self == SRC_SELECT_A::DMA_SRC_UART
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_ASRC`"]
    #[inline(always)]
    pub fn is_dma_src_asrc(&self) -> bool {
        *self == SRC_SELECT_A::DMA_SRC_ASRC
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_PBUS`"]
    #[inline(always)]
    pub fn is_dma_src_pbus(&self) -> bool {
        *self == SRC_SELECT_A::DMA_SRC_PBUS
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_DMIC`"]
    #[inline(always)]
    pub fn is_dma_src_dmic(&self) -> bool {
        *self == SRC_SELECT_A::DMA_SRC_DMIC
    }
}
#[doc = "Write proxy for field `SRC_SELECT`"]
pub struct SRC_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_SELECT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Data reads are triggered by the I2C request line"]
    #[inline(always)]
    pub fn dma_src_i2c(self) -> &'a mut W {
        self.variant(SRC_SELECT_A::DMA_SRC_I2C)
    }
    #[doc = "Data reads are triggered by the SPI0 request line"]
    #[inline(always)]
    pub fn dma_src_spi0(self) -> &'a mut W {
        self.variant(SRC_SELECT_A::DMA_SRC_SPI0)
    }
    #[doc = "Data reads are triggered by the SPI1 request line"]
    #[inline(always)]
    pub fn dma_src_spi1(self) -> &'a mut W {
        self.variant(SRC_SELECT_A::DMA_SRC_SPI1)
    }
    #[doc = "Data reads are triggered by the PCM request line"]
    #[inline(always)]
    pub fn dma_src_pcm(self) -> &'a mut W {
        self.variant(SRC_SELECT_A::DMA_SRC_PCM)
    }
    #[doc = "Data reads are triggered by the UART request line"]
    #[inline(always)]
    pub fn dma_src_uart(self) -> &'a mut W {
        self.variant(SRC_SELECT_A::DMA_SRC_UART)
    }
    #[doc = "Data reads are triggered by the ASRC request line"]
    #[inline(always)]
    pub fn dma_src_asrc(self) -> &'a mut W {
        self.variant(SRC_SELECT_A::DMA_SRC_ASRC)
    }
    #[doc = "Data reads are triggered by the P Bus request line"]
    #[inline(always)]
    pub fn dma_src_pbus(self) -> &'a mut W {
        self.variant(SRC_SELECT_A::DMA_SRC_PBUS)
    }
    #[doc = "Data reads are triggered by the DMIC request line"]
    #[inline(always)]
    pub fn dma_src_dmic(self) -> &'a mut W {
        self.variant(SRC_SELECT_A::DMA_SRC_DMIC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Select the priority level for this channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHANNEL_PRIORITY_A {
    #[doc = "0: Set the priority of DMA channel to 0 (Lowest)"]
    DMA_PRIORITY_0 = 0,
    #[doc = "1: Set the priority of DMA channel to 1"]
    DMA_PRIORITY_1 = 1,
    #[doc = "2: Set the priority of DMA channel to 2"]
    DMA_PRIORITY_2 = 2,
    #[doc = "3: Set the priority of DMA channel to 3 (Highest)"]
    DMA_PRIORITY_3 = 3,
}
impl From<CHANNEL_PRIORITY_A> for u8 {
    #[inline(always)]
    fn from(variant: CHANNEL_PRIORITY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHANNEL_PRIORITY`"]
pub type CHANNEL_PRIORITY_R = crate::R<u8, CHANNEL_PRIORITY_A>;
impl CHANNEL_PRIORITY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL_PRIORITY_A {
        match self.bits {
            0 => CHANNEL_PRIORITY_A::DMA_PRIORITY_0,
            1 => CHANNEL_PRIORITY_A::DMA_PRIORITY_1,
            2 => CHANNEL_PRIORITY_A::DMA_PRIORITY_2,
            3 => CHANNEL_PRIORITY_A::DMA_PRIORITY_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_PRIORITY_0`"]
    #[inline(always)]
    pub fn is_dma_priority_0(&self) -> bool {
        *self == CHANNEL_PRIORITY_A::DMA_PRIORITY_0
    }
    #[doc = "Checks if the value of the field is `DMA_PRIORITY_1`"]
    #[inline(always)]
    pub fn is_dma_priority_1(&self) -> bool {
        *self == CHANNEL_PRIORITY_A::DMA_PRIORITY_1
    }
    #[doc = "Checks if the value of the field is `DMA_PRIORITY_2`"]
    #[inline(always)]
    pub fn is_dma_priority_2(&self) -> bool {
        *self == CHANNEL_PRIORITY_A::DMA_PRIORITY_2
    }
    #[doc = "Checks if the value of the field is `DMA_PRIORITY_3`"]
    #[inline(always)]
    pub fn is_dma_priority_3(&self) -> bool {
        *self == CHANNEL_PRIORITY_A::DMA_PRIORITY_3
    }
}
#[doc = "Write proxy for field `CHANNEL_PRIORITY`"]
pub struct CHANNEL_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL_PRIORITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL_PRIORITY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Set the priority of DMA channel to 0 (Lowest)"]
    #[inline(always)]
    pub fn dma_priority_0(self) -> &'a mut W {
        self.variant(CHANNEL_PRIORITY_A::DMA_PRIORITY_0)
    }
    #[doc = "Set the priority of DMA channel to 1"]
    #[inline(always)]
    pub fn dma_priority_1(self) -> &'a mut W {
        self.variant(CHANNEL_PRIORITY_A::DMA_PRIORITY_1)
    }
    #[doc = "Set the priority of DMA channel to 2"]
    #[inline(always)]
    pub fn dma_priority_2(self) -> &'a mut W {
        self.variant(CHANNEL_PRIORITY_A::DMA_PRIORITY_2)
    }
    #[doc = "Set the priority of DMA channel to 3 (Highest)"]
    #[inline(always)]
    pub fn dma_priority_3(self) -> &'a mut W {
        self.variant(CHANNEL_PRIORITY_A::DMA_PRIORITY_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Select the type of transfer implemented by DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRANSFER_TYPE_A {
    #[doc = "0: DMA channel will provide a memory-to-memory data transfer"]
    DMA_TRANSFER_M_TO_M = 0,
    #[doc = "1: DMA channel will provide a memory-to-peripheral data transfer"]
    DMA_TRANSFER_M_TO_P = 1,
    #[doc = "2: DMA channel will provide a peripheral-to-memory data transfer"]
    DMA_TRANSFER_P_TO_M = 2,
    #[doc = "3: DMA channel will provide a peripheral-to-peripheral data transfer"]
    DMA_TRANSFER_P_TO_P = 3,
}
impl From<TRANSFER_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TRANSFER_TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRANSFER_TYPE`"]
pub type TRANSFER_TYPE_R = crate::R<u8, TRANSFER_TYPE_A>;
impl TRANSFER_TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRANSFER_TYPE_A {
        match self.bits {
            0 => TRANSFER_TYPE_A::DMA_TRANSFER_M_TO_M,
            1 => TRANSFER_TYPE_A::DMA_TRANSFER_M_TO_P,
            2 => TRANSFER_TYPE_A::DMA_TRANSFER_P_TO_M,
            3 => TRANSFER_TYPE_A::DMA_TRANSFER_P_TO_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_TRANSFER_M_TO_M`"]
    #[inline(always)]
    pub fn is_dma_transfer_m_to_m(&self) -> bool {
        *self == TRANSFER_TYPE_A::DMA_TRANSFER_M_TO_M
    }
    #[doc = "Checks if the value of the field is `DMA_TRANSFER_M_TO_P`"]
    #[inline(always)]
    pub fn is_dma_transfer_m_to_p(&self) -> bool {
        *self == TRANSFER_TYPE_A::DMA_TRANSFER_M_TO_P
    }
    #[doc = "Checks if the value of the field is `DMA_TRANSFER_P_TO_M`"]
    #[inline(always)]
    pub fn is_dma_transfer_p_to_m(&self) -> bool {
        *self == TRANSFER_TYPE_A::DMA_TRANSFER_P_TO_M
    }
    #[doc = "Checks if the value of the field is `DMA_TRANSFER_P_TO_P`"]
    #[inline(always)]
    pub fn is_dma_transfer_p_to_p(&self) -> bool {
        *self == TRANSFER_TYPE_A::DMA_TRANSFER_P_TO_P
    }
}
#[doc = "Write proxy for field `TRANSFER_TYPE`"]
pub struct TRANSFER_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSFER_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRANSFER_TYPE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DMA channel will provide a memory-to-memory data transfer"]
    #[inline(always)]
    pub fn dma_transfer_m_to_m(self) -> &'a mut W {
        self.variant(TRANSFER_TYPE_A::DMA_TRANSFER_M_TO_M)
    }
    #[doc = "DMA channel will provide a memory-to-peripheral data transfer"]
    #[inline(always)]
    pub fn dma_transfer_m_to_p(self) -> &'a mut W {
        self.variant(TRANSFER_TYPE_A::DMA_TRANSFER_M_TO_P)
    }
    #[doc = "DMA channel will provide a peripheral-to-memory data transfer"]
    #[inline(always)]
    pub fn dma_transfer_p_to_m(self) -> &'a mut W {
        self.variant(TRANSFER_TYPE_A::DMA_TRANSFER_P_TO_M)
    }
    #[doc = "DMA channel will provide a peripheral-to-peripheral data transfer"]
    #[inline(always)]
    pub fn dma_transfer_p_to_p(self) -> &'a mut W {
        self.variant(TRANSFER_TYPE_A::DMA_TRANSFER_P_TO_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Configure whether the destination address should increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEST_ADDR_INC_A {
    #[doc = "0: Do not increment the destination address used by DMA channel"]
    DMA_DEST_ADDR_STATIC = 0,
    #[doc = "1: Increment destination address used by DMA channel"]
    DMA_DEST_ADDR_INC = 1,
}
impl From<DEST_ADDR_INC_A> for bool {
    #[inline(always)]
    fn from(variant: DEST_ADDR_INC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEST_ADDR_INC`"]
pub type DEST_ADDR_INC_R = crate::R<bool, DEST_ADDR_INC_A>;
impl DEST_ADDR_INC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEST_ADDR_INC_A {
        match self.bits {
            false => DEST_ADDR_INC_A::DMA_DEST_ADDR_STATIC,
            true => DEST_ADDR_INC_A::DMA_DEST_ADDR_INC,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_ADDR_STATIC`"]
    #[inline(always)]
    pub fn is_dma_dest_addr_static(&self) -> bool {
        *self == DEST_ADDR_INC_A::DMA_DEST_ADDR_STATIC
    }
    #[doc = "Checks if the value of the field is `DMA_DEST_ADDR_INC`"]
    #[inline(always)]
    pub fn is_dma_dest_addr_inc(&self) -> bool {
        *self == DEST_ADDR_INC_A::DMA_DEST_ADDR_INC
    }
}
#[doc = "Write proxy for field `DEST_ADDR_INC`"]
pub struct DEST_ADDR_INC_W<'a> {
    w: &'a mut W,
}
impl<'a> DEST_ADDR_INC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEST_ADDR_INC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not increment the destination address used by DMA channel"]
    #[inline(always)]
    pub fn dma_dest_addr_static(self) -> &'a mut W {
        self.variant(DEST_ADDR_INC_A::DMA_DEST_ADDR_STATIC)
    }
    #[doc = "Increment destination address used by DMA channel"]
    #[inline(always)]
    pub fn dma_dest_addr_inc(self) -> &'a mut W {
        self.variant(DEST_ADDR_INC_A::DMA_DEST_ADDR_INC)
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
#[doc = "Configure whether the source address should increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_ADDR_INC_A {
    #[doc = "0: Do not increment the source address used by DMA channel"]
    DMA_SRC_ADDR_STATIC = 0,
    #[doc = "1: Increment source address used by DMA channel"]
    DMA_SRC_ADDR_INC = 1,
}
impl From<SRC_ADDR_INC_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_ADDR_INC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRC_ADDR_INC`"]
pub type SRC_ADDR_INC_R = crate::R<bool, SRC_ADDR_INC_A>;
impl SRC_ADDR_INC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_ADDR_INC_A {
        match self.bits {
            false => SRC_ADDR_INC_A::DMA_SRC_ADDR_STATIC,
            true => SRC_ADDR_INC_A::DMA_SRC_ADDR_INC,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_ADDR_STATIC`"]
    #[inline(always)]
    pub fn is_dma_src_addr_static(&self) -> bool {
        *self == SRC_ADDR_INC_A::DMA_SRC_ADDR_STATIC
    }
    #[doc = "Checks if the value of the field is `DMA_SRC_ADDR_INC`"]
    #[inline(always)]
    pub fn is_dma_src_addr_inc(&self) -> bool {
        *self == SRC_ADDR_INC_A::DMA_SRC_ADDR_INC
    }
}
#[doc = "Write proxy for field `SRC_ADDR_INC`"]
pub struct SRC_ADDR_INC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_ADDR_INC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_ADDR_INC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not increment the source address used by DMA channel"]
    #[inline(always)]
    pub fn dma_src_addr_static(self) -> &'a mut W {
        self.variant(SRC_ADDR_INC_A::DMA_SRC_ADDR_STATIC)
    }
    #[doc = "Increment source address used by DMA channel"]
    #[inline(always)]
    pub fn dma_src_addr_inc(self) -> &'a mut W {
        self.variant(SRC_ADDR_INC_A::DMA_SRC_ADDR_INC)
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
#[doc = "Select the addressing mode for this channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR_MODE_A {
    #[doc = "0: Use circular addressing for DMA channel"]
    DMA_ADDR_CIRC = 0,
    #[doc = "1: Use linear addressing for DMA channel"]
    DMA_ADDR_LIN = 1,
}
impl From<ADDR_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDR_MODE`"]
pub type ADDR_MODE_R = crate::R<bool, ADDR_MODE_A>;
impl ADDR_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR_MODE_A {
        match self.bits {
            false => ADDR_MODE_A::DMA_ADDR_CIRC,
            true => ADDR_MODE_A::DMA_ADDR_LIN,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_ADDR_CIRC`"]
    #[inline(always)]
    pub fn is_dma_addr_circ(&self) -> bool {
        *self == ADDR_MODE_A::DMA_ADDR_CIRC
    }
    #[doc = "Checks if the value of the field is `DMA_ADDR_LIN`"]
    #[inline(always)]
    pub fn is_dma_addr_lin(&self) -> bool {
        *self == ADDR_MODE_A::DMA_ADDR_LIN
    }
}
#[doc = "Write proxy for field `ADDR_MODE`"]
pub struct ADDR_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDR_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use circular addressing for DMA channel"]
    #[inline(always)]
    pub fn dma_addr_circ(self) -> &'a mut W {
        self.variant(ADDR_MODE_A::DMA_ADDR_CIRC)
    }
    #[doc = "Use linear addressing for DMA channel"]
    #[inline(always)]
    pub fn dma_addr_lin(self) -> &'a mut W {
        self.variant(ADDR_MODE_A::DMA_ADDR_LIN)
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
#[doc = "Enable DMA Channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Disable DMA channel (channel will wait on current transfer before stopping)"]
    DMA_DISABLE = 0,
    #[doc = "1: Enable DMA channel"]
    DMA_ENABLE = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DMA_DISABLE,
            true => ENABLE_A::DMA_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_DISABLE`"]
    #[inline(always)]
    pub fn is_dma_disable(&self) -> bool {
        *self == ENABLE_A::DMA_DISABLE
    }
    #[doc = "Checks if the value of the field is `DMA_ENABLE`"]
    #[inline(always)]
    pub fn is_dma_enable(&self) -> bool {
        *self == ENABLE_A::DMA_ENABLE
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable DMA channel (channel will wait on current transfer before stopping)"]
    #[inline(always)]
    pub fn dma_disable(self) -> &'a mut W {
        self.variant(ENABLE_A::DMA_DISABLE)
    }
    #[doc = "Enable DMA channel"]
    #[inline(always)]
    pub fn dma_enable(self) -> &'a mut W {
        self.variant(ENABLE_A::DMA_ENABLE)
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
    #[doc = "Bits 30:31 - Select the destination address step size"]
    #[inline(always)]
    pub fn dest_addr_step_size(&self) -> DEST_ADDR_STEP_SIZE_R {
        DEST_ADDR_STEP_SIZE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Select the source address step size"]
    #[inline(always)]
    pub fn src_addr_step_size(&self) -> SRC_ADDR_STEP_SIZE_R {
        SRC_ADDR_STEP_SIZE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 27 - Configure the destination address to either increment or decrement"]
    #[inline(always)]
    pub fn dest_addr_step_mode(&self) -> DEST_ADDR_STEP_MODE_R {
        DEST_ADDR_STEP_MODE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Configure the source address to either increment or decrement"]
    #[inline(always)]
    pub fn src_addr_step_mode(&self) -> SRC_ADDR_STEP_MODE_R {
        SRC_ADDR_STEP_MODE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Select the byte ordering for the DMA channel"]
    #[inline(always)]
    pub fn byte_order(&self) -> BYTE_ORDER_R {
        BYTE_ORDER_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Raise an interrupt when the DMA channel is disabled"]
    #[inline(always)]
    pub fn disable_int_enable(&self) -> DISABLE_INT_ENABLE_R {
        DISABLE_INT_ENABLE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Raise an interrupt when a state machine error occurs during a DMA transfer"]
    #[inline(always)]
    pub fn error_int_enable(&self) -> ERROR_INT_ENABLE_R {
        ERROR_INT_ENABLE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Raise an interrupt when the DMA transfer completes"]
    #[inline(always)]
    pub fn complete_int_enable(&self) -> COMPLETE_INT_ENABLE_R {
        COMPLETE_INT_ENABLE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Raise an interrupt when the DMA transfer reaches the counter value"]
    #[inline(always)]
    pub fn counter_int_enable(&self) -> COUNTER_INT_ENABLE_R {
        COUNTER_INT_ENABLE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Raise an interrupt when the DMA transfer starts"]
    #[inline(always)]
    pub fn start_int_enable(&self) -> START_INT_ENABLE_R {
        START_INT_ENABLE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Select the destination word size"]
    #[inline(always)]
    pub fn dest_word_size(&self) -> DEST_WORD_SIZE_R {
        DEST_WORD_SIZE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Select the source word size"]
    #[inline(always)]
    pub fn src_word_size(&self) -> SRC_WORD_SIZE_R {
        SRC_WORD_SIZE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:14 - Select the request line for the destination"]
    #[inline(always)]
    pub fn dest_select(&self) -> DEST_SELECT_R {
        DEST_SELECT_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - Select the request line for the source"]
    #[inline(always)]
    pub fn src_select(&self) -> SRC_SELECT_R {
        SRC_SELECT_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Select the priority level for this channel"]
    #[inline(always)]
    pub fn channel_priority(&self) -> CHANNEL_PRIORITY_R {
        CHANNEL_PRIORITY_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Select the type of transfer implemented by DMA channel"]
    #[inline(always)]
    pub fn transfer_type(&self) -> TRANSFER_TYPE_R {
        TRANSFER_TYPE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Configure whether the destination address should increment"]
    #[inline(always)]
    pub fn dest_addr_inc(&self) -> DEST_ADDR_INC_R {
        DEST_ADDR_INC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Configure whether the source address should increment"]
    #[inline(always)]
    pub fn src_addr_inc(&self) -> SRC_ADDR_INC_R {
        SRC_ADDR_INC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Select the addressing mode for this channel"]
    #[inline(always)]
    pub fn addr_mode(&self) -> ADDR_MODE_R {
        ADDR_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable DMA Channel"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31 - Select the destination address step size"]
    #[inline(always)]
    pub fn dest_addr_step_size(&mut self) -> DEST_ADDR_STEP_SIZE_W {
        DEST_ADDR_STEP_SIZE_W { w: self }
    }
    #[doc = "Bits 28:29 - Select the source address step size"]
    #[inline(always)]
    pub fn src_addr_step_size(&mut self) -> SRC_ADDR_STEP_SIZE_W {
        SRC_ADDR_STEP_SIZE_W { w: self }
    }
    #[doc = "Bit 27 - Configure the destination address to either increment or decrement"]
    #[inline(always)]
    pub fn dest_addr_step_mode(&mut self) -> DEST_ADDR_STEP_MODE_W {
        DEST_ADDR_STEP_MODE_W { w: self }
    }
    #[doc = "Bit 26 - Configure the source address to either increment or decrement"]
    #[inline(always)]
    pub fn src_addr_step_mode(&mut self) -> SRC_ADDR_STEP_MODE_W {
        SRC_ADDR_STEP_MODE_W { w: self }
    }
    #[doc = "Bit 25 - Select the byte ordering for the DMA channel"]
    #[inline(always)]
    pub fn byte_order(&mut self) -> BYTE_ORDER_W {
        BYTE_ORDER_W { w: self }
    }
    #[doc = "Bit 24 - Raise an interrupt when the DMA channel is disabled"]
    #[inline(always)]
    pub fn disable_int_enable(&mut self) -> DISABLE_INT_ENABLE_W {
        DISABLE_INT_ENABLE_W { w: self }
    }
    #[doc = "Bit 23 - Raise an interrupt when a state machine error occurs during a DMA transfer"]
    #[inline(always)]
    pub fn error_int_enable(&mut self) -> ERROR_INT_ENABLE_W {
        ERROR_INT_ENABLE_W { w: self }
    }
    #[doc = "Bit 22 - Raise an interrupt when the DMA transfer completes"]
    #[inline(always)]
    pub fn complete_int_enable(&mut self) -> COMPLETE_INT_ENABLE_W {
        COMPLETE_INT_ENABLE_W { w: self }
    }
    #[doc = "Bit 21 - Raise an interrupt when the DMA transfer reaches the counter value"]
    #[inline(always)]
    pub fn counter_int_enable(&mut self) -> COUNTER_INT_ENABLE_W {
        COUNTER_INT_ENABLE_W { w: self }
    }
    #[doc = "Bit 20 - Raise an interrupt when the DMA transfer starts"]
    #[inline(always)]
    pub fn start_int_enable(&mut self) -> START_INT_ENABLE_W {
        START_INT_ENABLE_W { w: self }
    }
    #[doc = "Bits 18:19 - Select the destination word size"]
    #[inline(always)]
    pub fn dest_word_size(&mut self) -> DEST_WORD_SIZE_W {
        DEST_WORD_SIZE_W { w: self }
    }
    #[doc = "Bits 16:17 - Select the source word size"]
    #[inline(always)]
    pub fn src_word_size(&mut self) -> SRC_WORD_SIZE_W {
        SRC_WORD_SIZE_W { w: self }
    }
    #[doc = "Bits 12:14 - Select the request line for the destination"]
    #[inline(always)]
    pub fn dest_select(&mut self) -> DEST_SELECT_W {
        DEST_SELECT_W { w: self }
    }
    #[doc = "Bits 9:11 - Select the request line for the source"]
    #[inline(always)]
    pub fn src_select(&mut self) -> SRC_SELECT_W {
        SRC_SELECT_W { w: self }
    }
    #[doc = "Bits 6:7 - Select the priority level for this channel"]
    #[inline(always)]
    pub fn channel_priority(&mut self) -> CHANNEL_PRIORITY_W {
        CHANNEL_PRIORITY_W { w: self }
    }
    #[doc = "Bits 4:5 - Select the type of transfer implemented by DMA channel"]
    #[inline(always)]
    pub fn transfer_type(&mut self) -> TRANSFER_TYPE_W {
        TRANSFER_TYPE_W { w: self }
    }
    #[doc = "Bit 3 - Configure whether the destination address should increment"]
    #[inline(always)]
    pub fn dest_addr_inc(&mut self) -> DEST_ADDR_INC_W {
        DEST_ADDR_INC_W { w: self }
    }
    #[doc = "Bit 2 - Configure whether the source address should increment"]
    #[inline(always)]
    pub fn src_addr_inc(&mut self) -> SRC_ADDR_INC_W {
        SRC_ADDR_INC_W { w: self }
    }
    #[doc = "Bit 1 - Select the addressing mode for this channel"]
    #[inline(always)]
    pub fn addr_mode(&mut self) -> ADDR_MODE_W {
        ADDR_MODE_W { w: self }
    }
    #[doc = "Bit 0 - Enable DMA Channel"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
