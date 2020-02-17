#[doc = "Reader of register PCM_CTRL"]
pub type R = crate::R<u32, super::PCM_CTRL>;
#[doc = "Writer for register PCM_CTRL"]
pub type W = crate::W<u32, super::PCM_CTRL>;
#[doc = "Register PCM_CTRL `reset()`'s with value 0x0601"]
impl crate::ResetValue for super::PCM_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0601
    }
}
#[doc = "PCM clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCM_CLK_POL_A {
    #[doc = "0: PCM input data sampled on PMC_CLK falling edge"]
    PCM_SAMPLE_FALLING_EDGE = 0,
    #[doc = "1: PCM input data sampled on PMC_CLK rising edge"]
    PCM_SAMPLE_RISING_EDGE = 1,
}
impl From<PCM_CLK_POL_A> for bool {
    #[inline(always)]
    fn from(variant: PCM_CLK_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PCM_CLK_POL`"]
pub type PCM_CLK_POL_R = crate::R<bool, PCM_CLK_POL_A>;
impl PCM_CLK_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCM_CLK_POL_A {
        match self.bits {
            false => PCM_CLK_POL_A::PCM_SAMPLE_FALLING_EDGE,
            true => PCM_CLK_POL_A::PCM_SAMPLE_RISING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `PCM_SAMPLE_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_pcm_sample_falling_edge(&self) -> bool {
        *self == PCM_CLK_POL_A::PCM_SAMPLE_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `PCM_SAMPLE_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_pcm_sample_rising_edge(&self) -> bool {
        *self == PCM_CLK_POL_A::PCM_SAMPLE_RISING_EDGE
    }
}
#[doc = "Write proxy for field `PCM_CLK_POL`"]
pub struct PCM_CLK_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> PCM_CLK_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCM_CLK_POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PCM input data sampled on PMC_CLK falling edge"]
    #[inline(always)]
    pub fn pcm_sample_falling_edge(self) -> &'a mut W {
        self.variant(PCM_CLK_POL_A::PCM_SAMPLE_FALLING_EDGE)
    }
    #[doc = "PCM input data sampled on PMC_CLK rising edge"]
    #[inline(always)]
    pub fn pcm_sample_rising_edge(self) -> &'a mut W {
        self.variant(PCM_CLK_POL_A::PCM_SAMPLE_RISING_EDGE)
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
#[doc = "Select whether the data will be transmitted starting with the MSB or LSB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT_ORDER_A {
    #[doc = "0: PCM data is ordered from MSB to LSB."]
    PCM_BIT_ORDER_MSB_FIRST = 0,
    #[doc = "1: PCM data is ordered from LSB to MSB."]
    PCM_BIT_ORDER_LSB_FIRST = 1,
}
impl From<BIT_ORDER_A> for bool {
    #[inline(always)]
    fn from(variant: BIT_ORDER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BIT_ORDER`"]
pub type BIT_ORDER_R = crate::R<bool, BIT_ORDER_A>;
impl BIT_ORDER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT_ORDER_A {
        match self.bits {
            false => BIT_ORDER_A::PCM_BIT_ORDER_MSB_FIRST,
            true => BIT_ORDER_A::PCM_BIT_ORDER_LSB_FIRST,
        }
    }
    #[doc = "Checks if the value of the field is `PCM_BIT_ORDER_MSB_FIRST`"]
    #[inline(always)]
    pub fn is_pcm_bit_order_msb_first(&self) -> bool {
        *self == BIT_ORDER_A::PCM_BIT_ORDER_MSB_FIRST
    }
    #[doc = "Checks if the value of the field is `PCM_BIT_ORDER_LSB_FIRST`"]
    #[inline(always)]
    pub fn is_pcm_bit_order_lsb_first(&self) -> bool {
        *self == BIT_ORDER_A::PCM_BIT_ORDER_LSB_FIRST
    }
}
#[doc = "Write proxy for field `BIT_ORDER`"]
pub struct BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_ORDER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIT_ORDER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PCM data is ordered from MSB to LSB."]
    #[inline(always)]
    pub fn pcm_bit_order_msb_first(self) -> &'a mut W {
        self.variant(BIT_ORDER_A::PCM_BIT_ORDER_MSB_FIRST)
    }
    #[doc = "PCM data is ordered from LSB to MSB."]
    #[inline(always)]
    pub fn pcm_bit_order_lsb_first(self) -> &'a mut W {
        self.variant(BIT_ORDER_A::PCM_BIT_ORDER_LSB_FIRST)
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
#[doc = "Select what bits to use for transmit data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_ALIGN_A {
    #[doc = "0: Use MSBs of transmit buffer when word size is less than 32 bits."]
    PCM_TX_ALIGN_MSB = 0,
    #[doc = "1: Use LSBs of transmit buffer when word size is less than 32 bits."]
    PCM_TX_ALIGN_LSB = 1,
}
impl From<TX_ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_ALIGN`"]
pub type TX_ALIGN_R = crate::R<bool, TX_ALIGN_A>;
impl TX_ALIGN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_ALIGN_A {
        match self.bits {
            false => TX_ALIGN_A::PCM_TX_ALIGN_MSB,
            true => TX_ALIGN_A::PCM_TX_ALIGN_LSB,
        }
    }
    #[doc = "Checks if the value of the field is `PCM_TX_ALIGN_MSB`"]
    #[inline(always)]
    pub fn is_pcm_tx_align_msb(&self) -> bool {
        *self == TX_ALIGN_A::PCM_TX_ALIGN_MSB
    }
    #[doc = "Checks if the value of the field is `PCM_TX_ALIGN_LSB`"]
    #[inline(always)]
    pub fn is_pcm_tx_align_lsb(&self) -> bool {
        *self == TX_ALIGN_A::PCM_TX_ALIGN_LSB
    }
}
#[doc = "Write proxy for field `TX_ALIGN`"]
pub struct TX_ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ALIGN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_ALIGN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use MSBs of transmit buffer when word size is less than 32 bits."]
    #[inline(always)]
    pub fn pcm_tx_align_msb(self) -> &'a mut W {
        self.variant(TX_ALIGN_A::PCM_TX_ALIGN_MSB)
    }
    #[doc = "Use LSBs of transmit buffer when word size is less than 32 bits."]
    #[inline(always)]
    pub fn pcm_tx_align_lsb(self) -> &'a mut W {
        self.variant(TX_ALIGN_A::PCM_TX_ALIGN_LSB)
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
#[doc = "Select the number of bits per PCM word\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WORD_SIZE_A {
    #[doc = "0: Use 8-bits words"]
    PCM_WORD_SIZE_8 = 0,
    #[doc = "1: Use 16-bits words"]
    PCM_WORD_SIZE_16 = 1,
    #[doc = "2: Use 24-bits words"]
    PCM_WORD_SIZE_24 = 2,
    #[doc = "3: Use 32-bits words"]
    PCM_WORD_SIZE_32 = 3,
}
impl From<WORD_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: WORD_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WORD_SIZE`"]
pub type WORD_SIZE_R = crate::R<u8, WORD_SIZE_A>;
impl WORD_SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WORD_SIZE_A {
        match self.bits {
            0 => WORD_SIZE_A::PCM_WORD_SIZE_8,
            1 => WORD_SIZE_A::PCM_WORD_SIZE_16,
            2 => WORD_SIZE_A::PCM_WORD_SIZE_24,
            3 => WORD_SIZE_A::PCM_WORD_SIZE_32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PCM_WORD_SIZE_8`"]
    #[inline(always)]
    pub fn is_pcm_word_size_8(&self) -> bool {
        *self == WORD_SIZE_A::PCM_WORD_SIZE_8
    }
    #[doc = "Checks if the value of the field is `PCM_WORD_SIZE_16`"]
    #[inline(always)]
    pub fn is_pcm_word_size_16(&self) -> bool {
        *self == WORD_SIZE_A::PCM_WORD_SIZE_16
    }
    #[doc = "Checks if the value of the field is `PCM_WORD_SIZE_24`"]
    #[inline(always)]
    pub fn is_pcm_word_size_24(&self) -> bool {
        *self == WORD_SIZE_A::PCM_WORD_SIZE_24
    }
    #[doc = "Checks if the value of the field is `PCM_WORD_SIZE_32`"]
    #[inline(always)]
    pub fn is_pcm_word_size_32(&self) -> bool {
        *self == WORD_SIZE_A::PCM_WORD_SIZE_32
    }
}
#[doc = "Write proxy for field `WORD_SIZE`"]
pub struct WORD_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> WORD_SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WORD_SIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Use 8-bits words"]
    #[inline(always)]
    pub fn pcm_word_size_8(self) -> &'a mut W {
        self.variant(WORD_SIZE_A::PCM_WORD_SIZE_8)
    }
    #[doc = "Use 16-bits words"]
    #[inline(always)]
    pub fn pcm_word_size_16(self) -> &'a mut W {
        self.variant(WORD_SIZE_A::PCM_WORD_SIZE_16)
    }
    #[doc = "Use 24-bits words"]
    #[inline(always)]
    pub fn pcm_word_size_24(self) -> &'a mut W {
        self.variant(WORD_SIZE_A::PCM_WORD_SIZE_24)
    }
    #[doc = "Use 32-bits words"]
    #[inline(always)]
    pub fn pcm_word_size_32(self) -> &'a mut W {
        self.variant(WORD_SIZE_A::PCM_WORD_SIZE_32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Align the PCM frame signal to the first/last bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAME_ALIGN_A {
    #[doc = "0: Align the PCM frame signal to the last bit of the frame."]
    PCM_FRAME_ALIGN_LAST = 0,
    #[doc = "1: Align the PCM frame signal to the first bit of the frame."]
    PCM_FRAME_ALIGN_FIRST = 1,
}
impl From<FRAME_ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: FRAME_ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRAME_ALIGN`"]
pub type FRAME_ALIGN_R = crate::R<bool, FRAME_ALIGN_A>;
impl FRAME_ALIGN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAME_ALIGN_A {
        match self.bits {
            false => FRAME_ALIGN_A::PCM_FRAME_ALIGN_LAST,
            true => FRAME_ALIGN_A::PCM_FRAME_ALIGN_FIRST,
        }
    }
    #[doc = "Checks if the value of the field is `PCM_FRAME_ALIGN_LAST`"]
    #[inline(always)]
    pub fn is_pcm_frame_align_last(&self) -> bool {
        *self == FRAME_ALIGN_A::PCM_FRAME_ALIGN_LAST
    }
    #[doc = "Checks if the value of the field is `PCM_FRAME_ALIGN_FIRST`"]
    #[inline(always)]
    pub fn is_pcm_frame_align_first(&self) -> bool {
        *self == FRAME_ALIGN_A::PCM_FRAME_ALIGN_FIRST
    }
}
#[doc = "Write proxy for field `FRAME_ALIGN`"]
pub struct FRAME_ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_ALIGN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAME_ALIGN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Align the PCM frame signal to the last bit of the frame."]
    #[inline(always)]
    pub fn pcm_frame_align_last(self) -> &'a mut W {
        self.variant(FRAME_ALIGN_A::PCM_FRAME_ALIGN_LAST)
    }
    #[doc = "Align the PCM frame signal to the first bit of the frame."]
    #[inline(always)]
    pub fn pcm_frame_align_first(self) -> &'a mut W {
        self.variant(FRAME_ALIGN_A::PCM_FRAME_ALIGN_FIRST)
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
#[doc = "Use a long/short PCM frame signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAME_WIDTH_A {
    #[doc = "0: The frame is high for one PCM clock period."]
    PCM_FRAME_WIDTH_SHORT = 0,
    #[doc = "1: The frame is high for half of the frame length."]
    PCM_FRAME_WIDTH_LONG = 1,
}
impl From<FRAME_WIDTH_A> for bool {
    #[inline(always)]
    fn from(variant: FRAME_WIDTH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRAME_WIDTH`"]
pub type FRAME_WIDTH_R = crate::R<bool, FRAME_WIDTH_A>;
impl FRAME_WIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAME_WIDTH_A {
        match self.bits {
            false => FRAME_WIDTH_A::PCM_FRAME_WIDTH_SHORT,
            true => FRAME_WIDTH_A::PCM_FRAME_WIDTH_LONG,
        }
    }
    #[doc = "Checks if the value of the field is `PCM_FRAME_WIDTH_SHORT`"]
    #[inline(always)]
    pub fn is_pcm_frame_width_short(&self) -> bool {
        *self == FRAME_WIDTH_A::PCM_FRAME_WIDTH_SHORT
    }
    #[doc = "Checks if the value of the field is `PCM_FRAME_WIDTH_LONG`"]
    #[inline(always)]
    pub fn is_pcm_frame_width_long(&self) -> bool {
        *self == FRAME_WIDTH_A::PCM_FRAME_WIDTH_LONG
    }
}
#[doc = "Write proxy for field `FRAME_WIDTH`"]
pub struct FRAME_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAME_WIDTH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The frame is high for one PCM clock period."]
    #[inline(always)]
    pub fn pcm_frame_width_short(self) -> &'a mut W {
        self.variant(FRAME_WIDTH_A::PCM_FRAME_WIDTH_SHORT)
    }
    #[doc = "The frame is high for half of the frame length."]
    #[inline(always)]
    pub fn pcm_frame_width_long(self) -> &'a mut W {
        self.variant(FRAME_WIDTH_A::PCM_FRAME_WIDTH_LONG)
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
#[doc = "Select the number of words per PCM frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FRAME_LENGTH_A {
    #[doc = "0: A frame contains 2 words."]
    PCM_MULTIWORD_2 = 0,
    #[doc = "1: A frame contains 4 words."]
    PCM_MULTIWORD_4 = 1,
    #[doc = "2: A frame contains 6 words."]
    PCM_MULTIWORD_6 = 2,
    #[doc = "3: A frame contains 8 words."]
    PCM_MULTIWORD_8 = 3,
    #[doc = "4: A frame contains 10 words."]
    PCM_MULTIWORD_10 = 4,
    #[doc = "5: A frame contains 12 words."]
    PCM_MULTIWORD_12 = 5,
    #[doc = "6: A frame contains 14 words."]
    PCM_MULTIWORD_14 = 6,
    #[doc = "7: A frame contains 16 words."]
    PCM_MULTIWORD_16 = 7,
}
impl From<FRAME_LENGTH_A> for u8 {
    #[inline(always)]
    fn from(variant: FRAME_LENGTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FRAME_LENGTH`"]
pub type FRAME_LENGTH_R = crate::R<u8, FRAME_LENGTH_A>;
impl FRAME_LENGTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAME_LENGTH_A {
        match self.bits {
            0 => FRAME_LENGTH_A::PCM_MULTIWORD_2,
            1 => FRAME_LENGTH_A::PCM_MULTIWORD_4,
            2 => FRAME_LENGTH_A::PCM_MULTIWORD_6,
            3 => FRAME_LENGTH_A::PCM_MULTIWORD_8,
            4 => FRAME_LENGTH_A::PCM_MULTIWORD_10,
            5 => FRAME_LENGTH_A::PCM_MULTIWORD_12,
            6 => FRAME_LENGTH_A::PCM_MULTIWORD_14,
            7 => FRAME_LENGTH_A::PCM_MULTIWORD_16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PCM_MULTIWORD_2`"]
    #[inline(always)]
    pub fn is_pcm_multiword_2(&self) -> bool {
        *self == FRAME_LENGTH_A::PCM_MULTIWORD_2
    }
    #[doc = "Checks if the value of the field is `PCM_MULTIWORD_4`"]
    #[inline(always)]
    pub fn is_pcm_multiword_4(&self) -> bool {
        *self == FRAME_LENGTH_A::PCM_MULTIWORD_4
    }
    #[doc = "Checks if the value of the field is `PCM_MULTIWORD_6`"]
    #[inline(always)]
    pub fn is_pcm_multiword_6(&self) -> bool {
        *self == FRAME_LENGTH_A::PCM_MULTIWORD_6
    }
    #[doc = "Checks if the value of the field is `PCM_MULTIWORD_8`"]
    #[inline(always)]
    pub fn is_pcm_multiword_8(&self) -> bool {
        *self == FRAME_LENGTH_A::PCM_MULTIWORD_8
    }
    #[doc = "Checks if the value of the field is `PCM_MULTIWORD_10`"]
    #[inline(always)]
    pub fn is_pcm_multiword_10(&self) -> bool {
        *self == FRAME_LENGTH_A::PCM_MULTIWORD_10
    }
    #[doc = "Checks if the value of the field is `PCM_MULTIWORD_12`"]
    #[inline(always)]
    pub fn is_pcm_multiword_12(&self) -> bool {
        *self == FRAME_LENGTH_A::PCM_MULTIWORD_12
    }
    #[doc = "Checks if the value of the field is `PCM_MULTIWORD_14`"]
    #[inline(always)]
    pub fn is_pcm_multiword_14(&self) -> bool {
        *self == FRAME_LENGTH_A::PCM_MULTIWORD_14
    }
    #[doc = "Checks if the value of the field is `PCM_MULTIWORD_16`"]
    #[inline(always)]
    pub fn is_pcm_multiword_16(&self) -> bool {
        *self == FRAME_LENGTH_A::PCM_MULTIWORD_16
    }
}
#[doc = "Write proxy for field `FRAME_LENGTH`"]
pub struct FRAME_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_LENGTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAME_LENGTH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "A frame contains 2 words."]
    #[inline(always)]
    pub fn pcm_multiword_2(self) -> &'a mut W {
        self.variant(FRAME_LENGTH_A::PCM_MULTIWORD_2)
    }
    #[doc = "A frame contains 4 words."]
    #[inline(always)]
    pub fn pcm_multiword_4(self) -> &'a mut W {
        self.variant(FRAME_LENGTH_A::PCM_MULTIWORD_4)
    }
    #[doc = "A frame contains 6 words."]
    #[inline(always)]
    pub fn pcm_multiword_6(self) -> &'a mut W {
        self.variant(FRAME_LENGTH_A::PCM_MULTIWORD_6)
    }
    #[doc = "A frame contains 8 words."]
    #[inline(always)]
    pub fn pcm_multiword_8(self) -> &'a mut W {
        self.variant(FRAME_LENGTH_A::PCM_MULTIWORD_8)
    }
    #[doc = "A frame contains 10 words."]
    #[inline(always)]
    pub fn pcm_multiword_10(self) -> &'a mut W {
        self.variant(FRAME_LENGTH_A::PCM_MULTIWORD_10)
    }
    #[doc = "A frame contains 12 words."]
    #[inline(always)]
    pub fn pcm_multiword_12(self) -> &'a mut W {
        self.variant(FRAME_LENGTH_A::PCM_MULTIWORD_12)
    }
    #[doc = "A frame contains 14 words."]
    #[inline(always)]
    pub fn pcm_multiword_14(self) -> &'a mut W {
        self.variant(FRAME_LENGTH_A::PCM_MULTIWORD_14)
    }
    #[doc = "A frame contains 16 words."]
    #[inline(always)]
    pub fn pcm_multiword_16(self) -> &'a mut W {
        self.variant(FRAME_LENGTH_A::PCM_MULTIWORD_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Enable the frame duration for each word\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAME_SUBFRAMES_A {
    #[doc = "0: Generate a frame signal every frame."]
    PCM_SUBFRAME_DISABLE = 0,
    #[doc = "1: Generate a frame signal every word."]
    PCM_SUBFRAME_ENABLE = 1,
}
impl From<FRAME_SUBFRAMES_A> for bool {
    #[inline(always)]
    fn from(variant: FRAME_SUBFRAMES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRAME_SUBFRAMES`"]
pub type FRAME_SUBFRAMES_R = crate::R<bool, FRAME_SUBFRAMES_A>;
impl FRAME_SUBFRAMES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAME_SUBFRAMES_A {
        match self.bits {
            false => FRAME_SUBFRAMES_A::PCM_SUBFRAME_DISABLE,
            true => FRAME_SUBFRAMES_A::PCM_SUBFRAME_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PCM_SUBFRAME_DISABLE`"]
    #[inline(always)]
    pub fn is_pcm_subframe_disable(&self) -> bool {
        *self == FRAME_SUBFRAMES_A::PCM_SUBFRAME_DISABLE
    }
    #[doc = "Checks if the value of the field is `PCM_SUBFRAME_ENABLE`"]
    #[inline(always)]
    pub fn is_pcm_subframe_enable(&self) -> bool {
        *self == FRAME_SUBFRAMES_A::PCM_SUBFRAME_ENABLE
    }
}
#[doc = "Write proxy for field `FRAME_SUBFRAMES`"]
pub struct FRAME_SUBFRAMES_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_SUBFRAMES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAME_SUBFRAMES_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Generate a frame signal every frame."]
    #[inline(always)]
    pub fn pcm_subframe_disable(self) -> &'a mut W {
        self.variant(FRAME_SUBFRAMES_A::PCM_SUBFRAME_DISABLE)
    }
    #[doc = "Generate a frame signal every word."]
    #[inline(always)]
    pub fn pcm_subframe_enable(self) -> &'a mut W {
        self.variant(FRAME_SUBFRAMES_A::PCM_SUBFRAME_ENABLE)
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
#[doc = "Select whether data transfer will be controlled by the CM3 or the DMA for PCM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTROLLER_A {
    #[doc = "0: The CM3 controls PCM data transfers."]
    PCM_CONTROLLER_CM3 = 0,
    #[doc = "1: The DMA controls PCM data transfers."]
    PCM_CONTROLLER_DMA = 1,
}
impl From<CONTROLLER_A> for bool {
    #[inline(always)]
    fn from(variant: CONTROLLER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CONTROLLER`"]
pub type CONTROLLER_R = crate::R<bool, CONTROLLER_A>;
impl CONTROLLER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONTROLLER_A {
        match self.bits {
            false => CONTROLLER_A::PCM_CONTROLLER_CM3,
            true => CONTROLLER_A::PCM_CONTROLLER_DMA,
        }
    }
    #[doc = "Checks if the value of the field is `PCM_CONTROLLER_CM3`"]
    #[inline(always)]
    pub fn is_pcm_controller_cm3(&self) -> bool {
        *self == CONTROLLER_A::PCM_CONTROLLER_CM3
    }
    #[doc = "Checks if the value of the field is `PCM_CONTROLLER_DMA`"]
    #[inline(always)]
    pub fn is_pcm_controller_dma(&self) -> bool {
        *self == CONTROLLER_A::PCM_CONTROLLER_DMA
    }
}
#[doc = "Write proxy for field `CONTROLLER`"]
pub struct CONTROLLER_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTROLLER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONTROLLER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The CM3 controls PCM data transfers."]
    #[inline(always)]
    pub fn pcm_controller_cm3(self) -> &'a mut W {
        self.variant(CONTROLLER_A::PCM_CONTROLLER_CM3)
    }
    #[doc = "The DMA controls PCM data transfers."]
    #[inline(always)]
    pub fn pcm_controller_dma(self) -> &'a mut W {
        self.variant(CONTROLLER_A::PCM_CONTROLLER_DMA)
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
#[doc = "Enable/disable the PCM interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Disable the PCM interface."]
    PCM_DISABLE = 0,
    #[doc = "1: Enable the PCM interface."]
    PCM_ENABLE = 1,
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
            false => ENABLE_A::PCM_DISABLE,
            true => ENABLE_A::PCM_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PCM_DISABLE`"]
    #[inline(always)]
    pub fn is_pcm_disable(&self) -> bool {
        *self == ENABLE_A::PCM_DISABLE
    }
    #[doc = "Checks if the value of the field is `PCM_ENABLE`"]
    #[inline(always)]
    pub fn is_pcm_enable(&self) -> bool {
        *self == ENABLE_A::PCM_ENABLE
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
    #[doc = "Disable the PCM interface."]
    #[inline(always)]
    pub fn pcm_disable(self) -> &'a mut W {
        self.variant(ENABLE_A::PCM_DISABLE)
    }
    #[doc = "Enable the PCM interface."]
    #[inline(always)]
    pub fn pcm_enable(self) -> &'a mut W {
        self.variant(ENABLE_A::PCM_ENABLE)
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
#[doc = "Use the PCM interface as a master/slave\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE_A {
    #[doc = "0: The PCM interface generates the PCM_FRAME."]
    PCM_SELECT_MASTER = 0,
    #[doc = "1: The PCM interface received an external PCM_FRAME."]
    PCM_SELECT_SLAVE = 1,
}
impl From<SLAVE_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLAVE`"]
pub type SLAVE_R = crate::R<bool, SLAVE_A>;
impl SLAVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE_A {
        match self.bits {
            false => SLAVE_A::PCM_SELECT_MASTER,
            true => SLAVE_A::PCM_SELECT_SLAVE,
        }
    }
    #[doc = "Checks if the value of the field is `PCM_SELECT_MASTER`"]
    #[inline(always)]
    pub fn is_pcm_select_master(&self) -> bool {
        *self == SLAVE_A::PCM_SELECT_MASTER
    }
    #[doc = "Checks if the value of the field is `PCM_SELECT_SLAVE`"]
    #[inline(always)]
    pub fn is_pcm_select_slave(&self) -> bool {
        *self == SLAVE_A::PCM_SELECT_SLAVE
    }
}
#[doc = "Write proxy for field `SLAVE`"]
pub struct SLAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLAVE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The PCM interface generates the PCM_FRAME."]
    #[inline(always)]
    pub fn pcm_select_master(self) -> &'a mut W {
        self.variant(SLAVE_A::PCM_SELECT_MASTER)
    }
    #[doc = "The PCM interface received an external PCM_FRAME."]
    #[inline(always)]
    pub fn pcm_select_slave(self) -> &'a mut W {
        self.variant(SLAVE_A::PCM_SELECT_SLAVE)
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
    #[doc = "Bit 14 - PCM clock polarity"]
    #[inline(always)]
    pub fn pcm_clk_pol(&self) -> PCM_CLK_POL_R {
        PCM_CLK_POL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Select whether the data will be transmitted starting with the MSB or LSB"]
    #[inline(always)]
    pub fn bit_order(&self) -> BIT_ORDER_R {
        BIT_ORDER_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Select what bits to use for transmit data"]
    #[inline(always)]
    pub fn tx_align(&self) -> TX_ALIGN_R {
        TX_ALIGN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Select the number of bits per PCM word"]
    #[inline(always)]
    pub fn word_size(&self) -> WORD_SIZE_R {
        WORD_SIZE_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Align the PCM frame signal to the first/last bit"]
    #[inline(always)]
    pub fn frame_align(&self) -> FRAME_ALIGN_R {
        FRAME_ALIGN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Use a long/short PCM frame signal"]
    #[inline(always)]
    pub fn frame_width(&self) -> FRAME_WIDTH_R {
        FRAME_WIDTH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Select the number of words per PCM frame"]
    #[inline(always)]
    pub fn frame_length(&self) -> FRAME_LENGTH_R {
        FRAME_LENGTH_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - Enable the frame duration for each word"]
    #[inline(always)]
    pub fn frame_subframes(&self) -> FRAME_SUBFRAMES_R {
        FRAME_SUBFRAMES_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Select whether data transfer will be controlled by the CM3 or the DMA for PCM"]
    #[inline(always)]
    pub fn controller(&self) -> CONTROLLER_R {
        CONTROLLER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable/disable the PCM interface"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Use the PCM interface as a master/slave"]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - PCM clock polarity"]
    #[inline(always)]
    pub fn pcm_clk_pol(&mut self) -> PCM_CLK_POL_W {
        PCM_CLK_POL_W { w: self }
    }
    #[doc = "Bit 12 - Select whether the data will be transmitted starting with the MSB or LSB"]
    #[inline(always)]
    pub fn bit_order(&mut self) -> BIT_ORDER_W {
        BIT_ORDER_W { w: self }
    }
    #[doc = "Bit 11 - Select what bits to use for transmit data"]
    #[inline(always)]
    pub fn tx_align(&mut self) -> TX_ALIGN_W {
        TX_ALIGN_W { w: self }
    }
    #[doc = "Bits 9:10 - Select the number of bits per PCM word"]
    #[inline(always)]
    pub fn word_size(&mut self) -> WORD_SIZE_W {
        WORD_SIZE_W { w: self }
    }
    #[doc = "Bit 8 - Align the PCM frame signal to the first/last bit"]
    #[inline(always)]
    pub fn frame_align(&mut self) -> FRAME_ALIGN_W {
        FRAME_ALIGN_W { w: self }
    }
    #[doc = "Bit 7 - Use a long/short PCM frame signal"]
    #[inline(always)]
    pub fn frame_width(&mut self) -> FRAME_WIDTH_W {
        FRAME_WIDTH_W { w: self }
    }
    #[doc = "Bits 4:6 - Select the number of words per PCM frame"]
    #[inline(always)]
    pub fn frame_length(&mut self) -> FRAME_LENGTH_W {
        FRAME_LENGTH_W { w: self }
    }
    #[doc = "Bit 3 - Enable the frame duration for each word"]
    #[inline(always)]
    pub fn frame_subframes(&mut self) -> FRAME_SUBFRAMES_W {
        FRAME_SUBFRAMES_W { w: self }
    }
    #[doc = "Bit 2 - Select whether data transfer will be controlled by the CM3 or the DMA for PCM"]
    #[inline(always)]
    pub fn controller(&mut self) -> CONTROLLER_W {
        CONTROLLER_W { w: self }
    }
    #[doc = "Bit 1 - Enable/disable the PCM interface"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 0 - Use the PCM interface as a master/slave"]
    #[inline(always)]
    pub fn slave(&mut self) -> SLAVE_W {
        SLAVE_W { w: self }
    }
}
